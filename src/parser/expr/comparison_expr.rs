use super::math_expr::binary_operator;
use super::*;

expr_enum!(ComparisonExpr => And | Or | LessThan | MoreThan);

binary_operator!(LessThan => '<');
binary_operator!(MoreThan => '>');
binary_operator!(And => Token::AndOp);
binary_operator!(Or => Token::OrOp);
