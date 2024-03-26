use crate::lexer::token::{identifier::IdentifierKind::*, Token, TokenKind, TokenKind::*};
use anyhow::{bail, Result};
use std::iter::Peekable;

/// Consumes the next token if it matches the expected kind and Err otherwise
pub fn consumes<T>(tokens_iter: &mut Peekable<T>, kind: TokenKind) -> Result<()>
where
    T: Iterator<Item = Token>,
{
    match tokens_iter.peek() {
        Some(Token { token_kind }) if *token_kind == kind => {
            tokens_iter.next();
            Ok(())
        }
        _ => bail!("Parser expected `{}`", kind),
    }
}

/// Consumes the next token if it matches the expected kind otherwise does nothing
pub fn consumes_if_matches<T>(tokens_iter: &mut Peekable<T>, kind: TokenKind)
where
    T: Iterator<Item = Token>,
{
    match tokens_iter.peek() {
        Some(Token { token_kind }) if *token_kind == kind => {
            tokens_iter.next();
        }
        _ => (),
    }
}

pub fn consume_and_return_variable<T>(tokens_iter: &mut Peekable<T>) -> Result<String>
where
    T: Iterator<Item = Token>,
{
    if let Some(Token {
        token_kind: Identifier(Variable(_)),
    }) = tokens_iter.peek()
    {
        match tokens_iter.next().unwrap().token_kind {
            Identifier(Variable(x)) => Ok(x),
            _ => unreachable!(),
        }
    } else {
        bail!("Parser expected variable after `let` keyword")
    }
}
