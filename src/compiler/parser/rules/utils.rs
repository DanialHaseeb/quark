use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::controlflow::Block;
use crate::language::grammar::expression::{Expression, Items};
use crate::language::grammar::Statement;
use crate::language::lexicon::token::{Kind::*, Token};
use crate::language::utils::Span;

pub fn items<I>(
	stream: &mut std::iter::Peekable<I>,
	source: &[Vec<char>],
) -> Result<Option<Items>>
where
	I: Iterator<Item = Token>,
{
	if stream.peek().is_some_and(|token| token.is_closing())
	{
		Ok(None)
	}
	else
	{
		let mut expressions = Vec::new();

		let expression = Expression::try_from_stream(stream, source)?;

		let start = expression.span.start;
		let mut end = expression.span.end;

		expressions.push(expression);

		while stream.next_if(|token| token.kind == Comma).is_some()
		{
			let expression = Expression::try_from_stream(stream, source)?;
			end = expression.span.end;
			expressions.push(expression);
		}

		let span = Span { start, end };

		Ok(Some(Items { span, expressions }))
	}
}

pub fn block<I>(
	stream: &mut Peekable<I>,
	source: &[Vec<char>],
) -> Result<Block>
where
	I: Iterator<Item = Token>,
{
	let open = stream.next().expect("Left brace expected.");

	assert_eq!(open.kind, BraceLeft);

	let mut statements = Vec::new();

	while let Some(token) = stream.peek()
	{
		if token.kind == BraceRight
		{
			break;
		}
		let statement = Statement::try_from_stream(stream, source)?;
		statements.push(statement);
	}
	let close = stream.next();

	match close
	{
		Some(close) if close.kind == BraceRight =>
		{
			let span = Span {
				start: open.span.start,
				end: close.span.end,
			};

			Ok(Block { span, statements })
		}
		_ => bail!(source.error(open.span, error::BRACE)),
	}
}
