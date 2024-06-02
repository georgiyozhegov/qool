#[derive(Debug, PartialEq)]
pub enum BinaryOperator
{
        Add,
        Subtract,
}

#[derive(Debug, PartialEq)]
pub enum Token
{
        Identifier(String),
        Integer(i32),
        Assign,
        BinaryOperator(BinaryOperator),
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
