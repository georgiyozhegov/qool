use crate::lex::*;
use crate::parse::*;

#[derive(Debug)]
pub enum Instruction
{
        CreateVariable(String),
        LoadIdentifier(String),
        LoadInteger(i32),
        Add,
        Subtract,
        Multiply,
        Divide,
}

fn generate_expression(expression: Expression) -> Vec<Instruction>
{
        let mut instructions = Vec::new();
        match expression {
                Expression::Identifier(identifier) => {
                        instructions.push(Instruction::LoadIdentifier(identifier))
                }
                Expression::Integer(integer) => {
                        instructions.push(Instruction::LoadInteger(integer))
                }
                Expression::Binary(left, operator, right) => {
                        instructions.extend(generate_expression(*left));
                        instructions.extend(generate_expression(*right));
                        instructions.push(match operator {
                                BinaryOperator::Add => Instruction::Add,
                                BinaryOperator::Subtract => Instruction::Subtract,
                                BinaryOperator::Multiply => Instruction::Multiply,
                                BinaryOperator::Divide => Instruction::Divide,
                                _ => todo!(),
                        });
                }
                _ => todo!(),
        }
        instructions
}

pub fn generate(statements: Vec<Statement>) -> Vec<Instruction>
{
        let mut instructions = Vec::new();
        for statement in statements {
                match statement {
                        Statement::Variable(identifier, expression) => {
                                let expression_instructions = generate_expression(expression);
                                println!("{expression_instructions:?}");
                                instructions.extend(expression_instructions);
                                instructions.push(Instruction::CreateVariable(identifier));
                        }
                        _ => todo!(),
                }
        }
        instructions
}
