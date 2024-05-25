use std::iter::Peekable;

use anyhow::{bail, Result};

use super::*;
use crate::compiler::Error;
use crate::language::grammar::expression::{Expression, Items, Kind};
use crate::language::grammar::FunctionCall;
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

	fn primary<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Self>
	where
		I: Iterator<Item = Token>,
	{
		let token = match stream.next()
		{
			Some(token) => token,
			None =>
			{
				bail!("Expected Expression (If you see this, something went wrong while parsing)")
			}
		};

		let expression = match token.kind
		{
			Identifier(name) =>
			{
				let peek = stream.peek();
				let start = token.span.start;
				let mut end = token.span.end;
				let kind = match peek
				{
					Some(Token {
						kind: ParenthesisLeft,
						..
					}) =>
					{
						let span_left = stream.next().expect("Expected Parenthesis").span;
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
						Kind::Identifier(Token {
							span: Span { start, end },
							kind: Identifier(name),
						})
						// bail!(source.error(token.span, error::PARAMS_AFTER))
					}
				};

				Self {
					span: Span { start, end },
					kind,
				}
			}

			Number(_) | String(_) | Boolean(_) =>
			{
				let span = token.span;
				let kind = Kind::Literal(token);

				Self { span, kind }
			}

			ParenthesisLeft =>
			{
				let expression = Box::new(Self::try_from_stream(stream, source)?);

				let start = token.span.start;

				let end = match stream.next()
				{
					Some(token) if (token.kind == ParenthesisRight) => token.span.end,
					_ => bail!(source.error(token.span, error::PARENTHESIS)),
				};

				let span = Span { start, end };
				let kind = Kind::Parenthesised(expression);

				Self { span, kind }
			}
			BracketLeft =>
			{
				let open = token.span;
				let start = open.start;
				let items = utils::items(stream, source)?;
				let mut structure: Vec<Option<Items>> = vec![items];

				while stream.next_if(|token| token.kind == Bar).is_some()
				{
					stream.next_if(|token| token.kind == Bar);
					structure.push(utils::items(stream, source)?);
				}

				let closing = stream.next();
				let seen_bar = structure.len() > 1;

				if seen_bar
				{
					return match closing
					{
						Some(token) if token.is_matrix_closing() =>
						{
							let span = Span {
								start,
								end: token.span.end,
							};

							Ok(Self {
								span,
								kind: Kind::Matrix(structure),
							})
						}
						_ => bail!(source.error(open, error::MATRIX_BRACKET)),
					};
				}
				else
				{
					match closing
					{
						Some(Token {
							span,
							kind: BracketRight | BracketRightWithA,
						}) =>
						{
							let span = Span {
								start,
								end: span.end,
							};

							Self {
								span,
								kind: Kind::List(structure),
							}
						}
						Some(Token {
							span,
							kind: BracketRightWithM,
						}) =>
						{
							let span = Span {
								start,
								end: span.end,
							};

							Self {
								span,
								kind: Kind::Matrix(structure),
							}
						}
						_ => bail!(source.error(open, error::BRACKET)),
					}
				}
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

	/// Checks whether the token is a matrix closing parenthesis.
	///
	/// ### Returns
	/// * `true` if the token is a matrix closing parenthesis.
	/// * `false` otherwise.
	pub fn is_matrix_closing(&self) -> bool
	{
		self.kind == BracketRight || self.kind == BracketRightWithM
	}

	/// Checks whether the token is a closing parenthesis or Bracket.
	///
	/// ### Returns
	/// * `true` if the token is a closing parenthesis or Bracket.
	/// * `false` otherwise.
	pub fn is_list_closing(&self) -> bool
	{
		self.kind == BracketRight || self.kind == BracketRightWithA
	}

	pub fn is_closing(&self) -> bool
	{
		self.kind == BracketRight
			|| self.kind == ParenthesisRight
			|| self.kind == BracketRightWithA
			|| self.kind == BracketRightWithM
	}
}
