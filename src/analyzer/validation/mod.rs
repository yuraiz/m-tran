mod comparison;
mod math;
mod short;
mod top;

use crate::parser::expr::{self, ComparisonExpr, MathExpr};
use crate::parser::{Fun, Spanned, Type};

use super::Context;

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Int,
    String,
    Boolean,
    Char,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprType {
    Primitive(Primitive),
    Array(Box<ExprType>),
    Range(Box<ExprType>),
    Unit,
}

impl From<&Type> for ExprType {
    fn from(value: &Type) -> Self {
        match value {
            Type::Simple(value) => {
                use Primitive::*;
                let primitive = match value.0.as_str() {
                    "Int" => Int,
                    "String" => String,
                    "Boolean" => Boolean,
                    _ => return ExprType::Unit,
                };
                Self::Primitive(primitive)
            }
            Type::Generic(ty, params) => {
                if params.len() != 1 || ty.0 != "Array" {
                    Self::Unit
                } else {
                    let param = ExprType::from(&params[0]);
                    if param != ExprType::Unit {
                        ExprType::Array(Box::new(param))
                    } else {
                        ExprType::Unit
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunType {
    pub args: Vec<ExprType>,
    pub ret_type: ExprType,
}

impl From<&Fun> for FunType {
    fn from(value: &Fun) -> Self {
        let ret_type = if let Some(ref ty) = value.ret_type {
            ty.into()
        } else {
            ExprType::Unit
        };
        Self {
            args: value.args.iter().map(|(_, ty)| ty.into()).collect(),
            ret_type,
        }
    }
}

pub trait Validate {
    fn validate(&self, _context: &mut Context) -> Option<ExprType>;
}

impl<E> Validate for Spanned<E>
where
    E: Validate,
{
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let span = context.last_span.replace(self.span);
        let ty = self.expr.validate(context);
        context.last_span = span;
        ty
    }
}

impl Validate for expr::Expr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        match self {
            expr::Expr::TopExpr(expr) => expr.validate(context),
            expr::Expr::MathExpr(expr) => expr.validate(context),
            expr::Expr::ComparisonExpr(expr) => expr.validate(context),
            expr::Expr::ShortExpr(expr) => expr.validate(context),
        }
    }
}

fn ensure_type_equality<L, R>(left: &L, right: &R, context: &mut Context) -> Option<ExprType>
where
    L: Validate,
    R: Validate,
{
    let l = left.validate(context);
    let r = right.validate(context)?;
    let l = l?;
    if l == r {
        Some(r)
    } else {
        context.error("wrong operands".to_owned());
        None
    }
}
