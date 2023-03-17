mod token;

pub use token::Token;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Pair<'a> {
    source: &'a str,
    pub token: Token,
    pub span: Span,
}

impl<'a> Pair<'a> {
    pub fn str(&self) -> &'a str {
        let Span { lo, hi } = self.span;
        &self.source[lo..hi]
    }
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Pair<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, new_remaining) = Token::parse(self.remaining);

        let lo = self.original.len() - self.remaining.len();
        let hi = self.original.len() - new_remaining.len();
        let span = Span { lo, hi };

        // dbg!(&self.original[lo..hi]);

        self.remaining = new_remaining;

        if token == Token::WhiteSpace {
            return self.next();
        }

        if token == Token::End {
            None
        } else {
            Some(Pair {
                source: self.original,
                token,
                span,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        let source = r#"
        fun main() {
            println("Hello, World!")
        }
        "#;

        let tokens: Vec<Token> = Lexer::new(source).map(|pair| pair.token).collect();

        use Token::*;

        assert_eq!(
            &tokens[..],
            [
                NewLine,
                Fun,
                Ident,
                Symbol('('),
                Symbol(')'),
                Symbol('{'),
                NewLine,
                Ident,
                Symbol('('),
                Str,
                Symbol(')'),
                NewLine,
                Symbol('}'),
                NewLine,
            ]
        );
    }

    #[test]
    fn samples() {
        fn all_expected(source: &str) -> bool {
            Lexer::new(source).all(|pair| pair.token != Token::Unexpected)
        }

        assert!(all_expected(include_str!("../samples/hello.kt")));
        assert!(all_expected(include_str!("../samples/arrays.kt")));
        assert!(all_expected(include_str!("../samples/factorial.kt")));
    }
}
