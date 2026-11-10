// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Inferred results are linear holes, never variables in the type language.
//! workflow: language-features-chirho/rank-n-visible-type-application-chirho

use super::*;

/// No Clone and no environment entry: only its owner may fill or join this
/// result. Ordinary type variables retain their existing monotype policy.
pub(in crate::infer_chirho) struct InferredResultChirho {
    owner_chirho: usize,
    ty_chirho: Option<TyChirho>,
}

impl InferredResultChirho {
    pub(in crate::infer_chirho) fn new_chirho(owner_chirho: usize) -> Self {
        Self {
            owner_chirho,
            ty_chirho: None,
        }
    }

    pub(in crate::infer_chirho) fn ty_chirho(&self) -> Option<&TyChirho> {
        self.ty_chirho.as_ref()
    }

    pub(in crate::infer_chirho) fn finish_chirho(self) -> Option<TyChirho> {
        self.ty_chirho
    }
}

impl InferCtxChirho {
    /// Infer each RHS independently. A deeper pattern implication requires a
    /// monotype on *every* join, including after an ordinary first branch.
    /// Copies and ancestry walks settle before unification borrows its loan.
    pub(in crate::infer_chirho) fn fill_inferred_result_chirho(
        &mut self,
        result_chirho: &mut InferredResultChirho,
        rhs_chirho: &TyChirho,
        substitution_chirho: &SubstChirho,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return SubstChirho::empty_chirho();
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let prepared_chirho = (|| {
            let mut charge_chirho = |work_chirho: usize, output_chirho: usize| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            };
            let mut rhs_chirho =
                substitution_chirho.apply_bounded_chirho(rhs_chirho, &mut charge_chirho)?;
            let previous_chirho = match &result_chirho.ty_chirho {
                Some(ty_chirho) => {
                    Some(substitution_chirho.apply_bounded_chirho(ty_chirho, &mut charge_chirho)?)
                }
                None => None,
            };
            let mut scope_chirho = self.active_equality_scope_chirho;
            let mut monomorphic_chirho = false;
            let mut escaping_chirho = false;
            while scope_chirho != result_chirho.owner_chirho {
                budget_chirho.work_chirho(1)?;
                let frame_chirho = self.equality_scopes_chirho.get(scope_chirho)?;
                monomorphic_chirho |= frame_chirho.requires_monomorphic_result_chirho();
                if previous_chirho.is_none() {
                    let projected_chirho = self
                        .project_pattern_result_type_chirho(
                            scope_chirho,
                            &rhs_chirho,
                            &mut budget_chirho,
                        )
                        .ok()?;
                    rhs_chirho = projected_chirho.0;
                    escaping_chirho |= projected_chirho.1;
                }
                scope_chirho = frame_chirho.parent_chirho()?;
            }
            let rhs_is_monotype_chirho = crate::unify_chirho::is_monotype_bounded_chirho(
                &rhs_chirho,
                &mut |work_chirho, output_chirho| {
                    budget_chirho.node_chirho(output_chirho)?;
                    budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                },
            )
            .ok()?;
            Some((
                previous_chirho,
                rhs_chirho,
                monomorphic_chirho,
                rhs_is_monotype_chirho,
                escaping_chirho,
            ))
        })();
        if let Err(error_chirho) = self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            prepared_chirho.is_none(),
            span_chirho,
        ) {
            self.record_body_kind_error_chirho(error_chirho);
            return SubstChirho::empty_chirho();
        }
        let Some((
            previous_chirho,
            rhs_chirho,
            monomorphic_chirho,
            rhs_is_monotype_chirho,
            escaping_chirho,
        )) = prepared_chirho
        else {
            return SubstChirho::empty_chirho();
        };
        if escaping_chirho {
            self.diagnostics_chirho.push_chirho(
                haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                    haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(200),
                    "constructor pattern's existential type would escape its scope",
                    span_chirho,
                ),
            );
            return SubstChirho::empty_chirho();
        }
        let mut joined_chirho = SubstChirho::empty_chirho();
        if monomorphic_chirho {
            for ty_chirho in previous_chirho.iter().chain(std::iter::once(&rhs_chirho)) {
                let monotype_chirho = self.fresh_value_meta_chirho(span_chirho);
                match self.unify_ordinary_chirho(&monotype_chirho, ty_chirho, span_chirho) {
                    Ok(mut step_chirho) => {
                        if self.apply_subst_all_chirho(&mut step_chirho).is_err() {
                            return SubstChirho::empty_chirho();
                        }
                        joined_chirho = step_chirho.compose_chirho(&joined_chirho);
                    }
                    Err(error_chirho) => {
                        self.report_unify_error_chirho(&error_chirho);
                        return SubstChirho::empty_chirho();
                    }
                }
            }
        }
        // A non-polymorphic inferred value still follows the old Type-kind
        // projection. Its resulting kind equations must be returned to the
        // caller, not merely installed in live elaboration metadata.
        if !monomorphic_chirho && rhs_is_monotype_chirho {
            let value_chirho = self.fresh_value_meta_chirho(span_chirho);
            match self.unify_ordinary_chirho(&value_chirho, &rhs_chirho, span_chirho) {
                Ok(mut step_chirho) => {
                    if self.apply_subst_all_chirho(&mut step_chirho).is_err() {
                        return SubstChirho::empty_chirho();
                    }
                    joined_chirho = step_chirho.compose_chirho(&joined_chirho);
                }
                Err(error_chirho) => {
                    self.report_unify_error_chirho(&error_chirho);
                    return SubstChirho::empty_chirho();
                }
            }
        }
        if let Some(previous_chirho) = &previous_chirho {
            match self.unify_normalized_chirho(previous_chirho, &rhs_chirho, span_chirho) {
                Ok(mut step_chirho) => {
                    if self.apply_subst_all_chirho(&mut step_chirho).is_err() {
                        return SubstChirho::empty_chirho();
                    }
                    joined_chirho = step_chirho.compose_chirho(&joined_chirho);
                }
                Err(error_chirho) => {
                    self.report_unify_error_chirho(&error_chirho);
                    return SubstChirho::empty_chirho();
                }
            }
        }
        // Only a first fill publishes a public type. Later alternatives check
        // that same type under their local evidence and may never replace it.
        // The caller applies only the public substitution after scope exit.
        if result_chirho.ty_chirho.is_none() {
            result_chirho.ty_chirho = Some(rhs_chirho);
        }
        joined_chirho
    }
}
