use crate::lexer::token::operator::DoubleCharKind::*;
use crate::lexer::token::operator::OperatorKind::*;
use crate::lexer::token::{Token, TokenKind::*};
use anyhow::Result;
use expression::BinaryExpr;
use util::LookbackIterator;

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
    let iterator = LookbackIterator::new(tokens);

    Ok(Tree {
        node: Node { children: vec![] },
    })
}

fn expression(iterator: &LookbackIterator) -> Expression {
    equality(iterator)
}

fn equality(iterator: &LookbackIterator) -> Expression {
    let mut expr = comparison();

    while let Operator(DoubleChar(BangEqual)) | Operator(DoubleChar(EqualEqual)) =
        iterator.peek().unwrap().token_kind
    {
        {
            let operator = iterator.peek_prev().unwrap().clone();
            let right = comparison();
            expr = Expression::Binary(BinaryExpr::new(Box::new(expr), operator, Box::new(right)));
        }
    }
    expr
}

fn comparison() -> Expression {
    todo!()
}
