// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! # Rigid type variables (skolems) and local type refinements
//!
//! A *skolem* is a type constant standing for a signature-bound type variable
//! while the binding's body is checked: `f :: a -> a` is checked with `a`
//! rigid, so the body can use `a` but can never decide that `a` is `Int`.
//! Skolems are `TyChirho::ForallVarChirho` values (which unify only with
//! themselves) whose name carries a `%N` suffix so that two `a`s from
//! different signatures stay distinct; `Display` hides the suffix.
//!
//! A *refinement* is a local given equality `skolem ~ type` learned from a
//! GADT constructor pattern: matching `IntE :: Int -> Expr Int` against a
//! scrutinee of type `Expr a` teaches `a ~ Int` for that alternative only.
//! Refinements live in the innermost type-environment scope
//! (`TyEnvChirho`) and are applied during type normalization, so they vanish
//! when the pattern's scope ends.
//!
//! workflow: language-features-chirho/rigid-type-variables-chirho

use std::collections::HashMap;

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};

/// Separates a skolem's source name from its uniqueness ordinal. `%` cannot
/// occur inside a Haskell identifier, so a skolem name never collides with a
/// type variable written in source.
pub const SKOLEM_MARKER_CHIRHO: char = '%';

/// Build the unique internal name of a skolem from its source name.
pub fn skolem_name_chirho(base_chirho: &str, ordinal_chirho: u32) -> String {
    format!("{base_chirho}{SKOLEM_MARKER_CHIRHO}{ordinal_chirho}")
}

/// Whether a `ForallVarChirho` name denotes a skolem (as opposed to a
/// synonym / type-family pattern variable, which carries a plain name).
pub fn is_skolem_name_chirho(name_chirho: &str) -> bool {
    name_chirho.contains(SKOLEM_MARKER_CHIRHO)
}

/// The user-facing spelling of a skolem: the source name without the ordinal.
pub fn skolem_display_name_chirho(name_chirho: &str) -> &str {
    name_chirho
        .split(SKOLEM_MARKER_CHIRHO)
        .next()
        .unwrap_or(name_chirho)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkolemResourceExhaustedChirho;

struct SkolemBudgetChirho {
    work_chirho: usize,
    output_chirho: usize,
}
impl SkolemBudgetChirho {
    fn new_chirho() -> Self {
        Self {
            work_chirho: 10_000_000,
            output_chirho: 16_384,
        }
    }
    fn charge_chirho(
        &mut self,
        work_chirho: usize,
        output_chirho: usize,
    ) -> Result<(), SkolemResourceExhaustedChirho> {
        self.work_chirho = self
            .work_chirho
            .checked_sub(work_chirho)
            .ok_or(SkolemResourceExhaustedChirho)?;
        self.output_chirho = self
            .output_chirho
            .checked_sub(output_chirho)
            .ok_or(SkolemResourceExhaustedChirho)?;
        Ok(())
    }
}

fn visit_terms_chirho(
    term_chirho: &TyChirho,
    budget_chirho: &mut SkolemBudgetChirho,
    action_chirho: &mut impl FnMut(
        &TyChirho,
        &mut SkolemBudgetChirho,
    ) -> Result<(), SkolemResourceExhaustedChirho>,
) -> Result<(), SkolemResourceExhaustedChirho> {
    // This borrowed walk does not recurse. Ordinary predicates may exceed the
    // proof-metadata depth limit without losing their ordinary meaning.
    let mut pending_chirho = vec![term_chirho];
    while let Some(term_chirho) = pending_chirho.pop() {
        budget_chirho.charge_chirho(1, 0)?;
        action_chirho(term_chirho, budget_chirho)?;
        match term_chirho {
            TyChirho::AppChirho(left_chirho, right_chirho)
            | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                budget_chirho.charge_chirho(2, 2)?;
                pending_chirho.extend([left_chirho.as_ref(), right_chirho.as_ref()]);
            }
            TyChirho::ListChirho(element_chirho) => {
                budget_chirho.charge_chirho(1, 1)?;
                pending_chirho.push(element_chirho);
            }
            TyChirho::TupleChirho(elements_chirho) => {
                budget_chirho.charge_chirho(elements_chirho.len(), elements_chirho.len())?;
                pending_chirho.extend(elements_chirho);
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                budget_chirho.charge_chirho(
                    context_chirho.len().saturating_add(1),
                    context_chirho.len().saturating_add(1),
                )?;
                pending_chirho.push(body_chirho);
                pending_chirho.extend(context_chirho);
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                budget_chirho.charge_chirho(
                    vars_chirho.len().saturating_add(1),
                    vars_chirho.len().saturating_add(1),
                )?;
                pending_chirho.push(body_chirho);
                for record_chirho in vars_chirho.records_chirho() {
                    if let Some(kind_chirho) = &record_chirho.kind_chirho {
                        pending_chirho.push(kind_chirho.as_ref());
                    }
                }
            }
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
        }
    }
    Ok(())
}

