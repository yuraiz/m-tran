mod control_expr;
mod math_expr;
mod short_expr;
mod top_expr;

pub use control_expr::*;
pub use math_expr::*;
pub use short_expr::*;
pub use top_expr::*;

use super::*;

expr_enum!(Expr => TopExpr | MathExpr | ShortExpr);

macro_rules! expr_enum {
    ($name:ident => $($type:ident)|+ ) => {
        #[allow(clippy::enum_variant_names)]
        #[derive(Debug, PartialEq)]
        pub enum $name {
             $($type($type)),+
        }

        impl TryParse for $name {
            fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {

                $(
                    if let Ok((r, pairs)) = $type::try_parse(pairs) {
                        return Ok(($name::$type(r), pairs))
                    }
                )+

                Err(ParseError::WrongExprType(&stringify!($name)))
            }
        }
    };
}

pub(crate) use expr_enum;

#[cfg(test)]
mod tests {
    use crate::parser::helpers::test_helpers::make;

    #[test]
    fn all_expected() {
        make::<super::Fun>(include_str!("../../samples/hello.kt"));
        dbg!(make::<super::Fun>(
            r#"fun printArray(array: Array<Int>) {
                println("[")
                for (item in array) {
                    println(a + item)
                }
                println("]")
            }"#
        ));
        // make::<super::Fun>(include_str!("../../samples/factorial.kt"));
    }
}
