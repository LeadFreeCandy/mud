use std::{string::ParseError, collections::HashMap};

use crate::{lexing::{Operator, Lexer, Token}, errors::{ParseResult, ErrorType}};

#[derive(Debug)]
pub enum Expression {
    Integer(u64),
    BinaryOperation(Operator, Box<Expression>, Box<Expression>),
    UnaryOperation(Operator, Box<Expression>),
}

pub struct Parser {
    lexer: Lexer,
    token: Token,
    precedence_lookup: HashMap<Operator, u8>,
}

impl Parser {
    pub fn new(program: Vec<u8>) -> Self {
        let mut precedence_lookup = HashMap::new();

        precedence_lookup.insert(Operator::Addition, 2);
        precedence_lookup.insert(Operator::Multiplication, 1);

        Self { 
            lexer: Lexer::new(program),
            token: Token::Eof,
            precedence_lookup,
        }
    }

    pub fn parse(&mut self) -> ParseResult<Expression> {
        self.advance()?;

        self.binary_operation(2)
    }

    fn binary_operation(&mut self, precedence: u8) -> ParseResult<Expression> {
        if precedence == 0 {
            return self.term();
        }

        let mut expr = self.binary_operation(precedence - 1)?;

        while let Token::Operator(op) = self.token {
            if *self.precedence_lookup.get(&op).ok_or(ErrorType::ParseError("Invalid Operator".to_string()))? == precedence {
                self.advance()?;
                expr = Expression::BinaryOperation(op, Box::new(expr), Box::new(self.binary_operation(precedence - 1)?));
            }
            else {
                break;
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expression> {

        match self.token {
            Token::Integer(i) => {
                self.advance()?;
                Ok(Expression::Integer(i))
            }

            _ => Err(ErrorType::ParseError("Expected term".to_string()))
        }
    }

    fn advance(&mut self) -> ParseResult<()> {
        self.token = self.lexer.next()?;
        Ok(())
    }
}
