mod lexer;
mod util;

use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("Expected a path argument.");
    let code = fs::read(path).expect("IO Error");
    println!("{:?}", code.len());
    for token in lexer::Lexer::new(&code)
        .collect::<Result<Vec<_>, _>>()
        .expect("Lexer error")
    {
        println!("{:?}", token);
    }
}
