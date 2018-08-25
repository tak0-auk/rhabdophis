use std::io::{self, Write};

use parser::lexer::Lexer;
use parser::parser::Parser;
use object;

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

    loop {
        let _ = io::stdout().write(b">>> ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                let mut tokens = Lexer::new(line.clone()).get_tokens();
                let mut parser = Parser::new(tokens);
                parser.parse();
            },
            Err(_) => {
                println!("error");
            },
        };

        line.clear();
    }
    // io::stdout().write(src.as_bytes());
}