use super::*;

expr_enum!(TopExpr => ControlExpr | Binding | Set | Call);

#[derive(Debug, PartialEq)]
pub struct Binding {
    is_mut: bool,
    set: Set,
}

impl TryParse for Binding {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (pair, pairs) = if let Ok(res) = expect_token(pairs, Token::Val) {
            res
        } else if let Ok(res) = expect_token(pairs, Token::Var) {
            res
        } else {
            return Err(ParseError::WrongExprType("Binding"));
        };

        let is_mut = pair.token == Token::Var;

        let (set, pairs) = Set::try_parse(pairs)?;

        let binding = Binding { is_mut, set };

        Ok((binding, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct Set {
    name: Ident,
    expr: Box<Expr>,
}

impl TryParse for Set {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (name, pairs) = Ident::try_parse(pairs)?;

        let pairs = expect_symbol(pairs, '=')?;

        let (expr, pairs) = Expr::try_parse(pairs)?;

        let set = Set {
            name,
            expr: Box::new(expr),
        };

        Ok((set, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct Call {
    name: Ident,
    args: Vec<Expr>,
}

impl TryParse for Call {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (name, pairs) = Ident::try_parse(pairs)?;

        let (args, pairs) =
            expect_sequence(pairs, '('.into(), ')'.into(), ','.into(), Expr::try_parse)?;

        let call = Call { name, args };

        Ok((call, pairs))
    }
}
