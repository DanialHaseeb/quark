use anyhow::Result;
use clap::Parser;

use quark::cli::Command;

fn main() -> Result<()>
{
	Command::parse().run()
}
