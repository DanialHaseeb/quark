use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::{fs, process};

use anyhow::{ensure, Context, Result};

use super::*;
use crate::compiler::Compile;

/// The default name of the Quark source file.
pub const SOURCE: &str = "source.q";

/// The default name of the target Python file.
pub const TARGET: &str = "target.py";

/// The default name of the README file.
pub const README: &str = "README.md";

/// The default contents of the Quark source file.
pub const SOURCE_CONTENTS: &str = r#"print("Hello, World!")"#;

/// The default contents of the README file.
pub const README_CONTENTS: &str = r##"# Quark Project"##;

/// The default Python interpreter used to run the compiled output.
pub const PYTHON: &str = "python3";

/// Creates a new Quark project.
///
/// ### Parameters
/// * `project` - The name of the new project.
///
/// ### Errors
/// * If the project directory cannot be created.
/// * If the main Quark file cannot be created.
/// * If the README file cannot be created.
pub fn new(project: &str) -> Result<()>
{
	let project = Path::new(project);
	let source = project.join(SOURCE);
	let readme = project.join(README);

	fs::create_dir(project).context(error::CREATE_DIRECTORY)?;
	fs::write(source, SOURCE_CONTENTS).context(error::CREATE_SOURCE)?;
	fs::write(readme, README_CONTENTS).context(error::CREATE_README)
}

/// Compiles your Quark file to valid Python.
///
/// ### Parameters
/// * `input` - The (relative) path to the Quark source file.
/// * `output` - The (relative) path to the target Python file.
///
/// ### Errors
/// * If the input file does not have the correct extension.
/// * If the output file does not have the correct extension.
/// * If the Quark file cannot be read.
/// * If the Quark code cannot be compiled.
/// * If the Python code cannot be written to the output file.
pub fn build(input: &str, output: &str) -> Result<()>
{
	let input = Path::new(input);
	let extension = input.extension().map(OsStr::to_str);
	ensure!(extension == Some(Some("q")), error::SOURCE_EXTENSION);

	let output = Path::new(output);
	let extension = output.extension().map(OsStr::to_str);
	ensure!(extension == Some(Some("py")), error::TARGET_EXTENSION);

	let source = fs::read_to_string(input).context(error::READ_SOURCE)?;
	let target = source.compile()?;

	fs::write(output, target).context(error::CREATE_TARGET)
}

/// Compiles and executes your Quark code.
///
/// ### Parameters
/// * `input` - The (relative) path to the Quark source file.
///
/// ### Errors
/// * If the Quark file cannot be read.
/// * If the Quark code cannot be compiled.
/// * If the target Python file cannot be created.
/// * If the target Python file cannot be run.
/// * If the output cannot be written to the console.
pub fn run(input: &str) -> Result<()>
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

/// Checks your current project for errors.
///
/// ### Parameters
/// * `input` - The (relative) path to the Quark source file.
///
/// ### Errors
/// * If the Quark file cannot be read.
/// * If the Quark code cannot be compiled.
pub fn check(input: &str) -> Result<()>
{
	let input = Path::new(input);
	let extension = input.extension().map(OsStr::to_str);
	ensure!(extension == Some(Some("q")), error::SOURCE_EXTENSION);

	let source = fs::read_to_string(input).context(error::READ_SOURCE)?;
	source.compile()?;

	Ok(eprintln!("No errors found."))
}
