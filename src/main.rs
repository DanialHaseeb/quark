use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile Quark code to Python and print to stdout
    #[command(arg_required_else_help = true)]
    Default {
        /// Input Quark file
        #[clap(index = 1, required = true, value_name = "FILE_NAME")]
        input: String,
    },

    /// Compile Quark code to Python and run it
    #[command(arg_required_else_help = true)]
    Run {
        /// Input Quark file
        #[clap(index = 1, required = true, value_name = "FILE_NAME")]
        input: String,
    },

    /// Compile Quark code to Python and write to a file
    #[command(arg_required_else_help = true)]
    Output {
        /// Input Quark file
        #[clap(index = 1, required = true, value_name = "FILE_NAME")]
        input: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output = "output.py";

    match args.command {
        Commands::Default { input } => {
            let compiled_code = read_and_compile(input)?;
            println!("{}", compiled_code);
        }
        Commands::Run { input } => {
            write_to_file(input, output)?;

            let output = Command::new("python")
                .arg("output.py")
                .output()
                .context("Failed to execute Python 🐍 script")?;

            let output = String::from_utf8_lossy(&output.stdout);
            println!("{}", output);
        }
        Commands::Output { input } => {
            write_to_file(input, output)?;

            println!("Output written to output.py 🐍\n./output.py");
        }
    }

    Ok(())
}

fn read_and_compile(file_name: String) -> Result<String> {
    let source = fs::read_to_string(file_name).context("Failed to read input file 🤕")?;
    let compiled_code = quark::compile(source).context("Failed to compile Quark code 💥")?;

    Ok(compiled_code)
}

fn write_to_file(file_name: String, output: &str) -> Result<()> {
    let compiled_code = read_and_compile(file_name)?;
    fs::write(output, compiled_code).with_context(|| "Failed to write to output.py ✍️")?;

    Ok(())
}
