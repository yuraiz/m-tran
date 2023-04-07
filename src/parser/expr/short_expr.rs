use super::*;

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq)]
pub enum ShortExpr {
    Ident(Ident),
    GetByIndex(GetByIndex),
    Literal(Literal),
}

impl TryParse for ShortExpr {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let pair = pairs.get(0).ok_or(ParseError::UnexpectedEndOfInput)?;

        match pair.token {
            Token::Char | Token::Str | Token::Int(_) | Token::Bool(_) => {
                let (r, pairs) = try_parse(pairs)?;
                Ok((ShortExpr::Literal(r), pairs))
            }
            Token::Ident => {
                if let Ok((r, pairs)) = try_parse(pairs) {
                    Ok((ShortExpr::GetByIndex(r), pairs))
                } else {
                    let (r, pairs) = try_parse(pairs)?;
                    Ok((ShortExpr::Ident(r), pairs))
                }
            }
            _ => Err(ParseError::WrongExprType(*pair, &stringify!(ShortExpr))),
        }
    }
}

impl std::fmt::Debug for ShortExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(child) => child.fmt(f),
            Self::Literal(child) => child.fmt(f),
            Self::GetByIndex(child) => child.fmt(f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ident(pub String);

impl TryParse for Ident {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (ident_pair, pairs) = expect_token(pairs, Token::Ident)?;
        Ok((Self(ident_pair.str().to_owned()), pairs))
    }
}

#[derive(Debug, PartialEq)]
pub struct GetByIndex {
    pub ident: Spanned<Ident>,
    pub index: BoxedExpr,
}

impl TryParse for GetByIndex {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let (ident, pairs) = try_parse(pairs)?;
        let pairs = expect_symbol(pairs, '[')?;
        let (index, pairs) = try_parse(pairs)?;
        let pairs = expect_symbol(pairs, ']')?;

        Ok((Self { ident, index }, pairs))
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Bool(bool),
    Char(char),
    String(String),
}

impl TryParse for Literal {
    fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
        let pair = pairs.first().ok_or(ParseError::UnexpectedEndOfInput)?;
        let literal = match pair.token {
            Token::Char => Self::Char(parse_char(pair.str())),
            Token::Str => Self::String(parse_string(pair.str())),
            Token::Int(val) => Self::Int(val),
            Token::Bool(val) => Self::Bool(val),
            _ => return Err(ParseError::WrongExprType(*pair, "Literal")),
        };

        Ok((literal, &pairs[1..]))
    }
}

fn parse_char(string: &str) -> char {
    let mut chars = string.chars().skip(1).take(2);
    let c = chars.next().unwrap();

    if c == '\\' {
        let escaped = chars.next().unwrap();
        match escaped {
            't' => '\t',
            'b' => '\x08',
            'n' => '\n',
            'r' => '\r',
            '\'' | '\\' => escaped,
            _ => unreachable!(),
        }
    } else {
        c
    }
}

fn parse_string(string: &str) -> String {
    let substring = &string['"'.len_utf8()..string.len() - '"'.len_utf8()];

    if !substring.chars().any(|c| c == '\\') {
        substring.to_owned()
    } else {
        let mut new_str = String::new();
        let mut escaped = false;

        for c in substring.chars() {
            if c == '\\' {
                if escaped {
                    new_str.push(c)
                }
                escaped = !escaped;
            } else if escaped {
                let c = match c {
                    't' => '\t',
                    'b' => '\x08',
                    'n' => '\n',
                    'r' => '\r',
                    '\"' | '\\' | '$' => c,
                    _ => unreachable!(),
                };
                escaped = !escaped;
                new_str.push(c);
            } else {
                new_str.push(c);
            }
        }

        new_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::test_helpers::*;

    #[test]
    fn ident() {
        let ident: ShortExpr = make("source");
        assert_eq!(ident, ShortExpr::Ident(Ident("source".into())))
    }

    #[test]
    fn get_by_index() {
        assert_eq!(
            make::<ShortExpr>("a[i]"),
            ShortExpr::GetByIndex(GetByIndex {
                ident: make("a"),
                index: make("i")
            })
        )
    }

    #[test]
    fn int_literal() {
        let literal: ShortExpr = make("123");
        assert_eq!(literal, ShortExpr::Literal(Literal::Int(123)));

        let literal: ShortExpr = make("0x42");
        assert_eq!(literal, ShortExpr::Literal(Literal::Int(0x42)));

        let literal: ShortExpr = make("0b101");
        assert_eq!(literal, ShortExpr::Literal(Literal::Int(0b101)));
    }

    #[test]
    fn bool_literal() {
        let literal: ShortExpr = make("true");
        assert_eq!(literal, ShortExpr::Literal(Literal::Bool(true)));

        let literal: ShortExpr = make("true");
        assert_eq!(literal, ShortExpr::Literal(Literal::Bool(true)));
    }

    #[test]
    fn char_literal() {
        let literal: ShortExpr = make("'a'");
        assert_eq!(literal, ShortExpr::Literal(Literal::Char('a')));

        let literal: ShortExpr = make(r#"'\t'"#);
        assert_eq!(literal, ShortExpr::Literal(Literal::Char('\t')));

        let literal: ShortExpr = make(r#"'\\'"#);
        assert_eq!(literal, ShortExpr::Literal(Literal::Char('\\')));
    }

    #[test]
    fn string_literal() {
        let literal: ShortExpr = make(r#""Hello""#);
        assert_eq!(literal, ShortExpr::Literal(Literal::String("Hello".into())));

        let literal: ShortExpr = make(r#""\r\"\n\t""#);
        assert_eq!(
            literal,
            ShortExpr::Literal(Literal::String("\r\"\n\t".into()))
        );
    }
}
