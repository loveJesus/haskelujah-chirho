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
    fn prepare_required_match_seed_chirho(
        &mut self,
        arm_chirho: &MatchArmChirho,
        required_chirho: &[crate::ty_chirho::QuantifiedVariableChirho],
        parameters_chirho: &[TyChirho],
        result_chirho: &TyChirho,
        givens_chirho: &[PredChirho],
        span_chirho: SpanChirho,
    ) -> Result<
        (Vec<TyChirho>, TyChirho, Vec<PredChirho>),
        crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho,
    > {
        let exhausted_chirho = || crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
            message_chirho: "required equation instantiation resource bound exhausted".into(),
            span_chirho,
        };
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        budget_chirho
            .node_chirho(
                required_chirho
                    .len()
                    .checked_mul(3)
                    .ok_or_else(exhausted_chirho)?,
            )
            .ok_or_else(exhausted_chirho)?;
        if arm_chirho.pats_chirho.len() < required_chirho.len() {
            return Err(
                crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                    message_chirho: "function equation omits required type patterns".into(),
                    span_chirho,
                },
            );
        }
        let mut prefix_chirho = SubstChirho::empty_chirho();
        for (record_chirho, pattern_chirho) in required_chirho.iter().zip(&arm_chirho.pats_chirho) {
            let image_chirho = if let Some(image_chirho) = self
                .skolem_bindings_chirho
                .lookup_image_chirho(&record_chirho.variable_chirho)
            {
                crate::subst_chirho::SubstitutionImageChirho {
                    image_chirho:
                        super::equality_normalization_chirho::copy_term_with_budget_chirho(
                            &image_chirho.image_chirho,
                            &mut budget_chirho,
                        )
                        .ok_or_else(exhausted_chirho)?,
                    kind_chirho: image_chirho.kind_chirho.clone(),
                    source_chirho: image_chirho.source_chirho.clone(),
                }
            } else {
                return Err(
                    crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                        message_chirho: "required equation lost its signed universal scope".into(),
                        span_chirho,
                    },
                );
            };
            prefix_chirho.insert_chirho(
                record_chirho.variable_chirho,
                image_chirho.image_chirho.clone(),
            );
            let mut pattern_chirho = pattern_chirho;
            while let PatChirho::ParenChirho { inner_chirho, .. } = pattern_chirho {
                pattern_chirho = inner_chirho;
            }
            match pattern_chirho {
                PatChirho::VarChirho(name_chirho) => {
                    if let Some(proof_chirho) = name_chirho.type_variable_chirho()
                        && record_chirho.source_chirho.as_ref() != Some(proof_chirho)
                    {
                        return Err(
                            crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                                message_chirho:
                                    "required pattern lost its checked binder correspondence".into(),
                                span_chirho: name_chirho.span_chirho(),
                            },
                        );
                    }
                    budget_chirho
                        .node_chirho(name_chirho.text_chirho().len().saturating_add(1))
                        .ok_or_else(exhausted_chirho)?;
                    self.install_type_pattern_alias_image_chirho(
                        name_chirho.text_chirho().to_owned(),
                        record_chirho.variable_chirho,
                        image_chirho,
                    );
                }
                PatChirho::WildcardChirho(_) => self.install_anonymous_type_pattern_image_chirho(
                    record_chirho.variable_chirho,
                    image_chirho,
                ),
                _ => {
                    return Err(
                        crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                            message_chirho: "unsupported required type-binder pattern".into(),
                            span_chirho: pattern_chirho.span_chirho(),
                        },
                    );
                }
            }
        }
        let mut copy_chirho = |term_chirho: &TyChirho| {
            SubstChirho::empty_chirho().apply_scheme_term_bounded_chirho(
                term_chirho,
                &mut self.elaboration_state_chirho.next_variable_chirho,
                &mut |work_chirho, output_chirho| {
                    budget_chirho.node_chirho(output_chirho)?;
                    budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                },
            )
        };
        let parameters_chirho = parameters_chirho
            .iter()
            .map(&mut copy_chirho)
            .collect::<Option<Vec<_>>>()
            .ok_or_else(exhausted_chirho)?;
        let result_chirho = copy_chirho(result_chirho).ok_or_else(exhausted_chirho)?;
        let arity_chirho = parameters_chirho.len();
        let seed_chirho = TyChirho::fun_n_chirho(parameters_chirho, result_chirho);
        let (body_chirho, givens_chirho) = self
            .instantiate_predicate_terms_chirho(
                givens_chirho,
                &seed_chirho,
                &prefix_chirho,
                &mut budget_chirho,
            )
            .ok_or_else(exhausted_chirho)?;
        let (parameters_chirho, result_chirho) =
            Self::split_fun_ty_for_arity_chirho(&body_chirho, arity_chirho)
                .ok_or_else(exhausted_chirho)?;
        Ok((parameters_chirho, result_chirho, givens_chirho))
    }

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
            seed_vars_chirho.extend(
                pred_chirho
                    .semantic_tys_chirho()
                    .flat_map(TyChirho::free_vars_chirho),
            );
        }
        seed_vars_chirho.sort_by_key(|tv_chirho| tv_chirho.0);
        seed_vars_chirho.dedup();
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let quantified_chirho = self.quantify_variables_chirho(
            seed_vars_chirho,
            &[],
            &[],
            &SubstChirho::empty_chirho(),
            haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho,
            &mut budget_chirho,
        );
        let seed_scheme_chirho = SchemeChirho {
            vars_chirho: quantified_chirho,
            preds_chirho: preds_chirho
                .iter()
                .map(|pred_chirho| {
                    SchemePredChirho::from_proof_chirho(
                        pred_chirho.class_identity_chirho().clone(),
                        crate::ty_chirho::SchemePredPartsChirho {
                            kind_tys_chirho: pred_chirho.kind_tys_chirho.clone(),
                            class_name_chirho: pred_chirho.class_name_chirho.clone(),
                            ty_chirho: pred_chirho.ty_chirho.clone(),
                            extra_tys_chirho: pred_chirho.extra_tys_chirho.clone(),
                        },
                    )
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
        required_chirho: &[crate::ty_chirho::QuantifiedVariableChirho],
    ) -> (SubstChirho, TyChirho) {
        if matches_chirho.is_empty() {
            return (SubstChirho::empty_chirho(), self.fresh_var_chirho());
        }

        let first_chirho = &matches_chirho[0];
        let arity_chirho = first_chirho
            .pats_chirho
            .len()
            .saturating_sub(required_chirho.len());
        let mut subst_chirho = SubstChirho::empty_chirho();
        let signature_givens_chirho = if authoritative_seed_chirho {
            self.given_preds_chirho.clone()
        } else {
            Vec::new()
        };

        for match_arm_chirho in matches_chirho {
            self.env_chirho.push_scope_chirho();
            let pattern_frame_chirho = self.begin_pattern_implication_chirho();

            let required_seed_chirho = if required_chirho.is_empty() {
                None
            } else {
                match self.prepare_required_match_seed_chirho(
                    match_arm_chirho,
                    required_chirho,
                    &param_tys_chirho,
                    &result_ty_chirho,
                    &signature_givens_chirho,
                    span_chirho,
                ) {
                    Ok(seed_chirho) => Some(seed_chirho),
                    Err(error_chirho) => {
                        self.non_deferrable_diagnostics_chirho
                            .push_chirho(error_chirho.into_diagnostic_chirho());
                        let _solution_chirho = self.end_pattern_implication_chirho(
                            pattern_frame_chirho,
                            &result_ty_chirho,
                            span_chirho,
                        );
                        self.env_chirho.pop_scope_chirho();
                        continue;
                    }
                }
            };

            // Each equation gets its own fresh param/result types so GADT
            // pattern matching can refine types independently per equation.
            // For explicit signatures / expected types, freshen the entire
            // seeded function type per equation so relationships like
            // `ASeq f u -> f u` stay linked within that equation, but do not
            // leak refinements such as `u ~ ()` from one equation into the next.
            let (eq_param_tys_chirho, eq_result_ty_chirho, eq_given_preds_chirho) =
                if authoritative_seed_chirho {
                    let (parameters_chirho, result_chirho, givens_chirho) = required_seed_chirho
                        .as_ref()
                        .map(|(parameters_chirho, result_chirho, givens_chirho)| {
                            (
                                parameters_chirho.as_slice(),
                                result_chirho,
                                givens_chirho.as_slice(),
                            )
                        })
                        .unwrap_or((
                            &param_tys_chirho,
                            &result_ty_chirho,
                            &signature_givens_chirho,
                        ));
                    self.freshen_match_seed_and_givens_chirho(
                        parameters_chirho,
                        result_chirho,
                        givens_chirho,
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
                .skip(required_chirho.len())
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
            &[],
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
                TyChirho::QualChirho { body_chirho, .. }
                | TyChirho::ForallChirho { body_chirho, .. } => {
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
        let mut prefix_records_chirho = Vec::new();
        let mut required_records_chirho = Vec::new();
        let mut contexts_chirho = Vec::new();
        let mut required_body_chirho = expected_ty_chirho;
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        loop {
            if budget_chirho.work_chirho(1).is_none() {
                self.type_normalization_exhausted_chirho.set(true);
                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
            }
            match required_body_chirho {
                TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    if budget_chirho
                        .node_chirho(vars_chirho.len().saturating_mul(2))
                        .is_none()
                    {
                        self.type_normalization_exhausted_chirho.set(true);
                        return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                    }
                    if matches!(required_body_chirho, TyChirho::RequiredForallChirho { .. }) {
                        required_records_chirho.extend_from_slice(vars_chirho.records_chirho());
                    }
                    prefix_records_chirho.extend_from_slice(vars_chirho.records_chirho());
                    required_body_chirho = body_chirho;
                }
                TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                } => {
                    for predicate_chirho in context_chirho {
                        let Some(predicate_chirho) =
                            super::equality_normalization_chirho::copy_term_with_budget_chirho(
                                predicate_chirho,
                                &mut budget_chirho,
                            )
                        else {
                            self.type_normalization_exhausted_chirho.set(true);
                            return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                        };
                        contexts_chirho.push(predicate_chirho);
                    }
                    required_body_chirho = body_chirho;
                }
                _ => break,
            }
        }
        let required_arity_chirho = required_records_chirho.len();
        if required_arity_chirho > 0 {
            if arity_chirho < required_arity_chirho {
                self.non_deferrable_diagnostics_chirho.push_chirho(
                    crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                        message_chirho: "function equation omits required type patterns".into(),
                        span_chirho,
                    }
                    .into_diagnostic_chirho(),
                );
                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
            }
            let Some(body_chirho) =
                super::equality_normalization_chirho::copy_term_with_budget_chirho(
                    required_body_chirho,
                    &mut budget_chirho,
                )
            else {
                self.type_normalization_exhausted_chirho.set(true);
                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
            };
            let scheme_chirho = SchemeChirho {
                vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
                    prefix_records_chirho,
                ),
                preds_chirho: Vec::new(),
                ty_chirho: if contexts_chirho.is_empty() {
                    body_chirho
                } else {
                    TyChirho::QualChirho {
                        context_chirho: contexts_chirho,
                        body_chirho: Box::new(body_chirho),
                    }
                },
            };
            let ((substitution_chirho, _), public_chirho) = self.with_rigid_signature_chirho(
                &scheme_chirho,
                span_chirho,
                |context_chirho, opened_chirho| {
                    let term_arity_chirho = arity_chirho - required_arity_chirho;
                    let Some((parameters_chirho, result_chirho, givens_chirho)) = context_chirho
                        .open_expected_function_spine_chirho(opened_chirho, term_arity_chirho)
                    else {
                        context_chirho
                            .non_deferrable_diagnostics_chirho
                            .push_chirho(
                            crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                                message_chirho:
                                    "required function equation has no matching checked value spine"
                                        .into(),
                                span_chirho,
                            }
                            .into_diagnostic_chirho(),
                        );
                        return (SubstChirho::empty_chirho(), opened_chirho.clone());
                    };
                    context_chirho.with_signature_givens_chirho(
                        givens_chirho,
                        opened_chirho,
                        |context_chirho| {
                            context_chirho.infer_matches_with_seed_chirho(
                                matches_chirho,
                                span_chirho,
                                parameters_chirho,
                                result_chirho,
                                true,
                                &required_records_chirho,
                            )
                        },
                    )
                },
            );
            let substitution_chirho =
                self.publish_signature_solution_chirho(substitution_chirho, &public_chirho);
            let checked_chirho = substitution_chirho.apply_ty_chirho(expected_ty_chirho);
            return (substitution_chirho, checked_chirho);
        }

        if arity_chirho == 0 {
            return self.infer_matches_with_seed_chirho(
                matches_chirho,
                span_chirho,
                vec![],
                expected_ty_chirho.clone(),
                true,
                &[],
            );
        }
        // A function equation may bind fewer arguments than its signature and
        // return the remaining function (for example `f x = \y -> ...`).
        // Preserve that residual function as the RHS expected type.
        if self.fun_arity_chirho(expected_ty_chirho) < arity_chirho {
            return self.infer_matches_chirho(matches_chirho, span_chirho);
        }
        if let Some((param_tys_chirho, result_ty_chirho, givens_chirho)) =
            self.open_expected_function_spine_chirho(expected_ty_chirho, arity_chirho)
        {
            let (substitution_chirho, _) = self.with_signature_givens_chirho(
                givens_chirho,
                expected_ty_chirho,
                |context_chirho| {
                    context_chirho.infer_matches_with_seed_chirho(
                        matches_chirho,
                        span_chirho,
                        param_tys_chirho,
                        result_ty_chirho,
                        true,
                        &[],
                    )
                },
            );
            let checked_chirho = substitution_chirho.apply_ty_chirho(expected_ty_chirho);
            (substitution_chirho, checked_chirho)
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
        let (param_tys_chirho, result_ty_chirho, givens_chirho) =
            self.open_expected_function_spine_chirho(expected_ty_chirho, arity_chirho)?;
        let (substitution_chirho, _) = self.with_signature_givens_chirho(
            givens_chirho,
            expected_ty_chirho,
            |context_chirho| {
                context_chirho.infer_lambda_with_seed_chirho(
                    pats_chirho,
                    body_chirho,
                    param_tys_chirho,
                    result_ty_chirho,
                    span_chirho,
                )
            },
        );
        let checked_chirho = substitution_chirho.apply_ty_chirho(expected_ty_chirho);
        Some((substitution_chirho, checked_chirho))
    }
}
