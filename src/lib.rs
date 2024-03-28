pub mod command;
pub mod compiler;

use anyhow::Result;
use command::Command;

/// Runs the command given on the command-line interface.
///
/// ### Arguments
/// - `command` - The command to run.
///
/// ### Returns
/// A `Result` indicating success or failure.
pub fn run(command: Command) -> Result<()>
{
	match command
	{
		Command::New { project_name } => command::new(project_name),
		Command::Build { file_path, output } => command::build(file_path, output),
		Command::Run { file_path } => command::run(file_path),
		Command::Check { file_path } => command::check(file_path),
	}
}
