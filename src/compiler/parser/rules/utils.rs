use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::expression::{Expression, Items};
use crate::language::grammar::functions::{Parameter, Params};
use crate::language::grammar::{Block, Statement};
use crate::language::lexicon::token::{Kind::*, Token};
use crate::language::utils::Span;

// PRIOR: checks if the next token is a expression until closing token
pub fn items<I>(
	stream: &mut std::iter::Peekable<I>,
	source: &[Vec<char>],
) -> Result<Option<Items>>
where
	I: Iterator<Item = Token>,
{
	match stream.peek()
	{
		Some(token) if token.is_item_closing() => Ok(None),
		None => Ok(None),
		Some(_) =>
		{
			let mut expressions = Vec::new();

			let expression = Expression::try_from_stream(stream, source)?;

			let start = expression.span.start;
			let mut end = expression.span.end;

			expressions.push(expression);

			loop
			{
				match stream.peek()
				{
					Some(Token { kind: Comma, .. }) =>
					{
						stream.next().expect("Comma");
						let expression = Expression::try_from_stream(stream, source)?;
						end = expression.span.end;
						expressions.push(expression);
					}
					Some(token) if token.is_item_closing() =>
					{
						break;
					}
					Some(token) =>
					{
						bail!(source.error(token.span, error::COMMA));
					}
					None =>
					{
						break;
					}
				}
			}

			let span = Span { start, end };

			Ok(Some(Items { span, expressions }))
		}
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

// PRIOR: checks if the next token is a param until closing token
pub fn params<I>(
	stream: &mut Peekable<I>,
	source: &[Vec<char>],
) -> Result<Option<Params>>
where
	I: Iterator<Item = Token>,
{
	match stream.peek()
	{
		Some(token) if matches!(token.kind, ParenthesisRight) => Ok(None),
		None => Ok(None),
		Some(_) =>
		{
			let mut parameters = Vec::new();

			let parameter = try_parameter_from_stream(stream, source)?;

			let start = parameter.span.start;
			let mut end = parameter.span.end;

			parameters.push(parameter);

			while stream.next_if(|token| token.kind == Comma).is_some()
			{
				let paramater = try_parameter_from_stream(stream, source)?;
				end = paramater.span.end;
				parameters.push(paramater);
			}

			let span = Span { start, end };

			Ok(Some(Params { span, parameters }))
		}
	}
}

fn try_parameter_from_stream<I>(
	stream: &mut Peekable<I>,
	source: &[Vec<char>],
) -> Result<Parameter>
where
	I: Iterator<Item = Token>,
{
	let token = stream.next().expect("Identifier");
	match token.kind
	{
		Identifier(name) => Ok(Parameter {
			span: token.span,
			name,
		}),
		_ => bail!(source.error(token.span, error::IDENTIFIER)),
	}
}

impl Token
{
	pub fn is_item_closing(&self) -> bool
	{
		self.kind == BracketRight
			|| self.kind == ParenthesisRight
			|| self.kind == BracketRightWithA
			|| self.kind == BracketRightWithM
			|| self.kind == Bar
	}
}
