mod validation;
use std::collections::HashMap;

use validation::*;

use crate::lexer::Span;
use crate::parser::Program;

#[derive(Debug, Default)]
pub struct Context<'a> {
    current_ret_type: Option<ExprType>,
    last_span: Option<Span>,
    functions: HashMap<&'a str, FunType>,
    scopes: Vec<HashMap<String, ExprType>>,
    errors: Vec<(Span, String)>,
}

impl<'a> Context<'a> {
    fn get_functions(&'_ mut self, prog: &'a Program) {
        for fun in &prog.functions {
            let name = fun.name.0.as_str();
            let ty = FunType::from(fun);

            if self.functions.get(&name).is_some() {
                self.error(format!("function {name} already defined"));
            } else {
                self.functions.insert(name, ty);
            }
        }
    }

    fn validate_functions(&'_ mut self, prog: &'a Program) {
        for fun in &prog.functions {
            self.current_ret_type = if let Some(ref ty) = fun.ret_type {
                Some(ExprType::from(ty))
            } else {
                Some(ExprType::Unit)
            };

            self.push_scope();
            for expr in &fun.body {
                expr.validate(self);
            }
            self.pop_scope();
        }
    }

    fn find_predefined_fun_ret_type(&mut self, ident: &str, args: &[ExprType]) -> Option<ExprType> {
        match ident {
            "print" | "println" => Some(ExprType::Primitive(Primitive::String)),
            "arrayOf" => {
                if let Some(first) = args.first() {
                    if args.iter().all(|t| t == first) {
                        Some(ExprType::Array(first.to_owned().into()))
                    } else {
                        self.error("arrayOf arguments must have the same type".to_string());
                        None
                    }
                } else {
                    self.error("arrayOf must have at least one argument".to_string());
                    None
                }
            }
            "readln" => Some(ExprType::Primitive(Primitive::String)),
            "readlnInt" => Some(ExprType::Primitive(Primitive::Int)),
            "readlnBoolean" => Some(ExprType::Primitive(Primitive::Boolean)),
            _ => None,
        }
    }

    fn find_fun_ret_type(&mut self, ident: &str, args: &[ExprType]) -> Option<ExprType> {
        if let Some(ty) = self.find_predefined_fun_ret_type(ident, args) {
            Some(ty)
        } else {
            if let Some(ty) = self.functions.get(ident) {
                if ty.args == args {
                    Some(ty.ret_type.clone())
                } else {
                    self.error(format!(
                        "function with name {ident} found but it's arguments wrong"
                    ));
                    None
                }
            } else {
                self.error(format!("function with name {ident} not found"));
                None
            }
        }
    }

    fn find_var_type(&self, ident: &str) -> Option<ExprType> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(ident) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn add_var_type(&mut self, ident: String, ty: ExprType) {
        let scope = self.scopes.last_mut().expect("scope exists");
        if scope.get(&ident).is_some() {
            self.error(format!("binding {ident} already defined"));
        } else {
            scope.insert(ident, ty);
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(Default::default());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn error(&mut self, err: String) {
        let span = self.last_span.unwrap_or(Span { lo: 0, hi: 0 });
        self.errors.push((span, err))
    }
}

fn check_program(prog: &Program) -> Vec<(Span, String)> {
    let mut context = Context::default();

    context.functions.insert(
        "println",
        FunType {
            args: vec![ExprType::Primitive(Primitive::String)],
            ret_type: ExprType::Unit,
        },
    );

    context.functions.insert(
        "print",
        FunType {
            args: vec![ExprType::Primitive(Primitive::String)],
            ret_type: ExprType::Unit,
        },
    );

    context.functions.insert(
        "arrayOf",
        FunType {
            args: vec![ExprType::Primitive(Primitive::String)],
            ret_type: ExprType::Unit,
        },
    );

    context.get_functions(prog);

    context.validate_functions(prog);

    context.errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;
    use crate::parser::*;

    #[test]
    fn validation() {
        fn validate(source: &str) {
            let pairs: Vec<_> = Lexer::new(source).collect();
            let (prog, _) = Program::try_parse(&pairs).unwrap();
            assert!(check_program(&prog).is_empty());
        }

        validate(include_str!("../samples/arrays.kt"));
        validate(include_str!("../samples/hello.kt"));
        validate(include_str!("../samples/factorial.kt"));
    }
}
