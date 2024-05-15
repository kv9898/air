//
// main_loop.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;
use futures::StreamExt;
use once_cell::sync::Lazy;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tokio::task::JoinError;
use tokio::task::JoinHandle;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::Diagnostic;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::Client;
use url::Url;

use crate::lsp;
use crate::lsp::backend::LspMessage;
use crate::lsp::backend::LspNotification;
use crate::lsp::backend::LspRequest;
use crate::lsp::backend::LspResponse;
use crate::lsp::documents::Document;
use crate::lsp::handlers;
use crate::lsp::state::WorldState;
use crate::lsp::state_handlers;
use crate::lsp::state_handlers::ConsoleInputs;

pub(crate) type TokioUnboundedSender<T> = tokio::sync::mpsc::UnboundedSender<T>;
pub(crate) type TokioUnboundedReceiver<T> = tokio::sync::mpsc::UnboundedReceiver<T>;

// The global instance of the auxiliary event channel, used for sending log
// messages from a free function. Since this is an unbounded channel, sending
// a log message is not async nor blocking.
static mut AUXILIARY_EVENT_TX: Lazy<Arc<Mutex<Option<TokioUnboundedSender<AuxiliaryEvent>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

// This is the syntax for trait aliases until an official one is stabilised.
// This alias is for the future of a `JoinHandle<anyhow::Result<T>>`
trait AnyhowJoinHandleFut<T>:
    future::Future<Output = std::result::Result<anyhow::Result<T>, tokio::task::JoinError>>
{
}
impl<T, F> AnyhowJoinHandleFut<T> for F where
    F: future::Future<Output = std::result::Result<anyhow::Result<T>, tokio::task::JoinError>>
{
}

// Alias for a list of join handle futures
type TaskList<T> = futures::stream::FuturesUnordered<Pin<Box<dyn AnyhowJoinHandleFut<T> + Send>>>;

#[derive(Debug)]
pub(crate) enum Event {
    Lsp(LspMessage),
    Task(LspTask),
    Kernel(KernelNotification),
}
#[derive(Debug)]
pub(crate) enum LspTask {
    RefreshDiagnostics(Url, Document, WorldState),
    RefreshAllDiagnostics(),
    PublishDiagnostics(Url, Vec<Diagnostic>, Option<i32>),
}

#[derive(Debug)]
pub(crate) enum KernelNotification {
    DidChangeConsoleInputs(ConsoleInputs),
}

#[derive(Debug)]
enum AuxiliaryEvent {
    Log(lsp_types::MessageType, String),
    SpawnedTask(JoinHandle<anyhow::Result<()>>),
    JoinedTask(std::result::Result<anyhow::Result<()>, JoinError>),
}

/// Global state for the main loop
///
/// This is a singleton that fully owns the source of truth for `WorldState`
/// which contains the inputs of all LSP methods. The `main_loop()` method is
/// the heart of the LSP. The tower-lsp backend and the Jupyter kernel
/// communicate with the main loop through the `Event` channel that is passed on
/// construction.
pub(crate) struct GlobalState {
    /// The global world state containing all inputs for LSP analysis lives
    /// here. The dispatcher provides refs, exclusive refs, or snapshots
    /// (clones) to handlers.
    world: WorldState,

    /// LSP client shared with tower-lsp and the log loop
    client: Client,

    /// Event channels for the main loop. The tower-lsp methods forward
    /// notifications and requests here via `Event::Lsp`. We also receive
    /// messages from the kernel via `Event::Kernel`, and from ourselves via
    /// `Event::Task`.
    events_tx: TokioUnboundedSender<Event>,
    events_rx: TokioUnboundedReceiver<Event>,

    /// Event channels for the latency-sensitive auxiliary loop. Used for
    /// sending log messages and joining spawned tasks.
    auxiliary_event_tx: TokioUnboundedSender<AuxiliaryEvent>,
    auxiliary_event_rx: Option<TokioUnboundedReceiver<AuxiliaryEvent>>,
}

/// State for the auxiliary loop
///
/// The auxiliary loop handles latency-sensitive events such as log messages. A
/// main loop tick might takes many milliseconds and might have a lot of events
/// in queue, so it's not appropriate for events that need immediate handling.
///
/// The auxiliary loop currently handles:
/// - Log messages.
/// - Joining of spawned blocking tasks to relay any errors or panics to the LSP log.
struct AuxiliaryState {
    client: Client,
    auxiliary_event_rx: TokioUnboundedReceiver<AuxiliaryEvent>,
    tasks: TaskList<()>,
}

