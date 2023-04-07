use super::Eval;
use crate::{
    interpreter::{Context, Object},
    parser::expr::ComparisonExpr,
};

impl Eval for ComparisonExpr {
    fn eval(&self, context: &Context) -> Object {
        let (l, r) = match self {
            ComparisonExpr::LessThan(expr) => (expr.left.eval(context), expr.right.eval(context)),
            ComparisonExpr::MoreThan(expr) => (expr.left.eval(context), expr.right.eval(context)),
        };

        use Object::*;

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
