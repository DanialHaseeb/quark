use anyhow::Result;
use std::fmt;
use std::iter::Peekable;

use crate::lexer::token::{separator::SeparatorKind::*, Token, TokenKind::*};

use super::super::utils::consume_if_matches;
use super::expression::{grammar::expression, structs::ExpressionKind};

use StatementKind::*;

// Statements are things that you can possible not store in variables like print operation.
pub enum StatementKind {
    Expresssion(ExpressionKind),
    // TODO: implement other statement types like `print`, `while`, `if`, `return`, `function` etc.
}
/// Grammar Rule:
/// statement -> expression_statement;
pub fn statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    let expresssion = expression_statement(tokens_iter)?;
    Ok(expresssion)
}

/// expression_statement -> expression  ";";
pub fn expression_statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    let expression = expression(tokens_iter)?;
    consume_if_matches(tokens_iter, Separator(Semicolon))?;
    Ok(Expresssion(expression))
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expresssion(expression) => writeln!(f, "{}", expression),
        }
    }
}
