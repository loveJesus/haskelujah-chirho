// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded simultaneous substitution for already-freshened checked terms.
//! Replacement images are never reinterpreted in the producer's binder scope.

use super::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use std::collections::{HashMap, HashSet};

fn image_free_chirho(
    term_chirho: &TyChirho,
    free_chirho: &mut HashSet<TyVarChirho>,
    bound_chirho: &mut Vec<TyVarChirho>,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    depth_chirho: usize,
) -> Option<()> {
    charge_chirho(1, 0)?;
    if depth_chirho > 128 {
        return None;
    }
    match term_chirho {
        TyChirho::VarChirho(variable_chirho) => {
            charge_chirho(bound_chirho.len(), 0)?;
            if !bound_chirho.contains(variable_chirho) {
                charge_chirho(1, 1)?;
                free_chirho.insert(*variable_chirho);
            }
        }
        TyChirho::AppChirho(left_chirho, right_chirho)
        | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
            image_free_chirho(
                left_chirho,
                free_chirho,
                bound_chirho,
                charge_chirho,
                depth_chirho + 1,
            )?;
            image_free_chirho(
                right_chirho,
                free_chirho,
                bound_chirho,
                charge_chirho,
                depth_chirho + 1,
            )?;
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            for term_chirho in context_chirho {
                image_free_chirho(
                    term_chirho,
                    free_chirho,
                    bound_chirho,
                    charge_chirho,
                    depth_chirho + 1,
                )?;
            }
            image_free_chirho(
                body_chirho,
                free_chirho,
                bound_chirho,
                charge_chirho,
                depth_chirho + 1,
            )?;
        }
        TyChirho::TupleChirho(elements_chirho) => {
            for element_chirho in elements_chirho {
                image_free_chirho(
                    element_chirho,
                    free_chirho,
                    bound_chirho,
                    charge_chirho,
                    depth_chirho + 1,
                )?;
            }
        }
        TyChirho::ListChirho(element_chirho) => image_free_chirho(
            element_chirho,
            free_chirho,
            bound_chirho,
            charge_chirho,
            depth_chirho + 1,
        )?,
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            charge_chirho(vars_chirho.len(), vars_chirho.len())?;
            let mark_chirho = bound_chirho.len();
            for record_chirho in vars_chirho.records_chirho() {
                if let Some(kind_chirho) = &record_chirho.kind_chirho {
                    image_free_chirho(
                        kind_chirho,
                        free_chirho,
                        bound_chirho,
                        charge_chirho,
                        depth_chirho + 1,
                    )?;
                }
                bound_chirho.push(record_chirho.variable_chirho);
            }
            let result_chirho = image_free_chirho(
                body_chirho,
                free_chirho,
                bound_chirho,
                charge_chirho,
                depth_chirho + 1,
            );
            bound_chirho.truncate(mark_chirho);
            result_chirho?;
        }
        TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
    }
    Some(())
}

