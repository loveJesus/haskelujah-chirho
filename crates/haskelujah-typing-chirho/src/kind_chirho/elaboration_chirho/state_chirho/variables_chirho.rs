// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Live variable binding status belongs to the existing State record. Source
//! identity/specificity alone does not distinguish an instantiation meta from
//! a producer-bound universal after the temporary rigid scope has closed.

use super::{ElaborationErrorChirho, ElaborationStateChirho, SpanChirho, TyVarChirho};
use crate::infer_chirho::TypeFamilyEnvChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{QuantifiedVariableChirho, TyChirho};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElaborationVariableBindingChirho {
    MetaChirho,
    UniversalChirho,
    UnknownChirho,
}

#[derive(Debug, Clone)]
pub struct ElaborationVariableChirho {
    pub quantifier_chirho: QuantifiedVariableChirho,
    pub binding_chirho: ElaborationVariableBindingChirho,
}

impl ElaborationVariableChirho {
    pub(crate) fn meta_chirho(quantifier_chirho: QuantifiedVariableChirho) -> Self {
        Self {
            quantifier_chirho,
            binding_chirho: ElaborationVariableBindingChirho::MetaChirho,
        }
    }

    pub(crate) fn unknown_chirho(variable_chirho: TyVarChirho) -> Self {
        Self {
            quantifier_chirho: QuantifiedVariableChirho::unknown_chirho(variable_chirho),
            binding_chirho: ElaborationVariableBindingChirho::UnknownChirho,
        }
    }
}

impl std::ops::Deref for ElaborationVariableChirho {
    type Target = QuantifiedVariableChirho;
    fn deref(&self) -> &Self::Target {
        &self.quantifier_chirho
    }
}

impl From<QuantifiedVariableChirho> for ElaborationVariableChirho {
    fn from(quantifier_chirho: QuantifiedVariableChirho) -> Self {
        Self {
            quantifier_chirho,
            binding_chirho: ElaborationVariableBindingChirho::UnknownChirho,
        }
    }
}

impl std::ops::DerefMut for ElaborationVariableChirho {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.quantifier_chirho
    }
}

impl ElaborationStateChirho {
    /// Follow only an ordinary, committed inference substitution. The existing
    /// table is staged under this owner's remaining allowance. Exhaustion
    /// publishes no records; an unclosed kind equation retains unknown image
    /// metadata and publishes no partial solution. Extra closed kind equations
    /// return in the effective substitution for the ordinary commit path.
    /// workflow: type-identity-chirho/committed-meta-kinds-chirho
    pub(crate) fn follow_committed_variable_kinds_chirho(
        &mut self,
        committed_chirho: &SubstChirho,
        catalogue_chirho: &TypeFamilyEnvChirho,
        span_chirho: SpanChirho,
    ) -> Result<Option<SubstChirho>, ElaborationErrorChirho> {
        self.charge_chirho(0, 0, span_chirho)?;
        let Some(owner_chirho) = self.owner_chirho else {
            return Ok(None);
        };
        if committed_chirho.is_empty_chirho() {
            return Ok(None);
        }

        // Borrow, never reset, the original owner's remaining allowance. A
        // complete table traversal is admitted before it allocates a staging
        // table, and every copied term is separately charged before traversal.
        let initial_chirho = (self.work_left_chirho, self.output_left_chirho);
        let mut remaining_chirho = initial_chirho;
        let staged_chirho = stage_committed_records_chirho(
            &self.variable_kinds_chirho,
            &self.substitution_chirho,
            committed_chirho,
            catalogue_chirho,
            owner_chirho,
            &mut |work_chirho, output_chirho| {
                let next_chirho = remaining_chirho
                    .0
                    .checked_sub(work_chirho)
                    .zip(remaining_chirho.1.checked_sub(output_chirho))?;
                remaining_chirho = next_chirho;
                Some(())
            },
        );
        self.charge_chirho(
            initial_chirho.0 - remaining_chirho.0,
            initial_chirho.1 - remaining_chirho.1,
            span_chirho,
        )?;
        let (records_chirho, effective_chirho) = match staged_chirho {
            Ok(staged_chirho) => staged_chirho,
            Err(CommittedKindFailureChirho::ResourceChirho) => {
                return Err(self.resource_failure_chirho(
                    "committed variable-kind transport bound exhausted",
                    span_chirho,
                ));
            }
        };
        self.variable_kinds_chirho = records_chirho;
        Ok(effective_chirho)
    }

