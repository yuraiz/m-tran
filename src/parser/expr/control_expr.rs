use super::*;

expr_enum!(ControlExpr => If | For | While | Return);

#[derive(Debug, PartialEq)]
pub struct Return(Option<Box<Expr>>);

impl TryParse for Return {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::Return)?;

        if let Ok((expr, pairs)) = Expr::try_parse(pairs) {
            Ok((Return(Some(Box::new(expr))), pairs))
        } else {
            Ok((Return(None), pairs))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct For {
    var: Ident,
    iterable: Box<Expr>,
    body: Vec<Expr>,
}

impl TryParse for For {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::For)?;
        let pairs = expect_symbol(pairs, '(')?;
        let (var, pairs) = Ident::try_parse(pairs)?;
        let (_, pairs) = expect_token(pairs, Token::In)?;
        let (expr, pairs) = Expr::try_parse(pairs)?;
        let pairs = expect_symbol(pairs, ')')?;

        let (body, pairs) = expect_body(pairs)?;

        let f = For {
            var,
            iterable: Box::new(expr),
            body,
        };

        Ok((f, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct While {
    expr: Box<Expr>,
    body: Vec<Expr>,
}

impl TryParse for While {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::While)?;
        let pairs = expect_symbol(pairs, '(')?;
        let (expr, pairs) = Expr::try_parse(pairs)?;
        let pairs = expect_symbol(pairs, ')')?;

        let (body, pairs) = expect_body(pairs)?;

        let w = While {
            expr: Box::new(expr),
            body,
        };

        Ok((w, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct If {
    expr: Box<Expr>,
    body: Vec<Expr>,
    else_branch: Vec<Expr>,
}

impl TryParse for If {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::If)?;
        let pairs = expect_symbol(pairs, '(')?;

        let (expr, pairs) = Expr::try_parse(pairs)?;

        let pairs = expect_symbol(pairs, ')')?;

        let (body, pairs) = expect_body(pairs)?;

        let (else_branch, pairs) = if let Ok((_, pairs)) = expect_token(pairs, Token::Else) {
            if let Ok((e, pairs)) = Expr::try_parse(pairs) {
                (vec![e], pairs)
            } else {
                expect_body(pairs)?
            }
        } else {
            (vec![], pairs)
        };

        let i = If {
            expr: Box::new(expr),
            body,
            else_branch,
        };

        Ok((i, pairs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::test_helpers::*;

    #[test]
    fn ret_expr() {
        make::<ControlExpr>("return fn(value)");
    }

    #[test]
    fn if_expr() {
        make::<ControlExpr>(
            "if (value) {
                return kek
            }",
        );
    }

    #[test]
    fn if_else() {
        make::<ControlExpr>(
            r#"if (value) {
                return kek
            } else if (kek) {
                println("kek")
            } else {
                return lol
            }"#,
        );
    }

    #[test]
    fn while_loop() {
        make::<ControlExpr>(
            r#"while (expr) {
                a = b
                b = a
            }"#,
        );
    }

    #[test]
    fn for_loop() {
        make::<ControlExpr>(
            r#"for (item in collection) {
                sum = sum + item
            }"#,
        );
    }

    #[test]
    fn for_range() {
        make::<ControlExpr>(
            r#"for (num in 0..(n + 3)) {
                sum = sum + num
            }"#,
        );
    }
}
