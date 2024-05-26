use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::statement::{Kind, Statement};
use crate::language::grammar::{
	controlflow::*, AssignmentStmt, Declaration, EchoStmt, Expression,
	FunctionCall, ReturnStmt,
};
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
		let mut end;

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
					_ => bail!(source.error(span, error::SEMICOLON_AFTER)),
				};
				Kind::Continue(ContinueStmt { span })
			}

			Return =>
			{
				let span = stream.next().expect("Return Token").span;
				let expression = Expression::try_from_stream(stream, source)?;
				let start = span.start;
				end = expression.span.end;

				end = match stream.next()
				{
					Some(Token {
						span,
						kind: Semicolon,
					}) => span.end,
					_ => bail!(source.error(Span { start, end }, error::SEMICOLON_AFTER)),
				};
				Kind::Return(ReturnStmt {
					span: Span { start, end },
					expression,
				})
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
					_ => bail!(source.error(span, error::SEMICOLON_AFTER)),
				};
				Kind::Break(BreakStmt { span })
			}

			Echo =>
			{
				let echo = EchoStmt::try_from_stream(stream, source)?;
				let span = echo.span;

				end = match stream.next()
				{
					Some(Token {
						span,
						kind: Semicolon,
					}) => span.end,
					_ => bail!(source.error(span, error::SEMICOLON_AFTER)),
				};

				Kind::Echo(echo)
			}

			Function =>
			{
				let declaration = Declaration::try_from_stream(stream, source)?;
				end = declaration.span.end;
				Kind::Declaration(declaration)
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
					_ => bail!(source.error(declaration.span, error::SEMICOLON_AFTER)),
				};
				Kind::Declaration(declaration)
			}

			Identifier(_) =>
			{
				let identifier = stream.next().expect("Identifier Token");
				let start = identifier.span.start;
				let peek = stream.peek();

				let name = match identifier.kind
				{
					Identifier(name) => name,
					_ => unreachable!(),
				};

				let kind = match peek
				{
					Some(Token { kind: Equal, .. }) =>
					{
						end = stream.next().expect("Equal Token").span.end;
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
							start,
							end: value.span.end,
						};

						Kind::Assignment(AssignmentStmt {
							span,
							identifier: name,
							expression: value,
						})
					}
					Some(Token {
						kind: ParenthesisLeft,
						..
					}) =>
					{
						let span_left = stream.next().expect("Parenthesis Token").span;
						let arguments = utils::items(stream, source)?;
						let span_right = match stream.next()
						{
							Some(token) if matches!(token.kind, ParenthesisRight) =>
							{
								token.span
							}
							_ => bail!(source.error(span_left, error::PARENTHESIS)),
						};

						end = span_right.end;

						let span = Span { start, end };

						Kind::FunctionCall(FunctionCall {
							name,
							span,
							arguments,
						})
					}
					_ =>
					{
						bail!(source.error(identifier.span, error::PARAMS_AFTER))
					}
				};

				end = match stream.next()
				{
					Some(Token {
						span,
						kind: Semicolon,
					}) => span.end,
					_ => bail!(source.error(Span { start, end }, error::SEMICOLON_AFTER)),
				};
				kind
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
					_ => bail!(source.error(expression.span, error::SEMICOLON_AFTER)),
				};
				Kind::Expression(expression)
			}
		};

		let span = Span { start, end };

		Ok(Some(Statement { kind, span }))
	}
}

impl EchoStmt
{
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let span = stream.next().expect("Echo Token").span;
		let start = span.start;
		let mut end = span.end;

		let mut arguments = Vec::new();
		let expression = Expression::try_from_stream(stream, source);

		match expression
		{
			Ok(expression) =>
			{
				end = expression.span.end;
				arguments.push(expression);
			}
			Err(_) =>
			{
				bail!(source.error(Span { start, end }, error::EXPRESSION_AFTER))
			}
		}

		while stream.next_if(|token| token.kind == Comma).is_some()
		{
			let expression = Expression::try_from_stream(stream, source)?;
			end = expression.span.end;
			arguments.push(expression);
		}
		let span = Span { start, end };
		Ok(EchoStmt { span, arguments })
	}
}
