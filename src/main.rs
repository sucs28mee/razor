mod lexer;
mod util;

use std::{env, fs};

use crate::util::Spanned;

fn main() {
    let path = env::args().nth(1).expect("Expected a path argument.");
    let bytes = fs::read(path).expect("IO Error");
    let (tokens, errors) = lexer::Lexer::new(&bytes).fold(
        (Vec::new(), Vec::new()),
        |(mut tokens, mut errors), span| {
            let (index, len) = (span.index(), span.len());
            match span.value {
                Ok(token) => tokens.push(token.spanned(index, len)),
                Err(error) => errors.push(error.spanned(index, len)),
            }
            (tokens, errors)
        },
    );

    println!("\nErrors:\n");
    for (i, error) in errors.into_iter().enumerate() {
        println!("{i}: {:?}", error);
    }

    println!("\nTokens:\n");
    for (i, token) in tokens.into_iter().enumerate() {
        println!("{i}: {:?}", token);
    }
}
