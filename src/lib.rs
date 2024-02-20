use anyhow::Result;
use std::fs;

pub fn compile(file: String) -> Result<()>
{
	let source = fs::read_to_string(file)?;
	let stream = source.chars();
	let tokens = lex(stream)?;
	let syntax = parse(tokens)?;
	let target = translate(syntax);
	Ok(println!("{target}"))
}

struct Token {}

enum TokenType
{
	// Single-character tokens
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// One or two character tokens
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals
	Identifier,
	String,
	Number,

	// Keywords
	And,
	Else,
	False,
	Fun,
	For,
	If,
	Nil,
	Or,
	Print,
	Return,
	True,
	While,
	// Class,
	// This,
	// Mut,
	Eof,
}

struct Tree {}

fn lex(stream: impl Iterator<Item = char>) -> Result<Vec<Token>>
{
	let mut stream = stream.enumerate().peekable();
	todo!()
}

fn parse(tokens: Vec<Token>) -> Result<Tree>
{
  todo!()
}

fn translate(syntax: Tree) -> String
{
  todo!()
}
