mod expr;
mod helpers;
mod parse_error;

use crate::lexer::*;

use self::helpers::*;
use expr::*;
pub use parse_error::{ParseError, ParseResult};

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Expr>,
}

// #[derive(Debug)]
// pub struct Expr {
//     pub span: Span,
//     pub node: ExprNode,
// }

// impl TryParse for Expr {}

// impl PartialEq for Expr {
//     fn eq(&self, other: &Self) -> bool {
//         self.node == other.node
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum ExprNode {
//     Fun(Fun),
// }

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
    pub body: Vec<Expr>,
}

pub trait TryParse {
    fn try_parse<'a>(_pairs: &'a [Pair<'a>]) -> ParseResult<Self>
    where
        Self: Sized,
    {
        Err(ParseError::NotImplementedYet)
    }
}

impl TryParse for Type {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (ident, pairs) = Ident::try_parse(pairs)?;

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
        let (name, pairs) = Ident::try_parse(pairs)?;

        let (args, pairs) = expect_sequence(pairs, '('.into(), ')'.into(), ','.into(), |pairs| {
            let (arg_name, pairs) = Ident::try_parse(pairs)?;
            let pairs = expect_symbol(pairs, ':')?;
            let (arg_type, pairs) = Type::try_parse(pairs)?;
            Ok(((arg_name, arg_type), pairs))
        })?;

        let (ret_type, pairs) = match expect_symbol(pairs, ':') {
            Ok(pairs) => {
                let (ty, pairs) = Type::try_parse(pairs)?;
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
}
