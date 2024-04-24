use crate::language::{lexicon::Token, utils::Span};

/// An expression in a Quark programme.
///
/// ### Rule
/// * _expression_ -> _primary_ | _prefix_ | _infix_
/// * _primary_ -> _literal_ | _identifier_ | `(` _expression_ `)`
#[derive(Debug, PartialEq, Clone)]
pub struct Expression
{
	/// The span of the expression.
	pub span: Span,

	/// The kind of the expression.
	pub kind: Kind,
}

/// The kind of an expression.
#[derive(Debug, PartialEq, Clone)]
pub enum Kind
{
	/// An identifier expression.
	///
	/// ### Rule
	/// * _identifier_ -> _letter_ | `_` { _letter_ | _digit_ | `_` }*
	/// * _letter_ -> `a`..`z` | `A`..`Z`
	/// * _digit_ -> `0`..`9`
	Identifier(String),

	/// A literal expression.
	/// a
	/// ### Rule
	/// * _literal_ -> _string_ | _number_ | _boolean_
	Literal(Token),

	/// A parenthesised expression.
	///
	/// ### Rule
	/// * _parenthesised_ -> `(` _expression_ `)`
	Parenthesised(Box<Expression>),

	/// A unary prefix expression.
	///
	/// ### Rule
	/// * _prefix_ -> _operator_ _expression_
	Prefix
	{
		/// The operator of the prefix expression.
		operator: Token,
		/// The operand of the prefix expression.
		operand: Box<Expression>,
	},

	/// A binary infix expression.
	///
	/// ### Rule
	/// * _infix_ -> _expression_ _operator_ _expression_
	Infix
	{
		/// The left operand of the infix expression.
		left: Box<Expression>,
		/// The operator of the infix expression.
		operator: Token,
		/// The right operand of the infix expression.
		right: Box<Expression>,
	},
}