    pub(crate) fn variable_binding_chirho(
        &self,
        variable_chirho: TyVarChirho,
    ) -> ElaborationVariableBindingChirho {
        self.variable_kinds_chirho.get(&variable_chirho).map_or(
            ElaborationVariableBindingChirho::UnknownChirho,
            |record_chirho| record_chirho.binding_chirho,
        )
    }

    /// Only a genuine fresh-image producer calls this admission. Merely
    /// receiving a kind or copied source metadata does not prove Meta status.
    pub(in crate::kind_chirho::elaboration_chirho) fn set_fresh_variable_kind_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        kind_chirho: TyChirho,
    ) {
        let record_chirho = self
            .variable_kinds_chirho
            .entry(variable_chirho)
            .or_insert_with(|| ElaborationVariableChirho::unknown_chirho(variable_chirho));
        if record_chirho.binding_chirho != ElaborationVariableBindingChirho::UniversalChirho {
            record_chirho.binding_chirho = ElaborationVariableBindingChirho::MetaChirho;
        }
        record_chirho.kind_chirho = Some(std::sync::Arc::new(kind_chirho));
    }

    /// Called at an actual written/lexical binder or generalization boundary,
    /// never with imported canonical slot ordinals.
    pub(crate) fn mark_universal_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        self.charge_chirho(
            1,
            usize::from(!self.variable_kinds_chirho.contains_key(&variable_chirho)),
            span_chirho,
        )?;
        self.variable_kinds_chirho
            .entry(variable_chirho)
            .or_insert_with(|| ElaborationVariableChirho::unknown_chirho(variable_chirho))
            .binding_chirho = ElaborationVariableBindingChirho::UniversalChirho;
        Ok(())
    }

    /// Preserve the barrier on the chosen substitution representative even
    /// when neither side has source specificity. Unknown never proves Meta.
    pub(super) fn link_variable_binding_chirho(
        &mut self,
        source_chirho: TyVarChirho,
        target_chirho: TyVarChirho,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        use ElaborationVariableBindingChirho::{MetaChirho, UniversalChirho, UnknownChirho};
        let target_binding_chirho = self.variable_binding_chirho(target_chirho);
        let binding_chirho = match (
            self.variable_binding_chirho(source_chirho),
            target_binding_chirho,
        ) {
            (UniversalChirho, _) | (_, UniversalChirho) => UniversalChirho,
            (MetaChirho, MetaChirho) => MetaChirho,
            _ => UnknownChirho,
        };
        if binding_chirho != target_binding_chirho {
            self.charge_chirho(
                1,
                usize::from(!self.variable_kinds_chirho.contains_key(&target_chirho)),
                span_chirho,
            )?;
            self.variable_kinds_chirho
                .entry(target_chirho)
                .or_insert_with(|| ElaborationVariableChirho::unknown_chirho(target_chirho))
                .binding_chirho = binding_chirho;
        }
        Ok(())
    }
}

enum CommittedKindFailureChirho {
    ResourceChirho,
}

/// Scratch data for this one commit, not a second live catalogue. Source
/// origins and written specificity remain attached to their original records.
fn stage_committed_records_chirho(
    original_chirho: &HashMap<TyVarChirho, ElaborationVariableChirho>,
    checked_links_chirho: &SubstChirho,
    committed_chirho: &SubstChirho,
    catalogue_chirho: &TypeFamilyEnvChirho,
    owner_chirho: haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ElaborationOwnerChirho,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
) -> Result<
    (
        HashMap<TyVarChirho, ElaborationVariableChirho>,
        Option<SubstChirho>,
    ),
    CommittedKindFailureChirho,
