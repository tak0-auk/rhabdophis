use parser::token::Token;

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

    }

}