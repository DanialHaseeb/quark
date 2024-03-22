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
fn test_input() {
    let input = "123 * (45.67)";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();

    assert_eq!(tokens.len(), 5);
    assert_eq!(
        tokens,
        vec![
            Token {
                token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::Int(123))),
            },
            Token {
                token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Asterisk)),
            },
            Token {
                token_kind: TokenKind::Separator(SeparatorKind::Left(Parenthesis)),
            },
            Token {
                token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::Float(45.67))),
            },
            Token {
                token_kind: TokenKind::Separator(
                    quark::lexer::token::separator::SeparatorKind::Right(Parenthesis)
                ),
            },
        ]
    );
}
