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
            expr::ShortExpr::Ident(ident) => ident.validate(context),
            expr::ShortExpr::GetByIndex(get_by_index) => get_by_index.validate(context),
            expr::ShortExpr::Literal(literal) => literal.validate(context),
        }
    }
}

impl Validate for expr::Ident {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let name = &self.0;
        if let Some(ty) = context.find_var_type(&name) {
            Some(ty)
        } else {
            context.error(format!("ident {name} not found"));
            None
        }
    }
}

impl Validate for expr::GetByIndex {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let ident = self.ident.validate(context);
        let index = self.index.validate(context);

        match index {
            Some(ExprType::Primitive(Primitive::Int)) | None => {}
            _ => {
                context.error_with_span("only Int index is supported".to_string(), self.index.span)
            }
        };

        match ident? {
            ExprType::Array(ty) => Some(*ty),
            ExprType::Primitive(Primitive::String) => Some(ExprType::Primitive(Primitive::Char)),
            _ => {
                context.error("index operator is only supported by arrays and strings".to_string());
                None
            }
        }
    }
}
