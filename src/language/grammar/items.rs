use super::Expression;
use crate::language::utils::Span;

#[derive(Debug, PartialEq, Clone)]
/// An list of items in a Quark programme.
///
/// ### Rule
/// * _items_ -> _expression_ { `,` _expression_ }*
pub struct Items
{
	/// The span of the expression.
	pub span: Span,

	/// The comma separated items.
	pub expressions: Vec<Expression>,
}
