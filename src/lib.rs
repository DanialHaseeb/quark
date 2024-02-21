use crate::lexer::token::Token;
use anyhow::Result;
use std::fs;
pub mod lexer;
pub mod parser;

pub fn compile(file: String) -> Result<()> {
    let source = fs::read_to_string(file)?;
    let stream = source.chars().peekable();
    let tokens = lexer::scan(stream)?;
    let syntax = parse(tokens)?;
    let target = translate(syntax);
    Ok(println!("{target}"))
}

struct Tree {}

fn parse(tokens: Vec<Token>) -> Result<Tree> {
    todo!()
}

fn translate(syntax: Tree) -> String {
    todo!()
}
