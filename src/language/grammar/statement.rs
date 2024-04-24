use crate::language::utils::Span;

/// A statement in a Quark programme.
///
/// ### Rule
/// * _statement_ -> { _declaration_ | _expression_ } `;`
#[derive(Debug, PartialEq, Clone)]
pub struct Statement
{
	/// The span of the statement.
	pub span: Span,

	/// The kind of the statement.
	pub kind: Kind,
}

/// The kind of a statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Kind
{
	/// A declaration statement.
	Declaration(super::Declaration),

	/// An expression statement.
	Expression(super::Expression),
}

impl Kind
{
	/// Determines the span of the underlying kind of statement.
	///
	/// ### Returns
	/// * The span of the underlying kind of statement.
	pub fn span(&self) -> Span
	{
		match self
		{
			Self::Declaration(declaration) => declaration.span,
			Self::Expression(expression) => expression.span,
		}
	}
}
