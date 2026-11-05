// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared semantic argument traversal. Hidden kind arguments precede ordinary
//! arguments here, but never change functional-dependency or runtime indices.

use super::TyChirho;

#[derive(Clone, Copy)]
pub struct PredicateTermsChirho<'predicate_chirho> {
    pub(super) kind_tys_chirho: &'predicate_chirho [TyChirho],
    pub(super) ty_chirho: &'predicate_chirho TyChirho,
    pub(super) extra_tys_chirho: &'predicate_chirho [TyChirho],
}

pub struct MappedPredicateTermsChirho {
    pub kind_tys_chirho: Vec<TyChirho>,
    pub ty_chirho: TyChirho,
    pub extra_tys_chirho: Vec<TyChirho>,
}

impl<'predicate_chirho> PredicateTermsChirho<'predicate_chirho> {
    pub fn new_chirho(
        kind_tys_chirho: &'predicate_chirho [TyChirho],
        ty_chirho: &'predicate_chirho TyChirho,
        extra_tys_chirho: &'predicate_chirho [TyChirho],
    ) -> Self {
        Self {
            kind_tys_chirho,
            ty_chirho,
            extra_tys_chirho,
        }
    }

    pub fn iter_chirho(self) -> impl Iterator<Item = &'predicate_chirho TyChirho> {
        self.kind_tys_chirho
            .iter()
            .chain(std::iter::once(self.ty_chirho))
            .chain(self.extra_tys_chirho)
    }

    pub fn mapped_chirho(
        self,
        mut map_chirho: impl FnMut(&TyChirho) -> TyChirho,
    ) -> MappedPredicateTermsChirho {
        MappedPredicateTermsChirho {
            kind_tys_chirho: self.kind_tys_chirho.iter().map(&mut map_chirho).collect(),
            ty_chirho: map_chirho(self.ty_chirho),
            extra_tys_chirho: self.extra_tys_chirho.iter().map(map_chirho).collect(),
        }
    }

    pub fn try_mapped_chirho<ErrorChirho>(
        self,
        mut map_chirho: impl FnMut(&TyChirho) -> Result<TyChirho, ErrorChirho>,
    ) -> Result<MappedPredicateTermsChirho, ErrorChirho> {
        Ok(MappedPredicateTermsChirho {
            kind_tys_chirho: self
                .kind_tys_chirho
                .iter()
                .map(&mut map_chirho)
                .collect::<Result<_, _>>()?,
            ty_chirho: map_chirho(self.ty_chirho)?,
            extra_tys_chirho: self
                .extra_tys_chirho
                .iter()
                .map(map_chirho)
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Both predicate representations use this one conflation contract.
pub trait PredicateRecordChirho {
    fn terms_chirho(&self) -> PredicateTermsChirho<'_>;
    fn class_name_chirho(&self) -> &str;
    fn class_identity_chirho(
        &self,
    ) -> &haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho;
    fn refuse_metadata_chirho(
        &mut self,
        refusal_chirho: haskelujah_ast_chirho::name_chirho::PredicateMetadataRefusalChirho,
    );
    fn meet_class_identity_chirho(
        &mut self,
        other_chirho: &haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho,
    );
}

pub use haskelujah_ast_chirho::name_chirho::PredicateMetadataRefusalChirho as PredicateSetErrorChirho;

/// First-owned-record order and the legacy key are conserved even after new
/// metadata reasoning stops. There is no ordinary-error or duplicate fallback.
pub struct OrderedPredicateSetChirho<PredicateChirho> {
    predicates_chirho: Vec<PredicateChirho>,
    metadata_refusal_chirho: Option<PredicateSetErrorChirho>,
}

impl<PredicateChirho: PredicateRecordChirho> OrderedPredicateSetChirho<PredicateChirho> {
    pub fn new_chirho(predicates_chirho: Vec<PredicateChirho>) -> Self {
        let mut remaining_chirho = 1usize << 25;
        Self::with_metadata_work_chirho(predicates_chirho, 1 << 25, |weight_chirho| {
            if remaining_chirho == 0 {
                return false;
            }
            match remaining_chirho.checked_sub(weight_chirho) {
                Some(next_chirho) => {
                    remaining_chirho = next_chirho;
                    true
                }
                None => false,
            }
        })
    }

    /// `charge_chirho(0)` queries whether the module proof allowance is still
    /// live; it must return false after terminal soft exhaustion.
    pub fn with_metadata_work_chirho(
        predicates_chirho: Vec<PredicateChirho>,
        work_limit_chirho: usize,
        mut charge_chirho: impl FnMut(usize) -> bool,
    ) -> Self {
        let mut buckets_chirho: std::collections::HashMap<u64, Vec<(usize, usize)>> =
            std::collections::HashMap::new();
        let mut plan_chirho = Vec::new();
        let mut refusal_chirho = None;
        let mut unique_count_chirho = 0usize;
        for (index_chirho, predicate_chirho) in predicates_chirho.iter().enumerate() {
            if refusal_chirho.is_none() && !charge_chirho(0) {
                refusal_chirho = Some(PredicateSetErrorChirho::WorkChirho {
                    limit_chirho: work_limit_chirho,
                });
            }
            if refusal_chirho.is_none() {
                refusal_chirho = match predicate_weight_chirho(predicate_chirho) {
                    Err(error_chirho) => Some(error_chirho),
                    Ok(weight_chirho) if !charge_chirho(weight_chirho) => {
                        Some(PredicateSetErrorChirho::WorkChirho {
                            limit_chirho: work_limit_chirho,
                        })
                    }
                    Ok(_) => None,
                };
            }
            // This is the pre-existing legacy dedup operation, using borrowed
            // iterative keys rather than cloning deep terms into a HashSet.
            let key_chirho =
                super::legacy_predicate_key_chirho::fingerprint_chirho(predicate_chirho);
            let bucket_chirho = buckets_chirho.entry(key_chirho).or_default();
            let mut existing_chirho = None;
            for &(previous_chirho, target_chirho) in bucket_chirho.iter() {
                if refusal_chirho.is_none() && !charge_chirho(1) {
                    refusal_chirho = Some(PredicateSetErrorChirho::WorkChirho {
                        limit_chirho: work_limit_chirho,
                    });
                }
                if super::legacy_predicate_key_chirho::equal_chirho(
                    &predicates_chirho[previous_chirho],
                    predicate_chirho,
                ) {
                    existing_chirho = Some(target_chirho);
                    break;
                }
            }
            let target_chirho = existing_chirho.unwrap_or(unique_count_chirho);
            if existing_chirho.is_none() {
                bucket_chirho.push((index_chirho, target_chirho));
                unique_count_chirho += 1;
            }
            plan_chirho.push(target_chirho);
        }
        let mut unique_chirho: Vec<PredicateChirho> = Vec::new();
        for (mut predicate_chirho, target_chirho) in predicates_chirho.into_iter().zip(plan_chirho)
        {
            if let Some(refusal_chirho) = refusal_chirho {
                predicate_chirho.refuse_metadata_chirho(refusal_chirho);
            }
            if target_chirho == unique_chirho.len() {
                unique_chirho.push(predicate_chirho);
            } else {
                unique_chirho[target_chirho]
                    .meet_class_identity_chirho(predicate_chirho.class_identity_chirho());
            }
        }
        Self {
            predicates_chirho: unique_chirho,
            metadata_refusal_chirho: refusal_chirho,
        }
    }

    pub fn metadata_refusal_chirho(&self) -> Option<PredicateSetErrorChirho> {
        self.metadata_refusal_chirho
    }
    pub fn into_vec_chirho(self) -> Vec<PredicateChirho> {
        self.predicates_chirho
    }
}

/// Preserve the method scheme's written constraints exactly. Only the first
/// legacy-equal enclosing predicate receives metadata from the incoming one.
/// The contains search is legacy work; new proof work is independently bounded.
pub fn append_enclosing_predicate_chirho<PredicateChirho: PredicateRecordChirho>(
    predicates_chirho: &mut Vec<PredicateChirho>,
    mut incoming_chirho: PredicateChirho,
    work_limit_chirho: usize,
    mut charge_chirho: impl FnMut(usize) -> bool,
) -> Option<PredicateSetErrorChirho> {
    let first_chirho = predicates_chirho.iter().position(|predicate_chirho| {
        super::legacy_predicate_key_chirho::equal_chirho(predicate_chirho, &incoming_chirho)
    });
    let mut refusal_chirho = if !charge_chirho(0) {
        Some(PredicateSetErrorChirho::WorkChirho {
            limit_chirho: work_limit_chirho,
        })
    } else {
        None
    };
    if refusal_chirho.is_none() {
        refusal_chirho = match predicate_weight_chirho(&incoming_chirho) {
            Err(error_chirho) => Some(error_chirho),
            Ok(weight_chirho) if !charge_chirho(weight_chirho) => {
                Some(PredicateSetErrorChirho::WorkChirho {
                    limit_chirho: work_limit_chirho,
                })
            }
            Ok(_) => None,
        };
    }
    if let Some(index_chirho) = first_chirho {
        if refusal_chirho.is_none() {
            refusal_chirho = match predicate_weight_chirho(&predicates_chirho[index_chirho]) {
                Err(error_chirho) => Some(error_chirho),
                Ok(weight_chirho) if !charge_chirho(weight_chirho) => {
                    Some(PredicateSetErrorChirho::WorkChirho {
                        limit_chirho: work_limit_chirho,
                    })
                }
                Ok(_) => None,
            };
        }
        if let Some(refusal_chirho) = refusal_chirho {
            predicates_chirho[index_chirho].refuse_metadata_chirho(refusal_chirho);
        } else {
            predicates_chirho[index_chirho]
                .meet_class_identity_chirho(incoming_chirho.class_identity_chirho());
        }
    } else {
        if let Some(refusal_chirho) = refusal_chirho {
            incoming_chirho.refuse_metadata_chirho(refusal_chirho);
        }
        predicates_chirho.push(incoming_chirho);
    }
    refusal_chirho
}

fn predicate_weight_chirho(
    predicate_chirho: &impl PredicateRecordChirho,
) -> Result<usize, PredicateSetErrorChirho> {
    use haskelujah_ast_chirho::name_chirho::{
        PredicateClassIdentityChirho, PredicateClassProofChirho,
    };
    let mut weight_chirho = predicate_chirho
        .class_name_chirho()
        .len()
        .checked_add(1)
        .ok_or(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 })?;
    if let PredicateClassProofChirho::ProvenChirho(PredicateClassIdentityChirho::SourceChirho {
        unit_chirho,
        module_chirho,
        name_chirho,
    }) = predicate_chirho.class_identity_chirho()
    {
        weight_chirho = weight_chirho
            .checked_add(unit_chirho.as_str_chirho().len())
            .and_then(|weight_chirho| weight_chirho.checked_add(module_chirho.len()))
            .and_then(|weight_chirho| weight_chirho.checked_add(name_chirho.len()))
            .ok_or(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 })?;
    }
    for term_chirho in predicate_chirho.terms_chirho().iter_chirho() {
        term_weight_chirho(term_chirho, 0, &mut weight_chirho)?;
    }
    if weight_chirho > 4096 {
        return Err(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 });
    }
    Ok(weight_chirho)
}

