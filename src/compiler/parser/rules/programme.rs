use std::iter::Peekable;

use anyhow::Result;

use crate::language::grammar::{Programme, Statement};
use crate::language::lexicon::Token;
use crate::language::utils::{Position, Span};

impl Programme
{
	/// Creates a programme from a stream of tokens.
	///
	/// ### Parameters
	/// * `stream` - The stream of tokens.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The programme if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	///
	/// ### Errors
	/// * If the programme cannot be created.
	pub fn try_from_stream<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Programme>
	where
		I: Iterator<Item = Token>,
	{
		let mut statements = Vec::new();

		while let Some(statement) = Statement::try_from_stream(stream, source)?
		{
			statements.push(statement);
		}

		let span = match (statements.first(), statements.last())
		{
			(Some(first), Some(last)) => Span {
				start: first.span.start,
				end: last.span.end,
			},
			_ => Span {
				start: Position { line: 1, column: 1 },
				end: Position { line: 1, column: 1 },
			},
		};

		Ok(Programme { span, statements })
	}
}
