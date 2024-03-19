use crate::lexer::token::Token;

pub enum Expression {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
}

pub trait Print {
    fn print(&self) -> String;
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

impl Print for Expression {
    fn print(&self) -> String {
        match self {
            Expression::Binary(expr) => expr.print(),
            Expression::Unary(expr) => expr.print(),
            Expression::Literal(expr) => expr.print(),
            Expression::Grouping(expr) => expr.print(),
        }
    }
}

impl Print for BinaryExpr {
    fn print(&self) -> String {
        format!(
            "({} {} {})",
            self.operator,
            self.left.print(),
            self.right.print()
        )
    }
}

impl Print for UnaryExpr {
    fn print(&self) -> String {
        format!("({} {})", self.operator, self.right.print())
    }
}

impl Print for LiteralExpr {
    fn print(&self) -> String {
        format!("{}", self.value)
    }
}

impl Print for GroupingExpr {
    fn print(&self) -> String {
        format!("(group {})", self.expression.print())
    }
}
