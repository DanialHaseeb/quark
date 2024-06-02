use std::collections::HashMap;

use crate::language::semantics::r#type::Type;

pub struct Table
{
	pub constants: HashMap<String, Type>,
	pub variables: HashMap<String, Type>,
	pub functions: HashMap<String, Type>,
}

impl Table
{
	pub fn new() -> Self
	{
		Self {
			constants: HashMap::new(),
			variables: HashMap::new(),
			functions: HashMap::new(),
		}
	}
}
