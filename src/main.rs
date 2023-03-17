mod lexer;
mod parser;

use lexer::{Lexer, Token};

fn main() {
    let source = include_str!("samples/hello.kt");
    let pairs: Vec<_> = Lexer::new(source).collect();

    println!("source: {source}");
    println!();

    for pair in &pairs {
        let tok = pair.token;
        let string = pair.str();
        match tok {
            Token::NewLine => println!("new line"),
            Token::Symbol(s) => println!("symbol : {s}"),
            _ => println!("{tok:?} : {string}"),
        }
    }
}
