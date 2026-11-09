// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Local candidate data, never a clone of the live inference context.

use super::super::equality_normalization_chirho::EqualityBudgetChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{QuantifiedVariableChirho, TyChirho, TyVarChirho};
use std::collections::HashMap;

pub(super) fn charge_chirho(
    budget_chirho: &mut EqualityBudgetChirho,
    work_chirho: usize,
    output_chirho: usize,
) -> Option<()> {
    budget_chirho.node_chirho(output_chirho)?;
    budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
}

pub(super) struct FundepCandidateChirho {
    pub next_variable_chirho: u32,
    pub substitution_chirho: SubstChirho,
    /// Only P2's actual fresh-image producer supplies these records.
    pub records_chirho: HashMap<TyVarChirho, QuantifiedVariableChirho>,
}

impl FundepCandidateChirho {
    pub fn new_chirho(next_variable_chirho: u32) -> Self {
        Self {
            next_variable_chirho,
            substitution_chirho: SubstChirho::empty_chirho(),
            records_chirho: HashMap::new(),
        }
    }

    pub fn copy_chirho(&self, budget_chirho: &mut EqualityBudgetChirho) -> Result<Self, ()> {
        charge_chirho(
            budget_chirho,
            self.records_chirho.len(),
            self.records_chirho.len(),
        )
        .ok_or(())?;
        let mut substitution_chirho = SubstChirho::empty_chirho();
        for (variable_chirho, image_chirho) in self.substitution_chirho.iter_chirho() {
            let image_chirho = SubstChirho::empty_chirho()
                .apply_bounded_chirho(image_chirho, &mut |work_chirho, output_chirho| {
                    charge_chirho(budget_chirho, work_chirho, output_chirho)
                })
                .ok_or(())?;
            substitution_chirho.insert_chirho(*variable_chirho, image_chirho);
        }
        Ok(Self {
            next_variable_chirho: self.next_variable_chirho,
            substitution_chirho,
            records_chirho: self.records_chirho.clone(),
        })
    }

    pub fn resolve_chirho(
        &self,
        term_chirho: &TyChirho,
        budget_chirho: &mut EqualityBudgetChirho,
    ) -> Result<TyChirho, ()> {
        self.substitution_chirho
            .resolve_owned_bounded_chirho(term_chirho, &mut |work_chirho, output_chirho| {
                charge_chirho(budget_chirho, work_chirho, output_chirho)
            })
            .ok_or(())
    }
}

pub(super) fn copy_predicate_chirho(
    predicate_chirho: &crate::class_chirho::PredChirho,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Result<crate::class_chirho::PredChirho, ()> {
    charge_chirho(
        budget_chirho,
        predicate_chirho
            .class_name_chirho
            .len()
            .checked_add(1)
            .ok_or(())?,
        1,
    )
    .ok_or(())?;
    let terms_chirho = predicate_chirho
        .terms_chirho()
        .try_mapped_chirho(&mut |term_chirho| {
            SubstChirho::empty_chirho()
                .apply_bounded_chirho(term_chirho, &mut |work_chirho, output_chirho| {
                    charge_chirho(budget_chirho, work_chirho, output_chirho)
                })
                .ok_or(())
        })?;
    Ok(crate::class_chirho::PredChirho::from_proof_chirho(
        predicate_chirho.class_identity_chirho().clone(),
        crate::class_chirho::PredPartsChirho {
            class_name_chirho: predicate_chirho.class_name_chirho.clone(),
            kind_tys_chirho: terms_chirho.kind_tys_chirho,
            ty_chirho: terms_chirho.ty_chirho,
            extra_tys_chirho: terms_chirho.extra_tys_chirho,
        },
    ))
}

pub(super) fn same_predicate_chirho(
    left_chirho: &crate::class_chirho::PredChirho,
    right_chirho: &crate::class_chirho::PredChirho,
    budget_chirho: &mut EqualityBudgetChirho,
) -> Result<bool, ()> {
    charge_chirho(
        budget_chirho,
        left_chirho
            .class_name_chirho
            .len()
            .checked_add(right_chirho.class_name_chirho.len())
            .ok_or(())?,
        0,
    )
    .ok_or(())?;
    if left_chirho.class_identity_chirho() != right_chirho.class_identity_chirho()
        || left_chirho.class_name_chirho != right_chirho.class_name_chirho
        || left_chirho.kind_tys_chirho.len() != right_chirho.kind_tys_chirho.len()
        || left_chirho.extra_tys_chirho.len() != right_chirho.extra_tys_chirho.len()
    {
        return Ok(false);
    }
    for (left_chirho, right_chirho) in left_chirho
        .terms_chirho()
        .iter_chirho()
        .zip(right_chirho.terms_chirho().iter_chirho())
    {
        if !crate::ty_chirho::same_type_contract_chirho(
            left_chirho,
            right_chirho,
            &mut |work_chirho| charge_chirho(budget_chirho, work_chirho, 0),
        )
        .ok_or(())?
        {
            return Ok(false);
        }
    }
    Ok(true)
}
