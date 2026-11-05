// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Unification
//!
//! Robinson unification for `TyChirho`. Produces a substitution that makes two
//! types identical, or an error explaining why they cannot be unified.

use haskelujah_ast_chirho::promotion_chirho::is_promoted_key_chirho;
use haskelujah_span_chirho::SpanChirho;

/// Strip qualified import alias prefix from a type constructor name.
/// e.g. "N.Operator" -> "Operator", "Data.Map.Map" -> "Map"
fn strip_qualifier_chirho(name_chirho: &str) -> &str {
    name_chirho.rsplit('.').next().unwrap_or(name_chirho)
}

fn has_qualifier_chirho(name_chirho: &str) -> bool {
    name_chirho.contains('.')
}

fn same_tycon_name_chirho(left_name_chirho: &str, right_name_chirho: &str) -> bool {
    // A promoted-constructor key is opaque (row 686): only the whole key identifies it.
    if is_promoted_key_chirho(left_name_chirho) || is_promoted_key_chirho(right_name_chirho) {
        return left_name_chirho == right_name_chirho;
    }
    left_name_chirho == right_name_chirho
        || (has_qualifier_chirho(left_name_chirho)
            && has_qualifier_chirho(right_name_chirho)
            && strip_qualifier_chirho(left_name_chirho)
                == strip_qualifier_chirho(right_name_chirho))
}

mod quantifiers_chirho;
pub(crate) use quantifiers_chirho::align_forall_prefix_chirho;

use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};

/// An error from unification.
#[derive(Debug, Clone, PartialEq)]
pub enum UnifyErrorChirho {
    /// A bounded ordinary monotype check could not inspect its complete input.
    ResourceExhaustedChirho { span_chirho: SpanChirho },
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
            Self::ResourceExhaustedChirho { span_chirho }
            | Self::MismatchChirho { span_chirho, .. }
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
    unify_with_kind_policy_chirho(
        ty1_chirho,
        ty2_chirho,
        span_chirho,
        false,
        0,
        &mut 10_000_000,
    )
}

/// Checked kind terms share the unifier, with nominal constructors and no
/// numeric widening or invisible-forall stripping. This does not infer kinds.
pub(crate) fn unify_kind_terms_chirho(
    left_chirho: &TyChirho,
    right_chirho: &TyChirho,
    span_chirho: SpanChirho,
) -> Result<SubstChirho, UnifyErrorChirho> {
    let mut work_chirho = 10_000_000usize;
    let mut output_chirho = 16_384usize;
    let mut charge_chirho = |work_amount_chirho: usize, output_amount_chirho: usize| {
        work_chirho = work_chirho.checked_sub(work_amount_chirho)?;
        output_chirho = output_chirho.checked_sub(output_amount_chirho)?;
        Some(())
    };
    let left_chirho = SubstChirho::empty_chirho()
        .apply_bounded_chirho(left_chirho, &mut charge_chirho)
        .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
    let right_chirho = SubstChirho::empty_chirho()
        .apply_bounded_chirho(right_chirho, &mut charge_chirho)
        .ok_or(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho })?;
    unify_with_kind_policy_chirho(
        &left_chirho,
        &right_chirho,
        span_chirho,
        true,
        0,
        &mut work_chirho,
    )
}

