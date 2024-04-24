use super::Synthesis;
use crate::language::grammar::declaration::{Declaration, Kind};

impl Synthesis for Declaration
{
	fn synthesise(self) -> std::string::String
	{
		match self.kind
		{
			Kind::Variable { name, value, .. } =>
			{
				let value = value.synthesise();

				format!("{name} = {value}")
			}

			Kind::Function => unreachable!(),
		}
	}
}
