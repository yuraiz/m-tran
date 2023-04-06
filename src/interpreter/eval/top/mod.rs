mod control;

use super::Eval;
use crate::{
    interpreter::{Context, Object},
    parser::expr::{self, TopExpr},
};

impl Eval for TopExpr {
    fn eval(&self, context: &Context) -> Object {
        match self {
            TopExpr::ControlExpr(control_expr) => control_expr.eval(context),
            TopExpr::Binding(binding) => binding.eval(context),
            TopExpr::Set(set) => set.eval(context),
            TopExpr::Call(expr) => {
                let name = &expr.name.0;
                let args = expr.args.iter().map(|e| e.eval(context)).collect();
                context.call_function(name, args)
            }
            TopExpr::SetByIndex(set_by_index) => set_by_index.eval(context),
        }
    }
}

impl Eval for expr::Binding {
    fn eval(&self, context: &Context) -> Object {
        let ref set = self.set;
        let obj = set.expr.eval(context);
        context.var(&set.name.0, obj);
        Object::Unit
    }
}

impl Eval for expr::Set {
    fn eval(&self, context: &Context) -> Object {
        let obj = self.expr.eval(context);
        context.set(&self.name.0, obj);
        Object::Unit
    }
}

impl Eval for expr::SetByIndex {
    fn eval(&self, context: &Context) -> Object {
        let name = &self.name.0;
        let value = self.expr.eval(context);
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
                    *entry = value;
                } else {
                    context.exception("Index out of range".to_owned())
                }
            }
            _ => unreachable!(),
        }

        context.set(name, obj);

        Object::Unit
    }
}