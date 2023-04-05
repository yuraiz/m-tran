use super::Eval;
use crate::{
    interpreter::{Context, Object},
    parser::expr::{self, ShortExpr},
};

impl Eval for ShortExpr {
    fn eval(&self, context: &Context) -> Object {
        match self {
            ShortExpr::Ident(ident) => context.get(&ident.0),
            ShortExpr::Literal(literal) => match literal {
                expr::Literal::Int(i) => Object::Int(*i),
                expr::Literal::Bool(b) => Object::Boolean(*b),
                expr::Literal::Char(c) => Object::Char(*c),
                expr::Literal::String(s) => Object::String(s.clone()),
            },
        }
    }
}