fn reserve_variables_chirho(
    term_chirho: &TyChirho,
    next_chirho: &mut u32,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    depth_chirho: usize,
) -> Option<()> {
    charge_chirho(1, 0)?;
    if depth_chirho > 128 {
        return None;
    }
    let mut reserve_chirho = |variable_chirho: TyVarChirho| -> Option<()> {
        *next_chirho = (*next_chirho).max(variable_chirho.0.checked_add(1)?);
        Some(())
    };
    match term_chirho {
        TyChirho::VarChirho(variable_chirho) => reserve_chirho(*variable_chirho)?,
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            charge_chirho(vars_chirho.len(), 0)?;
            for record_chirho in vars_chirho.records_chirho() {
                reserve_chirho(record_chirho.variable_chirho)?;
            }
            for record_chirho in vars_chirho.records_chirho() {
                if let Some(kind_chirho) = &record_chirho.kind_chirho {
                    reserve_variables_chirho(
                        kind_chirho,
                        next_chirho,
                        charge_chirho,
                        depth_chirho + 1,
                    )?;
                }
            }
            reserve_variables_chirho(body_chirho, next_chirho, charge_chirho, depth_chirho + 1)?;
        }
        TyChirho::AppChirho(left_chirho, right_chirho)
        | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
            reserve_variables_chirho(left_chirho, next_chirho, charge_chirho, depth_chirho + 1)?;
            reserve_variables_chirho(right_chirho, next_chirho, charge_chirho, depth_chirho + 1)?;
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            for term_chirho in context_chirho {
                reserve_variables_chirho(
                    term_chirho,
                    next_chirho,
                    charge_chirho,
                    depth_chirho + 1,
                )?;
            }
            reserve_variables_chirho(body_chirho, next_chirho, charge_chirho, depth_chirho + 1)?;
        }
        TyChirho::ListChirho(element_chirho) => {
            reserve_variables_chirho(element_chirho, next_chirho, charge_chirho, depth_chirho + 1)?
        }
        TyChirho::TupleChirho(elements_chirho) => {
            for term_chirho in elements_chirho {
                reserve_variables_chirho(
                    term_chirho,
                    next_chirho,
                    charge_chirho,
                    depth_chirho + 1,
                )?;
            }
        }
        TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
    }
    Some(())
}

struct BoundedSubstitutionChirho<'source_chirho, 'budget_chirho, ChargeChirho> {
    source_chirho: &'source_chirho SubstChirho,
    free_chirho: HashMap<TyVarChirho, HashSet<TyVarChirho>>,
    charge_chirho: &'budget_chirho mut ChargeChirho,
    resolve_owned_chirho: bool,
    active_variables_chirho: HashSet<TyVarChirho>,
    renaming_chirho: Vec<(TyVarChirho, TyVarChirho)>,
    next_variable_chirho: Option<&'budget_chirho mut u32>,
}

