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
        context.last_span = Some(self.span);
        self.expr.validate(context)
    }
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

impl Validate for expr::Set {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let ident = self.name.0.clone();
        let ty = self.expr.validate(context)?;
        if let Some(expected) = context.find_var_type(&ident) {
            if expected != ty {
                context.error(format!("variable {ident} found but it has different type"));
                None
            } else {
                Some(ExprType::Unit)
            }
        } else {
            context.error(format!("variable {ident} not found in scope"));
            None
        }
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
            context.error("condition must have boolean type".to_owned());
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
            _ => {
                context.error("only array and range are iterable types".to_owned());
                return None;
            }
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
            context.error("condition must have boolean type".to_owned());
            None
        }
    }
}

impl Validate for expr::Return {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let expected = context.current_ret_type.clone().unwrap();

        if let Some(ref expr) = self.0 {
            let actual = expr.validate(context)?;

            if actual != expected {
                context.error(format!("wrong return type"));
            }
        } else {
            if expected != ExprType::Unit {
                context.error(format!("wrong return type"));
            }
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
        use expr::TopExpr::*;
        let expr: &dyn Validate = match self {
            Call(expr) => expr,
            Binding(expr) => expr,
            Set(expr) => expr,
            ControlExpr(expr) => expr,
            _ => panic!(),
        };
        expr.validate(context)
    }
}

impl Validate for expr::Call {
    fn validate(&self, context: &mut Context) -> Option<ExprType> {
        let args: Vec<_> = self
            .args
            .iter()
            .filter_map(|arg| arg.validate(context))
            .collect();

        if args.len() == self.args.len() {
            if let Some(ret_type) = context.find_fun_ret_type(&self.name.0, &args) {
                Some(ret_type)
            } else {
                None
            }
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
        context.error("wrong operands".to_owned());
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
        // Arrays and ranges aren't comparable
        match ty {
            Some(ExprType::Primitive(_)) => ty,
            _ => {
                context.error("can't compare types".to_string());
                None
            }
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
