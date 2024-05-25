use crate::language::utils::Span;

use super::Expression;

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

	/// A `If` Conditional statemtn
	If(super::IfStmt),

	/// A `While` Conditional statemtent
	While(super::WhileStmt),

	/// A break statement
	Break(super::BreakStmt),

	/// A break statement
	Continue(super::ContinueStmt),

	Return(super::ReturnStmt),

	Assignment(super::AssignmentStmt),

	Echo(EchoStmt),

	FunctionCall(super::FunctionCall),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block
{
	pub span: Span,
	pub statements: Vec<Option<Statement>>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct EchoStmt
{
	pub span: Span,
	pub arguments: Vec<Expression>,
}
