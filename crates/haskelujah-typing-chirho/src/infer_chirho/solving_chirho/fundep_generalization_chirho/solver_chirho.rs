// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Close uniquely selected typed instance trees before Rule 2 or legacy defaulting.
//! Supply, metadata, equalities, captures and wanteds remain untouched on a miss.

use super::super::equality_normalization_chirho::EqualityBudgetChirho;
use super::InferCtxChirho;
use super::candidate_chirho::{
    FundepCandidateChirho, charge_chirho, copy_predicate_chirho, same_predicate_chirho,
};
use crate::class_chirho::{InstDeclChirho, PredChirho};
use crate::kind_chirho::elaboration_chirho::{
    ElaborationVariableBindingChirho, check_monomorphic_kind_chirho,
};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use crate::unify_chirho::unify_monomorphic_bounded_chirho;
use haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho;
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ElaborationOwnerChirho;
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashSet;
use std::sync::Arc;

const FUNDEP_DEPTH_LIMIT_CHIRHO: usize = 64;
type FundepResultChirho<ValueChirho> = Result<ValueChirho, FundepFailureChirho>;

/// Private diagnostic reason only. It never decides a candidate's result.
enum FundepFailureChirho {
    BoundedChirho(Option<&'static str>),
    DepthChirho {
        actual_chirho: usize,
        limit_chirho: usize,
    },
}

impl From<()> for FundepFailureChirho {
    fn from(_: ()) -> Self {
        Self::BoundedChirho(None)
    }
}

impl FundepFailureChirho {
    fn bounded_chirho(segment_chirho: &'static str) -> Self {
        Self::BoundedChirho(Some(segment_chirho))
    }

    fn detail_chirho(&self) -> String {
        match self {
            Self::BoundedChirho(Some(segment_chirho)) => {
                format!("FD solving; segment={segment_chirho}")
            }
            Self::BoundedChirho(None) => {
                "FD solving; unattributed nested bounded operation".to_owned()
            }
            Self::DepthChirho {
                actual_chirho,
                limit_chirho,
            } => format!(
                "FD solving; guard=FD tree depth, actual={actual_chirho}, limit={limit_chirho}"
            ),
        }
    }
}

#[derive(Clone, Copy)]
enum FundepNormalizationStageChirho {
    FirstResolveChirho,
    PublicNormalizeChirho,
    ContractCompareChirho,
    FinalResolveChirho,
}

impl FundepNormalizationStageChirho {
    fn label_chirho(self) -> &'static str {
        match self {
            Self::FirstResolveChirho => "FirstR",
            Self::PublicNormalizeChirho => "PublicN",
            Self::ContractCompareChirho => "ContractCompare",
            Self::FinalResolveChirho => "FinalR",
        }
    }
}

/// Four fixed diagnostic counters. They never supply authority or allowance.
#[derive(Clone, Copy, Default)]
struct FundepNormalizationMetricChirho {
    calls_chirho: usize,
    spent_work_chirho: usize,
    spent_output_chirho: usize,
}

struct FundepSolveChirho<'context_chirho> {
    context_chirho: &'context_chirho mut InferCtxChirho,
    owner_chirho: ElaborationOwnerChirho,
    base_next_chirho: u32,
    budget_chirho: EqualityBudgetChirho,
    loan_chirho: (usize, usize),
    span_chirho: SpanChirho,
    normalization_metrics_chirho: [FundepNormalizationMetricChirho; 4],
}

