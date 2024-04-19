use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::language::{Symbol, Token};

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
	pub fn from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Self>>
	where
		I: Iterator<Item = Symbol>,
	{
		let token = match stream.peek()
		{
			None => None,

			Some(symbol) if symbol.is_whitespace() =>
			{
				Some(Token::from_whitespace(stream, source)?)
			}

			Some(symbol) if symbol.is_identifier_head() =>
			{
				Some(Token::from_identifier_head(stream, source))
			}

			Some(symbol) if symbol.is_number_head() =>
			{
				Token::from_number_head(stream, source)
			}

			_ => bail!(error::SYMBOL),
		};

		Ok(token)
	}
}
