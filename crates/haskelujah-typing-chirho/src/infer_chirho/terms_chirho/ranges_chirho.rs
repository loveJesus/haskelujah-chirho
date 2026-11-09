// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Range syntax selects a standard Enum method, even under RebindableSyntax.
//! Endpoint expressions retain their usual lexical and literal rules.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::InferCtxChirho;
use super::builtins_chirho::enumeration_scheme_chirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    /// Infer every endpoint at the same element type and record the selected Enum
    /// method's obligation under the range's span. No source binding selects it.
    pub(super) fn infer_range_chirho(
        &mut self,
        from_chirho: &ExprChirho,
        then_chirho: Option<&ExprChirho>,
        to_chirho: Option<&ExprChirho>,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        let method_chirho = match (then_chirho, to_chirho) {
            (None, None) => "enumFrom",
            (Some(_), None) => "enumFromThen",
            (None, Some(_)) => "enumFromTo",
            (Some(_), Some(_)) => "enumFromThenTo",
        };
        let scheme_chirho = enumeration_scheme_chirho(method_chirho)
            .expect("range forms select a standard Enum enumeration");
        let (_method_ty_chirho, preds_chirho) =
            self.instantiate_scheme_parts_chirho(&scheme_chirho);
        let elem_chirho = preds_chirho[0].ty_chirho.clone();
        self.capture_reference_evidence_chirho(span_chirho, None, &preds_chirho);
        for pred_chirho in preds_chirho {
            self.defer_pred_chirho(pred_chirho, span_chirho);
        }
        let mut subst_chirho = SubstChirho::empty_chirho();
        for endpoint_chirho in [Some(from_chirho), then_chirho, to_chirho]
            .into_iter()
            .flatten()
        {
            let (mut endpoint_subst_chirho, endpoint_ty_chirho) =
                self.infer_expr_chirho(endpoint_chirho);
            if self
                .apply_subst_all_chirho(&mut endpoint_subst_chirho)
                .is_err()
            {
                return (
                    SubstChirho::empty_chirho(),
                    Self::terminal_commit_type_chirho(),
                );
            };
            subst_chirho = endpoint_subst_chirho.compose_chirho(&subst_chirho);
            match self.unify_normalized_chirho(
                &subst_chirho.apply_ty_chirho(&elem_chirho),
                &subst_chirho.apply_ty_chirho(&endpoint_ty_chirho),
                endpoint_chirho.span_chirho(),
            ) {
                Ok(mut unifier_chirho) => {
                    if self.apply_subst_all_chirho(&mut unifier_chirho).is_err() {
                        return (
                            SubstChirho::empty_chirho(),
                            Self::terminal_commit_type_chirho(),
                        );
                    };
                    subst_chirho = unifier_chirho.compose_chirho(&subst_chirho);
                }
                Err(error_chirho) => self.report_unify_error_chirho(&error_chirho),
            }
        }
        let elem_chirho = subst_chirho.apply_ty_chirho(&elem_chirho);
        (subst_chirho, TyChirho::ListChirho(Box::new(elem_chirho)))
    }
}

#[cfg(test)]
#[path = "ranges_tests_chirho.rs"]
mod tests_chirho;
