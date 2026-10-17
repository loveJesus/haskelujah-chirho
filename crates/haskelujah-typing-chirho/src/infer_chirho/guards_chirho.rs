// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typing guarded arms: a guarded right-hand side's and a multi-way if's.
//!
//! An arm's qualifiers run in order in a scope of the arm's own, each one's
//! bindings reaching the qualifiers after it and the body: a boolean guard is a
//! `Bool`, checked as an `if` condition is; a pattern guard binds its pattern
//! against its source's type as a case alternative binds against its scrutinee;
//! a `let` binds locally.
//!
//! Every arm's result is decided INSIDE the arm's scope, while the GADT
//! refinements its pattern guards proved still hold: `| Refl <- eq = value`
//! with `eq :: Equal a Bool` gives `value :: a` the type `Bool` there and
//! nowhere else. A result read after the arm's scope closed would have lost
//! the refinement, and a valid program would be rejected against its signature.
//!
//! Who judges an arm's result differs in one deliberate way. A multi-way if's
//! arms are an `if`'s branches, so a mismatch between them is an error here
//! (TcMultiWayIfFail). A guarded right-hand side with an expected type (an
//! equation's result, a case alternative's) hands an arm that cannot meet it
//! to its caller, which judges it as it judges an unguarded body. With no
//! expected type (a pattern binding, an ordinary case alternative) a later arm
//! that disagrees with the first is dropped and left to the signature check,
//! the leniency it has always had; changing that is its own measured unit.
//! workflow: language-features-chirho/pattern-matching-chirho

