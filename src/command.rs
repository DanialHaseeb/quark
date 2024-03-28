use std::io::Write;

use anyhow::{Context, Result};

/// The command line arguments.
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub enum Command
{
	/// Creates a new Quark project
	New
	{
		/// The name of the new project
		project_name: String,
	},
	/// Compiles your Quark file to valid Python
	Build
	{
		/// The (relative) path to the Quark file
		#[clap(default_value = "main.q")]
		file_path: String,
		/// The name of the output file
		#[clap(short, long, default_value = "target.py")]
		output: String,
	},
	/// Compiles and executes your Quark code
	Run
	{
		/// The (relative) path to the Quark file
		#[clap(default_value = "main.q")]
		file_path: String,
	},
	/// Checks your current project for errors
	Check
	{
		/// The (relative) path to the Quark file
		#[clap(default_value = "main.q")]
		file_path: String,
	},
}

/// Creates a new Quark project.
///
/// ### Arguments
/// - `project_name` - The name of the new project
///
/// ### Returns
/// A `Result` indicating success or failure.
pub fn new(project_name: String) -> Result<()>
{
	// 1. Create new directory inside current directory with the project name.
	// 2. Create new file inside project directory called `main.q`.
	// 3. Create `REAMDE.md` file inside project directory.

	let project_path = std::path::Path::new(&project_name);
	std::fs::create_dir(project_path)
		.context("Failed to create project directory 📁")?;
	std::fs::write(project_path.join("main.q"), "")
		.context("Failed to create main.q file 📄")?;
	std::fs::write(project_path.join("README.md"), "")
		.context("Failed to create README.md file 📄")
}

/// Compiles your Quark file to valid Python.
///
/// ### Arguments
/// - `file_path` - The (relative) path to the Quark file
/// - `output` - The name of the output file
///
/// ### Returns
/// A `Result` indicating success or failure.
pub fn build(file_path: String, output: String) -> Result<()>
{
	// 1. Read the contents of the Quark file.
	// 2. Compile the Quark code to Python.
	// 3. Write the Python code to the output file.

	let source = std::fs::read_to_string(file_path)
		.context("Failed to read input file 🤕")?;
	let target = super::compiler::compile(source)
		.context("Failed to compile Quark code 💥")?;
	std::fs::write(output, target).context("Failed to write to output file ❎")
}

/// Compiles and executes your Quark code.
///
/// ### Arguments
/// - `file_path` - The (relative) path to the Quark file
///
/// ### Returns
/// A `Result` indicating success or failure.
pub fn run(file_path: String) -> Result<()>
{
	// 1. Compile Quark code to Python.
	// 2. Create target Python file.
	// 3. Execute target Python file.
	// 4. Print output to console.

	let source = std::fs::read_to_string(file_path)
		.context("Failed to read input file 🤕")?;
	let target = super::compiler::compile(source)
		.context("Failed to compile Quark code 💥")?;
	let target_file = std::path::Path::new("target").with_extension("py");
	std::fs::write(&target_file, target)
		.context("Failed to write to target.py ❎")?;
	let output = std::process::Command::new("python3")
		.arg(&target_file)
		.output()
		.context("Failed to execute target.py 🚀")?;
	std::io::stdout()
		.write_all(&output.stdout)
		.context("Failed to write to stdout 📭")
}

/// Checks your current project for errors.
///
/// ### Arguments
/// - `file_path` - The (relative) path to the Quark file
///
/// ### Returns
/// A `Result` indicating success or failure.
pub fn check(file_path: String) -> Result<()>
{
	// 1. Read the contents of the Quark file.
	// 2. Compile the Quark code to Python.
	// 3. Report success or failure.

	let source = std::fs::read_to_string(&file_path)
		.context("Failed to read input file 🤕")?;
	crate::compiler::compile(source)
		.context("Failed to compile Quark code 💥")?;
	Ok(eprintln!("No errors found in {} 🎉", file_path))
}
