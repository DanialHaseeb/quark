use anyhow::{bail, Result};
use std::iter::Peekable;

use crate::{
	compiler::{
		error::Error,
		parser::{error, rules::utils::block},
	},
	language::{
		grammar::{controlflow::*, Expression},
		lexicon::token::Kind::*,
		lexicon::Token,
		utils::Span,
	},
};

impl IfStmt
{
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let Span { start, mut end } = stream.next().expect("while").span;

		let result = Expression::try_from_stream(stream, source);

		let condition = match result
		{
			Ok(condition) => condition,
			Err(_) =>
			{
				bail!(source.error(Span { start, end }, error::CONDITION_AFTER))
			}
		};

		end = condition.span.end;

		let body = match stream.peek()
		{
			Some(Token {
				kind: BraceLeft, ..
			}) =>
			{
				let body = block(stream, source)?;
				end = body.span.end;
				body
			}

			_ => bail!(source.error(Span { start, end }, error::BLOCK_AFTER)),
		};

		let else_body = match stream.peek()
		{
			Some(Token { kind: Else, .. }) =>
			{
				end = stream.next().expect("else Token").span.end;
				match stream.peek()
				{
					Some(Token {
						kind: BraceLeft, ..
					}) =>
					{
						let body = block(stream, source)?;
						end = body.span.end;
						Some(body)
					}
					_ => bail!(source.error(Span { start, end }, error::BLOCK_AFTER)),
				}
			}
			_ => None,
		};

		Ok(Self {
			span: Span { start, end },
			condition,
			else_body,
			body,
		})
	}
}
impl WhileStmt
{
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let Span { start, mut end } = stream.next().expect("while").span;

		let result = Expression::try_from_stream(stream, source);

		let condition = match result
		{
			Ok(condition) => condition,
			Err(_) =>
			{
				bail!(source.error(Span { start, end }, error::CONDITION_AFTER))
			}
		};

		end = condition.span.end;

		match stream.peek()
		{
			Some(Token {
				kind: BraceLeft, ..
			}) =>
			{
				let body = block(stream, source)?;
				let span = Span {
					start,
					end: body.span.end,
				};
				Ok(Self {
					span,
					condition,
					body,
				})
			}

			_ => bail!(source.error(Span { start, end }, error::BLOCK_AFTER)),
		}
	}
}
