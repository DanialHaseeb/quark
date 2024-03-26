pub mod declaration;
pub mod utils;

use core::fmt;

use self::declaration::{declaration, Declaration};
use crate::generator::CodeGenerator;

use super::lexer::token::Token;
use anyhow::Result;
pub struct Program(pub Vec<Declaration>);

pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut tokens = tokens.into_iter().peekable();
    let mut declarations = Vec::new();

    while tokens.peek().is_some() {
        let declaration = declaration(&mut tokens)?;
        declarations.push(declaration);
    }

    Ok(Program(declarations))
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for declaration in &self.0 {
            write!(f, "{}", declaration)?;
        }
        Ok(())
    }
}

impl CodeGenerator for Program {
    fn generate(&self) -> String {
        let mut output = String::new();
        for declaration in &self.0 {
            output.push_str(&declaration.generate());
        }
        output
    }
}
