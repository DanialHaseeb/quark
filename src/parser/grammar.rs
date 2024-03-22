use crate::lexer::token::identifier::IdentifierKind::*;
use crate::lexer::token::identifier::KeywordKind::*;
use crate::lexer::token::literal::LiteralKind::*;
use crate::lexer::token::operator::DoubleCharKind::*;
use crate::lexer::token::operator::OperatorKind::*;
use crate::lexer::token::operator::SingleCharKind::*;
use crate::lexer::token::separator::Delimiter::*;
use crate::lexer::token::separator::SeparatorKind::*;
use crate::lexer::token::{Token, TokenKind::*};

use super::expression::BinaryExpr;
use super::expression::Expression;
use super::expression::GroupingExpr;
use super::expression::LiteralExpr;
use super::expression::UnaryExpr;
use super::util::Peekback;

/// Grammar Rule:
///
/// expression     → equality ;
fn expression(tokens_iter: &mut Peekback) -> Expression {
    equality(tokens_iter)
}

/// Grammar Rule:
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
fn equality(tokens_iter: &mut Peekback) -> Expression {
    let mut expression = comparison(tokens_iter);

    while let Operator(DoubleChar(BangEqual)) | Operator(DoubleChar(EqualEqual)) =
        tokens_iter.peek().unwrap().token_kind
    {
        {}
        let operator = tokens_iter.next().unwrap().clone();
        let right = comparison(tokens_iter);

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }
    expression
}

/// Grammar Rule:
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
fn comparison(tokens_iter: &mut Peekback) -> Expression {
    let mut expression = term(tokens_iter);

    while let Operator(SingleChar(Greater))
    | Operator(SingleChar(Less))
    | Operator(DoubleChar(LessEqual))
    | Operator(DoubleChar(GreaterEqual)) = tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = term(tokens_iter);

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        })
    }

    expression
}

/// Grammar Rule
/// term           → factor ( ( "-" | "+" ) factor )* ;
fn term(tokens_iter: &mut Peekback) -> Expression {
    let mut expression = factor(tokens_iter);

    while let Operator(SingleChar(Minus)) | Operator(SingleChar(Plus)) =
        tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = factor(tokens_iter);

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        })
    }

    expression
}

/// Grammar Rule:
/// factor         → unary ( ( "/" | "*" ) unary )* ;
fn factor(tokens_iter: &mut Peekback) -> Expression {
    let mut expression = unary(tokens_iter);

    while let Operator(SingleChar(Slash)) | Operator(SingleChar(Asterisk)) =
        tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = factor(tokens_iter);

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        })
    }

    expression
}

/// Grammar Rule:
/// unary          → ( "!" | "-" ) unary;
fn unary(tokens_iter: &mut Peekback) -> Expression {
    if let Operator(SingleChar(Bang)) | Operator(SingleChar(Minus)) =
        tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = unary(tokens_iter);

        return Expression::Unary(UnaryExpr {
            operator,
            right: Box::new(right),
        });
    }
    primary(tokens_iter)
}

/// Grammar Rule:
/// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
fn primary(tokens_iter: &mut Peekback) -> Expression {
    let token_kind = tokens_iter.peek().unwrap().token_kind.clone();

    if let Literal(Number(_))
    | Literal(String(_))
    | Identifier(Keyword(True))
    | Identifier(Keyword(False)) = token_kind
    {
        Expression::Literal(LiteralExpr {
            value: Token { token_kind },
        })
    } else if let Separator(Left(Parenthesis)) = token_kind {
        let expression = Box::new(expression(tokens_iter));

        tokens_iter
            .consume(Token::new(Separator(Right(Parenthesis))))
            .expect("Expected ')' after expression on {line_number}"); // TODO: Add line number to Token struct so we can display it here as the error message

        Expression::Grouping(GroupingExpr { expression })
    } else {
        panic!("Unexpected token: {:?}", token_kind)
    }
}
