use std::fmt::{Display, Formatter, Result};

// MARK: Position

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

// MARK: Span

/// A span of symbols in a source file.
///
/// The span is given by the start and end positions of the span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span
{
	/// The start position of the span.
	pub start: Position,

	/// The end position of the span.
	pub end: Position,
}

impl Display for Span
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		let Self { start, end } = self;
		write!(formatter, "{start}--{end}")
	}
}

// MARK: Symbol

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

impl Display for Symbol
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		let Self { position, value } = self;
		write!(formatter, "{position}: {value}")
	}
}
