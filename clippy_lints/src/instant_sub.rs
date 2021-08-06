use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty;
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::source_map::Spanned;

use clippy_utils::diagnostics::*;
use clippy_utils::match_def_path;
use clippy_utils::paths;
use clippy_utils::source::snippet;
declare_clippy_lint! {
    pub INSTANT_SUB,
    correctness,
    "checks for `instant_a - instant_b`"
}

declare_lint_pass!(InstantSub => [INSTANT_SUB]);

impl LateLintPass<'_> for InstantSub {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ Expr<'_>) {
        if_chain! {
            if let ExprKind::Binary(Spanned { node: BinOpKind::Sub, .. }, left, right) = &expr.kind;
            if let ty::Adt(ladt, _) = cx.typeck_results().expr_ty(left).peel_refs().kind();
            if let ty::Adt(radt, _) = cx.typeck_results().expr_ty(right).peel_refs().kind();
            if match_def_path(cx, ladt.did, &paths::INSTANT);
            if match_def_path(cx, radt.did, &paths::INSTANT);
            then {
                let applicability = Applicability::MachineApplicable;
                let lhs = snippet(cx, left.span, "");
                let rhs = snippet(cx, right.span, "");
                span_lint_and_sugg(
                    cx,
                    INSTANT_SUB,
                    expr.span,
                    &format!("Calling `.checked_duration_since()` is better that this calculation"),
                    "try",
                    format!(
                        "{}.{}({})",
                        lhs,"checked_duration_since",rhs,
                    ),
                    applicability,
                );
            }
        }
    }
}
