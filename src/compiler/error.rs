use crate::language::{Position, Span};

pub trait Error
{
	/// Creates a pretty string that reports an error.
	///
	/// ### Parameters
	/// * `span` - The span of the error.
	/// * `message` - The error message.
	///
	/// ### Returns
	/// * The pretty string that reports an error.
	fn error(&self, span: Span, message: &str) -> String;
}

impl Error for &[Vec<char>]
{
	fn error(&self, Span { start, end }: Span, message: &str) -> String
	{
		let Position { line, column } = start;

		let header = format!("--> {start}--{end}");
		let prefix = format!("{line} | ");
		let source = self[line].iter().collect::<String>();
		let indent = " ".repeat(prefix.len() + column);
		let length = end.column - start.column + 1;
		let arrows = "^".repeat(length);

		format!(
		"{header}
		{prefix}{source}
		{indent}{arrows}
		{message}")
	}
}
