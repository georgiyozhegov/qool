use qoolang::generate::*;
use qoolang::lex::lex;
use qoolang::parse::parse;
use std::collections::HashMap;
use std::fs;
use std::env::args;

fn execute(instructions: Vec<Instruction>) -> HashMap<String, i32>
{
        let mut stack = Vec::new();
        let mut variables = HashMap::new();
        for instruction in instructions {
                match instruction {
                        Instruction::CreateVariable(identifier) => {
                                variables.insert(identifier, stack.pop().unwrap());
                        }
                        Instruction::LoadIdentifier(identifier) => {
                                stack.push(*variables.get(&identifier).expect(format!(
                                        "RUNTIME ERROR: identifier '{identifier}' is not found"
                                )
                                .as_str()));
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
        variables
}

fn main()
{
        let source_path = args().nth(1).expect("SYSTEM ERROR: expected source path as first argument");
        let source = fs::read_to_string(&source_path).expect(format!("SYSTEM ERROR: file '{source_path}' is not found").as_str());
        let source = source.chars().peekable();
        let tokens = lex(source);
        let tokens = tokens.into_iter().peekable();
        let tree = parse(tokens);
        let instructions = generate(tree);
        let variables = execute(instructions);
        println!("{variables:?}");
}
