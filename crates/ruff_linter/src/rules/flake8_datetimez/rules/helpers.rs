use ruff_python_ast::{Expr, ExprAttribute};

use crate::checkers::ast::Checker;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum DatetimeModuleAntipattern {
    NoTzArgumentPassed,
    NonePassedToTzArgument,
}

/// Check if any parent expression in the chain is a call to `astimezone`.
/// This assumes that the current expression is a `datetime.datetime` object.
/// It will traverse up through method chains to find astimezone calls.
pub(super) fn parent_expr_is_astimezone(checker: &Checker) -> bool {
    checker
        .semantic()
        .current_expressions()
        .skip(1)
        .take_while(|expr| matches!(expr, Expr::Attribute(_) | Expr::Call(_)))
        .any(|expr| matches!(expr, Expr::Attribute(ExprAttribute { attr, .. }) if attr.as_str() == "astimezone"))
}
