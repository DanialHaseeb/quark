use anyhow::Result;
use clap::Parser;
use quark::command::Command;

fn main() -> Result<()>
{
	let command = Command::parse();
	quark::run(command)
}
