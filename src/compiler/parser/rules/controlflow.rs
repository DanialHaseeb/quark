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
				bail!(source.error(
					Span { start, end },
					format!("{} {}", error::CONDITION_AFTER, "if keyboard").as_str()
				))
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

			_ => bail!(source.error(
				Span { start, end },
				format!("{} {}", error::BLOCK_AFTER, "if condition").as_str()
			)),
		}
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
				bail!(source.error(
					Span { start, end },
					format!("{} {}", error::CONDITION_AFTER, "while keyboard").as_str()
				))
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

			_ => bail!(source.error(
				Span { start, end },
				format!("{} {}", error::BLOCK_AFTER, "while condition").as_str()
			)),
		}
	}
}
