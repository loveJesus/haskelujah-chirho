// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Check body type syntax with the same module kind solver, after the ordinary
//! implication has opened its actual universals and pattern/signature givens.

use super::*;
use crate::kind_chirho::elaboration_chirho::{
    ArgumentElaborationContextChirho, CheckedBodyTypeChirho, ElaborationErrorChirho,
    elaborate_body_pattern_type_chirho, elaborate_body_type_chirho,
};

impl InferCtxChirho {
    pub(in crate::infer_chirho) fn check_body_type_argument_chirho(
        &mut self,
        syntax_chirho: &TypeChirho,
    ) -> (TyChirho, Option<TyChirho>) {
        if let Some(checked_chirho) = self.check_body_type_chirho(syntax_chirho, false) {
            return (checked_chirho.term_chirho, Some(checked_chirho.kind_chirho));
        }
        let kind_chirho = self.checked_type_argument_kind_chirho(syntax_chirho);
        (
            self.ast_type_to_ty_in_scope_chirho(syntax_chirho),
            kind_chirho,
        )
    }

    pub(in crate::infer_chirho) fn record_body_kind_error_chirho(
        &mut self,
        error_chirho: ElaborationErrorChirho,
    ) {
        let scoped_chirho = (|| {
            let Some(sides_chirho) = error_chirho.mismatch_chirho.as_ref() else {
                return Ok(false);
            };
            for side_chirho in [&sides_chirho.0, &sides_chirho.1] {
                let TyChirho::ForallVarChirho(name_chirho) = side_chirho else {
                    continue;
                };
                for (variable_chirho, term_chirho) in self.skolem_bindings_chirho.iter_chirho() {
                    let TyChirho::ForallVarChirho(actual_chirho) = term_chirho else {
                        continue;
                    };
                    self.elaboration_state_chirho.charge_chirho(
                        name_chirho
                            .len()
                            .saturating_add(actual_chirho.len())
                            .saturating_add(1),
                        0,
                        error_chirho.span_chirho,
                    )?;
                    if actual_chirho == name_chirho
                        && self
                            .skolem_bindings_chirho
                            .lookup_image_chirho(variable_chirho)
                            .is_some_and(|image_chirho| {
                                image_chirho.kind_chirho.is_some()
                                    && image_chirho.source_chirho.is_some()
                            })
                    {
                        return Ok(true);
                    }
                }
            }
            Ok::<_, ElaborationErrorChirho>(false)
        })();
        let scoped_chirho = match scoped_chirho {
            Ok(scoped_chirho) => scoped_chirho,
            Err(failure_chirho) => {
                self.non_deferrable_diagnostics_chirho
                    .push_chirho(failure_chirho.into_diagnostic_chirho());
                return;
            }
        };
        if scoped_chirho {
            self.diagnostics_chirho.push_chirho(
                haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                    haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(
                        super::super::TYPE_MISMATCH_CODE_CHIRHO,
                    ),
                    error_chirho.message_chirho,
                    error_chirho.span_chirho,
                ),
            );
        } else {
            self.non_deferrable_diagnostics_chirho
                .push_chirho(error_chirho.into_diagnostic_chirho());
        }
    }

    pub(in crate::infer_chirho) fn check_body_type_chirho(
        &mut self,
        syntax_chirho: &TypeChirho,
        value_kind_chirho: bool,
    ) -> Option<CheckedBodyTypeChirho> {
        self.check_body_syntax_chirho(syntax_chirho.span_chirho(), |context_chirho| {
            elaborate_body_type_chirho(syntax_chirho, value_kind_chirho, context_chirho)
        })
    }

    pub(in crate::infer_chirho) fn check_body_pattern_type_chirho(
        &mut self,
        syntax_chirho: &TypeChirho,
        expected_kind_chirho: &TyChirho,
    ) -> Option<CheckedBodyTypeChirho> {
        self.check_body_syntax_chirho(syntax_chirho.span_chirho(), |context_chirho| {
            elaborate_body_pattern_type_chirho(syntax_chirho, expected_kind_chirho, context_chirho)
        })
    }

    fn check_body_syntax_chirho<ResultChirho>(
        &mut self,
        span_chirho: SpanChirho,
        elaborate_chirho: impl FnOnce(
            ArgumentElaborationContextChirho<'_>,
        )
            -> Result<(ResultChirho, SubstChirho), ElaborationErrorChirho>,
    ) -> Option<ResultChirho> {
        self.elaboration_state_chirho.owner_chirho?;
        let (Some(type_scope_chirho), Some(family_scope_chirho), Some(promotion_scope_chirho)) = (
            self.type_constructor_scope_chirho.clone(),
            self.family_scope_chirho.clone(),
            self.promotion_scope_chirho.clone(),
        ) else {
            self.record_body_kind_error_chirho(ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: "body type lost its checked import scope".into(),
                span_chirho,
            });
            return None;
        };
        let result_chirho = elaborate_chirho(ArgumentElaborationContextChirho {
            state_chirho: &mut self.elaboration_state_chirho,
            registry_chirho: &mut self.type_families_chirho,
            type_scope_chirho: &type_scope_chirho,
            family_scope_chirho: &family_scope_chirho,
            promotion_scope_chirho: &promotion_scope_chirho,
            scoped_variables_chirho: &self.scoped_tyvars_chirho,
            images_chirho: &self.skolem_bindings_chirho,
            refinements_chirho: &self.env_chirho,
            equality_chirho: super::super::EqualityNormalizationViewChirho {
                substitutions_chirho: &self.equality_substitutions_chirho,
                legacy_synonyms_chirho: &self.type_synonyms_chirho,
                type_scope_chirho: Some(&type_scope_chirho),
                family_scope_chirho: Some(&family_scope_chirho),
                family_module_chirho: &self.family_module_name_chirho,
                promotion_present_chirho: true,
                evidence_chirho: self.equality_scopes_chirho[self.active_equality_scope_chirho]
                    .evidence_view_chirho(),
            },
        });
        match result_chirho {
            Ok((checked_chirho, links_chirho)) => {
                self.apply_subst_all_chirho(&links_chirho);
                Some(checked_chirho)
            }
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                None
            }
        }
    }
}
