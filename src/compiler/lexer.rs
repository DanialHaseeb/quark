use anyhow::{bail, Result};
use itertools::peek_nth;
use token::{
	identifier::{Identifier, IdentifierKind},
	literal::LiteralKind,
	operator::OperatorKind,
	separator::SeparatorKind,
	Token,
	TokenKind::*,
};

pub mod token;

pub fn lex(source: String) -> Result<Vec<Token>>
{
	let mut stream = peek_nth(source.chars());

	let mut tokens = Vec::new();

	while let Some(&symbol) = stream.peek()
	{
		let token_type = match symbol
		{
			_ if symbol.is_whitespace() =>
			{
				stream.next();
				Whitespace
			}
			_ if symbol.is_identifier() =>
			{
				Identifier(IdentifierKind::new(&mut stream))
			}
			'(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' =>
			{
				Separator(SeparatorKind::new(&mut stream))
			}
			'+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' =>
			{
				Operator(OperatorKind::new(&mut stream))
			}

			'"' => Literal(LiteralKind::new(&mut stream)),
			_ if symbol.is_ascii_digit() => Literal(LiteralKind::new(&mut stream)),
			_ => bail!("Unexpected symbol: {symbol}"),
		};

		if token_type != Whitespace
		{
			tokens.push(Token::new(token_type));
		}
	}

	Ok(tokens)
}
