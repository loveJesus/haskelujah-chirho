// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Check an expression's annotation universally, then instantiate its outer
//! scheme at ordinary uses. Nested quantifiers remain in the checked body.

use super::InferCtxChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

mod escape_chirho;
#[cfg(test)]
mod tests_chirho;

impl InferCtxChirho {
    /// The producer returns the checked sigma, so visible type application can
    /// consume its quantifiers before ordinary instantiation erases the prefix.
    pub(super) fn check_expression_annotation_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
        annotation_chirho: &TypeChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, SchemeChirho) {
        let (scheme_chirho, variables_chirho) =
            self.ast_type_to_scheme_with_var_map_chirho(annotation_chirho);
        let saved_scope_chirho = self.scoped_tyvars_chirho.clone();
        self.scope_signature_for_body_chirho(annotation_chirho, &variables_chirho);
        let ((substitution_chirho, _), public_chirho) = self.with_rigid_signature_chirho(
            &scheme_chirho,
            span_chirho,
            |context_chirho, expected_chirho| {
                context_chirho.infer_expr_against_expected_chirho(
                    expression_chirho,
                    expected_chirho,
                    span_chirho,
                )
            },
        );
        self.scoped_tyvars_chirho = saved_scope_chirho;
        let substitution_chirho =
            self.publish_signature_solution_chirho(substitution_chirho, &public_chirho);
        let checked_chirho = substitution_chirho.apply_scheme_chirho(&scheme_chirho);
        (substitution_chirho, checked_chirho)
    }

    /// Each expected sigma checks escape before restoring its own rigid names.
    /// Nested checking cannot hide a committed outer-meta solution by dropping
    /// its temporary skolem map before its parent's scope check.
    pub(super) fn with_rigid_signature_chirho<ResultChirho>(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
        check_chirho: impl FnOnce(&mut Self, &TyChirho) -> ResultChirho,
    ) -> (ResultChirho, SubstChirho) {
        let saved_skolems_chirho = self.skolem_bindings_chirho.clone();
        let (expected_chirho, givens_chirho) = self.skolemize_scheme_parts_chirho(scheme_chirho);
        let local_skolems_chirho: std::collections::HashSet<String> = scheme_chirho
            .vars_chirho
            .iter()
            .filter_map(|variable_chirho| {
                if saved_skolems_chirho
                    .lookup_chirho(variable_chirho)
                    .is_some()
                {
                    return None;
                }
                match self.skolem_bindings_chirho.lookup_chirho(variable_chirho) {
                    Some(TyChirho::ForallVarChirho(name_chirho)) => Some(name_chirho.clone()),
                    _ => None,
                }
            })
            .collect();
        let captured_chirho = if local_skolems_chirho.is_empty() {
            None
        } else {
            Some(self.capture_signature_metas_chirho(scheme_chirho))
        };
        let mut public_chirho = SubstChirho::empty_chirho();
        let result_chirho = self.with_signature_givens_and_finish_chirho(
            givens_chirho,
            &expected_chirho,
            |context_chirho| check_chirho(context_chirho, &expected_chirho),
            |context_chirho| {
                if let Some(captured_chirho) = captured_chirho {
                    public_chirho = context_chirho.check_annotation_escape_chirho(
                        captured_chirho,
                        &local_skolems_chirho,
                        span_chirho,
                    );
                }
            },
        );
        self.skolem_bindings_chirho = saved_skolems_chirho;
        (result_chirho, public_chirho)
    }

    /// A safe public image replaces a scope-local image; it is a domain update
    /// as well as substitution composition, just like pattern implication exit.
    pub(super) fn publish_signature_solution_chirho(
        &mut self,
        checked_chirho: SubstChirho,
        public_chirho: &SubstChirho,
    ) -> SubstChirho {
        let mut combined_chirho = public_chirho.compose_chirho(&checked_chirho);
        for (variable_chirho, image_chirho) in public_chirho.iter_chirho() {
            combined_chirho.insert_chirho(*variable_chirho, image_chirho.clone());
        }
        self.apply_subst_all_chirho(public_chirho);
        combined_chirho
    }
}
