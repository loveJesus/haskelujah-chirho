// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One body tail for equations and lambdas; implication owners finish outside.

use super::*;
use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;

pub(super) enum BodyOrderChirho {
    ExpectedFirstChirho,
    RhsFirstChirho,
}

impl InferCtxChirho {
    pub(super) fn check_pattern_body_chirho(
        &mut self,
        where_chirho: &[LocalBindChirho],
        types_chirho: (&TyChirho, &TyChirho),
        mut substitution_chirho: SubstChirho,
        order_chirho: BodyOrderChirho,
        body_chirho: &mut impl FnMut(&mut Self, &TyChirho) -> (SubstChirho, TyChirho),
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let (expected_chirho, result_chirho) = types_chirho;
        self.infer_local_binds_chirho(where_chirho, &mut substitution_chirho);
        let expected_chirho = substitution_chirho.apply_ty_chirho(expected_chirho);
        let (mut step_chirho, body_ty_chirho) = body_chirho(self, &expected_chirho);
        if self.apply_subst_all_chirho(&mut step_chirho).is_err() {
            return SubstChirho::empty_chirho();
        };
        substitution_chirho = step_chirho.compose_chirho(&substitution_chirho);
        let result_chirho = substitution_chirho.apply_ty_chirho(result_chirho);
        let (left_chirho, right_chirho) = match order_chirho {
            BodyOrderChirho::ExpectedFirstChirho => (&result_chirho, &body_ty_chirho),
            BodyOrderChirho::RhsFirstChirho => (&body_ty_chirho, &result_chirho),
        };
        match self.unify_normalized_chirho(left_chirho, right_chirho, span_chirho) {
            Ok(mut step_chirho) => {
                if self.apply_subst_all_chirho(&mut step_chirho).is_err() {
                    return SubstChirho::empty_chirho();
                };
                substitution_chirho = step_chirho.compose_chirho(&substitution_chirho);
            }
            Err(error_chirho) => self.report_unify_error_chirho(&error_chirho),
        }
        substitution_chirho
    }
}
