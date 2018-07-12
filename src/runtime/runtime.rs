use std::io::{self, Write};

use parser::lexer::Lexer;
use parser::parser::Parser;

#[derive(Debug)]
pub struct Runtime {
    lexer: Lexer,
    parser: Parser,
}

// impl Runtime {
//     pub fn new() -> Runtime {

//     }
// }

pub fn execute_file() {

}

pub fn exec_repl() {
    let mut line = String::new();
    // let mut token: Vec<Token> = vec![];

    loop {
        let _ = io::stdout().write(b">>> ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                let parser = Parser::new(
                        Lexer::new(line.clone()).get_tokens()
                );
                line.clear();
                println!("{:?}", parser);
            },
            Err(_) => {
                println!("error");
            },
        };
    }
    // io::stdout().write(src.as_bytes());
}