impl GlobalState {
    /// Create a new global state
    ///
    /// # Arguments
    ///
    /// * `client`: The tower-lsp cient shared with the tower-lsp backend
    ///   and auxiliary loop.
    pub(crate) fn new(client: Client) -> Self {
        // Transmission channel for the main loop events. Shared with the
        // tower-lsp backend and the Jupyter kernel.
        let (events_tx, events_rx) = tokio_unbounded_channel::<Event>();

        // Transmission channels for log messages. This is handled in a separate
        // task loop for latency and ordering reasons.
        let (auxiliary_event_tx, auxiliary_event_rx) = tokio_unbounded_channel::<AuxiliaryEvent>();

        // Set global instance of this channel. This is used for logging
        // messages from a free function.
        unsafe {
            *AUXILIARY_EVENT_TX.lock().unwrap() = Some(auxiliary_event_tx.clone());
        }

        Self {
            world: WorldState::default(),
            client,
            events_tx,
            events_rx,
            auxiliary_event_tx,
            auxiliary_event_rx: Some(auxiliary_event_rx),
        }
    }

    /// Get `Event` transmission channel
    pub(crate) fn events_tx(&self) -> TokioUnboundedSender<Event> {
        self.events_tx.clone()
    }

    /// Start the main loop
    ///
    /// This takes ownership of all global state and handles one by one LSP
    /// requests, notifications, and other internal events and tasks.
    ///
    /// Returns a `JoinSet` that holds onto all tasks and state owned by the
    /// event loop. Drop it to cancel everything and shut down the service.
    pub(crate) fn main_loop(mut self) -> tokio::task::JoinSet<()> {
        let mut set = tokio::task::JoinSet::<()>::new();

        // Spawn latency-sensitive auxiliary loop
        let auxiliary_event_rx = self.auxiliary_event_rx.take().unwrap();
        AuxiliaryState::new(self.client.clone(), auxiliary_event_rx).spawn(&mut set);

        // Spawn main loop
        set.spawn(async move {
            loop {
                let event = self.next_event().await;
                if let Err(err) = self.handle_event(event).await {
                    lsp::log_error!("Failure while handling event: {err:?}")
                }
            }
        });

        set
    }

    async fn next_event(&mut self) -> Event {
        self.events_rx.recv().await.unwrap()
    }

