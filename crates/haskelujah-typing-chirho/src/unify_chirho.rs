// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Unification
//!
//! Robinson unification for `TyChirho`. Produces a substitution that makes two
//! types identical, or an error explaining why they cannot be unified.

use haskelujah_span_chirho::SpanChirho;

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};

/// An error from unification.
#[derive(Debug, Clone, PartialEq)]
pub enum UnifyErrorChirho {
    /// Two types are structurally incompatible.
    MismatchChirho {
        expected_chirho: TyChirho,
        actual_chirho: TyChirho,
        span_chirho: SpanChirho,
    },
    /// Occurs check failure: a type variable would need to be infinite.
    OccursCheckChirho {
        var_chirho: TyVarChirho,
        ty_chirho: TyChirho,
        span_chirho: SpanChirho,
    },
    /// Tuple arity mismatch.
    TupleArityChirho {
        expected_chirho: usize,
        actual_chirho: usize,
        span_chirho: SpanChirho,
    },
}

impl UnifyErrorChirho {
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::MismatchChirho { span_chirho, .. }
            | Self::OccursCheckChirho { span_chirho, .. }
            | Self::TupleArityChirho { span_chirho, .. } => *span_chirho,
        }
    }
}

/// Unify two types, returning a substitution that makes them equal.
pub fn unify_chirho(
    ty1_chirho: &TyChirho,
    ty2_chirho: &TyChirho,
    span_chirho: SpanChirho,
) -> Result<SubstChirho, UnifyErrorChirho> {
    match (ty1_chirho, ty2_chirho) {
        // Two identical type constructors
        (TyChirho::ConChirho(a_chirho), TyChirho::ConChirho(b_chirho)) if a_chirho == b_chirho => {
            Ok(SubstChirho::empty_chirho())
        }

        // Numeric widening: Int unifies with Double (implicit fromInteger)
        (TyChirho::ConChirho(a_chirho), TyChirho::ConChirho(b_chirho))
            if (a_chirho == "Int" && b_chirho == "Double")
                || (a_chirho == "Double" && b_chirho == "Int") =>
        {
            Ok(SubstChirho::empty_chirho())
        }

        // Bind a unification variable
        (TyChirho::VarChirho(v_chirho), ty_chirho) => bind_var_chirho(*v_chirho, ty_chirho, span_chirho),
        (ty_chirho, TyChirho::VarChirho(v_chirho)) => bind_var_chirho(*v_chirho, ty_chirho, span_chirho),

        // Function types
        (TyChirho::FunChirho(a1_chirho, b1_chirho, _), TyChirho::FunChirho(a2_chirho, b2_chirho, _)) => {
            let s1_chirho = unify_chirho(a1_chirho, a2_chirho, span_chirho)?;
            let b1_sub_chirho = s1_chirho.apply_ty_chirho(b1_chirho);
            let b2_sub_chirho = s1_chirho.apply_ty_chirho(b2_chirho);
            let s2_chirho = unify_chirho(&b1_sub_chirho, &b2_sub_chirho, span_chirho)?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }

        // Type application
        (TyChirho::AppChirho(f1_chirho, a1_chirho), TyChirho::AppChirho(f2_chirho, a2_chirho)) => {
            let s1_chirho = unify_chirho(f1_chirho, f2_chirho, span_chirho)?;
            let a1_sub_chirho = s1_chirho.apply_ty_chirho(a1_chirho);
            let a2_sub_chirho = s1_chirho.apply_ty_chirho(a2_chirho);
            let s2_chirho = unify_chirho(&a1_sub_chirho, &a2_sub_chirho, span_chirho)?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }

        // Tuple types (must have same arity)
        (TyChirho::TupleChirho(elems1_chirho), TyChirho::TupleChirho(elems2_chirho)) => {
            if elems1_chirho.len() != elems2_chirho.len() {
                return Err(UnifyErrorChirho::TupleArityChirho {
                    expected_chirho: elems1_chirho.len(),
                    actual_chirho: elems2_chirho.len(),
                    span_chirho,
                });
            }
            let mut subst_chirho = SubstChirho::empty_chirho();
            for (e1_chirho, e2_chirho) in elems1_chirho.iter().zip(elems2_chirho.iter()) {
                let e1_sub_chirho = subst_chirho.apply_ty_chirho(e1_chirho);
                let e2_sub_chirho = subst_chirho.apply_ty_chirho(e2_chirho);
                let s_chirho = unify_chirho(&e1_sub_chirho, &e2_sub_chirho, span_chirho)?;
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);
            }
            Ok(subst_chirho)
        }

        // List types
        (TyChirho::ListChirho(a_chirho), TyChirho::ListChirho(b_chirho)) => {
            unify_chirho(a_chirho, b_chirho, span_chirho)
        }

        // List normalization: App(Con("[]"), a) ≡ List(a)
        // The list type constructor `[]` applied to a type `a` must unify with `[a]`.
        (TyChirho::AppChirho(f_chirho, a1_chirho), TyChirho::ListChirho(a2_chirho))
            if matches!(f_chirho.as_ref(), TyChirho::ConChirho(n) if n == "[]") =>
        {
            unify_chirho(a1_chirho, a2_chirho, span_chirho)
        }
        (TyChirho::ListChirho(a1_chirho), TyChirho::AppChirho(f_chirho, a2_chirho))
            if matches!(f_chirho.as_ref(), TyChirho::ConChirho(n) if n == "[]") =>
        {
            unify_chirho(a1_chirho, a2_chirho, span_chirho)
        }

        // General List ↔ App unification: treat List(a) as App(Con("[]"), a)
        // so that `m a` can unify with `[a]` by binding `m := []`.
        (TyChirho::AppChirho(f_chirho, a1_chirho), TyChirho::ListChirho(a2_chirho)) => {
            let list_con_chirho = TyChirho::ConChirho("[]".to_string());
            let s1_chirho = unify_chirho(f_chirho, &list_con_chirho, span_chirho)?;
            let a1_sub_chirho = s1_chirho.apply_ty_chirho(a1_chirho);
            let a2_sub_chirho = s1_chirho.apply_ty_chirho(a2_chirho);
            let s2_chirho = unify_chirho(&a1_sub_chirho, &a2_sub_chirho, span_chirho)?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }
        (TyChirho::ListChirho(a1_chirho), TyChirho::AppChirho(f_chirho, a2_chirho)) => {
            let list_con_chirho = TyChirho::ConChirho("[]".to_string());
            let s1_chirho = unify_chirho(f_chirho, &list_con_chirho, span_chirho)?;
            let a1_sub_chirho = s1_chirho.apply_ty_chirho(a1_chirho);
            let a2_sub_chirho = s1_chirho.apply_ty_chirho(a2_chirho);
            let s2_chirho = unify_chirho(&a1_sub_chirho, &a2_sub_chirho, span_chirho)?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }

        // Tuple normalization: App(App(Con("(,)"), a), b) ≡ Tuple([a, b])
        // and higher arities: App(...App(Con("(,,)"), a)..., c) ≡ Tuple([a, b, c])
        (TyChirho::AppChirho(..), TyChirho::TupleChirho(elems_chirho)) => {
            if let Some(app_elems_chirho) = collect_tuple_app_chirho(ty1_chirho) {
                if app_elems_chirho.len() == elems_chirho.len() {
                    let mut subst_chirho = SubstChirho::empty_chirho();
                    for (e1_chirho, e2_chirho) in app_elems_chirho.iter().zip(elems_chirho.iter()) {
                        let e1_sub_chirho = subst_chirho.apply_ty_chirho(e1_chirho);
                        let e2_sub_chirho = subst_chirho.apply_ty_chirho(e2_chirho);
                        let s_chirho = unify_chirho(&e1_sub_chirho, &e2_sub_chirho, span_chirho)?;
                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    }
                    return Ok(subst_chirho);
                }
            }
            // General case: convert Tuple to App form and try again
            let tuple_as_app_chirho = tuple_to_app_chirho(elems_chirho);
            unify_chirho(ty1_chirho, &tuple_as_app_chirho, span_chirho)
        }
        (TyChirho::TupleChirho(elems_chirho), TyChirho::AppChirho(..)) => {
            if let Some(app_elems_chirho) = collect_tuple_app_chirho(ty2_chirho) {
                if app_elems_chirho.len() == elems_chirho.len() {
                    let mut subst_chirho = SubstChirho::empty_chirho();
                    for (e1_chirho, e2_chirho) in elems_chirho.iter().zip(app_elems_chirho.iter()) {
                        let e1_sub_chirho = subst_chirho.apply_ty_chirho(e1_chirho);
                        let e2_sub_chirho = subst_chirho.apply_ty_chirho(e2_chirho);
                        let s_chirho = unify_chirho(&e1_sub_chirho, &e2_sub_chirho, span_chirho)?;
                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    }
                    return Ok(subst_chirho);
                }
            }
            // General case: convert Tuple to App form and try again
            let tuple_as_app_chirho = tuple_to_app_chirho(elems_chirho);
            unify_chirho(&tuple_as_app_chirho, ty2_chirho, span_chirho)
        }

        // ForallVar: two identical forall-bound variables
        (TyChirho::ForallVarChirho(a_chirho), TyChirho::ForallVarChirho(b_chirho))
            if a_chirho == b_chirho =>
        {
            Ok(SubstChirho::empty_chirho())
        }

        // ForallChirho: alpha-rename bound vars and unify bodies
        (
            TyChirho::ForallChirho { vars_chirho: v1_chirho, body_chirho: b1_chirho },
            TyChirho::ForallChirho { vars_chirho: v2_chirho, body_chirho: b2_chirho },
        ) if v1_chirho.len() == v2_chirho.len() => {
            // Alpha-rename: substitute v2's bound vars with v1's in b2
            let mut rename_chirho = SubstChirho::empty_chirho();
            for (v1_item_chirho, v2_item_chirho) in v1_chirho.iter().zip(v2_chirho.iter()) {
                rename_chirho.insert_chirho(
                    *v2_item_chirho,
                    TyChirho::VarChirho(*v1_item_chirho),
                );
            }
            let b2_renamed_chirho = rename_chirho.apply_ty_chirho(b2_chirho);
            unify_chirho(b1_chirho, &b2_renamed_chirho, span_chirho)
        }

        // ForallChirho vs concrete type: strip the forall wrapper and unify the body.
        // The bound vars become free TyVarChirho and participate in unification normally.
        // This is consistent with GHC 9.0+ SimpleSubsumption.
        (TyChirho::ForallChirho { body_chirho, .. }, other_chirho)
            if !matches!(other_chirho, TyChirho::VarChirho(_)) =>
        {
            unify_chirho(body_chirho, other_chirho, span_chirho)
        }
        (other_chirho, TyChirho::ForallChirho { body_chirho, .. })
            if !matches!(other_chirho, TyChirho::VarChirho(_)) =>
        {
            unify_chirho(other_chirho, body_chirho, span_chirho)
        }

        // Function ↔ App unification: treat Fun(a, b) as App(App(Con("->"), a), b)
        // so that `f a b` can unify with `a -> b` by binding `f := (->)`.
        (TyChirho::AppChirho(_, _), TyChirho::FunChirho(a_chirho, b_chirho, _)) => {
            let fun_as_app_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("->".to_string())),
                    Box::new((**a_chirho).clone()),
                )),
                Box::new((**b_chirho).clone()),
            );
            unify_chirho(ty1_chirho, &fun_as_app_chirho, span_chirho)
        }
        (TyChirho::FunChirho(a_chirho, b_chirho, _), TyChirho::AppChirho(_, _)) => {
            let fun_as_app_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("->".to_string())),
                    Box::new((**a_chirho).clone()),
                )),
                Box::new((**b_chirho).clone()),
            );
            unify_chirho(&fun_as_app_chirho, ty2_chirho, span_chirho)
        }

        // Everything else is a mismatch
        _ => Err(UnifyErrorChirho::MismatchChirho {
            expected_chirho: ty1_chirho.clone(),
            actual_chirho: ty2_chirho.clone(),
            span_chirho,
        }),
    }
}

