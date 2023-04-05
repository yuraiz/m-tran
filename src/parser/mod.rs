mod expr;
mod helpers;
mod parse_error;

use crate::lexer::*;

use self::helpers::*;
use expr::*;
pub use parse_error::{ParseError, ParseResult};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Fun>,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Simple(Ident),
    Generic(Ident, Vec<Type>),
}

#[derive(Debug, PartialEq)]
pub struct Fun {
    pub name: Ident,
    pub ret_type: Option<Type>,
    pub args: Vec<(Ident, Type)>,
    pub body: Vec<TopExpr>,
}

pub trait TryParse {
    fn try_parse<'a>(_pairs: &'a [Pair<'a>]) -> ParseResult<Self>
    where
        Self: Sized,
    {
        Err(ParseError::NotImplementedYet)
    }
}

pub fn try_parse<'a, T>(pairs: &'a [Pair<'a>]) -> ParseResult<'a, T>
where
    T: TryParse,
{
    T::try_parse(pairs)
}

impl<T> TryParse for Box<T>
where
    T: TryParse,
{
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (res, pairs) = try_parse(pairs)?;
        Ok((Box::new(res), pairs))
    }
}

impl TryParse for Program {
    fn try_parse<'a>(mut pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let mut functions = vec![];
        loop {
            pairs = ignore_newlines(pairs);
            if pairs.is_empty() {
                return Ok((Self { functions }, &[]));
            }
            let (fun, p) = try_parse(pairs)?;
            pairs = p;
            functions.push(fun);
        }
    }
}

impl TryParse for Type {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (ident, pairs) = try_parse(pairs)?;

        if expect_symbol(pairs, '<').is_ok() {
            let (types, pairs) =
                expect_sequence(pairs, '<'.into(), '>'.into(), ','.into(), Type::try_parse)?;
            let ty = Self::Generic(ident, types);
            Ok((ty, pairs))
        } else {
            let ty = Self::Simple(ident);
            Ok((ty, pairs))
        }
    }
}

impl TryParse for Fun {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::Fun)?;
        let (name, pairs) = try_parse(pairs)?;

        let (args, pairs) = expect_sequence(pairs, '('.into(), ')'.into(), ','.into(), |pairs| {
            let (arg_name, pairs) = try_parse(pairs)?;
            let pairs = expect_symbol(pairs, ':')?;
            let (arg_type, pairs) = try_parse(pairs)?;
            Ok(((arg_name, arg_type), pairs))
        })?;

        let (ret_type, pairs) = match expect_symbol(pairs, ':') {
            Ok(pairs) => {
                let (ty, pairs) = try_parse(pairs)?;
                (Some(ty), pairs)
            }
            Err(_) => (None, pairs),
        };

        let (body, pairs) = expect_body(pairs)?;

        let fun = Fun {
            name,
            ret_type,
            args,
            body,
        };

        Ok((fun, pairs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::test_helpers::*;

    #[test]
    fn ident() {
        let ident: Ident = make("hello");
        assert_eq!(ident.0, "hello")
    }

    #[test]
    fn type_simple() {
        let ty: Type = make("Int");
        assert_eq!(ty, Type::Simple(make("Int")));
    }

    #[test]
    fn type_generic() {
        let ty: Type = make("Array<Int>");
        assert_eq!(ty, Type::Generic(make("Array"), vec![make("Int")]));
    }

    #[test]
    fn fun() {
        let fun: Fun = make("fun test(array: Array<Int>): Int {}");

        assert_eq!(&fun.name.0, "test");
        assert_eq!(fun.ret_type.unwrap(), make("Int"));
        assert_eq!(fun.args[..], [(Ident("array".into()), make("Array<Int>"))]);
        assert!(fun.body.is_empty())
    }

    #[test]
    fn program() {
        make::<Program>(include_str!("../samples/hello.kt"));
        make::<Program>(include_str!("../samples/arrays.kt"));
        dbg!(make::<Program>(include_str!("../samples/factorial.kt")));
    }
}
