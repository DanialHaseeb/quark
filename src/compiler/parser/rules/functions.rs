use anyhow::{bail, Result};
use std::iter::Peekable;

use super::*;
use crate::compiler::Error;
use crate::language::lexicon::token::Kind::*;

use crate::language::{grammar::FunctionDclr, lexicon::Token, utils::Span};

impl FunctionDclr
{
	/// ### Prior:
	/// has seen a Declaration token
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let fn_token = stream.next().expect("fn Token");
		let start = fn_token.span.start;
		let mut end = fn_token.span.end;
		let option = stream.next();

		let name = match option
		{
			Some(Token {
				kind: Identifier(name),
				span,
			}) =>
			{
				end = span.end;
				name
			}
			_ => bail!(source.error(Span { start, end }, error::FUNCTION_NAME)),
		};

		let span_left = match stream.next()
		{
			Some(token) if matches!(token.kind, ParenthesisLeft) => token.span,
			_ =>
			{
				bail!(source.error(Span { start, end }, error::EXPECT_PARENTHESIS))
			}
		};

		let parameters = utils::params(stream, source)?;

		match stream.next()
		{
			Some(token) if matches!(token.kind, ParenthesisRight) => token.span,
			_ => bail!(source.error(span_left, error::PARENTHESIS)),
		};

		match stream.peek()
		{
			Some(Token {
				kind: BraceLeft, ..
			}) =>
			{
				let body = utils::block(stream, source)?;
				let span = Span {
					start,
					end: body.span.end,
				};
				Ok(Self {
					span,
					name,
					parameters,
					body,
				})
			}

			_ => bail!(source.error(Span { start, end }, error::BLOCK_AFTER)),
		}
	}
}
