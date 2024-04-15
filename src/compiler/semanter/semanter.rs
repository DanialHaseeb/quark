use anyhow::Result;

use crate::compiler::parser::Programme;

pub trait Analyse
{
	/// Analyses the abstract syntax tree.
	///
	/// ### Parameters
	/// * `source` - The Quark source code.
	///
	/// ### Returns
	/// * The abstract syntax tree.
	///
	/// ### Errors
	/// * If the abstract syntax tree cannot be analysed.
	fn analyse(self, source: &[Vec<char>]) -> Result<Programme>;
}

impl Analyse for Programme
{
	fn analyse(self, _source: &[Vec<char>]) -> Result<Programme>
	{
		Ok(self)
	}
}
