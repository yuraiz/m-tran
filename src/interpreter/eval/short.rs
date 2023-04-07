use super::Eval;
use crate::{
    interpreter::{Context, Object},
    parser::expr::{self, GetByIndex, ShortExpr},
};

impl Eval for ShortExpr {
    fn eval(&self, context: &Context) -> Object {
        match self {
            ShortExpr::Ident(ident) => context.get(&ident.0),
            ShortExpr::GetByIndex(get_by_index) => get_by_index.eval(context),
            ShortExpr::Literal(literal) => match literal {
                expr::Literal::Int(i) => Object::Int(*i),
                expr::Literal::Bool(b) => Object::Boolean(*b),
                expr::Literal::Char(c) => Object::Char(*c),
                expr::Literal::String(s) => Object::String(s.clone()),
            },
        }
    }
}

impl Eval for GetByIndex {
    fn eval(&self, context: &Context) -> Object {
        let name = &self.ident.0;
        let Object::Int(index) = self.index.eval(context) else {
            unreachable!()
        };

        if index < 0 {
            context.exception("Index out of range".to_owned());
        }

        let mut obj = context.get(name);

        match obj {
            Object::Array(ref mut arr) => {
                if let Some(entry) = arr.get_mut(index as usize) {
                    entry.clone()
                } else {
                    context.exception("Index out of range".to_owned());
                    Object::Unit
                }
            }
            _ => unreachable!(),
        }
    }
}
