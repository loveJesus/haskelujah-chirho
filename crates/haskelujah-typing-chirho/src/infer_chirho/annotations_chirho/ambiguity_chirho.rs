// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equality-context ambiguity at source signatures. Two independently opened
//! copies share the existing catalogue/evidence, but only scratch fresh metas
//! can be solved. No live environment, wanted queue or substitution is edited.
//! Class/fundep and nested-sigma determination remain explicit unsupported cases.

use super::super::equality_normalization_chirho::{
    EqualityBudgetChirho, copy_term_with_budget_chirho, equal_chirho,
    substitute_meta_with_budget_chirho,
};
use super::*;
use crate::TyVarChirho;

#[derive(Clone, Copy)]
struct SignatureProbeEvidenceChirho<'evidence_chirho> {
    rules_chirho: &'evidence_chirho [(TyChirho, TyChirho)],
    givens_chirho: &'evidence_chirho [(TyChirho, TyChirho)],
}

enum SignatureDeterminationChirho {
    SupportedChirho { determined_chirho: bool },
    UnsupportedChirho,
}

impl InferCtxChirho {
    pub(in crate::infer_chirho) fn check_source_signature_ambiguity_chirho(
        &mut self,
        syntax_chirho: &TypeChirho,
    ) {
        if self.elaboration_state_chirho.owner_chirho.is_none()
            || self
                .type_validity_license_chirho
                .is_none_or(|license_chirho| license_chirho.allow_ambiguous_enabled_chirho())
        {
            return;
        }
        let scheme_chirho = self.ast_type_to_scheme_chirho(syntax_chirho);
        if scheme_chirho.preds_chirho.is_empty()
            || scheme_chirho.preds_chirho.iter().any(|predicate_chirho| {
                predicate_chirho.class_name_chirho != "~"
                    || predicate_chirho.extra_tys_chirho.len() != 1
            })
        {
            return;
        }
        let span_chirho = syntax_chirho.span_chirho();
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return;
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let result_chirho =
            self.probe_signature_equalities_chirho(&scheme_chirho, &mut budget_chirho, span_chirho);
        if let Err(error_chirho) = self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            result_chirho.is_none(),
            span_chirho,
        ) {
            self.record_body_kind_error_chirho(error_chirho);
            return;
        }
        if matches!(
            result_chirho,
            Some(SignatureDeterminationChirho::SupportedChirho {
                determined_chirho: false
            })
        ) {
            self.diagnostics_chirho.push_chirho(
                haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                    haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(
                        super::super::TYPE_MISMATCH_CODE_CHIRHO,
                    ),
                    "ambiguous signature: its equality context cannot be determined from its type",
                    span_chirho,
                ),
            );
        }
    }

    fn probe_signature_equalities_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        budget_chirho: &mut EqualityBudgetChirho,
        span_chirho: SpanChirho,
    ) -> Option<SignatureDeterminationChirho> {
        let mut captures_chirho = SubstChirho::empty_chirho();
        let mut free_chirho = std::collections::HashSet::new();
        for term_chirho in std::iter::once(&scheme_chirho.ty_chirho)
            .chain(
                scheme_chirho
                    .preds_chirho
                    .iter()
                    .flat_map(|predicate_chirho| predicate_chirho.semantic_tys_chirho()),
            )
            .chain(
                scheme_chirho
                    .vars_chirho
                    .records_chirho()
                    .iter()
                    .filter_map(|record_chirho| record_chirho.kind_chirho.as_deref()),
            )
        {
            if !self.probe_family_dependencies_supported_chirho(term_chirho, budget_chirho)? {
                return Some(SignatureDeterminationChirho::UnsupportedChirho);
            }
            free_chirho.extend(SubstChirho::free_variables_bounded_chirho(
                term_chirho,
                &mut |work_chirho, output_chirho| {
                    budget_chirho.node_chirho(output_chirho)?;
                    budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                },
            )?);
        }
        for variable_chirho in free_chirho
            .into_iter()
            .filter(|variable_chirho| !scheme_chirho.vars_chirho.contains(variable_chirho))
        {
            budget_chirho.node_chirho(1)?;
            let Some(image_chirho) = self
                .skolem_bindings_chirho
                .lookup_image_chirho(&variable_chirho)
            else {
                return Some(SignatureDeterminationChirho::UnsupportedChirho);
            };
            let term_chirho =
                copy_term_with_budget_chirho(&image_chirho.image_chirho, budget_chirho)?;
            captures_chirho.insert_image_chirho(
                variable_chirho,
                crate::subst_chirho::SubstitutionImageChirho {
                    image_chirho: term_chirho,
                    kind_chirho: image_chirho.kind_chirho.clone(),
                    source_chirho: image_chirho.source_chirho.clone(),
                },
            );
        }
        budget_chirho.node_chirho(captures_chirho.iter_chirho().count())?;
        let mut wanted_prefix_chirho = SubstChirho::empty_chirho();
        for (variable_chirho, term_chirho) in captures_chirho.iter_chirho() {
            let image_chirho = captures_chirho.lookup_image_chirho(variable_chirho)?;
            wanted_prefix_chirho.insert_image_chirho(
                *variable_chirho,
                crate::subst_chirho::SubstitutionImageChirho {
                    image_chirho: copy_term_with_budget_chirho(term_chirho, budget_chirho)?,
                    kind_chirho: image_chirho.kind_chirho.clone(),
                    source_chirho: image_chirho.source_chirho.clone(),
                },
            );
        }
        let mut rigid_prefix_chirho = captures_chirho;
        let floor_chirho = self.elaboration_state_chirho.next_variable_chirho;
        for record_chirho in scheme_chirho.vars_chirho.records_chirho() {
            let kind_chirho = self.instantiate_quantifier_kind_chirho(
                record_chirho,
                &wanted_prefix_chirho,
                budget_chirho,
            )?;
            let kind_chirho = match kind_chirho.as_deref() {
                Some(kind_chirho) => {
                    Some(copy_term_with_budget_chirho(kind_chirho, budget_chirho)?)
                }
                None => None,
            };
            let variable_chirho = self
                .elaboration_state_chirho
                .fresh_chirho(kind_chirho, span_chirho)
                .ok()?;
            wanted_prefix_chirho.insert_chirho(
                record_chirho.variable_chirho,
                TyChirho::VarChirho(variable_chirho),
            );
            let rigid_chirho = self.fresh_rigid_quantifier_image_chirho(
                record_chirho,
                &rigid_prefix_chirho,
                budget_chirho,
            )?;
            rigid_prefix_chirho.insert_image_chirho(record_chirho.variable_chirho, rigid_chirho);
        }
        let ceiling_chirho = self.elaboration_state_chirho.next_variable_chirho;
        let (wanted_body_chirho, wanteds_chirho) = self.instantiate_scheme_terms_chirho(
            scheme_chirho,
            &scheme_chirho.ty_chirho,
            &wanted_prefix_chirho,
            budget_chirho,
        )?;
        let (given_body_chirho, givens_chirho) = self.instantiate_scheme_terms_chirho(
            scheme_chirho,
            &scheme_chirho.ty_chirho,
            &rigid_prefix_chirho,
            budget_chirho,
        )?;
        let (wanted_body_chirho, wanteds_chirho) =
            self.expose_qualified_parts_chirho(wanted_body_chirho, wanteds_chirho);
        let (given_body_chirho, givens_chirho) =
            self.expose_qualified_parts_chirho(given_body_chirho, givens_chirho);
        if wanteds_chirho
            .iter()
            .chain(&givens_chirho)
            .any(|predicate_chirho| {
                predicate_chirho.class_name_chirho != "~"
                    || predicate_chirho.extra_tys_chirho.len() != 1
            })
        {
            return Some(SignatureDeterminationChirho::UnsupportedChirho);
        }
        let mut rules_chirho = Vec::new();
        let mut given_pairs_chirho = Vec::new();
        let empty_chirho = SubstChirho::empty_chirho();
        for given_chirho in givens_chirho {
            let lhs_chirho = self.normalize_signature_probe_chirho(
                &given_chirho.ty_chirho,
                &empty_chirho,
                &[],
                budget_chirho,
            )?;
            let rhs_chirho = self.normalize_signature_probe_chirho(
                &given_chirho.extra_tys_chirho[0],
                &empty_chirho,
                &[],
                budget_chirho,
            )?;
            budget_chirho.node_chirho(1)?;
            given_pairs_chirho.push((
                copy_term_with_budget_chirho(&lhs_chirho, budget_chirho)?,
                copy_term_with_budget_chirho(&rhs_chirho, budget_chirho)?,
            ));
            if let Some(rule_chirho) = self.given_equality_rule_chirho(lhs_chirho, rhs_chirho) {
                budget_chirho.node_chirho(1)?;
                rules_chirho.push(rule_chirho);
            }
        }
        for (left_chirho, right_chirho) in &given_pairs_chirho {
            for term_chirho in [left_chirho, right_chirho] {
                let free_chirho = SubstChirho::free_variables_bounded_chirho(
                    term_chirho,
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                )?;
                if free_chirho.iter().any(|variable_chirho| {
                    (floor_chirho..ceiling_chirho).contains(&variable_chirho.0)
                }) {
                    return Some(SignatureDeterminationChirho::UnsupportedChirho);
                }
            }
        }
        let rules_chirho =
            super::super::equality_normalization_chirho::prepare_owned_equality_rewrites_chirho(
                self,
                std::iter::once(rules_chirho.as_slice()).chain(
                    self.equality_scopes_chirho[self.active_equality_scope_chirho]
                        .evidence_view_chirho()
                        .iter_chirho(),
                ),
                budget_chirho,
            )
            .ok()?;
        // Scratch substitutions only bind the new wanted range; they cannot
        // change these rigid/captured premises. Normalize them once, too.
        for (left_chirho, right_chirho) in &mut given_pairs_chirho {
            *left_chirho = self.normalize_prepared_signature_probe_chirho(
                left_chirho,
                &empty_chirho,
                &rules_chirho,
                budget_chirho,
            )?;
            *right_chirho = self.normalize_prepared_signature_probe_chirho(
                right_chirho,
                &empty_chirho,
                &rules_chirho,
                budget_chirho,
            )?;
        }
        budget_chirho.node_chirho(
            rules_chirho
                .len()
                .saturating_add(wanteds_chirho.len())
                .saturating_add(1),
        )?;
        let mut pending_chirho = vec![(wanted_body_chirho, given_body_chirho, 0usize, true)];
        for wanted_chirho in wanteds_chirho {
            pending_chirho.push((
                wanted_chirho.ty_chirho,
                wanted_chirho.extra_tys_chirho.into_iter().next()?,
                0,
                false,
            ));
        }
        self.solve_signature_probe_chirho(
            pending_chirho,
            SignatureProbeEvidenceChirho {
                rules_chirho: &rules_chirho,
                givens_chirho: &given_pairs_chirho,
            },
            floor_chirho..ceiling_chirho,
            budget_chirho,
            span_chirho,
        )
    }

    fn probe_family_dependencies_supported_chirho(
        &self,
        term_chirho: &TyChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<bool> {
        let mut pending_chirho = vec![(term_chirho, 0usize)];
        while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
            budget_chirho.work_chirho(1)?;
            if depth_chirho >= 128 {
                return None;
            }
            match term_chirho {
                TyChirho::ConChirho(name_chirho) => {
                    budget_chirho.work_chirho(name_chirho.len())?;
                    if let Some(key_chirho) = self.family_name_key_chirho(name_chirho) {
                        let supported_chirho = self
                            .type_families_chirho
                            .declaration_schemes_chirho()
                            .get_chirho(&key_chirho)
                            .is_some_and(|header_chirho| {
                                header_chirho.owner_chirho.is_none()
                                    && !header_chirho.held_chirho
                                    && header_chirho.injectivity_chirho == Some(false)
                            });
                        if !supported_chirho {
                            return Some(false);
                        }
                    }
                }
                TyChirho::AppChirho(left_chirho, right_chirho)
                | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                    budget_chirho.node_chirho(2)?;
                    pending_chirho.push((left_chirho, depth_chirho + 1));
                    pending_chirho.push((right_chirho, depth_chirho + 1));
                }
                TyChirho::ListChirho(inner_chirho) => {
                    budget_chirho.node_chirho(1)?;
                    pending_chirho.push((inner_chirho, depth_chirho + 1));
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    budget_chirho.node_chirho(elements_chirho.len())?;
                    pending_chirho.extend(
                        elements_chirho
                            .iter()
                            .map(|term_chirho| (term_chirho, depth_chirho + 1)),
                    );
                }
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    budget_chirho.node_chirho(vars_chirho.len().saturating_add(1))?;
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                    pending_chirho.extend(
                        vars_chirho
                            .records_chirho()
                            .iter()
                            .filter_map(|record_chirho| record_chirho.kind_chirho.as_deref())
                            .map(|kind_chirho| (kind_chirho, depth_chirho + 1)),
                    );
                }
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    budget_chirho.node_chirho(context_chirho.len().saturating_add(1))?;
                    pending_chirho.push((body_chirho, depth_chirho + 1));
                    pending_chirho.extend(
                        context_chirho
                            .iter()
                            .map(|term_chirho| (term_chirho, depth_chirho + 1)),
                    );
                }
                TyChirho::VarChirho(_) | TyChirho::ForallVarChirho(_) => {}
            }
        }
        Some(true)
    }

    fn normalize_signature_probe_chirho(
        &self,
        term_chirho: &TyChirho,
        substitution_chirho: &SubstChirho,
        rules_chirho: &[(TyChirho, TyChirho)],
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<TyChirho> {
        let term_chirho = substitution_chirho.resolve_owned_bounded_chirho(
            term_chirho,
            &mut |work_chirho, output_chirho| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            },
        )?;
        super::super::EqualityNormalizationViewChirho {
            substitutions_chirho: &self.equality_substitutions_chirho,
            legacy_synonyms_chirho: &self.type_synonyms_chirho,
            type_scope_chirho: self.type_constructor_scope_chirho.as_deref(),
            family_scope_chirho: self.family_scope_chirho.as_deref(),
            family_module_chirho: &self.family_module_name_chirho,
            promotion_present_chirho: self.promotion_scope_chirho.is_some(),
            evidence_chirho: self.equality_scopes_chirho[self.active_equality_scope_chirho]
                .evidence_view_chirho(),
        }
        .normalize_with_probe_rules_chirho(
            &self.type_families_chirho,
            self.elaboration_state_chirho.owner_chirho.is_some(),
            self.elaboration_state_chirho.next_variable_chirho,
            &term_chirho,
            rules_chirho,
            budget_chirho,
        )
        .ok()
    }

    fn normalize_prepared_signature_probe_chirho(
        &self,
        term_chirho: &TyChirho,
        substitution_chirho: &SubstChirho,
        rules_chirho: &[(TyChirho, TyChirho)],
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<TyChirho> {
        let term_chirho = substitution_chirho.resolve_owned_bounded_chirho(
            term_chirho,
            &mut |work_chirho, output_chirho| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            },
        )?;
        super::super::equality_normalization_chirho::normalize_prepared_owned_equality_chirho(
            self,
            &term_chirho,
            rules_chirho,
            budget_chirho,
        )
        .ok()
    }

    /// Improve only from complete outstanding body equalities. Normalize the
    /// unassigned image first; no guessed substitution is used to select it.
    /// The proposed assignment must then prove the original context equality
    /// under actual givens alone. Body obligations retain their original check.
    fn improve_signature_probe_by_body_chirho(
        &self,
        variable_chirho: TyVarChirho,
        wanted_chirho: (&TyChirho, &TyChirho),
        substitution_chirho: &SubstChirho,
        rules_chirho: &[(TyChirho, TyChirho)],
        support_chirho: &[(TyChirho, TyChirho)],
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Option<Option<TyChirho>> {
        let (left_chirho, right_chirho) = wanted_chirho;
        if support_chirho.is_empty() {
            return Some(None);
        }
        let image_chirho = if matches!(left_chirho, TyChirho::VarChirho(actual_chirho) if *actual_chirho == variable_chirho)
        {
            right_chirho
        } else {
            left_chirho
        };
        budget_chirho.node_chirho(rules_chirho.len().saturating_add(support_chirho.len()))?;
        let mut combined_chirho = Vec::new();
        for (left_chirho, right_chirho) in rules_chirho {
            combined_chirho.push((
                copy_term_with_budget_chirho(left_chirho, budget_chirho)?,
                copy_term_with_budget_chirho(right_chirho, budget_chirho)?,
            ));
        }
        for (left_chirho, right_chirho) in support_chirho {
            let left_chirho = self.normalize_prepared_signature_probe_chirho(
                left_chirho,
                substitution_chirho,
                rules_chirho,
                budget_chirho,
            )?;
            let right_chirho = self.normalize_prepared_signature_probe_chirho(
                right_chirho,
                substitution_chirho,
                rules_chirho,
                budget_chirho,
            )?;
            if let Some(rule_chirho) = self.given_equality_rule_chirho(left_chirho, right_chirho) {
                combined_chirho.push(rule_chirho);
            }
        }
        let candidate_chirho = self.normalize_prepared_signature_probe_chirho(
            image_chirho,
            substitution_chirho,
            &combined_chirho,
            budget_chirho,
        )?;
        let free_chirho = SubstChirho::free_variables_bounded_chirho(
            &candidate_chirho,
            &mut |work_chirho, output_chirho| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            },
        )?;
        if free_chirho.contains(&variable_chirho) {
            return Some(None);
        }
        let left_chirho = substitute_meta_with_budget_chirho(
            left_chirho,
            variable_chirho,
            &candidate_chirho,
            budget_chirho,
        )?;
        let right_chirho = substitute_meta_with_budget_chirho(
            right_chirho,
            variable_chirho,
            &candidate_chirho,
            budget_chirho,
        )?;
        let left_chirho = self.normalize_prepared_signature_probe_chirho(
            &left_chirho,
            substitution_chirho,
            rules_chirho,
            budget_chirho,
        )?;
        let right_chirho = self.normalize_prepared_signature_probe_chirho(
            &right_chirho,
            substitution_chirho,
            rules_chirho,
            budget_chirho,
        )?;
        if equal_chirho(&left_chirho, &right_chirho, budget_chirho)? {
            Some(Some(candidate_chirho))
        } else {
            Some(None)
        }
    }

    fn solve_signature_probe_chirho(
        &self,
        mut pending_chirho: Vec<(TyChirho, TyChirho, usize, bool)>,
        evidence_chirho: SignatureProbeEvidenceChirho<'_>,
        fresh_chirho: std::ops::Range<u32>,
        budget_chirho: &mut EqualityBudgetChirho,
        span_chirho: SpanChirho,
    ) -> Option<SignatureDeterminationChirho> {
        let mut substitution_chirho = SubstChirho::empty_chirho();
        let mut body_support_chirho = Vec::new();
        for _round_chirho in 0..128 {
            let mut deferred_chirho = Vec::new();
            let mut progressed_chirho = false;
            while let Some((left_chirho, right_chirho, depth_chirho, body_chirho)) =
                pending_chirho.pop()
            {
                budget_chirho.work_chirho(1)?;
                if depth_chirho >= 128 {
                    return None;
                }
                let left_chirho = self.normalize_prepared_signature_probe_chirho(
                    &left_chirho,
                    &substitution_chirho,
                    evidence_chirho.rules_chirho,
                    budget_chirho,
                )?;
                let right_chirho = self.normalize_prepared_signature_probe_chirho(
                    &right_chirho,
                    &substitution_chirho,
                    evidence_chirho.rules_chirho,
                    budget_chirho,
                )?;
                if equal_chirho(&left_chirho, &right_chirho, budget_chirho)? {
                    progressed_chirho = true;
                    continue;
                }
                let mut entailed_chirho = false;
                for (given_left_chirho, given_right_chirho) in evidence_chirho.givens_chirho {
                    if (equal_chirho(&left_chirho, given_left_chirho, budget_chirho)?
                        && equal_chirho(&right_chirho, given_right_chirho, budget_chirho)?)
                        || (equal_chirho(&left_chirho, given_right_chirho, budget_chirho)?
                            && equal_chirho(&right_chirho, given_left_chirho, budget_chirho)?)
                    {
                        entailed_chirho = true;
                        break;
                    }
                }
                if entailed_chirho {
                    progressed_chirho = true;
                    continue;
                }
                let assignment_chirho = match (&left_chirho, &right_chirho) {
                    (TyChirho::VarChirho(variable_chirho), _)
                        if fresh_chirho.contains(&variable_chirho.0) =>
                    {
                        Some((*variable_chirho, &right_chirho))
                    }
                    (_, TyChirho::VarChirho(variable_chirho))
                        if fresh_chirho.contains(&variable_chirho.0) =>
                    {
                        Some((*variable_chirho, &left_chirho))
                    }
                    _ => None,
                };
                if let Some((variable_chirho, image_chirho)) = assignment_chirho {
                    match crate::unify_chirho::unify_chirho(
                        &TyChirho::VarChirho(variable_chirho),
                        image_chirho,
                        span_chirho,
                    ) {
                        Ok(solution_chirho) => {
                            for (variable_chirho, image_chirho) in solution_chirho.iter_chirho() {
                                if !fresh_chirho.contains(&variable_chirho.0) {
                                    return Some(SignatureDeterminationChirho::UnsupportedChirho);
                                }
                                substitution_chirho.insert_chirho(
                                    *variable_chirho,
                                    copy_term_with_budget_chirho(image_chirho, budget_chirho)?,
                                );
                            }
                            progressed_chirho = true;
                            continue;
                        }
                        Err(crate::unify_chirho::UnifyErrorChirho::ResourceExhaustedChirho {
                            ..
                        }) => return None,
                        Err(crate::unify_chirho::UnifyErrorChirho::OccursCheckChirho {
                            ..
                        }) if self.ty_mentions_family_chirho(image_chirho) => {
                            if !body_chirho
                                && let Some(image_chirho) = self
                                    .improve_signature_probe_by_body_chirho(
                                        variable_chirho,
                                        (&left_chirho, &right_chirho),
                                        &substitution_chirho,
                                        evidence_chirho.rules_chirho,
                                        &body_support_chirho,
                                        budget_chirho,
                                    )?
                            {
                                match crate::unify_chirho::unify_chirho(&TyChirho::VarChirho(variable_chirho), &image_chirho, span_chirho) {
                                    Ok(_) => { substitution_chirho.insert_chirho(variable_chirho, image_chirho); progressed_chirho = true; continue; }
                                    Err(crate::unify_chirho::UnifyErrorChirho::ResourceExhaustedChirho { .. }) => return None,
                                    Err(_) => {}
                                }
                            }
                            budget_chirho.node_chirho(1)?;
                            deferred_chirho.push((
                                left_chirho,
                                right_chirho,
                                depth_chirho,
                                body_chirho,
                            ));
                            continue;
                        }
                        Err(_) => {
                            return Some(SignatureDeterminationChirho::SupportedChirho {
                                determined_chirho: false,
                            });
                        }
                    }
                }
                // Family applications are opaque before decomposition. In
                // particular K beta ~ K skolem never establishes beta=skolem.
                if self.ty_is_family_app_chirho(&left_chirho)
                    || self.ty_is_family_app_chirho(&right_chirho)
                {
                    budget_chirho.node_chirho(1)?;
                    deferred_chirho.push((left_chirho, right_chirho, depth_chirho, body_chirho));
                    continue;
                }
                budget_chirho.node_chirho(1)?;
                match (left_chirho, right_chirho) {
                    (
                        TyChirho::AppChirho(left_fun_chirho, left_arg_chirho),
                        TyChirho::AppChirho(right_fun_chirho, right_arg_chirho),
                    ) => {
                        pending_chirho.push((
                            *left_fun_chirho,
                            *right_fun_chirho,
                            depth_chirho + 1,
                            body_chirho,
                        ));
                        pending_chirho.push((
                            *left_arg_chirho,
                            *right_arg_chirho,
                            depth_chirho + 1,
                            body_chirho,
                        ));
                    }
                    (
                        TyChirho::FunChirho(left_arg_chirho, left_result_chirho, left_mult_chirho),
                        TyChirho::FunChirho(
                            right_arg_chirho,
                            right_result_chirho,
                            right_mult_chirho,
                        ),
                    ) if left_mult_chirho == right_mult_chirho => {
                        pending_chirho.push((
                            *left_arg_chirho,
                            *right_arg_chirho,
                            depth_chirho + 1,
                            body_chirho,
                        ));
                        pending_chirho.push((
                            *left_result_chirho,
                            *right_result_chirho,
                            depth_chirho + 1,
                            body_chirho,
                        ));
                    }
                    (TyChirho::ListChirho(left_chirho), TyChirho::ListChirho(right_chirho)) => {
                        pending_chirho.push((
                            *left_chirho,
                            *right_chirho,
                            depth_chirho + 1,
                            body_chirho,
                        ))
                    }
                    (TyChirho::TupleChirho(left_chirho), TyChirho::TupleChirho(right_chirho))
                        if left_chirho.len() == right_chirho.len() =>
                    {
                        budget_chirho.node_chirho(left_chirho.len())?;
                        pending_chirho.extend(left_chirho.into_iter().zip(right_chirho).map(
                            |(left_chirho, right_chirho)| {
                                (left_chirho, right_chirho, depth_chirho + 1, body_chirho)
                            },
                        ));
                    }
                    (
                        TyChirho::ForallChirho { .. }
                        | TyChirho::RequiredForallChirho { .. }
                        | TyChirho::QualChirho { .. },
                        _,
                    )
                    | (
                        _,
                        TyChirho::ForallChirho { .. }
                        | TyChirho::RequiredForallChirho { .. }
                        | TyChirho::QualChirho { .. },
                    ) => return Some(SignatureDeterminationChirho::UnsupportedChirho),
                    _ => {
                        return Some(SignatureDeterminationChirho::SupportedChirho {
                            determined_chirho: false,
                        });
                    }
                }
            }
            if deferred_chirho.is_empty() {
                return Some(SignatureDeterminationChirho::SupportedChirho {
                    determined_chirho: true,
                });
            }
            if !progressed_chirho {
                if body_support_chirho.is_empty() {
                    // These are still obligations, never new givens. They may
                    // justify congruence while deriving a context improvement,
                    // but must all close under the actual givens afterwards.
                    for (left_chirho, right_chirho, _, body_chirho) in &deferred_chirho {
                        if *body_chirho
                            && (self.ty_is_family_app_chirho(left_chirho)
                                || self.ty_is_family_app_chirho(right_chirho))
                        {
                            let left_chirho =
                                copy_term_with_budget_chirho(left_chirho, budget_chirho)?;
                            let right_chirho =
                                copy_term_with_budget_chirho(right_chirho, budget_chirho)?;
                            if let Some(rule_chirho) =
                                self.given_equality_rule_chirho(left_chirho, right_chirho)
                            {
                                budget_chirho.node_chirho(1)?;
                                body_support_chirho.push(rule_chirho);
                            }
                        }
                    }
                    if !body_support_chirho.is_empty() {
                        pending_chirho = deferred_chirho;
                        continue;
                    }
                }
                return Some(SignatureDeterminationChirho::SupportedChirho {
                    determined_chirho: false,
                });
            }
            pending_chirho = deferred_chirho;
        }
        None
    }
}
