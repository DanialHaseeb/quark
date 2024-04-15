use anyhow::Result;
use clap::Parser;

use quark::interface::Command;

fn main() -> Result<()>
{
	Command::parse().run()
}
