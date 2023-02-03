use crate::parser::*;
use crate::lexer::error::{MudResult, ErrorType};

pub fn compile(program: Vec<u8>) -> MudResult<Vec<u8>>{
    let mut parser = Parser::new(program);
    let expression = parser.parse()?;

    let output = convert(expression)?.as_bytes().to_owned();
    Ok(output)
}
 
pub fn compile_full(program: Vec<u8>) -> MudResult<Vec<u8>>{
    let output = compile(program)?;

    Ok(
        format!(
"#include <stdio.h>
#include <stdlib.h>

int main() {{
    {}}}",
            String::from_utf8(output).unwrap()
            ).into_bytes()
      )
}

fn binary_op_transpile(op: Operator, lhs: &str, rhs: &str) -> MudResult<String> {
    match op {
        Operator::Plus => Ok(format!("({lhs}+{rhs})")),
        Operator::Minus => Ok(format!("({lhs}-{rhs})")),
        Operator::Asterisk => Ok(format!("({lhs}*{rhs})")),
        Operator::Semicolon=> Ok(format!("{lhs};\n{rhs}")),
        Operator::Colon=> Ok(format!("{rhs} {lhs}")),
        Operator::Equals=> Ok(format!("{lhs} = {rhs}")),

        _ => Err(ErrorType::CompileError(format!("Binary operator {:?} cannot be transpiled", op))),
    }
}

fn unary_op_transpile(op: Operator, oprand: &str) -> MudResult<String> {
    match op {
        Operator::Minus => Ok(format!("-({oprand})")),
        Operator::LessThan => Ok(format!("printf(\"{{}}\", {oprand})")),
        _ => Err(ErrorType::CompileError(format!("Unary operator {:?} cannot be transpiled", op))),
    }
}

fn convert(expression: Expression) -> MudResult<String> {
    match expression {
        Expression::Integer(val) => {
            Ok(val.to_string())
        },
        Expression::Identifier(s) => {
            Ok(s)
        }
        Expression::UnaryOperation(op, expr) => {
            unary_op_transpile(op, &convert(*expr)?)
        },
        Expression::BinaryOperation(op, lhs, rhs) => {
            binary_op_transpile(op, &convert(*lhs)?, &convert(*rhs)?)
        },
        Expression::Null => Ok(String::new()),
    }
}
