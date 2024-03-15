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
            let (index, len) = (span.start(), span.len());
            match span.value {
                Ok(token) => tokens.push(token.spanned(index, len)),
                Err(error) => errors.push(error.spanned(index, len)),
            }
            (tokens, errors)
        },
    );

    let code = String::from_utf8_lossy(&bytes);
    if !errors.is_empty() {
        println!("\nLexer Errors:\n");
        println!(
            "{}",
            util::map_spans(&errors, &*code, |str| format!("\x1b[41m{str}\x1b[0m")).expect("xd")
        );
    }

    println!("\nTokens:\n");
    for (i, token) in tokens.into_iter().enumerate() {
        println!("{i}: {:?}", token);
    }
}
