mod lexer;
mod parser;

use lexer::Lexer;
use parser::{Program, TryParse};

fn main() {
    let source = include_str!("samples/hello.kt");
    let pairs: Vec<_> = Lexer::new(source).collect();
    let program = Program::try_parse(&pairs).unwrap();
    dbg!(program);
}
