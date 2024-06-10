use qool::generate::*;
use qool::lex::lex;
use qool::parse::parse;
use std::collections::HashMap;
use std::fs;

fn execute(instructions: Vec<Instruction>) -> (Vec<i32>, HashMap<String, i32>)
{
        let mut stack = Vec::new();
        let mut variables = HashMap::new();
        for instruction in instructions {
                match instruction {
                        Instruction::CreateVariable(identifier) => {
                                variables.insert(identifier, stack.pop().unwrap());
                        }
                        Instruction::LoadIdentifier(identifier) => {
                                stack.push(*variables.get(&identifier).unwrap_or_else(||
                                        panic!("RUNTIME ERROR: identifier '{identifier}' is not found")
                                ));
                        }
                        Instruction::LoadInteger(integer) => stack.push(integer),
                        Instruction::Add => {
                                let right = stack.pop().unwrap();
                                let left = stack.pop().unwrap();
                                stack.push(left + right);
                        }
                        Instruction::Subtract => {
                                let right = stack.pop().unwrap();
                                let left = stack.pop().unwrap();
                                stack.push(left - right);
                        }
                        Instruction::Multiply => {
                                let right = stack.pop().unwrap();
                                let left = stack.pop().unwrap();
                                stack.push(left * right);
                        }
                        Instruction::Divide => {
                                let right = stack.pop().unwrap();
                                let left = stack.pop().unwrap();
                                stack.push(left / right);
                        }
                };
        }
        (stack, variables)
}

fn main()
{
        let source = fs::read_to_string("file.qool").unwrap();
        let source = source.chars().peekable();
        let tokens = lex(source);
        let tokens = tokens.into_iter().peekable();
        let tree = parse(tokens);
        println!("ast: {tree:?}");
        let instructions = generate(tree);
        println!("instructions: {instructions:?}");
        let (stack, variables) = execute(instructions);
        println!("stack: {stack:?} | variables: {variables:?}");
}
