mod analyzer;
mod interpreter;
mod lexer;
mod parser;

use analyzer::{check_program, pretty_print_error};
use interpreter::Context;
use lexer::{Lexer, Span};
use parser::{Program, TryParse};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass path to kotlin file as argument"),
        2 => {
            let file_name = &args[1];
            if let Ok(source) = std::fs::read_to_string(file_name) {
                interpret(&source);
            } else {
                println!("File {file_name} not found")
            };
        }
        _ => println!("Too many arguments"),
    };
}

fn interpret(source: &str) {
    let pairs: Vec<_> = Lexer::new(source).collect();

    match Program::try_parse(&pairs) {
        Ok((prog, pairs)) if pairs.is_empty() => {
            let errors = check_program(&prog);

            if errors.is_empty() {
                Context::new(prog).run();
            } else {
                match errors.len() {
                    1 => eprintln!("Found error:"),
                    num => eprintln!("Found {num} errors:"),
                };
                for (span, ref message) in check_program(&prog) {
                    pretty_print_error(source, span, message)
                }
            }
        }
        Ok(_) => eprintln!("Source is not fully parsed"),
        Err(error) => print_parse_error(source, error),
    }
}

fn print_parse_error(source: &str, error: parser::ParseError) {
    eprintln!("Syntax Error:");
    match error {
        parser::ParseError::UnexpectedEndOfInput => pretty_print_error(
            source,
            Span {
                lo: source.len(),
                hi: source.len(),
            },
            "Unexpected end of input",
        ),
        parser::ParseError::NotImplementedYet => eprintln!("Use of not implemented feature"),
        parser::ParseError::WrongExprType(pair, expected) => pretty_print_error(
            source,
            pair.span,
            &format!("Wrong, expression type, expected: {expected}"),
        ),
        parser::ParseError::UnexpectedToken(pair, expected) => pretty_print_error(
            source,
            pair.span,
            &format!(
                "Wrong token, expected {expected:?} but got {unexpected:?}",
                unexpected = pair.token
            ),
        ),
    }
}
