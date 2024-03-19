use anyhow::Result;
use itertools::peek_nth;
use parser::Tree;
use std::fs;
pub mod lexer;
pub mod parser;

pub fn compile(file: String) -> Result<()> {
    let source = fs::read_to_string(file)?;
    let stream = peek_nth(source.chars());
    let tokens = lexer::scan(stream)?;
    let syntax = parser::generate(tokens)?;
    let target = translate(syntax);
    Ok(println!("{target}"))
}

fn translate(syntax: parser::Tree) -> String {
    todo!()
}
