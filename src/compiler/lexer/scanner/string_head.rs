use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::utils::Span;
use crate::language::lexicon::token;
use crate::language::lexicon::{Token, Symbol};

impl Token
{
	/// Creates a token from a stream that starts with a symbol that potentially
	/// starts a string literal.
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
	pub fn try_from_string_head<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Token>>
	where
		I: Iterator<Item = Symbol>,
	{
		// Consume and get the position of the opening quote.
		let start = match stream.next()
		{
			Some(symbol) => symbol.position,
			None => return Ok(None),
		};

		let mut end = start;

		let mut lexeme = String::new();

		loop
		{
			match stream.next()
			{
				Some(symbol) if symbol.value == '"' =>
				{
					end = symbol.position;
					break;
				}

				Some(symbol) if symbol.value == '\n' =>
				{
					lexeme.push_str("\\n");
				}

				Some(symbol) =>
				{
					lexeme.push(symbol.value);
				}

				None => bail!(source.error(Span { start, end }, error::QUOTE)),
			}
		}

		let token = Self {
			span: Span { start, end },
			kind: token::Kind::String(lexeme),
		};

		Ok(Some(token))
	}
}
