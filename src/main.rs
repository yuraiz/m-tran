mod analyzer;
mod interpreter;
mod lexer;
mod parser;

use analyzer::{check_program, pretty_print_error};
use interpreter::Context;
use lexer::Lexer;
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
    let prog = Program::try_parse(&pairs).unwrap().0;

    let errors = check_program(&prog);

    if errors.is_empty() {
        Context::new(prog).run();
    } else {
        for (span, ref message) in check_program(&prog) {
            pretty_print_error(source, span, message)
        }
    }
}
