use quark::{
    lexer::token::{
        literal::{LiteralKind, NumberKind},
        operator::{OperatorKind, SingleCharKind},
        Token, TokenKind,
    },
    parser::statement::expression::structs::{
        BinaryExprKind, ExpressionKind, GroupingExprKind, LiteralExprKind, UnaryExprKind,
    },
};

#[test]
fn test_expression_printing() {
    let expression = ExpressionKind::BinaryExpr(BinaryExprKind {
        left: Box::new(ExpressionKind::UnaryExpr(UnaryExprKind {
            operator: Token {
                token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Minus)),
            },
            right: Box::new(ExpressionKind::LiteralExpr(LiteralExprKind {
                value: Token {
                    token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::Int(123))),
                },
            })),
        })),
        operator: Token {
            token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Asterisk)),
        },
        right: Box::new(ExpressionKind::GroupingExpr(GroupingExprKind {
            expression: Box::new(ExpressionKind::LiteralExpr(LiteralExprKind {
                value: Token {
                    token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::Float(45.67))),
                },
            })),
        })),
    });

    assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))");
}

#[test]
fn test_numbers_expresssion() {
    let input = "1 + 2";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let mut tokens = tokens.into_iter().peekable();
    let expression =
        quark::parser::statement::expression::grammar::expression(&mut tokens).unwrap();
    assert_eq!(format!("{}", expression), "(+ 1 2)");
}

#[test]
fn test_numbers_grouping_expresssion() {
    let input = "-123 * (45.67)";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let mut tokens = tokens.into_iter().peekable();
    let expression =
        quark::parser::statement::expression::grammar::expression(&mut tokens).unwrap();
    assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))");
}

#[test]
fn test_numbers_parsing() {
    let input = "1 + 2;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "(+ 1 2)\n");
}

#[test]
fn test_paranthesis_parsing() {
    let input = "-123 * (45.67);";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))\n");
}
