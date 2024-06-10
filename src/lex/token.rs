#[derive(Debug, PartialEq)]
pub enum BinaryOperator
{
        Add,
        Subtract,
        Multiply,
        Divide,
        Equal,
        Greater,
        Less,
        EqualOrGreater,
        EqualOrLess,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator
{
        Not,
}

#[derive(Debug, PartialEq)]
pub enum Token
{
        Identifier(String),
        Integer(i32),
        Assign,
        BinaryOperator(BinaryOperator),
        UnaryOperator(UnaryOperator),
        OpenParenthesis,
        CloseParenthesis,
        Mutable,
}

impl Token
{
        pub fn is_binary_operator(&self) -> bool
        {
                matches!(self, Token::BinaryOperator(..))
        }
}
