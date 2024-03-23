use crate::lexer::token::{identifier::IdentifierKind::*, Token, TokenKind, TokenKind::*};
use anyhow::{bail, Result};
use std::iter::Peekable;

pub fn consume_if_matches<T>(tokens_iter: &mut Peekable<T>, kind: TokenKind) -> Result<()>
where
    T: Iterator<Item = Token>,
{
    match tokens_iter.peek() {
        Some(Token { token_kind }) if *token_kind == kind => {
            tokens_iter.next();
            Ok(())
        }
        _ => bail!("Expected `{}` after expression", kind),
    }
}

pub fn consume_and_return_if_variable<T>(tokens_iter: &mut Peekable<T>) -> Result<String>
where
    T: Iterator<Item = Token>,
{
    match tokens_iter.peek() {
        Some(Token {
            token_kind: Identifier(Variable(_)),
        }) => {
            let x = tokens_iter.next().unwrap();
            match x.token_kind {
                Identifier(Variable(identifier)) => Ok(identifier),
                _ => unreachable!(),
            }
        }
        _ => bail!("Expected variable after `let` keyword"),
    }
}
