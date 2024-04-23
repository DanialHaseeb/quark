use anyhow::Result;

use crate::language::{token::Token, utils::Symbol};

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

impl Symbol
{
	/// Creates a vector of symbols.
	///
	/// ### Parameters
	/// * `line` - The line number of each symbol in the vector.
	/// * `string` - The string of characters to be converted.
	///
	/// ### Returns
	/// * A vector of symbols created from the given string.
	///
	/// ### Examples
	/// ```rust
	/// use quark::language::Symbol;
	///
	/// let symbols = Symbol::vector(0, "abc");
	///
	/// assert_eq!(symbols.len(), 3);
	/// assert_eq!(symbols[0].value, 'a');
	/// assert_eq!(symbols[1].value, 'b');
	/// assert_eq!(symbols[2].value, 'c');
	/// ```
	pub fn vector((line, string): (usize, &str)) -> Vec<Self>
	{
		format!("{string}\n")
			.chars()
			.enumerate()
			.map(|(column, value)| Symbol::new(line, column, value))
			.collect()
	}
}
