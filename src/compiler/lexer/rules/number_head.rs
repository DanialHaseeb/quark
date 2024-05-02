use std::iter::Peekable;

use crate::language::lexicon::token::Kind::{Dot, Number};
use crate::language::lexicon::{Symbol, Token};
use crate::language::utils::Span;

impl Token
{
	/// Creates a token from a stream that starts with a symbol that potentially
	/// starts a number.
	///
	/// ### Parameters
	/// * `stream` - The stream of symbols.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The next token if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	pub fn from_number_head<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Option<Self>
	where
		I: Iterator<Item = Symbol>,
	{
		let Symbol {
			position: start,
			value,
		} = stream.next()?;

		let mut end = start;
		let mut seen_dot = (value == '.');

		if seen_dot && !stream.peek().is_some_and(Symbol::is_digit)
		{
			return Self {
				span: Span { start, end },
				kind: Dot,
			}
			.into();
		}

		let mut lexeme = String::from(value);

		while let Some(&Symbol { position, value }) = stream.peek()
		{
			let next = source[position.line][position.column + 1];
			match value
			{
				_ if value.is_ascii_digit() =>
				{
					lexeme.push(value);
					end = position;
					stream.next();
				}

				'.' if !seen_dot && next.is_ascii_digit() =>
				{
					seen_dot = true;
					lexeme.push(value);
					stream.next();
				}

				_ => break,
			}
		}

		if let Some(symbol) = stream.next_if(|&symbol| symbol.value == 'i')
		{
			lexeme.push('j');
			end = symbol.position;
		}

		Self {
			span: Span { start, end },
			kind: Number(lexeme),
		}
		.into()
	}
}

impl Symbol
{
	/// Determines if the symbol is a digit.
	///
	/// ### Returns
	/// * `true` if the symbol is a digit.
	/// * `false` otherwise.
	pub fn is_digit(&self) -> bool
	{
		self.value.is_ascii_digit()
	}
}
