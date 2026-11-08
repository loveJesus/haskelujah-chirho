// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordinary scheme spines are iterative. Lexical binder scopes use the common
//! checked substitution and its existing depth limit under the SAME allowance.

use super::SubstChirho;
use crate::ty_chirho::TyChirho;
use std::collections::HashSet;

enum OrdinaryTermTaskChirho<'term_chirho> {
    VisitChirho(&'term_chirho TyChirho, bool),
    FinishChirho(&'term_chirho TyChirho),
}

impl SubstChirho {
    /// Reserve semantic IDs before introducing a nested consumer binder. This
    /// walk borrows ordinary structure and never recursively follows images.
    fn reserve_ordinary_ids_chirho(
        &self,
        term_chirho: &TyChirho,
        next_chirho: &mut u32,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<()> {
        let mut pending_chirho = vec![(term_chirho, true)];
        let mut images_chirho = HashSet::new();
        while let Some((term_chirho, active_chirho)) = pending_chirho.pop() {
            charge_chirho(1, 0)?;
            match term_chirho {
                TyChirho::VarChirho(variable_chirho) => {
                    *next_chirho = (*next_chirho).max(variable_chirho.0.checked_add(1)?);
                    if active_chirho && images_chirho.insert(*variable_chirho) {
                        charge_chirho(1, 1)?;
                        if let Some(image_chirho) = self.lookup_chirho(variable_chirho) {
                            pending_chirho.push((image_chirho, false));
                        }
                    }
                }
                TyChirho::AppChirho(left_chirho, right_chirho)
                | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                    charge_chirho(2, 2)?;
                    pending_chirho.extend([
                        (left_chirho.as_ref(), active_chirho),
                        (right_chirho.as_ref(), active_chirho),
                    ]);
                }
                TyChirho::ListChirho(element_chirho) => {
                    charge_chirho(1, 1)?;
                    pending_chirho.push((element_chirho, active_chirho));
                }
                TyChirho::TupleChirho(elements_chirho) => {
                    charge_chirho(elements_chirho.len(), elements_chirho.len())?;
                    pending_chirho.extend(
                        elements_chirho
                            .iter()
                            .map(|term_chirho| (term_chirho, active_chirho)),
                    );
                }
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    charge_chirho(
                        context_chirho.len().checked_add(1)?,
                        context_chirho.len().checked_add(1)?,
                    )?;
                    pending_chirho.push((body_chirho, active_chirho));
                    pending_chirho.extend(
                        context_chirho
                            .iter()
                            .map(|term_chirho| (term_chirho, active_chirho)),
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
                    charge_chirho(
                        vars_chirho.len().checked_add(1)?,
                        vars_chirho.len().checked_add(1)?,
                    )?;
                    pending_chirho.push((body_chirho, active_chirho));
                    for record_chirho in vars_chirho.records_chirho() {
                        *next_chirho =
                            (*next_chirho).max(record_chirho.variable_chirho.0.checked_add(1)?);
                        if let Some(kind_chirho) = &record_chirho.kind_chirho {
                            pending_chirho.push((kind_chirho, active_chirho));
                        }
                    }
                }
                TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
            }
        }
        Some(())
    }

    /// Simultaneous substitution on an ordinary term spine. A soft metadata
    /// refusal does not discard its deep ordinary predicate. Every pending
    /// task, copied node and lexical operation shares the caller's budget.
    pub(crate) fn apply_scheme_term_bounded_chirho(
        &self,
        term_chirho: &TyChirho,
        next_chirho: &mut u32,
        charge_chirho: &mut impl FnMut(usize, usize) -> Option<()>,
    ) -> Option<TyChirho> {
        self.reserve_ordinary_ids_chirho(term_chirho, next_chirho, charge_chirho)?;
        let mut pending_chirho = vec![OrdinaryTermTaskChirho::VisitChirho(term_chirho, true)];
        let mut values_chirho = Vec::new();
        while let Some(task_chirho) = pending_chirho.pop() {
            charge_chirho(1, 1)?;
            match task_chirho {
                OrdinaryTermTaskChirho::VisitChirho(term_chirho, active_chirho) => {
                    match term_chirho {
                        TyChirho::VarChirho(variable_chirho) => {
                            if active_chirho
                                && let Some(image_chirho) = self.lookup_chirho(variable_chirho)
                            {
                                pending_chirho
                                    .push(OrdinaryTermTaskChirho::VisitChirho(image_chirho, false));
                            } else {
                                values_chirho.push(term_chirho.clone());
                            }
                        }
                        TyChirho::ConChirho(name_chirho)
                        | TyChirho::ForallVarChirho(name_chirho) => {
                            charge_chirho(name_chirho.len(), name_chirho.len())?;
                            values_chirho.push(term_chirho.clone());
                        }
                        TyChirho::AppChirho(left_chirho, right_chirho)
                        | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                            charge_chirho(3, 3)?;
                            pending_chirho.push(OrdinaryTermTaskChirho::FinishChirho(term_chirho));
                            pending_chirho.push(OrdinaryTermTaskChirho::VisitChirho(
                                right_chirho,
                                active_chirho,
                            ));
                            pending_chirho.push(OrdinaryTermTaskChirho::VisitChirho(
                                left_chirho,
                                active_chirho,
                            ));
                        }
                        TyChirho::ListChirho(element_chirho) => {
                            charge_chirho(2, 2)?;
                            pending_chirho.push(OrdinaryTermTaskChirho::FinishChirho(term_chirho));
                            pending_chirho.push(OrdinaryTermTaskChirho::VisitChirho(
                                element_chirho,
                                active_chirho,
                            ));
                        }
                        TyChirho::TupleChirho(elements_chirho) => {
                            charge_chirho(
                                elements_chirho.len().checked_add(1)?,
                                elements_chirho.len().checked_add(1)?,
                            )?;
                            pending_chirho.push(OrdinaryTermTaskChirho::FinishChirho(term_chirho));
                            pending_chirho.extend(elements_chirho.iter().rev().map(
                                |term_chirho| {
                                    OrdinaryTermTaskChirho::VisitChirho(term_chirho, active_chirho)
                                },
                            ));
                        }
                        TyChirho::QualChirho {
                            context_chirho,
                            body_chirho,
                        } => {
                            charge_chirho(
                                context_chirho.len().checked_add(2)?,
                                context_chirho.len().checked_add(2)?,
                            )?;
                            pending_chirho.push(OrdinaryTermTaskChirho::FinishChirho(term_chirho));
                            pending_chirho.push(OrdinaryTermTaskChirho::VisitChirho(
                                body_chirho,
                                active_chirho,
                            ));
                            pending_chirho.extend(context_chirho.iter().rev().map(|term_chirho| {
                                OrdinaryTermTaskChirho::VisitChirho(term_chirho, active_chirho)
                            }));
                        }
                        TyChirho::ForallChirho { .. } | TyChirho::RequiredForallChirho { .. } => {
                            let copied_chirho = if active_chirho {
                                self.apply_freshened_bounded_chirho(
                                    term_chirho,
                                    next_chirho,
                                    charge_chirho,
                                )?
                            } else {
                                Self::empty_chirho()
                                    .apply_bounded_chirho(term_chirho, charge_chirho)?
                            };
                            values_chirho.push(copied_chirho);
                        }
                    }
                }
                OrdinaryTermTaskChirho::FinishChirho(term_chirho) => match term_chirho {
                    TyChirho::AppChirho(..) | TyChirho::FunChirho(..) => {
                        let right_chirho = Box::new(values_chirho.pop()?);
                        let left_chirho = Box::new(values_chirho.pop()?);
                        values_chirho.push(match term_chirho {
                            TyChirho::FunChirho(_, _, multiplicity_chirho) => {
                                TyChirho::FunChirho(left_chirho, right_chirho, *multiplicity_chirho)
                            }
                            _ => TyChirho::AppChirho(left_chirho, right_chirho),
                        });
                    }
                    TyChirho::ListChirho(_) => {
                        let element_chirho = values_chirho.pop()?;
                        values_chirho.push(TyChirho::ListChirho(Box::new(element_chirho)));
                    }
                    TyChirho::TupleChirho(original_chirho) => {
                        let length_chirho = original_chirho.len();
                        let elements_chirho = values_chirho
                            .split_off(values_chirho.len().checked_sub(length_chirho)?);
                        values_chirho.push(TyChirho::TupleChirho(elements_chirho));
                    }
                    TyChirho::QualChirho {
                        context_chirho: original_chirho,
                        ..
                    } => {
                        let length_chirho = original_chirho.len();
                        let body_chirho = values_chirho.pop()?;
                        let context_chirho = values_chirho
                            .split_off(values_chirho.len().checked_sub(length_chirho)?);
                        values_chirho.push(TyChirho::QualChirho {
                            context_chirho,
                            body_chirho: Box::new(body_chirho),
                        });
                    }
                    _ => return None,
                },
            }
        }
        (values_chirho.len() == 1)
            .then(|| values_chirho.pop())
            .flatten()
    }
}
