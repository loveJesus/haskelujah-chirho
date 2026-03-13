// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Unification
//!
//! Robinson unification for `TyChirho`. Produces a substitution that makes two
//! types identical, or an error explaining why they cannot be unified.

use rhasky_span_chirho::SpanChirho;

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

        // Bind a unification variable
        (TyChirho::VarChirho(v_chirho), ty_chirho) => bind_var_chirho(*v_chirho, ty_chirho, span_chirho),
        (ty_chirho, TyChirho::VarChirho(v_chirho)) => bind_var_chirho(*v_chirho, ty_chirho, span_chirho),

        // Function types
        (TyChirho::FunChirho(a1_chirho, b1_chirho), TyChirho::FunChirho(a2_chirho, b2_chirho)) => {
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

        // ForallVar: two identical forall-bound variables
        (TyChirho::ForallVarChirho(a_chirho), TyChirho::ForallVarChirho(b_chirho))
            if a_chirho == b_chirho =>
        {
            Ok(SubstChirho::empty_chirho())
        }

        // Everything else is a mismatch
        _ => Err(UnifyErrorChirho::MismatchChirho {
            expected_chirho: ty1_chirho.clone(),
            actual_chirho: ty2_chirho.clone(),
            span_chirho,
        }),
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
}