impl<ChargeChirho: FnMut(usize, usize) -> Option<()>>
    BoundedSubstitutionChirho<'_, '_, ChargeChirho>
{
    fn walk_chirho(
        &mut self,
        term_chirho: &TyChirho,
        bound_chirho: &mut Vec<TyVarChirho>,
        active_chirho: bool,
        depth_chirho: usize,
    ) -> Option<TyChirho> {
        (self.charge_chirho)(1, 1)?;
        if depth_chirho > 128 {
            return None;
        }
        Some(match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                (self.charge_chirho)(self.renaming_chirho.len(), 0)?;
                if active_chirho
                    && let Some((_, image_chirho)) = self
                        .renaming_chirho
                        .iter()
                        .rev()
                        .find(|(source_chirho, _)| source_chirho == variable_chirho)
                {
                    return Some(TyChirho::VarChirho(*image_chirho));
                }
                (self.charge_chirho)(bound_chirho.len(), 0)?;
                if active_chirho
                    && !bound_chirho.contains(variable_chirho)
                    && let Some(image_chirho) = self.source_chirho.lookup_chirho(variable_chirho)
                {
                    if self.resolve_owned_chirho {
                        (self.charge_chirho)(2, 1)?;
                        if !self.active_variables_chirho.insert(*variable_chirho) {
                            return None;
                        }
                        let result_chirho =
                            self.walk_chirho(image_chirho, bound_chirho, true, depth_chirho + 1);
                        self.active_variables_chirho.remove(variable_chirho);
                        return result_chirho;
                    }
                    if self.next_variable_chirho.is_none()
                        && let Some(free_chirho) = self.free_chirho.get(variable_chirho)
                    {
                        (self.charge_chirho)(bound_chirho.len(), 0)?;
                        if bound_chirho
                            .iter()
                            .any(|variable_chirho| free_chirho.contains(variable_chirho))
                        {
                            return None;
                        }
                    }
                    let next_variable_chirho = self.next_variable_chirho.take();
                    let renaming_chirho = std::mem::take(&mut self.renaming_chirho);
                    let result_chirho =
                        self.walk_chirho(image_chirho, &mut Vec::new(), false, depth_chirho + 1);
                    self.next_variable_chirho = next_variable_chirho;
                    self.renaming_chirho = renaming_chirho;
                    return result_chirho;
                }
                TyChirho::VarChirho(*variable_chirho)
            }
            TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                (self.charge_chirho)(name_chirho.len(), name_chirho.len())?;
                term_chirho.clone()
            }
            TyChirho::AppChirho(left_chirho, right_chirho) => {
                let left_chirho =
                    self.walk_chirho(left_chirho, bound_chirho, active_chirho, depth_chirho + 1)?;
                let right_chirho =
                    self.walk_chirho(right_chirho, bound_chirho, active_chirho, depth_chirho + 1)?;
                if let Some(key_chirho) =
                    crate::ty_chirho::representation_chirho::folded_universe_key_chirho(
                        &left_chirho,
                        &right_chirho,
                    )
                {
                    (self.charge_chirho)(key_chirho.len(), key_chirho.len())?;
                    TyChirho::ConChirho(key_chirho.to_owned())
                } else {
                    TyChirho::AppChirho(Box::new(left_chirho), Box::new(right_chirho))
                }
            }
            TyChirho::FunChirho(left_chirho, right_chirho, multiplicity_chirho) => {
                TyChirho::FunChirho(
                    Box::new(self.walk_chirho(
                        left_chirho,
                        bound_chirho,
                        active_chirho,
                        depth_chirho + 1,
                    )?),
                    Box::new(self.walk_chirho(
                        right_chirho,
                        bound_chirho,
                        active_chirho,
                        depth_chirho + 1,
                    )?),
                    *multiplicity_chirho,
                )
            }
            TyChirho::ListChirho(element_chirho) => {
                TyChirho::ListChirho(Box::new(self.walk_chirho(
                    element_chirho,
                    bound_chirho,
                    active_chirho,
                    depth_chirho + 1,
                )?))
            }
            TyChirho::TupleChirho(elements_chirho) => {
                (self.charge_chirho)(elements_chirho.len(), elements_chirho.len())?;
                TyChirho::TupleChirho(
                    elements_chirho
                        .iter()
                        .map(|element_chirho| {
                            self.walk_chirho(
                                element_chirho,
                                bound_chirho,
                                active_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Option<_>>()?,
                )
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                (self.charge_chirho)(context_chirho.len(), context_chirho.len())?;
                TyChirho::QualChirho {
                    context_chirho: context_chirho
                        .iter()
                        .map(|term_chirho| {
                            self.walk_chirho(
                                term_chirho,
                                bound_chirho,
                                active_chirho,
                                depth_chirho + 1,
                            )
                        })
                        .collect::<Option<_>>()?,
                    body_chirho: Box::new(self.walk_chirho(
                        body_chirho,
                        bound_chirho,
                        active_chirho,
                        depth_chirho + 1,
                    )?),
                }
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                (self.charge_chirho)(vars_chirho.len(), vars_chirho.len())?;
                let mark_chirho = bound_chirho.len();
                let rename_mark_chirho = self.renaming_chirho.len();
                let mut records_chirho = Vec::new();
                for record_chirho in vars_chirho.records_chirho() {
                    let mut mapped_chirho = record_chirho.clone();
                    mapped_chirho.kind_chirho = match &record_chirho.kind_chirho {
                        Some(kind_chirho) => Some(std::sync::Arc::new(self.walk_chirho(
                            kind_chirho,
                            bound_chirho,
                            active_chirho,
                            depth_chirho + 1,
                        )?)),
                        None => None,
                    };
                    if active_chirho
                        && let Some(next_chirho) = self.next_variable_chirho.as_deref_mut()
                    {
                        mapped_chirho.variable_chirho = TyVarChirho(*next_chirho);
                        *next_chirho = next_chirho.checked_add(1)?;
                        (self.charge_chirho)(1, 1)?;
                        self.renaming_chirho
                            .push((record_chirho.variable_chirho, mapped_chirho.variable_chirho));
                    }
                    records_chirho.push(mapped_chirho);
                    bound_chirho.push(record_chirho.variable_chirho);
                }
                let body_chirho =
                    self.walk_chirho(body_chirho, bound_chirho, active_chirho, depth_chirho + 1);
                bound_chirho.truncate(mark_chirho);
                self.renaming_chirho.truncate(rename_mark_chirho);
                let body_chirho = Box::new(body_chirho?);
                let variables_chirho =
                    crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                        records_chirho,
                    );
                if matches!(term_chirho, TyChirho::RequiredForallChirho { .. }) {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho,
                    }
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: variables_chirho,
                        body_chirho,
                    }
                }
            }
        })
    }
}

