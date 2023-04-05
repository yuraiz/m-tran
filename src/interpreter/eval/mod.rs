mod comparison;
mod math;
mod short;
mod top;

use crate::parser::expr::Expr;
use crate::parser::Spanned;

use super::Context;
use super::Object;

pub trait Eval {
    fn eval(&self, context: &Context) -> Object;
}

impl Eval for Expr {
    fn eval(&self, context: &Context) -> Object {
        match self {
            Expr::TopExpr(top_expr) => top_expr.eval(context),
            Expr::MathExpr(math_expr) => math_expr.eval(context),
            Expr::ComparisonExpr(comparison) => comparison.eval(context),
            Expr::ShortExpr(short_expr) => short_expr.eval(context),
        }
    }
}

impl<E> Eval for Spanned<E>
where
    E: Eval,
{
    fn eval(&self, context: &Context) -> Object {
        self.expr.eval(context)
    }
}
