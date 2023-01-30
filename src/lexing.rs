use std::collections::HashMap;

use crate::errors::{ParseResult, ErrorType};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
}

#[derive(Debug)]
pub enum Lexeme {
    Integer(u64),
    Operator(Operator),
    Eof,
}

pub struct Lexer {
    program: Vec<u8>,
    index: usize,

    operator_map: HashMap<String, Operator>, // NOTE: Maybe change these lookup methods
    operators: [bool; 256], 
}

impl Lexer {
    pub fn new(program: Vec<u8>) -> Self {
        let mut operator_map = HashMap::new();
        let mut operators = [false; 256];

        operator_map.insert("+".to_string(), Operator::Addition);
        operator_map.insert("-".to_string(), Operator::Subtraction);
        operator_map.insert("*".to_string(), Operator::Multiplication);

        for op in operator_map.keys() {
            for c in op.bytes() {
                operators[c as usize] = true;
            }
        }

        Self { 
            program, 
            index: 0,

            operator_map,
            operators,
         }
    }

    pub fn next(&mut self) -> ParseResult<Lexeme> {
        while self.peek().is_ascii_whitespace() {
            self.index += 1;
        }

        match self.peek() {
            c if c.is_ascii_digit() => self.integer(),
            c if self.operators[c as usize] => self.operator(),
            0 => Ok(Lexeme::Eof),
            _ => Err(ErrorType::LexError("Invalid character".to_string()))
        }
    }

    fn integer(&mut self) -> ParseResult<Lexeme> {
        let mut int: u64 = 0;

        while self.peek().is_ascii_digit() {
            int = int
                .checked_mul(10).ok_or(ErrorType::LexError("Overflowing integer literal".to_string()))?
                .checked_add((self.peek() - b'0') as u64).ok_or(ErrorType::LexError("Overflowing integer literal".to_string()))?;

            self.index += 1;
        }

        Ok(Lexeme::Integer(int))
    }

    fn operator(&mut self) -> ParseResult<Lexeme> {
        let mut op_string = String::new();

        let mut largest_op = None;
        let mut op_last_index = 0;

        while self.operators[self.peek() as usize] {
            op_string.push(self.peek() as char);

            if let Some(op) = self.operator_map.get(&op_string) {
                largest_op = Some(*op);
                op_last_index = self.index;
            }

            self.index += 1;
        }

        self.index = op_last_index + 1;
        
        Ok(Lexeme::Operator(largest_op.ok_or(ErrorType::LexError("Invalid operator".to_string()))?))
    }

    fn peek(&mut self) -> u8 {
        *self.program.get(self.index).unwrap_or(&0)
    }
}
