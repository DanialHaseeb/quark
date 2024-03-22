use anyhow::Result;
use parser::expression::Expression;
use std::fs;
pub mod lexer;
pub mod parser;

pub fn compile(file: String) -> Result<()> {
    let source = fs::read_to_string(file)?;
    let tokens = lexer::lex(source)?;
    let syntax = parser::parse(tokens)?;
    println!("{}", syntax);
    let target = translate(syntax);
    Ok(println!("{target}"))
}

fn translate(syntax: Expression) -> String {
    todo!()
}
