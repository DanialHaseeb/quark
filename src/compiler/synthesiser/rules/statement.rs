use super::Synthesis;
use crate::language::grammar::statement::{Kind, Statement};

impl Synthesis for Statement
{
	fn synthesise(self) -> std::string::String
	{
		match self.kind
		{
			Kind::Declaration(declaration) => declaration.synthesise(),
			Kind::Expression(expression) => expression.synthesise(),
		}
	}
}
