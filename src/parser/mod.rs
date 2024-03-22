use crate::lexer::token::Token;
use anyhow::Result;
use util::Peekback;

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
    let iterator = Peekback::new(tokens);

    Ok(Tree {
        node: Node { children: vec![] },
    })
}
