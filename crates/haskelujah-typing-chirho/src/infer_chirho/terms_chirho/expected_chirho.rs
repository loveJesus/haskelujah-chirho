// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expressions checked against an expected type.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::build_infix_app_expr_chirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyChirho;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    /// Sections share ordinary lambda/application checking. The compiler-only
    /// binder uses the existing fresh supply and cannot capture a written name;
    /// the actual operator keeps its selected name and occurrence origin.
    /// workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
    pub(super) fn build_section_lambda_chirho(
        &mut self,
        op_chirho: &NameChirho,
        arg_chirho: &ExprChirho,
        fixed_left_chirho: bool,
        span_chirho: SpanChirho,
    ) -> ExprChirho {
        let TyChirho::VarChirho(variable_chirho) = self.fresh_var_chirho() else {
            unreachable!("fresh section variable Chirho")
        };
        let binder_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            format!("$section:{}_chirho", variable_chirho.0),
            span_chirho,
        ));
        let reference_chirho = ExprChirho::VarChirho(binder_chirho.clone());
        let (left_chirho, right_chirho) = if fixed_left_chirho {
            (arg_chirho, &reference_chirho)
        } else {
            (&reference_chirho, arg_chirho)
        };
        ExprChirho::LamChirho {
            pats_chirho: vec![PatChirho::VarChirho(binder_chirho)],
            body_chirho: Box::new(build_infix_app_expr_chirho(
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            )),
            span_chirho,
        }
    }

    pub(super) fn contains_polymorphic_component_chirho(ty_chirho: &TyChirho) -> bool {
        match ty_chirho {
            TyChirho::QualChirho { .. }
            | TyChirho::ForallChirho { .. }
            | TyChirho::RequiredForallChirho { .. } => true,
            TyChirho::AppChirho(fun_chirho, arg_chirho)
            | TyChirho::FunChirho(fun_chirho, arg_chirho, _) => {
                Self::contains_polymorphic_component_chirho(fun_chirho)
                    || Self::contains_polymorphic_component_chirho(arg_chirho)
            }
            TyChirho::TupleChirho(elements_chirho) => elements_chirho
                .iter()
                .any(Self::contains_polymorphic_component_chirho),
            TyChirho::ListChirho(element_chirho) => {
                Self::contains_polymorphic_component_chirho(element_chirho)
            }
            TyChirho::VarChirho(_) | TyChirho::ConChirho(_) | TyChirho::ForallVarChirho(_) => false,
        }
    }

    /// Inferred fallbacks return a current actual type; their substitution
    /// transports the external expected input before the new equality check.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub(super) fn infer_expr_against_expected_chirho(
        &mut self,
        expr_chirho: &ExprChirho,
        expected_ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        if self.has_non_deferrable_failure_chirho() {
            return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
        }
        let normalized_expected_chirho = self.normalize_ty_chirho(expected_ty_chirho);
        if matches!(normalized_expected_chirho, TyChirho::QualChirho { .. }) {
            let (body_chirho, givens_chirho) =
                self.expose_qualified_parts_chirho(normalized_expected_chirho, Vec::new());
            let (substitution_chirho, _) =
                self.with_signature_givens_chirho(givens_chirho, &body_chirho, |context_chirho| {
                    context_chirho.infer_expr_against_expected_chirho(
                        expr_chirho,
                        &body_chirho,
                        span_chirho,
                    )
                });
            let checked_chirho = substitution_chirho.apply_ty_chirho(expected_ty_chirho);
            return (substitution_chirho, checked_chirho);
        }
        if matches!(
            expr_chirho,
            ExprChirho::VarChirho(_) | ExprChirho::ConChirho(_) | ExprChirho::TypeAppChirho { .. }
        ) && let Some(result_chirho) =
            self.infer_application_spine_chirho(expr_chirho, Some(expected_ty_chirho), span_chirho)
        {
            return result_chirho;
        }
        match expr_chirho {
            ExprChirho::ParenChirho { inner_chirho, .. } => self
                .infer_expr_against_expected_chirho(inner_chirho, expected_ty_chirho, span_chirho),
            ExprChirho::LeftSectionChirho {
                op_chirho,
                arg_chirho,
                span_chirho: section_span_chirho,
            }
            | ExprChirho::RightSectionChirho {
                op_chirho,
                arg_chirho,
                span_chirho: section_span_chirho,
            } => {
                let lambda_chirho = self.build_section_lambda_chirho(
                    op_chirho,
                    arg_chirho,
                    matches!(expr_chirho, ExprChirho::RightSectionChirho { .. }),
                    *section_span_chirho,
                );
                self.infer_expr_against_expected_chirho(
                    &lambda_chirho,
                    expected_ty_chirho,
                    *section_span_chirho,
                )
            }
            ExprChirho::TypeLamChirho {
                binder_chirho,
                body_chirho,
                ..
            } => {
                let normalized_expected_chirho = self.normalize_ty_chirho(expected_ty_chirho);
                let TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho: forall_body_chirho,
                } = &normalized_expected_chirho
                else {
                    return self.infer_expr_chirho(expr_chirho);
                };
                let Some((first_var_chirho, remaining_vars_chirho)) = vars_chirho.split_first()
                else {
                    return self.infer_expr_chirho(expr_chirho);
                };

                let expected_body_chirho = if remaining_vars_chirho.is_empty() {
                    forall_body_chirho.as_ref().clone()
                } else {
                    TyChirho::ForallChirho {
                        vars_chirho: vars_chirho.slice_chirho(1..vars_chirho.len()),
                        body_chirho: forall_body_chirho.clone(),
                    }
                };
                let binder_text_chirho = binder_chirho.text_chirho().to_string();
                let previous_chirho = self
                    .scoped_tyvars_chirho
                    .insert(binder_text_chirho.clone(), *first_var_chirho);
                let (subst_chirho, _) = self.infer_expr_against_expected_chirho(
                    body_chirho,
                    &expected_body_chirho,
                    span_chirho,
                );
                if let Some(previous_chirho) = previous_chirho {
                    self.scoped_tyvars_chirho
                        .insert(binder_text_chirho, previous_chirho);
                } else {
                    self.scoped_tyvars_chirho.remove(&binder_text_chirho);
                }
                let final_ty_chirho = subst_chirho.apply_ty_chirho(&normalized_expected_chirho);
                (subst_chirho, final_ty_chirho)
            }
            // As for `if`: polymorphic arm results need bidirectional checking.
            // workflow: language-features-chirho/rank-n-visible-type-application-chirho
            ExprChirho::MultiWayIfChirho { arms_chirho, .. }
                if Self::contains_polymorphic_component_chirho(expected_ty_chirho) =>
            {
                self.infer_multiway_if_chirho(arms_chirho, Some(expected_ty_chirho))
            }
            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                span_chirho: if_span_chirho,
            } if Self::contains_polymorphic_component_chirho(expected_ty_chirho) => self
                .check_if_against_expected_chirho(
                    cond_chirho,
                    then_chirho,
                    else_chirho,
                    *if_span_chirho,
                    expected_ty_chirho,
                ),
            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } if Self::contains_polymorphic_component_chirho(expected_ty_chirho)
                || self.type_contains_skolem_chirho(expected_ty_chirho) =>
            {
                // A rigid expected type must reach every alternative, because a
                // GADT alternative sees it through its own refinement.
                // Preserve ordinary/GADT case leniency by entering this path
                // only when a nested polymorphic type actually needs to reach
                // alternative lambdas.
                // workflow: language-features-chirho/rank-n-visible-type-application-chirho
                let (mut scrut_subst_chirho, scrut_ty_chirho) =
                    self.infer_expr_chirho(scrutinee_chirho);
                if self
                    .apply_subst_all_chirho(&mut scrut_subst_chirho)
                    .is_err()
                {
                    return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                };
                let mut subst_chirho = scrut_subst_chirho;

                for alt_chirho in alts_chirho {
                    self.env_chirho.push_scope_chirho();
                    let pattern_frame_chirho = self.begin_pattern_implication_with_roots_chirho(
                        [&scrut_ty_chirho, expected_ty_chirho],
                        span_chirho,
                    );
                    if self.has_non_deferrable_failure_chirho() {
                        return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                    }

                    self.bind_pat_against_chirho(
                        &alt_chirho.pat_chirho,
                        &scrut_ty_chirho,
                        &mut subst_chirho,
                    );

                    self.infer_local_binds_chirho(
                        &alt_chirho.where_binds_chirho,
                        &mut subst_chirho,
                    );

                    let alt_expected_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    let (mut alt_subst_chirho, alt_ty_chirho) = self
                        .infer_rhs_against_expected_chirho(
                            &alt_chirho.rhs_chirho,
                            &alt_expected_chirho,
                            alt_chirho.span_chirho,
                        );
                    if self.apply_subst_all_chirho(&mut alt_subst_chirho).is_err() {
                        return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                    };
                    subst_chirho = alt_subst_chirho.compose_chirho(&subst_chirho);

                    // A guarded alternative's arms meet the expected type
                    // inside their own scopes. Retain the existing lenient
                    // result merge so GADT-refined alternatives do not gain
                    // false diagnostics.
                    let alt_ty_sub_chirho = subst_chirho.apply_ty_chirho(&alt_ty_chirho);
                    let expected_alt_final_chirho =
                        subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    if let Ok(mut unify_subst_chirho) = self.unify_normalized_chirho(
                        &alt_ty_sub_chirho,
                        &expected_alt_final_chirho,
                        alt_chirho.span_chirho,
                    ) {
                        if self
                            .apply_subst_all_chirho(&mut unify_subst_chirho)
                            .is_err()
                        {
                            return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                        };
                        subst_chirho = unify_subst_chirho.compose_chirho(&subst_chirho);
                    }

                    let solved_chirho = self.end_pattern_implication_chirho(
                        pattern_frame_chirho,
                        &subst_chirho.apply_ty_chirho(expected_ty_chirho),
                        alt_chirho.span_chirho,
                    );
                    subst_chirho = solved_chirho.compose_chirho(&subst_chirho);
                    self.env_chirho.pop_scope_chirho();
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }
            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                ..
            } => self
                .infer_lambda_against_expected_chirho(
                    pats_chirho,
                    body_chirho,
                    expected_ty_chirho,
                    span_chirho,
                )
                .unwrap_or_else(|| self.infer_expr_chirho(expr_chirho)),
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho: app_span_chirho,
            } => {
                if let Some(result_chirho) = self.infer_application_spine_chirho(
                    expr_chirho,
                    Some(expected_ty_chirho),
                    *app_span_chirho,
                ) {
                    return result_chirho;
                }
                // Bidirectional application checking lets a known result type
                // specialize variables that also occur in the parameter. This
                // is required for `g True :: Char` when `g :: F a -> a` and
                // `F Char ~ Bool`.
                // workflow: language-features-chirho/rank-n-visible-type-application-chirho
                let (mut fun_subst_chirho, inferred_fun_ty_chirho) =
                    self.infer_expr_chirho(fun_chirho);
                if self.apply_subst_all_chirho(&mut fun_subst_chirho).is_err() {
                    return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                };
                let inferred_fun_sub_chirho =
                    fun_subst_chirho.apply_ty_chirho(&inferred_fun_ty_chirho);

                if let Some((links_chirho, required_result_ty_chirho)) = self
                    .infer_required_type_application_chirho(
                        &inferred_fun_sub_chirho,
                        arg_chirho,
                        *app_span_chirho,
                    )
                {
                    let fun_subst_chirho = links_chirho.compose_chirho(&fun_subst_chirho);
                    return match self.unify_normalized_chirho(
                        &required_result_ty_chirho,
                        expected_ty_chirho,
                        *app_span_chirho,
                    ) {
                        Ok(mut result_subst_chirho) => {
                            if self
                                .apply_subst_all_chirho(&mut result_subst_chirho)
                                .is_err()
                            {
                                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                            };
                            let combined_chirho =
                                result_subst_chirho.compose_chirho(&fun_subst_chirho);
                            let final_ty_chirho =
                                combined_chirho.apply_ty_chirho(expected_ty_chirho);
                            (combined_chirho, final_ty_chirho)
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                            (fun_subst_chirho, required_result_ty_chirho)
                        }
                    };
                }

                let normalized_fun_ty_chirho = self.normalize_ty_chirho(&inferred_fun_sub_chirho);
                if let TyChirho::FunChirho(expected_arg_ty_chirho, inferred_result_ty_chirho, _) =
                    normalized_fun_ty_chirho
                {
                    let mut subst_chirho = fun_subst_chirho;

                    // Seed from the result when possible. If a family result
                    // cannot reduce until the argument is known, defer this
                    // diagnostic and retry after checking the argument.
                    if let Ok(mut result_subst_chirho) = self.unify_normalized_chirho(
                        &inferred_result_ty_chirho,
                        expected_ty_chirho,
                        *app_span_chirho,
                    ) {
                        if self
                            .apply_subst_all_chirho(&mut result_subst_chirho)
                            .is_err()
                        {
                            return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                        };
                        subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                    }

                    let expected_arg_sub_chirho =
                        subst_chirho.apply_ty_chirho(&expected_arg_ty_chirho);
                    let (mut arg_subst_chirho, inferred_arg_ty_chirho) = self
                        .infer_expr_against_expected_chirho(
                            arg_chirho,
                            &expected_arg_sub_chirho,
                            *app_span_chirho,
                        );
                    if self.apply_subst_all_chirho(&mut arg_subst_chirho).is_err() {
                        return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                    };
                    subst_chirho = arg_subst_chirho.compose_chirho(&subst_chirho);

                    let inferred_arg_sub_chirho =
                        subst_chirho.apply_ty_chirho(&inferred_arg_ty_chirho);
                    let expected_arg_final_chirho =
                        subst_chirho.apply_ty_chirho(&expected_arg_ty_chirho);
                    match self.unify_normalized_chirho(
                        &inferred_arg_sub_chirho,
                        &expected_arg_final_chirho,
                        *app_span_chirho,
                    ) {
                        Ok(mut arg_unify_subst_chirho) => {
                            if self
                                .apply_subst_all_chirho(&mut arg_unify_subst_chirho)
                                .is_err()
                            {
                                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                            };
                            subst_chirho = arg_unify_subst_chirho.compose_chirho(&subst_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    let inferred_result_final_chirho =
                        subst_chirho.apply_ty_chirho(&inferred_result_ty_chirho);
                    let expected_result_final_chirho =
                        subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    match self.unify_normalized_chirho(
                        &inferred_result_final_chirho,
                        &expected_result_final_chirho,
                        *app_span_chirho,
                    ) {
                        Ok(mut result_subst_chirho) => {
                            if self
                                .apply_subst_all_chirho(&mut result_subst_chirho)
                                .is_err()
                            {
                                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                            };
                            subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }

                    let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                    return (subst_chirho, final_ty_chirho);
                }

                let (mut arg_subst_chirho, inferred_arg_ty_chirho) =
                    self.infer_expr_chirho(arg_chirho);
                if self.apply_subst_all_chirho(&mut arg_subst_chirho).is_err() {
                    return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                };
                let mut subst_chirho = arg_subst_chirho.compose_chirho(&fun_subst_chirho);
                let inferred_fun_final_chirho =
                    subst_chirho.apply_ty_chirho(&inferred_fun_sub_chirho);
                let inferred_arg_final_chirho =
                    subst_chirho.apply_ty_chirho(&inferred_arg_ty_chirho);
                let result_ty_chirho = self.fresh_value_meta_chirho(*app_span_chirho);
                let expected_fun_ty_chirho =
                    TyChirho::fun_chirho(inferred_arg_final_chirho, result_ty_chirho.clone());
                match self.unify_normalized_chirho(
                    &inferred_fun_final_chirho,
                    &expected_fun_ty_chirho,
                    *app_span_chirho,
                ) {
                    Ok(mut app_subst_chirho) => {
                        if self.apply_subst_all_chirho(&mut app_subst_chirho).is_err() {
                            return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                        };
                        subst_chirho = app_subst_chirho.compose_chirho(&subst_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        return (subst_chirho, result_ty_chirho);
                    }
                }

                let result_ty_final_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                let expected_result_final_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                match self.unify_normalized_chirho(
                    &result_ty_final_chirho,
                    &expected_result_final_chirho,
                    *app_span_chirho,
                ) {
                    Ok(mut result_subst_chirho) => {
                        if self
                            .apply_subst_all_chirho(&mut result_subst_chirho)
                            .is_err()
                        {
                            return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                        };
                        subst_chirho = result_subst_chirho.compose_chirho(&subst_chirho);
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                    }
                }
                let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }
            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho: infix_span_chirho,
            } => {
                let app_expr_chirho = build_infix_app_expr_chirho(
                    left_chirho,
                    op_chirho,
                    right_chirho,
                    *infix_span_chirho,
                );
                self.infer_expr_against_expected_chirho(
                    &app_expr_chirho,
                    expected_ty_chirho,
                    *infix_span_chirho,
                )
            }
            ExprChirho::ListChirho {
                elements_chirho,
                span_chirho: list_span_chirho,
            } => {
                let expected_norm_chirho = self.normalize_ty_chirho(expected_ty_chirho);
                let TyChirho::ListChirho(expected_elem_ty_chirho) = expected_norm_chirho else {
                    let (s_chirho, actual_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                    let expected_sub_chirho = s_chirho.apply_ty_chirho(expected_ty_chirho);
                    match self.unify_normalized_chirho(
                        &actual_ty_chirho,
                        &expected_sub_chirho,
                        span_chirho,
                    ) {
                        Ok(mut su_chirho) => {
                            if self.apply_subst_all_chirho(&mut su_chirho).is_err() {
                                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                            };
                            let combined_chirho = su_chirho.compose_chirho(&s_chirho);
                            let final_ty_chirho =
                                combined_chirho.apply_ty_chirho(expected_ty_chirho);
                            return (combined_chirho, final_ty_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                            return (s_chirho, actual_ty_chirho);
                        }
                    }
                };

                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut elem_expected_ty_chirho = (*expected_elem_ty_chirho).clone();

                for elem_chirho in elements_chirho {
                    let elem_expected_sub_chirho =
                        subst_chirho.apply_ty_chirho(&elem_expected_ty_chirho);
                    let (mut elem_subst_chirho, elem_ty_chirho) = self
                        .infer_expr_against_expected_chirho(
                            elem_chirho,
                            &elem_expected_sub_chirho,
                            *list_span_chirho,
                        );
                    if self.apply_subst_all_chirho(&mut elem_subst_chirho).is_err() {
                        return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                    };
                    subst_chirho = elem_subst_chirho.compose_chirho(&subst_chirho);

                    let elem_expected_final_chirho =
                        subst_chirho.apply_ty_chirho(&elem_expected_ty_chirho);
                    let elem_actual_final_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    match self.unify_normalized_chirho(
                        &elem_actual_final_chirho,
                        &elem_expected_final_chirho,
                        *list_span_chirho,
                    ) {
                        Ok(mut su_chirho) => {
                            if self.apply_subst_all_chirho(&mut su_chirho).is_err() {
                                return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                            };
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                            elem_expected_ty_chirho =
                                subst_chirho.apply_ty_chirho(&elem_expected_ty_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(expected_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }
            _ => {
                // A `do` block's last statement is checked against the rigid
                // expected type (its alternatives may refine it); the block's
                // own arm consumes the hint.
                // workflow: language-features-chirho/rigid-type-variables-chirho
                if matches!(expr_chirho, ExprChirho::DoChirho { .. })
                    && self.type_contains_skolem_chirho(expected_ty_chirho)
                {
                    self.do_last_expected_chirho = Some(expected_ty_chirho.clone());
                }
                let (s_chirho, actual_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                self.do_last_expected_chirho = None;
                let expected_sub_chirho = s_chirho.apply_ty_chirho(expected_ty_chirho);
                match self.unify_normalized_chirho(
                    &actual_ty_chirho,
                    &expected_sub_chirho,
                    span_chirho,
                ) {
                    Ok(mut su_chirho) => {
                        if self.apply_subst_all_chirho(&mut su_chirho).is_err() {
                            return (SubstChirho::empty_chirho(), expected_ty_chirho.clone());
                        };
                        let combined_chirho = su_chirho.compose_chirho(&s_chirho);
                        let final_ty_chirho = combined_chirho.apply_ty_chirho(expected_ty_chirho);
                        (combined_chirho, final_ty_chirho)
                    }
                    Err(err_chirho) => {
                        self.report_unify_error_chirho(&err_chirho);
                        (s_chirho, actual_ty_chirho)
                    }
                }
            }
        }
    }
}
