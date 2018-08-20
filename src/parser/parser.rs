use parser::token::{ Token, TokenKind };

use object::object::{ PyObject };
use object::pyIntObject::{ PyIntObject };

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
        }
    }
}

impl Parser {
    pub fn parse(&mut self) {
        // let mut before_obj: PyObject;
        for token in &self.tokens {
            match token.kind {
                TokenKind::Int(i) => {
                    let int = PyIntObject::new(i);
                    println!("{:?}", int);
                },
                TokenKind::Plus => {

                },
                _ => {
                    println!("{:?}", token);
                },
            }
        }
    }

}