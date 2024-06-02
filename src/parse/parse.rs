use super::*;
use crate::lex::*;
use std::iter::Peekable;
use std::vec::IntoIter;
use std::process::exit;

macro_rules! error {
    ($message: expr) => {
        {
        eprintln!("PARSING ERROR: {}", $message);
        exit(1);
        }
    };
}

fn binary_expression(left: Expression, source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        let operator = match source.next() {
                Some(Token::BinaryOperator(operator)) => operator,
                _ => error!("expected binary operator"),
        };
        let right = expression(source);
        Expression::Binary(Box::new(left), operator, Box::new(right))
}

fn unary_expression(operator: UnaryOperator, source: &mut Peekable<IntoIter<Token>>) -> Expression 
{
    let expression = expression(source);
    Expression::Unary(operator, Box::new(expression))
}

fn expression(source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        let token = source.next();
        if source.peek().is_some() {
                match token {
                        Some(Token::Identifier(identifier)) => binary_expression(Expression::Identifier(identifier), source),
                        Some(Token::Integer(integer)) => binary_expression(Expression::Integer(integer), source),
                        Some(Token::UnaryOperator(operator)) => unary_expression(operator, source),
                        _ => error!("expected identifier or literal followed by expression"),
                }
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
                Some(Token::Assign) => expression(source),
                _ => error!("expected assign operator"),
        }
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
