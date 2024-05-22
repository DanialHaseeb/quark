use anyhow::Result;

use crate::compiler::parser::parse::Tree;

impl Tree
{
	/// Semantically analyses the abstract syntax tree.
	///
	/// ### Returns
	/// * The abstract syntax tree if it is semantically accurate.
	///
	/// ### Errors
	/// * If there are semantic errors in the abstract syntax tree.
	pub fn analyse(self, _source: &[Vec<char>]) -> Result<Self>
	{
		// TODO: variable rebinds for let
		// TODO: expression types matching 1 + "hi" = bad
		// TODO: function name rebinds
		// TODO: argument types maching
		// TODO: Standard lib functions for print (maybe input too)?
		// TODO: existing variable usage only
		// TODO: calling an existing function only
		// TODO: check if `continue` and `break` are inside a `while` block
		Ok(self)
	}
}
