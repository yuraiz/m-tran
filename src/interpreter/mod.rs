mod eval;
mod object;

use object::Object;

use crate::{
    interpreter::eval::Eval,
    parser::{Fun, Program},
};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Default)]
pub struct Context {
    functions: HashMap<String, Fun>,
    scopes: RefCell<Vec<HashMap<String, Object>>>,
    ret_item: RefCell<Option<Object>>,
}

impl Context {
    pub fn new(prog: Program) -> Self {
        let functions = prog
            .functions
            .into_iter()
            .map(|f| (f.name.0.clone(), f))
            .collect();

        Self {
            functions,
            ..Default::default()
        }
    }

    pub fn run(&self) {
        self.call_function("main", vec![]);
    }

    fn get(&self, name: &str) -> Object {
        for scope in self.scopes.borrow().iter().rev() {
            if let Some(obj) = scope.get(name) {
                return obj.clone();
            }
        }
        unreachable!()
    }

    fn var(&self, name: &str, obj: Object) {
        self.scopes
            .borrow_mut()
            .last_mut()
            .unwrap()
            .insert(name.to_owned(), obj);
    }

    fn set(&self, name: &str, obj: Object) {
        for scope in self.scopes.borrow_mut().iter_mut().rev() {
            if scope.get(name).is_some() {
                scope.insert(name.to_owned(), obj);
                return;
            }
        }
        unreachable!()
    }

    fn push(&self) {
        self.scopes.borrow_mut().push(Default::default())
    }

    fn pop(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn exception(&self, message: String) -> ! {
        eprintln!("{}", message);
        std::process::exit(0)
    }

    fn call_function(&self, name: &str, args: Vec<Object>) -> Object {
        if let Some(obj) = self.call_predefined_function(name, &args) {
            return obj;
        }

        if let Some(fun) = self.functions.get(name).to_owned() {
            assert_eq!(fun.args.len(), args.len());
            let names = fun.args.iter().map(|a| (a.0).0.to_owned());

            self.scopes.borrow_mut().push(names.zip(args).collect());

            for expr in &fun.body {
                expr.eval(self);
                if let Some(obj) = self.ret_item.borrow_mut().take() {
                    self.pop();
                    return obj;
                }
            }

            self.pop();
            Object::Unit
        } else {
            unimplemented!("function {name} doesn't exist")
        }
    }

    fn call_predefined_function(&self, name: &str, args: &[Object]) -> Option<Object> {
        use funcs::*;
        let fun = match name {
            "println" => println,
            "print" => print,
            "arrayOf" => arrayOf,
            "readln" => readln,
            "readlnInt" => readlnInt,
            "readlnBoolean" => readlnBoolean,
            _ => return None,
        };
        Some(fun(args.to_vec(), self))
    }
}

mod funcs {
    #![allow(non_snake_case)]

    use super::*;

    pub fn print(args: Vec<Object>, _: &Context) -> Object {
        for arg in args {
            print!("{}", arg.to_string())
        }
        Object::Unit
    }

    pub fn println(args: Vec<Object>, c: &Context) -> Object {
        print(args, c);
        println!();
        Object::Unit
    }

    pub fn arrayOf(args: Vec<Object>, _: &Context) -> Object {
        Object::Array(std::rc::Rc::new(RefCell::new(args)))
    }

    // "readln" => Some(ExprType::Primitive(Primitive::String)),
    // "readlnInt" => Some(ExprType::Primitive(Primitive::Int)),
    // "readlnBoolean" => Some(ExprType::Primitive(Primitive::Boolean)),

    fn read_string(context: &Context) -> String {
        if let Some(Ok(string)) = std::io::stdin().lines().next() {
            string
        } else {
            context.exception("Failed to read input".into())
        }
    }

    pub fn readln(_: Vec<Object>, context: &Context) -> Object {
        Object::String(read_string(context))
    }

    pub fn readlnInt(_: Vec<Object>, context: &Context) -> Object {
        if let Ok(int) = read_string(context).parse() {
            Object::Int(int)
        } else {
            context.exception("Failed to parse Int".into())
        }
    }

    pub fn readlnBoolean(_: Vec<Object>, context: &Context) -> Object {
        if let Ok(int) = read_string(context).parse() {
            Object::Boolean(int)
        } else {
            context.exception("Failed to parse Boolean".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;
    use crate::parser::*;

    #[test]
    fn interpretation() {
        fn interpret(source: &str) {
            let pairs: Vec<_> = Lexer::new(source).collect();
            let (prog, _) = Program::try_parse(&pairs).unwrap();
            Context::new(prog).run()
        }

        interpret(include_str!("../samples/arrays.kt"));
        interpret(include_str!("../samples/hello.kt"));
        interpret(include_str!("../samples/factorial.kt"));
        interpret(include_str!("../samples/sort.kt"));
    }
}
