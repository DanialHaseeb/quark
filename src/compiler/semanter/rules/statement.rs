use anyhow::{ensure, Result};

use crate::{
	compiler::semanter::table::Table,
	language::{
		grammar::{
			statement::{Kind::*, Statement},
			IfStmt, WhileStmt,
		},
		semantics::r#type::Type,
	},
};

impl Statement
{
	/// Semantically analyses the statement.
	///
	/// ### Errors
	/// * If there are semantic errors in the statement.
	pub fn analyse(&self, symbol: &mut Table) -> Result<()>
	{
		match &self.kind
		{
			Declaration(declaration) => declaration.analyse(symbol),
			Expression(expression) => expression.analyse(symbol),
			If(IfStmt { condition, .. }) | While(WhileStmt { condition, .. }) =>
			{
				ensure!(
					condition.r#type(symbol)? == Type::Boolean,
					"Invalid condition"
				);
				Ok(())
			}
			Assignment(assignment) =>
			{
				ensure!(
					symbol.variables.contains_key(&assignment.identifier),
					"Invalid assignment"
				);
				Ok(())
			}
			Echo(_) | Break(_) | Continue(_) | Return(_) | FunctionCall(_) => Ok(()),
		}
	}
}
