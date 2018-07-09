const DOC: &'static str =
"Rhabdophis is implement for Rust
Try \' --help\' for more infomation";

use std::env;

fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    let _ = match args.first() {
        Some(file_name) => {

        },
        None => {
            println!("{}", DOC);
            // executer::exec_repl()
        },
    };

}