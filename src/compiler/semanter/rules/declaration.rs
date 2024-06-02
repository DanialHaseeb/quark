use anyhow::{bail, Result};

use crate::{
	compiler::semanter::table::Table,
	language::grammar::declaration::{Declaration, Kind::*},
};

impl Declaration
{
	/// Semantically analyses the declaration.
	///
	/// ### Errors
	/// * If there are semantic errors in the declaration.
	pub fn analyse(&self, symbol: &mut Table) -> Result<()>
	{
		match &self.kind
		{
			Variable {
				name,
				value,
				is_mutable,
			} =>
			{
				if *is_mutable
					&& symbol
						.variables
						.insert(name.clone(), value.r#type(symbol)?)
						.is_some()
				{
					bail!("Variable '{}' already declared", name)
				}

				if !is_mutable
					&& symbol
						.constants
						.insert(name.clone(), value.r#type(symbol)?)
						.is_some()
				{
					bail!("Constant '{}' already declared", name)
				}
			}

			Function(function) =>
			{
				if symbol
					.functions
					.insert(function.name.clone(), function.return_type)
					.is_some()
				{
					bail!("Function '{}' already declared", function.name)
				}
			}
		};

		Ok(())
	}
}
