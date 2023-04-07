mod validation;
use std::collections::HashMap;

use validation::*;

use crate::lexer::Span;
use crate::parser::expr::Ident;
use crate::parser::{Program, Spanned};

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

            let args = fun
                .args
                .iter()
                .map(|arg| {
                    let name = (arg.0).0.clone();
                    let ty = ExprType::from(&arg.1);
                    (name, ty)
                })
                .collect();

            self.scopes.push(args);

            for expr in &fun.body {
                expr.validate(self);
            }
            self.pop_scope();
        }
    }

    fn check_main(&mut self) {
        if let Some(fun) = self.functions.get("main") {
            if !fun.args.is_empty() {
                self.error_with_span(
                    "function main must accept no arguments".to_string(),
                    Span { lo: 0, hi: 0 },
                );
            }
        } else {
            self.error("function main not found".to_string());
        }
    }

    fn find_predefined_fun_ret_type(
        &mut self,
        ident: &Spanned<Ident>,
        args: Option<&[ExprType]>,
    ) -> Option<ExprType> {
        match ident.0.as_str() {
            "print" | "println" => Some(ExprType::Primitive(Primitive::String)),
            "arrayOf" => {
                if let Some(args) = args {
                    if let Some(first) = args.first() {
                        if args.iter().any(|t| t != first) {
                            self.error("arrayOf arguments must have the same type".to_string());
                        }
                        Some(ExprType::Array(first.to_owned().into()))
                    } else {
                        self.error("arrayOf must have at least one argument".to_string());
                        None
                    }
                } else {
                    None
                }
            }
            other => {
                let arg_count = args.map(|a| a.len()).unwrap_or(1);
                match other {
                    "readln" | "readlnInt" | "readlnBoolean" => {
                        if arg_count != 0 {
                            self.error_with_span(
                                format!("{other} accepts no arguments"),
                                ident.span,
                            );
                        }
                        match other {
                            "readln" => Some(ExprType::Primitive(Primitive::String)),
                            "readlnInt" => Some(ExprType::Primitive(Primitive::Int)),
                            "readlnBoolean" => Some(ExprType::Primitive(Primitive::Boolean)),
                            _ => unreachable!(),
                        }
                    }
                    _ => None,
                }
            }
        }
    }

    fn find_fun_ret_type(
        &mut self,
        ident: &Spanned<Ident>,
        args: Option<&[ExprType]>,
    ) -> Option<ExprType> {
        let name = ident.0.as_str();

        if let Some(ty) = self.find_predefined_fun_ret_type(ident, args) {
            Some(ty)
        } else if let Some(ty) = self.functions.get(name) {
            if ty.args == args? {
                Some(ty.ret_type.clone())
            } else {
                self.error(format!(
                    "function with name {name} found but it's arguments wrong"
                ));
                None
            }
        } else {
            self.error_with_span(format!("function with name {name} not found"), ident.span);
            None
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

    fn error_with_span(&mut self, err: String, span: Span) {
        self.errors.push((span, err));
    }

    fn error(&mut self, err: String) {
        let span = self.last_span.unwrap_or(Span { lo: 0, hi: 0 });
        self.error_with_span(err, span);
    }
}

pub fn check_program(prog: &Program) -> Vec<(Span, String)> {
    let mut context = Context::default();

    context.get_functions(prog);

    context.check_main();

    context.validate_functions(prog);

    context.errors
}

pub fn pretty_print_error(source: &str, span: Span, message: &str) {
    let yellow = "\x1b[93m";
    let white = "\x1b[0m";

    let Span { lo, hi } = span;

    if lo == 0 && hi == 0 {
        eprintln!("  {yellow}{message}{white}");
        return;
    }

    let (before, error) = source.split_at(lo);
    let (error, after) = error.split_at(hi - lo);

    let line_num = before.chars().filter(|&c| c == '\n').count() + 1;
    let before = before.split('\n').last().unwrap_or_default();
    let after = after.split('\n').next().unwrap_or_default();

    let line_count = source.chars().filter(|&c| c == '\n').count() + 1;
    let indent = line_count.to_string().len() + 1;

    let e = "";

    eprintln!("{e:indent$} |");

    eprintln!("{line_num:>indent$} | {before}{error}{after}");

    eprintln!(
        "{e:indent$} | {e:s$}{yellow}{e:^>w$} {message}{white}",
        s = before.chars().count(),
        w = error.chars().count(),
    );
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