fn rewrite_with_budget_chirho(
    ty_chirho: &TyChirho,
    rewrite_chirho: &mut impl FnMut(&str) -> Option<TyChirho>,
    budget_chirho: &mut SkolemBudgetChirho,
    depth_chirho: usize,
) -> Result<TyChirho, SkolemResourceExhaustedChirho> {
    budget_chirho.charge_chirho(1, 1)?;
    if depth_chirho > 128 {
        return Err(SkolemResourceExhaustedChirho);
    }
    Ok(match ty_chirho {
        TyChirho::ForallVarChirho(name_chirho) if is_skolem_name_chirho(name_chirho) => {
            budget_chirho.charge_chirho(name_chirho.len(), 0)?;
            if let Some(image_chirho) = rewrite_chirho(name_chirho) {
                SubstChirho::empty_chirho()
                    .apply_bounded_chirho(&image_chirho, &mut |work_chirho, output_chirho| {
                        budget_chirho.charge_chirho(work_chirho, output_chirho).ok()
                    })
                    .ok_or(SkolemResourceExhaustedChirho)?
            } else {
                budget_chirho.charge_chirho(name_chirho.len(), name_chirho.len())?;
                ty_chirho.clone()
            }
        }
        TyChirho::VarChirho(_) => ty_chirho.clone(),
        TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
            budget_chirho.charge_chirho(name_chirho.len(), name_chirho.len())?;
            ty_chirho.clone()
        }
        TyChirho::AppChirho(left_chirho, right_chirho) => TyChirho::AppChirho(
            Box::new(rewrite_with_budget_chirho(
                left_chirho,
                rewrite_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            Box::new(rewrite_with_budget_chirho(
                right_chirho,
                rewrite_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
        ),
        TyChirho::FunChirho(left_chirho, right_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(rewrite_with_budget_chirho(
                left_chirho,
                rewrite_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            Box::new(rewrite_with_budget_chirho(
                right_chirho,
                rewrite_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?),
            *mult_chirho,
        ),
        TyChirho::ListChirho(element_chirho) => {
            TyChirho::ListChirho(Box::new(rewrite_with_budget_chirho(
                element_chirho,
                rewrite_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?))
        }
        TyChirho::TupleChirho(elements_chirho) => {
            budget_chirho.charge_chirho(elements_chirho.len(), elements_chirho.len())?;
            TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|term_chirho| {
                        rewrite_with_budget_chirho(
                            term_chirho,
                            rewrite_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            )
        }
        TyChirho::QualChirho {
            context_chirho,
            body_chirho,
        } => {
            budget_chirho.charge_chirho(context_chirho.len(), context_chirho.len())?;
            TyChirho::QualChirho {
                context_chirho: context_chirho
                    .iter()
                    .map(|term_chirho| {
                        rewrite_with_budget_chirho(
                            term_chirho,
                            rewrite_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )
                    })
                    .collect::<Result<_, _>>()?,
                body_chirho: Box::new(rewrite_with_budget_chirho(
                    body_chirho,
                    rewrite_chirho,
                    budget_chirho,
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
            budget_chirho.charge_chirho(vars_chirho.len(), vars_chirho.len())?;
            let mut records_chirho = Vec::new();
            for record_chirho in vars_chirho.records_chirho() {
                let mut mapped_chirho = record_chirho.clone();
                mapped_chirho.kind_chirho = record_chirho
                    .kind_chirho
                    .as_ref()
                    .map(|kind_chirho| {
                        rewrite_with_budget_chirho(
                            kind_chirho,
                            rewrite_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )
                        .map(std::sync::Arc::new)
                    })
                    .transpose()?;
                records_chirho.push(mapped_chirho);
            }
            let vars_chirho =
                crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(records_chirho);
            let body_chirho = Box::new(rewrite_with_budget_chirho(
                body_chirho,
                rewrite_chirho,
                budget_chirho,
                depth_chirho + 1,
            )?);
            if matches!(ty_chirho, TyChirho::RequiredForallChirho { .. }) {
                TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                }
            } else {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
            }
        }
    })
}

/// Rewrites every rigid occurrence, including ones present only in binder kinds.
pub fn rewrite_skolems_chirho(
    ty_chirho: &TyChirho,
    rewrite_chirho: &mut impl FnMut(&str) -> Option<TyChirho>,
) -> Result<TyChirho, SkolemResourceExhaustedChirho> {
    rewrite_with_budget_chirho(
        ty_chirho,
        rewrite_chirho,
        &mut SkolemBudgetChirho::new_chirho(),
        0,
    )
}

pub fn contains_skolem_chirho(ty_chirho: &TyChirho) -> Result<bool, SkolemResourceExhaustedChirho> {
    let mut found_chirho = false;
    visit_terms_chirho(
        ty_chirho,
        &mut SkolemBudgetChirho::new_chirho(),
        &mut |term_chirho, budget_chirho| {
            if let TyChirho::ForallVarChirho(name_chirho) = term_chirho {
                budget_chirho.charge_chirho(name_chirho.len(), 0)?;
                found_chirho |= is_skolem_name_chirho(name_chirho);
            }
            Ok(())
        },
    )?;
    Ok(found_chirho)
}

/// Opens only actual rigid names. All semantic input IDs are reserved first,
/// including lexical binders/kinds, so the fresh image cannot be captured.
pub fn open_skolems_chirho(
    ty_chirho: &TyChirho,
    opened_chirho: &mut HashMap<String, TyVarChirho>,
    fresh_chirho: &mut impl FnMut() -> TyVarChirho,
) -> Result<TyChirho, SkolemResourceExhaustedChirho> {
    let mut budget_chirho = SkolemBudgetChirho::new_chirho();
    let mut reserved_chirho = std::collections::HashSet::new();
    visit_terms_chirho(
        ty_chirho,
        &mut budget_chirho,
        &mut |term_chirho, budget_chirho| {
            match term_chirho {
                TyChirho::VarChirho(variable_chirho) => {
                    budget_chirho.charge_chirho(1, 1)?;
                    reserved_chirho.insert(*variable_chirho);
                }
                TyChirho::ForallChirho { vars_chirho, .. }
                | TyChirho::RequiredForallChirho { vars_chirho, .. } => {
                    budget_chirho.charge_chirho(vars_chirho.len(), vars_chirho.len())?;
                    reserved_chirho.extend(vars_chirho.iter().copied());
                }
                _ => {}
            }
            Ok(())
        },
    )?;
    let mut failed_chirho = false;
    let mut attempts_chirho = 16_384usize;
    let result_chirho = rewrite_with_budget_chirho(
        ty_chirho,
        &mut |name_chirho| {
            if let Some(variable_chirho) = opened_chirho.get(name_chirho) {
                return Some(TyChirho::VarChirho(*variable_chirho));
            }
            while attempts_chirho > 0 {
                attempts_chirho -= 1;
                let variable_chirho = fresh_chirho();
                if reserved_chirho.insert(variable_chirho) {
                    opened_chirho.insert(name_chirho.to_owned(), variable_chirho);
                    return Some(TyChirho::VarChirho(variable_chirho));
                }
            }
            failed_chirho = true;
            None
        },
        &mut budget_chirho,
        0,
    )?;
    if failed_chirho {
        Err(SkolemResourceExhaustedChirho)
    } else {
        Ok(result_chirho)
    }
}

fn close_with_budget_chirho(
    ty_chirho: &TyChirho,
    opened_chirho: &HashMap<String, TyVarChirho>,
    budget_chirho: &mut SkolemBudgetChirho,
) -> Result<TyChirho, SkolemResourceExhaustedChirho> {
    budget_chirho.charge_chirho(opened_chirho.len(), opened_chirho.len())?;
    let mut substitution_chirho = SubstChirho::empty_chirho();
    for (name_chirho, variable_chirho) in opened_chirho {
        budget_chirho.charge_chirho(name_chirho.len(), name_chirho.len())?;
        substitution_chirho.insert_chirho(
            *variable_chirho,
            TyChirho::ForallVarChirho(name_chirho.clone()),
        );
    }
    substitution_chirho
        .apply_bounded_chirho(ty_chirho, &mut |work_chirho, output_chirho| {
            budget_chirho.charge_chirho(work_chirho, output_chirho).ok()
        })
        .ok_or(SkolemResourceExhaustedChirho)
}

/// Closing respects every lexical prefix, including dependent binder kinds.
pub fn close_skolems_chirho(
    ty_chirho: &TyChirho,
    opened_chirho: &HashMap<String, TyVarChirho>,
) -> Result<TyChirho, SkolemResourceExhaustedChirho> {
    close_with_budget_chirho(
        ty_chirho,
        opened_chirho,
        &mut SkolemBudgetChirho::new_chirho(),
    )
}

/// What a refinement unification learned: the local equalities on skolems,
/// and the bindings of ordinary unification variables that hold regardless
/// of scope and must be composed into the running substitution.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RefinementOutcomeChirho {
    pub refinements_chirho: Vec<(String, TyChirho)>,
    pub residual_subst_chirho: SubstChirho,
}

/// Split the unifier of two skolem-opened types back into skolem
/// refinements and ordinary variable bindings.
pub fn split_refinement_unifier_chirho(
    unifier_chirho: &SubstChirho,
    opened_chirho: &HashMap<String, TyVarChirho>,
) -> Result<RefinementOutcomeChirho, SkolemResourceExhaustedChirho> {
    let mut budget_chirho = SkolemBudgetChirho::new_chirho();
    budget_chirho.charge_chirho(
        opened_chirho
            .len()
            .saturating_add(unifier_chirho.len_chirho()),
        opened_chirho.len(),
    )?;
    let mut outcome_chirho = RefinementOutcomeChirho::default();
    // DETERMINISM: refinements are recorded in a Vec, so iterate the opened
    // map in a fixed order rather than hash order.
    let mut opened_sorted_chirho: Vec<(&String, &TyVarChirho)> = opened_chirho.iter().collect();
    opened_sorted_chirho.sort_by_key(|(_name_chirho, var_chirho)| **var_chirho);
    for (name_chirho, var_chirho) in opened_sorted_chirho {
        let image_chirho = unifier_chirho.apply_ty_chirho(&TyChirho::VarChirho(*var_chirho));
        if image_chirho == TyChirho::VarChirho(*var_chirho) {
            continue;
        }
        let closed_chirho =
            close_with_budget_chirho(&image_chirho, opened_chirho, &mut budget_chirho)?;
        if closed_chirho == TyChirho::ForallVarChirho(name_chirho.clone()) {
            continue;
        }
        outcome_chirho
            .refinements_chirho
            .push((name_chirho.clone(), closed_chirho));
    }
    let opened_vars_chirho: Vec<TyVarChirho> = opened_chirho.values().copied().collect();
    for (var_chirho, ty_chirho) in unifier_chirho.iter_chirho() {
        if opened_vars_chirho.contains(var_chirho) {
            continue;
        }
        outcome_chirho.residual_subst_chirho.insert_chirho(
            *var_chirho,
            close_with_budget_chirho(ty_chirho, opened_chirho, &mut budget_chirho)?,
        );
    }
    Ok(outcome_chirho)
}

/// Whether a data constructor's declared type can *refine* the scrutinee it
/// is matched against, i.e. its result type is not the plain
/// `T a b c` with distinct variables of an ordinary constructor. This is
/// exactly the GADT case (`IntE :: Int -> Expr Int`, `Refl :: Equal a a`),
/// including constructors whose equality context was folded into the result.
pub fn constructor_result_refines_chirho(con_ty_chirho: &TyChirho) -> bool {
    let mut result_chirho = con_ty_chirho;
    loop {
        match result_chirho {
            TyChirho::FunChirho(_, next_chirho, _) => result_chirho = next_chirho,
            TyChirho::ForallChirho { body_chirho, .. } => result_chirho = body_chirho,
            _ => break,
        }
    }
    let mut args_chirho: Vec<&TyChirho> = Vec::new();
    let mut head_chirho = result_chirho;
    while let TyChirho::AppChirho(fun_chirho, arg_chirho) = head_chirho {
        args_chirho.push(arg_chirho);
        head_chirho = fun_chirho;
    }
    if !matches!(head_chirho, TyChirho::ConChirho(_)) {
        return false;
    }
    let mut seen_chirho: Vec<TyVarChirho> = Vec::new();
    for arg_chirho in args_chirho {
        match arg_chirho {
            TyChirho::VarChirho(var_chirho) => {
                if seen_chirho.contains(var_chirho) {
                    return true;
                }
                seen_chirho.push(*var_chirho);
            }
            _ => return true,
        }
    }
    false
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_span_chirho::SpanChirho;

    fn skolem_chirho(name_chirho: &str) -> TyChirho {
        TyChirho::ForallVarChirho(skolem_name_chirho(name_chirho, 7))
    }

    #[test]
    fn skolem_names_round_trip_chirho() {
        let name_chirho = skolem_name_chirho("a", 3);
        assert!(is_skolem_name_chirho(&name_chirho));
        assert!(!is_skolem_name_chirho("a"));
        assert_eq!(skolem_display_name_chirho(&name_chirho), "a");
        assert_eq!(skolem_display_name_chirho("plain"), "plain");
    }

    #[test]
    fn skolem_displays_as_its_source_name_chirho() {
        assert_eq!(skolem_chirho("a").to_string(), "a");
        let fun_chirho = TyChirho::fun_chirho(skolem_chirho("a"), TyChirho::int_chirho());
        assert_eq!(fun_chirho.to_string(), "(a -> Int)");
    }

    #[test]
    fn skolem_unifies_only_with_itself_chirho() {
        let a_chirho = skolem_chirho("a");
        let b_chirho = TyChirho::ForallVarChirho(skolem_name_chirho("a", 8));
        assert!(
            crate::unify_chirho::unify_chirho(&a_chirho, &a_chirho, SpanChirho::DUMMY_CHIRHO)
                .is_ok()
        );
        assert!(
            crate::unify_chirho::unify_chirho(&a_chirho, &b_chirho, SpanChirho::DUMMY_CHIRHO)
                .is_err()
        );
        assert!(
            crate::unify_chirho::unify_chirho(
                &a_chirho,
                &TyChirho::int_chirho(),
                SpanChirho::DUMMY_CHIRHO
            )
            .is_err()
        );
        // A unification variable may still be solved TO a skolem.
        let var_chirho = TyChirho::VarChirho(TyVarChirho(0));
        let subst_chirho =
            crate::unify_chirho::unify_chirho(&var_chirho, &a_chirho, SpanChirho::DUMMY_CHIRHO)
                .unwrap();
        assert_eq!(subst_chirho.apply_ty_chirho(&var_chirho), a_chirho);
    }

    #[test]
    fn open_and_close_skolems_round_trip_chirho() {
        let ty_chirho = TyChirho::fun_chirho(
            skolem_chirho("a"),
            TyChirho::ListChirho(Box::new(skolem_chirho("b"))),
        );
        let mut opened_chirho = HashMap::new();
        let mut next_chirho = 100u32;
        let mut fresh_chirho = || {
            next_chirho += 1;
            TyVarChirho(next_chirho)
        };
        let opened_ty_chirho =
            open_skolems_chirho(&ty_chirho, &mut opened_chirho, &mut fresh_chirho)
                .expect("bounded open");
        assert!(!contains_skolem_chirho(&opened_ty_chirho).expect("bounded inspection"));
        assert_eq!(opened_chirho.len(), 2);
        assert_eq!(
            close_skolems_chirho(&opened_ty_chirho, &opened_chirho).expect("bounded close"),
            ty_chirho
        );
    }

    #[test]
    fn refinement_unifier_splits_skolem_equalities_from_var_bindings_chirho() {
        // scrutinee `Expr a` (a skolem) against constructor result `Expr Int`
        // whose instantiation also bound an ordinary variable t1 := Bool.
        let mut opened_chirho = HashMap::new();
        opened_chirho.insert(skolem_name_chirho("a", 7), TyVarChirho(50));
        let mut unifier_chirho = SubstChirho::empty_chirho();
        unifier_chirho.insert_chirho(TyVarChirho(50), TyChirho::int_chirho());
        unifier_chirho.insert_chirho(TyVarChirho(1), TyChirho::bool_chirho());
        let outcome_chirho = split_refinement_unifier_chirho(&unifier_chirho, &opened_chirho)
            .expect("bounded split");
        assert_eq!(
            outcome_chirho.refinements_chirho,
            vec![(skolem_name_chirho("a", 7), TyChirho::int_chirho())]
        );
        assert_eq!(outcome_chirho.residual_subst_chirho.len_chirho(), 1);
        assert_eq!(
            outcome_chirho
                .residual_subst_chirho
                .lookup_chirho(&TyVarChirho(1)),
            Some(&TyChirho::bool_chirho())
        );
    }

    #[test]
    fn refinement_between_two_skolems_is_recorded_once_chirho() {
        // `Refl :: Equal a a` against `Equal a b`: the unifier says v_a := v_b.
        let a_name_chirho = skolem_name_chirho("a", 1);
        let b_name_chirho = skolem_name_chirho("b", 2);
        let mut opened_chirho = HashMap::new();
        opened_chirho.insert(a_name_chirho.clone(), TyVarChirho(50));
        opened_chirho.insert(b_name_chirho.clone(), TyVarChirho(51));
        let mut unifier_chirho = SubstChirho::empty_chirho();
        unifier_chirho.insert_chirho(TyVarChirho(50), TyChirho::VarChirho(TyVarChirho(51)));
        let outcome_chirho = split_refinement_unifier_chirho(&unifier_chirho, &opened_chirho)
            .expect("bounded split");
        assert_eq!(
            outcome_chirho.refinements_chirho,
            vec![(a_name_chirho, TyChirho::ForallVarChirho(b_name_chirho))]
        );
        assert!(outcome_chirho.residual_subst_chirho.is_empty_chirho());
    }

    #[test]
    fn plain_constructor_results_do_not_refine_chirho() {
        let t_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::VarChirho(TyVarChirho(0))),
        );
        let just_chirho = TyChirho::fun_chirho(TyChirho::VarChirho(TyVarChirho(0)), t_chirho);
        assert!(!constructor_result_refines_chirho(&just_chirho));
        assert!(!constructor_result_refines_chirho(&TyChirho::ConChirho(
            "Unit".to_string()
        )));
    }

    #[test]
    fn gadt_constructor_results_refine_chirho() {
        let expr_int_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Expr".to_string())),
            Box::new(TyChirho::int_chirho()),
        );
        let int_e_chirho = TyChirho::fun_chirho(TyChirho::int_chirho(), expr_int_chirho);
        assert!(constructor_result_refines_chirho(&int_e_chirho));
        let equal_a_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Equal".to_string())),
                Box::new(TyChirho::VarChirho(TyVarChirho(0))),
            )),
            Box::new(TyChirho::VarChirho(TyVarChirho(0))),
        );
        assert!(constructor_result_refines_chirho(&equal_a_a_chirho));
    }
    #[test]
    fn kind_only_rigids_open_without_capturing_lexical_ids_chirho() {
        let name_chirho = skolem_name_chirho("kChirho", 7);
        let mut record_chirho =
            crate::ty_chirho::QuantifiedVariableChirho::unknown_chirho(TyVarChirho(0));
        record_chirho.kind_chirho = Some(std::sync::Arc::new(TyChirho::ForallVarChirho(
            name_chirho.clone(),
        )));
        let term_chirho = TyChirho::ForallChirho {
            vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
                record_chirho,
            ]),
            body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(0))),
        };
        assert!(contains_skolem_chirho(&term_chirho).expect("bounded scan"));
        let mut next_chirho = 0;
        let mut opened_chirho = HashMap::new();
        let opened_term_chirho = open_skolems_chirho(&term_chirho, &mut opened_chirho, &mut || {
            let variable_chirho = TyVarChirho(next_chirho);
            next_chirho += 1;
            variable_chirho
        })
        .expect("bounded open avoids input IDs");
        assert_ne!(opened_chirho[&name_chirho], TyVarChirho(0));
        assert_eq!(
            close_skolems_chirho(&opened_term_chirho, &opened_chirho).expect("bounded close"),
            term_chirho
        );
        let protected_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![opened_chirho[&name_chirho]].into(),
            body_chirho: Box::new(TyChirho::VarChirho(opened_chirho[&name_chirho])),
        };
        assert_eq!(
            close_skolems_chirho(&protected_chirho, &opened_chirho).expect("bound IDs stay bound"),
            protected_chirho
        );
    }

    #[test]
    fn borrowed_skolem_scan_preserves_deep_predicates_and_refuses_excess_work_chirho() {
        let mut kind_chirho = TyChirho::int_chirho();
        for _index_chirho in 0..130 {
            kind_chirho = TyChirho::ListChirho(Box::new(kind_chirho));
        }
        let mut record_chirho =
            crate::ty_chirho::QuantifiedVariableChirho::unknown_chirho(TyVarChirho(0));
        record_chirho.kind_chirho = Some(std::sync::Arc::new(kind_chirho));
        let term_chirho = TyChirho::ForallChirho {
            vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
                record_chirho,
            ]),
            body_chirho: Box::new(TyChirho::int_chirho()),
        };
        assert_eq!(contains_skolem_chirho(&term_chirho), Ok(false));
        assert_eq!(
            rewrite_skolems_chirho(&term_chirho, &mut |_| None),
            Err(SkolemResourceExhaustedChirho)
        );
        let wide_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho(); 16_385]);
        assert_eq!(
            contains_skolem_chirho(&wide_chirho),
            Err(SkolemResourceExhaustedChirho)
        );
    }
}
