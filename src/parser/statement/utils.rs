use crate::lexer::token::{separator::SeparatorKind, Token, TokenKind::*};
use anyhow::{bail, Result};
use std::iter::Peekable;

pub fn consume_if_matches<T>(tokens_iter: &mut Peekable<T>, separator: SeparatorKind) -> Result<()>
where
    T: Iterator<Item = Token>,
{
    match tokens_iter.peek() {
        Some(Token {
            token_kind: Separator(kind),
            ..
        }) if *kind == separator => {
            tokens_iter.next();
            Ok(())
        }
        _ => bail!("Expected `{}` after expression", separator),
    }
}
