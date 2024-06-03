use super::token::*;
use std::iter::Peekable;
use std::str::Chars;

macro_rules! skip {
        ($source: expr) => {
                $source.next();
        };
}

macro_rules! take_until {
        ($source: expr, $predicate: ident) => {{
                let mut buffer = String::new();
                while let Some(c) = $source.peek() {
                        if $predicate(*c) {
                                buffer.push(*c);
                                skip!($source);
                        }
                        else {
                                break;
                        }
                }
                buffer
        }};
}

fn is_alpha(c: char) -> bool
{
        c.is_ascii_alphabetic()
}

fn alpha(source: &mut Peekable<Chars>) -> Token
{
        let token = take_until!(source, is_alpha);
        match token.as_str() {
                "mutable" => Token::Mutable,
                _ => Token::Identifier(token),
        }
}

fn is_digit(c: char) -> bool
{
        c.is_ascii_digit()
}

fn digit(source: &mut Peekable<Chars>) -> Token
{
        let token = take_until!(source, is_digit);
        Token::Integer(token.parse().unwrap())
}

fn token(source: &mut Peekable<Chars>) -> Option<Token>
{
        let c = source.peek()?;
        if is_alpha(*c) {
                Some(alpha(source))
        }
        else if is_digit(*c) {
                Some(digit(source))
        }
        else if *c == ':' {
                skip!(source);
                Some(Token::Assign)
        }
        else if *c == '+' {
                skip!(source);
                Some(Token::BinaryOperator(BinaryOperator::Add))
        }
        else if *c == '-' {
                skip!(source);
                Some(Token::BinaryOperator(BinaryOperator::Subtract))
        }
        else if *c == '*' {
                skip!(source);
                Some(Token::BinaryOperator(BinaryOperator::Multiply))
        }
        else if *c == '/' {
                skip!(source);
                Some(Token::BinaryOperator(BinaryOperator::Divide))
        }
        else if *c == '!' {
                skip!(source);
                Some(Token::UnaryOperator(UnaryOperator::Not))
        }
        else if *c == '(' {
                skip!(source);
                Some(Token::OpenParenthesis)
        }
        else if *c == ')' {
                skip!(source);
                Some(Token::CloseParenthesis)
        }
        else if c.is_whitespace() {
                skip!(source);
                token(source)
        }
        else {
                todo!()
        }
}

pub fn lex(mut source: Peekable<Chars>) -> Vec<Token>
{
        let mut tokens = Vec::new();
        while let Some(token) = token(&mut source) {
                tokens.push(token);
        }
        tokens
}
