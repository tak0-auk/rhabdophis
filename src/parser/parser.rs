use parser::token::{ Token, TokenKind };

use object::object::{ PyObject };
use object::int::{ PyIntObject };
use object::*;

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
        // let mut before_obj: Box<PyObject>;
        let mut nodes: Vec<PyObject> = vec![];
        // let ti = &self.tokens.iter();
        for token in &self.tokens {
            match token.kind {
                TokenKind::Identifier(ref s) => {
                    let obj = PyObject::new(s.clone());
                    println!("{:?}", obj);
                },
                TokenKind::Int(i) => {
                    let int = PyIntObject::new(i);
                    println!("{:?}", int);
                },
                TokenKind::String(ref s) => {
                    let string = string::PyStringObject::new(s.to_string());
                    println!("{:?}", string);
                },
                TokenKind::Equal => {
                    let o = nodes.last();

                },
                TokenKind::LPar => {
                    // (&ti).next();
                }
                _ => {
                    println!("{:?}", token);
                },
            }
        }


    }

}