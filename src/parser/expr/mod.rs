mod comparison_expr;
mod control_expr;
mod math_expr;
mod short_expr;
mod top_expr;

pub use comparison_expr::*;
pub use control_expr::*;
pub use math_expr::*;
pub use short_expr::*;
pub use top_expr::*;

use super::*;

expr_enum!(Expr => TopExpr | MathExpr | ComparisonExpr | ShortExpr);

macro_rules! expr_enum {
    ($name:ident => $($type:ident)|+ ) => {
        #[allow(clippy::enum_variant_names)]
        #[derive(PartialEq)]
        pub enum $name {
             $($type($type)),+
        }

        impl TryParse for $name {
            fn try_parse<'a>(pairs: &'a [Pair<'a>]) -> ParseResult<Self> {
                let pair = pairs.get(0).ok_or(ParseError::UnexpectedEndOfInput)?;

                $(
                    if let Ok((r, pairs)) = $type::try_parse(pairs) {
                        return Ok(($name::$type(r), pairs))
                    }
                )+

                Err(ParseError::WrongExprType(*pair, &stringify!($name)))
            }
        }

        impl std::fmt::Debug for $name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                $(
                    Self::$type(child) => child.fmt(f)
                ),+
            }
        }
}
    };
}

pub(crate) use expr_enum;
