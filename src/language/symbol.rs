use super::*;

/// A symbol in a source file.
///
/// The symbol is given by a position and a value. The position is the position
/// of the symbol in the source file. The value is the character that represents
/// the symbol.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Symbol
{
	/// The position of the symbol in the source file.
	pub position: Position,

	/// The character that represents the symbol.
	pub value: char,
}

impl Symbol
{
	/// Creates a new symbol.
	///
	/// ### Parameters
	/// * `line` - The line of the symbol in source code.
	/// * `column` - The column of the symbol in source code.
	/// * `value` - The raw `char` value of the symbol.
	///
	/// ### Returns
	/// * The new symbol created from the given line, column, and value.
	///
	/// ### Examples
	/// ```rust
	/// use quark::language::{Position, Symbol};
	/// let position = Position { line: 0, column: 0 };
	/// let symbol = Symbol::new(0, 0, 'a');
	/// assert_eq!(symbol.position, position);
	/// assert_eq!(symbol.value, 'a');
	/// ```
	pub fn new(line: usize, column: usize, value: char) -> Self
	{
		let position = Position { line, column };
		Self { position, value }
	}

	/// Creates a vector of symbols.
	///
	/// ### Parameters
	/// * `line` - The line number of each symbol in the vector.
	/// * `string` - The string of characters to be converted.
	///
	/// ### Returns
	/// * A vector of symbols created from the given string.
	///
	/// ### Examples
	/// ```rust
	/// use quark::language::Symbol;
	/// let symbols = Symbol::vector(0, "abc");
	/// assert_eq!(symbols.len(), 3);
	/// assert_eq!(symbols[0].value, 'a');
	/// assert_eq!(symbols[1].value, 'b');
	/// assert_eq!(symbols[2].value, 'c');
	/// ```
	pub fn vector((line, string): (usize, &str)) -> Vec<Self>
	{
		format!("{string}\n")
			.chars()
			.enumerate()
			.map(|(column, value)| Symbol::new(line, column, value))
			.collect()
	}

	/// Checks if the symbol is a whitespace character.
	///
	/// ### Returns
	/// * `true` if the symbol is a whitespace character.
	/// * `false` otherwise.
	///
	/// ### Examples
	/// ```rust
	/// use quark::language::{Position, Symbol};
	/// let position = Position { line: 0, column: 0 };
	/// let symbol = Symbol { position, value: ' ' };
	/// assert!(symbol.is_whitespace());
	/// let symbol = Symbol { position, value: 'a' };
	/// assert!(!symbol.is_whitespace());
	/// ```
	pub fn is_whitespace(&self) -> bool
	{
		self.value.is_whitespace()
	}

	/// Checks if the symbol is a starting symbol of an identifier token.
	///
	/// ### Returns
	/// * `true` if the symbol is a starting symbol of an identifier token.
	/// * `false` otherwise.
	///
	/// ### Examples
	/// ```rust
	/// use quark::language::{Position, Symbol};
	/// let position = Position { line: 0, column: 0 };
	/// let symbol = Symbol { position, value: 'a' };
	/// assert!(symbol.is_identifier_start());
	/// let symbol = Symbol { position, value: '1' };
	/// assert!(!symbol.is_identifier_start());
	/// ```
	pub fn is_identifier_head(&self) -> bool
	{
		self.value.is_alphabetic() || self.value == '_'
	}

	/// Checks if the symbol is a starting symbol of a number token.
	///
	/// ### Returns
	/// * `true` if the symbol is a starting symbol of a number token.
	/// * `false` otherwise.
	///
	/// ### Examples
	/// ```rust
	/// use quark::language::{Position, Symbol};
	/// let position = Position { line: 0, column: 0 };
	/// let symbol = Symbol { position, value: '1' };
	/// assert!(symbol.is_number_head());
	/// let symbol = Symbol { position, value: 'a' };
	/// assert!(!symbol.is_number_head());
	/// let symbol = Symbol { position, value: '.' };
	/// assert!(symbol.is_number_head());
	/// ```
	pub fn is_number_head(&self) -> bool
	{
		self.value.is_ascii_digit() || self.value == '.'
	}
}
