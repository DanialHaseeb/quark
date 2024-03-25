pub mod expression;
pub mod statement;

use anyhow::Result;
use std::fmt;
use std::iter::Peekable;

use crate::lexer::token::{
    identifier::IdentifierKind::*, identifier::KeywordKind, operator::OperatorKind::*,
    operator::SingleCharKind::*, separator::SeparatorKind::*, Token, TokenKind::*,
};

use super::utils::{consume_and_return_if_variable, consume_if_matches};
use expression::{grammar::expression, structs::ExpressionKind};
use statement::{statement, StatementKind};

use Declaration::*;

pub enum Declaration {
    VariableDeclaration(VariableDeclarationBody),
    Statement(StatementKind),
}

pub struct VariableDeclarationBody {
    pub identifier: String,
    pub expression: ExpressionKind,
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
