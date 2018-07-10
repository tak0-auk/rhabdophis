extern crate rhabdophis;
use rhabdophis::parser::lexer;
use rhabdophis::util;

fn main() {
    println!("it is example!");
    let text = util::util::get_file_text("examples/sample.py");
    println!("Source");
    println!("------------------------------");
    print!("{}", text);
    println!("------------------------------");
    let mut lex = lexer::Lexer::new(text);
    for token in lex.get_tokens() {
        println!("{:?}", token.value);
    }
}