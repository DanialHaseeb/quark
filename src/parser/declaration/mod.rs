pub mod expression;
pub mod utils;

use anyhow::Result;
use std::fmt;
use std::iter::Peekable;

use crate::lexer::token::identifier::IdentifierKind::*;
use crate::lexer::token::identifier::KeywordKind;
use crate::lexer::token::operator::OperatorKind::*;
use crate::lexer::token::operator::SingleCharKind::*;
use crate::lexer::token::separator::SeparatorKind::*;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind::*;

use expression::grammar::expression;
use expression::structs::ExpressionKind;

use self::utils::consume_and_return_if_variable;
use self::utils::consume_if_matches;

use Declaration::*;
use StatementKind::*;

pub enum Declaration {
    VariableDeclaration(VariableDeclarationBody),
    Statement(StatementKind),
}

pub struct VariableDeclarationBody {
    pub identifier: String,
    pub expression: ExpressionKind,
}

// Statements are things that you can possible not store in variables like print operation.
pub enum StatementKind {
    Expresssion(ExpressionKind),
    // TODO: implement other statement types like `print`, `while`, `if`, `return`, `function` etc.
}

// Grammar Rule:
// Declaration -> "let" VARIABLE "=" expression ";" | Statement;
pub fn declaration<T>(tokens_iter: &mut Peekable<T>) -> Result<Declaration>
where
    T: Iterator<Item = Token>,
{
    if let Some(Identifier(Keyword(KeywordKind::Let))) =
        tokens_iter.peek().map(|token| &token.token_kind)
    {
        tokens_iter.next().unwrap();
        let identifier = consume_and_return_if_variable(tokens_iter)?;

        consume_if_matches(tokens_iter, Operator(SingleChar(Equal)))?;

        let expr = expression(tokens_iter)?;

        consume_if_matches(tokens_iter, Separator(Semicolon))?;

        Ok(VariableDeclaration(VariableDeclarationBody {
            identifier,
            expression: expr,
        }))
    } else {
        let statement_thing = statement(tokens_iter)?;
        Ok(Statement(statement_thing))
    }
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

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableDeclaration(VariableDeclarationBody {
                identifier,
                expression,
            }) => writeln!(f, "let {} = {};", identifier, expression),
            Statement(statement) => write!(f, "{}", statement),
        }
    }
}
