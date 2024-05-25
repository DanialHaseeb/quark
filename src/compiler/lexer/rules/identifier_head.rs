use std::iter::Peekable;

use crate::language::lexicon::token;
use crate::language::lexicon::{Symbol, Token};
use crate::language::utils::Span;

impl Token
{
	/// Creates a token from a stream that starts with a symbol that potentially
	/// starts an identifier.
	///
	/// ### Parameters
	/// * `stream` - The stream of symbols.
	///
	/// ### Returns
	/// * The next token if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	pub fn from_identifier_head<I>(stream: &mut Peekable<I>) -> Option<Self>
	where I: Iterator<Item = Symbol>
	{
		let Symbol {
			position: start,
			character,
		} = stream.next()?;

		let mut end = start;

		let mut lexeme = String::from(character);

		while let Some(symbol) = stream.next_if(Symbol::continues_identifier)
		{
			lexeme.push(symbol.character);
			end = symbol.position;
		}

		Self {
			span: Span { start, end },
			kind: token::Kind::from_identifier(lexeme),
		}
		.into()
	}
}

impl Symbol
{
	/// Determines if the symbol is a valid continuation of an identifier.
	///
	/// ### Returns
	/// * `true` if the symbol can continue an identifier.
	/// * `false` otherwise.
	pub fn continues_identifier(&self) -> bool
	{
		self.character.is_alphabetic()
			|| self.character.is_ascii_digit()
			|| self.character == '_'
	}
}

impl token::Kind
{
	/// Creates a kind of token from an identifier lexeme.
	///
	/// ### Parameters
	/// * `lexeme` - The lexeme of the identifier.
	///
	/// ### Returns
	/// * The kind of token.
	pub fn from_identifier(lexeme: String) -> Self
	{
		use token::Kind::*;

		match lexeme.as_str()
		{
			"true" => Boolean(true),
			"false" => Boolean(false),
			"let" => Constant,
			"var" => Variable,
			"fn" => Function,
			// "proc" => Procedure,
			"if" => If,
			"else" => Else,
			"while" => While,
			"echo" => Echo,
			"for" => For,
			"in" => In,
			"return" => Return,
			"break" => Break,
			"continue" => Continue,
			"and" => And,
			"or" => Or,
			"not" => Not,
			"xor" => Xor,
			_ => Identifier(lexeme),
		}
	}
}
