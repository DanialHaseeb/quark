use anyhow::Result;
use clap::Parser;

use super::*;

/// The command line arguments for the Quark CLI.
///
/// This enum represents the different commands that the user can run with the
/// Quark CLI.  Each command has its own set of arguments that can be passed in
/// from the command line.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Command
{
	/// Creates a new Quark project.
	New
	{
		/// The name of the new project.
		project: String,
	},

	/// Compiles your Quark file to valid Python.
	Build
	{
		/// The (relative) path to the Quark source file.
		#[clap(default_value = SOURCE)]
		input: String,

		/// The (relative) path to the output Python file.
		#[clap(short, long, default_value = TARGET)]
		output: String,
	},

	/// Compiles and executes your Quark code.
	Run
	{
		/// The (relative) path to the Quark file.
		#[clap(default_value = SOURCE)]
		input: String,
	},

	/// Checks your current project for errors.
	Check
	{
		/// The (relative) path to the Quark file.
		#[clap(default_value = SOURCE)]
		input: String,
	},
}

impl Command
{
	/// Runs the command given on the command-line interface.
	///
	/// ### Errors
	/// * If there are any issues running the command.
	pub fn run(&self) -> Result<()>
	{
		match self
		{
			Self::New { project } => new(project),
			Self::Build { input, output } => build(input, output),
			Self::Run { input } => run(input),
			Self::Check { input } => check(input),
		}
	}
}
