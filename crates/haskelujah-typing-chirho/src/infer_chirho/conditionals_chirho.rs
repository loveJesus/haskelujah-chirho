// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typing `if c then t else e`: the condition is a `Bool`, and both branches
//! have the expression's type.
//!
//! Ordinary inference infers the condition and unifies it with `Bool`, then
//! infers both branches and unifies them. When the expected type has a
//! polymorphic component (a rank-N argument or result), the condition is
//! checked against `Bool` and each branch against the expected type instead, so
//! a branch lambda's parameter takes the polymorphic type the signature gives.
//! workflow: language-features-chirho/conditionals-chirho

use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_span_chirho::SpanChirho;

use super::InferCtxChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;

impl InferCtxChirho {
    /// `if` by ordinary inference.
    /// workflow: language-features-chirho/conditionals-chirho
    pub(super) fn infer_if_chirho(
        &mut self,
        cond_chirho: &ExprChirho,
        then_chirho: &ExprChirho,
        else_chirho: &ExprChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        let (s1_chirho, cond_ty_chirho) = self.infer_expr_chirho(cond_chirho);
        self.apply_subst_all_chirho(&s1_chirho);

        // Condition must be Bool
        match self.unify_normalized_chirho(&cond_ty_chirho, &TyChirho::bool_chirho(), span_chirho) {
            Ok(sc_chirho) => {
                self.apply_subst_all_chirho(&sc_chirho);
                // After unifying condition with Bool, eagerly check if
                // any deferred numeric predicate now targets Bool (e.g.
                // `if 42 then ...` → Num Bool). Generalization would
                // absorb this ground predicate silently; catch it here.
                self.report_deferred_numeric_bool_predicates_chirho();
            }
            Err(err_chirho) => {
                self.report_unify_error_chirho(&err_chirho);
            }
        }

        let (s2_chirho, then_ty_chirho) = self.infer_expr_chirho(then_chirho);
        self.apply_subst_all_chirho(&s2_chirho);

        let (s3_chirho, else_ty_chirho) = self.infer_expr_chirho(else_chirho);

        // Then and else branches must have the same type
        let then_sub_chirho = s3_chirho.apply_ty_chirho(&then_ty_chirho);
        match self.unify_normalized_chirho(&then_sub_chirho, &else_ty_chirho, span_chirho) {
            Ok(s4_chirho) => {
                let combined_chirho = s4_chirho.compose_chirho(
                    &s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho)),
                );
                let final_ty_chirho = combined_chirho.apply_ty_chirho(&else_ty_chirho);
                self.apply_subst_all_chirho(&s4_chirho);
                (combined_chirho, final_ty_chirho)
            }
            Err(err_chirho) => {
                self.report_unify_error_chirho(&err_chirho);
                let combined_chirho =
                    s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                (combined_chirho, else_ty_chirho)
            }
        }
    }

    /// `if` checked against an expected type with a polymorphic component.
    /// workflow: language-features-chirho/rank-n-visible-type-application-chirho
    pub(super) fn check_if_against_expected_chirho(
        &mut self,
        cond_chirho: &ExprChirho,
        then_chirho: &ExprChirho,
        else_chirho: &ExprChirho,
        if_span_chirho: SpanChirho,
        expected_ty_chirho: &TyChirho,
    ) -> (SubstChirho, TyChirho) {
        // Polymorphic branch results require bidirectional checking:
        // ordinary inference would bind each branch lambda parameter
        // monomorphically before the signature can reach it.
        // workflow: language-features-chirho/rank-n-visible-type-application-chirho
        let bool_ty_chirho = TyChirho::bool_chirho();
        let (cond_subst_chirho, _) =
            self.infer_expr_against_expected_chirho(cond_chirho, &bool_ty_chirho, if_span_chirho);
        let mut subst_chirho = cond_subst_chirho;
        self.apply_subst_all_chirho(&subst_chirho);
        self.report_deferred_numeric_bool_predicates_chirho();

        let then_expected_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
        let (then_subst_chirho, _) = self.infer_expr_against_expected_chirho(
            then_chirho,
            &then_expected_chirho,
            if_span_chirho,
        );
        subst_chirho = then_subst_chirho.compose_chirho(&subst_chirho);
        self.apply_subst_all_chirho(&then_subst_chirho);

        let else_expected_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
        let (else_subst_chirho, _) = self.infer_expr_against_expected_chirho(
            else_chirho,
            &else_expected_chirho,
            if_span_chirho,
        );
        subst_chirho = else_subst_chirho.compose_chirho(&subst_chirho);
        self.apply_subst_all_chirho(&else_subst_chirho);

        let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
        (subst_chirho, final_ty_chirho)
    }
}
