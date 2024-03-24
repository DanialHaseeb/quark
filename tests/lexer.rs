use itertools::assert_equal;
use quark::lexer::token::{
    identifier::IdentifierKind::*,
    literal::{LiteralKind, NumberKind},
    operator::{OperatorKind, SingleCharKind},
    separator::{Delimiter::*, SeparatorKind},
    Token, TokenKind,
};

#[test]
fn test_number_expressions() {
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

#[test]
fn test_complex_numbers() {
    let input = "
-1i
-1.0000i
x = 1 + 1.1i
";

    let tokens = quark::lexer::lex(input.to_string()).unwrap();

    assert_equal(
        tokens,
        vec![
            Token {
                token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Minus)),
            },
            Token {
                token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::ImgInt(1))),
            },
            Token {
                token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Minus)),
            },
            Token {
                token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::ImgFloat(1.0000))),
            },
            Token {
                token_kind: TokenKind::Identifier(Variable("x".into())),
            },
            Token {
                token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Equal)),
            },
            Token {
                token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::Int(1))),
            },
            Token {
                token_kind: TokenKind::Operator(OperatorKind::SingleChar(SingleCharKind::Plus)),
            },
            Token {
                token_kind: TokenKind::Literal(LiteralKind::Number(NumberKind::ImgFloat(1.1))),
            },
        ],
    );
}
