use super::Eval;
use crate::{
    interpreter::{Context, Object},
    parser::expr::{self, MathExpr},
};

impl Eval for MathExpr {
    fn eval(&self, context: &Context) -> Object {
        use Object::*;

        match self {
            MathExpr::Neg(expr) => match expr.0.eval(context) {
                Object::Int(num) => Object::Int(num.wrapping_neg()),
                _ => unreachable!(),
            },
            MathExpr::Range(expr) => expr.eval(context),
            MathExpr::Sub(expr) => {
                let l = expr.left.eval(context);
                let r = expr.right.eval(context);

                match (l, r) {
                    (Int(l), Int(r)) => Int(l.wrapping_sub(r)),
                    _ => unreachable!(),
                }
            }
            MathExpr::Mul(expr) => {
                let l = expr.left.eval(context);
                let r = expr.right.eval(context);

                match (l, r) {
                    (Int(l), Int(r)) => Int(l.wrapping_mul(r)),
                    _ => unreachable!(),
                }
            }
            MathExpr::Div(expr) => {
                let l = expr.left.eval(context);
                let r = expr.right.eval(context);

                match (l, r) {
                    (Int(l), Int(r)) => Int(l.wrapping_div(r)),
                    _ => unreachable!(),
                }
            }
            MathExpr::Parens(expr) => expr.0.eval(context),
            MathExpr::Add(expr) => expr.eval(context),
        }
    }
}

impl Eval for expr::Range {
    fn eval(&self, context: &Context) -> Object {
        let l = self.left.eval(context);
        let r = self.right.eval(context);
        Object::Range(l.into(), r.into())
    }
}

impl Eval for expr::Add {
    fn eval(&self, context: &Context) -> Object {
        use Object::*;
        let l = self.left.eval(context);
        let r = self.right.eval(context);

        match (l, r) {
            (Int(l), Int(r)) => Int(l.wrapping_add(r)),
            (String(string), other) => {
                let mut string = string.to_owned();
                string.push_str(&other.to_string());
                String(string)
            }
            (other, String(s)) => {
                let mut string = other.to_string();
                string.push_str(&s);
                String(string)
            }
            _ => unreachable!(),
        }
    }
}
