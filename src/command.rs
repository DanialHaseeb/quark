use clap::Parser;

/// The command line arguments.
#[derive(Parser)]
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
