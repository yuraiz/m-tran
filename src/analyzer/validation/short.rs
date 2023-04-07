use super::*;

impl Validate for expr::Literal {
    fn validate(&self, _context: &mut Context) -> Option<ExprType> {
        use expr::Literal::*;
        let primitive = match self {
            Int(_) => Primitive::Int,
            Bool(_) => Primitive::Boolean,
            Char(_) => Primitive::Char,
            String(_) => Primitive::String,
        };
        Some(ExprType::Primitive(primitive))
    }
}

impl Validate for expr::ShortExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        match self {
            expr::ShortExpr::Ident(expr::Ident(name)) => {
                if let Some(ty) = context.find_var_type(&name) {
                    Some(ty)
                } else {
                    context.error(format!("ident {name} not found"));
                    None
                }
            }
            expr::ShortExpr::Literal(literal) => literal.validate(context),
        }
    }
}
