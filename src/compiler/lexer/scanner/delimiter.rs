use std::iter::Peekable;

use crate::language::{Symbol, Token};

impl Token
{
	/// Creates
	pub fn from_delimiter<I>(stream: &mut Peekable<I>) -> Token
	where I: Iterator<Item = Symbol>
	{
		todo!()
	}
}
