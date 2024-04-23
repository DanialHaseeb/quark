use crate::language::grammar::Statement;
use crate::language::utils::Span;

/// A Quark programme.
///
/// ### Rule
/// * _programme_ -> _statement_*
pub struct Programme
{
	/// The span of the programme.
	pub span: Span,

	/// The statements in the programme.
	pub statements: Vec<Statement>,
}
