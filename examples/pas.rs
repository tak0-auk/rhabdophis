extern crate rhabdophis;
use rhabdophis::parser::lexer;
use rhabdophis::parser::parser;
use rhabdophis::util;

fn main() {
    println!("it is example parser!");
    let text = util::util::get_file_text("examples/sample.py");
    println!("Source");
    println!("------------------------------");
    print!("{}", text);
    println!("------------------------------");
    let mut parser = parser::Parser::new(lexer::Lexer::new(text).get_tokens());
    parser.parse();
}