pub mod lexer;
pub mod parser;

use anyhow::Result;
use parser::Program;
use std::fs;

pub fn compile(file: String) -> Result<()> {
    let source = fs::read_to_string(file)?;
    let tokens = lexer::lex(source)?;
    let syntax = parser::parse(tokens)?;
    let target = translate(syntax);
    Ok(println!("{target}"))
}

fn translate(syntax: Program) -> String {
    todo!()
}