use haskelujah_ast_chirho::expr_chirho::{GuardedExprChirho, StmtChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

use super::InferCtxChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;

impl InferCtxChirho {
    /// A guarded right-hand side's arms with no expected type (a pattern
    /// binding, an ordinary case alternative): the first arm decides the type,
    /// and a later arm that disagrees is not reported.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn infer_guard_arms_chirho(
        &mut self,
        arms_chirho: &[GuardedExprChirho],
    ) -> (SubstChirho, TyChirho) {
        let result_ty_chirho = self.fresh_var_chirho();
        let (subst_chirho, _unmet_chirho) =
            self.infer_guard_arms_meeting_chirho(arms_chirho, &result_ty_chirho);
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
        (subst_chirho, final_ty_chirho)
    }

    /// The same arms where the enclosing equation or alternative expects
    /// `expected_chirho`. Every arm meets it inside its own scope. When an arm
    /// cannot, that arm's type is the result, so the caller judges the arm as
    /// it judges an unguarded body: an equation reports it, and a case
    /// alternative keeps its lenient merge. Nothing is reported here.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn infer_guard_arms_against_expected_chirho(
        &mut self,
        arms_chirho: &[GuardedExprChirho],
        expected_chirho: &TyChirho,
    ) -> (SubstChirho, TyChirho) {
        let (subst_chirho, unmet_chirho) =
            self.infer_guard_arms_meeting_chirho(arms_chirho, expected_chirho);
        let final_ty_chirho =
            subst_chirho.apply_ty_chirho(unmet_chirho.as_ref().unwrap_or(expected_chirho));
        (subst_chirho, final_ty_chirho)
    }

    /// Each arm's body meets `result_ty_chirho` inside the arm's own scope.
    /// Returns the substitution and, for the first arm that could not meet
    /// it, that arm's type as the arm saw it, refinements applied, so a
    /// mismatch is stated in the refined type, as GHC states it.
    fn infer_guard_arms_meeting_chirho(
        &mut self,
        arms_chirho: &[GuardedExprChirho],
        result_ty_chirho: &TyChirho,
    ) -> (SubstChirho, Option<TyChirho>) {
        let mut subst_chirho = SubstChirho::empty_chirho();
        let mut unmet_chirho: Option<TyChirho> = None;
        for arm_chirho in arms_chirho {
            self.env_chirho.push_scope_chirho();
            let body_ty_chirho = self.infer_guard_arm_chirho(arm_chirho, None, &mut subst_chirho);
            match self.unify_normalized_chirho(
                &subst_chirho.apply_ty_chirho(result_ty_chirho),
                &body_ty_chirho,
                arm_chirho.span_chirho,
            ) {
                Ok(result_subst_chirho) => {
                    subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&result_subst_chirho);
                }
                Err(_) => {
                    if unmet_chirho.is_none() {
                        unmet_chirho = Some(self.normalize_ty_chirho(&body_ty_chirho));
                    }
                }
            }
            self.env_chirho.pop_scope_chirho();
        }
        (subst_chirho, unmet_chirho)
    }

    /// A multi-way if: every arm's result is the expression's type, checked
    /// as an `if`'s branches are. With an `expected_chirho` type (a polymorphic
    /// one, as for `if`), each body is checked against it instead.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn infer_multiway_if_chirho(
        &mut self,
        arms_chirho: &[GuardedExprChirho],
        expected_chirho: Option<&TyChirho>,
    ) -> (SubstChirho, TyChirho) {
        let mut subst_chirho = SubstChirho::empty_chirho();
        let result_ty_chirho = match expected_chirho {
            Some(expected_chirho) => expected_chirho.clone(),
            None => self.fresh_var_chirho(),
        };
        for arm_chirho in arms_chirho {
            self.env_chirho.push_scope_chirho();
            let arm_expected_chirho =
                expected_chirho.map(|_| subst_chirho.apply_ty_chirho(&result_ty_chirho));
            let body_ty_chirho = self.infer_guard_arm_chirho(
                arm_chirho,
                arm_expected_chirho.as_ref(),
                &mut subst_chirho,
            );
            if expected_chirho.is_none() {
                match self.unify_normalized_chirho(
                    &subst_chirho.apply_ty_chirho(&result_ty_chirho),
                    &body_ty_chirho,
                    arm_chirho.span_chirho,
                ) {
                    Ok(result_subst_chirho) => {
                        subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&result_subst_chirho);
                    }
                    Err(err_chirho) => self.report_unify_error_chirho(&err_chirho),
                }
            }
            self.env_chirho.pop_scope_chirho();
        }
        let final_ty_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
        (subst_chirho, final_ty_chirho)
    }

    /// One arm in the CURRENT scope (the caller opens the arm's own): its
    /// qualifiers, then its body, against `expected_chirho` when there is one.
    /// Returns the body's type under the substitution so far.
    fn infer_guard_arm_chirho(
        &mut self,
        arm_chirho: &GuardedExprChirho,
        expected_chirho: Option<&TyChirho>,
        subst_chirho: &mut SubstChirho,
    ) -> TyChirho {
        self.infer_guard_quals_chirho(
            &arm_chirho.quals_chirho,
            arm_chirho.span_chirho,
            subst_chirho,
        );
        let (body_subst_chirho, body_ty_chirho) = match expected_chirho {
            Some(expected_chirho) => self.infer_expr_against_expected_chirho(
                &arm_chirho.body_chirho,
                expected_chirho,
                arm_chirho.span_chirho,
            ),
            None => self.infer_expr_chirho(&arm_chirho.body_chirho),
        };
        *subst_chirho = body_subst_chirho.compose_chirho(subst_chirho);
        self.apply_subst_all_chirho(&body_subst_chirho);
        subst_chirho.apply_ty_chirho(&body_ty_chirho)
    }

    /// An arm's qualifiers, in order, in the current scope.
    fn infer_guard_quals_chirho(
        &mut self,
        quals_chirho: &[StmtChirho],
        span_chirho: SpanChirho,
        subst_chirho: &mut SubstChirho,
    ) {
        for qual_chirho in quals_chirho {
            match qual_chirho {
                StmtChirho::ExprChirho { expr_chirho, .. } => {
                    let (guard_subst_chirho, guard_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                    *subst_chirho = guard_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&guard_subst_chirho);
                    let guard_ty_sub_chirho = subst_chirho.apply_ty_chirho(&guard_ty_chirho);
                    match self.unify_normalized_chirho(
                        &guard_ty_sub_chirho,
                        &TyChirho::bool_chirho(),
                        span_chirho,
                    ) {
                        Ok(bool_subst_chirho) => {
                            *subst_chirho = bool_subst_chirho.compose_chirho(subst_chirho);
                            self.apply_subst_all_chirho(&bool_subst_chirho);
                            self.report_deferred_numeric_bool_predicates_chirho();
                        }
                        Err(err_chirho) => self.report_unify_error_chirho(&err_chirho),
                    }
                }
                StmtChirho::BindChirho {
                    pat_chirho,
                    expr_chirho,
                    ..
                } => {
                    let (source_subst_chirho, source_ty_chirho) =
                        self.infer_expr_chirho(expr_chirho);
                    *subst_chirho = source_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&source_subst_chirho);
                    self.bind_pat_against_chirho(
                        pat_chirho,
                        &source_ty_chirho,
                        span_chirho,
                        subst_chirho,
                    );
                }
                StmtChirho::LetChirho { binds_chirho, .. } => {
                    self.infer_local_binds_chirho(binds_chirho, subst_chirho);
                }
            }
        }
    }

    /// Bind `pat_chirho` against the type of what it matches, in the current
    /// scope: a case alternative against its scrutinee, a pattern guard against
    /// its source. The pattern's type is unified with the scrutinee's; when they
    /// differ, a GADT pattern may refine the scrutinee instead.
    /// workflow: language-features-chirho/pattern-matching-chirho
    pub(super) fn bind_pat_against_chirho(
        &mut self,
        pat_chirho: &PatChirho,
        scrut_ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
        subst_chirho: &mut SubstChirho,
    ) {
        let pat_ty_chirho = self.fresh_var_chirho();
        let pat_subst_chirho = self.bind_pat_chirho(pat_chirho, &pat_ty_chirho);
        *subst_chirho = pat_subst_chirho.compose_chirho(subst_chirho);
        self.apply_subst_all_chirho(&pat_subst_chirho);

        let pat_ty_sub_chirho = subst_chirho.apply_ty_chirho(&pat_ty_chirho);
        let scrut_ty_sub_chirho = subst_chirho.apply_ty_chirho(scrut_ty_chirho);
        match self.unify_normalized_chirho(&pat_ty_sub_chirho, &scrut_ty_sub_chirho, span_chirho) {
            Ok(unify_subst_chirho) => {
                *subst_chirho = unify_subst_chirho.compose_chirho(subst_chirho);
                self.apply_subst_all_chirho(&unify_subst_chirho);
            }
            Err(err_chirho) => {
                if let Some(unify_subst_chirho) = self.refine_scrutinee_by_pattern_chirho(
                    pat_chirho,
                    &pat_ty_sub_chirho,
                    &scrut_ty_sub_chirho,
                ) {
                    *subst_chirho = unify_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&unify_subst_chirho);
                } else {
                    self.report_unify_error_chirho(&err_chirho);
                }
            }
        }
    }
}
