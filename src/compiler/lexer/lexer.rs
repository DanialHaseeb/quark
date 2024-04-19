use anyhow::Result;

use crate::language::{Symbol, Token};

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
	fn lex(self, source: &[Vec<char>]) -> Result<Vec<Token>>
	{
		let mut stream =
			self.lines().enumerate().flat_map(Symbol::vector).peekable();

		let mut tokens = Vec::new();

		while let Some(token) = Token::try_from_stream(&mut stream, source)?
		{
			tokens.push(token);
		}

		Ok(tokens)
	}
}
