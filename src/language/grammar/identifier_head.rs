use crate::language::utils::Span;

use super::{expression::Items, Expression};

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentStmt
{
	pub span: Span,
	pub identifier: String,
	pub expression: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall
{
	pub span: Span,
	pub name: String,
	pub arguments: Option<Items>,
}
