use std::iter::Peekable;

use crate::language::{Symbol, Token};

impl Token
{
	/// Creates
	pub fn from_operator_head<I>(stream: &mut Peekable<I>) -> Token
	where I: Iterator<Item = Symbol>
	{
		todo!()
	}
}
