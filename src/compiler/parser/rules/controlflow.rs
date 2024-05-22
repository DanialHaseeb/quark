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
		let span = stream.next().expect("If").span;
		let condition = Expression::try_from_stream(stream, source)?;
		let body = block(stream, source)?;

		let span = Span {
			start: span.start,
			end: body.span.end,
		};

		Ok(Self {
			span,
			condition,
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
		let span = stream.next().expect("while").span;
		let condition = Expression::try_from_stream(stream, source)?;

		match stream.peek()
		{
			Some(Token {
				kind: BraceLeft, ..
			}) =>
			{
				let body = block(stream, source)?;
				let span = Span {
					start: span.start,
					end: body.span.end,
				};
				Ok(Self {
					span,
					condition,
					body,
				})
			}

			_ => bail!(source.error(condition.span, error::BLOCK)),
		}
	}
}
