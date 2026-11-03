// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Match arms and lambdas, inferred alone or against an expected type.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use crate::class_chirho::PredChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    /// Infer the type of a set of match arms (function equations).
    fn freshen_match_seed_and_givens_chirho(
        &mut self,
        param_tys_chirho: &[TyChirho],
        result_ty_chirho: &TyChirho,
        preds_chirho: &[PredChirho],
    ) -> Option<(Vec<TyChirho>, TyChirho, Vec<PredChirho>)> {
        let seed_fun_ty_chirho =
            TyChirho::fun_n_chirho(param_tys_chirho.to_vec(), result_ty_chirho.clone());
        let mut seed_vars_chirho: Vec<TyVarChirho> =
            seed_fun_ty_chirho.free_vars_chirho().into_iter().collect();
        for pred_chirho in preds_chirho {
            seed_vars_chirho.extend(pred_chirho.ty_chirho.free_vars_chirho());
            for extra_ty_chirho in &pred_chirho.extra_tys_chirho {
                seed_vars_chirho.extend(extra_ty_chirho.free_vars_chirho());
            }
        }
        seed_vars_chirho.sort_by_key(|tv_chirho| tv_chirho.0);
        seed_vars_chirho.dedup();
        let seed_scheme_chirho = SchemeChirho {
            vars_chirho: seed_vars_chirho,
            preds_chirho: preds_chirho
                .iter()
                .map(|pred_chirho| SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho.clone(),
                    ty_chirho: pred_chirho.ty_chirho.clone(),
                    extra_tys_chirho: pred_chirho.extra_tys_chirho.clone(),
                })
                .collect(),
            ty_chirho: seed_fun_ty_chirho,
        };
        let (fresh_fun_ty_chirho, fresh_preds_chirho) =
            self.instantiate_scheme_parts_chirho(&seed_scheme_chirho);
        let (fresh_params_chirho, fresh_result_chirho) =
            Self::split_fun_ty_for_arity_chirho(&fresh_fun_ty_chirho, param_tys_chirho.len())?;
        Some((fresh_params_chirho, fresh_result_chirho, fresh_preds_chirho))
    }

    fn infer_matches_with_seed_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
        param_tys_chirho: Vec<TyChirho>,
        result_ty_chirho: TyChirho,
        authoritative_seed_chirho: bool,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let first_chirho = &matches_chirho[0];
        let arity_chirho = first_chirho.pats_chirho.len();
        let mut subst_chirho = SubstChirho::empty_chirho();
        let signature_givens_chirho = if authoritative_seed_chirho {
            self.given_preds_chirho.clone()
        } else {
            Vec::new()
        };

        for match_arm_chirho in matches_chirho {
            self.env_chirho.push_scope_chirho();
            let pattern_frame_chirho = self.begin_pattern_implication_chirho();

            // Each equation gets its own fresh param/result types so GADT
            // pattern matching can refine types independently per equation.
            // For explicit signatures / expected types, freshen the entire
            // seeded function type per equation so relationships like
            // `ASeq f u -> f u` stay linked within that equation, but do not
            // leak refinements such as `u ~ ()` from one equation into the next.
            let (eq_param_tys_chirho, eq_result_ty_chirho, eq_given_preds_chirho) =
                if authoritative_seed_chirho {
                    self.freshen_match_seed_and_givens_chirho(
                        &param_tys_chirho,
                        &result_ty_chirho,
                        &signature_givens_chirho,
                    )
                    .unwrap_or_else(|| {
                        let fresh_params_chirho: Vec<TyChirho> =
                            (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect();
                        let fresh_result_chirho = self.fresh_var_chirho();
                        (fresh_params_chirho, fresh_result_chirho, Vec::new())
                    })
                } else {
                    (
                        (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect(),
                        self.fresh_var_chirho(),
                        Vec::new(),
                    )
                };
            if authoritative_seed_chirho {
                self.given_preds_chirho.extend(eq_given_preds_chirho);
            }

            // Bind pattern variables against per-equation fresh types
            for (pat_chirho, pat_ty_chirho) in match_arm_chirho
                .pats_chirho
                .iter()
                .zip(eq_param_tys_chirho.iter())
            {
                let pat_ty_sub_chirho = subst_chirho.apply_ty_chirho(pat_ty_chirho);
                let sp_chirho = self.bind_pat_chirho(pat_chirho, &pat_ty_sub_chirho);
                subst_chirho = sp_chirho.compose_chirho(&subst_chirho);
                self.apply_subst_all_chirho(&sp_chirho);
            }

            self.infer_local_binds_chirho(&match_arm_chirho.where_binds_chirho, &mut subst_chirho);

            // Infer RHS
            let expected_result_sub_chirho = if authoritative_seed_chirho {
                subst_chirho.apply_ty_chirho(&eq_result_ty_chirho)
            } else {
                subst_chirho.apply_ty_chirho(&result_ty_chirho)
            };
            let (sr_chirho, rhs_ty_chirho) = self.infer_rhs_against_expected_chirho(
                &match_arm_chirho.rhs_chirho,
                &expected_result_sub_chirho,
                span_chirho,
            );
            subst_chirho = sr_chirho.compose_chirho(&subst_chirho);
            self.apply_subst_all_chirho(&sr_chirho);

            // Unify RHS with per-equation result type
            let eq_result_sub_chirho = subst_chirho.apply_ty_chirho(&eq_result_ty_chirho);
            match self.unify_normalized_chirho(&eq_result_sub_chirho, &rhs_ty_chirho, span_chirho) {
                Ok(su_chirho) => {
                    subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&su_chirho);
                }
                Err(err_chirho) => {
                    self.report_unify_error_chirho(&err_chirho);
                }
            }

            if !authoritative_seed_chirho {
                // Unify per-equation function type with overall function type.
                // This propagates non-GADT constraints while allowing GADT
                // equations to have independently refined types.
                for (eq_p_chirho, p_chirho) in
                    eq_param_tys_chirho.iter().zip(param_tys_chirho.iter())
                {
                    let eq_p_sub_chirho = subst_chirho.apply_ty_chirho(eq_p_chirho);
                    let p_sub_chirho = subst_chirho.apply_ty_chirho(p_chirho);
                    if let Ok(s_chirho) =
                        self.unify_normalized_chirho(&eq_p_sub_chirho, &p_sub_chirho, span_chirho)
                    {
                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&s_chirho);
                    }
                }
                // Unify per-equation result with overall result type.
                // For non-GADT cases this propagates the body type to the
                // function result; for GADTs the signature check provides
                // the authoritative type.
                let eq_r_sub_chirho = subst_chirho.apply_ty_chirho(&eq_result_ty_chirho);
                let r_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                match self.unify_normalized_chirho(&eq_r_sub_chirho, &r_sub_chirho, span_chirho) {
                    Ok(s_chirho) => {
                        subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&s_chirho);
                    }
                    Err(err_chirho) => {
                        // For arity-0 bindings (e.g. x = True) there are no
                        // GADT patterns to refine types, so a mismatch between
                        // inferred and expected result is a real type error.
                        // For higher-arity matches, GADT pattern matching can
                        // refine per-equation types, so we allow the mismatch.
                        if arity_chirho == 0 {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }
            }

            let solved_chirho = self.end_pattern_implication_chirho(
                pattern_frame_chirho,
                &subst_chirho.apply_ty_chirho(&eq_result_ty_chirho),
                span_chirho,
            );
            subst_chirho = solved_chirho.compose_chirho(&subst_chirho);
            self.env_chirho.pop_scope_chirho();
        }

        // Build the function type: param1 -> param2 -> ... -> result
        let final_params_chirho: Vec<TyChirho> = param_tys_chirho
            .iter()
            .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
            .collect();
        let final_result_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
        let fun_ty_chirho = TyChirho::fun_n_chirho(final_params_chirho, final_result_chirho);

        (subst_chirho, fun_ty_chirho)
    }

    pub(super) fn infer_matches_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let arity_chirho = matches_chirho[0].pats_chirho.len();
        let param_tys_chirho: Vec<TyChirho> =
            (0..arity_chirho).map(|_| self.fresh_var_chirho()).collect();
        let result_ty_chirho = self.fresh_var_chirho();
        self.infer_matches_with_seed_chirho(
            matches_chirho,
            span_chirho,
            param_tys_chirho,
            result_ty_chirho,
            false,
        )
    }

    fn split_fun_ty_for_arity_chirho(
        ty_chirho: &TyChirho,
        arity_chirho: usize,
    ) -> Option<(Vec<TyChirho>, TyChirho)> {
        let mut current_chirho = ty_chirho.clone();
        let mut params_chirho = Vec::with_capacity(arity_chirho);

        for _ in 0..arity_chirho {
            match current_chirho {
                TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                    params_chirho.push(*arg_chirho);
                    current_chirho = *result_chirho;
                }
                TyChirho::ForallChirho { body_chirho, .. } => {
                    current_chirho = *body_chirho;
                    match current_chirho {
                        TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
                            params_chirho.push(*arg_chirho);
                            current_chirho = *result_chirho;
                        }
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }

        Some((params_chirho, current_chirho))
    }

    pub(super) fn fun_arity_chirho(&self, ty_chirho: &TyChirho) -> usize {
        let mut arity_chirho = 0;
        let mut current_chirho = ty_chirho;
        loop {
            if self.matches_function_result_alias_expansion_chirho(current_chirho) {
                return arity_chirho;
            }
            match current_chirho {
                TyChirho::ForallChirho { body_chirho, .. } => {
                    current_chirho = body_chirho;
                }
                TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    arity_chirho += vars_chirho.len();
                    current_chirho = body_chirho;
                }
                TyChirho::FunChirho(_, result_chirho, _) => {
                    arity_chirho += 1;
                    current_chirho = result_chirho;
                }
                _ => return arity_chirho,
            }
        }
    }

    pub(super) fn infer_matches_against_expected_chirho(
        &mut self,
        matches_chirho: &[MatchArmChirho],
        span_chirho: SpanChirho,
        expected_ty_chirho: &TyChirho,
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let arity_chirho = matches_chirho[0].pats_chirho.len();
        // workflow: language-features-chirho/rank-n-visible-type-application-chirho
        let mut required_groups_chirho = Vec::new();
        let mut required_body_chirho = expected_ty_chirho.clone();
        while let TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } = required_body_chirho
        {
            required_groups_chirho.push(vars_chirho);
            required_body_chirho = *body_chirho;
        }
        let required_arity_chirho: usize = required_groups_chirho.iter().map(Vec::len).sum();
        if required_arity_chirho > 0 {
            if arity_chirho < required_arity_chirho {
                return self.infer_matches_chirho(matches_chirho, span_chirho);
            }
            let term_arity_chirho = arity_chirho - required_arity_chirho;
            if self.fun_arity_chirho(&required_body_chirho) != term_arity_chirho {
                return self.infer_matches_chirho(matches_chirho, span_chirho);
            }

            let trimmed_matches_chirho: Vec<MatchArmChirho> = matches_chirho
                .iter()
                .cloned()
                .map(|mut match_chirho| {
                    match_chirho.pats_chirho.drain(..required_arity_chirho);
                    match_chirho
                })
                .collect();
            let Some((param_tys_chirho, result_ty_chirho)) =
                Self::split_fun_ty_for_arity_chirho(&required_body_chirho, term_arity_chirho)
            else {
                return self.infer_matches_chirho(matches_chirho, span_chirho);
            };
            let (subst_chirho, inferred_body_chirho) = self.infer_matches_with_seed_chirho(
                &trimmed_matches_chirho,
                span_chirho,
                param_tys_chirho,
                result_ty_chirho,
                true,
            );
            let inferred_ty_chirho = required_groups_chirho.into_iter().rev().fold(
                inferred_body_chirho,
                |body_chirho, vars_chirho| TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho: Box::new(body_chirho),
                },
            );
            return (subst_chirho, inferred_ty_chirho);
        }

        if arity_chirho == 0 {
            return self.infer_matches_with_seed_chirho(
                matches_chirho,
                span_chirho,
                vec![],
                expected_ty_chirho.clone(),
                true,
            );
        }
        // A function equation may bind fewer arguments than its signature and
        // return the remaining function (for example `f x = \y -> ...`).
        // Preserve that residual function as the RHS expected type.
        if self.fun_arity_chirho(expected_ty_chirho) < arity_chirho {
            return self.infer_matches_chirho(matches_chirho, span_chirho);
        }
        if let Some((param_tys_chirho, result_ty_chirho)) =
            Self::split_fun_ty_for_arity_chirho(expected_ty_chirho, arity_chirho)
        {
            self.infer_matches_with_seed_chirho(
                matches_chirho,
                span_chirho,
                param_tys_chirho,
                result_ty_chirho,
                true,
            )
        } else {
            self.infer_matches_chirho(matches_chirho, span_chirho)
        }
    }

    pub(super) fn infer_lambda_with_seed_chirho(
        &mut self,
        pats_chirho: &[PatChirho],
        body_chirho: &ExprChirho,
        param_tys_chirho: Vec<TyChirho>,
        result_ty_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        self.env_chirho.push_scope_chirho();
        let pattern_frame_chirho = self.begin_pattern_implication_chirho();

        let mut param_subst_chirho = SubstChirho::empty_chirho();
        for (pat_chirho, pat_ty_chirho) in pats_chirho.iter().zip(param_tys_chirho.iter()) {
            let pat_ty_sub_chirho = param_subst_chirho.apply_ty_chirho(pat_ty_chirho);
            let s_chirho = self.bind_pat_chirho(pat_chirho, &pat_ty_sub_chirho);
            param_subst_chirho = s_chirho.compose_chirho(&param_subst_chirho);
            self.apply_subst_all_chirho(&s_chirho);
        }

        let expected_result_sub_chirho = param_subst_chirho.apply_ty_chirho(&result_ty_chirho);
        let (body_subst_chirho, body_ty_chirho) = self.infer_expr_against_expected_chirho(
            body_chirho,
            &expected_result_sub_chirho,
            span_chirho,
        );
        let combined_subst_chirho = body_subst_chirho.compose_chirho(&param_subst_chirho);
        let expected_result_final_chirho = combined_subst_chirho.apply_ty_chirho(&result_ty_chirho);

        let mut final_subst_chirho = combined_subst_chirho.clone();
        match self.unify_normalized_chirho(
            &body_ty_chirho,
            &expected_result_final_chirho,
            span_chirho,
        ) {
            Ok(su_chirho) => {
                final_subst_chirho = su_chirho.compose_chirho(&final_subst_chirho);
                self.apply_subst_all_chirho(&su_chirho);
            }
            Err(err_chirho) => {
                self.report_unify_error_chirho(&err_chirho);
            }
        }

        let solved_chirho = self.end_pattern_implication_chirho(
            pattern_frame_chirho,
            &final_subst_chirho.apply_ty_chirho(&result_ty_chirho),
            span_chirho,
        );
        final_subst_chirho = solved_chirho.compose_chirho(&final_subst_chirho);
        self.env_chirho.pop_scope_chirho();

        let mut result_fun_ty_chirho = final_subst_chirho.apply_ty_chirho(&result_ty_chirho);
        for param_ty_chirho in param_tys_chirho.into_iter().rev() {
            let param_sub_chirho = final_subst_chirho.apply_ty_chirho(&param_ty_chirho);
            result_fun_ty_chirho = TyChirho::fun_chirho(param_sub_chirho, result_fun_ty_chirho);
        }

        (final_subst_chirho, result_fun_ty_chirho)
    }

    pub(super) fn infer_lambda_against_expected_chirho(
        &mut self,
        pats_chirho: &[PatChirho],
        body_chirho: &ExprChirho,
        expected_ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        let arity_chirho = pats_chirho.len();
        let (param_tys_chirho, result_ty_chirho) =
            Self::split_fun_ty_for_arity_chirho(expected_ty_chirho, arity_chirho)?;
        Some(self.infer_lambda_with_seed_chirho(
            pats_chirho,
            body_chirho,
            param_tys_chirho,
            result_ty_chirho,
            span_chirho,
        ))
    }
}
