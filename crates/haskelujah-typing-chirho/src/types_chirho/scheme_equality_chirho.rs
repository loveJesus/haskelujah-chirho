// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact closed-scheme equality, not unification or a permissive subsumption.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use crate::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use std::collections::HashMap;
#[cfg(test)]
#[path = "scheme_equality_tests_chirho.rs"]
mod tests_chirho;

fn canonical_type_chirho(
    ty_chirho: &TyChirho,
    identities_chirho: &mut HashMap<TyVarChirho, TyVarChirho>,
    named_chirho: &HashMap<&str, TyVarChirho>,
    next_chirho: &mut u32,
    budget_chirho: &mut usize,
    depth_chirho: usize,
) -> Option<TyChirho> {
    *budget_chirho = budget_chirho.checked_sub(1)?;
    if depth_chirho > 256 {
        return None;
    }
    let mut child_chirho = |ty_chirho: &TyChirho| {
        canonical_type_chirho(
            ty_chirho,
            identities_chirho,
            named_chirho,
            next_chirho,
            budget_chirho,
            depth_chirho + 1,
        )
    };
    Some(match ty_chirho {
        TyChirho::VarChirho(identity_chirho) => {
            TyChirho::VarChirho(*identities_chirho.get(identity_chirho)?)
        }
        TyChirho::ConChirho(name_chirho) => named_chirho
            .get(name_chirho.as_str())
            .map(|identity_chirho| TyChirho::VarChirho(*identity_chirho))
            .unwrap_or_else(|| ty_chirho.clone()),
        // A named rigid variable here is free, not a promise we can close.
        TyChirho::ForallVarChirho(name_chirho) => {
            TyChirho::VarChirho(*named_chirho.get(name_chirho.as_str())?)
        }
        TyChirho::AppChirho(left_chirho, right_chirho) => TyChirho::AppChirho(
            Box::new(child_chirho(left_chirho)?),
            Box::new(child_chirho(right_chirho)?),
        ),
        TyChirho::KindAppChirho(left_chirho, right_chirho) => TyChirho::KindAppChirho(
            Box::new(child_chirho(left_chirho)?),
            Box::new(child_chirho(right_chirho)?),
        ),
        TyChirho::FunChirho(left_chirho, right_chirho, multiplicity_chirho) => TyChirho::FunChirho(
            Box::new(child_chirho(left_chirho)?),
            Box::new(child_chirho(right_chirho)?),
            *multiplicity_chirho,
        ),
        TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
            elements_chirho
                .iter()
                .map(child_chirho)
                .collect::<Option<_>>()?,
        ),
        TyChirho::ListChirho(element_chirho) => {
            TyChirho::ListChirho(Box::new(child_chirho(element_chirho)?))
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            let mut undo_chirho = Vec::new();
            let mut canonical_vars_chirho = Vec::new();
            for identity_chirho in vars_chirho {
                let fresh_chirho = TyVarChirho(*next_chirho);
                *next_chirho = next_chirho.checked_add(1)?;
                undo_chirho.push((
                    *identity_chirho,
                    identities_chirho.insert(*identity_chirho, fresh_chirho),
                ));
                canonical_vars_chirho.push(fresh_chirho);
            }
            let body_chirho = canonical_type_chirho(
                body_chirho,
                identities_chirho,
                named_chirho,
                next_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?;
            for (identity_chirho, previous_chirho) in undo_chirho.into_iter().rev() {
                if let Some(previous_chirho) = previous_chirho {
                    identities_chirho.insert(identity_chirho, previous_chirho);
                } else {
                    identities_chirho.remove(&identity_chirho);
                }
            }
            if matches!(ty_chirho, TyChirho::ForallChirho { .. }) {
                TyChirho::ForallChirho {
                    vars_chirho: canonical_vars_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            } else {
                TyChirho::RequiredForallChirho {
                    vars_chirho: canonical_vars_chirho,
                    body_chirho: Box::new(body_chirho),
                }
            }
        }
    })
}

fn canonical_scheme_chirho(scheme_chirho: &SchemeChirho) -> Option<SchemeChirho> {
    let mut identities_chirho = HashMap::new();
    let mut next_chirho = 0u32;
    for identity_chirho in &scheme_chirho.vars_chirho {
        if identities_chirho
            .insert(*identity_chirho, TyVarChirho(next_chirho))
            .is_some()
        {
            return None;
        }
        next_chirho = next_chirho.checked_add(1)?;
    }
    let mut budget_chirho = 16_384;
    let named_chirho = HashMap::new();
    let mut convert_chirho = |ty_chirho: &TyChirho| {
        canonical_type_chirho(
            ty_chirho,
            &mut identities_chirho,
            &named_chirho,
            &mut next_chirho,
            &mut budget_chirho,
            0,
        )
    };
    let ty_chirho = convert_chirho(&scheme_chirho.ty_chirho)?;
    let preds_chirho = scheme_chirho
        .preds_chirho
        .iter()
        .map(|pred_chirho| {
            Some(SchemePredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: convert_chirho(&pred_chirho.ty_chirho)?,
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(&mut convert_chirho)
                    .collect::<Option<_>>()?,
            })
        })
        .collect::<Option<_>>()?;
    Some(SchemeChirho {
        vars_chirho: (0..scheme_chirho.vars_chirho.len() as u32)
            .map(TyVarChirho)
            .collect(),
        preds_chirho,
        ty_chirho,
    })
}

pub(crate) fn canonical_named_body_chirho(
    parameters_chirho: &[&str],
    body_chirho: &TyChirho,
) -> Option<TyChirho> {
    let named_chirho: HashMap<_, _> = parameters_chirho
        .iter()
        .enumerate()
        .map(|(index_chirho, name_chirho)| (*name_chirho, TyVarChirho(index_chirho as u32)))
        .collect();
    if named_chirho.len() != parameters_chirho.len() {
        return None;
    }
    canonical_type_chirho(
        body_chirho,
        &mut HashMap::new(),
        &named_chirho,
        &mut (parameters_chirho.len() as u32),
        &mut 16_384,
        0,
    )
}

impl SchemeChirho {
    /// Both promises must be closed. Bound IDs may differ; retained kind arguments, constructor
    /// identities, predicates, quantifier visibility and multiplicity may not.
    pub fn alpha_equivalent_chirho(&self, other_chirho: &Self) -> bool {
        match (
            canonical_scheme_chirho(self),
            canonical_scheme_chirho(other_chirho),
        ) {
            (Some(left_chirho), Some(right_chirho)) => left_chirho == right_chirho,
            _ => false,
        }
    }
}
