use std::iter::Peekable;

use anyhow::Result;

use crate::language::{Symbol, Token};

impl Token
{
	/// Creates
	pub fn try_from_symbol<I>(
		stream: &mut Peekable<I>,
		source: &[Vec<char>],
	) -> Result<Token>
	where
		I: Iterator<Item = Symbol>,
	{
		todo!()
	}
}
