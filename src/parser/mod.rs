pub mod statement;

use core::fmt;

use super::lexer::token::Token;
use anyhow::Result;
use statement::{statement, StatementKind};

pub struct Program(Vec<StatementKind>);

pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut tokens = tokens.into_iter().peekable();
    let mut statements = Vec::new();

    while tokens.peek().is_some() {
        let statement = statement(&mut tokens)?;
        statements.push(statement);
    }

    Ok(Program(statements))
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.0 {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}
