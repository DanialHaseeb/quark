use crate::language::grammar::{AssignmentStmt, FunctionCall};

use super::Synthesis;

impl Synthesis for AssignmentStmt
{
	fn synthesise(self) -> std::string::String
	{
		format!("{} = {}", self.identifier, self.expression.synthesise())
	}
}

impl Synthesis for FunctionCall
{
	fn synthesise(self) -> std::string::String
	{
		let arguments = match self.arguments
		{
			Some(args) => args
				.expressions
				.into_iter()
				.map(|expression| expression.synthesise())
				.collect::<Vec<_>>()
				.join(", "),
			None => String::new(),
		};

		format!("{}({})", self.name, arguments)
	}
}
