use anyhow::Result;

use super::*;

/// Types that can be lexed.
///
/// This trait is used to convert Quark source code into lexical tokens.
pub trait Lex
{
	/// Lexes the Quark source code into tokens.
	///
	/// ### Parameters
	/// * `source` - The Quark source code.
	///
	/// ### Returns
	/// * The lexical tokens.
	///
	/// ### Errors
	/// * If the Quark source code cannot be lexed.
	fn lex(self, source: &[Vec<char>]) -> Result<Vec<Token>>;
}

impl Lex for String
{
	fn lex(self, _source: &[Vec<char>]) -> Result<Vec<Token>>
	{
		super::lex(self)
	}
}
