use crate::parse::*;
use crate::lex::*;

fn variables(code: &mut String, statements: &Vec<Statement>) {
    code.push_str("section .bss\n");
    for statement in statements {
        match statement {
            Statement::Variable(identifier, _) => {
                let line = format!("\t{identifier}: resq 1\n");
                code.push_str(&line);
            }
            _ => {},
        }
    }
}

fn expression_(code: &mut String, expression: Expression)
{
    match expression {
        Expression::Integer(integer) => {
            let line = format!("\tmov rax, {integer}\n");
            code.push_str(&line);
        },
        Expression::Identifier(identifier) => {
            let line = format!("\tmov rax, [{identifier}]\n");
            code.push_str(&line);
        },
        Expression::Binary(left, operator, right) => {
            expression_(code, *left);
            code.push_str("\tmov rbx, rax\n");
            expression_(code, *right);
            code.push_str(match operator {
                BinaryOperator::Add => "\tadd rax, rbx\n",
                BinaryOperator::Subtract => "\tsub rax, rbx\n",
                _ => todo!(),
            })
        },
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
            Statement::Variable(identifier, expression) => {
                expression_(code, expression.clone());
                let line = format!("\tmov QWORD [{identifier}], rax\n");
                code.push_str(&line);
            },
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

