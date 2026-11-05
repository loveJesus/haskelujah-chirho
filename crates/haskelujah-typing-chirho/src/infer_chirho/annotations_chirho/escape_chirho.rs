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
            bound_chirho.extend(vars_chirho);
            let result_chirho = reaches_skolem_chirho(
                body_chirho,
                substitutions_chirho,
                skolems_chirho,
                bound_chirho,
                budget_chirho,
                depth_chirho + 1,
            );
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
pub(super) struct SignatureCapturesChirho {
    variables_chirho: HashSet<TyVarChirho>,
    budget_chirho: EqualityBudgetChirho,
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
            bound_chirho.extend(vars_chirho);
            let result_chirho =
                collect_free_chirho(body_chirho, bound_chirho, captured_chirho, depth_chirho + 1);
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
    pub(super) fn capture_signature_metas_chirho(
        &self,
        scheme_chirho: &SchemeChirho,
    ) -> Option<SignatureCapturesChirho> {
        let mut captured_chirho = SignatureCapturesChirho {
            variables_chirho: HashSet::new(),
            budget_chirho: EqualityBudgetChirho::new_chirho(),
        };
        for entry_chirho in self
            .env_chirho
            .bindings_in_scope_order_chirho()
            .map(|(_, entry_chirho)| entry_chirho)
            .chain(std::iter::once(Some(scheme_chirho)))
        {
            captured_chirho.budget_chirho.work_chirho(1)?;
            let Some(entry_chirho) = entry_chirho else {
                continue;
            };
            captured_chirho
                .budget_chirho
                .work_chirho(entry_chirho.vars_chirho.len())?;
            let mut bound_chirho = entry_chirho.vars_chirho.clone();
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
        Some(captured_chirho)
    }

    fn project_signature_term_chirho(
        &self,
        term_chirho: &TyChirho,
        variables_chirho: &[TyVarChirho],
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<TyChirho> {
        if variables_chirho.is_empty() {
            return self
                .normalize_signature_public_image_chirho(term_chirho, budget_chirho)
                .ok();
        }
        budget_chirho.node_chirho(variables_chirho.len().saturating_add(1))?;
        let protected_chirho = TyChirho::ForallChirho {
            vars_chirho: variables_chirho.to_vec(),
            body_chirho: Box::new(copy_term_with_budget_chirho(term_chirho, budget_chirho)?),
        };
        match self
            .normalize_signature_public_image_chirho(&protected_chirho, budget_chirho)
            .ok()?
        {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } if vars_chirho == variables_chirho => Some(*body_chirho),
            _ => None,
        }
    }

    /// Replacing a numeric root alone cannot repair an environment entry that
    /// already contains its former rigid image. Project those affected schemes
    /// through the same checked evidence before it leaves scope, atomically.
    fn project_signature_environment_chirho(
        &mut self,
        skolems_chirho: &HashSet<String>,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<()> {
        let mut replacements_chirho = Vec::new();
        for (index_chirho, name_chirho, scheme_chirho) in self.env_chirho.located_bindings_chirho()
        {
            budget_chirho.work_chirho(1)?;
            budget_chirho.work_chirho(scheme_chirho.vars_chirho.len())?;
            let mut bound_chirho = scheme_chirho.vars_chirho.clone();
            let mut affected_chirho = false;
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
            let ty_chirho = self.project_signature_term_chirho(
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
                    vars_chirho: scheme_chirho.vars_chirho.clone(),
                    preds_chirho: predicates_chirho,
                    ty_chirho,
                },
            ));
        }
        for (index_chirho, name_chirho, scheme_chirho) in replacements_chirho {
            self.env_chirho.replace_located_binding_chirho(
                index_chirho,
                &name_chirho,
                scheme_chirho,
            );
        }
        Some(())
    }

    pub(super) fn check_annotation_escape_chirho(
        &mut self,
        captured_chirho: Option<SignatureCapturesChirho>,
        skolems_chirho: &HashSet<String>,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let Some(mut captured_chirho) = captured_chirho else {
            self.type_normalization_exhausted_chirho.set(true);
            return SubstChirho::empty_chirho();
        };
        let mut public_chirho = SubstChirho::empty_chirho();
        let mut escaped_chirho = Some(false);
        for variable_chirho in captured_chirho.variables_chirho {
            if captured_chirho.budget_chirho.work_chirho(1).is_none() {
                escaped_chirho = None;
                break;
            }
            let Some(image_chirho) = self
                .equality_substitutions_chirho
                .lookup_chirho(&variable_chirho)
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
            if escaped_chirho == Some(true) {
                escaped_chirho = self
                    .normalize_signature_public_image_chirho(
                        image_chirho,
                        &mut captured_chirho.budget_chirho,
                    )
                    .ok()
                    .and_then(|normalized_chirho| {
                        let escaping_chirho = reaches_skolem_chirho(
                            &normalized_chirho,
                            &SubstChirho::empty_chirho(),
                            skolems_chirho,
                            &mut Vec::new(),
                            &mut captured_chirho.budget_chirho,
                            0,
                        )?;
                        if !escaping_chirho {
                            public_chirho.insert_chirho(variable_chirho, normalized_chirho);
                        }
                        Some(escaping_chirho)
                    });
            }
            if escaped_chirho != Some(false) {
                break;
            }
        }
        if escaped_chirho == Some(false)
            && !public_chirho.is_empty_chirho()
            && self
                .project_signature_environment_chirho(
                    skolems_chirho,
                    &mut captured_chirho.budget_chirho,
                )
                .is_none()
        {
            escaped_chirho = None;
        }
        match escaped_chirho {
            Some(false) => {}
            Some(true) => {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
                        "expression annotation's rigid variable escapes into an enclosing type",
                        span_chirho,
                    ))
            }
            None => self.type_normalization_exhausted_chirho.set(true),
        }
        public_chirho
    }
}
