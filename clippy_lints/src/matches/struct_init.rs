use clippy_utils::diagnostics::span_lint_and_help;
use if_chain::if_chain;
use rustc_hir::{
    intravisit::{Visitor},
    Expr, ExprKind,
};
use rustc_lint::LateContext;
use rustc_session::declare_tool_lint;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for match expression in struct field initialization.
    ///
    /// ### Why is this bad?
    /// This is generate lots of LLVM IR which slows down the compilation time.
    ///
    /// ### Example
    /// ```ignore
    /// Struct {
    ///    a: match {...}
    /// }
    /// ```
    ///
    /// ```
    pub MATCH_IN_FIELD_INIT,
    pedantic,
    "using match expression in field initialization"
}

pub fn check_struct_init<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
    let mut visitor = ExprVisitor { cx };
    if let ExprKind::Struct(_, fields, _) = expr.kind {
        for f in fields {
            visitor.visit_expr(&f.expr);
        }
    }

    if let ExprKind::Tup(fields) = expr.kind {
        for f in fields {
            visitor.visit_expr(&f);
        }
    }
}

struct ExprVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
}

impl<'a, 'tcx> Visitor<'tcx> for ExprVisitor<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if_chain! {
            if let ExprKind::Match(..) = expr.kind;
            then {
                span_lint_and_help(
                    self.cx,
                    MATCH_IN_FIELD_INIT,
                    expr.span,
                    "using match expression in field initialization",
                    Some(expr.span),
                    "Consider move this match expression out of initialization expression"
                );
            }
        }
    }
}
