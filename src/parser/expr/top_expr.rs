use super::*;

expr_enum!(TopExpr => ControlExpr | Binding | Set | Call | SetByIndex);

#[derive(Debug, PartialEq)]
pub struct Binding {
    pub is_mut: bool,
    pub set: Set,
}

impl TryParse for Binding {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (pair, pairs) = if let Ok(res) = expect_token(pairs, Token::Val) {
            res
        } else if let Ok(res) = expect_token(pairs, Token::Var) {
            res
        } else {
            let pair = pairs.get(1).ok_or(ParseError::UnexpectedEndOfInput)?;
            return Err(ParseError::WrongExprType(*pair, "Binding"));
        };

        let is_mut = pair.token == Token::Var;

        let (set, pairs) = Set::try_parse(pairs)?;

        let binding = Binding { is_mut, set };

        Ok((binding, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct Set {
    pub name: Ident,
    pub expr: BoxedExpr,
}

impl TryParse for Set {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (name, pairs) = try_parse(pairs)?;

        let pairs = expect_symbol(pairs, '=')?;

        let (expr, pairs) = try_parse(pairs)?;

        let set = Set { name, expr };

        Ok((set, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub name: Ident,
    pub args: Vec<Expr>,
}

impl TryParse for Call {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (name, pairs) = try_parse(pairs)?;

        let (args, pairs) =
            expect_sequence(pairs, '('.into(), ')'.into(), ','.into(), Expr::try_parse)?;

        let call = Call { name, args };

        Ok((call, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct SetByIndex {
    pub name: Ident,
    pub index: BoxedExpr,
    pub expr: BoxedExpr,
}

impl TryParse for SetByIndex {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (name, pairs) = Ident::try_parse(pairs)?;

        let pairs = expect_symbol(pairs, '[')?;
        let (index, pairs) = try_parse(pairs)?;
        let pairs = expect_symbol(pairs, ']')?;

        let pairs = expect_symbol(pairs, '=')?;

        let (expr, pairs) = try_parse(pairs)?;

        let set = SetByIndex { name, index, expr };

        Ok((set, pairs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::test_helpers::make;

    #[test]
    fn call() {
        make::<Expr>("hello(world)");
        make::<Expr>(r#"println(a + item)"#);
    }

    #[test]
    fn set() {
        assert_eq!(
            make::<TopExpr>("a = b"),
            TopExpr::Set(Set {
                name: make("a"),
                expr: make("b")
            })
        )
    }

    #[test]
    fn set_by_index() {
        assert_eq!(
            make::<TopExpr>("a[i + j * n] = b"),
            TopExpr::SetByIndex(SetByIndex {
                name: Ident("a".to_string()),
                index: make("i + j * n"),
                expr: make("b"),
            })
        );
    }

    #[test]
    fn bindings() {
        let val: TopExpr = make("val hello = 0");
        assert_eq!(
            val,
            TopExpr::Binding(Binding {
                is_mut: false,
                set: make("hello = 0")
            })
        );

        let var: TopExpr = make("var test = 4");
        assert_eq!(
            var,
            TopExpr::Binding(Binding {
                is_mut: true,
                set: make("test = 4")
            })
        );
    }
}
