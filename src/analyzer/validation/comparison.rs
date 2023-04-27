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
            ComparisonExpr::And(expr) => {
                ensure_type_equality(expr.left.as_ref(), expr.right.as_ref(), context)
            }
            ComparisonExpr::Or(expr) => {
                ensure_type_equality(expr.left.as_ref(), expr.right.as_ref(), context)
            }
        };

        if matches!(self, ComparisonExpr::And(_) | ComparisonExpr::Or(_)) {
            let boolean = ExprType::Primitive(Primitive::Boolean);
            return if ty? != boolean {
                context.error("boolean operators only applicable to booleans".to_string());
                None
            } else {
                Some(boolean)
            };
        }

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