    #[rustfmt::skip]
    /// Handle event of main loop
    ///
    /// The events are attached to _exclusive_, _sharing_, or _snapshot_
    /// handlers.
    ///
    /// - Exclusive handlers are passed an `&mut` to the world state so they can
    ///   update it.
    /// - Sharing handlers are passed a simple reference. In principle we could
    ///   run these concurrently but we typically run a handler one at a time.
    /// - When concurrent handlers are needed for performance reason (one tick
    ///   of the main loop should be as fast as possible to increase throughput)
    ///   they are spawned on blocking threads and provided a snapshot (clone) of
    ///   the state.
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Lsp(msg) => match msg {
                LspMessage::Notification(notif) => {
                    lsp::log_info!("{notif:#?}");

                    match notif {
                        LspNotification::Initialized(_params) => {
                        },
                        LspNotification::DidChangeWorkspaceFolders(_params) => {
                            // TODO: Restart indexer with new folders.
                        },
                        LspNotification::DidChangeConfiguration(_params) => {
                            // TODO: Get config updates.
                        },
                        LspNotification::DidChangeWatchedFiles(_params) => {
                            // TODO: Re-index the changed files.
                        },
                        LspNotification::DidOpenTextDocument(params) => {
                            state_handlers::did_open(params, self.events_tx(), &mut self.world)?;
                        },
                        LspNotification::DidChangeTextDocument(params) => {
                            state_handlers::did_change(params, self.events_tx(), &mut self.world)?;
                        },
                        LspNotification::DidSaveTextDocument(_params) => {
                            // Currently ignored
                        },
                        LspNotification::DidCloseTextDocument(params) => {
                            state_handlers::did_close(params, self.events_tx(), &mut self.world)?;
                        },
                    }
                },

                LspMessage::Request(request, tx) => {
                    lsp::log_info!("{request:#?}");

                    match request {
                        LspRequest::Initialize(params) => {
                            Self::respond(tx, state_handlers::initialize(params, self.state_mut()), LspResponse::Initialize)?;
                        },
                        LspRequest::Shutdown() => {
                            // TODO
                            Self::respond(tx, Ok(()), LspResponse::Shutdown)?;
                        },
                        LspRequest::WorkspaceSymbol(params) => {
                            Self::respond(tx, handlers::symbol(params), LspResponse::WorkspaceSymbol)?;
                        },
                        LspRequest::DocumentSymbol(params) => {
                            Self::respond(tx, handlers::document_symbol(params, self.state_ref()), LspResponse::DocumentSymbol)?;
                        },
                        LspRequest::ExecuteCommand(_params) => {
                            Self::respond(tx, handlers::execute_command(&self.client).await, LspResponse::ExecuteCommand)?;
                        },
                        LspRequest::Completion(params) => {
                            Self::respond(tx, handlers::completion(params, self.state_ref()), LspResponse::Completion)?;
                        },
                        LspRequest::CompletionResolve(params) => {
                            Self::respond(tx, handlers::handle_completion_resolve(params), LspResponse::CompletionResolve)?;
                        },
                        LspRequest::Hover(params) => {
                            Self::respond(tx, handlers::handle_hover(params, self.state_ref()), LspResponse::Hover)?;
                        },
                        LspRequest::SignatureHelp(params) => {
                            Self::respond(tx, handlers::handle_signature_help(params, self.state_ref()), LspResponse::SignatureHelp)?;
                        },
                        LspRequest::GotoDefinition(params) => {
                            Self::respond(tx, handlers::handle_goto_definition(params, self.state_ref()), LspResponse::GotoDefinition)?;
                        },
                        LspRequest::GotoImplementation(_params) => {
                            // TODO
                            Self::respond(tx, Ok(None), LspResponse::GotoImplementation)?;
                        },
                        LspRequest::SelectionRange(params) => {
                            Self::respond(tx, handlers::handle_selection_range(params, self.state_ref()), LspResponse::SelectionRange)?;
                        },
                        LspRequest::References(params) => {
                            Self::respond(tx, handlers::handle_references(params, self.state_ref()), LspResponse::References)?;
                        },
                        LspRequest::StatementRange(params) => {
                            Self::respond(tx, handlers::handle_statement_range(params, self.state_ref()), LspResponse::StatementRange)?;
                        },
                        LspRequest::HelpTopic(params) => {
                            Self::respond(tx, handlers::handle_help_topic(params, self.state_ref()), LspResponse::HelpTopic)?;
                        },
                    };
                },
            },

            Event::Task(task) => match task {
                LspTask::RefreshDiagnostics(url, doc, state) => {
                    let events_tx = self.events_tx();
                    self.spawn_blocking(move || handlers::refresh_diagnostics(url, doc, events_tx, state));
                },
                LspTask::RefreshAllDiagnostics() => {
                    let state = self.state_clone();
                    let events_tx = self.events_tx();
                    handlers::refresh_all_diagnostics(self, events_tx, state)?;
                },
                LspTask::PublishDiagnostics(uri, diagnostics, version) => {
                    handlers::publish_diagnostics(&self.client, uri, diagnostics, version).await?;
                },
            },

