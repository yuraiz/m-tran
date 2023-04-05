use super::Eval;
use crate::{
    interpreter::{Context, Object},
    parser::expr::{self, ControlExpr},
};

impl Eval for ControlExpr {
    fn eval(&self, context: &Context) -> Object {
        match self {
            ControlExpr::If(expr) => expr.eval(context),
            ControlExpr::For(expr) => expr.eval(context),
            ControlExpr::While(expr) => expr.eval(context),
            ControlExpr::Return(expr) => {
                if let Some(e) = &expr.0 {
                    context.ret_item.replace(Some(e.eval(context)));
                } else {
                    context.ret_item.replace(Some(Object::Unit));
                }
                Object::Unit
            }
        }
    }
}

impl Eval for expr::If {
    fn eval(&self, context: &Context) -> Object {
        let bool = match self.expr.eval(context) {
            Object::Boolean(bool) => bool,
            _ => unreachable!(),
        };

        context.push();
        if bool {
            for expr in &self.body {
                expr.eval(context);
            }
        } else {
            for expr in &self.else_branch {
                expr.eval(context);
            }
        }
        context.pop();

        Object::Unit
    }
}

impl Eval for expr::For {
    fn eval(&self, context: &Context) -> Object {
        let name = &self.var.0;
        let iterable = self.iterable.eval(context);

        context.push();

        match iterable {
            Object::String(string) => {
                for c in string.chars() {
                    context.var(name, Object::Char(c));
                    for expr in &self.body {
                        expr.eval(context);
                    }
                }
            }
            Object::Array(arr) => {
                for obj in arr {
                    context.var(name, obj);
                    for expr in &self.body {
                        expr.eval(context);
                    }
                }
            }
            Object::Range(l, r) => match (*l, *r) {
                (Object::Int(l), Object::Int(r)) => {
                    for i in l..=r {
                        context.var(name, Object::Int(i));
                        for expr in &self.body {
                            expr.eval(context);
                        }
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        context.pop();
        Object::Unit
    }
}

impl Eval for expr::While {
    fn eval(&self, context: &Context) -> Object {
        while match self.expr.eval(context) {
            Object::Boolean(bool) => bool,
            _ => unreachable!(),
        } {
            context.push();
            for expr in &self.body {
                expr.eval(context);
            }
            context.pop();
        }
        Object::Unit
    }
}
