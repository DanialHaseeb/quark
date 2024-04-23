use std::iter::Peekable;

use anyhow::Result;

use crate::language::lexicon::{Token, Symbol};

impl Token
{
	/// Creates a token from a stream that starts with whitespace(s).
	///
	/// ### Parameters
	/// * `stream` - The stream of symbols.
	/// * `source` - The source code.
	///
	/// ### Returns
	/// * The next token if it can be constructed from the stream.
	/// * `None` if the stream is empty.
	///
	/// ### Errors
	/// * If the token cannot be created.
	pub fn try_from_whitespace<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Option<Token>>
	where
		I: Iterator<Item = Symbol>,
	{
		while stream.next_if(Symbol::is_whitespace).is_some()
		{}

		Self::try_from_stream(stream, source)
	}
}

impl Symbol
{
	/// Determines if the symbol is a whitespace character.
	///
	/// ### Returns
	/// * `true` if the symbol is a whitespace character.
	/// * `false` otherwise.
	///
	/// ### Examples
	/// ```rust
	/// use quark::language::Symbol;
	///
	/// let symbol = Symbol::new(0, 0, ' ');
	/// assert!(symbol.is_whitespace());
	///
	/// let symbol = Symbol::new(0, 0, '\t');
	/// assert!(symbol.is_whitespace());
	///
	/// let symbol = Symbol::new(0, 0, '\n');
	/// assert!(symbol.is_whitespace());
	///
	/// let symbol = Symbol::new(0, 0, 'a');
	/// assert!(!symbol.is_whitespace());
	/// ```
	pub fn is_whitespace(&self) -> bool
	{
		self.value.is_whitespace()
	}
}
