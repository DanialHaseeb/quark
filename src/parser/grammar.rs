use crate::lexer::token::identifier::IdentifierKind::*;
use crate::lexer::token::identifier::KeywordKind::*;
use crate::lexer::token::literal::LiteralKind::*;
use crate::lexer::token::operator::DoubleCharKind::*;
use crate::lexer::token::operator::OperatorKind::*;
use crate::lexer::token::operator::SingleCharKind::*;
use crate::lexer::token::separator::Delimiter::*;
use crate::lexer::token::separator::SeparatorKind::*;
use crate::lexer::token::{Token, TokenKind::*};
use anyhow::anyhow;
use anyhow::Result;

use super::expression::BinaryExpr;
use super::expression::Expression;
use super::expression::GroupingExpr;
use super::expression::LiteralExpr;
use super::expression::UnaryExpr;
use super::util::Peekback;

/// Grammar Rule:
/// expression     → equality ;
pub fn expression(tokens_iter: &mut Peekback) -> Result<Expression> {
    equality(tokens_iter)
}

/// Grammar Rule:
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
fn equality(tokens_iter: &mut Peekback) -> Result<Expression> {
    let mut expression = comparison(tokens_iter)?;

    while let Operator(DoubleChar(BangEqual)) | Operator(DoubleChar(EqualEqual)) =
        tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = comparison(tokens_iter)?;

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }

    Ok(expression)
}

/// Grammar Rule:
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
fn comparison(tokens_iter: &mut Peekback) -> Result<Expression> {
    let mut expression = term(tokens_iter)?;

    while let Operator(SingleChar(Greater))
    | Operator(SingleChar(Less))
    | Operator(DoubleChar(LessEqual))
    | Operator(DoubleChar(GreaterEqual)) = tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = term(tokens_iter)?;

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }

    Ok(expression)
}

/// Grammar Rule
/// term           → factor ( ( "-" | "+" ) factor )* ;
fn term(tokens_iter: &mut Peekback) -> Result<Expression> {
    let mut expression = factor(tokens_iter)?;

    while let Operator(SingleChar(Minus)) | Operator(SingleChar(Plus)) =
        tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = factor(tokens_iter)?;

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        })
    }

    Ok(expression)
}

/// Grammar Rule:
/// factor         → unary ( ( "/" | "*" ) unary )* ;
fn factor(tokens_iter: &mut Peekback) -> Result<Expression> {
    let mut expression = unary(tokens_iter)?;

    while let Operator(SingleChar(Slash)) | Operator(SingleChar(Asterisk)) =
        tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = factor(tokens_iter)?;

        expression = Expression::Binary(BinaryExpr {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        })
    }

    Ok(expression)
}

/// Grammar Rule:
/// unary          → ( "!" | "-" ) unary;
fn unary(tokens_iter: &mut Peekback) -> Result<Expression> {
    if let Operator(SingleChar(Bang)) | Operator(SingleChar(Minus)) =
        tokens_iter.peek().unwrap().token_kind
    {
        let operator = tokens_iter.next().unwrap().clone();
        let right = unary(tokens_iter)?;

        return Ok(Expression::Unary(UnaryExpr {
            operator,
            right: Box::new(right),
        }));
    }
    primary(tokens_iter)
}

/// Grammar Rule:
/// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
fn primary(tokens_iter: &mut Peekback) -> Result<Expression> {
    let token_kind = tokens_iter.peek().unwrap().token_kind.clone();

    if let Literal(Number(_))
    | Literal(String(_))
    | Identifier(Keyword(True))
    | Identifier(Keyword(False)) = token_kind
    {
        Ok(Expression::Literal(LiteralExpr {
            value: Token { token_kind },
        }))
    } else if let Separator(Left(Parenthesis)) = token_kind {
        let expression = expression(tokens_iter)?;

        if tokens_iter
            .consume(Token::new(Separator(Right(Parenthesis))))
            .is_none()
        {
            return Err(anyhow!("Expected ')' after expression"));
        }

        Ok(Expression::Grouping(GroupingExpr {
            expression: Box::new(expression),
        }))
    } else {
        Err(anyhow!("Unexpected token: {:?}", token_kind))
    }
}

// TODO: Don't know if this is the best way to handle this
// TODO:function, let, if, for, while, print, return
fn synchronize(tokens_iter: &mut Peekback) {
    while let Some(token) = tokens_iter.next() {
        match token.token_kind {
            Identifier(Keyword(Function)) => return,
            Identifier(Keyword(While)) => return,
            Identifier(Keyword(For)) => return,
            Identifier(Keyword(If)) => return,
            Identifier(Keyword(Let)) => return,
            Identifier(Keyword(Break)) => return,
            Identifier(Keyword(Continue)) => return,
            Identifier(Keyword(Return)) => return,
            Identifier(Keyword(Print)) => return,
            // TODO: Add more keywords here like ELSE, ELSEIF
            _ => (),
        }
    }
}
