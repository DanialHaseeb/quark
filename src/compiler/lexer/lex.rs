use std::iter::Peekable;

use anyhow::Result;

use crate::language::lexicon::{Symbol, Token};

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

impl Token
{
	/// Creates a token from a stream of symbols.
	///
	/// ### Parameters
	/// * `stream` - The stream of symbols.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The next token if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	///
	/// ### Errors
	/// * If the token cannot be created.
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Self>>
	where
		I: Iterator<Item = Symbol>,
	{
		let value = match stream.peek()
		{
			Some(symbol) => symbol.value,
			None => return Ok(None),
		};

		let token = match value
		{
			// If the next symbol is whitespace.
			_ if value.is_whitespace() => Self::try_from_whitespace(stream, source)?,

			// If the next symbol potentially starts a comment.
			'/' => Self::try_from_comment_head(stream, source)?,

			// If the next symbol potentially starts an identifier.
			_ if value.is_alphabetic() => Token::from_identifier_head(stream),
			'_' => Self::from_identifier_head(stream),

			// If the next symbol potentially starts a number.
			_ if value.is_ascii_digit() => Self::from_number_head(stream, source),
			'.' => Self::from_number_head(stream, source),

			// If the next symbol potentially starts a string.
			'"' => Self::try_from_string_head(stream, source)?,

			// If the next symbol is a delimiter.
			'(' | ')' => Self::from_delimiter(stream),
			'[' | ']' => Self::from_delimiter(stream),
			'{' | '}' => Self::from_delimiter(stream),

			// If the next symbol potentially starts an operator.
			'+' | '-' | '*' | '%' | '^' => Self::from_operator_head(stream),
			'=' | '!' | '<' | '>' => Self::from_operator_head(stream),

			// If it is any other symbol.
			_ => Self::try_from_symbol(stream, source)?,
		};

		Ok(token)
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
	pub fn vector((line, string): (usize, &str)) -> Vec<Self>
	{
		format!("{string}\n")
			.chars()
			.enumerate()
			.map(|(column, value)| Self::new(line, column, value))
			.collect()
	}
}
