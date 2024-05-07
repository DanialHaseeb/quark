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

			Kind::List(items) =>
			{
				let items = items
					.expressions
					.into_iter()
					.map(|expr| expr.synthesise())
					.collect::<Vec<_>>()
					.join(", ");
				format!("[{items}]")
			}

			Kind::Matrix(items_list) =>
			{
				let mut output = "np.array([".to_string();
				for items in items_list.into_iter()
				{
					output.push('[');
					for expression in items.expressions.into_iter()
					{
						output.push_str(&expression.synthesise());
						output.push(',');
					}
					output.push(']');
				}
				output.push(']'); // Add this to close the outer array

				output
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
