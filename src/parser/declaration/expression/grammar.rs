use super::structs::{
    BinaryExprBody, ExpressionKind, ExpressionKind::*, GroupingExprBody, ListExprBody,
    LiteralExprBody, MatrixExprBody, UnaryExprBody, VariableExprBody,
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
/// expression -> logic_or ;
pub fn expression<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    logic_or(tokens_iter)
}

/// Grammar Rule:
/// logic_or -> logic_and ( "or" logic_and )*;
fn logic_or<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let mut expression = logic_and(tokens_iter)?;

    while let Some(Identifier(Keyword(Or))) = tokens_iter.peek().map(|token| &token.token_kind) {
        let operator = tokens_iter.next().unwrap();
        let right = logic_and(tokens_iter)?;

        expression = BinaryExpr(BinaryExprBody {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }

    Ok(expression)
}

/// Grammar Rule:
/// logic_and -> equality ( "and" equality )*;
fn logic_and<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let mut expression = equality(tokens_iter)?;

    while let Some(Identifier(Keyword(And))) = tokens_iter.peek().map(|token| &token.token_kind) {
        let operator = tokens_iter.next().unwrap();
        let right = equality(tokens_iter)?;

        expression = BinaryExpr(BinaryExprBody {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }

    Ok(expression)
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
/// primary  ->  NUMBER | STRING | LITERAL | "true" | "false" | LIST | MATRIX | "(" expression ")"
/// LIST -> "[" list_expression "]"
/// MATRIX -> "[" list_expression ("||" list_expression)* "]"
fn primary<T>(tokens_iter: &mut Peekable<T>) -> Result<ExpressionKind>
where
    T: Iterator<Item = Token>,
{
    let token_kind = tokens_iter.next().unwrap().token_kind;

    match token_kind {
        Literal(Number(_))
        | Literal(String(_))
        | Identifier(Keyword(True))
        | Identifier(Keyword(False)) => Ok(LiteralExpr(LiteralExprBody {
            value: Token { token_kind },
        })),
        Separator(Left(Parenthesis)) => {
            let expression = expression(tokens_iter)?;
            consume_if_matches(tokens_iter, Separator(Right(Parenthesis)))?;

            Ok(GroupingExpr(GroupingExprBody {
                expression: Box::new(expression),
            }))
        }
        Separator(Left(Bracket)) => {
            let mut list_expressions = vec![list_expression(tokens_iter)?];

            if let Some(Operator(SingleChar(Pipe))) =
                tokens_iter.peek().map(|token| &token.token_kind)
            {
                while let Some(Operator(SingleChar(Pipe))) =
                    tokens_iter.peek().map(|token| &token.token_kind)
                {
                    tokens_iter.next();
                    consume_if_matches(tokens_iter, Operator(SingleChar(Pipe)))?;
                    list_expressions.push(list_expression(tokens_iter)?);
                }

                consume_if_matches(tokens_iter, Separator(Right(Bracket)))?;
                Ok(MatrixExpr(MatrixExprBody { list_expressions }))
            } else {
                consume_if_matches(tokens_iter, Separator(Right(Bracket)))?;

                let only_expresssion = match list_expressions.pop() {
                    Some(element) => element,
                    None => unreachable!(),
                };

                Ok(ListExpr(only_expresssion))
            }
        }
        Identifier(Variable(name)) => Ok(VariableExpr(VariableExprBody { name })),
        _ => bail!("Unexpected token: {token_kind}"),
    }
}

/// Grammar Rule:
/// list_expression ->  expression ("," expression)*
fn list_expression<T>(tokens_iter: &mut Peekable<T>) -> Result<ListExprBody>
where
    T: Iterator<Item = Token>,
{
    let mut expressions = Vec::new();

    expressions.push(expression(tokens_iter)?);

    while let Some(Separator(Comma)) = tokens_iter.peek().map(|token| &token.token_kind) {
        tokens_iter.next();
        expressions.push(expression(tokens_iter)?);
    }

    Ok(ListExprBody { expressions })
}
