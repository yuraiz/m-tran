use super::*;

impl Validate for expr::ComparisonExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let ty = match self {
            ComparisonExpr::LessThan(expr) => {
                ensure_type_equality(expr.left.as_ref(), expr.right.as_ref(), context)
            }
            ComparisonExpr::MoreThan(expr) => {
                ensure_type_equality(expr.left.as_ref(), expr.right.as_ref(), context)
            }
        };
        // Arrays and ranges aren't comparable
        match ty {
            Some(ExprType::Primitive(primitive)) => match primitive {
                Primitive::Int | Primitive::String | Primitive::Char => {
                    Some(ExprType::Primitive(Primitive::Boolean))
                }
                Primitive::Boolean => {
                    context.error("can't compare booleans".to_string());
                    None
                }
            },
            _ => {
                context.error("can't compare types".to_string());
                None
            }
        }
    }
}
