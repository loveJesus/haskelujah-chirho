// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One checked commit updates the owned outward substitution and every live view.
//! Workflow: type-identity-chirho/committed-meta-kinds-chirho.md.

use super::InferCtxChirho;
use crate::kind_chirho::elaboration_chirho::{CapturedVariableKindsChirho, ElaborationErrorChirho};
use crate::subst_chirho::{SubstChirho, SubstitutionImageChirho};
use crate::ty_chirho::{TyChirho, TyVarChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[cfg(test)]
#[path = "effective_commit_tests_chirho.rs"]
mod effective_commit_tests_chirho;

struct ProjectedKindViewChirho<'view_chirho, 'budget_chirho> {
    effective_chirho: &'view_chirho SubstChirho,
    prepared_chirho: &'view_chirho CapturedVariableKindsChirho,
    skolems_chirho: &'view_chirho HashSet<String>,
    budget_chirho: &'budget_chirho mut crate::infer_chirho::EqualityBudgetChirho,
}

impl ProjectedKindViewChirho<'_, '_> {
    fn closed_chirho(
        &mut self,
        term_chirho: &TyChirho,
        bound_chirho: &mut Vec<TyVarChirho>,
        depth_chirho: usize,
    ) -> Option<bool> {
        self.budget_chirho.work_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                self.budget_chirho.work_chirho(bound_chirho.len())?;
                if bound_chirho.contains(variable_chirho) {
                    return Some(true);
                }
                if let Some(image_chirho) = self.effective_chirho.lookup_chirho(variable_chirho) {
                    if image_chirho != term_chirho
                        && !self.closed_chirho(image_chirho, bound_chirho, depth_chirho + 1)?
                    {
                        return Some(false);
                    }
                }
                if let Some(kind_chirho) =
                    self.prepared_chirho.prepared_kind_chirho(*variable_chirho)
                {
                    if !self.closed_chirho(kind_chirho, bound_chirho, depth_chirho + 1)? {
                        return Some(false);
                    }
                }
                Some(true)
            }
            TyChirho::ForallVarChirho(name_chirho) => {
                self.budget_chirho.work_chirho(name_chirho.len())?;
                Some(!self.skolems_chirho.contains(name_chirho))
            }
            TyChirho::ConChirho(_) => Some(true),
            TyChirho::AppChirho(function_chirho, argument_chirho)
            | TyChirho::FunChirho(function_chirho, argument_chirho, _) => Some(
                self.closed_chirho(function_chirho, bound_chirho, depth_chirho + 1)?
                    && self.closed_chirho(argument_chirho, bound_chirho, depth_chirho + 1)?,
            ),
            TyChirho::ListChirho(element_chirho) => {
                self.closed_chirho(element_chirho, bound_chirho, depth_chirho + 1)
            }
            TyChirho::TupleChirho(elements_chirho) => {
                for element_chirho in elements_chirho {
                    if !self.closed_chirho(element_chirho, bound_chirho, depth_chirho + 1)? {
                        return Some(false);
                    }
                }
                Some(true)
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                self.budget_chirho.node_chirho(vars_chirho.len())?;
                let mark_chirho = bound_chirho.len();
                let result_chirho = (|| {
                    for record_chirho in vars_chirho.records_chirho() {
                        if let Some(kind_chirho) = &record_chirho.kind_chirho {
                            if !self.closed_chirho(kind_chirho, bound_chirho, depth_chirho + 1)? {
                                return Some(false);
                            }
                        }
                        bound_chirho.push(record_chirho.variable_chirho);
                    }
                    self.closed_chirho(body_chirho, bound_chirho, depth_chirho + 1)
                })();
                bound_chirho.truncate(mark_chirho);
                result_chirho
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                for predicate_chirho in context_chirho {
                    if !self.closed_chirho(predicate_chirho, bound_chirho, depth_chirho + 1)? {
                        return Some(false);
                    }
                }
                self.closed_chirho(body_chirho, bound_chirho, depth_chirho + 1)
            }
        }
    }
}

impl InferCtxChirho {
    /// The existing terminal normalization error term carries no fresh ID,
    /// kind record or success authority after a non-deferrable commit failure.
    pub(super) fn terminal_commit_type_chirho() -> TyChirho {
        TyChirho::ConChirho("%normalization:resource-exhausted".into())
    }

