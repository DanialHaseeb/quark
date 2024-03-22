use crate::lexer::token::Token;
use anyhow::Result;
use util::Peekback;

use self::expression::Expression;
use self::grammar::expression;

pub mod expression;
pub mod grammar;
pub mod util;

pub fn parse(tokens: Vec<Token>) -> Result<Expression> {
    let mut tokens = Peekback::new(tokens);
    expression(&mut tokens)
}
