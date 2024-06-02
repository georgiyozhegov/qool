use super::*;
use crate::lex::*;
use std::iter::Peekable;
use std::vec::IntoIter;
use std::process::exit;

macro_rules! error {
    ($message: expr) => {
        {
        eprintln!("PARSE ERROR: {}", $message);
        exit(1);
        }
    };
}

fn binary_expression(left: Expression, source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        let operator = source.next();
        match operator {
                Some(Token::BinaryOperator(..)) => {},
                _ => error!("PARSE ERROR: expected binary operator"),
        }
        let right = expression(source);
        Expression::BinaryExpression(Box::new(left), operator.unwrap(), Box::new(right))
}

fn expression(source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        let token = source.next();
        if source.peek().is_some() {
                let left = match token {
                        Some(Token::Identifier(identifier)) => Expression::Identifier(identifier),
                        Some(Token::Integer(integer)) => Expression::Integer(integer),
                        _ => error!("expected identifier or literal followed by expression"),
                };
                binary_expression(left, source)
        }
        else {
                match token {
                        Some(Token::Identifier(identifier)) => Expression::Identifier(identifier),
                        Some(Token::Integer(integer)) => Expression::Integer(integer),
                        _ => error!("expected identifier or literal"),
                }
        }
}

fn assign(source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        match source.next() {
                Some(Token::Assign) => {},
                _ => error!("expected assign operator"),
        }
        expression(source)
}

pub fn statement(source: &mut Peekable<IntoIter<Token>>) -> Option<Statement>
{
        let token = source.next()?;
        match token {
                Token::Identifier(identifier) => {
                        Some(Statement::Variable(identifier, assign(source)))
                }
                _ => error!("invalid token"),
        }
}

pub fn parse(mut source: Peekable<IntoIter<Token>>) -> Vec<Statement>
{
        let mut statements = Vec::new();
        while let Some(statement) = statement(&mut source) {
                statements.push(statement);
        }
        statements
}
