use anyhow::Result;

use crate::compiler::parser::Programme;

use super::super::generator::{CodeGenerator, python};

/// Types that can be synthesised.
///
/// This trait is used to convert an abstract syntax tree into Python code.
pub trait Synthesise
{
	/// Synthesises the abstract syntax tree into Python code.
	///
	/// ### Parameters
	/// * `source` - The Quark source code.
	///
	/// ### Returns
	/// * The output Python code.
	///
	/// ### Errors
	/// * If the abstract syntax tree cannot be synthesised.
	fn synthesise(&self, source: &[Vec<char>]) -> Result<String>;
}

impl Synthesise for Programme
{
	fn synthesise(&self, source: &[Vec<char>]) -> Result<String>
	{
		Ok(python(self.generate()))
	}
}
