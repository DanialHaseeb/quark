pub mod identifier;
pub mod literal;
pub mod operator;
pub mod separator;

use std::fmt::{self, Display, Formatter};

use identifier::Identifier;
use literal::Literal;
use operator::Operator;
use separator::Separator;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Whitespace,
    Identifier(Identifier),
    Separator(Separator),
    Operator(Operator),
    Literal(Literal),
}

impl Token {
    pub fn new(kind: Kind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Kind::Whitespace => write!(f, "Whitespace"),
            Kind::Identifier(identifier) => write!(f, "{}", identifier),
            Kind::Separator(separator) => write!(f, "{}", separator),
            Kind::Operator(operator) => write!(f, "{}", operator),
            Kind::Literal(literal) => write!(f, "{}", literal),
        }
    }
}
