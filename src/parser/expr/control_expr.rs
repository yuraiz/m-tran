use super::*;

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq)]
pub enum ControlExpr {
    If(If),
    For(For),
    While(While),
    Return(Return),
}

impl TryParse for ControlExpr {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let pair = pairs.get(0).ok_or(ParseError::UnexpectedEndOfInput)?;

        match pair.token {
            Token::If => {
                let (r, pairs) = try_parse(pairs)?;
                Ok((ControlExpr::If(r), pairs))
            }
            Token::For => {
                let (r, pairs) = try_parse(pairs)?;
                Ok((ControlExpr::For(r), pairs))
            }
            Token::While => {
                let (r, pairs) = try_parse(pairs)?;
                Ok((ControlExpr::While(r), pairs))
            }
            Token::Return => {
                let (r, pairs) = try_parse(pairs)?;
                Ok((ControlExpr::Return(r), pairs))
            }
            _ => Err(ParseError::WrongExprType(*pair, "ControlExpr")),
        }
    }
}

impl std::fmt::Debug for ControlExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::If(child) => child.fmt(f),
            Self::For(child) => child.fmt(f),
            Self::While(child) => child.fmt(f),
            Self::Return(child) => child.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Return(pub Option<BoxedExpr>);

impl TryParse for Return {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::Return)?;

        if let Ok((expr, pairs)) = try_parse(pairs) {
            Ok((Return(Some(expr)), pairs))
        } else {
            Ok((Return(None), pairs))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct For {
    pub var: Ident,
    pub iterable: BoxedExpr,
    pub body: Body,
}

impl TryParse for For {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::For)?;
        let pairs = expect_symbol(pairs, '(')?;
        let (var, pairs) = try_parse(pairs)?;
        let (_, pairs) = expect_token(pairs, Token::In)?;
        let (iterable, pairs) = try_parse(pairs)?;
        let pairs = expect_symbol(pairs, ')')?;

        let (body, pairs) = expect_body(pairs)?;

        let f = For {
            var,
            iterable,
            body,
        };

        Ok((f, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub expr: BoxedExpr,
    pub body: Body,
}

impl TryParse for While {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::While)?;
        let pairs = expect_symbol(pairs, '(')?;
        let (expr, pairs) = try_parse(pairs)?;
        let pairs = expect_symbol(pairs, ')')?;

        let (body, pairs) = expect_body(pairs)?;

        let w = While { expr, body };

        Ok((w, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub expr: BoxedExpr,
    pub body: Body,
    pub else_branch: Body,
}

impl TryParse for If {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (_, pairs) = expect_token(pairs, Token::If)?;
        let pairs = expect_symbol(pairs, '(')?;

        let (expr, pairs) = try_parse(pairs)?;

        let pairs = expect_symbol(pairs, ')')?;

        let (body, pairs) = expect_body(pairs)?;

        let (else_branch, pairs) = if let Ok((_, pairs)) = expect_token(pairs, Token::Else) {
            if let Ok((top_expr, pairs)) = try_parse(pairs) {
                (vec![top_expr], pairs)
            } else {
                expect_body(pairs)?
            }
        } else {
            (vec![], pairs)
        };

        let i = If {
            expr,
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
