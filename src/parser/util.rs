use crate::lexer::token::Token;

pub struct GoBackIterator {
    current: usize,
    tokens: Vec<Token>,
}

impl GoBackIterator {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Advances the iterator and returns the next value.
    pub fn next(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
            Some(self.peek_prev().unwrap())
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn peek_prev(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