> {
    use CommittedKindFailureChirho::ResourceChirho;
    use ElaborationVariableBindingChirho::{MetaChirho, UniversalChirho, UnknownChirho};

    charge_chirho(original_chirho.len(), original_chirho.len()).ok_or(ResourceChirho)?;
    let mut records_chirho = original_chirho.clone();
    // The adopted declaration state still owns its earlier checked kind
    // links. Read them before the new term commit; do not copy ordinary term
    // equations back into that separately scoped declaration substitution.
    for record_chirho in records_chirho.values_mut() {
        charge_chirho(1, 0).ok_or(ResourceChirho)?;
        if let Some(kind_chirho) = &record_chirho.kind_chirho {
            record_chirho.kind_chirho = Some(Arc::new(
                checked_links_chirho
                    .resolve_owned_bounded_chirho(kind_chirho, charge_chirho)
                    .ok_or(ResourceChirho)?,
            ));
        }
    }
    charge_chirho(committed_chirho.len_chirho(), committed_chirho.len_chirho())
        .ok_or(ResourceChirho)?;
    let mut domains_chirho: Vec<_> = committed_chirho
        .iter_chirho()
        .map(|(variable_chirho, _)| *variable_chirho)
        .collect();
    charge_chirho(
        domains_chirho
            .len()
            .checked_mul(
                domains_chirho
                    .len()
                    .checked_ilog2()
                    .unwrap_or(0)
                    .saturating_add(1) as usize,
            )
            .ok_or(ResourceChirho)?,
        0,
    )
    .ok_or(ResourceChirho)?;
    domains_chirho.sort_unstable();

    // Resolve representatives before meeting statuses. This is associative
    // even when several aliases reach the same representative in one commit.
    for variable_chirho in &domains_chirho {
        let image_chirho = committed_chirho
            .resolve_owned_bounded_chirho(&TyChirho::VarChirho(*variable_chirho), charge_chirho)
            .ok_or(ResourceChirho)?;
        let TyChirho::VarChirho(target_chirho) = image_chirho else {
            // Binding to a compound does not grant authority to any of its FVs.
            continue;
        };
        if *variable_chirho == target_chirho {
            continue;
        }
        let source_binding_chirho = original_chirho
            .get(variable_chirho)
            .map_or(UnknownChirho, |record_chirho| record_chirho.binding_chirho);
        let target_binding_chirho = records_chirho
            .get(&target_chirho)
            .map_or(UnknownChirho, |record_chirho| record_chirho.binding_chirho);
        let binding_chirho = match (source_binding_chirho, target_binding_chirho) {
            (UniversalChirho, _) | (_, UniversalChirho) => UniversalChirho,
            (MetaChirho, MetaChirho) => MetaChirho,
            _ => UnknownChirho,
        };
        if let Some(record_chirho) = records_chirho.get_mut(&target_chirho) {
            record_chirho.binding_chirho = binding_chirho;
        } else if binding_chirho == UniversalChirho {
            charge_chirho(1, 1).ok_or(ResourceChirho)?;
            let mut record_chirho = ElaborationVariableChirho::unknown_chirho(target_chirho);
            record_chirho.binding_chirho = UniversalChirho;
            records_chirho.insert(target_chirho, record_chirho);
        }
    }

    let mut equations_chirho = Vec::new();
    let mut unprojected_chirho = HashSet::new();
    for variable_chirho in &domains_chirho {
        let Some(expected_chirho) = records_chirho
            .get(variable_chirho)
            .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
        else {
            charge_chirho(1, 1).ok_or(ResourceChirho)?;
            unprojected_chirho.insert(*variable_chirho);
            continue;
        };
        let image_chirho = committed_chirho
            .resolve_owned_bounded_chirho(&TyChirho::VarChirho(*variable_chirho), charge_chirho)
            .ok_or(ResourceChirho)?;
        let mut image_equations_chirho = Vec::new();
        let actual_chirho = super::super::check_monomorphic_kind_chirho(
            catalogue_chirho,
            owner_chirho,
            &image_chirho,
            committed_chirho,
            &|variable_chirho| {
                records_chirho
                    .get(&variable_chirho)
                    .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
            },
            &mut image_equations_chirho,
            charge_chirho,
        )
        .map_err(|()| ResourceChirho)?;
        let Some(actual_chirho) = actual_chirho else {
            // The strict adapter may lack a compound/scalar kind. Retain that
            // absence; do not turn partial telescope equations into agreement.
            charge_chirho(1, 1).ok_or(ResourceChirho)?;
            unprojected_chirho.insert(*variable_chirho);
            continue;
        };
        let expected_chirho = committed_chirho
            .resolve_owned_bounded_chirho(expected_chirho, charge_chirho)
            .ok_or(ResourceChirho)?;
        charge_chirho(
            image_equations_chirho
                .len()
                .checked_add(1)
                .ok_or(ResourceChirho)?,
            image_equations_chirho
                .len()
                .checked_add(1)
                .ok_or(ResourceChirho)?,
        )
        .ok_or(ResourceChirho)?;
        equations_chirho.extend(image_equations_chirho);
        equations_chirho.push((expected_chirho, actual_chirho));
    }

    let mut kind_solution_chirho = SubstChirho::empty_chirho();
    for (expected_chirho, actual_chirho) in equations_chirho {
        let expected_chirho = kind_solution_chirho
            .resolve_owned_bounded_chirho(&expected_chirho, charge_chirho)
            .ok_or(ResourceChirho)?;
        let actual_chirho = kind_solution_chirho
            .resolve_owned_bounded_chirho(&actual_chirho, charge_chirho)
            .ok_or(ResourceChirho)?;
        let Some(part_chirho) = crate::unify_chirho::unify_monomorphic_bounded_chirho(
            &expected_chirho,
            &actual_chirho,
            &|variable_chirho| {
                records_chirho
                    .get(&variable_chirho)
                    .is_some_and(|record_chirho| {
                        record_chirho.binding_chirho == MetaChirho
                            && record_chirho.kind_chirho.is_some()
                    })
            },
            charge_chirho,
        )
        .map_err(|()| ResourceChirho)?
        else {
            // This prerequisite cannot close the kind equation with actual
            // Meta authority. It does not add a new ordinary solver refusal.
            // Drop the entire local solution and keep the affected image kinds
            // unknown; no partial equation is published as agreement.
            charge_chirho(domains_chirho.len(), domains_chirho.len()).ok_or(ResourceChirho)?;
            unprojected_chirho.extend(domains_chirho.iter().copied());
            kind_solution_chirho = SubstChirho::empty_chirho();
            break;
        };
        kind_solution_chirho = compose_committed_substitution_chirho(
            &part_chirho,
            &kind_solution_chirho,
            charge_chirho,
        )?;
    }

    let effective_chirho = compose_committed_substitution_chirho(
        &kind_solution_chirho,
        committed_chirho,
        charge_chirho,
    )?;
    charge_chirho(unprojected_chirho.len(), unprojected_chirho.len()).ok_or(ResourceChirho)?;
    let uncertain_sources_chirho: Vec<_> = unprojected_chirho.iter().copied().collect();
    for variable_chirho in uncertain_sources_chirho {
        if let TyChirho::VarChirho(target_chirho) = committed_chirho
            .resolve_owned_bounded_chirho(&TyChirho::VarChirho(variable_chirho), charge_chirho)
            .ok_or(ResourceChirho)?
        {
            charge_chirho(1, 1).ok_or(ResourceChirho)?;
            unprojected_chirho.insert(target_chirho);
        }
    }
    for (variable_chirho, record_chirho) in &mut records_chirho {
        charge_chirho(1, 0).ok_or(ResourceChirho)?;
        let dependencies_chirho = record_chirho
            .kind_chirho
            .as_ref()
            .map(|kind_chirho| {
                SubstChirho::free_variables_bounded_chirho(kind_chirho, charge_chirho)
                    .ok_or(ResourceChirho)
            })
            .transpose()?;
        if let Some(dependencies_chirho) = &dependencies_chirho {
            charge_chirho(dependencies_chirho.len(), 0).ok_or(ResourceChirho)?;
        }
        if unprojected_chirho.contains(variable_chirho)
            || dependencies_chirho.is_some_and(|dependencies_chirho| {
                dependencies_chirho
                    .iter()
                    .any(|variable_chirho| unprojected_chirho.contains(variable_chirho))
            })
        {
            record_chirho.kind_chirho = None;
            if record_chirho.binding_chirho == MetaChirho {
                record_chirho.binding_chirho = UnknownChirho;
            }
        } else if let Some(kind_chirho) = &record_chirho.kind_chirho {
            record_chirho.kind_chirho = Some(Arc::new(
                effective_chirho
                    .resolve_owned_bounded_chirho(kind_chirho, charge_chirho)
                    .ok_or(ResourceChirho)?,
            ));
        }
    }
    Ok((
        records_chirho,
        (!kind_solution_chirho.is_empty_chirho()).then_some(effective_chirho),
    ))
}

