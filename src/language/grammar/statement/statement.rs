use crate::language::utils::Span;

/// A statement in a Quark programme.
///
/// ### Rule
/// * _statement_ -> { _declaration_ | _expression_ } `;`
pub struct Statement
{
	/// The span of the statement.
	span: Span,

	/// The kind of the statement.
	kind: Kind,
}

/// The kind of a statement.
pub enum Kind
{
	/// A declaration statement.
	Declaration(super::Declaration),

	/// An expression statement.
	Expression(super::Expression),
}
