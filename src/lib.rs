pub mod generator;
pub mod lexer;
pub mod parser;

use anyhow::Result;
use std::fs;

pub fn compile(file: String) -> Result<String> {
    let source = fs::read_to_string(file)?;
    let tokens = lexer::lex(source)?;
    let syntax = parser::parse(tokens)?;
    let target = generator::intermediate(syntax);
    let python = generator::python(target);
    Ok(python)
}
