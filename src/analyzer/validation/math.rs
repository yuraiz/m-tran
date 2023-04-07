use super::*;

impl Validate for expr::MathExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        match self {
            MathExpr::Neg(expr) => {
                let ty = expr.0.validate(context)?;
                if ty != ExprType::Primitive(Primitive::Int) {
                    context.error("negation only applicable to Int type".to_string());
                    None
                } else {
                    Some(ty)
                }
            }
            MathExpr::BoolNeg(expr) => {
                let ty = expr.0.validate(context)?;
                if ty != ExprType::Primitive(Primitive::Boolean) {
                    context.error("boolean negation only applicable to Boolean type".to_string());
                    None
                } else {
                    Some(ty)
                }
            }
            MathExpr::Parens(expr) => expr.0.validate(context),
            MathExpr::Range(expr) => expr.validate(context),
            MathExpr::Sub(expr) => {
                ensure_type_equality(expr.left.as_ref(), expr.right.as_ref(), context)
            }
            MathExpr::Mul(expr) => {
                ensure_type_equality(expr.left.as_ref(), expr.right.as_ref(), context)
            }
            MathExpr::Div(expr) => {
                ensure_type_equality(expr.left.as_ref(), expr.right.as_ref(), context)
            }
            MathExpr::Add(expr) => expr.validate(context),
        }
    }
}

impl Validate for expr::Range {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let ty = ensure_type_equality(self.left.as_ref(), self.right.as_ref(), context)?;
        Some(ExprType::Range(Box::new(ty)))
    }
}

impl Validate for expr::Add {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let left = self.left.validate(context)?;
        let right = self.right.validate(context)?;

        if [&left, &right].contains(&&ExprType::Unit) {
            context.error("it isn't possible to add items of unit type".to_owned());
            None
        } else if [&left, &right].contains(&&ExprType::Primitive(Primitive::String)) {
            Some(ExprType::Primitive(Primitive::String))
        } else {
            ensure_type_equality(self.left.as_ref(), self.right.as_ref(), context)
        }
    }
}
