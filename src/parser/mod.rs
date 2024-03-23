pub mod declaration;

use core::fmt;

use self::declaration::{declaration, Declaration};

use super::lexer::token::Token;
use anyhow::Result;
pub struct Program(Vec<Declaration>);

pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut tokens = tokens.into_iter().peekable();
    let mut statements = Vec::new();

    while tokens.peek().is_some() {
        let statement = declaration(&mut tokens)?;
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
