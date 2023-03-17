use super::*;

expr_enum!(ControlExpr => If | For | Return);

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
pub struct If {
    expr: Box<Expr>,
    body: Vec<Expr>,
    else_branch: Option<Box<Expr>>,
}

impl TryParse for If {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::If)?;
        let pairs = expect_symbol(pairs, '(')?;

        let (expr, pairs) = Expr::try_parse(pairs)?;

        let pairs = expect_symbol(pairs, ')')?;

        let (body, pairs) = expect_body(pairs)?;

        if expect_token(pairs, Token::Else).is_ok() {
            return Err(ParseError::NotImplementedYet);
        }

        let i = If {
            expr: Box::new(expr),
            body,
            else_branch: None,
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
}
