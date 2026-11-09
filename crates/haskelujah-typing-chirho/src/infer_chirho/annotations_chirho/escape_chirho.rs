// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A checked annotation's rigid variables cannot solve a captured outer meta.
//! Inspect committed substitutions only; speculative unification trials grant
//! no scope ownership and must not mutate a dependency graph.

use super::super::equality_normalization_chirho::{
    EqualityBudgetChirho, copy_term_with_budget_chirho,
};
use super::super::{InferCtxChirho, TYPE_MISMATCH_CODE_CHIRHO};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashSet;

fn reaches_skolem_chirho(
    term_chirho: &TyChirho,
    substitutions_chirho: &SubstChirho,
    skolems_chirho: &HashSet<String>,
    bound_chirho: &mut Vec<TyVarChirho>,
    budget_chirho: &mut EqualityBudgetChirho,
    depth_chirho: usize,
) -> Option<bool> {
    budget_chirho.work_chirho(1)?;
    if depth_chirho > 128 {
        return None;
    }
    match term_chirho {
        TyChirho::VarChirho(variable_chirho) => {
            budget_chirho.work_chirho(bound_chirho.len())?;
            if !bound_chirho.contains(variable_chirho)
                && let Some(image_chirho) = substitutions_chirho.lookup_chirho(variable_chirho)
                && !matches!(image_chirho, TyChirho::VarChirho(other_chirho) if other_chirho == variable_chirho)
            {
                return reaches_skolem_chirho(
                    image_chirho,
                    substitutions_chirho,
                    skolems_chirho,
                    &mut Vec::new(),
                    budget_chirho,
                    depth_chirho + 1,
                );
            }
            Some(false)
        }
        TyChirho::ForallVarChirho(name_chirho) => {
            budget_chirho.work_chirho(name_chirho.len())?;
            Some(skolems_chirho.contains(name_chirho))
        }
        TyChirho::ConChirho(_) => Some(false),
        TyChirho::AppChirho(function_chirho, argument_chirho)
        | TyChirho::FunChirho(function_chirho, argument_chirho, _) => Some(
            reaches_skolem_chirho(
                function_chirho,
                substitutions_chirho,
                skolems_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )? || reaches_skolem_chirho(
                argument_chirho,
                substitutions_chirho,
                skolems_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?,
        ),
        TyChirho::TupleChirho(elements_chirho) => {
            for element_chirho in elements_chirho {
                if reaches_skolem_chirho(
                    element_chirho,
                    substitutions_chirho,
                    skolems_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )? {
                    return Some(true);
                }
            }
            Some(false)
        }
        TyChirho::ListChirho(element_chirho) => reaches_skolem_chirho(
            element_chirho,
            substitutions_chirho,
            skolems_chirho,
            bound_chirho,
            budget_chirho,
            depth_chirho + 1,
        ),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            budget_chirho.work_chirho(vars_chirho.len())?;
            let previous_chirho = bound_chirho.len();
            let result_chirho = (|| {
                for record_chirho in vars_chirho.records_chirho() {
                    if let Some(kind_chirho) = &record_chirho.kind_chirho
                        && reaches_skolem_chirho(
                            kind_chirho,
                            substitutions_chirho,
                            skolems_chirho,
                            bound_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )?
                    {
                        return Some(true);
                    }
                    bound_chirho.push(record_chirho.variable_chirho);
                }
                reaches_skolem_chirho(
                    body_chirho,
                    substitutions_chirho,
                    skolems_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )
            })();
            bound_chirho.truncate(previous_chirho);
            result_chirho
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            for predicate_chirho in context_chirho {
                if reaches_skolem_chirho(
                    predicate_chirho,
                    substitutions_chirho,
                    skolems_chirho,
                    bound_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )? {
                    return Some(true);
                }
            }
            reaches_skolem_chirho(
                body_chirho,
                substitutions_chirho,
                skolems_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            )
        }
    }
}

/// Captures actual surrounding dependencies, not every earlier temporary meta
/// created while preparing an application. Unused planning metas cannot escape.
pub(in crate::infer_chirho) struct SignatureCapturesChirho {
    variables_chirho: HashSet<TyVarChirho>,
    budget_chirho: EqualityBudgetChirho,
    kinds_chirho: Option<crate::kind_chirho::elaboration_chirho::CapturedVariableKindsChirho>,
}

