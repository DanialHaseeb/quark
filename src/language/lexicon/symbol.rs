use crate::language::utils::Position;

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
	pub fn new(line: usize, column: usize, value: char) -> Self
	{
		let position = Position { line, column };
		Self { position, value }
	}
}
