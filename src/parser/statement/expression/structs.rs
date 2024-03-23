use crate::lexer::token::Token;
use std::fmt;
use ExpressionKind::*;

pub enum ExpressionKind {
    BinaryExpr(BinaryExprKind),
    UnaryExpr(UnaryExprKind),
    LiteralExpr(LiteralExprKind),
    GroupingExpr(GroupingExprKind),
}

pub struct BinaryExprKind {
    pub left: Box<ExpressionKind>,
    pub operator: Token,
    pub right: Box<ExpressionKind>,
}

pub struct UnaryExprKind {
    pub operator: Token,
    pub right: Box<ExpressionKind>,
}

pub struct LiteralExprKind {
    pub value: Token,
}

pub struct GroupingExprKind {
    pub expression: Box<ExpressionKind>,
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryExpr(expr) => write!(f, "{}", expr),
            UnaryExpr(expr) => write!(f, "{}", expr),
            LiteralExpr(expr) => write!(f, "{}", expr),
            GroupingExpr(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for BinaryExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

impl fmt::Display for UnaryExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operator, self.right)
    }
}

impl fmt::Display for LiteralExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for GroupingExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}
