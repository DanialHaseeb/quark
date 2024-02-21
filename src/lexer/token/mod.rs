pub mod identifier;
pub mod literal;
pub mod operator;
pub mod separator;

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
