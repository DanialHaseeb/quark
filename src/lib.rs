pub mod command;
pub mod generator;
pub mod lexer;
pub mod parser;

use anyhow::Result;
use command::Command;

pub fn run(command: Command) -> Result<()>
{
	match command
	{
		Command::New { project_name } => todo!(),
		Command::Build { file_path, output } => todo!(),
		Command::Run { file_path } => todo!(),
		Command::Check { file_path } => todo!(),
	}
}

pub fn compile(source: String) -> Result<String>
{
	let tokens = lexer::lex(source)?;
	let syntax = parser::parse(tokens)?;
	let interm = generator::intermediate(syntax);
	let python = generator::python(interm);
	Ok(python)
}
