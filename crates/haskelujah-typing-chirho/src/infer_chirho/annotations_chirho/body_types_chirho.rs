// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Check body type syntax with the same module kind solver, after the ordinary
//! implication has opened its actual universals and pattern/signature givens.

use super::*;
use crate::kind_chirho::elaboration_chirho::{
    ArgumentElaborationContextChirho, CheckedBodyTypeChirho, ElaborationErrorChirho,
    elaborate_body_pattern_type_chirho, elaborate_body_type_chirho,
    elaborate_value_parameters_chirho, normalize_value_parameter_equalities_chirho,
};

#[cfg(test)]
#[path = "body_scope_tests_chirho.rs"]
mod scope_tests_chirho;

pub(in crate::infer_chirho) struct ValueParameterScopeChirho {
    refinements_chirho: crate::env_chirho::RefinementMarkChirho,
    evidence_chirho: super::super::equality_scopes_chirho::EqualityEvidenceSnapshotChirho,
}

impl InferCtxChirho {
    /// Keep the original refinement prefix and add only actual written
    /// equalities normalized through the previously admitted entry evidence.
    pub(in crate::infer_chirho) fn extend_value_parameter_scope_chirho(
        &mut self,
        entry_chirho: &ValueParameterScopeChirho,
        predicates_chirho: &[crate::class_chirho::PredChirho],
        span_chirho: SpanChirho,
    ) -> Option<ValueParameterScopeChirho> {
        if let Err(error_chirho) = self.elaboration_state_chirho.charge_chirho(
            predicates_chirho.len(),
            predicates_chirho.len(),
            span_chirho,
        ) {
            self.record_body_kind_error_chirho(error_chirho);
            return None;
        }
        let equalities_chirho: Vec<_> = predicates_chirho
            .iter()
            .filter(|predicate_chirho| predicate_chirho.class_name_chirho == "~")
            .filter_map(|predicate_chirho| {
                predicate_chirho
                    .extra_tys_chirho
                    .first()
                    .map(|right_chirho| (&predicate_chirho.ty_chirho, right_chirho))
            })
            .collect();
        let normalized_chirho = self.check_body_syntax_at_scope_chirho(
            span_chirho,
            Some(entry_chirho),
            |context_chirho| {
                normalize_value_parameter_equalities_chirho(
                    &equalities_chirho,
                    span_chirho,
                    context_chirho,
                )
            },
        )?;
        let rules_chirho: Vec<_> = normalized_chirho
            .into_iter()
            .filter_map(|(left_chirho, right_chirho)| {
                self.given_equality_rule_chirho(left_chirho, right_chirho)
            })
            .collect();
        let allocation_chirho = usize::from(!rules_chirho.is_empty());
        if let Err(error_chirho) = self.elaboration_state_chirho.charge_chirho(
            allocation_chirho,
            allocation_chirho,
            span_chirho,
        ) {
            self.record_body_kind_error_chirho(error_chirho);
            return None;
        }
        Some(ValueParameterScopeChirho {
            refinements_chirho: entry_chirho.refinements_chirho,
            evidence_chirho: entry_chirho
                .evidence_chirho
                .with_rewrites_chirho(rules_chirho),
        })
    }

    pub(in crate::infer_chirho) fn value_parameter_scope_chirho(
        &self,
    ) -> Option<ValueParameterScopeChirho> {
        self.elaboration_state_chirho.owner_chirho?;
        Some(ValueParameterScopeChirho {
            refinements_chirho: self.env_chirho.refinement_mark_chirho(),
            evidence_chirho: self.equality_scopes_chirho[self.active_equality_scope_chirho]
                .evidence_snapshot_chirho(),
        })
    }

