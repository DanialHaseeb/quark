use crate::language::utils::Span;

use super::{Block, Expression};

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt
{
	pub span: Span,
	pub expression: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDclr
{
	pub span: Span,
	pub name: String,
	pub body: Block,
	pub return_type: ReturnType,
	pub parameters: Option<Params>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnType
{
	Number,
	String,
	Bool,
	Unit,
	Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter
{
	pub span: Span,
	pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Params
{
	pub span: Span,
	pub parameters: Vec<Parameter>,
}
