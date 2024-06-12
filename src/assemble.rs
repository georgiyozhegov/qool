use crate::lex::*;
use crate::parse::*;

const REGISTERS: [&str; 4] = ["rax", "rbx", "rcx", "rdx"];

fn variables(code: &mut String, statements: &Vec<Statement>)
{
        code.push_str("section .data\n");
        for statement in statements {
                match statement {
                        Statement::Variable(identifier, expression) => match expression {
                                Expression::Integer(integer) => {
                                        let line = format!("\t{identifier}: dq {integer}\n");
                                        code.push_str(&line);
                                }
                                _ => {}
                        },
                        _ => {}
                }
        }
        code.push_str("section .bss\n");
        for statement in statements {
                match statement {
                        Statement::Variable(identifier, expression) => {
                                if !matches!(expression, Expression::Integer(..)) {
                                        let line = format!("\t{identifier}: resq 1\n");
                                        code.push_str(&line);
                                }
                        }
                        _ => {}
                }
        }
}

fn expression_(code: &mut String, expression: Expression, register: &mut usize)
{
        match expression {
                Expression::Integer(integer) => {
                        let line = format!("\tmov {}, {}\n", REGISTERS[*register], integer);
                        *register += 1;
                        code.push_str(&line);
                }
                Expression::Identifier(identifier) => {
                        let line = format!("\tmov {}, [{}]\n", REGISTERS[*register], identifier);
                        *register += 1;
                        code.push_str(&line);
                }
                Expression::Binary(left, operator, right) => {
                        let first = REGISTERS[*register];
                        expression_(code, *left, register);
                        let second = REGISTERS[*register];
                        expression_(code, *right, register);
                        code.push_str(
                                match operator {
                                        BinaryOperator::Add => format!("\tadd {first}, {second}\n"),
                                        BinaryOperator::Subtract => {
                                                format!("\tsub {first}, {second}\n")
                                        }
                                        BinaryOperator::Multiply => {
                                                format!("\timul {first}, {second}\n")
                                        }
                                        _ => todo!(),
                                }
                                .as_str(),
                        );
                }
                _ => todo!(),
        }
}

fn program(code: &mut String, statements: &Vec<Statement>)
{
        code.push_str("section .text\n");
        code.push_str("\tglobal _start\n");
        code.push_str("_start:\n");
        for statement in statements {
                match statement {
                        Statement::MutableVariable(identifier, expression) => {
                                expression_(code, expression.clone(), &mut 0);
                                let line = format!("\tmov QWORD [{identifier}], rax\n");
                                code.push_str(&line);
                                code.push('\n');
                        }
                        Statement::Variable(identifier, expression) => {
                                if !matches!(expression, Expression::Integer(..)) {
                                        expression_(code, expression.clone(), &mut 0);
                                        let line = format!("\tmov QWORD [{identifier}], rax\n");
                                        code.push_str(&line);
                                        code.push('\n');
                                }
                        }
                        _ => todo!(),
                }
        }
}

fn exit(code: &mut String)
{
        code.push_str("\tmov rax, 60\n");
        code.push_str("\tmov rdi, 0\n");
        code.push_str("\tsyscall\n");
}

pub fn assemble(statements: Vec<Statement>) -> String
{
        let mut code = String::new();
        variables(&mut code, &statements);
        program(&mut code, &statements);
        exit(&mut code);
        code
}
