use anyhow::Result;

use super::super::lexer::token::Token;
use super::Programme;

/// Types that can be parsed.
///
/// This trait is used to convert lexical tokens into an abstract syntax tree.
pub trait Parse
{
	/// Parses the lexical tokens into an abstract syntax tree.
	///
	/// ### Parameters
	/// * `source` - The Quark source code.
	///
	/// ### Returns
	/// * The abstract syntax tree.
	///
	/// ### Errors
	/// * If the lexical tokens cannot be parsed.
	fn parse(self, source: &[Vec<char>]) -> Result<Programme>;
}

impl Parse for Vec<Token>
{
	fn parse(self, source: &[Vec<char>]) -> Result<Programme>
	{
		super::parse(self)
	}
}
