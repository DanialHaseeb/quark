use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::statement::{Kind, Statement};
use crate::language::grammar::{controlflow::*, Declaration, Expression};
use crate::language::lexicon::token::{Kind::*, Token};
use crate::language::utils::Span;

impl Statement
{
	/// Creates a statement from a stream of tokens.
	///
	/// ### Parameters
	/// * `stream` - The stream of tokens.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The statement if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	///
	/// ### Errors
	/// * If the statement cannot be created.
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Statement>>
	where
		I: Iterator<Item = Token>,
	{
		let start;
		let end;

		let kind = match stream.peek()
		{
			Some(token) =>
			{
				start = token.span.start;
				&token.kind
			}
			None => return Ok(None),
		};

		let kind = match kind
		{
			While =>
			{
				let while_ = WhileStmt::try_from_stream(stream, source)?;
				end = while_.span.end;
				Kind::While(while_)
			}

			If =>
			{
				let if_ = IfStmt::try_from_stream(stream, source)?;
				end = if_.span.end;
				Kind::If(if_)
			}

			Continue =>
			{
				let span = stream.next().expect("Continue Token").span;
				end = match stream.next()
				{
					Some(Token {
						span,
						kind: Semicolon,
					}) => span.end,
					_ => bail!(source.error(
						span,
						format!("{} {}", error::SEMICOLON_AFTER, "continue statement")
							.as_str()
					)),
				};
				Kind::Continue(ContinueStmt { span })
			}

			Break =>
			{
				let span = stream.next().expect("Break Token").span;
				end = match stream.next()
				{
					Some(Token {
						span,
						kind: Semicolon,
					}) => span.end,
					_ => bail!(source.error(
						span,
						format!("{} {}", error::SEMICOLON_AFTER, "break statement")
							.as_str()
					)),
				};
				Kind::Break(BreakStmt { span })
			}

			Constant | Variable =>
			{
				let declaration = Declaration::try_from_stream(stream, source)?;
				end = match stream.next()
				{
					Some(Token {
						span,
						kind: Semicolon,
					}) => span.end,
					_ => bail!(source.error(
						declaration.span,
						format!("{} {}", error::SEMICOLON_AFTER, "declaration statement")
							.as_str()
					)),
				};
				Kind::Declaration(declaration)
			}
			_ =>
			{
				let expression = Expression::try_from_stream(stream, source)?;
				end = match stream.next()
				{
					Some(Token {
						span,
						kind: Semicolon,
					}) => span.end,
					_ => bail!(source.error(
						expression.span,
						format!("{} {}", error::SEMICOLON_AFTER, "expression statment")
							.as_str(),
					)),
				};
				Kind::Expression(expression)
			}
		};

		let span = Span { start, end };

		Ok(Some(Statement { kind, span }))
	}
}
