use quark::{
    lexer::token::{
        literal::{LiteralKind::*, NumberKind::*},
        operator::{OperatorKind::*, SingleCharKind::*},
        Token,
        TokenKind::*,
    },
    parser::declaration::expression::grammar::expression,
    parser::declaration::expression::structs::{
        BinaryExprBody, ExpressionKind::*, GroupingExprBody, LiteralExprBody, UnaryExprBody,
    },
};

#[test]
fn test_expression_printing() {
    let expression = BinaryExpr(BinaryExprBody {
        left: Box::new(UnaryExpr(UnaryExprBody {
            operator: Token {
                token_kind: Operator(SingleChar(Minus)),
            },
            expression: Box::new(LiteralExpr(LiteralExprBody {
                value: Token {
                    token_kind: Literal(Number(Int(123))),
                },
            })),
        })),
        operator: Token {
            token_kind: Operator(SingleChar(Asterisk)),
        },
        right: Box::new(GroupingExpr(GroupingExprBody {
            expression: Box::new(LiteralExpr(LiteralExprBody {
                value: Token {
                    token_kind: Literal(Number(Float(45.67))),
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
    let expr = expression(&mut tokens).unwrap();
    assert_eq!(format!("{}", expr), "(+ 1 2)");
}

#[test]
fn test_numbers_grouping_expresssion() {
    let input = "-123 * (45.67)";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let mut tokens = tokens.into_iter().peekable();
    let expr = expression(&mut tokens).unwrap();
    assert_eq!(format!("{}", expr), "(* (- 123) (group 45.67))");
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

#[test]
fn test_paranthesis_parsing_with_multiple_operators() {
    let input = "-123 * (45.67) / 2;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "(* (- 123) (/ (group 45.67) 2))\n"
    );
}

#[test]
fn test_variable_parsing() {
    let input = "let x = 1;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "let x = 1;\n");
}

#[test]
fn test_variable_paranthesis_parsing_with_multiple_operators() {
    let input = "let x = -123 * (45.67) / 2;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "let x = (* (- 123) (/ (group 45.67) 2));\n"
    );
}

#[test]
fn test_logical_operators() {
    let input = "1 == 2 and 3 == 3 or 1 < 3;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "(Or (And (== 1 2) (== 3 3)) (< 1 3))\n"
    );
}
