use super::Synthesis;
use crate::language::grammar::statement::{Kind, Statement};

impl Synthesis for Statement
{
	fn synthesise(self) -> std::string::String
	{
		match self.kind
		{
			Kind::Declaration(declaration) =>
			{
				format!("{}{}", declaration.synthesise(), ";")
			}
			Kind::Expression(expression) =>
			{
				format!("{}{}", expression.synthesise(), ";")
			}
			Kind::If(if_) => if_.synthesise(),
			Kind::While(while_) => while_.synthesise(),
			Kind::Continue(continue_) => format!("{}{}", continue_.synthesise(), ";"),
			Kind::Break(break_) => format!("{}{}", break_.synthesise(), ";"),
		}
	}
}
