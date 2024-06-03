use anyhow::{ensure, Result};

use crate::language::lexicon::token::Kind::*;
use crate::{
	compiler::semanter::table::Table,
	language::{
		grammar::expression::{Expression, Kind::*},
		semantics::r#type::Type,
	},
};

impl Expression
{
	/// Semantically analyses the expression.
	///
	/// ### Errors
	/// * If there are semantic errors in the expression.
	pub fn analyse(&self, symbol: &Table) -> Result<()>
	{
		match &self.kind
		{
			FunctionCall(function) =>
			{
				ensure!(
					symbol.functions.contains_key(&function.name),
					"Invalid function call"
				);
				Ok(())
			}

			Prefix { operator, operand } => match operator.kind
			{
				Plus | Minus =>
				{
					ensure!(operand.r#type(symbol)? == Type::Number, "Invalid operand");
					Ok(())
				}
				Not =>
				{
					ensure!(operand.r#type(symbol)? == Type::Boolean, "Invalid operand");
					Ok(())
				}
				_ => unreachable!(),
			},

			Infix {
				operator,
				left,
				right,
			} =>
			{
				let left = left.r#type(symbol)?;
				let right = right.r#type(symbol)?;

				match operator.kind
				{
					Plus => ensure!(
						left == Type::Number || left == Type::String,
						"Invalid left operand"
					),
					Minus | Asterisk | Slash | Greater | GreaterEqual | Less | LessEqual | EqualEqual | ExclaimEqual => ensure!(left == Type::Number, "Invalid left operand"),
					And | Or | Xor =>
					{
						ensure!(left == Type::Boolean, "Invalid left operand")
					}
					_ => unreachable!(),
				};

				ensure!(left == right, "Invalid right operand");
				Ok(())
			}

			_ => Ok(()),
		}
	}
}
