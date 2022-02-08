use clippy_utils::diagnostics::*;
use clippy_utils::*;

declare_clippy_lint! {
    /// ### What is does
    /// Checks for invoking `borrow/borrow_mut` method of `core::cell::RefCell`
    /// in match scrutinee.
    ///
    /// ### Why is this bad?
    /// Unexpected runtime panicking `BorrowError` can be triggered.
    ///
    /// ### Know problems
    ///
    /// ### Example
    /// ```rust
    /// # use std::cell::RefCell;
    /// # let ref_cell = RefCell::new(Some(bool));
    /// if let Some(_) = *ref_cell.borrow() {
    ///                   ^^^^^^^^^^^^^^^
    ///                   |-- temporary borrow will last until...
    ///    *cell.borrow_mut() = None;
    /// }
    /// ^-- ... the end of this if-let statement
    /// ```
    pub REFCELL_BORROW,
    restriction,
    "borrow/borrow_mut at block tail may lead to BorrowError at runtime"
}

use super::*;
pub fn check_match_refcell_borrow<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>, msg: &str) {
    match &expr.kind {
        ExprKind::MethodCall(_, args, _) => {
            if let [arg] = &**args {
                let method_def_id = cx.typeck_results().type_dependent_def_id(expr.hir_id).unwrap();
                if match_def_path(cx, method_def_id, &paths::REFCELL_BORROW)
                    || match_def_path(cx, method_def_id, &paths::REFCELL_BORROWMUT)
                {
                    span_lint_and_help(
                        cx,
                        REFCELL_BORROW,
                        expr.span,
                        msg,
                        None,
                        "consider using a `let` binding to drop the temporary borrow",
                    );
                } else {
                    check_match_refcell_borrow(cx, arg, msg);
                }
            }
        },
        ExprKind::Unary(_, e) | ExprKind::Field(e, _) | ExprKind::Index(e, _) => {
            check_match_refcell_borrow(cx, e, msg);
        },
        _ => {},
    }
}
