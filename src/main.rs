mod parser;
use std::collections::HashMap;

use parser::{KotlinParser, Rule};
use pest::Parser;

fn main() {
    let fun = KotlinParser::parse(Rule::program, include_str!("samples/hello.kt"))
        .unwrap_or_else(|e| panic!("{}", e));

    let mut map = HashMap::new();

    for name in ["println", "print", "arrayOf"] {
        map.insert(name, Rule::fun);
    }

    // print_pairs(0, fun.clone());

    get_idents(fun.clone(), &mut map);
    check_idents(fun, &map);

    print_as_tables(&map);
}

fn get_idents<'a>(pairs: pest::iterators::Pairs<'a, Rule>, map: &mut HashMap<&'a str, Rule>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::var | Rule::val | Rule::fun | Rule::func_arg | Rule::r#for => {
                let rule = pair.as_rule();
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap();
                assert_eq!(ident.as_rule(), Rule::ident);
                map.insert(ident.as_str(), rule);
                get_idents(inner, map)
            }
            Rule::literal => {
                let pair = pair.into_inner().next().unwrap();
                map.insert(pair.as_str(), pair.as_rule());
            }
            _ => get_idents(pair.into_inner(), map),
        }
    }
}

fn check_idents(pairs: pest::iterators::Pairs<Rule>, map: &HashMap<&str, Rule>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::ident => {
                if !map.contains_key(pair.as_str()) {
                    println!("key {} not found", pair.as_str());
                }
            }
            _ => check_idents(pair.to_owned().into_inner(), map),
        };
    }
}

fn print_as_tables(map: &HashMap<&str, Rule>) {
    let var_bindings = map.iter().filter(|pair| {
        matches!(
            pair.1,
            Rule::val | Rule::var | Rule::func_arg | Rule::fun | Rule::r#for
        )
    });

    let literals = map
        .iter()
        .filter(|pair| matches!(pair.1, Rule::str | Rule::int | Rule::r#bool | Rule::r#char));

    println!("Identifiers:");
    for (name, ty) in var_bindings {
        let ty = match ty {
            Rule::val => "as val",
            Rule::var => "as var",
            Rule::func_arg => "as function arg",
            Rule::fun => "as function",
            Rule::r#for => "in for",
            _ => unreachable!(),
        };
        println!("  identifier {name} declared {ty}");
    }

    println!("Literals:");
    for (val, ty) in literals {
        let ty = match ty {
            Rule::str => "string",
            Rule::int => "integer",
            Rule::r#bool => "boolean",
            Rule::r#char => "char",
            _ => unreachable!(),
        };
        println!("  {ty} literal {val}");
    }
}

fn print_pairs(depth: usize, pairs: pest::iterators::Pairs<Rule>) {
    let count = pairs.clone().count();

    for pair in pairs {
        let child_count = pair.clone().into_inner().count();

        if count > 1 || depth == 0 {
            print!("{:>depth$}- ", "");
        } else {
            print!(" > ");
        }

        print!("{:?}", pair.as_rule());

        if child_count == 0 {
            println!(": {:?}", pair.as_str());
        } else {
            if child_count > 1 {
                println!();
            }
            print_pairs(depth + 1, pair.into_inner());
        }
    }
}