impl SignatureCapturesChirho {
    pub(in crate::infer_chirho) fn into_kind_capture_chirho(
        self,
    ) -> Option<crate::kind_chirho::elaboration_chirho::CapturedVariableKindsChirho> {
        self.kinds_chirho
    }
}

fn collect_free_chirho(
    term_chirho: &TyChirho,
    bound_chirho: &mut Vec<TyVarChirho>,
    captured_chirho: &mut SignatureCapturesChirho,
    depth_chirho: usize,
) -> Option<()> {
    captured_chirho.budget_chirho.work_chirho(1)?;
    if depth_chirho > 128 {
        return None;
    }
    match term_chirho {
        TyChirho::VarChirho(variable_chirho) => {
            captured_chirho
                .budget_chirho
                .work_chirho(bound_chirho.len())?;
            if !bound_chirho.contains(variable_chirho) {
                captured_chirho.budget_chirho.node_chirho(1)?;
                captured_chirho.variables_chirho.insert(*variable_chirho);
            }
        }
        TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
        TyChirho::AppChirho(function_chirho, argument_chirho)
        | TyChirho::FunChirho(function_chirho, argument_chirho, _) => {
            collect_free_chirho(
                function_chirho,
                bound_chirho,
                captured_chirho,
                depth_chirho + 1,
            )?;
            collect_free_chirho(
                argument_chirho,
                bound_chirho,
                captured_chirho,
                depth_chirho + 1,
            )?;
        }
        TyChirho::ListChirho(element_chirho) => collect_free_chirho(
            element_chirho,
            bound_chirho,
            captured_chirho,
            depth_chirho + 1,
        )?,
        TyChirho::TupleChirho(elements_chirho) => {
            for element_chirho in elements_chirho {
                collect_free_chirho(
                    element_chirho,
                    bound_chirho,
                    captured_chirho,
                    depth_chirho + 1,
                )?;
            }
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            captured_chirho
                .budget_chirho
                .work_chirho(vars_chirho.len())?;
            let previous_chirho = bound_chirho.len();
            let result_chirho = (|| {
                for record_chirho in vars_chirho.records_chirho() {
                    if let Some(kind_chirho) = &record_chirho.kind_chirho {
                        collect_free_chirho(
                            kind_chirho,
                            bound_chirho,
                            captured_chirho,
                            depth_chirho + 1,
                        )?;
                    }
                    bound_chirho.push(record_chirho.variable_chirho);
                }
                collect_free_chirho(body_chirho, bound_chirho, captured_chirho, depth_chirho + 1)
            })();
            bound_chirho.truncate(previous_chirho);
            result_chirho?;
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            for predicate_chirho in context_chirho {
                collect_free_chirho(
                    predicate_chirho,
                    bound_chirho,
                    captured_chirho,
                    depth_chirho + 1,
                )?;
            }
            collect_free_chirho(body_chirho, bound_chirho, captured_chirho, depth_chirho + 1)?;
        }
    }
    Some(())
}

impl InferCtxChirho {
    #[cfg(test)]
    pub(super) fn capture_signature_metas_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> Option<SignatureCapturesChirho> {
        self.capture_scope_metas_chirho(Some(scheme_chirho), &[], SpanChirho::DUMMY_CHIRHO)
    }

