mod analyzer;
mod lexer;
mod parser;

use analyzer::{check_program, pretty_print_error};
use lexer::Lexer;
use parser::{Program, TryParse};

fn main() {
    let source = include_str!("samples/arrays.kt");
    let pairs: Vec<_> = Lexer::new(source).collect();
    let prog = Program::try_parse(&pairs).unwrap().0;

    for (span, ref message) in check_program(&prog) {
        pretty_print_error(source, span, message)
    }
}