impl FundepSolveChirho<'_> {
    fn record_normalization_chirho(
        &mut self,
        stage_chirho: FundepNormalizationStageChirho,
        before_chirho: (usize, usize),
    ) {
        let after_chirho = self.budget_chirho.remaining_chirho();
        let metric_chirho = &mut self.normalization_metrics_chirho[stage_chirho as usize];
        metric_chirho.calls_chirho = metric_chirho.calls_chirho.saturating_add(1);
        metric_chirho.spent_work_chirho = metric_chirho
            .spent_work_chirho
            .saturating_add(before_chirho.0.saturating_sub(after_chirho.0));
        metric_chirho.spent_output_chirho = metric_chirho
            .spent_output_chirho
            .saturating_add(before_chirho.1.saturating_sub(after_chirho.1));
    }

    fn normalization_detail_chirho(&self) -> String {
        let stages_chirho = [
            FundepNormalizationStageChirho::FirstResolveChirho,
            FundepNormalizationStageChirho::PublicNormalizeChirho,
            FundepNormalizationStageChirho::ContractCompareChirho,
            FundepNormalizationStageChirho::FinalResolveChirho,
        ];
        let fields_chirho = stages_chirho
            .into_iter()
            .map(|stage_chirho| {
                let metric_chirho = &self.normalization_metrics_chirho[stage_chirho as usize];
                format!(
                    "{}(calls={}, work={}, output={})",
                    stage_chirho.label_chirho(),
                    metric_chirho.calls_chirho,
                    metric_chirho.spent_work_chirho,
                    metric_chirho.spent_output_chirho,
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "normalization stage totals=[{fields_chirho}]; other FD/P2 costs excluded; counters are diagnostic only"
        )
    }

    fn eligible_chirho(
        &mut self,
        proof_chirho: &PredicateClassProofChirho,
    ) -> FundepResultChirho<bool> {
        let PredicateClassProofChirho::ProvenChirho(identity_chirho) = proof_chirho else {
            return Ok(false);
        };
        let weight_chirho = match identity_chirho {
            haskelujah_ast_chirho::name_chirho::PredicateClassIdentityChirho::CompilerChirho(_) => {
                1
            }
            haskelujah_ast_chirho::name_chirho::PredicateClassIdentityChirho::SourceChirho {
                unit_chirho,
                module_chirho,
                name_chirho,
            } => unit_chirho
                .as_str_chirho()
                .len()
                .checked_add(module_chirho.len())
                .and_then(|weight_chirho| weight_chirho.checked_add(name_chirho.len()))
                .ok_or(FundepFailureChirho::bounded_chirho("FD eligibility"))?,
        };
        charge_chirho(&mut self.budget_chirho, weight_chirho, 0)
            .ok_or(FundepFailureChirho::bounded_chirho("FD eligibility"))?;
        let Some(class_chirho) = self
            .context_chirho
            .class_env_chirho
            .class_record_by_identity_chirho(identity_chirho)
        else {
            return Ok(false);
        };
        charge_chirho(
            &mut self.budget_chirho,
            self.context_chirho.given_preds_chirho.len(),
            0,
        )
        .ok_or(FundepFailureChirho::bounded_chirho("FD eligibility"))?;
        if self
            .context_chirho
            .given_preds_chirho
            .iter()
            .any(|given_chirho| given_chirho.class_identity_chirho() == proof_chirho)
        {
            return Ok(true);
        }
        let Some(instances_chirho) = self
            .context_chirho
            .class_env_chirho
            .instances_chirho()
            .get_chirho(&class_chirho.name_chirho)
        else {
            return Ok(false);
        };
        charge_chirho(&mut self.budget_chirho, instances_chirho.len_chirho(), 0)
            .ok_or(FundepFailureChirho::bounded_chirho("FD eligibility"))?;
        Ok(instances_chirho.iter_chirho().any(|instance_chirho| {
            instance_chirho.class_identity_chirho() == proof_chirho
                && instance_chirho
                    .checked_binders_chirho()
                    .is_some_and(|(owner_chirho, _)| owner_chirho == self.owner_chirho)
        }))
    }

    fn normalize_chirho(
        &mut self,
        term_chirho: &TyChirho,
        candidate_chirho: &FundepCandidateChirho,
    ) -> FundepResultChirho<TyChirho> {
        let before_chirho = self.budget_chirho.remaining_chirho();
        let resolved_chirho = candidate_chirho.resolve_chirho(term_chirho, &mut self.budget_chirho);
        self.record_normalization_chirho(
            FundepNormalizationStageChirho::FirstResolveChirho,
            before_chirho,
        );
        let resolved_chirho = resolved_chirho
            .map_err(|_| FundepFailureChirho::bounded_chirho("FD normalization FirstR"))?;
        let before_chirho = self.budget_chirho.remaining_chirho();
        let term_chirho = self
            .context_chirho
            .normalize_signature_public_image_chirho(&resolved_chirho, &mut self.budget_chirho);
        self.record_normalization_chirho(
            FundepNormalizationStageChirho::PublicNormalizeChirho,
            before_chirho,
        );
        let term_chirho = term_chirho
            .map_err(|_| FundepFailureChirho::bounded_chirho("FD normalization PublicN"))?;
        // Workflow: language-features-chirho/fundep-generalization-chirho.
        // Public normalization can expose a candidate-bound meta, so only an
        // exact, charged contract comparison proves the second resolve redundant.
        let before_chirho = self.budget_chirho.remaining_chirho();
        let comparison_chirho = crate::ty_chirho::same_type_contract_chirho(
            &resolved_chirho,
            &term_chirho,
            &mut |work_chirho| charge_chirho(&mut self.budget_chirho, work_chirho, 0),
        );
        self.record_normalization_chirho(
            FundepNormalizationStageChirho::ContractCompareChirho,
            before_chirho,
        );
        match comparison_chirho {
            Some(true) => Ok(term_chirho),
            Some(false) => {
                let before_chirho = self.budget_chirho.remaining_chirho();
                let result_chirho =
                    candidate_chirho.resolve_chirho(&term_chirho, &mut self.budget_chirho);
                self.record_normalization_chirho(
                    FundepNormalizationStageChirho::FinalResolveChirho,
                    before_chirho,
                );
                result_chirho
                    .map_err(|_| FundepFailureChirho::bounded_chirho("FD normalization FinalR"))
            }
            None => Err(FundepFailureChirho::bounded_chirho(
                "FD normalization ContractCompare",
            )),
        }
    }

    fn predicate_chirho(
        &mut self,
        predicate_chirho: &PredChirho,
        candidate_chirho: &FundepCandidateChirho,
    ) -> FundepResultChirho<PredChirho> {
        charge_chirho(
            &mut self.budget_chirho,
            predicate_chirho
                .class_name_chirho
                .len()
                .checked_add(1)
                .ok_or(FundepFailureChirho::bounded_chirho(
                    "FD predicate normalization",
                ))?,
            1,
        )
        .ok_or(FundepFailureChirho::bounded_chirho(
            "FD predicate normalization",
        ))?;
        let kinds_chirho = predicate_chirho
            .kind_tys_chirho
            .iter()
            .map(|term_chirho| self.normalize_chirho(term_chirho, candidate_chirho))
            .collect::<Result<_, _>>()?;
        let primary_chirho =
            self.normalize_chirho(&predicate_chirho.ty_chirho, candidate_chirho)?;
        let extras_chirho = predicate_chirho
            .extra_tys_chirho
            .iter()
            .map(|term_chirho| self.normalize_chirho(term_chirho, candidate_chirho))
            .collect::<Result<_, _>>()?;
        Ok(PredChirho::from_proof_chirho(
            predicate_chirho.class_identity_chirho().clone(),
            crate::class_chirho::PredPartsChirho {
                class_name_chirho: predicate_chirho.class_name_chirho.clone(),
                kind_tys_chirho: kinds_chirho,
                ty_chirho: primary_chirho,
                extra_tys_chirho: extras_chirho,
            },
        ))
    }

    fn may_bind_chirho(
        &self,
        variable_chirho: TyVarChirho,
        candidate_chirho: &FundepCandidateChirho,
        live_chirho: bool,
    ) -> bool {
        candidate_chirho
            .records_chirho
            .contains_key(&variable_chirho)
            || (live_chirho
                && variable_chirho.0 < self.base_next_chirho
                && self
                    .context_chirho
                    .elaboration_state_chirho
                    .variable_binding_chirho(variable_chirho)
                    == ElaborationVariableBindingChirho::MetaChirho)
    }

    fn kind_chirho(
        &mut self,
        term_chirho: &TyChirho,
        candidate_chirho: &FundepCandidateChirho,
        equations_chirho: &mut Vec<(TyChirho, TyChirho)>,
    ) -> FundepResultChirho<Option<TyChirho>> {
        let records_chirho = &candidate_chirho.records_chirho;
        let live_records_chirho = &self
            .context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho;
        let lookup_chirho = |variable_chirho| {
            records_chirho
                .get(&variable_chirho)
                .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
                .or_else(|| {
                    live_records_chirho
                        .get(&variable_chirho)
                        .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
                })
        };
        check_monomorphic_kind_chirho(
            &self.context_chirho.type_families_chirho,
            self.owner_chirho,
            term_chirho,
            &candidate_chirho.substitution_chirho,
            &lookup_chirho,
            equations_chirho,
            &mut |work_chirho, output_chirho| {
                charge_chirho(&mut self.budget_chirho, work_chirho, output_chirho)
            },
        )
        .map_err(|_| FundepFailureChirho::bounded_chirho("FD checked kind"))
    }

    fn raw_equation_chirho(
        &mut self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        candidate_chirho: &FundepCandidateChirho,
        live_chirho: bool,
    ) -> FundepResultChirho<Option<SubstChirho>> {
        let records_chirho = &candidate_chirho.records_chirho;
        let state_chirho = &self.context_chirho.elaboration_state_chirho;
        let base_next_chirho = self.base_next_chirho;
        let allowed_chirho = |variable_chirho: TyVarChirho| {
            records_chirho.contains_key(&variable_chirho)
                || (live_chirho
                    && variable_chirho.0 < base_next_chirho
                    && state_chirho.variable_binding_chirho(variable_chirho)
                        == ElaborationVariableBindingChirho::MetaChirho)
        };
        unify_monomorphic_bounded_chirho(
            left_chirho,
            right_chirho,
            &allowed_chirho,
            &mut |work_chirho, output_chirho| {
                charge_chirho(&mut self.budget_chirho, work_chirho, output_chirho)
            },
        )
        .map_err(|_| FundepFailureChirho::bounded_chirho("FD monotype equation"))
    }

    fn equation_chirho(
        &mut self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        candidate_chirho: &mut FundepCandidateChirho,
        live_chirho: bool,
    ) -> FundepResultChirho<bool> {
        let left_chirho = self.normalize_chirho(left_chirho, candidate_chirho)?;
        let right_chirho = self.normalize_chirho(right_chirho, candidate_chirho)?;
        // Validate both complete operands before decomposition. In particular,
        // a stuck family has no checked monotype application to decompose.
        let mut pending_chirho = Vec::new();
        let Some(left_kind_chirho) =
            self.kind_chirho(&left_chirho, candidate_chirho, &mut pending_chirho)?
        else {
            return Ok(false);
        };
        let Some(right_kind_chirho) =
            self.kind_chirho(&right_chirho, candidate_chirho, &mut pending_chirho)?
        else {
            return Ok(false);
        };
        charge_chirho(&mut self.budget_chirho, 2, 2)
            .ok_or(FundepFailureChirho::bounded_chirho("FD equation"))?;
        pending_chirho.push((left_kind_chirho, right_kind_chirho));
        let mut pending_chirho: Vec<_> = pending_chirho
            .into_iter()
            .map(|(left_chirho, right_chirho)| (left_chirho, right_chirho, true))
            .collect();
        pending_chirho.push((left_chirho, right_chirho, live_chirho));
        while let Some((left_chirho, right_chirho, equation_live_chirho)) = pending_chirho.pop() {
            let left_chirho = self.normalize_chirho(&left_chirho, candidate_chirho)?;
            let right_chirho = self.normalize_chirho(&right_chirho, candidate_chirho)?;
            let Some(part_chirho) = self.raw_equation_chirho(
                &left_chirho,
                &right_chirho,
                candidate_chirho,
                equation_live_chirho,
            )?
            else {
                return Ok(false);
            };
            for (variable_chirho, image_chirho) in part_chirho.iter_chirho() {
                if !self.may_bind_chirho(*variable_chirho, candidate_chirho, equation_live_chirho) {
                    return Ok(false);
                }
                let image_chirho = candidate_chirho
                    .resolve_chirho(image_chirho, &mut self.budget_chirho)
                    .map_err(|_| FundepFailureChirho::bounded_chirho("FD candidate resolution"))?;
                if candidate_chirho
                    .substitution_chirho
                    .lookup_chirho(variable_chirho)
                    .is_some()
                {
                    charge_chirho(&mut self.budget_chirho, 1, 1)
                        .ok_or(FundepFailureChirho::bounded_chirho("FD equation"))?;
                    pending_chirho.push((
                        TyChirho::VarChirho(*variable_chirho),
                        image_chirho,
                        equation_live_chirho,
                    ));
                    continue;
                }
                let expected_chirho = candidate_chirho
                    .records_chirho
                    .get(variable_chirho)
                    .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
                    .or_else(|| {
                        self.context_chirho
                            .elaboration_state_chirho
                            .variable_kinds_chirho
                            .get(variable_chirho)
                            .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
                    });
                let Some(expected_chirho) = expected_chirho else {
                    return Ok(false);
                };
                let expected_chirho = candidate_chirho
                    .resolve_chirho(expected_chirho, &mut self.budget_chirho)
                    .map_err(|_| FundepFailureChirho::bounded_chirho("FD candidate resolution"))?;
                let mut kind_equations_chirho = Vec::new();
                let Some(actual_chirho) =
                    self.kind_chirho(&image_chirho, candidate_chirho, &mut kind_equations_chirho)?
                else {
                    return Ok(false);
                };
                charge_chirho(
                    &mut self.budget_chirho,
                    kind_equations_chirho
                        .len()
                        .checked_add(2)
                        .ok_or(FundepFailureChirho::bounded_chirho("FD equation"))?,
                    kind_equations_chirho
                        .len()
                        .checked_add(2)
                        .ok_or(FundepFailureChirho::bounded_chirho("FD equation"))?,
                )
                .ok_or(FundepFailureChirho::bounded_chirho("FD equation"))?;
                pending_chirho.extend(
                    kind_equations_chirho
                        .into_iter()
                        .map(|(left_chirho, right_chirho)| (left_chirho, right_chirho, true)),
                );
                pending_chirho.push((expected_chirho, actual_chirho, true));
                candidate_chirho
                    .substitution_chirho
                    .insert_chirho(*variable_chirho, image_chirho);
            }
        }
        Ok(true)
    }

    fn open_chirho(
        &mut self,
        instance_chirho: &InstDeclChirho,
        candidate_chirho: &mut FundepCandidateChirho,
    ) -> FundepResultChirho<Option<InstDeclChirho>> {
        // P2 borrows the same owner's allowance. Settle our current loan first,
        // then re-lend what remains; two independent budgets never overlap.
        self.context_chirho
            .elaboration_state_chirho
            .settle_equality_budget_chirho(
                self.loan_chirho,
                &self.budget_chirho,
                false,
                self.span_chirho,
            )
            .map_err(|_| FundepFailureChirho::bounded_chirho("FD instance opening"))?;
        self.loan_chirho = (0, 0);
        self.budget_chirho = EqualityBudgetChirho::limited_chirho(0, 0);
        let opened_chirho = self.context_chirho.stage_instance_opening_chirho(
            instance_chirho,
            &mut candidate_chirho.next_variable_chirho,
            self.span_chirho,
        );
        self.budget_chirho = self
            .context_chirho
            .elaboration_state_chirho
            .lend_equality_budget_chirho(self.span_chirho)
            .map_err(|_| FundepFailureChirho::bounded_chirho("FD instance opening"))?;
        self.loan_chirho = self.budget_chirho.remaining_chirho();
        let Some((opened_chirho, _prefix_chirho)) = opened_chirho
            .map_err(|_| FundepFailureChirho::bounded_chirho("FD instance opening"))?
        else {
            return Ok(None);
        };
        let Some((owner_chirho, variables_chirho)) = opened_chirho.checked_binders_chirho() else {
            return Ok(None);
        };
        if owner_chirho != self.owner_chirho {
            return Ok(None);
        }
        for record_chirho in variables_chirho.records_chirho() {
            charge_chirho(&mut self.budget_chirho, 1, 1)
                .ok_or(FundepFailureChirho::bounded_chirho("FD instance opening"))?;
            if candidate_chirho
                .records_chirho
                .insert(record_chirho.variable_chirho, record_chirho.clone())
                .is_some()
            {
                return Ok(None);
            }
        }
        Ok(Some(opened_chirho))
    }

    fn matches_chirho(
        &mut self,
        wanted_chirho: &PredChirho,
        instance_chirho: &InstDeclChirho,
        candidate_chirho: &mut FundepCandidateChirho,
    ) -> FundepResultChirho<bool> {
        let Some(groups_chirho) = self
            .context_chirho
            .class_env_chirho
            .fundep_equations_chirho(
                wanted_chirho,
                instance_chirho,
                &mut |work_chirho, output_chirho| {
                    charge_chirho(&mut self.budget_chirho, work_chirho, output_chirho)
                },
            )?
        else {
            return Ok(false);
        };
        charge_chirho(
            &mut self.budget_chirho,
            groups_chirho.len(),
            groups_chirho.len(),
        )
        .ok_or(FundepFailureChirho::bounded_chirho(
            "FD determinant matching",
        ))?;
        let mut matched_chirho = false;
        let mut used_chirho = HashSet::new();
        // One FD may make another determinant known. Retry only unapplied
        // groups; every successful group contributes to the SAME candidate.
        for _round_chirho in 0..=groups_chirho.len() {
            let before_chirho = used_chirho.len();
            for (index_chirho, group_chirho) in groups_chirho.iter().enumerate() {
                if used_chirho.contains(&index_chirho) {
                    continue;
                }
                let mut trial_chirho = candidate_chirho
                    .copy_chirho(&mut self.budget_chirho)
                    .map_err(|_| FundepFailureChirho::bounded_chirho("FD candidate copy"))?;
                let mut determinant_chirho = true;
                for (pattern_chirho, target_chirho) in &group_chirho.hidden_kinds_chirho {
                    if !self.equation_chirho(
                        pattern_chirho,
                        target_chirho,
                        &mut trial_chirho,
                        true,
                    )? {
                        determinant_chirho = false;
                        break;
                    }
                }
                if !determinant_chirho {
                    continue;
                }
                for (pattern_chirho, target_chirho) in &group_chirho.determinants_chirho {
                    if !self.equation_chirho(
                        pattern_chirho,
                        target_chirho,
                        &mut trial_chirho,
                        false,
                    )? {
                        determinant_chirho = false;
                        break;
                    }
                }
                if !determinant_chirho {
                    continue;
                }
                for (pattern_chirho, target_chirho) in &group_chirho.consequences_chirho {
                    if !self.equation_chirho(
                        pattern_chirho,
                        target_chirho,
                        &mut trial_chirho,
                        true,
                    )? {
                        return Ok(false);
                    }
                }
                *candidate_chirho = trial_chirho;
                used_chirho.insert(index_chirho);
                matched_chirho = true;
            }
            if used_chirho.len() == before_chirho {
                break;
            }
        }
        if !matched_chirho {
            return Ok(false);
        }
        for (pattern_chirho, target_chirho) in
            std::iter::once((&instance_chirho.head_ty_chirho, &wanted_chirho.ty_chirho)).chain(
                instance_chirho
                    .extra_head_tys_chirho
                    .iter()
                    .zip(&wanted_chirho.extra_tys_chirho),
            )
        {
            if !self.equation_chirho(pattern_chirho, target_chirho, candidate_chirho, false)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn close_chirho(
        &mut self,
        wanted_chirho: &PredChirho,
        original_chirho: &FundepCandidateChirho,
        active_chirho: &mut Vec<PredChirho>,
        depth_chirho: usize,
    ) -> FundepResultChirho<Option<FundepCandidateChirho>> {
        charge_chirho(&mut self.budget_chirho, 1, 0).ok_or(FundepFailureChirho::BoundedChirho(
            Some("FD context closure"),
        ))?;
        if depth_chirho > FUNDEP_DEPTH_LIMIT_CHIRHO {
            return Err(FundepFailureChirho::DepthChirho {
                actual_chirho: depth_chirho,
                limit_chirho: FUNDEP_DEPTH_LIMIT_CHIRHO,
            });
        }
        if !self.eligible_chirho(wanted_chirho.class_identity_chirho())? {
            return Ok(None);
        }
        let wanted_chirho = self.predicate_chirho(wanted_chirho, original_chirho)?;
        let PredicateClassProofChirho::ProvenChirho(identity_chirho) =
            wanted_chirho.class_identity_chirho()
        else {
            return Ok(None);
        };
        let Some(class_chirho) = self
            .context_chirho
            .class_env_chirho
            .class_record_by_identity_chirho(identity_chirho)
            .cloned()
        else {
            return Ok(None);
        };
        for given_index_chirho in 0..self.context_chirho.given_preds_chirho.len() {
            let given_chirho = copy_predicate_chirho(
                &self.context_chirho.given_preds_chirho[given_index_chirho],
                &mut self.budget_chirho,
            )
            .map_err(|_| FundepFailureChirho::bounded_chirho("FD predicate copy or comparison"))?;
            let given_chirho = self.predicate_chirho(&given_chirho, original_chirho)?;
            if same_predicate_chirho(&given_chirho, &wanted_chirho, &mut self.budget_chirho)
                .map_err(|_| {
                    FundepFailureChirho::bounded_chirho("FD predicate copy or comparison")
                })?
            {
                return original_chirho
                    .copy_chirho(&mut self.budget_chirho)
                    .map_err(|_| FundepFailureChirho::bounded_chirho("FD candidate copy"))
                    .map(Some);
            }
        }
        for ancestor_chirho in active_chirho.iter() {
            if same_predicate_chirho(ancestor_chirho, &wanted_chirho, &mut self.budget_chirho)
                .map_err(|_| {
                    FundepFailureChirho::bounded_chirho("FD predicate copy or comparison")
                })?
            {
                return Ok(None);
            }
        }
        let Some(instances_chirho) = self
            .context_chirho
            .class_env_chirho
            .instances_chirho()
            .get_chirho(&class_chirho.name_chirho)
        else {
            return Ok(None);
        };
        charge_chirho(
            &mut self.budget_chirho,
            instances_chirho.len_chirho(),
            instances_chirho.len_chirho(),
        )
        .ok_or(FundepFailureChirho::bounded_chirho("FD context closure"))?;
        let instances_chirho: Vec<_> = instances_chirho.iter_chirho().map(Arc::clone).collect();
        let mut selected_chirho = None;
        for instance_chirho in instances_chirho {
            if instance_chirho.class_identity_chirho() != wanted_chirho.class_identity_chirho() {
                continue;
            }
            let mut candidate_chirho = original_chirho
                .copy_chirho(&mut self.budget_chirho)
                .map_err(|_| FundepFailureChirho::bounded_chirho("FD candidate copy"))?;
            let Some(opened_chirho) = self.open_chirho(&instance_chirho, &mut candidate_chirho)?
            else {
                continue;
            };
            if !self.matches_chirho(&wanted_chirho, &opened_chirho, &mut candidate_chirho)? {
                continue;
            }
            if selected_chirho.is_some() {
                return Ok(None);
            }
            selected_chirho = Some((opened_chirho, candidate_chirho));
        }
        let Some((opened_chirho, mut candidate_chirho)) = selected_chirho else {
            return Ok(None);
        };
        charge_chirho(&mut self.budget_chirho, 1, 1).ok_or(FundepFailureChirho::BoundedChirho(
            Some("FD context closure"),
        ))?;
        active_chirho.push(wanted_chirho);
        for child_chirho in &opened_chirho.context_chirho {
            let Some(next_chirho) = self.close_chirho(
                child_chirho,
                &candidate_chirho,
                active_chirho,
                depth_chirho + 1,
            )?
            else {
                active_chirho.pop();
                return Ok(None);
            };
            candidate_chirho = next_chirho;
        }
        active_chirho.pop();
        Ok(Some(candidate_chirho))
    }

    fn export_chirho(
        &mut self,
        candidate_chirho: &FundepCandidateChirho,
    ) -> FundepResultChirho<Option<SubstChirho>> {
        let mut result_chirho = SubstChirho::empty_chirho();
        for (variable_chirho, image_chirho) in candidate_chirho.substitution_chirho.iter_chirho() {
            if variable_chirho.0 >= self.base_next_chirho {
                continue;
            }
            if !self.may_bind_chirho(*variable_chirho, candidate_chirho, true) {
                return Ok(None);
            }
            let image_chirho = self.normalize_chirho(image_chirho, candidate_chirho)?;
            let variables_chirho = SubstChirho::free_variables_bounded_chirho(
                &image_chirho,
                &mut |work_chirho, output_chirho| {
                    charge_chirho(&mut self.budget_chirho, work_chirho, output_chirho)
                },
            )
            .ok_or(FundepFailureChirho::bounded_chirho("FD export"))?;
            if variables_chirho
                .iter()
                .any(|variable_chirho| variable_chirho.0 >= self.base_next_chirho)
            {
                return Ok(None);
            }
            let Some(kind_chirho) = self
                .context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho
                .get(variable_chirho)
                .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
            else {
                return Ok(None);
            };
            let kind_chirho = candidate_chirho
                .resolve_chirho(kind_chirho, &mut self.budget_chirho)
                .map_err(|_| FundepFailureChirho::bounded_chirho("FD candidate resolution"))?;
            let variables_chirho = SubstChirho::free_variables_bounded_chirho(
                &kind_chirho,
                &mut |work_chirho, output_chirho| {
                    charge_chirho(&mut self.budget_chirho, work_chirho, output_chirho)
                },
            )
            .ok_or(FundepFailureChirho::bounded_chirho("FD export"))?;
            if variables_chirho
                .iter()
                .any(|variable_chirho| variable_chirho.0 >= self.base_next_chirho)
            {
                return Ok(None);
            }
            if image_chirho != TyChirho::VarChirho(*variable_chirho) {
                result_chirho.insert_chirho(*variable_chirho, image_chirho);
            }
        }
        Ok(Some(result_chirho))
    }
}

impl InferCtxChirho {
    /// One solve shared by module-end Rule 2 and instance-method/legacy checking.
    /// Successful trees export only old checked Meta images: every local opening
    /// ID must zonk out, so no speculative supply or binder metadata is published.
    /// workflow: language-features-chirho/fundep-generalization-chirho
    pub(in crate::infer_chirho) fn solve_deferred_fundeps_chirho(
        &mut self,
        final_subst_chirho: &SubstChirho,
    ) -> SubstChirho {
        let Some(owner_chirho) = self.elaboration_state_chirho.owner_chirho else {
            return SubstChirho::empty_chirho();
        };
        let span_chirho = self
            .deferred_preds_chirho
            .first()
            .map(|(_, span_chirho)| *span_chirho)
            .unwrap_or(SpanChirho::DUMMY_CHIRHO);
        let budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.non_deferrable_diagnostics_chirho
                    .push_chirho(error_chirho.into_diagnostic_chirho());
                return SubstChirho::empty_chirho();
            }
        };
        let loan_chirho = budget_chirho.remaining_chirho();
        let base_next_chirho = self.elaboration_state_chirho.next_variable_chirho;
        let mut solver_chirho = FundepSolveChirho {
            context_chirho: self,
            owner_chirho,
            base_next_chirho,
            budget_chirho,
            loan_chirho,
            span_chirho,
            normalization_metrics_chirho: [FundepNormalizationMetricChirho::default(); 4],
        };
        let result_chirho = (|| -> FundepResultChirho<Option<(SubstChirho, HashSet<usize>)>> {
            let mut wanted_chirho = Vec::new();
            for index_chirho in 0..solver_chirho.context_chirho.deferred_preds_chirho.len() {
                let proof_chirho = solver_chirho.context_chirho.deferred_preds_chirho[index_chirho]
                    .0
                    .class_identity_chirho()
                    .clone();
                if !matches!(proof_chirho, PredicateClassProofChirho::ProvenChirho(_)) {
                    continue;
                }
                if !solver_chirho.eligible_chirho(&proof_chirho)? {
                    continue;
                }
                let predicate_chirho = copy_predicate_chirho(
                    &solver_chirho.context_chirho.deferred_preds_chirho[index_chirho].0,
                    &mut solver_chirho.budget_chirho,
                )?;
                let predicate_chirho = &predicate_chirho;
                let terms_chirho = predicate_chirho.terms_chirho().try_mapped_chirho(
                    &mut |term_chirho: &TyChirho| {
                        final_subst_chirho
                            .resolve_owned_bounded_chirho(
                                term_chirho,
                                &mut |work_chirho, output_chirho| {
                                    charge_chirho(
                                        &mut solver_chirho.budget_chirho,
                                        work_chirho,
                                        output_chirho,
                                    )
                                },
                            )
                            .ok_or(FundepFailureChirho::bounded_chirho("FD wanted staging"))
                    },
                )?;
                charge_chirho(
                    &mut solver_chirho.budget_chirho,
                    predicate_chirho
                        .class_name_chirho
                        .len()
                        .checked_add(1)
                        .ok_or(FundepFailureChirho::bounded_chirho("FD wanted staging"))?,
                    1,
                )
                .ok_or(FundepFailureChirho::bounded_chirho("FD wanted staging"))?;
                wanted_chirho.push((
                    index_chirho,
                    PredChirho::from_proof_chirho(
                        predicate_chirho.class_identity_chirho().clone(),
                        crate::class_chirho::PredPartsChirho {
                            class_name_chirho: predicate_chirho.class_name_chirho.clone(),
                            kind_tys_chirho: terms_chirho.kind_tys_chirho,
                            ty_chirho: terms_chirho.ty_chirho,
                            extra_tys_chirho: terms_chirho.extra_tys_chirho,
                        },
                    ),
                ));
            }
            let mut candidate_chirho = FundepCandidateChirho::new_chirho(base_next_chirho);
            let mut closed_chirho = HashSet::new();
            for _round_chirho in 0..=wanted_chirho.len() {
                let before_chirho = closed_chirho.len();
                for (index_chirho, predicate_chirho) in &wanted_chirho {
                    if closed_chirho.contains(index_chirho) {
                        continue;
                    }
                    if let Some(next_chirho) = solver_chirho.close_chirho(
                        predicate_chirho,
                        &candidate_chirho,
                        &mut Vec::new(),
                        0,
                    )? {
                        if solver_chirho.export_chirho(&next_chirho)?.is_some() {
                            candidate_chirho = next_chirho;
                            closed_chirho.insert(*index_chirho);
                        }
                    }
                }
                if closed_chirho.len() == before_chirho {
                    break;
                }
            }
            Ok(solver_chirho
                .export_chirho(&candidate_chirho)?
                .map(|substitution_chirho| (substitution_chirho, closed_chirho)))
        })();
        let failure_detail_chirho = result_chirho.as_ref().err().map(|failure_chirho| {
            format!(
                "{}; {}",
                failure_chirho.detail_chirho(),
                solver_chirho.normalization_detail_chirho(),
            )
        });
        let settled_chirho = solver_chirho
            .context_chirho
            .elaboration_state_chirho
            .settle_named_equality_budget_chirho(
                solver_chirho.loan_chirho,
                &solver_chirho.budget_chirho,
                result_chirho.is_err(),
                failure_detail_chirho.as_deref(),
                span_chirho,
            );
        if let Err(error_chirho) = settled_chirho {
            solver_chirho
                .context_chirho
                .non_deferrable_diagnostics_chirho
                .push_chirho(error_chirho.into_diagnostic_chirho());
            return SubstChirho::empty_chirho();
        }
        let Ok(Some((mut substitution_chirho, closed_chirho))) = result_chirho else {
            return SubstChirho::empty_chirho();
        };
        if solver_chirho
            .context_chirho
            .elaboration_state_chirho
            .owner_chirho
            != Some(owner_chirho)
            || solver_chirho
                .context_chirho
                .elaboration_state_chirho
                .next_variable_chirho
                != base_next_chirho
        {
            return SubstChirho::empty_chirho();
        }
        if !substitution_chirho.is_empty_chirho() {
            if solver_chirho
                .context_chirho
                .apply_subst_all_chirho(&mut substitution_chirho)
                .is_err()
            {
                return SubstChirho::empty_chirho();
            }
        }
        let mut index_chirho = 0usize;
        solver_chirho
            .context_chirho
            .deferred_preds_chirho
            .retain(|_| {
                let keep_chirho = !closed_chirho.contains(&index_chirho);
                index_chirho += 1;
                keep_chirho
            });
        substitution_chirho
    }
}
