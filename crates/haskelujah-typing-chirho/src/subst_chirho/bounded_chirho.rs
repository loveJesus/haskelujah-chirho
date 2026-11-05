// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded simultaneous substitution for already-freshened checked terms.
//! Replacement images are never reinterpreted in the producer's binder scope.

use super::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use std::collections::HashSet;

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
            bound_chirho.extend(vars_chirho);
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

struct BoundedSubstitutionChirho<'source_chirho, 'budget_chirho, ChargeChirho> {
    source_chirho: &'source_chirho SubstChirho,
    free_chirho: Vec<(TyVarChirho, HashSet<TyVarChirho>)>,
    charge_chirho: &'budget_chirho mut ChargeChirho,
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
                (self.charge_chirho)(bound_chirho.len(), 0)?;
                if active_chirho
                    && !bound_chirho.contains(variable_chirho)
                    && let Some(image_chirho) = self.source_chirho.lookup_chirho(variable_chirho)
                {
                    return self.walk_chirho(
                        image_chirho,
                        &mut Vec::new(),
                        false,
                        depth_chirho + 1,
                    );
                }
                TyChirho::VarChirho(*variable_chirho)
            }
            TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                (self.charge_chirho)(name_chirho.len(), name_chirho.len())?;
                term_chirho.clone()
            }
            TyChirho::AppChirho(left_chirho, right_chirho) => TyChirho::AppChirho(
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
            ),
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
                if active_chirho {
                    for (source_chirho, free_chirho) in &self.free_chirho {
                        (self.charge_chirho)(
                            bound_chirho.len().saturating_add(vars_chirho.len()),
                            0,
                        )?;
                        if !bound_chirho.contains(source_chirho)
                            && !vars_chirho.contains(source_chirho)
                            && vars_chirho
                                .iter()
                                .any(|variable_chirho| free_chirho.contains(variable_chirho))
                        {
                            return None;
                        }
                    }
                }
                let mark_chirho = bound_chirho.len();
                bound_chirho.extend(vars_chirho);
                let body_chirho =
                    self.walk_chirho(body_chirho, bound_chirho, active_chirho, depth_chirho + 1);
                bound_chirho.truncate(mark_chirho);
                let body_chirho = Box::new(body_chirho?);
                if matches!(term_chirho, TyChirho::RequiredForallChirho { .. }) {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: vars_chirho.clone(),
                        body_chirho,
                    }
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: vars_chirho.clone(),
                        body_chirho,
                    }
                }
            }
        })
    }
}

impl SubstChirho {
    /// Callers have freshened producer binders first. A remaining collision is
    /// an explicit refusal, never accidental capture or spelling-based repair.
    pub(crate) fn apply_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<TyChirho> {
        charge_chirho(self.len_chirho(), self.len_chirho())?;
        let mut free_chirho = Vec::new();
        for (variable_chirho, image_chirho) in self.iter_chirho() {
            let mut free_image_chirho = HashSet::new();
            image_free_chirho(
                image_chirho,
                &mut free_image_chirho,
                &mut Vec::new(),
                charge_chirho,
                0,
            )?;
            free_chirho.push((*variable_chirho, free_image_chirho));
        }
        BoundedSubstitutionChirho {
            source_chirho: self,
            free_chirho,
            charge_chirho,
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
                    vars_chirho: remaining_chirho.to_vec(),
                    body_chirho: Box::new(body_chirho),
                }
            } else {
                TyChirho::ForallChirho {
                    vars_chirho: remaining_chirho.to_vec(),
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
