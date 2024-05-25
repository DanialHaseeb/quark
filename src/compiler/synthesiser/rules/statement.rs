use super::Synthesis;
use crate::language::grammar::{
	statement::{EchoStmt, Kind, Statement},
	Block,
};

impl Synthesis for Statement
{
	fn synthesise(self) -> std::string::String
	{
		match self.kind
		{
			Kind::If(if_) => if_.synthesise(),
			Kind::While(while_) => while_.synthesise(),
			Kind::Declaration(declaration) => (declaration.synthesise()),

			Kind::Expression(expression) =>
			{
				format!("{}{}", expression.synthesise(), ";")
			}
			Kind::Continue(continue_) => format!("{}{}", continue_.synthesise(), ";"),
			Kind::Break(break_) => format!("{}{}", break_.synthesise(), ";"),
			Kind::Return(return_) => format!("{}{}", return_.synthesise(), ";"),
			Kind::Echo(echo) => format!("{}{}", echo.synthesise(), ";"),
			Kind::FunctionCall(function_call) =>
			{
				format!("{}{}", function_call.synthesise(), ";")
			}
			Kind::Assignment(assignment) =>
			{
				format!("{}{}", assignment.synthesise(), ";")
			}
		}
	}
}

impl Synthesis for Block
{
	fn synthesise(self) -> std::string::String
	{
		self
			.statements
			.into_iter()
			.flatten()
			.map(|statement| statement.synthesise())
			.collect::<Vec<_>>()
			.join("")
	}
}

impl Synthesis for EchoStmt
{
	fn synthesise(self) -> std::string::String
	{
		format!(
			"print({})",
			self
				.arguments
				.into_iter()
				.map(|arg| arg.synthesise())
				.collect::<Vec<_>>()
				.join(", ")
		)
	}
}
