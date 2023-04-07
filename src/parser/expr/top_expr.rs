use super::*;

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq)]
pub enum TopExpr {
    ControlExpr(ControlExpr),
    Binding(Binding),
    Set(Set),
    Call(Call),
    SetByIndex(SetByIndex),
}

impl TryParse for TopExpr {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let pair = pairs.get(0).ok_or(ParseError::UnexpectedEndOfInput)?;

        match pair.token {
            Token::Ident => {
                if let Ok((r, pairs)) = Set::try_parse(pairs) {
                    return Ok((TopExpr::Set(r), pairs));
                } else if let Ok((r, pairs)) = Call::try_parse(pairs) {
                    return Ok((TopExpr::Call(r), pairs));
                } else if let Ok((r, pairs)) = SetByIndex::try_parse(pairs) {
                    return Ok((TopExpr::SetByIndex(r), pairs));
                } else {
                    Err(ParseError::WrongExprType(*pair, &stringify!(TopExpr)))
                }
            }
            Token::If | Token::For | Token::While | Token::Return => {
                let (r, pairs) = try_parse(pairs)?;
                Ok((Self::ControlExpr(r), pairs))
            }
            Token::Var | Token::Val => {
                let (binding, pairs) = Binding::try_parse(pairs)?;
                Ok((Self::Binding(binding), pairs))
            }
            _ => Err(ParseError::WrongExprType(*pair, "TopExpr")),
        }
    }
}

impl std::fmt::Debug for TopExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ControlExpr(child) => child.fmt(f),
            Self::Binding(child) => child.fmt(f),
            Self::Set(child) => child.fmt(f),
            Self::Call(child) => child.fmt(f),
            Self::SetByIndex(child) => child.fmt(f),
        }
    }
}

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
    pub name: Spanned<Ident>,
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
    pub name: Spanned<Ident>,
    pub args: Vec<Spanned<Expr>>,
}

impl TryParse for Call {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (name, pairs) = try_parse(pairs)?;

        let (args, pairs) = expect_sequence(pairs, '('.into(), ')'.into(), ','.into(), try_parse)?;

        let call = Call { name, args };

        Ok((call, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct SetByIndex {
    pub get_by_index: GetByIndex,
    pub expr: BoxedExpr,
}

impl TryParse for SetByIndex {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (get_by_index, pairs) = try_parse(pairs)?;

        let pairs = expect_symbol(pairs, '=')?;

        let (expr, pairs) = try_parse(pairs)?;

        let set = SetByIndex { get_by_index, expr };

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
                get_by_index: make("a[i + j * n]"),
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