    pub(in crate::infer_chirho) fn capture_scope_metas_chirho(
        &mut self,
        scheme_chirho: Option<&SchemeChirho>,
        additional_chirho: &[&TyChirho],
        span_chirho: SpanChirho,
    ) -> Option<SignatureCapturesChirho> {
        let mut captured_chirho = SignatureCapturesChirho {
            variables_chirho: HashSet::new(),
            budget_chirho: EqualityBudgetChirho::new_chirho(),
            kinds_chirho: None,
        };
        for entry_chirho in self
            .env_chirho
            .bindings_in_scope_order_chirho()
            .map(|(_, entry_chirho)| entry_chirho)
            .chain(scheme_chirho.map(Some))
        {
            captured_chirho.budget_chirho.work_chirho(1)?;
            let Some(entry_chirho) = entry_chirho else {
                continue;
            };
            captured_chirho
                .budget_chirho
                .work_chirho(entry_chirho.vars_chirho.len())?;
            let mut bound_chirho = Vec::new();
            for record_chirho in entry_chirho.vars_chirho.records_chirho() {
                if let Some(kind_chirho) = &record_chirho.kind_chirho {
                    collect_free_chirho(kind_chirho, &mut bound_chirho, &mut captured_chirho, 0)?;
                }
                bound_chirho.push(record_chirho.variable_chirho);
            }
            collect_free_chirho(
                &entry_chirho.ty_chirho,
                &mut bound_chirho,
                &mut captured_chirho,
                0,
            )?;
            for predicate_chirho in &entry_chirho.preds_chirho {
                for term_chirho in predicate_chirho
                    .kind_tys_chirho
                    .iter()
                    .chain(std::iter::once(&predicate_chirho.ty_chirho))
                    .chain(&predicate_chirho.extra_tys_chirho)
                {
                    collect_free_chirho(term_chirho, &mut bound_chirho, &mut captured_chirho, 0)?;
                }
            }
        }
        for term_chirho in self.env_chirho.refinement_types_chirho().chain(
            self.given_preds_chirho
                .iter()
                .chain(
                    self.deferred_preds_chirho
                        .iter()
                        .map(|(predicate_chirho, _)| predicate_chirho),
                )
                .flat_map(|predicate_chirho| {
                    predicate_chirho
                        .kind_tys_chirho
                        .iter()
                        .chain(std::iter::once(&predicate_chirho.ty_chirho))
                        .chain(&predicate_chirho.extra_tys_chirho)
                }),
        ) {
            collect_free_chirho(term_chirho, &mut Vec::new(), &mut captured_chirho, 0)?;
        }
        for term_chirho in additional_chirho {
            collect_free_chirho(term_chirho, &mut Vec::new(), &mut captured_chirho, 0)?;
        }
        if let Err(error_chirho) = self.elaboration_state_chirho.charge_chirho(
            captured_chirho.variables_chirho.len(),
            captured_chirho.variables_chirho.len(),
            span_chirho,
        ) {
            self.record_body_kind_error_chirho(error_chirho);
            return None;
        }
        let roots_chirho: Vec<_> = captured_chirho.variables_chirho.iter().copied().collect();
        let kinds_chirho = match self.elaboration_state_chirho.capture_variable_kinds_chirho(
            &roots_chirho,
            &self.equality_substitutions_chirho,
            span_chirho,
        ) {
            Ok(kinds_chirho) => kinds_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return None;
            }
        };
        for (variable_chirho, _) in kinds_chirho.kinds_chirho() {
            captured_chirho.budget_chirho.node_chirho(1)?;
            captured_chirho.variables_chirho.insert(variable_chirho);
        }
        captured_chirho.kinds_chirho = Some(kinds_chirho);
        Some(captured_chirho)
    }

    fn project_signature_term_chirho(
        &self,
        term_chirho: &TyChirho,
        variables_chirho: &crate::ty_chirho::QuantifiedVariablesChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<(crate::ty_chirho::QuantifiedVariablesChirho, TyChirho)> {
        if variables_chirho.is_empty() {
            return self
                .normalize_signature_public_image_chirho(term_chirho, budget_chirho)
                .ok()
                .map(|term_chirho| (variables_chirho.clone(), term_chirho));
        }
        budget_chirho.node_chirho(variables_chirho.len().saturating_add(1))?;
        let protected_chirho = TyChirho::ForallChirho {
            vars_chirho: variables_chirho.clone(),
            body_chirho: Box::new(copy_term_with_budget_chirho(term_chirho, budget_chirho)?),
        };
        match self
            .normalize_signature_public_image_chirho(&protected_chirho, budget_chirho)
            .ok()?
        {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } if vars_chirho.iter().eq(variables_chirho.iter()) => {
                Some((vars_chirho, *body_chirho))
            }
            _ => None,
        }
    }

    /// Replacing a numeric root alone cannot repair an environment entry that
    /// already contains its former rigid image. Project those affected schemes
    /// through the same checked evidence before it leaves scope, atomically.
    fn project_signature_environment_chirho(
        &self,
        skolems_chirho: &HashSet<String>,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<Vec<(usize, String, SchemeChirho)>> {
        let mut replacements_chirho = Vec::new();
        for (index_chirho, name_chirho, scheme_chirho) in self.env_chirho.located_bindings_chirho()
        {
            budget_chirho.work_chirho(1)?;
            budget_chirho.work_chirho(scheme_chirho.vars_chirho.len())?;
            let mut bound_chirho = Vec::new();
            let mut affected_chirho = false;
            for record_chirho in scheme_chirho.vars_chirho.records_chirho() {
                if let Some(kind_chirho) = &record_chirho.kind_chirho {
                    affected_chirho |= reaches_skolem_chirho(
                        kind_chirho,
                        &self.equality_substitutions_chirho,
                        skolems_chirho,
                        &mut bound_chirho,
                        budget_chirho,
                        0,
                    )?;
                }
                bound_chirho.push(record_chirho.variable_chirho);
            }
            for term_chirho in std::iter::once(&scheme_chirho.ty_chirho).chain(
                scheme_chirho
                    .preds_chirho
                    .iter()
                    .flat_map(|predicate_chirho| {
                        predicate_chirho
                            .kind_tys_chirho
                            .iter()
                            .chain(std::iter::once(&predicate_chirho.ty_chirho))
                            .chain(&predicate_chirho.extra_tys_chirho)
                    }),
            ) {
                affected_chirho |= reaches_skolem_chirho(
                    term_chirho,
                    &self.equality_substitutions_chirho,
                    skolems_chirho,
                    &mut bound_chirho,
                    budget_chirho,
                    0,
                )?;
            }
            if !affected_chirho {
                continue;
            }
            let (variables_chirho, ty_chirho) = self.project_signature_term_chirho(
                &scheme_chirho.ty_chirho,
                &scheme_chirho.vars_chirho,
                budget_chirho,
            )?;
            let mut predicates_chirho = Vec::new();
            for predicate_chirho in &scheme_chirho.preds_chirho {
                budget_chirho
                    .node_chirho(predicate_chirho.class_name_chirho.len().saturating_add(1))?;
                let mut project_chirho = |term_chirho: &TyChirho| {
                    self.project_signature_term_chirho(
                        term_chirho,
                        &scheme_chirho.vars_chirho,
                        budget_chirho,
                    )
                    .map(|(_, term_chirho)| term_chirho)
                };
                predicates_chirho.push(crate::ty_chirho::SchemePredChirho::from_proof_chirho(
                    predicate_chirho.class_identity_chirho().clone(),
                    crate::ty_chirho::SchemePredPartsChirho {
                        kind_tys_chirho: predicate_chirho
                            .kind_tys_chirho
                            .iter()
                            .map(&mut project_chirho)
                            .collect::<Option<Vec<_>>>()?,
                        class_name_chirho: predicate_chirho.class_name_chirho.clone(),
                        ty_chirho: project_chirho(&predicate_chirho.ty_chirho)?,
                        extra_tys_chirho: predicate_chirho
                            .extra_tys_chirho
                            .iter()
                            .map(&mut project_chirho)
                            .collect::<Option<Vec<_>>>()?,
                    },
                ));
            }
            budget_chirho.node_chirho(
                name_chirho
                    .len()
                    .saturating_add(scheme_chirho.vars_chirho.len())
                    .saturating_add(1),
            )?;
            replacements_chirho.push((
                index_chirho,
                name_chirho.clone(),
                SchemeChirho {
                    vars_chirho: variables_chirho,
                    preds_chirho: predicates_chirho,
                    ty_chirho,
                },
            ));
        }
        Some(replacements_chirho)
    }

    pub(super) fn check_annotation_escape_chirho(
        &mut self,
        captured_chirho: Option<SignatureCapturesChirho>,
        skolems_chirho: &HashSet<String>,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let Some(mut captured_chirho) = captured_chirho else {
            if !self.has_non_deferrable_failure_chirho() {
                self.type_normalization_exhausted_chirho.set(true);
            }
            return SubstChirho::empty_chirho();
        };
        if self.has_non_deferrable_failure_chirho() {
            if let Some(kinds_chirho) = captured_chirho.kinds_chirho.take() {
                if let Err(error_chirho) = self
                    .elaboration_state_chirho
                    .restore_variable_kinds_chirho(kinds_chirho, span_chirho)
                {
                    self.record_commit_failure_chirho(&error_chirho);
                }
            }
            return SubstChirho::empty_chirho();
        }
        let mut public_chirho = SubstChirho::empty_chirho();
        let mut escaped_chirho = Some(false);
        for variable_chirho in &captured_chirho.variables_chirho {
            if captured_chirho.budget_chirho.work_chirho(1).is_none() {
                escaped_chirho = None;
                break;
            }
            let Some(image_chirho) = self
                .equality_substitutions_chirho
                .lookup_chirho(variable_chirho)
            else {
                continue;
            };
            escaped_chirho = reaches_skolem_chirho(
                image_chirho,
                &self.equality_substitutions_chirho,
                skolems_chirho,
                &mut Vec::new(),
                &mut captured_chirho.budget_chirho,
                0,
            );
            let normalized_chirho = if escaped_chirho == Some(true) {
                self.normalize_signature_public_image_chirho(
                    image_chirho,
                    &mut captured_chirho.budget_chirho,
                )
                .ok()
            } else if escaped_chirho == Some(false) {
                self.equality_substitutions_chirho
                    .resolve_owned_bounded_chirho(
                        image_chirho,
                        &mut |work_chirho, output_chirho| {
                            captured_chirho.budget_chirho.node_chirho(output_chirho)?;
                            captured_chirho
                                .budget_chirho
                                .work_chirho(work_chirho.saturating_sub(output_chirho))
                        },
                    )
            } else {
                None
            };
            let Some(normalized_chirho) = normalized_chirho else {
                escaped_chirho = None;
                break;
            };
            escaped_chirho = reaches_skolem_chirho(
                &normalized_chirho,
                &SubstChirho::empty_chirho(),
                skolems_chirho,
                &mut Vec::new(),
                &mut captured_chirho.budget_chirho,
                0,
            );
            if escaped_chirho != Some(false) {
                break;
            }
            if normalized_chirho != TyChirho::VarChirho(*variable_chirho) {
                public_chirho.insert_chirho(*variable_chirho, normalized_chirho);
            }
        }
        let mut kind_escape_chirho = false;
        if escaped_chirho == Some(true) {
            if let Some(kinds_chirho) = &captured_chirho.kinds_chirho {
                for (_, kind_chirho) in kinds_chirho.kinds_chirho() {
                    if let Some(kind_chirho) = kind_chirho {
                        match reaches_skolem_chirho(
                            kind_chirho,
                            &self.equality_substitutions_chirho,
                            skolems_chirho,
                            &mut Vec::new(),
                            &mut captured_chirho.budget_chirho,
                            0,
                        ) {
                            Some(true) => {
                                kind_escape_chirho = true;
                                break;
                            }
                            None => {
                                escaped_chirho = None;
                                break;
                            }
                            Some(false) => {}
                        }
                    }
                }
            }
        }
        let replacements_chirho =
            if escaped_chirho == Some(false) && !public_chirho.is_empty_chirho() {
                match self.project_signature_environment_chirho(
                    skolems_chirho,
                    &mut captured_chirho.budget_chirho,
                ) {
                    Some(replacements_chirho) => replacements_chirho,
                    None => {
                        escaped_chirho = None;
                        Vec::new()
                    }
                }
            } else {
                Vec::new()
            };
        if let Some(kinds_chirho) = captured_chirho.kinds_chirho.take() {
            if escaped_chirho == Some(false) {
                if self
                    .apply_projected_subst_all_chirho(
                        &mut public_chirho,
                        kinds_chirho,
                        skolems_chirho,
                        span_chirho,
                    )
                    .is_err()
                {
                    return SubstChirho::empty_chirho();
                }
            } else if let Err(error_chirho) = self
                .elaboration_state_chirho
                .restore_variable_kinds_chirho(kinds_chirho, span_chirho)
            {
                self.record_commit_failure_chirho(&error_chirho);
                return SubstChirho::empty_chirho();
            }
        }
        match escaped_chirho {
            Some(false) => {
                for (index_chirho, name_chirho, scheme_chirho) in replacements_chirho {
                    self.env_chirho.replace_located_binding_chirho(
                        index_chirho,
                        &name_chirho,
                        public_chirho.apply_scheme_chirho(&scheme_chirho),
                    );
                }
                public_chirho
            }
            Some(true) => {
                let diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
                    if kind_escape_chirho {
                        "expression annotation's rigid kind variable escapes into an enclosing type"
                    } else {
                        "expression annotation's rigid variable escapes into an enclosing type"
                    },
                    span_chirho,
                );
                if kind_escape_chirho {
                    if !self
                        .non_deferrable_diagnostics_chirho
                        .diagnostics_chirho()
                        .contains(&diagnostic_chirho)
                    {
                        self.non_deferrable_diagnostics_chirho
                            .push_chirho(diagnostic_chirho);
                    }
                } else {
                    self.diagnostics_chirho.push_chirho(diagnostic_chirho);
                }
                SubstChirho::empty_chirho()
            }
            None => {
                if !self.has_non_deferrable_failure_chirho() {
                    self.type_normalization_exhausted_chirho.set(true);
                }
                SubstChirho::empty_chirho()
            }
        }
    }
}
