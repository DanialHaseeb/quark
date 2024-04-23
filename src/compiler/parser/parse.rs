use anyhow::Result;

use crate::compiler::parser::parse;
use crate::language::grammar::Programme;

/// Types that can be parsed.
///
/// This trait is used to convert Quark lexical tokens into an abstract syntax
/// tree.
pub trait Parse
{
	/// Parses the Quark code into an abstract syntax tree.
	///
	/// ### Parameters
	/// * `source` - The source code to parse.
	///
	/// ### Returns
	/// * The abstract syntax tree.
	///
	/// ### Errors
	/// * If the Quark code cannot be parsed.
	fn parse(self, source: &[Vec<char>]) -> Result<parse::Tree>;
}

/// The abstract syntax tree.
///
/// This tree represents the structure of the Quark source code.
pub struct Tree(Programme);
