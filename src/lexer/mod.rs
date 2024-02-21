use anyhow::{bail, Result};
use std::iter::Peekable;
use token::identifier::{Identifier, IdentifierSymbol};
use token::literal::Literal;
use token::operator::Operator;
use token::separator::Separator;
use token::{Kind::*, Token};

pub mod token;

pub fn scan<T>(mut stream: Peekable<T>) -> Result<Vec<Token>>
where
    T: Iterator<Item = char>,
{
    let mut tokens = Vec::new();

    while let Some(&symbol) = stream.peek() {
        let kind = match symbol {
            _ if symbol.is_whitespace() => {
                stream.next();
                Whitespace
            }
            _ if symbol.is_identifier_symbol() => Identifier(Identifier::new(&mut stream)),
            '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' => Separator(Separator::new(&mut stream)),
            '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' => {
                Operator(Operator::new(&mut stream))
            }

            // TODO: What about negative numbers?
            '"' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Literal(Literal::new(&mut stream))
            }
            _ => bail!("Unexpected symbol: {symbol}"),
        };

        if kind != Whitespace {
            tokens.push(Token::new(kind));
            eprintln!("{:?}", tokens.last().unwrap());
        }
    }

    Ok(tokens)
}