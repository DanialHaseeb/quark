use anyhow::Result;

use crate::{compiler::semanter::table::Table, language::grammar::Programme};

impl Programme
{
	/// Semantically analyses the abstract syntax tree.
	///
	/// ### Errors
	/// * If there are semantic errors in the abstract syntax tree.
	pub fn analyse(&self) -> Result<()>
	{
		let mut symbol: Table = Table::new();

		for statement in &self.statements
		{
			statement.analyse(&mut symbol)?;
		}

		Ok(())
	}
}
