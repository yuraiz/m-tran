mod types;
use std::collections::HashMap;

use types::*;

use crate::parser::Program;

#[derive(Debug, Default)]
pub struct Context<'a> {
    current_ret_type: Option<ExprType>,
    functions: HashMap<&'a str, FunType>,
    scopes: Vec<HashMap<String, ExprType>>,
    errors: Vec<String>,
}

impl<'a> Context<'a> {
    fn get_functions(&'_ mut self, prog: &'a Program) {
        for fun in &prog.functions {
            let name = fun.name.0.as_str();
            let ty = FunType::from(fun);

            if self.functions.get(&name).is_some() {
                eprintln!("function {name} already defined");
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
                if expr.validate(self).is_none() {
                    dbg!(expr);
                    panic!("found error");
                }
            }
            self.pop_scope();
        }
    }

    fn find_fun_type(&self, ident: &str) -> Option<FunType> {
        if let Some(ty) = self.functions.get(ident) {
            Some(ty.clone())
        } else {
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
            eprintln!("function {ident} already defined");
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
}

fn check_program(prog: &Program) {
    let mut context = Context::default();

    context.functions.insert(
        "println",
        FunType {
            args: vec![ExprType::Primitive(Primitive::String)],
            ret_type: ExprType::Unit,
        },
    );

    context.get_functions(prog);

    context.validate_functions(prog);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;
    use crate::parser::*;

    #[test]
    fn validation() {
        let source = include_str!("../samples/hello.kt");

        let pairs: Vec<_> = Lexer::new(source).collect();
        let (prog, _) = Program::try_parse(&pairs).unwrap();
        check_program(&prog);
    }
}
