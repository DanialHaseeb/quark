use super::Synthesis;
use crate::language::grammar::expression::{Expression, Kind};
use crate::language::lexicon::token::Kind::*;

impl Synthesis for Expression
{
	fn synthesise(self) -> std::string::String
	{
		match self.kind
		{
			Kind::Identifier(token) => match token.kind
			{
				Identifier(name) => name,
				_ => unreachable!(),
			},

			Kind::Literal(token) => match token.kind
			{
				Number(value) => value,
				String(value) => format!("\"{value}\""),
				Bool(true) => "True".to_string(),
				Bool(false) => "False".to_string(),
				_ => unreachable!(),
			},

			Kind::Parenthesised(expression) =>
			{
				let inner = expression.synthesise();
				format!("({inner})")
			}

			Kind::Prefix { operator, operand } =>
			{
				let operator = match operator.kind
				{
					Plus => "+",
					Minus => "-",
					Not => "not ",
					_ => unreachable!(),
				};

				let operand = operand.synthesise();

				format!("{operator}{operand}")
			}

			Kind::Infix {
				left,
				operator,
				right,
			} =>
			{
				let left = left.synthesise();

				let operator = match operator.kind
				{
					Plus => "+",
					Minus => "-",
					Asterisk => "*",
					Slash => "/",
					Percent => "%",
					Caret => "**",
					And => "and",
					Or => "or",
					Equal => "==",
					ExclaimEqual => "!=",
					Less => "<",
					LessEqual => "<=",
					Greater => ">",
					GreaterEqual => ">=",
					_ => unreachable!(),
				};

				let right = right.synthesise();

				format!("{left} {operator} {right}")
			}
		}
	}
}
