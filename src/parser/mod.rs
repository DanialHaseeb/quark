use crate::lexer::token::Token;
use anyhow::Result;

use self::expression::Expression;
use self::grammar::expression;

pub mod expression;
pub mod grammar;

pub fn parse(tokens: Vec<Token>) -> Result<Expression> {
    let mut tokens = tokens.into_iter().peekable();
    expression(&mut tokens)
}
