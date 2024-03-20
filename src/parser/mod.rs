use crate::lexer::token::operator::DoubleCharKind::*;
use crate::lexer::token::operator::OperatorKind::*;
use crate::lexer::token::{Token, TokenKind::*};
use anyhow::Result;
use expression::BinaryExpr;
use util::GoBackIterator;

use self::expression::Expression;

pub mod expression;
pub mod grammar;
pub mod util;

struct Node {
    children: Vec<Node>,
}

pub struct Tree {
    node: Node,
}

pub fn generate(tokens: Vec<Token>) -> Result<Tree> {
    let traveler = GoBackIterator::new(tokens);

    Ok(Tree {
        node: Node { children: vec![] },
    })
}

fn expression(traveler: &GoBackIterator) -> Expression {
    equality(&traveler)
}

fn equality(traveler: &GoBackIterator) -> Expression {
    let mut expr = comparison();

    loop {
        match traveler.peek().unwrap().token_kind {
            Operator(DoubleChar(BangEqual)) | Operator(DoubleChar(EqualEqual)) => {
                let operator = *traveler.peek_prev().unwrap().clone();
                let right = comparison();
                let expr = BinaryExpr::new(Box::new(expr), operator, Box::new(right));
            }
            _ => break,
        }
    }
    expr
}

fn comparison() -> Expression {
    todo!()
}
