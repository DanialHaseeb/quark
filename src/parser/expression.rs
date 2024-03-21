use crate::lexer::token::Token;
use std::fmt;

pub enum Expression {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
}

pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expression>,
}

pub struct LiteralExpr {
    pub value: Token,
}

pub struct GroupingExpr {
    pub expression: Box<Expression>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary(expr) => write!(f, "{}", expr),
            Expression::Unary(expr) => write!(f, "{}", expr),
            Expression::Literal(expr) => write!(f, "{}", expr),
            Expression::Grouping(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operator, self.right)
    }
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}
