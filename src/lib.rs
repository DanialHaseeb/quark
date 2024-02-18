use std::fs;
use std::iter::Peekable;
use anyhow::Result;

pub fn compile(file: String) -> Result<()>
{
	let source = fs::read_to_string(file)?;
	let stream = source.chars().peekable();
	let tokens = lex(stream)?;
	let syntax = parse(tokens)?;
	let target = translate(syntax);
	Ok(println!("{target}"))
}

enum Token
{

}

struct Tree
{

}

fn lex<T>(stream: Peekable<T>) -> Result<Vec<Token>>
where T: Iterator<Item = char>
{
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
