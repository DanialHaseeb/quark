pub mod identifier;
pub mod literal;
pub mod operator;
pub mod separator;

use std::fmt;

use identifier::IdentifierKind;
use literal::LiteralKind;
use operator::OperatorKind;
use separator::SeparatorKind;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace,
    Identifier(IdentifierKind),
    Separator(SeparatorKind),
    Operator(OperatorKind),
    Literal(LiteralKind),
}

impl Token {
    pub fn new(token_type: TokenKind) -> Self {
        Self {
            token_kind: token_type,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_kind {
            TokenKind::Whitespace => unreachable!(),
            TokenKind::Identifier(identifier) => write!(f, "{}", identifier),
            TokenKind::Separator(separator) => write!(f, "{}", separator),
            TokenKind::Operator(operator) => write!(f, "{}", operator),
            TokenKind::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Whitespace => unreachable!(),
            TokenKind::Identifier(identifier) => write!(f, "{}", identifier),
            TokenKind::Separator(separator) => write!(f, "{}", separator),
            TokenKind::Operator(operator) => write!(f, "{}", operator),
            TokenKind::Literal(literal) => write!(f, "{}", literal),
        }
    }
}
