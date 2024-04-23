use crate::language::utils::Span;

/// An expression in a Quark programme.
///
/// ### Rule
/// * _expression_ -> _primary_ | _prefix_ | _infix_
/// * _primary_ -> _literal_ | _identifier_ | `(` _expression_ `)`
pub struct Expression
{
	/// The span of the expression.
	span: Span,

	/// The kind of the expression.
	kind: self::Kind,
}

/// The kind of an expression.
pub enum Kind {}
