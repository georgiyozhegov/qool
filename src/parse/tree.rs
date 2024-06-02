use crate::lex::*;

#[derive(Debug)]
pub enum Statement
{
        Variable(String, Expression),
}

#[derive(Debug)]
pub enum Expression
{
        Integer(i32),
        Identifier(String),
        BinaryExpression(Box<Expression>, Token, Box<Expression>),
}
