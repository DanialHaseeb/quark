use super::{Expression, FunctionDclr};
use crate::language::utils::Span;

/// A declaration in a Quark programme.
///
/// ### Rule
/// * _declaration_ -> _function_ | _variable_
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration
{
	/// The span of the declaration.
	pub span: Span,

	/// The kind of the declaration.
	pub kind: Kind,
}

/// The kind of a declaration.
#[derive(Debug, PartialEq, Clone)]
pub enum Kind
{
	/// A function declaration.
	Function(FunctionDclr),
	/// A variable declaration.
	///
	/// ### Rule
	/// * _variable_ -> { `let` | `var` } _identifier_ `=` _expression_
	Variable
	{
		/// The name of the variable.
		name: String,

		/// The value of the variable.
		value: Expression,

		/// Whether the variable is mutable.
		is_mutable: bool,
	},
}