fn term_weight_chirho(
    term_chirho: &TyChirho,
    depth_chirho: usize,
    weight_chirho: &mut usize,
) -> Result<(), PredicateSetErrorChirho> {
    if depth_chirho >= 128 {
        return Err(PredicateSetErrorChirho::DepthChirho { limit_chirho: 128 });
    }
    if *weight_chirho >= 4096 {
        return Err(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 });
    }
    *weight_chirho = weight_chirho
        .checked_add(1)
        .ok_or(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 })?;
    match term_chirho {
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            for constraint_chirho in context_chirho {
                term_weight_chirho(constraint_chirho, depth_chirho + 1, weight_chirho)?;
            }
            term_weight_chirho(body_chirho, depth_chirho + 1, weight_chirho)?;
        }
        TyChirho::VarChirho(_) => {}
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            *weight_chirho = weight_chirho
                .checked_add(name_chirho.len())
                .ok_or(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 })?;
        }
        TyChirho::AppChirho(left_chirho, right_chirho)
        | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
            term_weight_chirho(left_chirho, depth_chirho + 1, weight_chirho)?;
            term_weight_chirho(right_chirho, depth_chirho + 1, weight_chirho)?;
        }
        TyChirho::TupleChirho(elements_chirho) => {
            for element_chirho in elements_chirho {
                term_weight_chirho(element_chirho, depth_chirho + 1, weight_chirho)?;
            }
        }
        TyChirho::ListChirho(element_chirho) => {
            term_weight_chirho(element_chirho, depth_chirho + 1, weight_chirho)?
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            *weight_chirho = weight_chirho
                .checked_add(vars_chirho.len())
                .ok_or(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 })?;
            term_weight_chirho(body_chirho, depth_chirho + 1, weight_chirho)?;
        }
    }
    if *weight_chirho > 4096 {
        return Err(PredicateSetErrorChirho::WeightChirho { limit_chirho: 4096 });
    }
    Ok(())
}

#[cfg(test)]
#[path = "predicate_terms_tests_chirho.rs"]
mod tests_chirho;
