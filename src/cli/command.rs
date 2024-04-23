use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::{fs, process};

use anyhow::{ensure, Context, Result};
use clap::Parser;

use super::*;
use crate::compiler::Compile;

/// The default name of the Quark source file.
const SOURCE: &str = "source.q";

/// The default name of the target Python file.
const TARGET: &str = "target.py";

/// The default name of the README file.
const README: &str = "README.md";

/// The default contents of the Quark source file.
const SOURCE_CONTENTS: &str = r#"print("Hello, World!")"#;

/// The default contents of the README file.
const README_CONTENTS: &str = r##"# Quark Project"##;

/// The default Python interpreter used to run the compiled output.
const PYTHON: &str = "python3";

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
			Self::New { project } =>
			{
				let project = Path::new(project);
				let source = project.join(SOURCE);
				let readme = project.join(README);

				fs::create_dir(project).context(error::CREATE_DIRECTORY)?;
				fs::write(source, SOURCE_CONTENTS).context(error::CREATE_SOURCE)?;
				fs::write(readme, README_CONTENTS).context(error::CREATE_README)
			}

			Self::Build { input, output } =>
			{
				let input = Path::new(input);
				let extension = input.extension().and_then(OsStr::to_str);
				ensure!(extension == Some("q"), error::SOURCE_EXTENSION);

				let output = Path::new(output);
				let extension = output.extension().and_then(OsStr::to_str);
				ensure!(extension == Some("py"), error::TARGET_EXTENSION);

				let source = fs::read_to_string(input).context(error::READ_SOURCE)?;
				let target = source.compile()?;

				fs::write(output, target).context(error::CREATE_TARGET)
			}

			Self::Run { input } =>
			{
				let input = Path::new(input);
				let extension = input.extension().map(OsStr::to_str);
				ensure!(extension == Some(Some("q")), error::SOURCE_EXTENSION);

				let source = fs::read_to_string(input).context(error::READ_SOURCE)?;
				let target = source.compile()?;
				let file = Path::new(TARGET);

				fs::write(file, target).context(error::CREATE_TARGET)?;

				let output = process::Command::new(PYTHON)
					.arg(file)
					.output()
					.context(error::RUN_TARGET)?
					.stdout;

				io::stdout().write_all(&output).context(error::OUTPUT)
			}

			Self::Check { input } =>
			{
				let input = Path::new(input);
				let extension = input.extension().map(OsStr::to_str);
				ensure!(extension == Some(Some("q")), error::SOURCE_EXTENSION);

				let source = fs::read_to_string(input).context(error::READ_SOURCE)?;
				source.compile()?;

				Ok(eprintln!("No errors found."))
			}
		}
	}
}
