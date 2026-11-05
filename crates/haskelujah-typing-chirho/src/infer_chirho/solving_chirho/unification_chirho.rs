// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unification and subsumption of normalized types, and their error reports.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{OCCURS_CHECK_CODE_CHIRHO, TUPLE_ARITY_CODE_CHIRHO, TYPE_MISMATCH_CODE_CHIRHO};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{TyChirho, TyVarChirho};
use crate::unify_chirho::{UnifyErrorChirho, unify_chirho};
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    /// Unify two types after normalizing: expand type synonyms and reduce
    /// type families. This ensures that `String` unifies with `[Char]` and
    /// `F T1` unifies with `Char` when `type instance F T1 = Char`.
    /// The recursive cases re-normalize after intermediate substitutions so
    /// reductions like `PrimState (ST s) ~ s` become visible once `m ~ ST s`.
    pub(super) fn unify_ordinary_chirho(
        &self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<SubstChirho, UnifyErrorChirho> {
        let result_chirho = unify_chirho(left_chirho, right_chirho, span_chirho);
        if matches!(
            result_chirho,
            Err(UnifyErrorChirho::ResourceExhaustedChirho { .. })
        ) {
            self.type_normalization_exhausted_chirho.set(true);
        }
        result_chirho
    }

    pub(super) fn unify_normalized_chirho(
        &mut self,
        ty1_chirho: &TyChirho,
        ty2_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<SubstChirho, UnifyErrorChirho> {
        let n1_chirho = self.normalize_ty_chirho(ty1_chirho);
        let n2_chirho = self.normalize_ty_chirho(ty2_chirho);
        match (&n1_chirho, &n2_chirho) {
            (
                TyChirho::ForallChirho {
                    vars_chirho: vars1_chirho,
                    body_chirho: body1_chirho,
                },
                TyChirho::ForallChirho {
                    vars_chirho: vars2_chirho,
                    body_chirho: body2_chirho,
                },
            ) if vars1_chirho.len() == vars2_chirho.len() => {
                let (rename_chirho, kinds_chirho) =
                    crate::unify_chirho::align_forall_prefix_chirho(
                        vars1_chirho,
                        vars2_chirho,
                        span_chirho,
                    )?;
                let body1_chirho = kinds_chirho.apply_ty_chirho(body1_chirho);
                let body2_chirho =
                    kinds_chirho.apply_ty_chirho(&rename_chirho.apply_ty_chirho(body2_chirho));
                return Ok(self
                    .unify_normalized_chirho(&body1_chirho, &body2_chirho, span_chirho)?
                    .compose_chirho(&kinds_chirho));
            }
            (
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
                other_chirho,
            ) if !matches!(other_chirho, TyChirho::VarChirho(_)) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    instantiation_chirho.insert_chirho(*var_chirho, self.fresh_var_chirho());
                }
                let instantiated_body_chirho = instantiation_chirho.apply_ty_chirho(body_chirho);
                return self.unify_normalized_chirho(
                    &instantiated_body_chirho,
                    other_chirho,
                    span_chirho,
                );
            }
            (
                other_chirho,
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
            ) if !matches!(other_chirho, TyChirho::VarChirho(_)) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    instantiation_chirho.insert_chirho(*var_chirho, self.fresh_var_chirho());
                }
                let instantiated_body_chirho = instantiation_chirho.apply_ty_chirho(body_chirho);
                return self.unify_normalized_chirho(
                    other_chirho,
                    &instantiated_body_chirho,
                    span_chirho,
                );
            }
            _ => {}
        }
        // A type-family application stuck on an unsolved variable cannot be
        // compared yet: defer the equality rather than report a mismatch
        // (`n ~ N t` before `t` is known). This must happen BEFORE any
        // structural decomposition — a family application is not injective,
        // so `F t Bool ~ Maybe Int` never implies `F t ~ Maybe`. It is retried
        // when the variable is solved, when givens change, and at the end of
        // the module.
        // workflow: language-features-chirho/stuck-family-equalities-chirho
        if self.defer_stuck_family_equality_chirho(&n1_chirho, &n2_chirho, span_chirho) {
            return Ok(SubstChirho::empty_chirho());
        }
        if let (
            TyChirho::FunChirho(arg1_chirho, result1_chirho, _),
            TyChirho::FunChirho(arg2_chirho, result2_chirho, _),
        ) = (&n1_chirho, &n2_chirho)
        {
            let arg_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(arg1_chirho, arg2_chirho, span_chirho)?;
                let result1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(result1_chirho));
                let result2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(result2_chirho));
                let s2_chirho = self.unify_normalized_chirho(
                    &result1_sub_chirho,
                    &result2_sub_chirho,
                    span_chirho,
                )?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })();
            if let Ok(subst_chirho) = arg_first_result_chirho {
                return Ok(subst_chirho);
            }

            let result_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(result1_chirho, result2_chirho, span_chirho)?;
                let arg1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg1_chirho));
                let arg2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg2_chirho));
                let s2_chirho =
                    self.unify_normalized_chirho(&arg1_sub_chirho, &arg2_sub_chirho, span_chirho)?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })(
            );
            if let Ok(subst_chirho) = result_first_result_chirho {
                return Ok(subst_chirho);
            }
        }
        if let (
            TyChirho::AppChirho(fun1_chirho, arg1_chirho),
            TyChirho::AppChirho(fun2_chirho, arg2_chirho),
        ) = (&n1_chirho, &n2_chirho)
        {
            let fun_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(fun1_chirho, fun2_chirho, span_chirho)?;
                let arg1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg1_chirho));
                let arg2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(arg2_chirho));
                let s2_chirho =
                    self.unify_normalized_chirho(&arg1_sub_chirho, &arg2_sub_chirho, span_chirho)?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })();
            if let Ok(subst_chirho) = fun_first_result_chirho {
                return Ok(subst_chirho);
            }

            let arg_first_result_chirho: Result<SubstChirho, UnifyErrorChirho> = (|| {
                let s1_chirho =
                    self.unify_normalized_chirho(arg1_chirho, arg2_chirho, span_chirho)?;
                let fun1_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(fun1_chirho));
                let fun2_sub_chirho =
                    self.normalize_ty_chirho(&s1_chirho.apply_ty_chirho(fun2_chirho));
                let s2_chirho =
                    self.unify_normalized_chirho(&fun1_sub_chirho, &fun2_sub_chirho, span_chirho)?;
                Ok(s2_chirho.compose_chirho(&s1_chirho))
            })();
            if let Ok(subst_chirho) = arg_first_result_chirho {
                return Ok(subst_chirho);
            }
        }
        if !matches!(n2_chirho, TyChirho::VarChirho(_))
            && let Some(inner1_chirho) = self.saturated_st_inner_ty_chirho(&n1_chirho)
            && let Ok(subst_chirho) =
                self.unify_normalized_chirho(&inner1_chirho, &n2_chirho, span_chirho)
        {
            return Ok(subst_chirho);
        }
        if !matches!(n1_chirho, TyChirho::VarChirho(_))
            && let Some(inner2_chirho) = self.saturated_st_inner_ty_chirho(&n2_chirho)
            && let Ok(subst_chirho) =
                self.unify_normalized_chirho(&n1_chirho, &inner2_chirho, span_chirho)
        {
            return Ok(subst_chirho);
        }
        if let (TyChirho::ListChirho(inner1_chirho), TyChirho::ListChirho(inner2_chirho)) =
            (&n1_chirho, &n2_chirho)
        {
            return self.unify_normalized_chirho(inner1_chirho, inner2_chirho, span_chirho);
        }
        if let (TyChirho::TupleChirho(elems1_chirho), TyChirho::TupleChirho(elems2_chirho)) =
            (&n1_chirho, &n2_chirho)
        {
            if elems1_chirho.len() != elems2_chirho.len() {
                return self.unify_ordinary_chirho(&n1_chirho, &n2_chirho, span_chirho);
            }
            let mut subst_chirho = SubstChirho::empty_chirho();
            for (elem1_chirho, elem2_chirho) in elems1_chirho.iter().zip(elems2_chirho.iter()) {
                let elem1_sub_chirho =
                    self.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(elem1_chirho));
                let elem2_sub_chirho =
                    self.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(elem2_chirho));
                let s_chirho = self.unify_normalized_chirho(
                    &elem1_sub_chirho,
                    &elem2_sub_chirho,
                    span_chirho,
                )?;
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);
            }
            return Ok(subst_chirho);
        }
        self.unify_ordinary_chirho(&n1_chirho, &n2_chirho, span_chirho)
    }

    pub(super) fn subsume_normalized_chirho(
        &mut self,
        actual_chirho: &TyChirho,
        expected_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<SubstChirho, UnifyErrorChirho> {
        match (actual_chirho, expected_chirho) {
            (
                TyChirho::ForallChirho {
                    vars_chirho: actual_vars_chirho,
                    body_chirho: actual_body_chirho,
                },
                TyChirho::ForallChirho {
                    vars_chirho: expected_vars_chirho,
                    body_chirho: expected_body_chirho,
                },
            ) if actual_vars_chirho.len() == expected_vars_chirho.len() => {
                let (rename_chirho, kinds_chirho) =
                    crate::unify_chirho::align_forall_prefix_chirho(
                        actual_vars_chirho,
                        expected_vars_chirho,
                        span_chirho,
                    )?;
                let actual_chirho = kinds_chirho.apply_ty_chirho(actual_body_chirho);
                let expected_chirho = kinds_chirho
                    .apply_ty_chirho(&rename_chirho.apply_ty_chirho(expected_body_chirho));
                Ok(self
                    .unify_normalized_chirho(&actual_chirho, &expected_chirho, span_chirho)?
                    .compose_chirho(&kinds_chirho))
            }
            (
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
                _,
            ) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    let fresh_var_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    instantiation_chirho
                        .insert_chirho(*var_chirho, TyChirho::VarChirho(fresh_var_chirho));
                }
                let instantiated_actual_chirho = instantiation_chirho.apply_ty_chirho(body_chirho);
                self.subsume_normalized_chirho(
                    &instantiated_actual_chirho,
                    expected_chirho,
                    span_chirho,
                )
            }
            (
                _,
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                },
            ) => {
                let mut instantiation_chirho = SubstChirho::empty_chirho();
                for var_chirho in vars_chirho {
                    let fresh_var_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    instantiation_chirho
                        .insert_chirho(*var_chirho, TyChirho::VarChirho(fresh_var_chirho));
                }
                let instantiated_expected_chirho =
                    instantiation_chirho.apply_ty_chirho(body_chirho);
                self.subsume_normalized_chirho(
                    actual_chirho,
                    &instantiated_expected_chirho,
                    span_chirho,
                )
            }
            _ => self.unify_normalized_chirho(actual_chirho, expected_chirho, span_chirho),
        }
    }

    /// Record a unification error as a diagnostic.
    pub(super) fn report_unify_error_chirho(&mut self, err_chirho: &UnifyErrorChirho) {
        match err_chirho {
            UnifyErrorChirho::ResourceExhaustedChirho { .. } => {
                self.type_normalization_exhausted_chirho.set(true);
            }
            UnifyErrorChirho::MismatchChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                let mut diagnostic_chirho = DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
                    format!("type mismatch: expected `{expected_chirho}`, found `{actual_chirho}`"),
                    *span_chirho,
                )
                .with_note_chirho(format!("expected type: {expected_chirho}"))
                .with_note_chirho(format!("   found type: {actual_chirho}"));
                for rigid_chirho in [expected_chirho, actual_chirho] {
                    if let TyChirho::ForallVarChirho(name_chirho) = rigid_chirho
                        && crate::skolem_chirho::is_skolem_name_chirho(name_chirho)
                    {
                        diagnostic_chirho = diagnostic_chirho.with_note_chirho(format!(
                            "`{rigid_chirho}` is a rigid type variable bound by a type signature"
                        ));
                    }
                }
                self.diagnostics_chirho.push_chirho(diagnostic_chirho);
            }
            UnifyErrorChirho::OccursCheckChirho {
                var_chirho,
                ty_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(OCCURS_CHECK_CODE_CHIRHO),
                        format!("infinite type: `{var_chirho}` occurs in `{ty_chirho}`"),
                        *span_chirho,
                    ));
            }
            UnifyErrorChirho::TupleArityChirho {
                expected_chirho,
                actual_chirho,
                span_chirho,
            } => {
                self.diagnostics_chirho.push_chirho(
                    DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TUPLE_ARITY_CODE_CHIRHO),
                        format!(
                            "tuple arity mismatch: expected {expected_chirho} elements, found {actual_chirho}"
                        ),
                        *span_chirho,
                    ),
                );
            }
        }
    }
}
