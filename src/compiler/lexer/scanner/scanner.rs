use std::iter::Peekable;

use anyhow::Result;

use crate::language::lexicon::{Token, Symbol};

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
