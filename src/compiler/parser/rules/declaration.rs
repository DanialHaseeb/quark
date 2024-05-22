use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::declaration::{Declaration, Kind};
use crate::language::grammar::Expression;
use crate::language::lexicon::token::{Kind::*, Token};
use crate::language::utils::Span;

impl Declaration
{
	/// Creates a declaration from a stream of tokens.
	///
	/// ### Parameters
	/// * `stream` - The stream of tokens.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The declaration if it can be constructed from the stream.
	///
	/// ### Errors
	/// * If the statement cannot be created.
	///
	/// ### Panics
	/// * If the stream is empty.
	/// * If the stream does not start with a declarator.
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		match stream.peek().expect("Declarator").kind
		{
			Constant | Variable => Self::try_from_variable(stream, source),
			_ => unreachable!(),
		}
	}

	/// Creates a declaration from a stream of tokens that starts with a variable
	/// declarator.
	///
	/// ### Parameters
	/// * `stream` - The stream of tokens.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The declaration if it can be constructed from the stream.
	///
	/// ### Errors
	/// * If the declaration cannot be created.
	///
	/// ### Panics
	/// * If the stream is empty.
	/// * If the stream does not start with a variable declarator.
	fn try_from_variable<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let declarator = stream.next().expect("Declarator");

		let is_mutable = (declarator.kind == Variable);

		let start = declarator.span.start;
		let mut end = declarator.span.end;

		let identifier = match stream.next()
		{
			Some(token) if matches!(token.kind, Identifier(_)) => token,
			_ => bail!(source.error(
				Span { start, end },
				format!("{} {}", error::IDENTIFIER_AFTER, "declaration").as_str()
			)),
		};

		end = identifier.span.end;

		let equals = match stream.next()
		{
			Some(token) if token.kind == Equal => token,
			_ =>
			{
				bail!(source.error(
					Span { start, end },
					format!("{} {}", error::EQUALS_AFTER, "for assignment").as_str()
				))
			}
		};

		end = equals.span.end;

		let name = match identifier.kind
		{
			Identifier(name) => name,
			_ => unreachable!(),
		};

		let result = Expression::try_from_stream(stream, source);

		let value = match result
		{
			Ok(value) => value,
			Err(_) =>
			{
				bail!(source.error(
					Span { start, end },
					format!("{} {}", error::EXPRESSION_AFTER, "declaration",).as_str()
				))
			}
		};

		let span = Span {
			start: declarator.span.start,
			end: value.span.end,
		};

		let kind = Kind::Variable {
			name,
			value,
			is_mutable,
		};

		Ok(Self { span, kind })
	}
}
