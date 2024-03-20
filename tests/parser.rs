use quark::{
    lexer::token::{
        literal::NumberKind,
        operator::{OperatorKind, SingleCharKind},
        Token, TokenKind,
    },
    parser::expression::{BinaryExpr, Expression, GroupingExpr, LiteralExpr, UnaryExpr},
};

#[test]
fn test_expression_printing() {
    let expression = Expression::Binary(BinaryExpr {
        left: Box::new(Expression::Unary(UnaryExpr {
            operator: Token {
                token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Minus)),
            },
            right: Box::new(Expression::Literal(LiteralExpr {
                value: Token {
                    token_kind: TokenKind::Literal(LiteralType::Number(NumberKind::Int(123))),
                },
            })),
        })),
        operator: Token {
            token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Asterisk)),
        },
        right: Box::new(Expression::Grouping(GroupingExpr {
            expression: Box::new(Expression::Literal(LiteralExpr {
                value: Token {
                    token_kind: TokenKind::Literal(LiteralType::Number(NumberKind::Float(45.67))),
                },
            })),
        })),
    });

    assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))");
}
