use quark::{
    lexer::token::{
        literal::{Literal, Number},
        operator::{Operator, SingleCharOperator},
        Kind, Token,
    },
    parser::expression::{BinaryExpr, Expression, GroupingExpr, LiteralExpr, UnaryExpr},
};

#[test]
fn test_expression_printing() {
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

    assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))");
}
