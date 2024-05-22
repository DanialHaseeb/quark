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
		let Symbol {
			position: start,
			character: value,
		} = stream.next()?;

		let mut end = start;

		let kind = match value
		{
			'[' => BracketLeft,
			']' =>
			{
				let option = stream.next_if(|&symbol| {
					symbol.character == 'a' || symbol.character == 'm'
				});

				match option
				{
					Some(Symbol {
						position,
						character: 'a',
					}) =>
					{
						end = position;
						BracketRightWithA
					}
					Some(Symbol {
						position,
						character: 'm',
					}) =>
					{
						end = position;
						BracketRightWithM
					}
					None => BracketRight,
					_ => unreachable!(),
				}
			}
			'{' => BraceLeft,
			'}' => BraceRight,
			'(' => ParenthesisLeft,
			')' => ParenthesisRight,

			_ => unreachable!(),
		};

		let span = Span { start, end };

		Self { span, kind }.into()
	}
}
