use anyhow::Result;
use parser::Tree;
use std::fs;
pub mod lexer;
pub mod parser;

pub fn compile(file: String) -> Result<()> {
    let source = fs::read_to_string(file)?;
    let tokens = lexer::scan(source)?;
    let syntax = parser::generate(tokens)?;
    let target = translate(syntax);
    Ok(println!("{target}"))
}

fn translate(syntax: parser::Tree) -> String {
    todo!()
}
