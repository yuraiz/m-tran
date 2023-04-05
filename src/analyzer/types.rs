use crate::parser::expr::{self, ComparisonExpr, MathExpr};
use crate::parser::{Fun, Type};

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

impl Validate for expr::Binding {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let ident = self.set.name.0.clone();
        let ty = self.set.expr.validate(context)?;
        context.add_var_type(ident, ty.clone());
        Some(ty)
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
            _ => return None,
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
            None
        }
    }
}

impl Validate for expr::Return {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        if let Some(ref expr) = self.0 {
            expr.validate(context)?;
        }
        Some(ExprType::Unit)
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

impl Validate for expr::TopExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        match self {
            expr::TopExpr::Call(expr) => expr.validate(context),
            expr::TopExpr::Binding(expr) => expr.validate(context),

            _ => Some(ExprType::Unit),
        }
    }
}

impl Validate for expr::Call {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        if let Some(ty) = context.find_fun_type(&self.name.0) {
            Some(ty.ret_type)
        } else {
            None
        }
    }
}

impl Validate for expr::MathExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        match self {
            MathExpr::Neg(expr) => expr.0.validate(context),
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
            MathExpr::Add(_) => todo!(),
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
            None
        } else if [&left, &right].contains(&&ExprType::Primitive(Primitive::String)) {
            Some(ExprType::Primitive(Primitive::String))
        } else {
            ensure_type_equality(self.left.as_ref(), self.right.as_ref(), context)
        }
    }
}

fn ensure_type_equality<L, R>(left: &L, right: &R, context: &mut Context) -> Option<ExprType>
where
    L: Validate,
    R: Validate,
{
    let l = left.validate(context)?;
    let r = right.validate(context)?;
    if l == r {
        Some(r)
    } else {
        None
    }
}

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
        // Arrays aren't comparable
        match ty {
            Some(ExprType::Primitive(_)) => ty,
            _ => None,
        }
    }
}

impl Validate for expr::ShortExpr {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        match self {
            expr::ShortExpr::Ident(expr::Ident(name)) => context.find_var_type(&name),
            expr::ShortExpr::Literal(literal) => literal.validate(context),
        }
    }
}