    /// Ordinary solving can expose a previously unknown parameter kind. Read
    /// its solution while retaining exactly the evidence available at entry.
    pub(in crate::infer_chirho) fn check_solved_value_parameters_chirho(
        &mut self,
        entry_chirho: Option<&ValueParameterScopeChirho>,
        types_chirho: &[TyChirho],
        span_chirho: SpanChirho,
    ) {
        let Some(entry_chirho) = entry_chirho else {
            return;
        };
        let copied_chirho = (|| {
            self.elaboration_state_chirho.charge_chirho(
                types_chirho.len(),
                types_chirho.len(),
                span_chirho,
            )?;
            types_chirho
                .iter()
                .map(|term_chirho| {
                    self.elaboration_state_chirho
                        .copy_chirho(term_chirho, span_chirho)
                        .map(|term_chirho| (term_chirho, span_chirho))
                })
                .collect::<Result<Vec<_>, _>>()
        })();
        let entries_chirho = match copied_chirho {
            Ok(entries_chirho) => entries_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return;
            }
        };
        self.check_body_syntax_at_scope_chirho(span_chirho, Some(entry_chirho), |context_chirho| {
            elaborate_value_parameters_chirho(&entries_chirho, context_chirho)
        });
    }
    /// Inspect the whole actual parameter list before any of its patterns can
    /// add givens. Only enclosing implications authorize representation refinement.
    pub(in crate::infer_chirho) fn check_value_parameters_chirho(
        &mut self,
        patterns_chirho: &[haskelujah_ast_chirho::pat_chirho::PatChirho],
        types_chirho: &[TyChirho],
        span_chirho: SpanChirho,
    ) {
        use haskelujah_ast_chirho::pat_chirho::PatChirho;
        if self.elaboration_state_chirho.owner_chirho.is_none() {
            return;
        }
        let copied_chirho = (|| {
            self.elaboration_state_chirho.charge_chirho(
                types_chirho.len(),
                types_chirho.len(),
                span_chirho,
            )?;
            types_chirho
                .iter()
                .enumerate()
                .map(|(index_chirho, term_chirho)| {
                    let span_chirho = patterns_chirho
                        .get(index_chirho)
                        .map_or(span_chirho, PatChirho::span_chirho);
                    self.elaboration_state_chirho
                        .copy_chirho(term_chirho, span_chirho)
                        .map(|term_chirho| (term_chirho, span_chirho))
                })
                .collect::<Result<Vec<_>, _>>()
        })();
        let mut entries_chirho = match copied_chirho {
            Ok(entries_chirho) => entries_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return;
            }
        };
        for pattern_chirho in patterns_chirho {
            let mut pattern_chirho = pattern_chirho;
            for depth_chirho in 0..128 {
                if let Err(error_chirho) =
                    self.elaboration_state_chirho
                        .charge_chirho(1, 0, pattern_chirho.span_chirho())
                {
                    self.record_body_kind_error_chirho(error_chirho);
                    return;
                }
                pattern_chirho = match pattern_chirho {
                    PatChirho::ParenChirho { inner_chirho, .. }
                    | PatChirho::BangChirho { inner_chirho, .. }
                    | PatChirho::LazyChirho { inner_chirho, .. }
                    | PatChirho::AsChirho {
                        pattern_chirho: inner_chirho,
                        ..
                    } => inner_chirho,
                    PatChirho::TypeAnnotChirho {
                        pat_chirho: inner_chirho,
                        ty_chirho,
                        span_chirho,
                    } => {
                        if let Some(checked_chirho) = self.check_body_type_chirho(ty_chirho, true) {
                            entries_chirho.push((checked_chirho.term_chirho, *span_chirho));
                        }
                        inner_chirho
                    }
                    _ => break,
                };
                if depth_chirho == 127 {
                    let error_chirho = self.elaboration_state_chirho.resource_failure_chirho(
                        "value parameter pattern depth bound exhausted",
                        pattern_chirho.span_chirho(),
                    );
                    self.record_body_kind_error_chirho(error_chirho);
                    return;
                }
            }
        }
        self.check_body_syntax_chirho(span_chirho, |context_chirho| {
            elaborate_value_parameters_chirho(&entries_chirho, context_chirho)
        });
    }

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
        self.check_body_syntax_at_scope_chirho(span_chirho, None, elaborate_chirho)
    }

    fn check_body_syntax_at_scope_chirho<ResultChirho>(
        &mut self,
        span_chirho: SpanChirho,
        entry_chirho: Option<&ValueParameterScopeChirho>,
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
            refinement_mark_chirho: entry_chirho
                .map(|entry_chirho| entry_chirho.refinements_chirho),
            equality_chirho: super::super::EqualityNormalizationViewChirho {
                substitutions_chirho: &self.equality_substitutions_chirho,
                legacy_synonyms_chirho: &self.type_synonyms_chirho,
                type_scope_chirho: Some(&type_scope_chirho),
                family_scope_chirho: Some(&family_scope_chirho),
                family_module_chirho: &self.family_module_name_chirho,
                promotion_present_chirho: true,
                evidence_chirho: entry_chirho.map_or_else(
                    || {
                        self.equality_scopes_chirho[self.active_equality_scope_chirho]
                            .evidence_view_chirho()
                    },
                    |entry_chirho| entry_chirho.evidence_chirho.view_chirho(),
                ),
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
