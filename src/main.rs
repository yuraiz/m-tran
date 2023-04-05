mod lexer;
mod parser;

use lexer::{Lexer, Token};

use std::collections::HashMap;

fn main() {
    let source = include_str!("samples/arrays.kt");
    let pairs: Vec<_> = Lexer::new(source).collect();

    println!("SOURCE:\n{source}\n");

    let keywords = pairs
        .iter()
        .filter_map(|pair| {
            let desc = match pair.token {
                Token::Fun => "used to define function",
                Token::If | Token::Else | Token::Return => "conditional statement",
                Token::For | Token::While => "loop condition",
                Token::In => "used inside for condition",
                Token::Var | Token::Val => "variable binding",
                _ => return None,
            };
            Some((pair.str(), desc))
        })
        .collect();

    print_table("KEYWORDS TABLE", keywords);

    let functions = pairs
        .iter()
        .zip(pairs.iter().skip(1))
        .filter_map(|(pair1, pair2)| match pair1.token {
            Token::Fun => Some((pair2.str(), "function")),
            _ => None,
        })
        .collect();

    print_table("FUNCTIONS TABLE", functions);

    let idents = pairs
        .iter()
        .zip(pairs.iter().skip(1))
        .filter_map(|(pair1, pair2)| {
            let desc = match pair1.token {
                Token::Val => "val",
                Token::Var => "var",
                _ => return None,
            };

            Some((pair2.str(), desc))
        })
        .collect();

    print_table("VARIABLES TABLE", idents);
}

fn print_table(header: &str, items: HashMap<&str, &str>) {
    const WIDTH: usize = 40;

    println!("+{:->WIDTH$}+", "");
    println!("| {header:w$}| ", w = WIDTH - 1);
    println!("+{:->WIDTH$}+", "");
    for (name, desc) in items {
        println!("| {name:10} | {desc:w$} |", w = WIDTH - 15);
    }
    println!("+{:->WIDTH$}+", "");
}
