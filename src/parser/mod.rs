use crate::lexer::token::Token;
use anyhow::Result;

pub mod expression;

struct Node {
    children: Vec<Node>,
}

pub struct Tree {
    node: Node,
}

pub fn generate(tokens: Vec<Token>) -> Result<Tree> {
    todo!()
}
