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
			_ if value.is_whitespace() => Token::try_from_whitespace(stream, source)?,

			// If the next symbol potentially starts a comment.
			'/' => Token::try_from_comment_head(stream, source)?,

			// If the next symbol potentially starts an identifier.
			_ if value.is_alphabetic() => Token::from_identifier_head(stream),
			'_' => Token::from_identifier_head(stream),

			// If the next symbol potentially starts a number.
			_ if value.is_ascii_digit() => Token::from_number_head(stream, source),
			'.' => Token::from_number_head(stream, source),

			// If the next symbol potentially starts a string.
			'"' => Token::try_from_string_head(stream, source)?,

			// If the next symbol is a delimiter.
			'(' | ')' => Token::from_delimiter(stream),
			'[' | ']' => Token::from_delimiter(stream),
			'{' | '}' => Token::from_delimiter(stream),

			// If the next symbol potentially starts an operator.
			'+' | '-' | '*' | '%' | '^' => Token::from_operator_head(stream),
			'=' | '!' | '<' | '>' => Token::from_operator_head(stream),

			// If it is any other symbol.
			_ => Token::try_from_symbol(stream, source)?,
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
