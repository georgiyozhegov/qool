use crate::lex::*;

#[derive(Debug)]
pub enum Statement
{
        Variable(String, Expression),
        MutableVariable(String, Expression),
}

#[derive(Debug, Clone)]
pub enum Expression
{
        Integer(i32),
        Identifier(String),
        Unary(UnaryOperator, Box<Expression>),
        Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}
