#[derive(Debug, PartialEq)]
pub enum BinaryOperator
{
        Add,
        Subtract,
        Multiply,
        Divide,
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
}
impl Token
{
        pub fn is_binary_operator(&self) -> bool
        {
                match self {
                        Self::BinaryOperator(..) => true,
                        _ => false,
                }
        }
}
