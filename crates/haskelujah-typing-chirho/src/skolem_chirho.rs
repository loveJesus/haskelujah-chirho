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

/// Rebuild a type, replacing every skolem `ForallVarChirho` for which
/// `rewrite_chirho` returns `Some` with the returned type. Non-skolem
/// `ForallVarChirho` names (synonym pattern variables) are never offered.
pub fn rewrite_skolems_chirho(
    ty_chirho: &TyChirho,
    rewrite_chirho: &mut impl FnMut(&str) -> Option<TyChirho>,
) -> TyChirho {
    match ty_chirho {
        TyChirho::ForallVarChirho(name_chirho) if is_skolem_name_chirho(name_chirho) => {
            rewrite_chirho(name_chirho).unwrap_or_else(|| ty_chirho.clone())
        }
        TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {
            ty_chirho.clone()
        }
        TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
            Box::new(rewrite_skolems_chirho(fun_chirho, rewrite_chirho)),
            Box::new(rewrite_skolems_chirho(arg_chirho, rewrite_chirho)),
        ),
        TyChirho::KindAppChirho(fun_chirho, arg_chirho) => TyChirho::KindAppChirho(
            Box::new(rewrite_skolems_chirho(fun_chirho, rewrite_chirho)),
            Box::new(rewrite_skolems_chirho(arg_chirho, rewrite_chirho)),
        ),
        TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(rewrite_skolems_chirho(arg_chirho, rewrite_chirho)),
            Box::new(rewrite_skolems_chirho(result_chirho, rewrite_chirho)),
            *mult_chirho,
        ),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|elem_chirho| rewrite_skolems_chirho(elem_chirho, rewrite_chirho))
                .collect(),
        ),
        TyChirho::ListChirho(inner_chirho) => TyChirho::ListChirho(Box::new(
            rewrite_skolems_chirho(inner_chirho, rewrite_chirho),
        )),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(rewrite_skolems_chirho(body_chirho, rewrite_chirho)),
        },
        TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::RequiredForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(rewrite_skolems_chirho(body_chirho, rewrite_chirho)),
        },
    }
}

/// Whether a type mentions any skolem.
pub fn contains_skolem_chirho(ty_chirho: &TyChirho) -> bool {
    let mut found_chirho = false;
    rewrite_skolems_chirho(ty_chirho, &mut |_name_chirho| {
        found_chirho = true;
        None
    });
    found_chirho
}

/// Replace every skolem in `ty_chirho` by a unification variable so that an
/// ordinary unifier can *propose* bindings for the skolems. Newly opened
/// skolems are added to `opened_chirho`; skolems already present reuse their
/// variable so that the same skolem opens to the same variable across calls.
pub fn open_skolems_chirho(
    ty_chirho: &TyChirho,
    opened_chirho: &mut HashMap<String, TyVarChirho>,
    fresh_chirho: &mut impl FnMut() -> TyVarChirho,
) -> TyChirho {
    rewrite_skolems_chirho(ty_chirho, &mut |name_chirho| {
        let var_chirho = *opened_chirho
            .entry(name_chirho.to_string())
            .or_insert_with(&mut *fresh_chirho);
        Some(TyChirho::VarChirho(var_chirho))
    })
}

/// Undo `open_skolems_chirho` on a type: variables that stand for opened
/// skolems become those skolems again; everything else is untouched.
pub fn close_skolems_chirho(
    ty_chirho: &TyChirho,
    opened_chirho: &HashMap<String, TyVarChirho>,
) -> TyChirho {
    let back_chirho: HashMap<TyVarChirho, &str> = opened_chirho
        .iter()
        .map(|(name_chirho, var_chirho)| (*var_chirho, name_chirho.as_str()))
        .collect();
    close_skolems_with_chirho(ty_chirho, &back_chirho)
}

fn close_skolems_with_chirho(
    ty_chirho: &TyChirho,
    back_chirho: &HashMap<TyVarChirho, &str>,
) -> TyChirho {
    match ty_chirho {
        TyChirho::VarChirho(var_chirho) => match back_chirho.get(var_chirho) {
            Some(name_chirho) => TyChirho::ForallVarChirho((*name_chirho).to_string()),
            None => ty_chirho.clone(),
        },
        TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => ty_chirho.clone(),
        TyChirho::AppChirho(fun_chirho, arg_chirho) => TyChirho::AppChirho(
            Box::new(close_skolems_with_chirho(fun_chirho, back_chirho)),
            Box::new(close_skolems_with_chirho(arg_chirho, back_chirho)),
        ),
        TyChirho::KindAppChirho(fun_chirho, arg_chirho) => TyChirho::KindAppChirho(
            Box::new(close_skolems_with_chirho(fun_chirho, back_chirho)),
            Box::new(close_skolems_with_chirho(arg_chirho, back_chirho)),
        ),
        TyChirho::FunChirho(arg_chirho, result_chirho, mult_chirho) => TyChirho::FunChirho(
            Box::new(close_skolems_with_chirho(arg_chirho, back_chirho)),
            Box::new(close_skolems_with_chirho(result_chirho, back_chirho)),
            *mult_chirho,
        ),
        TyChirho::TupleChirho(elems_chirho) => TyChirho::TupleChirho(
            elems_chirho
                .iter()
                .map(|elem_chirho| close_skolems_with_chirho(elem_chirho, back_chirho))
                .collect(),
        ),
        TyChirho::ListChirho(inner_chirho) => TyChirho::ListChirho(Box::new(
            close_skolems_with_chirho(inner_chirho, back_chirho),
        )),
        TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::ForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(close_skolems_with_chirho(body_chirho, back_chirho)),
        },
        TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } => TyChirho::RequiredForallChirho {
            vars_chirho: vars_chirho.clone(),
            body_chirho: Box::new(close_skolems_with_chirho(body_chirho, back_chirho)),
        },
    }
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
) -> RefinementOutcomeChirho {
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
        let closed_chirho = close_skolems_chirho(&image_chirho, opened_chirho);
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
        outcome_chirho
            .residual_subst_chirho
            .insert_chirho(*var_chirho, close_skolems_chirho(ty_chirho, opened_chirho));
    }
    outcome_chirho
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
            open_skolems_chirho(&ty_chirho, &mut opened_chirho, &mut fresh_chirho);
        assert!(!contains_skolem_chirho(&opened_ty_chirho));
        assert_eq!(opened_chirho.len(), 2);
        assert_eq!(
            close_skolems_chirho(&opened_ty_chirho, &opened_chirho),
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
        let outcome_chirho = split_refinement_unifier_chirho(&unifier_chirho, &opened_chirho);
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
        let outcome_chirho = split_refinement_unifier_chirho(&unifier_chirho, &opened_chirho);
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
}
