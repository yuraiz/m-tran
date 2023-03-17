use std::error::Error;
use std::fmt::Display;

use crate::lexer::{Pair, Token};

#[derive(Debug)]
pub enum ParseError<'a> {
    UnexpectedEndOfInput,
    NotImplementedYet,
    WrongExprType(&'static str),
    UnexpectedToken(Pair<'a>, Token),
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseError::NotImplementedYet => write!(f, "Not implemented yer"),
            ParseError::WrongExprType(expected) => {
                write!(f, "Wrong expression type, expected {expected}")
            }
            ParseError::UnexpectedToken(pair, token) => {
                let unexpected_token = pair.token;
                let string = pair.str();
                write!(
                    f,
                    "Expected {token:?} but got {unexpected_token:?} : {string}"
                )
            }
        }
    }
}

impl Error for ParseError<'_> {}

pub type ParseResult<'a, T> = Result<(T, &'a [Pair<'a>]), ParseError<'a>>;
