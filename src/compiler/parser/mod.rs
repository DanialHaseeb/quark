pub mod parser;

pub use parser::*;

pub mod declaration;
pub mod utils;

use core::fmt;

use self::declaration::{declaration, Declaration};
use super::generator::CodeGenerator;

use super::lexer::token::Token;
use anyhow::Result;

pub struct Programme(pub Vec<Declaration>);

pub fn parse(tokens: Vec<Token>) -> Result<Programme>
{
	let mut tokens = tokens.into_iter().peekable();
	let mut declarations = Vec::new();

	while tokens.peek().is_some()
	{
		let declaration = declaration(&mut tokens)?;
		declarations.push(declaration);
	}

	Ok(Programme(declarations))
}

impl fmt::Display for Programme
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		for declaration in &self.0
		{
			write!(f, "{}", declaration)?;
		}
		Ok(())
	}
}

impl CodeGenerator for Programme
{
	fn generate(&self) -> String
	{
		let mut output = String::new();
		for declaration in &self.0
		{
			output.push_str(&declaration.generate());
		}
		output
	}
}
