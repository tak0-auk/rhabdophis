const DOC: &'static str =
"Rhabdophis is Python interpreter
Try \' --help\' for more infomation";

use std::env;

extern crate rhabdophis;
use rhabdophis::util::util;
use rhabdophis::runtime;
use rhabdophis::parser::lexer;
use rhabdophis::parser::parser;

fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    let _ = match args.first() {
        Some(file_name) => {
            let mut parser = parser::Parser::new(lexer::Lexer::new(
                util::get_file_text(file_name)
                ).get_tokens()
            );
            parser.parse();
        },
        None => {
            println!("{}", DOC);
            runtime::runtime::exec_repl();
        },
    };

}