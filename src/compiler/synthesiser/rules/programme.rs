use super::Synthesis;
use crate::language::grammar::Programme;

impl Synthesis for Programme
{
	fn synthesise(self) -> String
	{
		let mut result = String::new();

		for statement in self.statements
		{
			result.push_str(&statement.synthesise());
		}

		result
	}
}
