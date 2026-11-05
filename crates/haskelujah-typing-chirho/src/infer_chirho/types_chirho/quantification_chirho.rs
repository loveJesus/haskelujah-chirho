// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generalization keeps dependencies before their owners in the same lexical
//! record group used by schemes and nested forall types.
//! Workflow: language-features-chirho/rank-n-visible-type-application-chirho.md.

use super::{InferCtxChirho, equality_normalization_chirho::EqualityBudgetChirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{
    QuantifiedVariableChirho, QuantifiedVariablesChirho, TyChirho, TyVarChirho,
};
use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

impl InferCtxChirho {
    /// Opening consumes a checked kind under the preceding prefix. Committed
    /// ordinary substitutions specialize its consumer metas; given rewrite rules
    /// and family evidence are not consulted here.
    pub(super) fn instantiate_quantifier_kind_chirho(
        &mut self,
        record_chirho: &QuantifiedVariableChirho,
        prefix_chirho: &SubstChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<Option<Arc<TyChirho>>> {
        budget_chirho.node_chirho(1)?;
        let Some(kind_chirho) = &record_chirho.kind_chirho else {
            return Some(None);
        };
        let mut charge_chirho = |work_chirho: usize, output_chirho: usize| {
            budget_chirho.node_chirho(output_chirho)?;
            budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
        };
        let mapped_chirho = prefix_chirho.apply_freshened_bounded_chirho(
            kind_chirho,
            &mut self.next_var_chirho,
            &mut charge_chirho,
        )?;
        let mapped_chirho = self
            .equality_substitutions_chirho
            .resolve_owned_bounded_chirho(&mapped_chirho, &mut charge_chirho)?;
        Some(Some(Arc::new(mapped_chirho)))
    }

    pub(super) fn note_fresh_quantifier_kind_chirho(
        &mut self,
        image_chirho: &TyChirho,
        kind_chirho: &Option<Arc<TyChirho>>,
    ) {
        if let (TyChirho::VarChirho(variable_chirho), Some(kind_chirho)) =
            (image_chirho, kind_chirho)
        {
            self.elaboration_variable_kinds_chirho.insert(
                *variable_chirho,
                QuantifiedVariableChirho {
                    variable_chirho: *variable_chirho,
                    kind_chirho: Some(kind_chirho.clone()),
                    source_chirho: None,
                    specificity_chirho: None,
                },
            );
        }
    }

    /// Substitute a complete outer prefix while alpha-freshening nested
    /// binders in the same walk. An image cannot become a nested binder's use.
    pub(super) fn instantiate_scheme_terms_chirho(
        &mut self,
        scheme_chirho: &crate::ty_chirho::SchemeChirho,
        body_chirho: &TyChirho,
        prefix_chirho: &SubstChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<(TyChirho, Vec<crate::class_chirho::PredChirho>)> {
        budget_chirho.node_chirho(scheme_chirho.preds_chirho.len())?;
        for predicate_chirho in &scheme_chirho.preds_chirho {
            let width_chirho = predicate_chirho
                .kind_tys_chirho
                .len()
                .checked_add(predicate_chirho.extra_tys_chirho.len())?
                .checked_add(predicate_chirho.class_name_chirho.len())?;
            budget_chirho.node_chirho(width_chirho)?;
        }
        let mut charge_chirho = |work_chirho: usize, output_chirho: usize| {
            budget_chirho.node_chirho(output_chirho)?;
            budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
        };
        let mut map_chirho = |term_chirho: &TyChirho| {
            prefix_chirho.apply_scheme_term_bounded_chirho(
                term_chirho,
                &mut self.next_var_chirho,
                &mut charge_chirho,
            )
        };
        let body_chirho = map_chirho(body_chirho)?;
        let mut predicates_chirho = Vec::new();
        for predicate_chirho in &scheme_chirho.preds_chirho {
            predicates_chirho.push(crate::class_chirho::PredChirho::from_proof_chirho(
                predicate_chirho.class_identity_chirho().clone(),
                crate::class_chirho::PredPartsChirho {
                    kind_tys_chirho: predicate_chirho
                        .kind_tys_chirho
                        .iter()
                        .map(&mut map_chirho)
                        .collect::<Option<_>>()?,
                    class_name_chirho: predicate_chirho.class_name_chirho.clone(),
                    ty_chirho: map_chirho(&predicate_chirho.ty_chirho)?,
                    extra_tys_chirho: predicate_chirho
                        .extra_tys_chirho
                        .iter()
                        .map(&mut map_chirho)
                        .collect::<Option<_>>()?,
                },
            ));
        }
        Some((body_chirho, predicates_chirho))
    }

    /// Existing records retain their proof state. Only this actual generalization
    /// operation supplies visibility for newly introduced variables.
    pub(super) fn quantify_variables_chirho(
        &mut self,
        ordered_chirho: Vec<TyVarChirho>,
        existing_chirho: &[QuantifiedVariableChirho],
        scoped_chirho: &[TyVarChirho],
        substitution_chirho: &SubstChirho,
        initial_specificity_chirho: TyVarSpecificityChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> QuantifiedVariablesChirho {
        let mut refusal_chirho = None;
        let result_chirho = (|| {
            let mut charge_chirho = |work_chirho: usize, output_chirho: usize| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            };
            charge_chirho(
                ordered_chirho
                    .len()
                    .checked_add(existing_chirho.len())?
                    .checked_add(scoped_chirho.len())?,
                ordered_chirho.len().checked_add(existing_chirho.len())?,
            )?;
            let scoped_chirho: HashSet<_> = scoped_chirho.iter().copied().collect();
            let initial_chirho: HashSet<_> = ordered_chirho.iter().copied().collect();
            let mut provided_chirho = HashMap::new();
            for record_chirho in existing_chirho {
                charge_chirho(1, 1)?;
                if let Some(previous_chirho) =
                    provided_chirho.get(&record_chirho.variable_chirho).copied()
                {
                    let previous_chirho: &QuantifiedVariableChirho = previous_chirho;
                    let same_kind_chirho =
                        match (&previous_chirho.kind_chirho, &record_chirho.kind_chirho) {
                            (Some(left_chirho), Some(right_chirho)) => {
                                crate::ty_chirho::same_type_contract_chirho(
                                    left_chirho,
                                    right_chirho,
                                    &mut |weight_chirho| charge_chirho(weight_chirho, 0),
                                )?
                            }
                            (None, None) => true,
                            _ => false,
                        };
                    if !same_kind_chirho
                        || previous_chirho.source_chirho != record_chirho.source_chirho
                        || previous_chirho.specificity_chirho != record_chirho.specificity_chirho
                    {
                        refusal_chirho =
                            Some("conflicting checked quantifier records during generalization");
                        return None;
                    }
                } else {
                    provided_chirho.insert(record_chirho.variable_chirho, record_chirho);
                }
            }
            let mut pending_chirho = ordered_chirho.clone();
            let mut records_chirho = HashMap::new();
            let mut dependencies_chirho = HashMap::new();
            let mut cursor_chirho = 0;
            while let Some(variable_chirho) = pending_chirho.get(cursor_chirho).copied() {
                cursor_chirho += 1;
                charge_chirho(1, 0)?;
                if scoped_chirho.contains(&variable_chirho)
                    || records_chirho.contains_key(&variable_chirho)
                {
                    continue;
                }
                let mut record_chirho = if let Some(record_chirho) =
                    provided_chirho.get(&variable_chirho)
                {
                    (*record_chirho).clone()
                } else {
                    let mut record_chirho = self
                        .elaboration_variable_kinds_chirho
                        .get(&variable_chirho)
                        .cloned()
                        .unwrap_or_else(|| {
                            QuantifiedVariableChirho::unknown_chirho(variable_chirho)
                        });
                    // An inferred value binding introduces inferred quantifiers.
                    // A written signature keeps the actual producer's source flag;
                    // generated invisible arguments have no written source witness.
                    record_chirho.specificity_chirho = Some(
                        if initial_specificity_chirho == TyVarSpecificityChirho::InferredChirho {
                            TyVarSpecificityChirho::InferredChirho
                        } else if let Some(specificity_chirho) = record_chirho.specificity_chirho {
                            specificity_chirho
                        } else if (record_chirho.source_chirho.is_some()
                            || !self
                                .elaboration_variable_kinds_chirho
                                .contains_key(&variable_chirho))
                            && initial_chirho.contains(&variable_chirho)
                        {
                            initial_specificity_chirho
                        } else {
                            TyVarSpecificityChirho::InferredChirho
                        },
                    );
                    if let Some(kind_chirho) = &record_chirho.kind_chirho {
                        record_chirho.kind_chirho = Some(Arc::new(
                            self.elaboration_substitution_chirho
                                .resolve_owned_bounded_chirho(kind_chirho, &mut charge_chirho)?,
                        ));
                    }
                    record_chirho
                };
                if !provided_chirho.contains_key(&variable_chirho)
                    && let Some(kind_chirho) = &record_chirho.kind_chirho
                {
                    record_chirho.kind_chirho = Some(Arc::new(
                        self.equality_substitutions_chirho
                            .resolve_owned_bounded_chirho(kind_chirho, &mut charge_chirho)?,
                    ));
                }
                let kind_chirho = match &record_chirho.kind_chirho {
                    Some(kind_chirho) => Some(Arc::new(
                        substitution_chirho
                            .apply_bounded_chirho(kind_chirho, &mut charge_chirho)?,
                    )),
                    None => None,
                };
                let free_chirho = match &kind_chirho {
                    Some(kind_chirho) => {
                        SubstChirho::free_variables_bounded_chirho(kind_chirho, &mut charge_chirho)?
                    }
                    None => Vec::new(),
                };
                charge_chirho(free_chirho.len(), free_chirho.len())?;
                let free_chirho: Vec<_> = free_chirho
                    .into_iter()
                    .filter(|variable_chirho| !scoped_chirho.contains(variable_chirho))
                    .collect();
                pending_chirho.extend(free_chirho.iter().copied());
                record_chirho.kind_chirho = kind_chirho;
                charge_chirho(2, 2)?;
                dependencies_chirho.insert(variable_chirho, free_chirho);
                records_chirho.insert(variable_chirho, record_chirho);
            }
            // An iterative postorder keeps independent source binders stable and
            // refuses cycles instead of silently freshening an unbound capture.
            let mut visiting_chirho = HashSet::new();
            let mut done_chirho = HashSet::new();
            let mut output_chirho = Vec::new();
            for variable_chirho in ordered_chirho {
                if scoped_chirho.contains(&variable_chirho) {
                    continue;
                }
                charge_chirho(1, 1)?;
                let mut stack_chirho = vec![(variable_chirho, false)];
                while let Some((variable_chirho, exiting_chirho)) = stack_chirho.pop() {
                    charge_chirho(1, 0)?;
                    if done_chirho.contains(&variable_chirho) {
                        continue;
                    }
                    if exiting_chirho {
                        visiting_chirho.remove(&variable_chirho);
                        done_chirho.insert(variable_chirho);
                        charge_chirho(1, 1)?;
                        output_chirho.push(records_chirho.remove(&variable_chirho)?);
                        continue;
                    }
                    if !visiting_chirho.insert(variable_chirho) {
                        refusal_chirho =
                            Some("cyclic checked binder-kind dependency during generalization");
                        return None;
                    }
                    let dependencies_chirho = dependencies_chirho.get(&variable_chirho)?;
                    charge_chirho(
                        dependencies_chirho.len().checked_add(1)?,
                        dependencies_chirho.len().checked_add(1)?,
                    )?;
                    stack_chirho.push((variable_chirho, true));
                    stack_chirho.extend(
                        dependencies_chirho
                            .iter()
                            .rev()
                            .map(|variable_chirho| (*variable_chirho, false)),
                    );
                }
            }
            Some(QuantifiedVariablesChirho::from_records_chirho(
                output_chirho,
            ))
        })();
        result_chirho.unwrap_or_else(|| {
            if let Some(message_chirho) = refusal_chirho {
                self.non_deferrable_diagnostics_chirho.push_chirho(
                    haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                        haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(300),
                        message_chirho,
                        haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
                    ),
                );
            } else {
                self.type_normalization_exhausted_chirho.set(true);
            }
            QuantifiedVariablesChirho::default()
        })
    }
}

#[cfg(test)]
#[path = "quantification_tests_chirho.rs"]
mod tests_chirho;
