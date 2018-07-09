extern crate rhabdophis;
use rhabdophis::parser::lexer;
use rhabdophis::util;

fn main() {
    println!("it is example!");
    let mut lex = lexer::Lexer::new(util::util::get_file_text("sample.py"));
    for token in lex.get_tokens() {
        println!("{:?}", token);
    }
}