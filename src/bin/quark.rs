use anyhow::{Context, Result};
use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input Quark file
    #[clap(index = 1, required = true)]
    file: String,

    /// Output Python file
    #[clap(short = 'p', long)]
    python: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let compiled_code = quark::compile(args.file.clone())?;

    if args.python {
        fs::write("output.py", compiled_code).with_context(|| "Failed to write to output.py")?;
        println!("Python file written to output.py");
    } else {
        println!("{}", compiled_code);
    }

    Ok(())
}
