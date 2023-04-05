use super::math_expr::binary_operator;
use super::*;

expr_enum!(ComparisonExpr => LessThan | MoreThan);

binary_operator!(LessThan => '<');
binary_operator!(MoreThan => '>');
