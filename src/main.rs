const DOC: &'static str =
"Rhabdophis is implement for Rust
Try \' --help\' for more infomation";

use std::env;

extern crate rhabdophis;
use rhabdophis::util::util;
use rhabdophis::runtime;
use rhabdophis::parser;

fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    let _ = match args.first() {
        Some(file_name) => {
            for token in parser::lexer::Lexer::new(util::get_file_text(file_name)).get_tokens(){
                println!("{:?}", token);
            };
        },
        None => {
            println!("{}", DOC);
            runtime::runtime::exec_repl();
        },
    };

}