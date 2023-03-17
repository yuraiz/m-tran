use super::*;

expr_enum!(MathExpr => Sub | Add | Mul | Div | Parens);

#[derive(Debug, PartialEq)]
pub struct Parens(Box<Expr>);

impl TryParse for Parens {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let pairs = expect_symbol(pairs, '(')?;
        let mut matcing_paren_index = pairs.len();
        let mut nesting_level = 1;
        for (index, pair) in pairs.iter().enumerate() {
            if let Token::Symbol(s) = pair.token {
                if s == '(' {
                    nesting_level += 1;
                }
                if s == ')' {
                    nesting_level -= 1;
                    if nesting_level == 0 {
                        matcing_paren_index = index;
                        break;
                    }
                }
            }
        }

        if matcing_paren_index == pairs.len() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        let (enclosed, pairs) = pairs.split_at(matcing_paren_index);

        let (expr, _) = Expr::try_parse(enclosed)?;
        let pairs = expect_symbol(pairs, ')')?;
        Ok((Parens(Box::new(expr)), pairs))
    }
}

macro_rules! binary_operator {
    ($name:ident => $sym:literal) => {
        #[derive(Debug, PartialEq)]
        pub struct $name {
            left: Box<Expr>,
            right: Box<Expr>,
        }

        impl TryParse for $name {
            fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
                let sym_pos = get_toplevel_index_of(pairs, $sym);

                let Some(&pair) = pairs.get(sym_pos) else {
                                                    return Err(ParseError::UnexpectedEndOfInput);
                                        };

                if pair.token == Token::NewLine {
                    Err(ParseError::UnexpectedToken(pair, Token::Symbol($sym)))
                } else {
                    let (left, right) = pairs.split_at(sym_pos);

                    let (left, _) = Expr::try_parse(left)?;
                    let (right, pairs) = Expr::try_parse(&right[1..])?;
                    Ok((
                        Self {
                            left: Box::new(left),
                            right: Box::new(right),
                        },
                        pairs,
                    ))
                }
            }
        }
    };
}

fn get_toplevel_index_of<'a>(pairs: &'a [Pair<'a>], symbol: char) -> usize {
    let mut target_index = usize::MAX;
    let mut nesting_level = 0;
    for (index, pair) in pairs.iter().enumerate() {
        if let Token::Symbol(s) = pair.token {
            match s {
                '(' => nesting_level += 1,
                ')' => nesting_level -= 1,
                _ => {}
            }
            if nesting_level == 0 && s == symbol {
                target_index = index;
                break;
            }
        }
    }

    target_index
}

binary_operator!(Mul => '*');
binary_operator!(Div => '/');
binary_operator!(Add => '+');
binary_operator!(Sub => '-');

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;

    fn eval_expr(expr: Expr) -> i32 {
        match expr {
            Expr::TopExpr(_) => panic!(),
            Expr::MathExpr(math_expr) => match math_expr {
                MathExpr::Parens(Parens(expr)) => eval_expr(*expr),
                MathExpr::Mul(Mul { left, right }) => eval_expr(*left) * eval_expr(*right),
                MathExpr::Div(Div { left, right }) => eval_expr(*left) / eval_expr(*right),
                MathExpr::Add(Add { left, right }) => eval_expr(*left) + eval_expr(*right),
                MathExpr::Sub(Sub { left, right }) => eval_expr(*left) - eval_expr(*right),
            },
            Expr::ShortExpr(ShortExpr::Literal(Literal::Int(int))) => int,
            _ => panic!(),
        }
    }

    macro_rules! check {
        ($ex:expr) => {
            assert_eq!(eval_expr(make(stringify!($ex))), $ex)
        };
    }

    #[test]
    fn simple() {
        check!(35 + 142);
        check!(35 * 142);
        check!(35 / 142);
        check!(35 - 142);
    }

    #[test]
    fn harder() {
        dbg!(make::<Expr>("2 + 2 * 2"));

        check!(((4 + 2) + 3));

        check!((4 / 2 - 4) * ((4 + 2) + 3));
    }

    #[test]
    fn mul_add() {
        check!(5 * 3 + 4);
    }

    #[test]
    fn add_mul() {
        check!(5 + 3 * 4);
    }

    #[test]
    fn div_mul() {
        check!(30 / 5 * 5);
    }

    #[test]
    fn mul_div() {
        check!(30 * 5 / 5);
    }
}
