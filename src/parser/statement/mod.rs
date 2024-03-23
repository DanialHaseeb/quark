pub mod expression;
pub mod utils;

use std::fmt;
use std::iter::Peekable;

use crate::lexer::token::separator::SeparatorKind::*;
use crate::lexer::token::Token;
use anyhow::Result;
use expression::grammar::expression;
use expression::structs::ExpressionKind;

use StatementKind::*;

use self::utils::consume_if_matches;

pub enum StatementKind {
    Expresssion(ExpressionKind),
    // TODO: implement other statement types like `print`, `block`, etc.
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
    consume_if_matches(tokens_iter, Semicolon)?;
    Ok(Expresssion(expression))
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expresssion(expression) => writeln!(f, "{}", expression),
        }
    }
}
