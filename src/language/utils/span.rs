use std::fmt::{Display, Formatter, Result};

use super::*;

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
