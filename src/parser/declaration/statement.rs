use anyhow::Result;
use std::fmt;
use std::iter::Peekable;

use super::super::utils::consume_if_matches;
use super::expression::{grammar::expression, structs::ExpressionKind};
use crate::lexer::token::{
    identifier::{IdentifierKind::*, KeywordKind::*},
    separator::{Delimiter::*, SeparatorKind::*},
    Token,
    TokenKind::*,
};

use StatementKind::*;

// Statements are things that you can possible not store in variables like print operation.
pub enum StatementKind {
    ExpresssionStmt(ExpressionKind),
    PrintStmt(ExpressionKind),
    // TODO: implement other statement types like `print`, `while`, `if`, `return`, `function` etc.
}
/// Grammar Rule:
/// statement -> expression_statement | print_statement
pub fn statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    if let Some(Identifier(Keyword(Print))) = tokens_iter.peek().map(|token| &token.token_kind) {
        Ok(print_statement(tokens_iter)?)
    } else {
        Ok(expression_statement(tokens_iter)?)
    }
}

/// print_statement -> "print" "(" expression_statement ")";
fn print_statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    tokens_iter.next(); // skip the print token
    consume_if_matches(tokens_iter, Separator(Left(Parenthesis)))?;
    let expression = expression(tokens_iter)?;
    consume_if_matches(tokens_iter, Separator(Right(Parenthesis)))?;
    consume_if_matches(tokens_iter, Separator(Semicolon))?;
    Ok(PrintStmt(expression))
}

/// expression_statement -> expression  ";";
pub fn expression_statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    let expression = expression(tokens_iter)?;
    consume_if_matches(tokens_iter, Separator(Semicolon))?;
    Ok(ExpresssionStmt(expression))
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpresssionStmt(expression) => writeln!(f, "{}", expression),
            PrintStmt(expression) => writeln!(f, "print({})", expression),
        }
    }
}