/// Convert a Tuple([a, b]) to App(App(Con("(,)"), a), b).
/// For arity N, uses the appropriate tuple constructor "(,,...,)".
fn tuple_to_app_chirho(elems_chirho: &[TyChirho]) -> TyChirho {
    let arity_chirho = elems_chirho.len();
    let con_name_chirho = if arity_chirho == 0 {
        "()".to_string()
    } else {
        format!("({})", ",".repeat(arity_chirho - 1))
    };
    let mut result_chirho = TyChirho::ConChirho(con_name_chirho);
    for elem_chirho in elems_chirho {
        result_chirho = TyChirho::AppChirho(
            Box::new(result_chirho),
            Box::new(elem_chirho.clone()),
        );
    }
    result_chirho
}

/// Check if a type is a fully-applied tuple constructor, e.g.
/// `App(App(Con("(,)"), a), b)` → `Some([a, b])`.
/// Returns `None` if the type is not a tuple constructor application.
fn collect_tuple_app_chirho(ty_chirho: &TyChirho) -> Option<Vec<TyChirho>> {
    /// Walk the spine of applications to find the tuple constructor head.
    fn go_chirho(ty_chirho: &TyChirho, args_chirho: &mut Vec<TyChirho>) -> Option<usize> {
        match ty_chirho {
            TyChirho::ConChirho(name_chirho) => {
                // Check if name is a tuple constructor: "(,)", "(,,)", "(,,,)", etc.
                let n_chirho = name_chirho.as_str();
                if n_chirho.starts_with('(') && n_chirho.ends_with(')') {
                    let inner_chirho = &n_chirho[1..n_chirho.len() - 1];
                    if !inner_chirho.is_empty() && inner_chirho.chars().all(|c| c == ',') {
                        return Some(inner_chirho.len() + 1); // n commas = n+1 elements
                    }
                }
                None
            }
            TyChirho::AppChirho(f_chirho, a_chirho) => {
                args_chirho.push(a_chirho.as_ref().clone());
                go_chirho(f_chirho, args_chirho)
            }
            _ => None,
        }
    }

    let mut args_chirho = Vec::new();
    let arity_chirho = go_chirho(ty_chirho, &mut args_chirho)?;
    // Args were collected in reverse order (outermost first)
    args_chirho.reverse();
    if args_chirho.len() == arity_chirho {
        Some(args_chirho)
    } else {
        None // Partially applied tuple constructor — don't normalize
    }
}