/// Compose through the existing entry payload, keeping source and checked-kind
/// metadata instead of reconstructing substitution entries from a written key.
fn compose_committed_substitution_chirho(
    after_chirho: &SubstChirho,
    before_chirho: &SubstChirho,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
) -> Result<SubstChirho, CommittedKindFailureChirho> {
    use CommittedKindFailureChirho::ResourceChirho;
    let mut composed_chirho = SubstChirho::empty_chirho();
    charge_chirho(before_chirho.len_chirho(), before_chirho.len_chirho()).ok_or(ResourceChirho)?;
    for (variable_chirho, image_chirho) in before_chirho.iter_chirho() {
        let entry_chirho = before_chirho
            .lookup_image_chirho(variable_chirho)
            .expect("iterated substitution entry");
        let kind_chirho = entry_chirho
            .kind_chirho
            .as_ref()
            .map(|kind_chirho| {
                after_chirho
                    .resolve_owned_bounded_chirho(kind_chirho, charge_chirho)
                    .map(Arc::new)
                    .ok_or(ResourceChirho)
            })
            .transpose()?;
        composed_chirho.insert_image_chirho(
            *variable_chirho,
            crate::subst_chirho::SubstitutionImageChirho {
                image_chirho: after_chirho
                    .resolve_owned_bounded_chirho(image_chirho, charge_chirho)
                    .ok_or(ResourceChirho)?,
                kind_chirho,
                source_chirho: entry_chirho.source_chirho.clone(),
            },
        );
    }
    charge_chirho(after_chirho.len_chirho(), after_chirho.len_chirho()).ok_or(ResourceChirho)?;
    for (variable_chirho, image_chirho) in after_chirho.iter_chirho() {
        if composed_chirho.lookup_chirho(variable_chirho).is_some() {
            continue;
        }
        let entry_chirho = after_chirho
            .lookup_image_chirho(variable_chirho)
            .expect("iterated substitution entry");
        composed_chirho.insert_image_chirho(
            *variable_chirho,
            crate::subst_chirho::SubstitutionImageChirho {
                image_chirho: SubstChirho::empty_chirho()
                    .apply_bounded_chirho(image_chirho, charge_chirho)
                    .ok_or(ResourceChirho)?,
                kind_chirho: entry_chirho.kind_chirho.clone(),
                source_chirho: entry_chirho.source_chirho.clone(),
            },
        );
    }
    Ok(composed_chirho)
}