            Event::Kernel(notif) => match notif {
                KernelNotification::DidChangeConsoleInputs(inputs) => {
                    state_handlers::did_change_console_inputs(inputs, self.state_mut())?;
                },
            },
        }

        Ok(())
    }

    /// Respond to a request from the LSP
    ///
    /// We receive requests from the LSP client with a response channel.
    ///
    /// The response channel will be closed if the request has been cancelled on
    /// the tower-lsp side. In that case the future of the async request method
    /// has been dropped, along with the receiving side of this channel. It's
    /// unclear whether we want to support this sort of client-side cancellation
    /// better. We should probably focus on cancellation of expensive tasks
    /// running on side threads when the world state has changed.
    ///
    /// # Arguments
    ///
    /// * - `response_tx`: A response channel for the tower-lsp request handler.
    /// * - `response`: The response wrapped in a `anyhow::Result`.
    /// * - `into_lsp_response`: A constructor for the relevant `LspResponse` variant.
    fn respond<T>(
        response_tx: TokioUnboundedSender<anyhow::Result<LspResponse>>,
        response: anyhow::Result<T>,
        into_lsp_response: impl FnOnce(T) -> LspResponse,
    ) -> anyhow::Result<()> {
        let out = match response {
            Ok(_) => Ok(()),
            Err(ref err) => Err(anyhow!("Error while handling request: {err:?}")),
        };

        let response = response.map(into_lsp_response);

        // Ignore errors from a closed channel. This indicates the request has
        // been cancelled on the tower-lsp side.
        let _ = response_tx.send(response);

        out
    }

    /// Spawn a blocking task
    ///
    /// This runs handlers that do semantic analysis on a separate thread pool
    /// to avoid blocking the main loop.
    ///
    /// Use with care because this will cause out-of-order responses to LSP
    /// requests. This is allowed by the protocol as long as it doesn't affect
    /// the correctness of the responses. In particular, requests that respond
    /// with edits should be responded to in the order of arrival.
    ///
    /// If needed we could add a mechanism to mark handlers that must respond
    /// in order of arrival. Such a mechanism should probably not be the default
    /// because that would overly decrease the throughput of blocking tasks when
    /// a handler takes too much time.
    pub(crate) fn spawn_blocking<Handler>(&mut self, handler: Handler)
    where
        Handler: FnOnce() -> anyhow::Result<()>,
        Handler: Send + 'static,
    {
        let handle = tokio::task::spawn_blocking(|| handler());

        // Send the join handle to the auxiliary loop so it can log any errors
        // or panics as quickly as possible. Joining on the main loop would be
        // simpler but would result in confusing delays of log messages as the
        // loop might be stuck on a single tick for milliseconds at a time.
        self.auxiliary_event_tx
            .send(AuxiliaryEvent::SpawnedTask(handle))
            .unwrap();
    }

    fn state_ref(&self) -> &WorldState {
        &self.world
    }
    fn state_mut(&mut self) -> &mut WorldState {
        &mut self.world
    }
    fn state_clone(&self) -> WorldState {
        self.world.clone()
    }
}

// Needed for spawning the loop
unsafe impl Sync for AuxiliaryState {}

impl AuxiliaryState {
    fn new(client: Client, auxiliary_event_rx: TokioUnboundedReceiver<AuxiliaryEvent>) -> Self {
        let pending_tasks = futures::stream::FuturesUnordered::new();

        // Prevent the stream from ever being empty
        let pending = tokio::task::spawn(future::pending::<anyhow::Result<()>>());
        let pending = Box::pin(pending) as Pin<Box<dyn AnyhowJoinHandleFut<()> + Send>>;
        pending_tasks.push(pending);

        Self {
            client,
            auxiliary_event_rx,
            tasks: pending_tasks,
        }
    }

    /// Spawn the auxiliary loop
    ///
    /// Takes ownership of auxiliary state and spawns the low-latency auxiliary
    /// loop on the provided task set.
    fn spawn(mut self, set: &mut tokio::task::JoinSet<()>) {
        set.spawn({
            async move {
                loop {
                    match self.next_event().await {
                        AuxiliaryEvent::Log(level, message) => self.log(level, message).await,
                        AuxiliaryEvent::SpawnedTask(handle) => self.tasks.push(Box::pin(handle)),
                        AuxiliaryEvent::JoinedTask(result) => match result {
                            Err(err) => self.log_error(format!("A task panicked: {err:?}")).await,
                            Ok(Err(err)) => self.log_error(format!("A task failed: {err:?}")).await,
                            _ => (),
                        },
                    }
                }
            }
        });
    }

    async fn next_event(&mut self) -> AuxiliaryEvent {
        tokio::select! {
            event = self.auxiliary_event_rx.recv() => event.unwrap(),
            handle = self.tasks.next() => AuxiliaryEvent::JoinedTask(handle.unwrap()),
        }
    }

    async fn log(&self, level: MessageType, message: String) {
        self.client.log_message(level, message).await
    }
    async fn log_error(&self, message: String) {
        self.client.log_message(MessageType::ERROR, message).await
    }
}

/// Send a message to the LSP client. This is non-blocking and treated on a
/// latency-sensitive task.
pub(crate) fn log(level: lsp_types::MessageType, message: String) {
    if let Some(tx) = unsafe { &*AUXILIARY_EVENT_TX.lock().unwrap() } {
        // Check that channel is still alive in case the LSP was closed.
        // If closed, fallthrough.
        if let Ok(_) = tx.send(AuxiliaryEvent::Log(level.clone(), message.clone())) {
            return;
        }
    }

    // Log to the kernel as fallback
    log::warn!("LSP channel is closed, redirecting messages to Jupyter kernel");

    match level {
        MessageType::ERROR => log::error!("{message}"),
        MessageType::WARNING => log::warn!("{message}"),
        _ => log::info!("{message}"),
    };
}
