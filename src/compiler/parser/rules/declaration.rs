use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::declaration::{Declaration, Kind};
use crate::language::grammar::{Expression, FunctionDclr};
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
		let declarator = stream.peek().expect("Declarator");

		match declarator.kind
		{
			Function =>
			{
				let declaration = FunctionDclr::try_from_stream(stream, source)?;
				let span = declaration.span;
				Ok(Self {
					span,
					kind: Kind::Function(declaration),
				})
			}
			Constant | Variable =>
			{
				let declarator = stream.next().expect("Constant or Variable");
				let start = declarator.span.start;
				let mut end = declarator.span.end;

				let is_mutable = (declarator.kind == Variable);

				let identifier = match stream.next()
				{
					Some(token) if matches!(token.kind, Identifier(_)) => token,
					_ =>
					{
						bail!(source.error(Span { start, end }, error::IDENTIFIER_AFTER))
					}
				};

				end = identifier.span.end;

				let equals = match stream.next()
				{
					Some(token) if token.kind == Equal => token,
					_ =>
					{
						bail!(source.error(Span { start, end }, error::EQUALS_AFTER))
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
						bail!(source.error(Span { start, end }, error::EXPRESSION_AFTER))
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
			_ => unreachable!(),
		}
	}
}
