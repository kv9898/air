//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use air_r_syntax::{
    RSyntaxElement as SyntaxElement, RSyntaxNode as SyntaxNode, RSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn r_argument() -> RArgumentBuilder {
    RArgumentBuilder {
        name_clause: None,
        value: None,
    }
}
pub struct RArgumentBuilder {
    name_clause: Option<RArgumentNameClause>,
    value: Option<AnyRExpression>,
}
impl RArgumentBuilder {
    pub fn with_name_clause(mut self, name_clause: RArgumentNameClause) -> Self {
        self.name_clause = Some(name_clause);
        self
    }
    pub fn with_value(mut self, value: AnyRExpression) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> RArgument {
        RArgument::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_ARGUMENT,
            [
                self.name_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn r_argument_name_clause(
    name: AnyRArgumentName,
    eq_token: SyntaxToken,
) -> RArgumentNameClause {
    RArgumentNameClause::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_ARGUMENT_NAME_CLAUSE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
        ],
    ))
}
pub fn r_binary_expression(
    left: AnyRExpression,
    operator_token: SyntaxToken,
    right: AnyRExpression,
) -> RBinaryExpression {
    RBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn r_braced_expressions(
    l_curly_token: SyntaxToken,
    expressions: RExpressionList,
    r_curly_token: SyntaxToken,
) -> RBracedExpressions {
    RBracedExpressions::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BRACED_EXPRESSIONS,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(expressions.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn r_break_expression(break_token: SyntaxToken) -> RBreakExpression {
    RBreakExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BREAK_EXPRESSION,
        [Some(SyntaxElement::Token(break_token))],
    ))
}
pub fn r_call(function: AnyRExpression, arguments: RCallArguments) -> RCall {
    RCall::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_CALL,
        [
            Some(SyntaxElement::Node(function.into_syntax())),
            Some(SyntaxElement::Node(arguments.into_syntax())),
        ],
    ))
}
pub fn r_call_arguments(
    l_paren_token: SyntaxToken,
    items: RArgumentList,
    r_paren_token: SyntaxToken,
) -> RCallArguments {
    RCallArguments::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_CALL_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn r_complex_value(value_token: SyntaxToken) -> RComplexValue {
    RComplexValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_COMPLEX_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_dot_dot_i(value_token: SyntaxToken) -> RDotDotI {
    RDotDotI::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOT_DOT_I,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_dots(value_token: SyntaxToken) -> RDots {
    RDots::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOTS,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_double_value(value_token: SyntaxToken) -> RDoubleValue {
    RDoubleValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOUBLE_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_else_clause(else_token: SyntaxToken, alternative: AnyRExpression) -> RElseClause {
    RElseClause::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(alternative.into_syntax())),
        ],
    ))
}
pub fn r_extract_expression(
    left: AnyRExpression,
    operator_token: SyntaxToken,
    right: AnyRSelector,
) -> RExtractExpression {
    RExtractExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_EXTRACT_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn r_false_expression(false_token: SyntaxToken) -> RFalseExpression {
    RFalseExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_FALSE_EXPRESSION,
        [Some(SyntaxElement::Token(false_token))],
    ))
}
pub fn r_for_statement(
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    variable: RIdentifier,
    in_token: SyntaxToken,
    sequence: AnyRExpression,
    r_paren_token: SyntaxToken,
    body: AnyRExpression,
) -> RForStatement {
    RForStatement::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_FOR_STATEMENT,
        [
            Some(SyntaxElement::Token(for_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(variable.into_syntax())),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(sequence.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_function_definition(
    name_token: SyntaxToken,
    parameters: RParameters,
    body: AnyRExpression,
) -> RFunctionDefinition {
    RFunctionDefinition::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_FUNCTION_DEFINITION,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_identifier(name_token: SyntaxToken) -> RIdentifier {
    RIdentifier::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_IDENTIFIER,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn r_if_statement(
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnyRExpression,
    r_paren_token: SyntaxToken,
    consequence: AnyRExpression,
) -> RIfStatementBuilder {
    RIfStatementBuilder {
        if_token,
        l_paren_token,
        condition,
        r_paren_token,
        consequence,
        else_clause: None,
    }
}
pub struct RIfStatementBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnyRExpression,
    r_paren_token: SyntaxToken,
    consequence: AnyRExpression,
    else_clause: Option<RElseClause>,
}
impl RIfStatementBuilder {
    pub fn with_else_clause(mut self, else_clause: RElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> RIfStatement {
        RIfStatement::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_IF_STATEMENT,
            [
                Some(SyntaxElement::Token(self.if_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.condition.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.consequence.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn r_inf_expression(inf_token: SyntaxToken) -> RInfExpression {
    RInfExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_INF_EXPRESSION,
        [Some(SyntaxElement::Token(inf_token))],
    ))
}
pub fn r_integer_value(value_token: SyntaxToken) -> RIntegerValue {
    RIntegerValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_INTEGER_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_na_expression(value_token: SyntaxToken) -> RNaExpression {
    RNaExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NA_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_namespace_expression(
    left: AnyRSelector,
    operator_token: SyntaxToken,
    right: AnyRSelector,
) -> RNamespaceExpression {
    RNamespaceExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NAMESPACE_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn r_nan_expression(nan_token: SyntaxToken) -> RNanExpression {
    RNanExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NAN_EXPRESSION,
        [Some(SyntaxElement::Token(nan_token))],
    ))
}
pub fn r_next_expression(next_token: SyntaxToken) -> RNextExpression {
    RNextExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NEXT_EXPRESSION,
        [Some(SyntaxElement::Token(next_token))],
    ))
}
pub fn r_null_expression(null_token: SyntaxToken) -> RNullExpression {
    RNullExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NULL_EXPRESSION,
        [Some(SyntaxElement::Token(null_token))],
    ))
}
pub fn r_parameter(name: AnyRParameterName) -> RParameterBuilder {
    RParameterBuilder {
        name,
        default: None,
    }
}
pub struct RParameterBuilder {
    name: AnyRParameterName,
    default: Option<RParameterDefault>,
}
impl RParameterBuilder {
    pub fn with_default(mut self, default: RParameterDefault) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> RParameter {
        RParameter::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_PARAMETER,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn r_parameter_default(eq_token: SyntaxToken, value: AnyRExpression) -> RParameterDefault {
    RParameterDefault::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_PARAMETER_DEFAULT,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn r_parameters(
    l_paren_token: SyntaxToken,
    items: RParameterList,
    r_paren_token: SyntaxToken,
) -> RParameters {
    RParameters::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn r_parenthesized_expression(
    l_paren_token: SyntaxToken,
    body: AnyRExpression,
    r_paren_token: SyntaxToken,
) -> RParenthesizedExpression {
    RParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn r_repeat_statement(repeat_token: SyntaxToken, body: AnyRExpression) -> RRepeatStatement {
    RRepeatStatement::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_REPEAT_STATEMENT,
        [
            Some(SyntaxElement::Token(repeat_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_return_expression(return_token: SyntaxToken) -> RReturnExpression {
    RReturnExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_RETURN_EXPRESSION,
        [Some(SyntaxElement::Token(return_token))],
    ))
}
pub fn r_root(expressions: RExpressionList, eof_token: SyntaxToken) -> RRootBuilder {
    RRootBuilder {
        expressions,
        eof_token,
        bom_token: None,
    }
}
pub struct RRootBuilder {
    expressions: RExpressionList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl RRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> RRoot {
        RRoot::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.expressions.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn r_string_value(value_token: SyntaxToken) -> RStringValue {
    RStringValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_STRING_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_subset(function: AnyRExpression, arguments: RSubsetArguments) -> RSubset {
    RSubset::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_SUBSET,
        [
            Some(SyntaxElement::Node(function.into_syntax())),
            Some(SyntaxElement::Node(arguments.into_syntax())),
        ],
    ))
}
pub fn r_subset2(function: AnyRExpression, arguments: RSubset2Arguments) -> RSubset2 {
    RSubset2::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_SUBSET2,
        [
            Some(SyntaxElement::Node(function.into_syntax())),
            Some(SyntaxElement::Node(arguments.into_syntax())),
        ],
    ))
}
pub fn r_subset2_arguments(
    l_brack2_token: SyntaxToken,
    items: RArgumentList,
    r_brack2_token: SyntaxToken,
) -> RSubset2Arguments {
    RSubset2Arguments::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_SUBSET2_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_brack2_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_brack2_token)),
        ],
    ))
}
pub fn r_subset_arguments(
    l_brack_token: SyntaxToken,
    items: RArgumentList,
    r_brack_token: SyntaxToken,
) -> RSubsetArguments {
    RSubsetArguments::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_SUBSET_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn r_true_expression(true_token: SyntaxToken) -> RTrueExpression {
    RTrueExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_TRUE_EXPRESSION,
        [Some(SyntaxElement::Token(true_token))],
    ))
}
pub fn r_unary_expression(
    operator_token: SyntaxToken,
    argument: AnyRExpression,
) -> RUnaryExpression {
    RUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn r_while_statement(
    while_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnyRExpression,
    r_paren_token: SyntaxToken,
    body: AnyRExpression,
) -> RWhileStatement {
    RWhileStatement::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_WHILE_STATEMENT,
        [
            Some(SyntaxElement::Token(while_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_argument_list<I, S>(items: I, separators: S) -> RArgumentList
where
    I: IntoIterator<Item = RArgument>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = RSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    RArgumentList::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_ARGUMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn r_expression_list<I>(items: I) -> RExpressionList
where
    I: IntoIterator<Item = AnyRExpression>,
    I::IntoIter: ExactSizeIterator,
{
    RExpressionList::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_EXPRESSION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn r_parameter_list<I, S>(items: I, separators: S) -> RParameterList
where
    I: IntoIterator<Item = RParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = RSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    RParameterList::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn r_bogus<I>(slots: I) -> RBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogus::unwrap_cast(SyntaxNode::new_detached(RSyntaxKind::R_BOGUS, slots))
}
pub fn r_bogus_expression<I>(slots: I) -> RBogusExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogusExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BOGUS_EXPRESSION,
        slots,
    ))
}
pub fn r_bogus_value<I>(slots: I) -> RBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogusValue::unwrap_cast(SyntaxNode::new_detached(RSyntaxKind::R_BOGUS_VALUE, slots))
}
