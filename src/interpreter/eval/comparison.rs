use super::Eval;
use crate::{
    interpreter::{Context, Object},
    parser::expr::ComparisonExpr,
};

impl Eval for ComparisonExpr {
    fn eval(&self, context: &Context) -> Object {
        use ComparisonExpr::*;
        use Object::*;

        let (l, r) = match self {
            LessThan(expr) => (expr.left.eval(context), expr.right.eval(context)),
            MoreThan(expr) => (expr.left.eval(context), expr.right.eval(context)),
            And(expr) => {
                return if let Boolean(true) = expr.left.eval(context) {
                    return expr.right.eval(context);
                } else {
                    Boolean(false)
                };
            }
            Or(expr) => {
                return if let Boolean(false) = expr.left.eval(context) {
                    return expr.right.eval(context);
                } else {
                    Boolean(true)
                };
            }
        };

        let cmp = match (l, r) {
            (Int(l), Int(r)) => l.cmp(&r),
            (String(l), String(r)) => l.cmp(&r),
            (Boolean(l), Boolean(r)) => l.cmp(&r),
            (Char(l), Char(r)) => l.cmp(&r),
            _ => unreachable!(),
        };

        Boolean(matches!(
            (cmp, self),
            (std::cmp::Ordering::Less, ComparisonExpr::LessThan(_))
                | (std::cmp::Ordering::Greater, ComparisonExpr::MoreThan(_))
        ))
    }
}
