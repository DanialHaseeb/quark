pub mod generator;
pub mod lexer;
pub mod parser;

use anyhow::Result;

pub fn compile(source: String) -> Result<String>
{
	let tokens = lexer::lex(source)?;
	let syntax = parser::parse(tokens)?;
	let interm = generator::intermediate(syntax);
	let python = generator::python(interm);
	Ok(python)
}
