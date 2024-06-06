use qoolang::generate::*;
use qoolang::lex::lex;
use qoolang::parse::parse;
use std::collections::HashMap;

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
        let source = r#"
            a : 3
            b : 2
            c : (b + 2) * a
        "#;
        let source = source.chars().peekable();
        let tokens = lex(source);
        let tokens = tokens.into_iter().peekable();
        let tree = parse(tokens);
        let instructions = generate(tree);
        let variables = execute(instructions);
        println!("{variables:?}");
}
