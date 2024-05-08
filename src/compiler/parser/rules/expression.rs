use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::expression::{Expression, Kind};
use crate::language::lexicon::token::{Kind::*, Token};
use crate::language::utils::Span;

impl Expression
{
	/// Creates an expression from a stream of tokens.
	///
	/// ### Parameters
	/// * `stream` - The stream of tokens.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The expression if it can be constructed from the stream.
	///
	/// ### Errors
	/// * If the expression cannot be created.
	///
	/// ### Panics
	/// * If the stream is empty.
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		Self::or(stream, source)
	}

	fn or<I>(stream: &mut Peekable<I>, source: &[Vec<char>]) -> Result<Self>
	where I: Iterator<Item = Token>
	{
		let mut expression = Self::and(stream, source)?;

		while let Some(or) = stream.next_if(|token| token.kind == Or)
		{
			let left = Box::new(expression);
			let operator = or;
			let right = Box::new(Self::and(stream, source)?);

			let span = Span {
				start: left.span.start,
				end: right.span.end,
			};

			let kind = Kind::Infix {
				left,
				operator,
				right,
			};

			expression = Self { span, kind };
		}

		Ok(expression)
	}

	fn and<I>(stream: &mut Peekable<I>, source: &[Vec<char>]) -> Result<Self>
	where I: Iterator<Item = Token>
	{
		let mut expression = Self::equality(stream, source)?;

		while let Some(and) = stream.next_if(|token| token.kind == And)
		{
			let left = Box::new(expression);
			let operator = and;
			let right = Box::new(Self::equality(stream, source)?);

			let span = Span {
				start: left.span.start,
				end: right.span.end,
			};

			let kind = Kind::Infix {
				left,
				operator,
				right,
			};

			expression = Self { span, kind };
		}

		Ok(expression)
	}

	fn equality<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let mut expression = Self::comparison(stream, source)?;

		while let Some(operator) = stream.next_if(Token::is_equality)
		{
			let left = Box::new(expression);
			let right = Box::new(Self::comparison(stream, source)?);

			let span = Span {
				start: left.span.start,
				end: right.span.end,
			};

			let kind = Kind::Infix {
				left,
				operator,
				right,
			};

			expression = Self { span, kind };
		}

		Ok(expression)
	}

	fn comparison<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let mut expression = Self::term(stream, source)?;

		while let Some(operator) = stream.next_if(Token::is_comparison)
		{
			let left = Box::new(expression);
			let right = Box::new(Self::term(stream, source)?);

			let span = Span {
				start: left.span.start,
				end: right.span.end,
			};

			let kind = Kind::Infix {
				left,
				operator,
				right,
			};

			expression = Self { span, kind };
		}

		Ok(expression)
	}

	fn term<I>(stream: &mut Peekable<I>, source: &[Vec<char>]) -> Result<Self>
	where I: Iterator<Item = Token>
	{
		let mut expression = Self::factor(stream, source)?;

		while let Some(operator) = stream.next_if(Token::creates_term)
		{
			let left = Box::new(expression);
			let right = Box::new(Self::factor(stream, source)?);

			let span = Span {
				start: left.span.start,
				end: right.span.end,
			};

			let kind = Kind::Infix {
				left,
				operator,
				right,
			};

			expression = Self { span, kind };
		}

		Ok(expression)
	}

	fn factor<I>(stream: &mut Peekable<I>, source: &[Vec<char>]) -> Result<Self>
	where I: Iterator<Item = Token>
	{
		let mut expression = Self::prefix(stream, source)?;

		while let Some(operator) = stream.next_if(Token::creates_factor)
		{
			let left = Box::new(expression);
			let right = Box::new(Self::prefix(stream, source)?);

			let span = Span {
				start: left.span.start,
				end: right.span.end,
			};

			let kind = Kind::Infix {
				left,
				operator,
				right,
			};

			expression = Self { span, kind };
		}

		Ok(expression)
	}

	fn prefix<I>(stream: &mut Peekable<I>, source: &[Vec<char>]) -> Result<Self>
	where I: Iterator<Item = Token>
	{
		if let Some(operator) = stream.next_if(Token::is_prefix_operator)
		{
			let operand = Box::new(Self::prefix(stream, source)?);

			let span = Span {
				start: operator.span.start,
				end: operand.span.end,
			};

			let kind = Kind::Prefix { operator, operand };

			Ok(Self { span, kind })
		}
		else
		{
			Self::primary(stream, source)
		}
	}

	/// * _list_ -> `[` _items_ `]` `l`?
	fn list<I>(stream: &mut Peekable<I>, source: &[Vec<char>]) -> Result<Self>
	where I: Iterator<Item = Token>
	{
		todo!()
	}

	///  * _matrix_ -> `[` _items_ { (`||` | `|`) _items_ }* `]` `m`?
	fn matrix<I>(stream: &mut Peekable<I>, source: &[Vec<char>]) -> Result<Self>
	where I: Iterator<Item = Token>
	{
		todo!()
	}

	fn primary<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let token = stream.next().expect("Token");

		let expression = match token.kind
		{
			Identifier(_) =>
			{
				let span = token.span;
				let kind = Kind::Identifier(token);

				Self { span, kind }
			}

			Number(_) | String(_) | Bool(_) =>
			{
				let span = token.span;
				let kind = Kind::Literal(token);

				Self { span, kind }
			}

			Parenthesis(true) =>
			{
				let expression = Box::new(Self::try_from_stream(stream, source)?);

				let start = token.span.start;
				let end = match stream.next()
				{
					Some(token) if (token.kind == Parenthesis(false)) => token.span.end,
					_ => bail!(source.error(token.span, error::PARENTHESIS)),
				};

				let span = Span { start, end };
				let kind = Kind::Parenthesised(expression);

				Self { span, kind }
			}

			_ => bail!(source.error(token.span, error::EXPRESSION)),
		};

		Ok(expression)
	}
}

impl Token
{
	/// Checks whether the token can be used to perform an equality comparison.
	///
	/// ### Returns
	/// * `true` if the token can be used to perform an equality comparison.
	/// * `false` otherwise.
	pub fn is_equality(&self) -> bool
	{
		self.kind == EqualEqual || self.kind == ExclaimEqual
	}

	/// Checks whether the token can be used to perform a comparison.
	///
	/// ### Returns
	/// * `true` if the token can be used to perform a comparison.
	/// * `false` otherwise.
	pub fn is_comparison(&self) -> bool
	{
		self.kind == Less
			|| self.kind == LessEqual
			|| self.kind == Greater
			|| self.kind == GreaterEqual
	}

	/// Checks whether the token can be used to perform an addition or
	/// subtraction.
	///
	/// ### Returns
	/// * `true` if the token can be used to perform an addition or subtraction.
	/// * `false` otherwise.
	pub fn creates_term(&self) -> bool
	{
		self.kind == Plus || self.kind == Minus
	}

	/// Checks whether the token can be used to perform a multiplication,
	/// division, or modulo.
	///
	/// ### Returns
	/// * `true` if the token can be used to create a factor.
	/// * `false` otherwise.
	pub fn creates_factor(&self) -> bool
	{
		self.kind == Asterisk || self.kind == Slash || self.kind == Percent
	}

	/// Checks whether the token is a prefix operator.
	///
	/// ### Returns
	/// * `true` if the token is a prefix operator.
	/// * `false` otherwise.
	pub fn is_prefix_operator(&self) -> bool
	{
		self.kind == Plus || self.kind == Minus || self.kind == Not
	}
}
