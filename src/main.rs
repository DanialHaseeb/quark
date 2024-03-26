use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    /// Input Quark file
    #[clap(index = 1, required = true)]
    file: String,

    /// Output Python file
    #[clap(short = 'p', long)]
    python: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let source = fs::read_to_string(args.file)?;
    let compiled_code = quark::compile(source)?;

    if args.python {
        fs::write("output.py", compiled_code).with_context(|| "Failed to write to output.py ❎")?;
        println!("Output written to output.py 🐍\n./output.py");
    } else {
        println!("{}", compiled_code);
    }

    Ok(())
}
