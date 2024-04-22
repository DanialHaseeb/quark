use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::token::Kind::Slash;
use crate::language::{Span, Symbol, Token};

impl Token
{
	/// Creates a token from a stream that starts with a symbol that potentially
	/// starts a comment.
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
	pub fn try_from_comment_head<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Token>>
	where
		I: Iterator<Item = Symbol>,
	{
		// Consume and get the position of the first slash symbol.
		let position = match stream.next()
		{
			Some(symbol) => symbol.position,
			None => return Ok(None),
		};

		let span = Span {
			start: position,
			end: position,
		};

		match stream.peek().map(|&symbol| symbol.value)
		{
			// If the next symbol is a slash, then this is an end-of-line comment.
			Some('/') =>
			{
				while stream.next_if(|symbol| symbol.value != '\n').is_some()
				{}
				Self::try_from_stream(stream, source)
			}

			// If the next symbol is an asterisk, then this is a block comment.
			Some('*') =>
			{
				stream.next();

				while let Some(Symbol { value, .. }) = stream.next()
				{
					if value == '*'
					{
						match stream.next().map(|symbol| symbol.value)
						{
							Some('/') =>
							{
								return Self::try_from_stream(stream, source);
							}
							Some(_) => continue,
							None => break,
						}
					}
				}

				bail!(source.error(span, error::COMMENT));
			}

			// Otherwise, this was a single slash token all along.
			_ => Ok(Some(Self { span, kind: Slash })),
		}
	}
}
