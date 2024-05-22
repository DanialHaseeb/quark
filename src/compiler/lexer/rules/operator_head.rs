use std::iter::Peekable;

use crate::language::lexicon::token;
use crate::language::lexicon::{Symbol, Token};
use crate::language::utils::Span;

impl Token
{
	/// Creates a token from a stream that starts with a symbol that potentially
	/// starts an operator.
	///
	/// ### Parameters
	/// * `stream` - The stream of symbols.
	///
	/// ### Returns
	/// * The next token if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	pub fn from_operator_head<I>(stream: &mut Peekable<I>) -> Option<Self>
	where I: Iterator<Item = Symbol>
	{
		let Symbol {
			position: start,
			character,
		} = stream.next()?;

		let mut end = start;

		let mut lexeme = String::from(character);

		if let Some(symbol) = stream.next_if(|&symbol| {
			let lexeme = format!("{}{}", character, symbol.character);
			matches!(
				lexeme.as_str(),
				"->"
					| "-=" | "*="
					| "/=" | "%="
					| "^=" | "=="
					| "!=" | ">="
					| "<=" | "+="
			)
		})
		{
			lexeme.push(symbol.character);
			end = symbol.position;
		}

		Self {
			span: Span { start, end },
			kind: token::Kind::from_operator(lexeme),
		}
		.into()
	}
}

impl token::Kind
{
	/// Creates an operator token from a lexeme.
	///
	/// ### Parameters
	/// * `lexeme` - The lexeme of the operator.
	///
	/// ### Returns
	/// * The operator token if the lexeme is a valid operator.
	/// * `None` otherwise.
	pub fn from_operator(lexeme: String) -> Self
	{
		use token::Kind::*;

		match lexeme.as_str()
		{
			"+" => Plus,
			"-" => Minus,
			"*" => Asterisk,
			"/" => Slash,
			"%" => Percent,
			"^" => Caret,
			"=" => Equal,
			"->" => ArrowRight,
			"+=" => PlusEqual,
			"-=" => MinusEqual,
			"*=" => AsteriskEqual,
			"/=" => SlashEqual,
			"%=" => PercentEqual,
			"^=" => CaretEqual,
			"==" => EqualEqual,
			"!=" => ExclaimEqual,
			">" => Greater,
			">=" => GreaterEqual,
			"<" => Less,
			"<=" => LessEqual,
			_ => unreachable!(),
		}
	}
}
