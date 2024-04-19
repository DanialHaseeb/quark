use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::language::{Symbol, Token};

impl Token
{
	/// Converts a stream of symbols into a token.
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
	pub fn from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Self>>
	where
		I: Iterator<Item = Symbol>,
	{
		let symbol = match stream.peek()
		{
			Some(symbol) => symbol,
			None => return Ok(None),
		};

		let token = match symbol
		{
			_ if symbol.is_whitespace() => Token::from_whitespace(stream, source),
			_ if symbol.is_identifier_head() => Token::from_identifier_head(stream, source),
			_ if symbol.is_number_head() => Token::from_number_head(stream, source),
		};

		todo!()
	}
}
