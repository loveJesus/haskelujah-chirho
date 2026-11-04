// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared semantic argument traversal. Hidden kind arguments precede ordinary
//! arguments here, but never change functional-dependency or runtime indices.

use super::TyChirho;

#[derive(Clone, Copy)]
pub struct PredicateTermsChirho<'predicate_chirho> {
    kind_tys_chirho: &'predicate_chirho [TyChirho],
    ty_chirho: &'predicate_chirho TyChirho,
    extra_tys_chirho: &'predicate_chirho [TyChirho],
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
}

/// Both predicate representations use this one conflation contract.
pub trait PredicateRecordChirho {
    fn terms_chirho(&self) -> PredicateTermsChirho<'_>;
    fn class_name_chirho(&self) -> &str;
    fn class_identity_chirho(
        &self,
    ) -> &haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho;
    fn meet_class_identity_chirho(
        &mut self,
        other_chirho: &haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredicateSetErrorChirho {
    TermBoundChirho,
    WorkBoundChirho,
}

pub struct PredicateSetFailureChirho<PredicateChirho> {
    pub predicates_chirho: Vec<PredicateChirho>,
    pub error_chirho: PredicateSetErrorChirho,
}

/// First-occurrence order and the legacy key are conserved. Every duplicate's
/// proof meets the survivor *before* it is discarded. No predicate is copied.
pub struct OrderedPredicateSetChirho<PredicateChirho> {
    predicates_chirho: Vec<PredicateChirho>,
}

impl<PredicateChirho: PredicateRecordChirho + Eq + std::hash::Hash>
    OrderedPredicateSetChirho<PredicateChirho>
{
    pub fn try_new_chirho(
        predicates_chirho: Vec<PredicateChirho>,
    ) -> Result<Self, PredicateSetFailureChirho<PredicateChirho>> {
        let plan_chirho = match coalescence_plan_chirho(&predicates_chirho) {
            Ok(plan_chirho) => plan_chirho,
            Err(error_chirho) => {
                return Err(PredicateSetFailureChirho {
                    predicates_chirho,
                    error_chirho,
                });
            }
        };
        let mut unique_chirho: Vec<PredicateChirho> = Vec::new();
        for (predicate_chirho, target_chirho) in predicates_chirho.into_iter().zip(plan_chirho) {
            if target_chirho == unique_chirho.len() {
                unique_chirho.push(predicate_chirho);
            } else {
                unique_chirho[target_chirho]
                    .meet_class_identity_chirho(predicate_chirho.class_identity_chirho());
            }
        }
        Ok(Self {
            predicates_chirho: unique_chirho,
        })
    }

    pub fn into_vec_chirho(self) -> Vec<PredicateChirho> {
        self.predicates_chirho
    }
}

fn coalescence_plan_chirho<PredicateChirho: PredicateRecordChirho + Eq + std::hash::Hash>(
    predicates_chirho: &[PredicateChirho],
) -> Result<Vec<usize>, PredicateSetErrorChirho> {
    use std::hash::Hasher;
    let mut work_chirho = 1usize << 25;
    let mut buckets_chirho: std::collections::HashMap<u64, Vec<(usize, usize)>> =
        std::collections::HashMap::new();
    let mut plan_chirho = Vec::new();
    let mut unique_count_chirho = 0usize;
    for (index_chirho, predicate_chirho) in predicates_chirho.iter().enumerate() {
        let weight_chirho = predicate_weight_chirho(predicate_chirho)?;
        work_chirho = work_chirho
            .checked_sub(weight_chirho)
            .ok_or(PredicateSetErrorChirho::WorkBoundChirho)?;
        let mut hasher_chirho = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(predicate_chirho, &mut hasher_chirho);
        let bucket_chirho = buckets_chirho.entry(hasher_chirho.finish()).or_default();
        let mut existing_chirho = None;
        for &(previous_chirho, target_chirho) in bucket_chirho.iter() {
            work_chirho = work_chirho
                .checked_sub(weight_chirho)
                .ok_or(PredicateSetErrorChirho::WorkBoundChirho)?;
            if predicates_chirho[previous_chirho] == *predicate_chirho {
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
    Ok(plan_chirho)
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
        .ok_or(PredicateSetErrorChirho::TermBoundChirho)?;
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
            .ok_or(PredicateSetErrorChirho::TermBoundChirho)?;
    }
    for term_chirho in predicate_chirho.terms_chirho().iter_chirho() {
        term_weight_chirho(term_chirho, 0, &mut weight_chirho)?;
    }
    if weight_chirho > 4096 {
        return Err(PredicateSetErrorChirho::TermBoundChirho);
    }
    Ok(weight_chirho)
}

fn term_weight_chirho(
    term_chirho: &TyChirho,
    depth_chirho: usize,
    weight_chirho: &mut usize,
) -> Result<(), PredicateSetErrorChirho> {
    if depth_chirho >= 128 || *weight_chirho >= 4096 {
        return Err(PredicateSetErrorChirho::TermBoundChirho);
    }
    *weight_chirho = weight_chirho
        .checked_add(1)
        .ok_or(PredicateSetErrorChirho::TermBoundChirho)?;
    match term_chirho {
        TyChirho::VarChirho(_) => {}
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            *weight_chirho = weight_chirho
                .checked_add(name_chirho.len())
                .ok_or(PredicateSetErrorChirho::TermBoundChirho)?;
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
                .ok_or(PredicateSetErrorChirho::TermBoundChirho)?;
            term_weight_chirho(body_chirho, depth_chirho + 1, weight_chirho)?;
        }
    }
    if *weight_chirho > 4096 {
        return Err(PredicateSetErrorChirho::TermBoundChirho);
    }
    Ok(())
}

#[cfg(test)]
#[path = "predicate_terms_tests_chirho.rs"]
mod tests_chirho;