/// Subsumption check: is `actual` at least as polymorphic as `expected`?
///
/// Used for higher-rank type checking in function application: when the
/// expected argument type is `forall a. ...`, the actual argument must be
/// polymorphic enough to satisfy it.
///
/// Rules:
/// - If expected is ForallChirho: instantiate expected's vars with fresh vars,
///   check actual ≤ instantiated_expected. This is sound for the common rank-2
///   case where both sides have matching forall structure.
/// - If actual is ForallChirho: instantiate actual's vars with fresh vars,
///   then unify with expected.
/// - Otherwise: plain unification.
pub fn subsume_chirho(
    actual_chirho: &TyChirho,
    expected_chirho: &TyChirho,
    next_var_chirho: &mut u32,
    span_chirho: SpanChirho,
) -> Result<SubstChirho, UnifyErrorChirho> {
    match (actual_chirho, expected_chirho) {
        // Both ForallChirho: alpha-rename and unify bodies (same as unify)
        (
            TyChirho::ForallChirho { vars_chirho: va_chirho, body_chirho: ba_chirho },
            TyChirho::ForallChirho { vars_chirho: ve_chirho, body_chirho: be_chirho },
        ) if va_chirho.len() == ve_chirho.len() => {
            let mut rename_chirho = SubstChirho::empty_chirho();
            for (va_item_chirho, ve_item_chirho) in va_chirho.iter().zip(ve_chirho.iter()) {
                rename_chirho.insert_chirho(
                    *ve_item_chirho,
                    TyChirho::VarChirho(*va_item_chirho),
                );
            }
            let be_renamed_chirho = rename_chirho.apply_ty_chirho(be_chirho);
            unify_chirho(ba_chirho, &be_renamed_chirho, span_chirho)
        }

        // Actual is ForallChirho: instantiate and check against expected
        (TyChirho::ForallChirho { vars_chirho, body_chirho }, _) => {
            let mut inst_chirho = SubstChirho::empty_chirho();
            for v_chirho in vars_chirho {
                let fresh_chirho = TyVarChirho(*next_var_chirho);
                *next_var_chirho += 1;
                inst_chirho.insert_chirho(*v_chirho, TyChirho::VarChirho(fresh_chirho));
            }
            let inst_actual_chirho = inst_chirho.apply_ty_chirho(body_chirho);
            subsume_chirho(&inst_actual_chirho, expected_chirho, next_var_chirho, span_chirho)
        }

        // Expected is ForallChirho: instantiate and check actual against body
        (_, TyChirho::ForallChirho { vars_chirho, body_chirho }) => {
            let mut inst_chirho = SubstChirho::empty_chirho();
            for v_chirho in vars_chirho {
                let fresh_chirho = TyVarChirho(*next_var_chirho);
                *next_var_chirho += 1;
                inst_chirho.insert_chirho(*v_chirho, TyChirho::VarChirho(fresh_chirho));
            }
            let inst_expected_chirho = inst_chirho.apply_ty_chirho(body_chirho);
            subsume_chirho(actual_chirho, &inst_expected_chirho, next_var_chirho, span_chirho)
        }

        // Neither has ForallChirho: plain unification
        _ => unify_chirho(actual_chirho, expected_chirho, span_chirho),
    }
}

