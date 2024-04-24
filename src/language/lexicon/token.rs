use std::fmt::{Debug, Formatter, Result};

use crate::language::utils::Span;

/// A token in a source file.
///
/// A token is a word or symbol that represents a unit of meaning in the
/// language.
#[derive(Clone, PartialEq)]
pub struct Token
{
	/// The span of the token in the source file.
	pub span: Span,

	/// The kind of the token.
	pub kind: Kind,
}

impl Debug for Token
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		let Self { span, kind } = self;
		write!(formatter, "{span:?}: {kind:?}")
	}
}

/// The kind of lexical token.
///
/// The kind of token is a classification of the token into categories defined
/// by the language specification.
#[derive(Clone, Debug, PartialEq)]
pub enum Kind
{
	/// An identifier token.
	///
	/// ### Rule
	/// * _identifier_ -> _letter_ | `_` { _letter_ | _digit_ | `_` }*
	/// * _letter_ -> `a`..`z` | `A`..`Z`
	/// * _digit_ -> `0`..`9`
	Identifier(String),

	/// A string literal token.
	///
	/// ### Rule
	/// * _string_ -> `"` { Σ \ `"` }* `"`
	String(String),

	/// A number literal token.
	///
	/// ### Rule
	/// * _number_ -> _digit_* `.`? _digit_+ `i`?
	Number(String),

	/// A boolean literal token.
	///
	/// ### Rule
	/// * _boolean_ -> `true` | `false`
	Bool(bool),

	/// The `let` declarator token.
	Constant,

	/// The `var` declarator token.
	Variable,

	/// The `func` declarator token.
	Function,

	/// The `proc` declarator token.
	Procedure,

	/// The lexical token for the `if` keyword.
	If,

	/// The lexical token for the `else` keyword.
	Else,

	/// The lexical token for the `while` keyword.
	While,

	/// The lexical token for the `for` keyword.
	For,

	/// The lexical token for the `in` keyword.
	In,

	/// The lexical token for the `return` keyword.
	Return,

	/// The lexical token for the `break` keyword.
	Break,

	/// The lexical token for the `continue` keyword.
	Continue,

	/// The `and` operator.
	And,

	/// The `or` operator.
	Or,

	/// The `not` operator.
	Not,

	/// The `xor` operator.
	Xor,

	/// The `+` operator.
	Plus,

	/// The `-` operator.
	Minus,

	/// The `*` operator.
	Asterisk,

	/// The `/` operator.
	Slash,

	/// The `%` operator.
	Percent,

	/// The `^` operator.
	Caret,

	/// The `=` operator.
	Equal,

	/// The `+=` operator.
	PlusEqual,

	/// The `-=` operator.
	MinusEqual,

	/// The `*=` operator.
	AsteriskEqual,

	/// The `/=` operator.
	SlashEqual,

	/// The `%=` operator.
	PercentEqual,

	/// The `^=` operator.
	CaretEqual,

	/// The `==` operator.
	EqualEqual,

	/// The `!=` operator.
	ExclaimEqual,

	/// The `>` operator.
	Greater,

	/// The `>=` operator.
	GreaterEqual,

	/// The `<` operator.
	Less,

	/// The `<=` operator.
	LessEqual,

	/// A parenthesis literal token.
	///
	/// The associated boolean value indicates whether it is opening or closing:
	/// * `true` - Opening
	/// * `false` - Closing
	///
	/// ### Rule
	/// * _boolean_ -> `(` | `)`
	Parenthesis(bool),

	/// A bracket literal token.
	///
	/// The associated boolean value indicates whether it is opening or closing:
	/// * `true` - Opening
	/// * `false` - Closing
	///
	/// ### Rule
	/// * _bracket_ -> `[` | `]`
	Bracket(bool),

	/// A brace literal token.
	///
	/// The associated boolean value indicates whether it is opening or closing:
	/// * `true` - Opening
	/// * `false` - Closing
	///
	/// ### Rule
	/// * _brace_ -> `{` | `}`
	Brace(bool),

	/// The lexical token for the `.` symbol.
	Dot,

	/// The lexical token for the `,` symbol.
	Comma,

	/// The lexical token for the `:` symbol.
	Colon,

	/// The lexical token for the `;` symbol.
	Semicolon,

	/// The lexical token for the `|` symbol.
	Bar,
}
