use super::structs::{
    BinaryExprBody, ExpressionKind, ExpressionKind::*, GroupingExprBody, LiteralExprBody,
    UnaryExprBody, VariableExprBody,
};

use crate::lexer::token::{
    identifier::{IdentifierKind::*, KeywordKind::*},
    literal::LiteralKind::*,
    operator::{DoubleCharKind::*, OperatorKind::*, SingleCharKind::*},
    separator::{Delimiter::*, SeparatorKind::*},
    Token,
    TokenKind::*,
};

use super::super::utils::consume_if_matches;

use anyhow::{bail, Result};
use std::iter::Peekable;

/// Grammar Rule:
/// expression     → equality ;
pub fn expression<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    equality(tokens_iter)
}

/// Grammar Rule:
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
fn equality<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let mut expression = comparison(tokens_iter)?;

    while let Some(Operator(DoubleChar(BangEqual))) | Some(Operator(DoubleChar(EqualEqual))) =
        tokens_iter.peek().map(|token| &token.token_kind)
    {
        let operator = tokens_iter.next().unwrap();
        let right = comparison(tokens_iter)?;

        expression = BinaryExpr(BinaryExprBody {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }

    Ok(expression)
}

/// Grammar Rule:
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
fn comparison<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let mut expression = term(tokens_iter)?;

    while let Some(Operator(SingleChar(Greater)))
    | Some(Operator(SingleChar(Less)))
    | Some(Operator(DoubleChar(LessEqual)))
    | Some(Operator(DoubleChar(GreaterEqual))) =
        tokens_iter.peek().map(|token| &token.token_kind)
    {
        let operator = tokens_iter.next().unwrap();
        let right = term(tokens_iter)?;

        expression = BinaryExpr(BinaryExprBody {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }

    Ok(expression)
}

/// Grammar Rule
/// term           → factor ( ( "-" | "+" ) factor )* ;
fn term<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let mut expression = factor(tokens_iter)?;

    while let Some(Operator(SingleChar(Minus))) | Some(Operator(SingleChar(Plus))) =
        tokens_iter.peek().map(|token| &token.token_kind)
    {
        let operator = tokens_iter.next().unwrap();
        let right = factor(tokens_iter)?;

        expression = BinaryExpr(BinaryExprBody {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        })
    }

    Ok(expression)
}

/// Grammar Rule:
/// factor         → unary ( ( "/" | "*" ) unary )* ;
fn factor<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let mut expression = unary(tokens_iter)?;

    while let Some(Operator(SingleChar(Slash))) | Some(Operator(SingleChar(Asterisk))) =
        tokens_iter.peek().map(|token| &token.token_kind)
    {
        let operator = tokens_iter.next().unwrap();
        let right = factor(tokens_iter)?;

        expression = BinaryExpr(BinaryExprBody {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        })
    }

    Ok(expression)
}

/// Grammar Rule:
/// unary          → ( "!" | "-" ) unary;
fn unary<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    if let Some(Operator(SingleChar(Bang))) | Some(Operator(SingleChar(Minus))) =
        tokens_iter.peek().map(|token| &token.token_kind)
    {
        let operator = tokens_iter.next().unwrap();
        let right = unary(tokens_iter)?;

        return Ok(UnaryExpr(UnaryExprBody {
            operator,
            expression: Box::new(right),
        }));
    }

    primary(tokens_iter)
}

/// Grammar Rule:
/// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
fn primary<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let token_kind = tokens_iter.next().unwrap().token_kind;

    if let Literal(Number(_))
    | Literal(String(_))
    | Identifier(Keyword(True))
    | Identifier(Keyword(False)) = token_kind
    {
        Ok(LiteralExpr(LiteralExprBody {
            value: Token { token_kind },
        }))
    } else if let Separator(Left(Parenthesis)) = token_kind {
        let expression = expression(tokens_iter)?;
        consume_if_matches(tokens_iter, Separator(Right(Parenthesis)))?;

        Ok(GroupingExpr(GroupingExprBody {
            expression: Box::new(expression),
        }))
    } else if let Identifier(Variable(name)) = token_kind {
        Ok(VariableExpr(VariableExprBody { name }))
    } else {
        bail!("Unexpected token: {token_kind}")
    }
}

// TODO: Don't know if this is the best way to handle this
// TODO:function, let, if, for, while, print, return
fn synchronize<T>(tokens_iter: &mut Peekable<T>)
where
    T: Iterator<Item = Token>,
{
    for token in tokens_iter {
        match token.token_kind {
            Identifier(Keyword(Function)) => return,
            Identifier(Keyword(While)) => return,
            Identifier(Keyword(For)) => return,
            Identifier(Keyword(If)) => return,
            Identifier(Keyword(Let)) => return,
            Identifier(Keyword(Break)) => return,
            Identifier(Keyword(Continue)) => return,
            Identifier(Keyword(Return)) => return,
            // TODO: Add more keywords here like ELSE, ELSEIF
            _ => (),
        }
    }
}
