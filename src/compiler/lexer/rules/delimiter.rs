use std::iter::Peekable;

use crate::language::lexicon::token::Kind::*;
use crate::language::lexicon::{Symbol, Token};
use crate::language::utils::Span;

impl Token
{
	/// Creates a token from a stream that starts with a delimiter.
	///
	/// ### Parameters
	/// * `stream` - The stream of symbols.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The next token if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	pub fn from_delimiter<I>(stream: &mut Peekable<I>) -> Option<Self>
	where I: Iterator<Item = Symbol>
	{
		let Symbol { position, value } = stream.next()?;

		let span = Span {
			start: position,
			end: position,
		};

		let state = matches!(value, '(' | '[' | '{');

		let kind = match value
		{
			'(' | ')' => Parenthesis(state),
			'[' | ']' => Bracket(state),
			'{' | '}' => Brace(state),
			_ => unreachable!(),
		};

		Self { span, kind }.into()
	}
}
