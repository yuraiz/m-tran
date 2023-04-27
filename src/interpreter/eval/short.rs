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

        let obj = context.get(name);

        match obj {
            Object::Array(arr) => arr
                .borrow()
                .get(index as usize)
                .unwrap_or_else(|| context.exception("Index out of range".to_owned()))
                .to_owned(),
            _ => unreachable!(),
        }
    }
}