impl SubstChirho {
    pub(crate) fn reserve_fresh_variables_bounded_chirho(
        term_chirho: &TyChirho,
        next_chirho: &mut u32,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<()> {
        reserve_variables_chirho(term_chirho, next_chirho, charge_chirho, 0)
    }

    pub(crate) fn free_variables_bounded_chirho(
        term_chirho: &TyChirho,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<Vec<TyVarChirho>> {
        let mut free_chirho = HashSet::new();
        image_free_chirho(
            term_chirho,
            &mut free_chirho,
            &mut Vec::new(),
            charge_chirho,
            0,
        )?;
        let mut free_chirho: Vec<_> = free_chirho.into_iter().collect();
        charge_chirho(
            free_chirho.len().saturating_mul(
                free_chirho
                    .len()
                    .checked_ilog2()
                    .unwrap_or(0)
                    .saturating_add(1) as usize,
            ),
            0,
        )?;
        free_chirho.sort_unstable();
        Some(free_chirho)
    }

    /// Zonk only an already owner-validated solver substitution. Its images
    /// name that SAME lexical scope, so a preceding binder in an image stays
    /// bound. External argument instantiation uses apply_bounded_chirho instead.
    pub(crate) fn resolve_owned_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<TyChirho> {
        BoundedSubstitutionChirho {
            source_chirho: self,
            free_chirho: HashMap::new(),
            charge_chirho,
            resolve_owned_chirho: true,
            active_variables_chirho: HashSet::new(),
            renaming_chirho: Vec::new(),
            next_variable_chirho: None,
        }
        .walk_chirho(term_chirho, &mut Vec::new(), true, 0)
    }

    /// Simultaneously rekey free producer slots while alpha-renaming only
    /// binders introduced inside this term. Replacement images keep their scope.
    pub(crate) fn apply_freshened_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        next_chirho: &mut u32,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<TyChirho> {
        reserve_variables_chirho(term_chirho, next_chirho, charge_chirho, 0)?;
        let used_chirho = Self::free_variables_bounded_chirho(term_chirho, charge_chirho)?;
        charge_chirho(used_chirho.len(), used_chirho.len())?;
        let mut free_chirho = HashMap::new();
        for variable_chirho in used_chirho {
            let Some(image_chirho) = self.lookup_chirho(&variable_chirho) else {
                continue;
            };
            reserve_variables_chirho(image_chirho, next_chirho, charge_chirho, 0)?;
            let mut free_image_chirho = HashSet::new();
            image_free_chirho(
                image_chirho,
                &mut free_image_chirho,
                &mut Vec::new(),
                charge_chirho,
                0,
            )?;
            free_chirho.insert(variable_chirho, free_image_chirho);
        }
        BoundedSubstitutionChirho {
            source_chirho: self,
            free_chirho,
            charge_chirho,
            resolve_owned_chirho: false,
            active_variables_chirho: HashSet::new(),
            renaming_chirho: Vec::new(),
            next_variable_chirho: Some(next_chirho),
        }
        .walk_chirho(term_chirho, &mut Vec::new(), true, 0)
    }

    /// Callers have freshened producer binders first. A remaining collision is
    /// an explicit refusal, never accidental capture or spelling-based repair.
    pub(crate) fn apply_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<TyChirho> {
        let used_chirho = Self::free_variables_bounded_chirho(term_chirho, charge_chirho)?;
        charge_chirho(used_chirho.len(), used_chirho.len())?;
        let mut free_chirho = HashMap::new();
        for variable_chirho in used_chirho {
            let Some(image_chirho) = self.lookup_chirho(&variable_chirho) else {
                continue;
            };
            let mut free_image_chirho = HashSet::new();
            image_free_chirho(
                image_chirho,
                &mut free_image_chirho,
                &mut Vec::new(),
                charge_chirho,
                0,
            )?;
            free_chirho.insert(variable_chirho, free_image_chirho);
        }
        BoundedSubstitutionChirho {
            source_chirho: self,
            free_chirho,
            charge_chirho,
            resolve_owned_chirho: false,
            active_variables_chirho: HashSet::new(),
            renaming_chirho: Vec::new(),
            next_variable_chirho: None,
        }
        .walk_chirho(term_chirho, &mut Vec::new(), true, 0)
    }
}

fn quantified_body_chirho(
    term_chirho: &TyChirho,
    charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    depth_chirho: usize,
) -> Option<Option<(TyVarChirho, TyChirho)>> {
    charge_chirho(1, 1)?;
    if depth_chirho > 128 {
        return None;
    }
    let empty_chirho = SubstChirho::empty_chirho();
    match term_chirho {
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            let Some((variable_chirho, body_chirho)) =
                quantified_body_chirho(body_chirho, charge_chirho, depth_chirho + 1)?
            else {
                return Some(None);
            };
            charge_chirho(context_chirho.len(), context_chirho.len())?;
            let mut copied_chirho = Vec::new();
            for term_chirho in context_chirho {
                copied_chirho.push(empty_chirho.apply_bounded_chirho(term_chirho, charge_chirho)?);
            }
            Some(Some((
                variable_chirho,
                TyChirho::QualChirho {
                    context_chirho: copied_chirho,
                    body_chirho: Box::new(body_chirho),
                },
            )))
        }
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        }
        | TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => {
            let Some((variable_chirho, remaining_chirho)) = vars_chirho.split_first() else {
                return quantified_body_chirho(body_chirho, charge_chirho, depth_chirho + 1);
            };
            charge_chirho(remaining_chirho.len(), remaining_chirho.len())?;
            let body_chirho = empty_chirho.apply_bounded_chirho(body_chirho, charge_chirho)?;
            let body_chirho = if remaining_chirho.is_empty() {
                body_chirho
            } else if matches!(term_chirho, TyChirho::RequiredForallChirho { .. }) {
                TyChirho::RequiredForallChirho {
                    vars_chirho: vars_chirho.slice_chirho(1..vars_chirho.len()),
                    body_chirho: Box::new(body_chirho),
                }
            } else {
                TyChirho::ForallChirho {
                    vars_chirho: vars_chirho.slice_chirho(1..vars_chirho.len()),
                    body_chirho: Box::new(body_chirho),
                }
            };
            Some(Some((*variable_chirho, body_chirho)))
        }
        _ => Some(None),
    }
}

impl SubstChirho {
    /// `quantified_chirho` comes from a checked result-kind cursor. A forall
    /// occurring in an ordinary Type-kind term grants no beta authority.
    pub(crate) fn reduce_checked_argument_chirho(
        function_chirho: &TyChirho,
        argument_chirho: &TyChirho,
        quantified_chirho: bool,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<Option<(TyChirho, SubstChirho)>> {
        if !quantified_chirho {
            return Some(None);
        }
        if let Some((variable_chirho, body_chirho)) =
            quantified_body_chirho(function_chirho, charge_chirho, 0)?
        {
            let replacement_chirho = Self::singleton_chirho(
                variable_chirho,
                Self::empty_chirho().apply_bounded_chirho(argument_chirho, charge_chirho)?,
            );
            return Some(Some((
                replacement_chirho.apply_bounded_chirho(&body_chirho, charge_chirho)?,
                replacement_chirho,
            )));
        }
        Some(None)
    }
}
