use anyhow::Result;

use crate::compiler::parser::parse;
use crate::language::grammar::*;
use crate::language::lexicon::Token;
use crate::language::utils::{Position, Span};

/// Types that can be parsed.
///
/// This trait is used to convert Quark lexical tokens into an abstract syntax
/// tree.
pub trait Parse
{
	/// Parses the Quark code into an abstract syntax tree.
	///
	/// ### Parameters
	/// * `source` - The source code to parse.
	///
	/// ### Returns
	/// * The abstract syntax tree.
	///
	/// ### Errors
	/// * If the Quark code cannot be parsed.
	fn parse(self, source: &[Vec<char>]) -> Result<parse::Tree>;
}

/// The abstract syntax tree.
///
/// This tree represents the structure of the Quark source code.
pub struct Tree(pub Programme);

impl Parse for Vec<Token>
{
	fn parse(self, source: &[Vec<char>]) -> Result<parse::Tree>
	{
		let mut stream = self.into_iter().peekable();

		let mut statements = Vec::new();

		while let Some(statement) = Statement::try_from_stream(&mut stream, source)?
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

		Ok(Tree(Programme { span, statements }))
	}
}
