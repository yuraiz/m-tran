use super::*;

expr_enum!(MathExpr => Neg | Range | Sub | Add | Mul | Div | Parens);

#[derive(Debug, PartialEq)]
pub struct Parens(pub BoxedExpr);

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

        let (expr, _) = try_parse(enclosed)?;
        let pairs = expect_symbol(pairs, ')')?;
        Ok((Parens(expr), pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct Neg(pub BoxedExpr);

impl TryParse for Neg {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let pairs = expect_symbol(pairs, '-')?;
        let (expr, pairs) = try_parse(pairs)?;
        Ok((Self(expr), pairs))
    }
}

macro_rules! binary_operator {
    ($name:ident => $tok:expr) => {
        #[derive(Debug, PartialEq)]
        pub struct $name {
            pub left: BoxedExpr,
            pub right: BoxedExpr,
        }

        impl TryParse for $name {
            fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
                let operator: Token = $tok.into();

                let sym_pos = get_toplevel_index_of(pairs, operator);

                let &pair = pairs.get(sym_pos).ok_or(ParseError::UnexpectedEndOfInput)?;

                if pair.token == Token::NewLine {
                    Err(ParseError::UnexpectedToken(pair, operator))
                } else {
                    let (left, right) = pairs.split_at(sym_pos);

                    let (left, empty_pairs) = try_parse(left)?;
                    if !empty_pairs.is_empty() {
                        Err(ParseError::WrongExprType(stringify!($name)))
                    } else {
                        let (right, pairs) = try_parse(&right[1..])?;
                        Ok((Self { left, right }, pairs))
                    }
                }
            }
        }
    };
}

pub(crate) use binary_operator;

pub(crate) fn get_toplevel_index_of<'a>(pairs: &'a [Pair<'a>], token: Token) -> usize {
    let mut target_index = usize::MAX;
    let mut nesting_level = 0;
    for (index, pair) in pairs.iter().enumerate() {
        if let Token::Symbol(s) = pair.token {
            match s {
                '(' => nesting_level += 1,
                ')' => nesting_level -= 1,
                _ => {}
            }
        }
        if nesting_level == 0 && pair.token == token {
            target_index = index;
            break;
        }
    }

    target_index
}

binary_operator!(Mul => '*');
binary_operator!(Div => '/');
binary_operator!(Add => '+');
binary_operator!(Sub => '-');
binary_operator!(Range => Token::RangeOp);

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;

    fn eval_expr(expr: &Expr) -> i32 {
        match expr {
            Expr::TopExpr(_) => panic!(),
            Expr::MathExpr(math_expr) => match math_expr {
                MathExpr::Parens(Parens(expr)) => eval_expr(&expr),
                MathExpr::Mul(Mul { left, right }) => eval_expr(&left) * eval_expr(&right),
                MathExpr::Div(Div { left, right }) => eval_expr(&left) / eval_expr(&right),
                MathExpr::Add(Add { left, right }) => eval_expr(&left) + eval_expr(&right),
                MathExpr::Sub(Sub { left, right }) => eval_expr(&left) - eval_expr(&right),
                _ => panic!(),
            },
            Expr::ShortExpr(ShortExpr::Literal(Literal::Int(int))) => *int,
            _ => panic!(),
        }
    }

    macro_rules! check {
        ($ex:expr) => {
            assert_eq!(eval_expr(&make(stringify!($ex))), $ex)
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

    #[test]
    fn range() {
        assert_eq!(
            make::<Expr>("0..(len - 1)"),
            Expr::MathExpr(MathExpr::Range(Range {
                left: Box::new(make("0")),
                right: Box::new(make("(len - 1)"))
            }))
        )
    }
}
