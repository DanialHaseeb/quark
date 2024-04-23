use std::fmt::{Display, Formatter, Result};

/// The position of a symbol in a source file.
///
/// The position is given by a line number and a column number. The line number
/// is the 0-based index of the line in the source file. The column number is
/// the 0-based index of the column in the line.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position
{
	/// The 0-based index of the line in the source file.
	pub line: usize,

	/// The 0-based index of the column in the line.
	pub column: usize,
}

impl Display for Position
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		let Self { line, column } = self;
		write!(formatter, "[{line}; {column}]")
	}
}
