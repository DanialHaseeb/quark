use super::{Block, Expression};
use crate::language::utils::Span;

/// A If statement in a Quark programme.
///
/// ### Rule
/// * _statement_ -> { _declaration_ | _expression_ } `;`
#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt
{
	/// The span of the statement.
	pub span: Span,
	/// The condition expression
	pub condition: Expression,
	/// The body of the if statement
	pub body: Block,

	/// The else of the if statement
	pub else_body: Option<Block>,
}

/// A While statement in a Quark programme.
///
/// ### Rule
#[derive(Debug, PartialEq, Clone)]
pub struct WhileStmt
{
	/// The span of the statement.
	pub span: Span,
	/// The condition expression
	pub condition: Expression,
	/// The body of the if statement
	pub body: Block,
}

/// A Break statement in a Quark programme.
///
/// ### Rule
#[derive(Debug, PartialEq, Clone)]
pub struct BreakStmt
{
	/// The span of the statement.
	pub span: Span,
}

/// A Contine statement in a Quark programme.
///
/// ### Rule
#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStmt
{
	/// The span of the statement.
	pub span: Span,
}
