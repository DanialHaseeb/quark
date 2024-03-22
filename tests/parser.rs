use quark::{
    lexer::token::{
        literal::{LiteralKind, NumberKind},
        operator::{OperatorKind, SingleCharKind},
        separator::{Delimiter::*, SeparatorKind},
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
                    token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::Int(123))),
                },
            })),
        })),
        operator: Token {
            token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Asterisk)),
        },
        right: Box::new(Expression::Grouping(GroupingExpr {
            expression: Box::new(Expression::Literal(LiteralExpr {
                value: Token {
                    token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::Float(45.67))),
                },
            })),
        })),
    });

    assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))");
}

#[test]
fn test_input() {
    let input = "123 * (45.67)";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    println!("{:?}", tokens);
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "(* (123) (group 45.67))");
}
