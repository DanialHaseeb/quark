use crate::language::grammar::{
	functions::{Parameter, Params},
	FunctionDclr, ReturnStmt,
};

use super::Synthesis;

impl Synthesis for ReturnStmt
{
	fn synthesise(self) -> String
	{
		let mut output = String::from("return ");
		output.push_str(self.expression.synthesise().as_str());
		output
	}
}

impl Synthesis for FunctionDclr
{
	fn synthesise(self) -> String
	{
		let parameters = match self.parameters
		{
			Some(params) => params.synthesise(),
			None => String::new(),
		};
		let body = self.body.synthesise();

		let mut output = String::from("def ");
		output.push_str(self.name.as_str());
		output.push('(');
		output.push_str(parameters.as_str());
		output.push(')');
		output.push_str(":{");
		output.push_str(body.as_str());
		output.push('}');
		output
	}
}

impl Synthesis for Params
{
	fn synthesise(self) -> String
	{
		self
			.parameters
			.into_iter()
			.map(|param| param.synthesise())
			.collect::<Vec<_>>()
			.join(", ")
	}
}

impl Synthesis for Parameter
{
	fn synthesise(self) -> String
	{
		self.name
	}
}
