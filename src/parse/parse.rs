use super::*;
use crate::lex::*;
use std::iter::Peekable;
use std::process::exit;
use std::vec::IntoIter;

macro_rules! error {
        ($message: expr) => {{
                eprintln!("PARSING ERROR: {}", $message);
                exit(1);
        }};
}

fn binary_expression(left: Expression, source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        let operator = match source.next() {
                Some(Token::BinaryOperator(operator)) => operator,
                _ => error!("expected binary operator or end of line token"),
        };
        let right = expression(source);
        Expression::Binary(Box::new(left), operator, Box::new(right))
}

fn unary_expression(operator: UnaryOperator, source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        let expression = match source.next() {
                Some(Token::Identifier(identifier)) => Expression::Identifier(identifier),
                Some(Token::OpenParenthesis) => group_expression(source),
                _ => error!("invalid unary expression"),
        };
        let expression = Expression::Unary(operator, Box::new(expression));
        match source.peek() {
                Some(Token::BinaryOperator(..)) => binary_expression(expression, source),
                _ => expression,
        }
}

fn group_expression(source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        let expression = expression(source);
        if source.next() != Some(Token::CloseParenthesis) {
                error!("expected close parenthesis")
        }
        match source.peek() {
                Some(Token::BinaryOperator(..)) => binary_expression(expression, source),
                _ => expression,
        }
}

fn expression(source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        match (source.next(), source.peek()) {
                (Some(Token::Identifier(identifier)), Some(Token::BinaryOperator(..))) => {
                        binary_expression(Expression::Identifier(identifier), source)
                }
                (Some(Token::Integer(integer)), Some(Token::BinaryOperator(..))) => {
                        binary_expression(Expression::Integer(integer), source)
                }
                (Some(Token::UnaryOperator(operator)), ..) => unary_expression(operator, source),
                (Some(Token::OpenParenthesis), ..) => group_expression(source),
                (Some(Token::Identifier(identifier)), None)
                | (Some(Token::Identifier(identifier)), Some(Token::CloseParenthesis))
                | (Some(Token::Identifier(identifier)), Some(Token::Identifier(..))) => {
                        Expression::Identifier(identifier)
                }
                (Some(Token::Integer(integer)), None)
                | (Some(Token::Integer(integer)), Some(Token::CloseParenthesis))
                | (Some(Token::Integer(integer)), Some(Token::Identifier(..))) => {
                        Expression::Integer(integer)
                }
                (token, _) => {
                        println!("{token:?}");
                        error!("invalid expression")
                }
        }
}

fn assign(source: &mut Peekable<IntoIter<Token>>) -> Expression
{
        if source.next() == Some(Token::Assign) {
                expression(source)
        }
        else {
                error!("expected assign operator")
        }
}

fn variable(identifier: String, source: &mut Peekable<IntoIter<Token>>) -> Statement
{
        Statement::Variable(identifier, assign(source))
}

fn mutable_variable(source: &mut Peekable<IntoIter<Token>>) -> Statement
{
        let token = source.next();
        match token {
                Some(Token::Identifier(identifier)) => {
                        Statement::MutableVariable(identifier, assign(source))
                }
                _ => error!("expected identifier after 'mutable' keyword"),
        }
}

pub fn statement(source: &mut Peekable<IntoIter<Token>>) -> Option<Statement>
{
        let token = source.next()?;
        match token {
                Token::Identifier(identifier) => Some(variable(identifier, source)),
                Token::Mutable => Some(mutable_variable(source)),
                _ => error!(format!("invalid token: {:?}", token)),
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
