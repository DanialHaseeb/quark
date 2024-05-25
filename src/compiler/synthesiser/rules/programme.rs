use super::Synthesis;
use crate::language::grammar::Programme;

impl Synthesis for Programme
{
	fn synthesise(self) -> String
	{
		self
			.statements
			.into_iter()
			.map(|statement| statement.synthesise())
			.collect::<Vec<_>>()
			.join("")
	}
}
