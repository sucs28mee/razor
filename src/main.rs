mod codegen;
mod expr_tree;
mod lexer;
mod parser;
mod util;

use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("Expected a path argument.");
    let bytes = fs::read(path).expect("Couldn't read the source file.");

    let (mut tokens, mut errors) = (Vec::new(), Vec::new());
    for result in lexer::tokenize(bytes.clone()) {
        match result {
            Ok(token) => tokens.push(token),
            Err(error) => errors.push(error),
        }
    }

    let code = String::from_utf8_lossy(&bytes);
    if !errors.is_empty() {
        println!("\nLexer Errors:\n");
        println!(
            "{}",
            util::map_spans(&errors, &*code, |str| format!("\x1b[41m{str}\x1b[0m")).expect("xd")
        );

        for error in errors {
            println!("{:?}", error.value);
        }
    }

    println!("\nTokens:\n");
    for (i, token) in tokens.iter().enumerate() {
        println!("{i}: {:?}", token);
    }

    let (mut items, mut errors) = (Vec::new(), Vec::new());
    for result in parser::parse(tokens) {
        match result {
            Ok(token) => items.push(token),
            Err(error) => errors.push(error),
        }
    }

    println!("\nItems:\n");
    for item in items.iter() {
        println!("{:?}", item);
    }

    let code = codegen::gen_c(items).expect("Couldn't generate code.");
    println!("{code}");
}
