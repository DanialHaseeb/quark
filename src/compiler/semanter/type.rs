use anyhow::Result;

use crate::language::lexicon::token;
use crate::language::{
	grammar::expression::{Expression, Kind::*},
	semantics::r#type::Type,
};

use super::table::Table;

impl Expression
{
	pub fn r#type(&self, symbol: &Table) -> Result<Type>
	{
		self.analyse(symbol)?;

		match &self.kind
		{
			Identifier(token) =>
			{
				let name = match &token.kind
				{
					token::Kind::Identifier(name) => name.clone(),
					_ => unreachable!(),
				};

				if symbol.constants.contains_key(&name)
				{
					Ok(symbol.constants[&name])
				}
				else
				{
					Ok(symbol.variables[&name])
				}
			}

			Literal(token) => match token.kind
			{
				token::Kind::Boolean(_) => Ok(Type::Boolean),
				token::Kind::Number(_) => Ok(Type::Number),
				token::Kind::StringQ(_) => Ok(Type::String),
				_ => unreachable!(),
			},

			Parenthesised(expression) => expression.r#type(symbol),

			Prefix { operand, .. } => operand.r#type(symbol),

			Infix { left, .. } => left.r#type(symbol),

			FunctionCall(function) => Ok(symbol.functions[&function.name]),

			_ => Ok(Type::Unit),
		}
	}
}
