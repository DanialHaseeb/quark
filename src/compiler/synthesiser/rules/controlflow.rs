use super::Synthesis;
use crate::language::grammar::controlflow::{
	BreakStmt, ContinueStmt, IfStmt, WhileStmt,
};

impl Synthesis for WhileStmt
{
	fn synthesise(self) -> std::string::String
	{
		let mut output = String::from("while ");
		output.push_str(self.condition.synthesise().as_str());
		output.push_str(":{");
		for statement in self.body.statements
		{
			match statement
			{
				Some(statement) => output.push_str(statement.synthesise().as_str()),
				None => continue,
			}
		}

		output.push('}');
		output
	}
}

impl Synthesis for IfStmt
{
	fn synthesise(self) -> String
	{
		let mut output = String::from("if ");
		output.push_str(self.condition.synthesise().as_str());
		output.push_str(":{");

		for statement in self.body.statements
		{
			match statement
			{
				Some(statement) => output.push_str(statement.synthesise().as_str()),
				None => continue,
			}
		}

		output.push('}');
		output
	}
}

impl Synthesis for ContinueStmt
{
	fn synthesise(self) -> String
	{
		String::from("continue")
	}
}

impl Synthesis for BreakStmt
{
	fn synthesise(self) -> String
	{
		String::from("break")
	}
}