    pub(super) fn record_commit_failure_chirho(&mut self, error_chirho: &ElaborationErrorChirho) {
        self.restore_active_kind_captures_chirho();
        let diagnostic_chirho = error_chirho.clone().into_diagnostic_chirho();
        if !self
            .non_deferrable_diagnostics_chirho
            .diagnostics_chirho()
            .contains(&diagnostic_chirho)
        {
            self.non_deferrable_diagnostics_chirho
                .push_chirho(diagnostic_chirho);
        }
    }

    pub(super) fn restore_active_kind_captures_chirho(&mut self) {
        let mut owner_chirho = Some(self.active_equality_scope_chirho);
        while let Some(current_chirho) = owner_chirho {
            owner_chirho = self.equality_scopes_chirho[current_chirho].parent_chirho();
            if let Some(captured_chirho) = self.equality_scopes_chirho[current_chirho]
                .captured_kinds_chirho
                .take()
            {
                if let Err(error_chirho) = self
                    .elaboration_state_chirho
                    .restore_variable_kinds_chirho(captured_chirho, SpanChirho::DUMMY_CHIRHO)
                {
                    let diagnostic_chirho = error_chirho.into_diagnostic_chirho();
                    if !self
                        .non_deferrable_diagnostics_chirho
                        .diagnostics_chirho()
                        .contains(&diagnostic_chirho)
                    {
                        self.non_deferrable_diagnostics_chirho
                            .push_chirho(diagnostic_chirho);
                    }
                }
            }
        }
    }

