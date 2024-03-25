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
    IfStmt(IfStatementBody),
    WhileStmt(WhileStatementBody),
    PrintStmt(ExpressionKind),
    // TODO: implement other statement types like `print`, `while`, `if`, `return`, `function` etc.
}

pub struct IfStatementBody {
    pub condition: ExpressionKind,
    pub block: Block,
}

pub struct WhileStatementBody {
    pub condition: ExpressionKind,
    pub block: Block,
}

pub struct Block(pub Vec<StatementKind>);

/// Grammar Rule:
/// statement -> expression_statement | print_statement | if_statement
pub fn statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    match tokens_iter.peek().map(|token| &token.token_kind) {
        Some(Identifier(Keyword(Print))) => Ok(print_statement(tokens_iter)?),
        Some(Identifier(Keyword(If))) => Ok(if_statement(tokens_iter)?),
        Some(Identifier(Keyword(While))) => Ok(while_statement(tokens_iter)?),
        _ => expression_statement(tokens_iter),
    }
}
/// if_statement -> "if" expression block
fn if_statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    tokens_iter.next(); // skip the if token
    let condition = expression(tokens_iter)?;
    let block = block(tokens_iter)?;
    Ok(IfStmt(IfStatementBody { condition, block }))
}

/// while_statement -> "while" expression block
pub fn while_statement<T>(tokens_iter: &mut Peekable<T>) -> Result<StatementKind>
where
    T: Iterator<Item = Token>,
{
    tokens_iter.next(); // skip the while token
    let condition = expression(tokens_iter)?;
    let block = block(tokens_iter)?;
    Ok(WhileStmt(WhileStatementBody { condition, block }))
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

pub fn block<T>(tokens_iter: &mut Peekable<T>) -> Result<Block>
where
    T: Iterator<Item = Token>,
{
    consume_if_matches(tokens_iter, Separator(Left(Brace)))?;
    let mut statements = Vec::new();
    while let Some(token) = tokens_iter.peek() {
        if token.token_kind == Separator(Right(Brace)) {
            break;
        }
        statements.push(statement(tokens_iter)?);
    }
    consume_if_matches(tokens_iter, Separator(Right(Brace)))?;
    Ok(Block(statements))
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpresssionStmt(expression) => writeln!(f, "{}", expression),
            PrintStmt(expression) => writeln!(f, "print({})", expression),
            IfStmt(if_stmt) => writeln!(f, "if {} {}", if_stmt.condition, if_stmt.block),
            WhileStmt(while_stmt) => {
                writeln!(f, "while {} {}", while_stmt.condition, while_stmt.block)
            }
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for statement in &self.0 {
            write!(f, "{}", statement)?;
        }
        write!(f, "}}")
    }
}
