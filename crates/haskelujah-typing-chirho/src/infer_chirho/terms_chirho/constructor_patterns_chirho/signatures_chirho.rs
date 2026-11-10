// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pattern signatures reuse the implication's checked aliases and lexical undo.
//! workflow: type-identity-chirho/committed-meta-kinds-chirho

use super::*;

#[cfg(test)]
#[path = "signature_origin_tests_chirho.rs"]
mod signature_origin_tests_chirho;

impl InferCtxChirho {
    pub(in crate::infer_chirho) fn prepare_pattern_signature_chirho(
        &mut self,
        annotation_chirho: &TypeChirho,
        span_chirho: SpanChirho,
    ) -> bool {
        if !self.scoped_type_variables_chirho
            || self.elaboration_state_chirho.owner_chirho.is_none()
        {
            return true;
        }
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return false;
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let mark_chirho = self.type_pattern_alias_undo_chirho.len();
        let prepared_chirho = (|| {
            let mut seen_chirho = HashSet::new();
            for variable_chirho in self.scoped_tyvars_chirho.values() {
                budget_chirho.node_chirho(1)?;
                let source_chirho = self
                    .skolem_bindings_chirho
                    .lookup_image_chirho(variable_chirho)
                    .and_then(|image_chirho| image_chirho.source_chirho.as_ref())
                    .or_else(|| {
                        self.elaboration_state_chirho
                            .variable_kinds_chirho
                            .get(variable_chirho)
                            .and_then(|record_chirho| record_chirho.source_chirho.as_ref())
                    });
                let Some(source_chirho) = source_chirho else {
                    // An unproved enclosing scope cannot authorize rebinding.
                    // Preserve its old path without installing any new aliases.
                    return Some(false);
                };
                if let haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho {
                    declaration_chirho, ..
                } = source_chirho {
                    budget_chirho.work_chirho(declaration_chirho.len())?;
                }
                seen_chirho.insert(source_chirho.clone());
            }
            let enclosing_count_chirho = seen_chirho.len();
            let mut unknown_origin_chirho = false;
            self.prepare_type_pattern_variables_chirho(
                annotation_chirho,
                &mut seen_chirho,
                !self.checking_binding_pattern_annotations_chirho,
                &mut unknown_origin_chirho,
                &mut budget_chirho,
                0,
            )?;
            if unknown_origin_chirho {
                self.restore_type_pattern_aliases_chirho(mark_chirho);
                return Some(false);
            }
            if self.checking_binding_pattern_annotations_chirho {
                return Some(seen_chirho.len() != enclosing_count_chirho);
            }
            self.complete_type_pattern_kinds_chirho(mark_chirho, &mut budget_chirho)?;
            Some(false)
        })();
        if let Err(error_chirho) = self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            prepared_chirho.is_none(),
            span_chirho,
        ) {
            self.restore_type_pattern_aliases_chirho(mark_chirho);
            self.type_normalization_exhausted_chirho.set(true);
            self.record_body_kind_error_chirho(error_chirho);
            return false;
        }
        if prepared_chirho == Some(true) {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(
                        super::super::pattern_bindings_chirho::PATTERN_SIGNATURE_CODE_CHIRHO,
                    ),
                    "a pattern binding signature cannot introduce a scoped type variable",
                    span_chirho,
                ));
            return false;
        }
        true
    }
}
