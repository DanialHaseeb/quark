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
		output.push_str(self.body.synthesise().as_str());
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
		output.push_str(self.body.synthesise().as_str());
		output.push('}');

		if let Some(else_body) = self.else_body
		{
			output.push_str("else:{");
			output.push_str(else_body.synthesise().as_str());
			output.push('}');
		}
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
