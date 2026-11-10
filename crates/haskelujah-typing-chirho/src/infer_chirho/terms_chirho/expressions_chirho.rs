// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Expressions, inferred bottom-up.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{
    RECORD_WILDCARD_FIELD_CODE_CHIRHO, TYPED_HOLE_CODE_CHIRHO, UNBOUND_CONSTRUCTOR_CODE_CHIRHO,
    UNBOUND_VAR_CODE_CHIRHO, infer_lit_chirho, strip_name_qualifier_chirho,
};
use crate::class_chirho::PredChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{MultChirho, TyChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};

impl InferCtxChirho {
    /// The wanted a literal or a prefix negation brings. Its translation names the compiler's
    /// own class whatever is in scope (Report sections 3.2, 3.4 and 6.4.1), so the predicate
    /// carries that proof, and no `IsString` import is needed. Under RebindableSyntax the
    /// literal calls whatever `fromInteger` (and the negation whatever `negate`) is in scope,
    /// and under MagicHash it may be a primitive literal whose `#` the AST erases. Either way
    /// the predicate stays legacy and unproven, a named support boundary (row 692).
    /// workflow: language-features-chirho/fundep-generalization-chirho
    fn literal_predicate_chirho(
        &self,
        identity_chirho: CompilerClassChirho,
        ty_chirho: &TyChirho,
    ) -> PredChirho {
        if self.compiler_literals_chirho {
            PredChirho::compiler_chirho(identity_chirho, ty_chirho.clone())
        } else {
            PredChirho::new_chirho(identity_chirho.legacy_name_chirho(), ty_chirho.clone())
        }
    }

