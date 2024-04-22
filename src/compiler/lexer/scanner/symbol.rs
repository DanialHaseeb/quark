use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::token::Kind::*;
use crate::language::{Span, Symbol, Token};

impl Token
{
	/// Creates a token from a stream that starts with a symbol that potentially
	/// starts a single-symbol token.
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
	pub fn try_from_symbol<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Token>>
	where
		I: Iterator<Item = Symbol>,
	{
		let Symbol { position, value } = match stream.next()
		{
			Some(symbol) => symbol,
			None => return Ok(None),
		};

		let span = Span {
			start: position,
			end: position,
		};

		let kind = match value
		{
			',' => Comma,
			':' => Colon,
			';' => Semicolon,
			'|' => Bar,
			_ => bail!(source.error(span, error::SYMBOL)),
		};

		Ok(Some(Self { span, kind }))
	}
}
