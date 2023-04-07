use super::*;

impl Validate for expr::ControlExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        use expr::ControlExpr::*;
        let expr: &dyn Validate = match self {
            If(expr) => expr,
            For(expr) => expr,
            While(expr) => expr,
            Return(expr) => expr,
        };
        expr.validate(context)
    }
}

impl Validate for expr::If {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let is_bool = self.expr.validate(context)? == ExprType::Primitive(Primitive::Boolean);
        if is_bool {
            context.push_scope();
            for expr in &self.body {
                expr.validate(context)?;
            }
            context.pop_scope();
            context.push_scope();
            for expr in &self.else_branch {
                expr.validate(context)?;
            }
            context.pop_scope();
            Some(ExprType::Unit)
        } else {
            context.error("condition must have boolean type".to_owned());
            None
        }
    }
}

impl Validate for expr::For {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let iter_type = self.iterable.validate(context)?;

        let ty = match iter_type {
            ExprType::Array(ty) => ty,
            ExprType::Range(ty) => ty,
            _ => {
                context.error("only array and range are iterable types".to_owned());
                return None;
            }
        };

        context.push_scope();

        context.add_var_type(self.var.0.to_owned(), *ty);
        for expr in &self.body {
            expr.validate(context)?;
        }
        context.pop_scope();
        Some(ExprType::Unit)
    }
}

impl Validate for expr::While {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let is_bool = self.expr.validate(context)? == ExprType::Primitive(Primitive::Boolean);
        if is_bool {
            context.push_scope();
            for expr in &self.body {
                expr.validate(context)?;
            }
            context.pop_scope();
            Some(ExprType::Unit)
        } else {
            context.error("condition must have boolean type".to_owned());
            None
        }
    }
}

impl Validate for expr::Return {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let expected = context.current_ret_type.clone().unwrap();

        if let Some(ref expr) = self.0 {
            let actual = expr.validate(context)?;

            if actual != expected {
                context.error(format!("wrong return type"));
            }
        } else {
            if expected != ExprType::Unit {
                context.error(format!("wrong return type"));
            }
        }
        Some(ExprType::Unit)
    }
}
