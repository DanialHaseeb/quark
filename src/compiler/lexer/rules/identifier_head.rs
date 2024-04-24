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
			value,
		} = stream.next()?;

		let mut end = start;

		let mut lexeme = String::from(value);

		while let Some(symbol) = stream.next_if(Symbol::continues_identifier)
		{
			lexeme.push(symbol.value);
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
	///
	/// ### Examples
	/// ```rust
	/// use crate::language::Symbol;
	///
	/// let symbol = Symbol::new(0, 0, 'a');
	/// assert!(symbol.continues_identifier());
	///
	/// let symbol = Symbol::new(0, 0, '1');
	/// assert!(symbol.continues_identifier());
	///
	/// let symbol = Symbol::new(0, 0, '_');
	/// assert!(symbol.continues_identifier());
	///
	/// let symbol = Symbol::new(0, 0, ' ');
	/// assert!(!symbol.continues_identifier());
	///
	/// let symbol = Symbol::new(0, 0, '+');
	/// assert!(!symbol.continues_identifier());
	/// ```
	pub fn continues_identifier(&self) -> bool
	{
		false
			|| self.value.is_alphabetic()
			|| self.value.is_ascii_digit()
			|| self.value == '_'
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
	///
	/// ### Examples
	/// ```rust
	/// use crate::language::token::Kind;
	///
	/// let kind = Kind::from_identifier(String::from("true"));
	/// assert_eq!(kind, Kind::Bool(true));
	///
	/// let kind = Kind::from_identifier(String::from("hello"));
	/// assert_eq!(kind, Kind::Identifier(String::from("hello")));
	/// ```
	pub fn from_identifier(lexeme: String) -> Self
	{
		use token::Kind::*;

		match lexeme.as_str()
		{
			"true" => Bool(true),
			"false" => Bool(false),
			"let" => Constant,
			"var" => Variable,
			"func" => Function,
			"proc" => Procedure,
			"if" => If,
			"else" => Else,
			"while" => While,
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
