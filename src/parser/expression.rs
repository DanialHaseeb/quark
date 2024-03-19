use crate::lexer::token::{
    literal::{Literal, Number},
    operator::{Operator, SingleCharOperator},
    Kind, Token,
};

enum Expression {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
}

trait Print {
    fn print(&self) -> String;
}

struct BinaryExpr {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
}

struct UnaryExpr {
    operator: Token,
    right: Box<Expression>,
}

struct LiteralExpr {
    value: Token,
}

struct GroupingExpr {
    expression: Box<Expression>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_evaluation() {
        let expression = Expression::Binary(BinaryExpr {
            left: Box::new(Expression::Unary(UnaryExpr {
                operator: Token {
                    kind: Kind::Operator(Operator::SingleChar(SingleCharOperator::Minus)),
                },
                right: Box::new(Expression::Literal(LiteralExpr {
                    value: Token {
                        kind: Kind::Literal(Literal::Number(Number::Int(123))),
                    },
                })),
            })),
            operator: Token {
                kind: Kind::Operator(Operator::SingleChar(SingleCharOperator::Asterisk)),
            },
            right: Box::new(Expression::Grouping(GroupingExpr {
                expression: Box::new(Expression::Literal(LiteralExpr {
                    value: Token {
                        kind: Kind::Literal(Literal::Number(Number::Float(45.67))),
                    },
                })),
            })),
        });

        assert_eq!(expression.print(), "(* (- 123) (group 45.67))");
    }
}
