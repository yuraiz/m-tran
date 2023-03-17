use super::parse_error::*;
use crate::lexer::{Pair, Token};
use crate::parser::{Expr, TryParse};

pub fn expect_token<'a>(pairs: &'a [Pair<'a>], token: Token) -> ParseResult<Pair> {
    let pair = pairs.first().ok_or(ParseError::UnexpectedEndOfInput)?;
    if pair.token == token {
        Ok((*pair, &pairs[1..]))
    } else {
        Err(ParseError::UnexpectedToken(*pair, token))
    }
}

pub fn expect_symbol<'a>(
    pairs: &'a [Pair<'a>],
    symbol: char,
) -> Result<&'a [Pair<'a>], ParseError> {
    let pairs = expect_token(pairs, symbol.into())?;
    Ok(pairs.1)
}

pub fn expect_sequence<'a, T, F>(
    pairs: &'a [Pair<'a>],
    start: Token,
    end: Token,
    separator: Token,
    parse_seq_el: F,
) -> ParseResult<Vec<T>>
where
    F: Fn(&'a [Pair<'a>]) -> ParseResult<T>,
{
    let (_, pairs) = expect_token(pairs, start)?;

    let mut mut_pairs = pairs;
    let mut sequence = vec![];

    loop {
        match expect_token(mut_pairs, end) {
            Ok((_, pairs)) => {
                return Ok((sequence, pairs));
            }
            Err(err) => match err {
                ParseError::UnexpectedToken(_, _) => {}
                _ => return Err(err),
            },
        }

        let (element, pairs) = parse_seq_el(mut_pairs)?;
        sequence.push(element);

        match expect_token(pairs, separator) {
            Ok((_, pairs)) => mut_pairs = pairs,
            Err(err) => match err {
                ParseError::UnexpectedToken(_, _) => {
                    mut_pairs = pairs;
                }
                _ => return Err(err),
            },
        }
    }
}

pub fn ignore_token<'a>(pairs: &'a [Pair<'a>], token: Token) -> &'a [Pair<'a>] {
    match pairs.get(0) {
        Some(Pair { token: tok, .. }) if *tok == token => &pairs[1..],
        _ => pairs,
    }
}

pub fn expect_body<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Vec<super::Expr>> {
    let pairs = expect_symbol(pairs, '{')?;

    let mut mut_pairs = pairs;
    let mut sequence = vec![];

    loop {
        mut_pairs = ignore_token(mut_pairs, Token::NewLine);

        match expect_symbol(mut_pairs, '}') {
            Ok(pairs) => {
                return Ok((sequence, pairs));
            }
            Err(err) => match err {
                ParseError::UnexpectedToken(_, _) => {}
                _ => return Err(err),
            },
        }

        let (element, pairs) = Expr::try_parse(mut_pairs)?;
        sequence.push(element);

        mut_pairs = pairs;
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::TryParse;

    pub fn pairs(source: &str) -> Vec<Pair> {
        Lexer::new(source).collect()
    }

    pub fn make<T>(source: &str) -> T
    where
        T: TryParse,
    {
        let pairs = &pairs(source);
        let (res, pairs) = T::try_parse(&pairs).unwrap_or_else(|e| panic!("{e}"));
        assert!(pairs.is_empty(), "source is not fully parsed");
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;
    use test_helpers::*;

    #[test]
    fn sequence() {
        expect_sequence(
            &pairs("(hello)"),
            Token::Symbol('('),
            Token::Symbol(')'),
            Token::Symbol(','),
            |pairs| Ident::try_parse(pairs),
        )
        .unwrap();
    }

    #[test]
    fn arg() {
        expect_sequence(
            &pairs("(array: Array<Int>)"),
            Token::Symbol('('),
            Token::Symbol(')'),
            Token::Symbol(','),
            |pairs| {
                let (arg_name, pairs) = Ident::try_parse(pairs)?;
                dbg!(&arg_name);
                let (_, pairs) = expect_token(pairs, Token::Symbol(':'))?;
                let (arg_type, pairs) = Type::try_parse(pairs)?;
                dbg!(&arg_type);
                Ok(((arg_name, arg_type), pairs))
            },
        )
        .unwrap();
    }
}
