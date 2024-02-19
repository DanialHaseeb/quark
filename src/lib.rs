use std::fs;
use anyhow::Result;

pub fn compile(file: String) -> Result<()>
{
	let source = fs::read_to_string(file)?;
	let stream = source.chars();
	let tokens = lex(stream)?;
	let syntax = parse(tokens)?;
	let target = translate(syntax);
	Ok(println!("{target}"))
}

struct Token
{

}

struct Tree
{

}

fn lex(stream: impl Iterator<Item = char>) -> Result<Vec<Token>>
{
	let mut stream = stream.enumerate().peekable();
	unimplemented!()
}

fn parse(tokens: Vec<Token>) -> Result<Tree>
{
	unimplemented!()
}

fn translate(syntax: Tree) -> String
{
	unimplemented!()
}
