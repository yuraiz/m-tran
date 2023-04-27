#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    WhiteSpace,
    End,
    Ident,
    Char,
    Str,
    Int(i32),
    Bool(bool),
    AndOp,
    OrOp,
    RangeOp,
    NewLine,
    Symbol(char),

    // Keywords
    Fun,
    If,
    Else,
    For,
    In,
    While,
    Var,
    Val,
    Return,
    Break,

    Unexpected,
}

use Token::*;

impl Token {
    pub fn parse(string: &str) -> (Self, &str) {
        if string.is_empty() {
            return (End, string);
        }

        let parsing_functions = [
            parse_comment,
            parse_symbol,
            parse_kw,
            parse_bool,
            parse_char,
            parse_ident,
            parse_int,
            parse_new_line,
            parse_bool_op,
            parse_range_op,
            parse_str,
            parse_white_space,
        ];

        for func in parsing_functions {
            if let Some(result) = func(string) {
                return result;
            }
        }

        let mut unexpected_count = 1;

        while let (Unexpected, _) = Self::parse(&string[unexpected_count..]) {
            unexpected_count += 1;
        }

        (Unexpected, &string[unexpected_count..])
    }
}

impl From<char> for Token {
    fn from(val: char) -> Self {
        Token::Symbol(val)
    }
}

fn parse_comment(string: &str) -> Option<(Token, &str)> {
    if string.starts_with("/*") {
        if let Some(end) = string.find("*/") {
            Some((Token::WhiteSpace, &string[(end + "*/".len())..]))
        } else {
            Some((Token::Unexpected, ""))
        }
    } else if string.starts_with("//") {
        if let Some(end) = string.find("\n").or_else(|| string.find("\r")) {
            Some((Token::NewLine, &string[(end + 1)..]))
        } else {
            Some((Token::End, ""))
        }
    } else {
        None
    }
}

fn parse_symbol(string: &str) -> Option<(Token, &str)> {
    let c = string.chars().next()?;

    if "(){}[],:+-*/%<>=!".contains(c) {
        Some((Symbol(c), &string[1..]))
    } else {
        None
    }
}

fn parse_white_space(string: &str) -> Option<(Token, &str)> {
    let skip = string.chars().take_while(|c| *c == ' ').count();
    if skip != 0 {
        Some((WhiteSpace, &string[skip..]))
    } else {
        None
    }
}

fn parse_new_line(string: &str) -> Option<(Token, &str)> {
    let string = if let Some(string) = string.strip_prefix("\r\n") {
        string
    } else {
        match string.chars().next()? {
            '\r' | '\n' => &string[1..],
            _ => return None,
        }
    };

    Some((NewLine, string))
}

fn parse_bool_op(string: &str) -> Option<(Token, &str)> {
    if let Some(string) = string.strip_prefix("&&") {
        Some((AndOp, string))
    } else {
        string.strip_prefix("||").map(|string| (OrOp, string))
    }
}

fn parse_range_op(string: &str) -> Option<(Token, &str)> {
    string.strip_prefix("..").map(|string| (RangeOp, string))
}

fn parse_kw(string: &str) -> Option<(Token, &str)> {
    let alphabetic_count = string
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .count();
    let (alphabetic, string) = string.split_at(alphabetic_count);

    let token = match alphabetic {
        "fun" => Fun,
        "if" => If,
        "else" => Else,
        "for" => For,
        "in" => In,
        "while" => While,
        "var" => Var,
        "val" => Val,
        "return" => Return,
        "break" => Break,
        _ => return None,
    };

    Some((token, string))
}

fn parse_ident(string: &str) -> Option<(Token, &str)> {
    let alphanumeric_count = string
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .count();
    let (alphanumeric, string) = string.split_at(alphanumeric_count);

    if alphanumeric.chars().next()?.is_numeric() {
        None
    } else {
        Some((Ident, string))
    }
}

fn parse_bool(string: &str) -> Option<(Token, &str)> {
    if let Some(string) = string.strip_prefix("true") {
        Some((Bool(true), string))
    } else {
        string
            .strip_prefix("false")
            .map(|string| (Bool(false), string))
    }
}

fn parse_str(string: &str) -> Option<(Token, &str)> {
    const ESCAPE_SEQ_PARTS: &[u8] = br#"tbnr"\$"#;

    if string.chars().next()? != '"' {
        return None;
    }

    let string = &string[1..];

    let mut string_size = 0;
    let mut escape = false;

    let mut fail = false;

    for c in string.bytes() {
        string_size += 1;
        if c.is_ascii() {
            if escape {
                escape = false;
                if !ESCAPE_SEQ_PARTS.contains(&c) {
                    fail = true;
                }
            } else if c == b'\\' {
                escape = true;
            } else if c == b'"' {
                break;
            }
        }
    }

    if fail {
        Some((Unexpected, &string[string_size..]))
    } else {
        match string.chars().nth(string_size - 1) {
            Some(last) if last == '"' => Some((Str, &string[string_size..])),
            _ => Some((Unexpected, &string[string_size..])),
        }
    }
}

fn parse_char(string: &str) -> Option<(Token, &str)> {
    const ESCAPE_SEQ_PARTS: &[u8] = br#"tbnr'\"#;

    if string.chars().next()? != '\'' {
        return None;
    }

    let string = &string[1..];

    let mut string_size = 0;
    let mut escape = false;

    for c in string.bytes() {
        string_size += 1;
        if c.is_ascii() {
            if escape {
                escape = false;
                if !ESCAPE_SEQ_PARTS.contains(&c) {
                    return None;
                }
            } else if c == b'\\' {
                escape = true;
            } else if c == b'\'' {
                break;
            }
        }
    }

    Some((Char, &string[string_size..]))
}

fn parse_int(string: &str) -> Option<(Token, &str)> {
    let digit_count = string
        .bytes()
        .take_while(|b| b.is_ascii_hexdigit() || *b == b'x')
        .count();

    if digit_count == 0 {
        return None;
    }

    let (digits, string) = string.split_at(digit_count);

    let parse_res = if let Some(hex) = digits.strip_prefix("0x") {
        i32::from_str_radix(hex, 16)
    } else if let Some(binary) = digits.strip_prefix("0b") {
        i32::from_str_radix(binary, 2)
    } else {
        digits.parse()
    };

    let chars_after_num = string
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .count();

    let token = if let Ok(number) = parse_res {
        Int(number)
    } else {
        Unexpected
    };

    if chars_after_num == 0 {
        Some((token, string))
    } else {
        Some((Unexpected, &string[chars_after_num..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        assert_eq!(parse_str(r#""Hello, world""#), Some((Str, "")));
        assert_eq!(parse_str(r#""Hello, world"+=8"#), Some((Str, "+=8")));
        assert_eq!(parse_str(r#""Hello, world\n""#), Some((Str, "")));
        assert_eq!(parse_str(r#""\lol""#), Some((Unexpected, "")));
    }

    #[test]
    fn char() {
        assert_eq!(parse_char("'a'"), Some((Char, "")));
        assert_eq!(parse_char("'\\t'"), Some((Char, "")));
        assert_eq!(parse_char("'\\l'"), None);
    }

    #[test]
    fn int() {
        assert_eq!(parse_int("42"), Some((Int(42), "")));
        assert_eq!(parse_int("0xfd2"), Some((Int(0xfd2), "")));
        assert_eq!(parse_int("0b10011"), Some((Int(0b10011), "")));
        assert_eq!(parse_int("hello"), None);
    }
}
