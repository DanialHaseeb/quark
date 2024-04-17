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

	/// The `and` operator.
	And,

	/// The `or` operator.
	Or,

	/// The `not` operator.
	Not,

	/// The `xor` operator.
	Xor,

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

impl Kind
{
	/// Checks if this kind of token is a declarator.
	///
	/// A declarator is a token that declares a constant, variable, function, or
	/// procedure.
	///
	/// #### Returns
	/// * `true` if the token is a declarator.
	/// * `false` otherwise.
	///
	/// ### Examples
	/// ```
	/// use quark::language::token::Kind;
	/// assert!(Kind::Constant.is_declarator());
	/// assert!(Kind::Variable.is_declarator());
	/// assert!(!Kind::And.is_declarator());
	/// assert!(!Kind::Plus.is_declarator());
	/// ```
	pub const fn is_declarator(&self) -> bool
	{
		match self
		{
			Self::Constant => true,
			Self::Variable => true,
			Self::Function => true,
			Self::Procedure => true,
			_ => false,
		}
	}
}