    /// A concrete result producer consumes its effective links with the same
    /// original owner allowance, without consulting private equality evidence.
    pub(super) fn materialize_committed_type_chirho(
        &mut self,
        source_chirho: &SubstChirho,
        term_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Option<TyChirho> {
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_commit_failure_chirho(&error_chirho);
                return None;
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let result_chirho = source_chirho.resolve_owned_bounded_chirho(
            term_chirho,
            &mut |work_chirho, output_chirho| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            },
        );
        if let Err(error_chirho) = self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            result_chirho.is_none(),
            span_chirho,
        ) {
            self.record_commit_failure_chirho(&error_chirho);
            return None;
        }
        result_chirho
    }

    /// Stage the effective ordinary plus kind solution before changing the
    /// caller's carrier or any live view. Result must be handled at each call.
    /// workflow: type-identity-chirho/committed-meta-kinds-chirho
    pub(super) fn apply_subst_all_chirho(
        &mut self,
        subst_chirho: &mut SubstChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.commit_substitution_views_chirho(subst_chirho, None, None, SpanChirho::DUMMY_CHIRHO)
    }

    pub(super) fn apply_projected_subst_all_chirho(
        &mut self,
        subst_chirho: &mut SubstChirho,
        captured_chirho: CapturedVariableKindsChirho,
        skolems_chirho: &HashSet<String>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.commit_substitution_views_chirho(
            subst_chirho,
            Some(captured_chirho),
            Some(skolems_chirho),
            span_chirho,
        )
    }

    fn commit_substitution_views_chirho(
        &mut self,
        subst_chirho: &mut SubstChirho,
        captured_chirho: Option<CapturedVariableKindsChirho>,
        skolems_chirho: Option<&HashSet<String>>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        if self.has_non_deferrable_failure_chirho() {
            if let Some(captured_chirho) = captured_chirho {
                if let Err(error_chirho) = self
                    .elaboration_state_chirho
                    .restore_variable_kinds_chirho(captured_chirho, span_chirho)
                {
                    self.record_commit_failure_chirho(&error_chirho);
                    return Err(error_chirho);
                }
            }
            return Err(self
                .elaboration_state_chirho
                .failure_chirho()
                .cloned()
                .unwrap_or_else(|| ElaborationErrorChirho {
                    mismatch_chirho: None,
                    message_chirho: "substitution commit stopped after terminal inference failure"
                        .into(),
                    span_chirho,
                }));
        }
        let prepared_chirho = match self
            .elaboration_state_chirho
            .prepare_committed_variable_kinds_chirho(
                subst_chirho,
                &self.type_families_chirho,
                captured_chirho,
                Some(&self.skolem_bindings_chirho),
                span_chirho,
            ) {
            Ok(prepared_chirho) => prepared_chirho,
            Err(error_chirho) => {
                self.record_commit_failure_chirho(&error_chirho);
                return Err(error_chirho);
            }
        };
        if let Some(skolems_chirho) = skolems_chirho {
            if let Err(error_chirho) = self.validate_projected_commit_chirho(
                subst_chirho,
                &prepared_chirho,
                skolems_chirho,
                span_chirho,
            ) {
                if let Err(restore_chirho) = self
                    .elaboration_state_chirho
                    .restore_variable_kinds_chirho(prepared_chirho, span_chirho)
                {
                    self.record_commit_failure_chirho(&restore_chirho);
                    return Err(restore_chirho);
                }
                self.record_commit_failure_chirho(&error_chirho);
                return Err(error_chirho);
            }
        }
        let effective_chirho = match self
            .elaboration_state_chirho
            .publish_committed_variable_kinds_chirho(prepared_chirho, span_chirho)
        {
            Ok(effective_chirho) => effective_chirho,
            Err(error_chirho) => {
                self.record_commit_failure_chirho(&error_chirho);
                return Err(error_chirho);
            }
        };
        if let Some(effective_chirho) = effective_chirho {
            *subst_chirho = effective_chirho;
        }
        for (var_chirho, ty_chirho) in subst_chirho.iter_chirho() {
            if self.equality_pattern_depth_chirho > 0 {
                if self.equality_substitution_events_chirho.len() < 16_384 {
                    self.equality_substitution_events_chirho.push(*var_chirho);
                } else {
                    self.equality_substitution_events_exhausted_chirho = true;
                }
            }
            self.equality_substitutions_chirho
                .insert_chirho(*var_chirho, ty_chirho.clone());
        }
        self.env_chirho.apply_subst_chirho(subst_chirho);
        self.restriction_chirho
            .follow_substitution_chirho(subst_chirho);
        self.substitute_inferred_dictionary_owners_chirho(subst_chirho);
        self.recursive_references_chirho
            .apply_subst_chirho(subst_chirho);
        // Every argument of a multi-parameter predicate must follow the
        // substitution, not only the primary one: a functional-dependency
        // improvement or a solved variable in an extra position would
        // otherwise be lost (`MyReader r v` with `v := Int`).
        for (pred_chirho, _span_chirho) in &mut self.deferred_preds_chirho {
            *pred_chirho =
                pred_chirho.mapped_chirho(|ty_chirho| subst_chirho.apply_ty_chirho(ty_chirho));
        }
        for pred_chirho in &mut self.given_preds_chirho {
            *pred_chirho =
                pred_chirho.mapped_chirho(|ty_chirho| subst_chirho.apply_ty_chirho(ty_chirho));
        }
        for wanted_chirho in &mut self.deferred_equalities_chirho {
            wanted_chirho.lhs_chirho = subst_chirho.apply_ty_chirho(&wanted_chirho.lhs_chirho);
            wanted_chirho.rhs_chirho = subst_chirho.apply_ty_chirho(&wanted_chirho.rhs_chirho);
        }
        for (lhs_chirho, rhs_chirho) in &mut self.given_rewrites_chirho {
            *lhs_chirho = subst_chirho.apply_ty_chirho(lhs_chirho);
            *rhs_chirho = subst_chirho.apply_ty_chirho(rhs_chirho);
        }
        for capture_chirho in &mut self.occurrence_captures_chirho {
            capture_chirho.pred_chirho = capture_chirho
                .pred_chirho
                .mapped_chirho(|ty_chirho| subst_chirho.apply_ty_chirho(ty_chirho));
        }
        for capture_chirho in &mut self.reference_captures_chirho {
            for predicate_chirho in &mut capture_chirho.preds_chirho {
                *predicate_chirho = predicate_chirho
                    .mapped_chirho(|ty_chirho| subst_chirho.apply_ty_chirho(ty_chirho));
            }
        }
        // A literal's minted wanted follows the same substitution events, proof and arguments
        // kept, so a literal inside an instance method reaches finalization at the solution that
        // method's check found: the module's composition never holds an instance method's own
        // (row 692, C10).
        // workflow: language-features-chirho/dictionary-evidence-chirho
        for (_span_chirho, _origin_chirho, wanted_chirho, _context_chirho) in
            &mut self.literal_captures_chirho
        {
            *wanted_chirho =
                wanted_chirho.mapped_chirho(|ty_chirho| subst_chirho.apply_ty_chirho(ty_chirho));
        }
        if !self.occurrence_default_hints_chirho.is_empty() {
            let mut updated_hints_chirho = HashMap::new();
            // DETERMINISM: substitution can map two distinct vars onto the SAME
            // updated var, and the later insert then wins. Draining a HashMap
            // makes "later" depend on a per-process hash seed, so the surviving
            // defaulting hint — and with it the inferred type — could differ
            // between runs of the same binary on the same source.
            // See spec-chirho/bug-nondeterministic-typecheck-chirho.md
            let mut drained_hints_chirho: Vec<_> =
                self.occurrence_default_hints_chirho.drain().collect();
            drained_hints_chirho.sort_by_key(|(var_chirho, _)| *var_chirho);
            for (var_chirho, key_chirho) in drained_hints_chirho {
                if let TyChirho::VarChirho(updated_var_chirho) =
                    subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(var_chirho))
                {
                    updated_hints_chirho.insert(updated_var_chirho, key_chirho);
                }
            }
            self.occurrence_default_hints_chirho = updated_hints_chirho;
        }
        if !self.occurrence_generalized_vars_chirho.is_empty() {
            let mut updated_vars_chirho = HashSet::new();
            for var_chirho in self.occurrence_generalized_vars_chirho.drain() {
                if let TyChirho::VarChirho(updated_var_chirho) =
                    subst_chirho.apply_ty_chirho(&TyChirho::VarChirho(var_chirho))
                {
                    updated_vars_chirho.insert(updated_var_chirho);
                }
            }
            self.occurrence_generalized_vars_chirho = updated_vars_chirho;
        }
        Ok(())
    }

    fn validate_projected_commit_chirho(
        &mut self,
        input_chirho: &SubstChirho,
        prepared_chirho: &CapturedVariableKindsChirho,
        skolems_chirho: &HashSet<String>,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut budget_chirho = self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)?;
        let initial_chirho = budget_chirho.remaining_chirho();
        let effective_chirho = prepared_chirho.effective_chirho().unwrap_or(input_chirho);
        let valid_chirho = (|| {
            let input_variables_chirho = self.input_rooted_kind_variables_chirho(
                input_chirho,
                prepared_chirho,
                &mut budget_chirho,
            )?;
            let mut view_chirho = ProjectedKindViewChirho {
                effective_chirho,
                prepared_chirho,
                skolems_chirho,
                budget_chirho: &mut budget_chirho,
            };
            for (variable_chirho, image_chirho) in effective_chirho.iter_chirho() {
                if !input_variables_chirho.contains(variable_chirho) {
                    return Some(Err(
                        "public kind solution has no input-rooted dependency proof",
                    ));
                }
                if !view_chirho.closed_chirho(image_chirho, &mut Vec::new(), 0)? {
                    return Some(Err("public kind solution reaches a private rigid variable"));
                }
                if let Some(kind_chirho) = prepared_chirho.prepared_kind_chirho(*variable_chirho) {
                    if !view_chirho.closed_chirho(kind_chirho, &mut Vec::new(), 0)? {
                        return Some(Err("public kind solution reaches a private rigid variable"));
                    }
                }
            }
            for (_, kind_chirho) in prepared_chirho.kinds_chirho() {
                if let Some(kind_chirho) = kind_chirho {
                    if !view_chirho.closed_chirho(kind_chirho, &mut Vec::new(), 0)? {
                        return Some(Err("public kind solution reaches a private rigid variable"));
                    }
                }
            }
            Some(Ok(()))
        })();
        self.elaboration_state_chirho
            .settle_equality_budget_chirho(
                initial_chirho,
                &budget_chirho,
                valid_chirho.is_none(),
                span_chirho,
            )?;
        if let Some(Err(message_chirho)) = valid_chirho {
            return Err(ElaborationErrorChirho {
                mismatch_chirho: None,
                message_chirho: message_chirho.into(),
                span_chirho,
            });
        }
        Ok(())
    }

    /// This temporary dependency walk admits no binding authority. A new K
    /// domain must be reachable from the explicit public solution and actual
    /// checked kind edges; the strict stage still requires its genuine Meta
    /// record. Read a new image's live pre-publication kind, not its already
    /// zonked plan kind, so TYPE r -> Type does not erase the reason for r.
    fn input_rooted_kind_variables_chirho(
        &self,
        input_chirho: &SubstChirho,
        prepared_chirho: &CapturedVariableKindsChirho,
        budget_chirho: &mut crate::infer_chirho::EqualityBudgetChirho,
    ) -> Option<HashSet<TyVarChirho>> {
        fn append_chirho(
            term_chirho: &TyChirho,
            pending_chirho: &mut Vec<TyVarChirho>,
            budget_chirho: &mut crate::infer_chirho::EqualityBudgetChirho,
        ) -> Option<()> {
            let variables_chirho = SubstChirho::free_variables_bounded_chirho(
                term_chirho,
                &mut |work_chirho, output_chirho| {
                    budget_chirho.node_chirho(output_chirho)?;
                    budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                },
            )?;
            budget_chirho.node_chirho(variables_chirho.len())?;
            pending_chirho.extend(variables_chirho);
            Some(())
        }

        let mut pending_chirho = Vec::new();
        for (variable_chirho, image_chirho) in input_chirho.iter_chirho() {
            budget_chirho.node_chirho(1)?;
            pending_chirho.push(*variable_chirho);
            append_chirho(image_chirho, &mut pending_chirho, budget_chirho)?;
            if let Some(kind_chirho) = input_chirho
                .lookup_image_chirho(variable_chirho)?
                .kind_chirho
                .as_deref()
            {
                append_chirho(kind_chirho, &mut pending_chirho, budget_chirho)?;
            }
        }
        for (variable_chirho, kind_chirho) in prepared_chirho.kinds_chirho() {
            budget_chirho.node_chirho(1)?;
            pending_chirho.push(variable_chirho);
            if let Some(kind_chirho) = kind_chirho {
                append_chirho(kind_chirho, &mut pending_chirho, budget_chirho)?;
            }
        }
        let mut variables_chirho = HashSet::new();
        while let Some(variable_chirho) = pending_chirho.pop() {
            budget_chirho.work_chirho(1)?;
            if variables_chirho.contains(&variable_chirho) {
                continue;
            }
            budget_chirho.node_chirho(1)?;
            variables_chirho.insert(variable_chirho);
            // Their actual PRE-entry edges were added above. Current private
            // links may not replace the captured permission/kind boundary.
            if prepared_chirho.contains_pre_entry_chirho(variable_chirho) {
                continue;
            }
            if let Some(kind_chirho) = self
                .elaboration_state_chirho
                .variable_kinds_chirho
                .get(&variable_chirho)
                .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
            {
                append_chirho(kind_chirho, &mut pending_chirho, budget_chirho)?;
                let checked_chirho = self
                    .elaboration_state_chirho
                    .substitution_chirho
                    .resolve_owned_bounded_chirho(
                        kind_chirho,
                        &mut |work_chirho, output_chirho| {
                            budget_chirho.node_chirho(output_chirho)?;
                            budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                        },
                    )?;
                append_chirho(&checked_chirho, &mut pending_chirho, budget_chirho)?;
            }
        }
        Some(variables_chirho)
    }

    /// Close only the explicitly threaded outward domains. This never exports
    /// working equality-map entries or adds implication givens to the result.
    pub(super) fn finish_public_substitution_chirho(
        &mut self,
        source_chirho: &SubstChirho,
    ) -> Result<SubstChirho, ElaborationErrorChirho> {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_commit_failure_chirho(&error_chirho);
                return Err(error_chirho);
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let result_chirho = (|| {
            let mut result_chirho = SubstChirho::empty_chirho();
            for (variable_chirho, _) in source_chirho.iter_chirho() {
                budget_chirho.node_chirho(1)?;
                let entry_chirho = source_chirho.lookup_image_chirho(variable_chirho)?;
                let image_chirho = source_chirho.resolve_owned_bounded_chirho(
                    &TyChirho::VarChirho(*variable_chirho),
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                )?;
                let kind_chirho = match entry_chirho.kind_chirho.as_ref() {
                    Some(kind_chirho) => {
                        Some(Arc::new(source_chirho.resolve_owned_bounded_chirho(
                            kind_chirho,
                            &mut |work_chirho, output_chirho| {
                                budget_chirho.node_chirho(output_chirho)?;
                                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                            },
                        )?))
                    }
                    None => None,
                };
                result_chirho.insert_image_chirho(
                    *variable_chirho,
                    SubstitutionImageChirho {
                        image_chirho,
                        kind_chirho,
                        source_chirho: entry_chirho.source_chirho.clone(),
                    },
                );
            }
            Some(result_chirho)
        })();
        match self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            result_chirho.is_none(),
            span_chirho,
        ) {
            Ok(()) => Ok(result_chirho.expect("a non-exhausted bounded result")),
            Err(error_chirho) => {
                self.record_commit_failure_chirho(&error_chirho);
                Err(error_chirho)
            }
        }
    }
}
