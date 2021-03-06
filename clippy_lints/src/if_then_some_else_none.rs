use crate::utils;
use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::lint::in_external_macro;
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// **What it does:** Checks for if-else that could be written to `bool::then`.
    ///
    /// **Why is this bad?** Looks a little redundant. Using `bool::then` helps it have less lines of code.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # fn foo() -> bool { unimplemented!() }
    /// let a = if foo() {
    ///     println!("true!");
    ///     Some(42)
    /// } else {
    ///     None
    /// };
    /// ```
    ///
    /// Could be written:
    ///
    /// ```rust
    /// # fn foo() -> bool { unimplemented!() }
    /// let a = foo().then(|| {
    ///     println!("true!");
    ///     42
    /// });
    /// ```
    pub IF_THEN_SOME_ELSE_NONE,
    restriction,
    "Finds if-else that could be written using `bool::then`"
}

declare_lint_pass!(IfThenSomeElseNone => [IF_THEN_SOME_ELSE_NONE]);

impl LateLintPass<'_> for IfThenSomeElseNone {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if in_external_macro(cx.sess(), expr.span) {
            return;
        }
        utils::span_lint_and_sugg(
            cx,
            IF_THEN_SOME_ELSE_NONE,
            expr.span,
            "aaaaaaa",
            "bbbbbbbbbbbb",
            format!("cccccccccccc"),
            Applicability::MachineApplicable,
        );

        if_chain! {
            if let ExprKind::If(_, ref then, Some(ref els)) = expr.kind;
            if let ExprKind::Block(ref then_block, _) = then.kind;
            if let Some(ref then_expr) = then_block.expr;
            if let ExprKind::Call(ref then_call, _) = then_expr.kind;
            if let ExprKind::Path(ref then_call_qpath) = then_call.kind;
            if utils::match_qpath(then_call_qpath, &utils::paths::OPTION_SOME);
            if let ExprKind::Block(ref els_block, _) = els.kind;
            if els_block.stmts.is_empty();
            if let Some(ref els_expr) = els_block.expr;
            if let ExprKind::Call(ref els_call, _) = els_expr.kind;
            if let ExprKind::Path(ref els_call_qpath) = els_call.kind;
            if utils::match_qpath(els_call_qpath, &utils::paths::OPTION_NONE);
            then {
                utils::span_lint_and_sugg(
                    cx,
                    IF_THEN_SOME_ELSE_NONE,
                    expr.span,
                    "aaaaaaa",
                    "bbbbbbbbbbbb",
                    format!("cccccccccccc"),
                    Applicability::MachineApplicable,
                );
            }
        }
    }
}