/// Attempt to bind a unification variable to a type.
/// Performs the occurs check to prevent infinite types.
fn bind_var_chirho(
    var_chirho: TyVarChirho,
    ty_chirho: &TyChirho,
    span_chirho: SpanChirho,
) -> Result<SubstChirho, UnifyErrorChirho> {
    // Variable equals itself: no binding needed
    if *ty_chirho == TyChirho::VarChirho(var_chirho) {
        return Ok(SubstChirho::empty_chirho());
    }

    // Occurs check: var must not appear free in ty
    if ty_chirho.free_vars_chirho().contains(&var_chirho) {
        return Err(UnifyErrorChirho::OccursCheckChirho {
            var_chirho,
            ty_chirho: ty_chirho.clone(),
            span_chirho,
        });
    }

    Ok(SubstChirho::singleton_chirho(var_chirho, ty_chirho.clone()))
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn unify_identical_con_chirho() {
        let s_chirho = unify_chirho(
            &TyChirho::int_chirho(),
            &TyChirho::int_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
        assert!(s_chirho.is_empty_chirho());
    }

    #[test]
    fn unify_var_with_con_chirho() {
        let v_chirho = TyVarChirho(0);
        let s_chirho = unify_chirho(
            &TyChirho::VarChirho(v_chirho),
            &TyChirho::int_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
        assert_eq!(
            s_chirho.apply_ty_chirho(&TyChirho::VarChirho(v_chirho)),
            TyChirho::int_chirho()
        );
    }

    #[test]
    fn unify_two_vars_chirho() {
        let a_chirho = TyVarChirho(0);
        let b_chirho = TyVarChirho(1);
        let s_chirho = unify_chirho(
            &TyChirho::VarChirho(a_chirho),
            &TyChirho::VarChirho(b_chirho),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
        // After applying, both should map to the same type
        let ta_chirho = s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho));
        let tb_chirho = s_chirho.apply_ty_chirho(&TyChirho::VarChirho(b_chirho));
        assert_eq!(ta_chirho, tb_chirho);
    }

    #[test]
    fn unify_fun_types_chirho() {
        let a_chirho = TyVarChirho(0);
        let ty1_chirho =
            TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::int_chirho());
        let ty2_chirho = TyChirho::fun_chirho(TyChirho::bool_chirho(), TyChirho::int_chirho());

        let s_chirho =
            unify_chirho(&ty1_chirho, &ty2_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
        assert_eq!(
            s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho)),
            TyChirho::bool_chirho()
        );
    }

    #[test]
    fn unify_mismatch_chirho() {
        let result_chirho = unify_chirho(
            &TyChirho::int_chirho(),
            &TyChirho::bool_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(matches!(
            result_chirho,
            Err(UnifyErrorChirho::MismatchChirho { .. })
        ));
    }

    #[test]
    fn occurs_check_prevents_infinite_type_chirho() {
        let a_chirho = TyVarChirho(0);
        // t0 ~ [t0] would require t0 = [[[...infinite...]]]
        let result_chirho = unify_chirho(
            &TyChirho::VarChirho(a_chirho),
            &TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(matches!(
            result_chirho,
            Err(UnifyErrorChirho::OccursCheckChirho { .. })
        ));
    }

    #[test]
    fn unify_tuples_same_arity_chirho() {
        let a_chirho = TyVarChirho(0);
        let t1_chirho =
            TyChirho::TupleChirho(vec![TyChirho::VarChirho(a_chirho), TyChirho::int_chirho()]);
        let t2_chirho =
            TyChirho::TupleChirho(vec![TyChirho::bool_chirho(), TyChirho::int_chirho()]);

        let s_chirho =
            unify_chirho(&t1_chirho, &t2_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
        assert_eq!(
            s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho)),
            TyChirho::bool_chirho()
        );
    }

    #[test]
    fn unify_tuples_different_arity_fails_chirho() {
        let t1_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho()]);
        let t2_chirho =
            TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]);

        let result_chirho =
            unify_chirho(&t1_chirho, &t2_chirho, SpanChirho::DUMMY_CHIRHO);
        assert!(matches!(
            result_chirho,
            Err(UnifyErrorChirho::TupleArityChirho { .. })
        ));
    }

    #[test]
    fn unify_list_types_chirho() {
        let a_chirho = TyVarChirho(0);
        let s_chirho = unify_chirho(
            &TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
            &TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
        assert_eq!(
            s_chirho.apply_ty_chirho(&TyChirho::VarChirho(a_chirho)),
            TyChirho::int_chirho()
        );
    }

    #[test]
    fn unify_forall_same_structure_chirho() {
        // forall t0. t0 -> t0 ~ forall t1. t1 -> t1 (alpha-equivalent)
        let fa_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(0)],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(0)),
            )),
        };
        let fb_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(1)],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(1)),
                TyChirho::VarChirho(TyVarChirho(1)),
            )),
        };
        let s_chirho = unify_chirho(&fa_chirho, &fb_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
        assert!(s_chirho.is_empty_chirho());
    }

    #[test]
    fn unify_forall_with_concrete_fun_chirho() {
        // forall t0. t0 -> t0 ~ Int -> Int (strip forall, unify body)
        let forall_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(0)],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(0)),
            )),
        };
        let concrete_chirho = TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho());
        let s_chirho = unify_chirho(&forall_chirho, &concrete_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
        assert_eq!(
            s_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(0))),
            TyChirho::int_chirho()
        );
    }

    #[test]
    fn unify_forall_with_var_chirho() {
        // forall t0. t0 -> t0 ~ t5 (binds t5 to the forall type)
        let forall_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(0)],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(0)),
            )),
        };
        let var_chirho = TyChirho::VarChirho(TyVarChirho(5));
        let s_chirho = unify_chirho(&var_chirho, &forall_chirho, SpanChirho::DUMMY_CHIRHO).unwrap();
        assert_eq!(
            s_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(5))),
            forall_chirho
        );
    }

    #[test]
    fn subsume_forall_with_forall_chirho() {
        // forall t0. t0 -> t0 subsumes forall t1. t1 -> t1
        let fa_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(0)],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(0)),
            )),
        };
        let fb_chirho = TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(1)],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(1)),
                TyChirho::VarChirho(TyVarChirho(1)),
            )),
        };
        let mut next_var_chirho = 10u32;
        let result_chirho = subsume_chirho(
            &fa_chirho,
            &fb_chirho,
            &mut next_var_chirho,
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(result_chirho.is_ok());
    }
}