fn unify_with_kind_policy_chirho(
    ty1_chirho: &TyChirho,
    ty2_chirho: &TyChirho,
    span_chirho: SpanChirho,
    strict_kind_chirho: bool,
    depth_chirho: usize,
    work_left_chirho: &mut usize,
) -> Result<SubstChirho, UnifyErrorChirho> {
    if depth_chirho > 128 || *work_left_chirho == 0 {
        return Err(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho });
    }
    *work_left_chirho -= 1;
    match (ty1_chirho, ty2_chirho) {
        (
            TyChirho::QualChirho {
                context_chirho: left_context_chirho,
                body_chirho: left_body_chirho,
            },
            TyChirho::QualChirho {
                context_chirho: right_context_chirho,
                body_chirho: right_body_chirho,
            },
        ) if left_context_chirho.len() == right_context_chirho.len() => {
            let mut substitution_chirho = SubstChirho::empty_chirho();
            for (left_chirho, right_chirho) in left_context_chirho
                .iter()
                .zip(right_context_chirho)
                .chain(std::iter::once((
                    left_body_chirho.as_ref(),
                    right_body_chirho.as_ref(),
                )))
            {
                let left_chirho = substitution_chirho.apply_ty_chirho(left_chirho);
                let right_chirho = substitution_chirho.apply_ty_chirho(right_chirho);
                substitution_chirho = unify_with_kind_policy_chirho(
                    &left_chirho,
                    &right_chirho,
                    span_chirho,
                    strict_kind_chirho,
                    depth_chirho + 1,
                    work_left_chirho,
                )?
                .compose_chirho(&substitution_chirho);
            }
            Ok(substitution_chirho)
        }
        // Two identical type constructors. Accept different qualified spellings
        // of the same imported type constructor, but do not conflate a local
        // bare name like `Result` with an imported qualified name like `P.Result`.
        (TyChirho::ConChirho(a_chirho), TyChirho::ConChirho(b_chirho))
            if if strict_kind_chirho {
                a_chirho == b_chirho
            } else {
                same_tycon_name_chirho(a_chirho, b_chirho)
            } =>
        {
            Ok(SubstChirho::empty_chirho())
        }

        // Numeric widening: Int unifies with Double (implicit fromInteger)
        (TyChirho::ConChirho(a_chirho), TyChirho::ConChirho(b_chirho))
            if !strict_kind_chirho
                && ((a_chirho == "Int" && b_chirho == "Double")
                    || (a_chirho == "Double" && b_chirho == "Int")) =>
        {
            Ok(SubstChirho::empty_chirho())
        }

        // Bind a unification variable
        (TyChirho::VarChirho(v_chirho), ty_chirho) => {
            bind_var_with_kind_policy_chirho(*v_chirho, ty_chirho, span_chirho, strict_kind_chirho)
        }
        (ty_chirho, TyChirho::VarChirho(v_chirho)) => {
            bind_var_with_kind_policy_chirho(*v_chirho, ty_chirho, span_chirho, strict_kind_chirho)
        }

        // Function types
        (
            TyChirho::FunChirho(a1_chirho, b1_chirho, _),
            TyChirho::FunChirho(a2_chirho, b2_chirho, _),
        ) => {
            let s1_chirho = unify_with_kind_policy_chirho(
                a1_chirho,
                a2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            let b1_sub_chirho = s1_chirho.apply_ty_chirho(b1_chirho);
            let b2_sub_chirho = s1_chirho.apply_ty_chirho(b2_chirho);
            let s2_chirho = unify_with_kind_policy_chirho(
                &b1_sub_chirho,
                &b2_sub_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }

        // Type application
        (TyChirho::AppChirho(f1_chirho, a1_chirho), TyChirho::AppChirho(f2_chirho, a2_chirho)) => {
            let s1_chirho = unify_with_kind_policy_chirho(
                f1_chirho,
                f2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            let a1_sub_chirho = s1_chirho.apply_ty_chirho(a1_chirho);
            let a2_sub_chirho = s1_chirho.apply_ty_chirho(a2_chirho);
            let s2_chirho = unify_with_kind_policy_chirho(
                &a1_sub_chirho,
                &a2_sub_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
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
                let s_chirho = unify_with_kind_policy_chirho(
                    &e1_sub_chirho,
                    &e2_sub_chirho,
                    span_chirho,
                    strict_kind_chirho,
                    depth_chirho + 1,
                    work_left_chirho,
                )?;
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);
            }
            Ok(subst_chirho)
        }

        // List types
        (TyChirho::ListChirho(a_chirho), TyChirho::ListChirho(b_chirho)) => {
            unify_with_kind_policy_chirho(
                a_chirho,
                b_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
        }

        // List normalization: App(Con("[]"), a) ≡ List(a)
        // The list type constructor `[]` applied to a type `a` must unify with `[a]`.
        (TyChirho::AppChirho(f_chirho, a1_chirho), TyChirho::ListChirho(a2_chirho)) if matches!(f_chirho.as_ref(), TyChirho::ConChirho(n) if n == "[]") => {
            unify_with_kind_policy_chirho(
                a1_chirho,
                a2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
        }
        (TyChirho::ListChirho(a1_chirho), TyChirho::AppChirho(f_chirho, a2_chirho)) if matches!(f_chirho.as_ref(), TyChirho::ConChirho(n) if n == "[]") => {
            unify_with_kind_policy_chirho(
                a1_chirho,
                a2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
        }

        // General List ↔ App unification: treat List(a) as App(Con("[]"), a)
        // so that `m a` can unify with `[a]` by binding `m := []`.
        (TyChirho::AppChirho(f_chirho, a1_chirho), TyChirho::ListChirho(a2_chirho)) => {
            let list_con_chirho = TyChirho::ConChirho("[]".to_string());
            let s1_chirho = unify_with_kind_policy_chirho(
                f_chirho,
                &list_con_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            let a1_sub_chirho = s1_chirho.apply_ty_chirho(a1_chirho);
            let a2_sub_chirho = s1_chirho.apply_ty_chirho(a2_chirho);
            let s2_chirho = unify_with_kind_policy_chirho(
                &a1_sub_chirho,
                &a2_sub_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }
        (TyChirho::ListChirho(a1_chirho), TyChirho::AppChirho(f_chirho, a2_chirho)) => {
            let list_con_chirho = TyChirho::ConChirho("[]".to_string());
            let s1_chirho = unify_with_kind_policy_chirho(
                f_chirho,
                &list_con_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            let a1_sub_chirho = s1_chirho.apply_ty_chirho(a1_chirho);
            let a2_sub_chirho = s1_chirho.apply_ty_chirho(a2_chirho);
            let s2_chirho = unify_with_kind_policy_chirho(
                &a1_sub_chirho,
                &a2_sub_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            Ok(s2_chirho.compose_chirho(&s1_chirho))
        }

        // Tuple normalization: App(App(Con("(,)"), a), b) ≡ Tuple([a, b])
        // and higher arities: App(...App(Con("(,,)"), a)..., c) ≡ Tuple([a, b, c])
        (TyChirho::AppChirho(..), TyChirho::TupleChirho(elems_chirho)) => {
            if let Some(app_elems_chirho) = collect_tuple_app_chirho(ty1_chirho)
                && app_elems_chirho.len() == elems_chirho.len()
            {
                let mut subst_chirho = SubstChirho::empty_chirho();
                for (e1_chirho, e2_chirho) in app_elems_chirho.iter().zip(elems_chirho.iter()) {
                    let e1_sub_chirho = subst_chirho.apply_ty_chirho(e1_chirho);
                    let e2_sub_chirho = subst_chirho.apply_ty_chirho(e2_chirho);
                    let s_chirho = unify_with_kind_policy_chirho(
                        &e1_sub_chirho,
                        &e2_sub_chirho,
                        span_chirho,
                        strict_kind_chirho,
                        depth_chirho + 1,
                        work_left_chirho,
                    )?;
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                }
                return Ok(subst_chirho);
            }
            // General case: convert Tuple to App form and try again
            let tuple_as_app_chirho = tuple_to_app_chirho(elems_chirho);
            unify_with_kind_policy_chirho(
                ty1_chirho,
                &tuple_as_app_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
        }
        (TyChirho::TupleChirho(elems_chirho), TyChirho::AppChirho(..)) => {
            if let Some(app_elems_chirho) = collect_tuple_app_chirho(ty2_chirho)
                && app_elems_chirho.len() == elems_chirho.len()
            {
                let mut subst_chirho = SubstChirho::empty_chirho();
                for (e1_chirho, e2_chirho) in elems_chirho.iter().zip(app_elems_chirho.iter()) {
                    let e1_sub_chirho = subst_chirho.apply_ty_chirho(e1_chirho);
                    let e2_sub_chirho = subst_chirho.apply_ty_chirho(e2_chirho);
                    let s_chirho = unify_with_kind_policy_chirho(
                        &e1_sub_chirho,
                        &e2_sub_chirho,
                        span_chirho,
                        strict_kind_chirho,
                        depth_chirho + 1,
                        work_left_chirho,
                    )?;
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                }
                return Ok(subst_chirho);
            }
            // General case: convert Tuple to App form and try again
            let tuple_as_app_chirho = tuple_to_app_chirho(elems_chirho);
            unify_with_kind_policy_chirho(
                &tuple_as_app_chirho,
                ty2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
        }

        // ForallVar: two identical forall-bound variables
        (TyChirho::ForallVarChirho(a_chirho), TyChirho::ForallVarChirho(b_chirho))
            if a_chirho == b_chirho =>
        {
            Ok(SubstChirho::empty_chirho())
        }

        // ForallChirho: alpha-rename bound vars and unify bodies
        (
            TyChirho::ForallChirho {
                vars_chirho: v1_chirho,
                body_chirho: b1_chirho,
            },
            TyChirho::ForallChirho {
                vars_chirho: v2_chirho,
                body_chirho: b2_chirho,
            },
        ) if v1_chirho.len() == v2_chirho.len() => {
            // Alpha-rename: substitute v2's bound vars with v1's in b2
            let (rename_chirho, kinds_chirho) =
                align_forall_prefix_chirho(v1_chirho, v2_chirho, span_chirho)?;
            let b1_chirho = kinds_chirho.apply_ty_chirho(b1_chirho);
            let b2_chirho = kinds_chirho.apply_ty_chirho(&rename_chirho.apply_ty_chirho(b2_chirho));
            let body_solution_chirho = unify_with_kind_policy_chirho(
                &b1_chirho,
                &b2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            if strict_kind_chirho {
                quantifiers_chirho::protect_kind_body_chirho(
                    v1_chirho,
                    &body_solution_chirho,
                    &b1_chirho,
                    &b2_chirho,
                    span_chirho,
                )?;
            }
            Ok(body_solution_chirho.compose_chirho(&kinds_chirho))
        }

        // Required foralls unify only with required foralls. A regular term
        // application must consume this wrapper before its body is available.
        (
            TyChirho::RequiredForallChirho {
                vars_chirho: v1_chirho,
                body_chirho: b1_chirho,
            },
            TyChirho::RequiredForallChirho {
                vars_chirho: v2_chirho,
                body_chirho: b2_chirho,
            },
        ) if v1_chirho.len() == v2_chirho.len() => {
            let (rename_chirho, kinds_chirho) =
                align_forall_prefix_chirho(v1_chirho, v2_chirho, span_chirho)?;
            let b1_chirho = kinds_chirho.apply_ty_chirho(b1_chirho);
            let b2_chirho = kinds_chirho.apply_ty_chirho(&rename_chirho.apply_ty_chirho(b2_chirho));
            let body_solution_chirho = unify_with_kind_policy_chirho(
                &b1_chirho,
                &b2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )?;
            if strict_kind_chirho {
                quantifiers_chirho::protect_kind_body_chirho(
                    v1_chirho,
                    &body_solution_chirho,
                    &b1_chirho,
                    &b2_chirho,
                    span_chirho,
                )?;
            }
            Ok(body_solution_chirho.compose_chirho(&kinds_chirho))
        }

        // ForallChirho vs concrete type: strip the forall wrapper and unify the body.
        // The bound vars become free TyVarChirho and participate in unification normally.
        // This is consistent with GHC 9.0+ SimpleSubsumption.
        (TyChirho::ForallChirho { body_chirho, .. }, other_chirho)
            if !strict_kind_chirho && !matches!(other_chirho, TyChirho::VarChirho(_)) =>
        {
            unify_with_kind_policy_chirho(
                body_chirho,
                other_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
        }
        (other_chirho, TyChirho::ForallChirho { body_chirho, .. })
            if !strict_kind_chirho && !matches!(other_chirho, TyChirho::VarChirho(_)) =>
        {
            unify_with_kind_policy_chirho(
                other_chirho,
                body_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
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
            unify_with_kind_policy_chirho(
                ty1_chirho,
                &fun_as_app_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
        }
        (TyChirho::FunChirho(a_chirho, b_chirho, _), TyChirho::AppChirho(_, _)) => {
            let fun_as_app_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("->".to_string())),
                    Box::new((**a_chirho).clone()),
                )),
                Box::new((**b_chirho).clone()),
            );
            unify_with_kind_policy_chirho(
                &fun_as_app_chirho,
                ty2_chirho,
                span_chirho,
                strict_kind_chirho,
                depth_chirho + 1,
                work_left_chirho,
            )
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
        result_chirho = TyChirho::AppChirho(Box::new(result_chirho), Box::new(elem_chirho.clone()));
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
            TyChirho::ForallChirho {
                vars_chirho: va_chirho,
                body_chirho: ba_chirho,
            },
            TyChirho::ForallChirho {
                vars_chirho: ve_chirho,
                body_chirho: be_chirho,
            },
        ) if va_chirho.len() == ve_chirho.len() => {
            let (rename_chirho, kinds_chirho) =
                align_forall_prefix_chirho(va_chirho, ve_chirho, span_chirho)?;
            let actual_chirho = kinds_chirho.apply_ty_chirho(ba_chirho);
            let expected_chirho =
                kinds_chirho.apply_ty_chirho(&rename_chirho.apply_ty_chirho(be_chirho));
            Ok(unify_chirho(&actual_chirho, &expected_chirho, span_chirho)?
                .compose_chirho(&kinds_chirho))
        }

        (
            TyChirho::RequiredForallChirho {
                vars_chirho: va_chirho,
                body_chirho: ba_chirho,
            },
            TyChirho::RequiredForallChirho {
                vars_chirho: ve_chirho,
                body_chirho: be_chirho,
            },
        ) if va_chirho.len() == ve_chirho.len() => {
            let (rename_chirho, kinds_chirho) =
                align_forall_prefix_chirho(va_chirho, ve_chirho, span_chirho)?;
            let actual_chirho = kinds_chirho.apply_ty_chirho(ba_chirho);
            let expected_chirho =
                kinds_chirho.apply_ty_chirho(&rename_chirho.apply_ty_chirho(be_chirho));
            Ok(unify_chirho(&actual_chirho, &expected_chirho, span_chirho)?
                .compose_chirho(&kinds_chirho))
        }

        // Actual is ForallChirho: instantiate and check against expected
        (
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            },
            _,
        ) => {
            let mut inst_chirho = SubstChirho::empty_chirho();
            for v_chirho in vars_chirho {
                let fresh_chirho = TyVarChirho(*next_var_chirho);
                *next_var_chirho += 1;
                inst_chirho.insert_chirho(*v_chirho, TyChirho::VarChirho(fresh_chirho));
            }
            let inst_actual_chirho = inst_chirho.apply_ty_chirho(body_chirho);
            subsume_chirho(
                &inst_actual_chirho,
                expected_chirho,
                next_var_chirho,
                span_chirho,
            )
        }

        // Expected is ForallChirho: instantiate and check actual against body
        (
            _,
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            },
        ) => {
            let mut inst_chirho = SubstChirho::empty_chirho();
            for v_chirho in vars_chirho {
                let fresh_chirho = TyVarChirho(*next_var_chirho);
                *next_var_chirho += 1;
                inst_chirho.insert_chirho(*v_chirho, TyChirho::VarChirho(fresh_chirho));
            }
            let inst_expected_chirho = inst_chirho.apply_ty_chirho(body_chirho);
            subsume_chirho(
                actual_chirho,
                &inst_expected_chirho,
                next_var_chirho,
                span_chirho,
            )
        }

        // Neither has ForallChirho: plain unification
        _ => unify_chirho(actual_chirho, expected_chirho, span_chirho),
    }
}

/// Attempt to bind a unification variable to a type.
/// Performs the occurs check to prevent infinite types.
fn bind_var_with_kind_policy_chirho(
    var_chirho: TyVarChirho,
    ty_chirho: &TyChirho,
    span_chirho: SpanChirho,
    strict_kind_chirho: bool,
) -> Result<SubstChirho, UnifyErrorChirho> {
    // Variable equals itself: no binding needed
    if *ty_chirho == TyChirho::VarChirho(var_chirho) {
        return Ok(SubstChirho::empty_chirho());
    }

    // Ordinary metas are monotypes. Only the separate, short-lived Quick Look
    // substitution can choose a polytype, before ordinary argument checking.
    // Inspect before cloning so a wide input cannot bypass this bound.
    let mut pending_chirho = vec![(ty_chirho, 0usize)];
    let mut work_chirho = 16_384usize;
    let mut polymorphic_chirho = false;
    while let Some((term_chirho, depth_chirho)) = pending_chirho.pop() {
        if work_chirho == 0 || depth_chirho > 128 {
            return Err(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho });
        }
        work_chirho -= 1;
        let width_chirho = match term_chirho {
            TyChirho::TupleChirho(elements_chirho) => elements_chirho.len(),
            TyChirho::ForallChirho { vars_chirho, .. }
            | TyChirho::RequiredForallChirho { vars_chirho, .. } => vars_chirho.len(),
            TyChirho::QualChirho { context_chirho, .. } => context_chirho.len(),
            TyChirho::ConChirho(name_chirho) | TyChirho::ForallVarChirho(name_chirho) => {
                name_chirho.len()
            }
            _ => 0,
        };
        if width_chirho > work_chirho.saturating_sub(pending_chirho.len()) {
            return Err(UnifyErrorChirho::ResourceExhaustedChirho { span_chirho });
        }
        work_chirho -= width_chirho;
        match term_chirho {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                for record_chirho in vars_chirho.records_chirho() {
                    if let Some(kind_chirho) = &record_chirho.kind_chirho {
                        pending_chirho.push((kind_chirho.as_ref(), depth_chirho + 1));
                    }
                }
                polymorphic_chirho = true;
                pending_chirho.push((body_chirho, depth_chirho + 1));
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                polymorphic_chirho = true;
                pending_chirho.push((body_chirho, depth_chirho + 1));
                pending_chirho.extend(
                    context_chirho
                        .iter()
                        .map(|constraint_chirho| (constraint_chirho, depth_chirho + 1)),
                );
            }
            TyChirho::AppChirho(left_chirho, right_chirho)
            | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
                pending_chirho.extend([
                    (left_chirho.as_ref(), depth_chirho + 1),
                    (right_chirho.as_ref(), depth_chirho + 1),
                ]);
            }
            TyChirho::ListChirho(element_chirho) => {
                pending_chirho.push((element_chirho, depth_chirho + 1))
            }
            TyChirho::TupleChirho(elements_chirho) => pending_chirho.extend(
                elements_chirho
                    .iter()
                    .map(|element_chirho| (element_chirho, depth_chirho + 1)),
            ),
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => {}
        }
    }
    if polymorphic_chirho && !strict_kind_chirho {
        return Err(UnifyErrorChirho::MismatchChirho {
            expected_chirho: TyChirho::VarChirho(var_chirho),
            actual_chirho: ty_chirho.clone(),
            span_chirho,
        });
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
mod tests_chirho;
