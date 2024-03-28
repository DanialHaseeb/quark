pub mod command;
pub mod generator;
pub mod lexer;
pub mod parser;
pub mod compiler;

use anyhow::Result;
use command::Command;

pub fn run(command: Command) -> Result<()>
{
	match command
	{
		Command::New { project_name } => command::new(project_name),
		Command::Build { file_path, output } => command::build(file_path, output),
		Command::Run { file_path } => command::run(file_path),
		Command::Check { file_path } => command::check(file_path)
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
