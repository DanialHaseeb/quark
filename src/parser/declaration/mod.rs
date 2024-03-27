pub mod expression;
pub mod statement;

use anyhow::Result;
use std::{fmt, iter::Peekable};

use crate::{generator::CodeGenerator,
            lexer::token::{identifier::{IdentifierKind::*, KeywordKind},
                           operator::{OperatorKind::*, SingleCharKind::*},
                           separator::{Delimiter::*, SeparatorKind::*},
                           Token,
                           TokenKind::*}};

use super::utils::{consume_and_return_variable, consumes};
use expression::{grammar::expression, structs::ExpressionKind};
use statement::{statement, StatementKind};

use Declaration::*;

pub enum Declaration
{
	VariableDeclaration(VariableDeclarationBody),
	Statement(StatementKind),
}

pub struct Block(pub Vec<Declaration>);

pub struct VariableDeclarationBody
{
	pub identifier: String,
	pub expression: ExpressionKind,
}

// Grammar Rule:
// Declaration -> "let" VARIABLE "=" expression ";" | Statement;
pub fn declaration<T>(tokens_iter: &mut Peekable<T>) -> Result<Declaration>
	where T: Iterator<Item = Token>
{
	if let Some(Identifier(Keyword(KeywordKind::Let))) =
		tokens_iter.peek().map(|token| &token.token_kind)
	{
		tokens_iter.next().unwrap();
		let identifier = consume_and_return_variable(tokens_iter)?;

		consumes(tokens_iter, Operator(SingleChar(Equal)))?;

		let expr = expression(tokens_iter)?;

		consumes(tokens_iter, Separator(Semicolon))?;

		Ok(VariableDeclaration(VariableDeclarationBody { identifier,
		                                                 expression: expr }))
	}
	else
	{
		let statement_thing = statement(tokens_iter)?;
		Ok(Statement(statement_thing))
	}
}

impl fmt::Display for Declaration
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			VariableDeclaration(VariableDeclarationBody { identifier,
			                                              expression, }) =>
			{
				writeln!(f, "let {} = {};", identifier, expression)
			}
			Statement(statement) => write!(f, "{}", statement),
		}
	}
}

impl CodeGenerator for Declaration
{
	fn generate(&self) -> String
	{
		match self
		{
			VariableDeclaration(VariableDeclarationBody { identifier,
			                                              expression, }) =>
			{
				format!("{} = {};", identifier, &expression.generate())
			}
			Statement(statement) => statement.generate(),
		}
	}
}

impl CodeGenerator for Block
{
	fn generate(&self) -> String
	{
		let mut output = String::new();
		output.push('{');
		for statement in &self.0
		{
			output.push_str(&statement.generate());
		}
		output.push('}');
		output
	}
}

impl fmt::Display for Block
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		writeln!(f, "{{")?;
		for statement in &self.0
		{
			write!(f, "{}", statement)?;
		}
		write!(f, "}}")
	}
}

pub fn block<T>(tokens_iter: &mut Peekable<T>) -> Result<Block>
	where T: Iterator<Item = Token>
{
	consumes(tokens_iter, Separator(Left(Brace)))?;
	let mut declarations = Vec::new();
	while let Some(token) = tokens_iter.peek()
	{
		if token.token_kind == Separator(Right(Brace))
		{
			break;
		}
		declarations.push(declaration(tokens_iter)?);
	}
	consumes(tokens_iter, Separator(Right(Brace)))?;
	Ok(Block(declarations))
}
