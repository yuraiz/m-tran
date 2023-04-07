mod control;

use super::*;

impl Validate for expr::TopExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        use expr::TopExpr::*;
        let expr: &dyn Validate = match self {
            Call(expr) => expr,
            Binding(expr) => expr,
            Set(expr) => expr,
            ControlExpr(expr) => expr,
            SetByIndex(expr) => expr,
        };
        expr.validate(context)
    }
}

impl Validate for expr::Call {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let args: Vec<_> = self
            .args
            .iter()
            .filter_map(|arg| arg.validate(context))
            .collect();

        if args.len() == self.args.len() {
            if let Some(ret_type) = context.find_fun_ret_type(&self.name, Some(&args)) {
                Some(ret_type)
            } else {
                None
            }
        } else {
            if let Some(ret_type) = context.find_fun_ret_type(&self.name, None) {
                Some(ret_type)
            } else {
                None
            }
        }
    }
}

impl Validate for expr::Binding {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let ident = self.set.name.0.clone();
        let ty = self.set.expr.validate(context)?;
        context.add_var_type(ident, ty.clone());
        Some(ty)
    }
}

impl Validate for expr::Set {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let ident = self.name.0.clone();
        let ty = self.expr.validate(context);
        if let Some(expected) = context.find_var_type(&ident) {
            if expected != ty? {
                context.error(format!("variable {ident} found but it has different type"));
                None
            } else {
                Some(ExprType::Unit)
            }
        } else {
            context.error_with_span(
                format!("variable {ident} not found in scope"),
                self.name.span,
            );
            None
        }
    }
}

impl Validate for expr::SetByIndex {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        ensure_type_equality(&self.get_by_index, &*self.expr, context)
    }
}
