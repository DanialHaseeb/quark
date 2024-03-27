use anyhow::Result;
use clap::Parser;
use quark::command::Command;

fn main() -> Result<()>
{
	// let args = Args::parse();
	// let source = fs::read_to_string(args.file).context("Failed to read input file
	// 🤕")?; let compiled_code = quark::compile(source).context("Failed to compile
	// Quark code 💥")?;

	// if args.python
	// {
	// 	fs::write("output.py", compiled_code).with_context(|| "Failed to write to
	// output.py ❎")?; 	println!("Output written to output.py 🐍\n./output.py");
	// }
	// else
	// {
	// 	println!("{}", compiled_code);
	// }
	let command = Command::parse();
	quark::run(command)?;
	Ok(())
}
