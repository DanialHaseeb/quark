use std::iter::Peekable;

use crate::language::utils::Span;
use crate::language::lexicon::token;
use crate::language::lexicon::{Token, Symbol};

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
	pub fn from_operator_head<I>(stream: &mut Peekable<I>) -> Option<Token>
	where I: Iterator<Item = Symbol>
	{
		let Symbol {
			position: start,
			value,
		} = stream.next()?;

		let mut end = start;

		let mut lexeme = String::from(value);

		if let Some(symbol) = stream.next_if(|&symbol| symbol.value == '=')
		{
			lexeme.push(symbol.value);
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