    /// Ordinary numeric syntax selects the compiler class independently of
    /// lexical names. Its sealed closed Type parameter supplies this fresh
    /// Meta's actual kind, only in an owned checked module. OriginId is an
    /// occurrence identity, not a primitive-literal or permission marker.
    /// Workflow: type-identity-chirho/committed-meta-kinds-chirho.md.
    fn fresh_numeric_literal_chirho(
        &mut self,
        identity_chirho: CompilerClassChirho,
        span_chirho: haskelujah_span_chirho::SpanChirho,
    ) -> TyChirho {
        use haskelujah_ast_chirho::name_chirho::{
            CompilerClassGroundKindChirho, CompilerClassKindAtomChirho, CompilerClassKindChirho,
        };
        let schema_chirho = identity_chirho.schema_chirho();
        let closed_type_chirho = schema_chirho.visible_parameter_count_chirho() == 1
            && matches!(schema_chirho.closed_parameter_kind_chirho(0),
                Some(CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::GroundChirho(kind_chirho)))
                if *kind_chirho == CompilerClassGroundKindChirho::TYPE_CHIRHO);
        if !self.compiler_literals_chirho
            || !matches!(
                identity_chirho,
                CompilerClassChirho::NumChirho | CompilerClassChirho::FractionalChirho
            )
            || !closed_type_chirho
            || self.elaboration_state_chirho.owner_chirho.is_none()
            || self.elaboration_state_chirho.source_policy_chirho.is_none()
        {
            return self.fresh_var_chirho();
        }
        match self.elaboration_state_chirho.fresh_chirho(
            Some(crate::kind_chirho::elaboration_chirho::ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        ) {
            Ok(variable_chirho) => TyChirho::VarChirho(variable_chirho),
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                Self::terminal_commit_type_chirho()
            }
        }
    }

    /// Infer the type of an expression, returning (substitution, type).
    pub fn infer_expr_chirho(&mut self, expr_chirho: &ExprChirho) -> (SubstChirho, TyChirho) {
        if self.has_non_deferrable_failure_chirho() {
            return (
                SubstChirho::empty_chirho(),
                Self::terminal_commit_type_chirho(),
            );
        }
        let (substitution_chirho, ty_chirho) = self.infer_expr_inner_chirho(expr_chirho);
        let ty_chirho = self.expose_qualified_wanteds_chirho(ty_chirho, expr_chirho.span_chirho());
        (substitution_chirho, ty_chirho)
    }

    fn infer_expr_inner_chirho(&mut self, expr_chirho: &ExprChirho) -> (SubstChirho, TyChirho) {
        match expr_chirho {
            ExprChirho::LitChirho(lit_chirho) => match lit_chirho {
                LitChirho::IntChirho(_, span_chirho, _) => {
                    let lit_ty_chirho = self
                        .fresh_numeric_literal_chirho(CompilerClassChirho::NumChirho, *span_chirho);
                    if self.has_non_deferrable_failure_chirho() {
                        return (SubstChirho::empty_chirho(), lit_ty_chirho);
                    }
                    let wanted_chirho = self
                        .literal_predicate_chirho(CompilerClassChirho::NumChirho, &lit_ty_chirho);
                    self.capture_literal_evidence_chirho(lit_chirho, &wanted_chirho);
                    self.deferred_preds_chirho
                        .push((wanted_chirho, *span_chirho));
                    (SubstChirho::empty_chirho(), lit_ty_chirho)
                }
                LitChirho::FloatChirho(_, span_chirho, _) => {
                    let lit_ty_chirho = self.fresh_numeric_literal_chirho(
                        CompilerClassChirho::FractionalChirho,
                        *span_chirho,
                    );
                    if self.has_non_deferrable_failure_chirho() {
                        return (SubstChirho::empty_chirho(), lit_ty_chirho);
                    }
                    let wanted_chirho = self.literal_predicate_chirho(
                        CompilerClassChirho::FractionalChirho,
                        &lit_ty_chirho,
                    );
                    self.capture_literal_evidence_chirho(lit_chirho, &wanted_chirho);
                    self.deferred_preds_chirho
                        .push((wanted_chirho, *span_chirho));
                    (SubstChirho::empty_chirho(), lit_ty_chirho)
                }
                LitChirho::StringChirho(_, span_chirho, _) if self.overloaded_strings_chirho => {
                    let lit_ty_chirho = self.fresh_var_chirho();
                    let wanted_chirho = self.literal_predicate_chirho(
                        CompilerClassChirho::IsStringChirho,
                        &lit_ty_chirho,
                    );
                    self.capture_literal_evidence_chirho(lit_chirho, &wanted_chirho);
                    self.deferred_preds_chirho
                        .push((wanted_chirho, *span_chirho));
                    (SubstChirho::empty_chirho(), lit_ty_chirho)
                }
                _ => {
                    let ty_chirho = infer_lit_chirho(lit_chirho);
                    (SubstChirho::empty_chirho(), ty_chirho)
                }
            },

            ExprChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let full_name_chirho = name_chirho.full_name_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &full_name_chirho,
                        text_chirho,
                    );
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        self.recursive_references_chirho.capture_chirho(
                            text_chirho,
                            span_chirho,
                            name_chirho.origin_chirho(),
                            &scheme_chirho,
                        );
                        // Evidence-threading P2: instantiate via parts so each
                        // predicate's fresh class variable can be captured as an
                        // occurrence record (finalized after solving/defaulting).
                        let (ty_chirho, instantiated_preds_chirho) =
                            self.instantiate_scheme_parts_chirho(&scheme_chirho);
                        self.capture_occurrence_predicates_chirho(
                            name_chirho,
                            &instantiated_preds_chirho,
                        );
                        for pred_inst_chirho in instantiated_preds_chirho {
                            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
                        }
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => {
                        // Typed holes (row 695): an unbound unqualified `_` or `_foo` in
                        // expression position is GHC-88464, an error unless the caller defers
                        // typed holes. The driver's deferral policy decides that from real
                        // options. A bound `_foo` took the scheme arm above, and a qualified
                        // spelling is an unbound variable, not a hole. The hole still gets a
                        // fresh type so the rest of the module is checked.
                        // workflow: compiler-pipeline-chirho/error-deferral-chirho
                        if text_chirho.starts_with('?') {
                            // ImplicitParams: `?x` is an implicit parameter.
                            // Assign a fresh type variable so it can unify
                            // with whatever the constraint provides.
                            let ip_ty_chirho = self.fresh_var_chirho();
                            (SubstChirho::empty_chirho(), ip_ty_chirho)
                        } else if text_chirho.starts_with('_')
                            && name_chirho.qualifier_chirho().is_none()
                        {
                            let hole_ty_chirho = self.fresh_var_chirho();
                            let diag_chirho = DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(TYPED_HOLE_CODE_CHIRHO),
                                format!("found hole: `{text_chirho}` :: _"),
                                span_chirho,
                            );
                            self.diagnostics_chirho.push_chirho(diag_chirho);
                            (SubstChirho::empty_chirho(), hole_ty_chirho)
                        } else {
                            let mut diag_chirho = DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(UNBOUND_VAR_CODE_CHIRHO),
                                format!("unbound variable: `{full_name_chirho}`"),
                                span_chirho,
                            );
                            // Add "did you mean?" suggestions from the type env.
                            // Filter candidates: if the target is alphanumeric,
                            // don't suggest operator symbols (and vice versa).
                            let is_alpha_chirho =
                                text_chirho.chars().next().is_some_and(|c_chirho| {
                                    c_chirho.is_alphanumeric() || c_chirho == '_'
                                });
                            let candidates_chirho: Vec<&str> = self
                                .env_chirho
                                .all_names_chirho()
                                .into_iter()
                                .filter(|c_chirho| {
                                    if is_alpha_chirho {
                                        c_chirho.chars().next().is_some_and(|ch_chirho| {
                                            ch_chirho.is_alphanumeric() || ch_chirho == '_'
                                        })
                                    } else {
                                        true
                                    }
                                })
                                .collect();
                            let max_dist_chirho = haskelujah_diagnostics_chirho::suggest_chirho::default_max_distance_chirho(text_chirho.len());
                            let suggestions_chirho = haskelujah_diagnostics_chirho::suggest_chirho::suggest_similar_names_chirho(
                                text_chirho,
                                candidates_chirho,
                                max_dist_chirho,
                                3,
                            );
                            if let Some(note_chirho) = haskelujah_diagnostics_chirho::suggest_chirho::format_did_you_mean_chirho(&suggestions_chirho) {
                                diag_chirho = diag_chirho.with_note_chirho(note_chirho);
                            }
                            self.diagnostics_chirho.push_chirho(diag_chirho);
                            (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                        }
                    }
                }
            }

            ExprChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                let full_name_chirho = name_chirho.full_name_chirho();
                let span_chirho = name_chirho.span_chirho();
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &full_name_chirho,
                        text_chirho,
                    );
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        // Evidence-threading P2: instantiate via parts so each
                        // predicate's fresh class variable can be captured as an
                        // occurrence record (finalized after solving/defaulting).
                        let (ty_chirho, instantiated_preds_chirho) =
                            self.instantiate_scheme_parts_chirho(&scheme_chirho);
                        self.capture_occurrence_predicates_chirho(
                            name_chirho,
                            &instantiated_preds_chirho,
                        );
                        for pred_inst_chirho in instantiated_preds_chirho {
                            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
                        }
                        (SubstChirho::empty_chirho(), ty_chirho)
                    }
                    None => (SubstChirho::empty_chirho(), self.fresh_var_chirho()),
                }
            }

            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                span_chirho,
            } => {
                if let Some(result_chirho) =
                    self.infer_application_spine_chirho(expr_chirho, None, *span_chirho)
                {
                    return result_chirho;
                }
                let (mut s1_chirho, fun_ty_chirho) = self.infer_expr_chirho(fun_chirho);
                if self.apply_subst_all_chirho(&mut s1_chirho).is_err() {
                    return (
                        SubstChirho::empty_chirho(),
                        Self::terminal_commit_type_chirho(),
                    );
                };
                let fun_ty_chirho = s1_chirho.apply_ty_chirho(&fun_ty_chirho);

                if let Some((links_chirho, result_ty_chirho)) = self
                    .infer_required_type_application_chirho(
                        &fun_ty_chirho,
                        arg_chirho,
                        *span_chirho,
                    )
                {
                    return (links_chirho.compose_chirho(&s1_chirho), result_ty_chirho);
                }

                if let ExprChirho::LamChirho {
                    pats_chirho,
                    body_chirho,
                    ..
                } = arg_chirho.as_ref()
                {
                    let expected_arg_ty_chirho = self.fresh_var_chirho();
                    let result_ty_chirho = self.fresh_value_meta_chirho(*span_chirho);
                    let expected_fun_chirho = TyChirho::fun_chirho(
                        expected_arg_ty_chirho.clone(),
                        result_ty_chirho.clone(),
                    );
                    match self.unify_normalized_chirho(
                        &fun_ty_chirho,
                        &expected_fun_chirho,
                        *span_chirho,
                    ) {
                        Ok(mut s2_chirho) => {
                            if self.apply_subst_all_chirho(&mut s2_chirho).is_err() {
                                return (
                                    SubstChirho::empty_chirho(),
                                    Self::terminal_commit_type_chirho(),
                                );
                            };
                            let expected_arg_sub_chirho =
                                s2_chirho.apply_ty_chirho(&expected_arg_ty_chirho);
                            let (s3_chirho, arg_ty_chirho) = self
                                .infer_lambda_against_expected_chirho(
                                    pats_chirho,
                                    body_chirho,
                                    &expected_arg_sub_chirho,
                                    *span_chirho,
                                    crate::infer_chirho::SigmaExpressionKeyChirho::from_expression_chirho(arg_chirho),
                                )
                                .unwrap_or_else(|| self.infer_expr_chirho(arg_chirho));
                            let arg_ty_sub_chirho = s3_chirho.apply_ty_chirho(&arg_ty_chirho);
                            let expected_arg_final_chirho =
                                s3_chirho.apply_ty_chirho(&expected_arg_sub_chirho);
                            match self.unify_normalized_chirho(
                                &arg_ty_sub_chirho,
                                &expected_arg_final_chirho,
                                *span_chirho,
                            ) {
                                Ok(mut s4_chirho) => {
                                    if self.apply_subst_all_chirho(&mut s4_chirho).is_err() {
                                        return (
                                            SubstChirho::empty_chirho(),
                                            Self::terminal_commit_type_chirho(),
                                        );
                                    };
                                    let combined_chirho = s4_chirho.compose_chirho(
                                        &s3_chirho
                                            .compose_chirho(&s2_chirho.compose_chirho(&s1_chirho)),
                                    );
                                    let final_ty_chirho =
                                        combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                    (combined_chirho, final_ty_chirho)
                                }
                                Err(err_chirho) => {
                                    self.report_unify_error_chirho(&err_chirho);
                                    (
                                        s3_chirho
                                            .compose_chirho(&s2_chirho.compose_chirho(&s1_chirho)),
                                        self.fresh_var_chirho(),
                                    )
                                }
                            }
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                            (s1_chirho, self.fresh_var_chirho())
                        }
                    }
                } else {
                    let (s2_chirho, arg_ty_chirho) = self.infer_expr_chirho(arg_chirho);
                    let fun_ty_sub_chirho = s2_chirho.apply_ty_chirho(&fun_ty_chirho);

                    let result_ty_chirho = self.fresh_value_meta_chirho(*span_chirho);
                    let expected_fun_chirho =
                        TyChirho::fun_chirho(arg_ty_chirho, result_ty_chirho.clone());

                    match self.unify_normalized_chirho(
                        &fun_ty_sub_chirho,
                        &expected_fun_chirho,
                        *span_chirho,
                    ) {
                        Ok(mut s3_chirho) => {
                            if self.apply_subst_all_chirho(&mut s3_chirho).is_err() {
                                return (
                                    SubstChirho::empty_chirho(),
                                    Self::terminal_commit_type_chirho(),
                                );
                            };
                            let combined_chirho =
                                s3_chirho.compose_chirho(&s2_chirho.compose_chirho(&s1_chirho));
                            let final_ty_chirho =
                                combined_chirho.apply_ty_chirho(&result_ty_chirho);
                            (combined_chirho, final_ty_chirho)
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                            (
                                s2_chirho.compose_chirho(&s1_chirho),
                                self.fresh_var_chirho(),
                            )
                        }
                    }
                }
            }

            ExprChirho::LamChirho {
                pats_chirho,
                body_chirho,
                span_chirho,
            } => {
                let param_tys_chirho: Vec<TyChirho> = (0..pats_chirho.len())
                    .map(|_| self.fresh_value_parameter_chirho(*span_chirho))
                    .collect();
                let result_ty_chirho = self.fresh_var_chirho();
                self.infer_lambda_with_seed_chirho(
                    pats_chirho,
                    body_chirho,
                    param_tys_chirho,
                    result_ty_chirho,
                    *span_chirho,
                )
            }

            ExprChirho::TypeLamChirho {
                binder_chirho,
                body_chirho,
                ..
            } => {
                let ty_var_chirho = self.surface_binder_variable_chirho(
                    binder_chirho,
                    &mut std::collections::HashMap::new(),
                );
                let preceding_chirho = self.scoped_tyvars_chirho.clone();
                let kind_chirho = self.surface_quantifier_kind_chirho(
                    binder_chirho,
                    &preceding_chirho,
                    &mut super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho(),
                );
                let binder_text_chirho = binder_chirho.text_chirho().to_string();
                let previous_chirho = self
                    .scoped_tyvars_chirho
                    .insert(binder_text_chirho.clone(), ty_var_chirho);
                let (subst_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                if let Some(previous_chirho) = previous_chirho {
                    self.scoped_tyvars_chirho
                        .insert(binder_text_chirho, previous_chirho);
                } else {
                    self.scoped_tyvars_chirho.remove(&binder_text_chirho);
                }
                (
                    subst_chirho.clone(),
                    TyChirho::ForallChirho {
                        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![crate::ty_chirho::QuantifiedVariableChirho {
                            variable_chirho: ty_var_chirho,
                            source_chirho: binder_chirho.type_variable_chirho().cloned(),
                            specificity_chirho: Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho),
                            kind_chirho,
                        }]),
                        body_chirho: Box::new(subst_chirho.apply_ty_chirho(&body_ty_chirho)),
                    },
                )
            }

            ExprChirho::MultiWayIfChirho { arms_chirho, .. } => {
                self.infer_multiway_if_chirho(arms_chirho, None)
            }

            ExprChirho::IfChirho {
                cond_chirho,
                then_chirho,
                else_chirho,
                span_chirho,
            } => self.infer_if_chirho(cond_chirho, then_chirho, else_chirho, *span_chirho),

            ExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                let type_pattern_scope_mark_chirho = self.type_pattern_alias_undo_chirho.len();
                self.env_chirho.push_scope_chirho();
                let mut subst_chirho = SubstChirho::empty_chirho();
                self.infer_local_binds_chirho(binds_chirho, &mut subst_chirho);

                let (body_s_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                self.restore_type_pattern_aliases_chirho(type_pattern_scope_mark_chirho);
                self.env_chirho.pop_scope_chirho();

                let combined_chirho = body_s_chirho.compose_chirho(&subst_chirho);
                (combined_chirho, body_ty_chirho)
            }

            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut elem_tys_chirho = Vec::new();

                for elem_chirho in elements_chirho {
                    let (mut s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    if self.apply_subst_all_chirho(&mut s_chirho).is_err() {
                        return (
                            SubstChirho::empty_chirho(),
                            Self::terminal_commit_type_chirho(),
                        );
                    };
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);
                    elem_tys_chirho.push(ty_chirho);
                }

                // Apply accumulated substitution to element types
                let final_elems_chirho: Vec<TyChirho> = elem_tys_chirho
                    .iter()
                    .map(|t_chirho| subst_chirho.apply_ty_chirho(t_chirho))
                    .collect();

                (subst_chirho, TyChirho::TupleChirho(final_elems_chirho))
            }

            ExprChirho::ArithSeqChirho {
                from_chirho,
                then_chirho,
                to_chirho,
                span_chirho,
            } => self.infer_range_chirho(
                from_chirho,
                then_chirho.as_deref(),
                to_chirho.as_deref(),
                *span_chirho,
            ),

            ExprChirho::ListChirho {
                elements_chirho,
                span_chirho,
            } => {
                let elem_ty_chirho = self.fresh_list_element_meta_chirho(*span_chirho);
                let mut subst_chirho = SubstChirho::empty_chirho();

                for elem_chirho in elements_chirho {
                    let (mut s_chirho, ty_chirho) = self.infer_expr_chirho(elem_chirho);
                    if self.apply_subst_all_chirho(&mut s_chirho).is_err() {
                        return (
                            SubstChirho::empty_chirho(),
                            Self::terminal_commit_type_chirho(),
                        );
                    };
                    subst_chirho = s_chirho.compose_chirho(&subst_chirho);

                    let elem_sub_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    match self.unify_normalized_chirho(&elem_sub_chirho, &ty_chirho, *span_chirho) {
                        Ok(mut su_chirho) => {
                            if self.apply_subst_all_chirho(&mut su_chirho).is_err() {
                                return (
                                    SubstChirho::empty_chirho(),
                                    Self::terminal_commit_type_chirho(),
                                );
                            };
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                let final_elem_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                (
                    subst_chirho,
                    TyChirho::ListChirho(Box::new(final_elem_chirho)),
                )
            }

            ExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                span_chirho,
            } => {
                let (mut s1_chirho, scrut_ty_chirho) = self.infer_expr_chirho(scrutinee_chirho);
                if self.apply_subst_all_chirho(&mut s1_chirho).is_err() {
                    return (
                        SubstChirho::empty_chirho(),
                        Self::terminal_commit_type_chirho(),
                    );
                };

                let result_ty_chirho = self.fresh_var_chirho();
                let mut subst_chirho = s1_chirho;

                for alt_chirho in alts_chirho {
                    let type_pattern_scope_mark_chirho = self.type_pattern_alias_undo_chirho.len();
                    self.env_chirho.push_scope_chirho();
                    let pattern_frame_chirho = self.begin_pattern_implication_with_roots_chirho(
                        [&scrut_ty_chirho, &result_ty_chirho],
                        alt_chirho.span_chirho,
                    );
                    if self.has_non_deferrable_failure_chirho() {
                        return (SubstChirho::empty_chirho(), result_ty_chirho);
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

                    let (mut sr_chirho, alt_ty_chirho) =
                        self.infer_rhs_chirho(&alt_chirho.rhs_chirho);
                    if self.apply_subst_all_chirho(&mut sr_chirho).is_err() {
                        return (
                            SubstChirho::empty_chirho(),
                            Self::terminal_commit_type_chirho(),
                        );
                    };
                    subst_chirho = sr_chirho.compose_chirho(&subst_chirho);

                    // Every alternative must meet the common result under
                    // its own evidence. A later signature sees only the shared
                    // result and cannot recover an ignored branch mismatch.
                    let result_sub_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                    match self.unify_normalized_chirho(
                        &result_sub_chirho,
                        &alt_ty_chirho,
                        alt_chirho.span_chirho,
                    ) {
                        Ok(mut sa_chirho) => {
                            if self.apply_subst_all_chirho(&mut sa_chirho).is_err() {
                                return (
                                    SubstChirho::empty_chirho(),
                                    Self::terminal_commit_type_chirho(),
                                );
                            };
                            subst_chirho = sa_chirho.compose_chirho(&subst_chirho);
                        }
                        Err(error_chirho) => self.report_unify_error_chirho(&error_chirho),
                    }

                    let solved_chirho = self.end_pattern_implication_chirho(
                        pattern_frame_chirho,
                        &subst_chirho.apply_ty_chirho(&result_ty_chirho),
                        *span_chirho,
                    );
                    subst_chirho = solved_chirho.compose_chirho(&subst_chirho);
                    self.restore_type_pattern_aliases_chirho(type_pattern_scope_mark_chirho);
                    self.env_chirho.pop_scope_chirho();
                }

                let final_ty_chirho = subst_chirho.apply_ty_chirho(&result_ty_chirho);
                (subst_chirho, final_ty_chirho)
            }

            ExprChirho::DoChirho {
                stmts_chirho,
                span_chirho,
                ..
            } => self.infer_do_block_chirho(stmts_chirho, *span_chirho),

            ExprChirho::ListCompChirho {
                body_chirho,
                quals_chirho,
                parallel_quals_chirho,
                span_chirho,
            } => {
                let type_pattern_scope_mark_chirho = self.type_pattern_alias_undo_chirho.len();
                let mut body_aliases_chirho = Vec::new();
                let mut subst_chirho = SubstChirho::empty_chirho();
                let mut body_bindings_chirho = Vec::new();
                if parallel_quals_chirho.is_empty() {
                    let (group_subst_chirho, group_bindings_chirho, group_aliases_chirho) =
                        self.infer_list_comp_qual_group_chirho(quals_chirho, *span_chirho);
                    subst_chirho = group_subst_chirho.compose_chirho(&subst_chirho);
                    body_bindings_chirho.extend(group_bindings_chirho);
                    body_aliases_chirho.extend(group_aliases_chirho);
                } else {
                    for group_chirho in parallel_quals_chirho {
                        let (group_subst_chirho, group_bindings_chirho, group_aliases_chirho) =
                            self.infer_list_comp_qual_group_chirho(group_chirho, *span_chirho);
                        subst_chirho = group_subst_chirho.compose_chirho(&subst_chirho);
                        body_bindings_chirho.extend(group_bindings_chirho);
                        body_aliases_chirho.extend(group_aliases_chirho);
                    }
                }

                self.env_chirho.push_scope_chirho();
                for (name_chirho, variable_chirho, image_chirho) in body_aliases_chirho {
                    self.install_type_pattern_alias_image_chirho(
                        name_chirho,
                        variable_chirho,
                        image_chirho,
                    );
                }
                for (name_chirho, scheme_chirho) in body_bindings_chirho {
                    self.env_chirho.bind_chirho(
                        name_chirho,
                        subst_chirho.apply_scheme_chirho(&scheme_chirho),
                    );
                }
                let (mut body_subst_chirho, body_ty_chirho) = self.infer_expr_chirho(body_chirho);
                if self.apply_subst_all_chirho(&mut body_subst_chirho).is_err() {
                    return (
                        SubstChirho::empty_chirho(),
                        Self::terminal_commit_type_chirho(),
                    );
                };
                subst_chirho = body_subst_chirho.compose_chirho(&subst_chirho);
                self.restore_type_pattern_aliases_chirho(type_pattern_scope_mark_chirho);
                self.env_chirho.pop_scope_chirho();

                let final_elem_ty_chirho = subst_chirho.apply_ty_chirho(&body_ty_chirho);
                (
                    subst_chirho,
                    TyChirho::ListChirho(Box::new(final_elem_ty_chirho)),
                )
            }

            ExprChirho::NegChirho {
                expr_chirho,
                span_chirho,
            } => {
                let (s_chirho, expr_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                // `-e` is the Prelude's `negate e` (Report section 3.4): the compiler's own Num,
                // under the same safety flags as an ordinary literal (row 692).
                let wanted_chirho =
                    self.literal_predicate_chirho(CompilerClassChirho::NumChirho, &expr_ty_chirho);
                self.deferred_preds_chirho
                    .push((wanted_chirho, *span_chirho));
                (s_chirho, expr_ty_chirho)
            }

            ExprChirho::ParenChirho { inner_chirho, .. } => self.infer_expr_chirho(inner_chirho),

            ExprChirho::LeftSectionChirho {
                op_chirho,
                arg_chirho,
                span_chirho,
            }
            | ExprChirho::RightSectionChirho {
                op_chirho,
                arg_chirho,
                span_chirho,
            } => {
                let lambda_chirho = self.build_section_lambda_chirho(
                    op_chirho,
                    arg_chirho,
                    matches!(expr_chirho, ExprChirho::RightSectionChirho { .. }),
                    *span_chirho,
                );
                self.infer_expr_chirho(&lambda_chirho)
            }

            ExprChirho::AnnChirho {
                expr_chirho,
                ty_chirho: ann_ty_chirho,
                span_chirho,
            } => {
                let (substitution_chirho, scheme_chirho) = self.check_expression_annotation_chirho(
                    expr_chirho,
                    ann_ty_chirho,
                    *span_chirho,
                );
                let result_chirho = self.instantiate_chirho(&scheme_chirho, *span_chirho);
                (substitution_chirho, result_chirho)
            }

            // Infix application: `a op b` is treated as `(op a) b`
            ExprChirho::InfixChirho {
                left_chirho,
                op_chirho,
                right_chirho,
                span_chirho,
            } => {
                let app_expr_chirho =
                    build_infix_app_expr_chirho(left_chirho, op_chirho, right_chirho, *span_chirho);
                self.infer_expr_chirho(&app_expr_chirho)
            }

            // Record construction: Con { f1 = e1, f2 = e2, ... }
            // Treated as constructor application: infer constructor type,
            // then unify each field value with the successive argument types.
            ExprChirho::RecordConChirho {
                con_chirho,
                fields_chirho,
                has_wildcard_chirho,
                span_chirho,
            } => {
                let con_text_chirho = con_chirho.text_chirho();
                let con_full_name_chirho = con_chirho.full_name_chirho();
                self.report_blocked_field_account_chirho(
                    &con_full_name_chirho,
                    !fields_chirho.is_empty(),
                    *span_chirho,
                );
                let scheme_opt_chirho = self
                    .lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                        &con_full_name_chirho,
                        con_text_chirho,
                    );
                match scheme_opt_chirho {
                    Some(scheme_chirho) => {
                        let con_span_chirho = con_chirho.span_chirho();
                        let mut con_ty_chirho =
                            self.instantiate_chirho(&scheme_chirho, con_span_chirho);
                        let mut combined_chirho = SubstChirho::empty_chirho();
                        if let Some((ordered_field_names_chirho, _, _)) =
                            self.lookup_record_constructor_field_bundle_chirho(con_chirho)
                        {
                            // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                            self.report_record_construction_fields_chirho(
                                con_text_chirho,
                                &ordered_field_names_chirho,
                                fields_chirho,
                                *has_wildcard_chirho,
                                *span_chirho,
                            );
                            for field_name_chirho in &ordered_field_names_chirho {
                                let matched_field_chirho =
                                    fields_chirho.iter().find(|field_chirho| {
                                        strip_name_qualifier_chirho(
                                            &field_chirho.name_chirho.full_name_chirho(),
                                        ) == field_name_chirho
                                    });
                                let (s_chirho, field_ty_chirho) = if let Some(field_chirho) =
                                    matched_field_chirho
                                {
                                    self.infer_record_constructor_field_chirho(
                                        &field_chirho.value_chirho,
                                        &con_ty_chirho,
                                        field_chirho.span_chirho,
                                    )
                                } else if *has_wildcard_chirho {
                                    let var_scheme_chirho =
                                        self.env_chirho.lookup_chirho(field_name_chirho).cloned();
                                    if let Some(scheme_chirho) = var_scheme_chirho {
                                        (
                                            SubstChirho::empty_chirho(),
                                            self.instantiate_chirho(&scheme_chirho, *span_chirho),
                                        )
                                    } else {
                                        self.diagnostics_chirho.push_chirho(
                                            DiagnosticChirho::error_with_code_chirho(
                                                ErrorCodeChirho::error_chirho(
                                                    RECORD_WILDCARD_FIELD_CODE_CHIRHO,
                                                ),
                                                format!(
                                                    "RecordWildCards: variable `{}` not in scope",
                                                    field_name_chirho
                                                ),
                                                *span_chirho,
                                            ),
                                        );
                                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                                    }
                                } else {
                                    // Haskell record construction is keyed by field label, not by
                                    // source order. Consume omitted fields with fresh types so
                                    // later named fields still align to the constructor layout.
                                    (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                                };
                                combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                                con_ty_chirho = combined_chirho.apply_ty_chirho(&con_ty_chirho);
                                let result_ty_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::FunChirho(
                                    Box::new(field_ty_chirho),
                                    Box::new(result_ty_chirho.clone()),
                                    MultChirho::ManyChirho,
                                );
                                match self.unify_normalized_chirho(
                                    &con_ty_chirho,
                                    &expected_chirho,
                                    *span_chirho,
                                ) {
                                    Ok(mut s2_chirho) => {
                                        if self.apply_subst_all_chirho(&mut s2_chirho).is_err() {
                                            return (
                                                SubstChirho::empty_chirho(),
                                                Self::terminal_commit_type_chirho(),
                                            );
                                        };
                                        combined_chirho =
                                            s2_chirho.compose_chirho(&combined_chirho);
                                        con_ty_chirho =
                                            combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                    }
                                    Err(err_chirho) => {
                                        self.report_unify_error_chirho(&err_chirho);
                                        return (combined_chirho, self.fresh_var_chirho());
                                    }
                                }
                            }
                            (combined_chirho, con_ty_chirho)
                        } else {
                            // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                            // No declared field set. `C {}` supplies nothing, so every
                            // argument is a missing field and the construction has the
                            // constructor's result type; named fields on a local
                            // positional constructor are all undeclared (E0206).
                            if fields_chirho.is_empty() && !*has_wildcard_chirho {
                                self.report_omitted_strict_positional_chirho(
                                    con_text_chirho,
                                    *span_chirho,
                                );
                                let result_ty_chirho =
                                    self.constructor_result_type_chirho(con_ty_chirho);
                                return (combined_chirho, result_ty_chirho);
                            }
                            if self.report_fields_on_positional_constructor_chirho(
                                &con_full_name_chirho,
                                fields_chirho,
                            ) {
                                let result_ty_chirho =
                                    self.constructor_result_type_chirho(con_ty_chirho);
                                return (combined_chirho, result_ty_chirho);
                            }
                            for field_chirho in fields_chirho {
                                let (s_chirho, field_ty_chirho) = self
                                    .infer_record_constructor_field_chirho(
                                        &field_chirho.value_chirho,
                                        &con_ty_chirho,
                                        field_chirho.span_chirho,
                                    );
                                combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                                con_ty_chirho = combined_chirho.apply_ty_chirho(&con_ty_chirho);
                                let result_ty_chirho = self.fresh_var_chirho();
                                let expected_chirho = TyChirho::FunChirho(
                                    Box::new(field_ty_chirho),
                                    Box::new(result_ty_chirho.clone()),
                                    MultChirho::ManyChirho,
                                );
                                match self.unify_normalized_chirho(
                                    &con_ty_chirho,
                                    &expected_chirho,
                                    *span_chirho,
                                ) {
                                    Ok(mut s2_chirho) => {
                                        if self.apply_subst_all_chirho(&mut s2_chirho).is_err() {
                                            return (
                                                SubstChirho::empty_chirho(),
                                                Self::terminal_commit_type_chirho(),
                                            );
                                        };
                                        combined_chirho =
                                            s2_chirho.compose_chirho(&combined_chirho);
                                        con_ty_chirho =
                                            combined_chirho.apply_ty_chirho(&result_ty_chirho);
                                    }
                                    Err(err_chirho) => {
                                        self.report_unify_error_chirho(&err_chirho);
                                        return (combined_chirho, self.fresh_var_chirho());
                                    }
                                }
                            }
                            (combined_chirho, con_ty_chirho)
                        }
                    }
                    None => {
                        // A record construction needs a data constructor in scope;
                        // `Int{}` names a type, not a constructor (T23739c). The
                        // Var arm reports a scheme-less name the same way. Its own code keeps
                        // `-fdefer-out-of-scope-variables`, which reaches only variables, off it.
                        // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                        self.diagnostics_chirho.push_chirho(
                            DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(UNBOUND_CONSTRUCTOR_CODE_CHIRHO),
                                format!("unbound constructor: `{con_text_chirho}`"),
                                con_chirho.span_chirho(),
                            ),
                        );
                        (SubstChirho::empty_chirho(), self.fresh_var_chirho())
                    }
                }
            }

            // Record update: expr { f1 = e1, ... }
            // For now, infer the expression type and pass through
            ExprChirho::RecordUpdateChirho {
                expr_chirho,
                fields_chirho,
                span_chirho,
            } => {
                let (s1_chirho, base_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                let mut combined_chirho = s1_chirho;
                if self.apply_subst_all_chirho(&mut combined_chirho).is_err() {
                    return (
                        SubstChirho::empty_chirho(),
                        Self::terminal_commit_type_chirho(),
                    );
                };
                let base_ty_chirho = combined_chirho.apply_ty_chirho(&base_ty_chirho);
                if let Some((s_chirho, result_ty_chirho)) = self.infer_record_update_fields_chirho(
                    &base_ty_chirho,
                    fields_chirho,
                    *span_chirho,
                ) {
                    combined_chirho = s_chirho.compose_chirho(&combined_chirho);
                    return (combined_chirho, result_ty_chirho);
                }
                // No constructor declares all the updated fields. When the
                // record type is one this module declares, that is GHC's
                // "constructor does not have field" error; an imported or
                // unknown type keeps the permissive path below.
                // workflow: language-features-chirho/rigid-type-variables-chirho (records)
                self.report_undeclared_record_fields_chirho(&base_ty_chirho, fields_chirho);
                // Unknown constructor layout: type-check the values and keep the
                // record's type.
                for field_chirho in fields_chirho {
                    let (mut fs_chirho, _field_ty_chirho) =
                        self.infer_expr_chirho(&field_chirho.value_chirho);
                    if self.apply_subst_all_chirho(&mut fs_chirho).is_err() {
                        return (
                            SubstChirho::empty_chirho(),
                            Self::terminal_commit_type_chirho(),
                        );
                    };
                    combined_chirho = fs_chirho.compose_chirho(&combined_chirho);
                }
                (combined_chirho, base_ty_chirho)
            }

            // TypeApplications consume an actual specified quantifier and
            // compare the producer-checked kind before ordinary instantiation.
            ExprChirho::TypeAppChirho {
                expr_chirho: inner_expr_chirho,
                ty_chirho,
                span_chirho,
            } => {
                if let Some(result_chirho) =
                    self.infer_named_type_app_chirho(expr_chirho, *span_chirho)
                {
                    return result_chirho;
                }

                let (s1_chirho, inferred_chirho) = self
                    .infer_application_sigma_chirho(inner_expr_chirho, *span_chirho)
                    .unwrap_or_else(|| self.infer_expr_chirho(inner_expr_chirho));
                let inferred_chirho =
                    self.expose_qualified_wanteds_chirho(inferred_chirho, *span_chirho);
                let (target_chirho, kind_chirho) = self.check_body_type_argument_chirho(ty_chirho);
                let inferred_sub_chirho = s1_chirho.apply_ty_chirho(&inferred_chirho);
                if let Some(result_chirho) = self.apply_visible_type_arg_chirho(
                    &inferred_sub_chirho,
                    target_chirho,
                    kind_chirho.as_ref(),
                    *span_chirho,
                ) {
                    return (s1_chirho, result_chirho);
                }
                self.diagnostics_chirho.push_chirho(
                    haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                        haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(
                            super::TYPE_MISMATCH_CODE_CHIRHO,
                        ),
                        "no specified type binder remains for visible type application",
                        *span_chirho,
                    ),
                );
                (s1_chirho, inferred_chirho)
            }

            // For remaining expression forms, return a fresh variable
            _ => (SubstChirho::empty_chirho(), self.fresh_var_chirho()),
        }
    }
}

pub(super) fn build_infix_app_expr_chirho(
    left_chirho: &ExprChirho,
    op_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    right_chirho: &ExprChirho,
    span_chirho: haskelujah_span_chirho::SpanChirho,
) -> ExprChirho {
    let right_arg_chirho = match right_chirho {
        ExprChirho::LamChirho { .. } | ExprChirho::TypeLamChirho { .. } => {
            ExprChirho::ParenChirho {
                inner_chirho: Box::new(right_chirho.clone()),
                span_chirho,
            }
        }
        _ => right_chirho.clone(),
    };
    ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(op_chirho.clone())),
            arg_chirho: Box::new(left_chirho.clone()),
            span_chirho,
        }),
        arg_chirho: Box::new(right_arg_chirho),
        span_chirho,
    }
}
