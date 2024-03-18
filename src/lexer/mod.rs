use anyhow::{bail, Result};
use itertools::structs::PeekNth;
use token::identifier::{Identifier, IdentifierSymbol};
use token::literal::Literal;
use token::operator::Operator;
use token::separator::Separator;
use token::{Kind::*, Token};

pub mod token;

pub fn scan<T>(mut stream: PeekNth<T>) -> Result<Vec<Token>>
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
            '"' => Literal(Literal::new(&mut stream)),
            _ if symbol.is_ascii_digit() => Literal(Literal::new(&mut stream)),
            _ => bail!("Unexpected symbol: {symbol}"),
        };

        if kind != Whitespace {
            tokens.push(Token::new(kind));
            eprintln!("{:?}", tokens.last().unwrap());
        }
    }

    Ok(tokens)
}
