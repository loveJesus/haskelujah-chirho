// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Scheme lookups, and visible or required type applications.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{canonical_value_name_text_chirho, is_placeholder_import_scheme_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho};
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::NameChirho;
use haskelujah_ast_chirho::promotion_chirho::unquoted_promoted_spelling_chirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

impl InferCtxChirho {
    pub(super) fn lookup_value_scheme_with_qualified_suffix_fallback_chirho(
        &self,
        full_name_chirho: &str,
        bare_name_chirho: &str,
    ) -> Option<SchemeChirho> {
        self.lookup_value_scheme_keyed_chirho(full_name_chirho, bare_name_chirho)
            .map(|(_entry_chirho, scheme_chirho)| scheme_chirho)
    }

    /// The scheme a written value name resolves to, with the qualified environment entry
    /// that resolved it: the name's own qualified spelling, or the one entry it is a suffix
    /// of. An unqualified name, and a qualified name that falls back to its bare outermost
    /// binding, carry no entry. An occurrence capture keeps the entry, so the evidence join
    /// judges a proof against the scheme its use took, not the bare spelling: `P.print`
    /// beside a program's own `print` (row 698).
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub(super) fn lookup_value_scheme_keyed_chirho(
        &self,
        full_name_chirho: &str,
        bare_name_chirho: &str,
    ) -> Option<(Option<String>, SchemeChirho)> {
        let canonical_full_name_chirho = canonical_value_name_text_chirho(full_name_chirho);
        let canonical_bare_name_chirho = canonical_value_name_text_chirho(bare_name_chirho);
        if self
            .env_chirho
            .is_masked_chirho(&canonical_full_name_chirho)
        {
            return None;
        }
        if canonical_full_name_chirho == canonical_bare_name_chirho {
            return self
                .env_chirho
                .lookup_chirho(&canonical_bare_name_chirho)
                .map(|scheme_chirho| (None, scheme_chirho.clone()));
        }

        if let Some(scheme_chirho) = self.env_chirho.lookup_chirho(&canonical_full_name_chirho) {
            return Some((Some(canonical_full_name_chirho), scheme_chirho.clone()));
        }

        let suffix_chirho = format!(".{canonical_full_name_chirho}");
        let mut suffix_matches_chirho = self
            .env_chirho
            .all_bindings_chirho()
            .into_iter()
            .filter(|(name_chirho, _scheme_chirho)| name_chirho.ends_with(&suffix_chirho))
            .map(|(name_chirho, scheme_chirho)| (Some(name_chirho.clone()), scheme_chirho.clone()));
        let first_match_chirho = suffix_matches_chirho.next();
        if first_match_chirho.is_some() && suffix_matches_chirho.next().is_none() {
            return first_match_chirho;
        }

        // A qualified name never falls back to a local binder: `L.x` beside a
        // `let x` named the local (row 598).
        self.env_chirho
            .lookup_outermost_chirho(&canonical_bare_name_chirho)
            .map(|scheme_chirho| (None, scheme_chirho.clone()))
    }

    pub(super) fn lookup_record_constructor_field_bundle_chirho(
        &mut self,
        con_chirho: &NameChirho,
    ) -> Option<(Vec<String>, Vec<TyChirho>, TyChirho)> {
        let con_text_chirho = con_chirho.text_chirho();
        let con_full_name_chirho = con_chirho.full_name_chirho();
        let scheme_chirho = self.lookup_value_scheme_with_qualified_suffix_fallback_chirho(
            &con_full_name_chirho,
            con_text_chirho,
        )?;
        if is_placeholder_import_scheme_chirho(&scheme_chirho) {
            return None;
        }

        let mut field_names_chirho = self
            .con_field_names_chirho
            .get(&con_full_name_chirho)
            .and_then(|scope_chirho| scope_chirho.physical_labels_chirho())
            .filter(|labels_chirho| !labels_chirho.is_empty())
            .map(|labels_chirho| labels_chirho.to_vec())?;
        let instantiated_chirho = self.instantiate_chirho(&scheme_chirho, con_chirho.span_chirho());
        let mut remaining_ty_chirho = self.open_constructor_forall_chirho(instantiated_chirho);
        let mut field_tys_chirho = Vec::new();
        while field_tys_chirho.len() < field_names_chirho.len() {
            match remaining_ty_chirho {
                TyChirho::FunChirho(arg_ty_chirho, result_ty_chirho, _) => {
                    field_tys_chirho.push(*arg_ty_chirho);
                    remaining_ty_chirho = *result_ty_chirho;
                }
                _ => break,
            }
        }
        field_names_chirho.truncate(field_tys_chirho.len());
        Some((field_names_chirho, field_tys_chirho, remaining_ty_chirho))
    }

    pub(super) fn peel_type_app_chain_chirho<'a>(
        expr_chirho: &'a ExprChirho,
        type_args_chirho: &mut Vec<&'a TypeChirho>,
    ) -> &'a ExprChirho {
        match expr_chirho {
            ExprChirho::ParenChirho { inner_chirho, .. } => {
                Self::peel_type_app_chain_chirho(inner_chirho, type_args_chirho)
            }
            ExprChirho::TypeAppChirho {
                expr_chirho,
                ty_chirho,
                ..
            } => {
                let base_chirho = Self::peel_type_app_chain_chirho(expr_chirho, type_args_chirho);
                type_args_chirho.push(ty_chirho);
                base_chirho
            }
            _ => expr_chirho,
        }
    }

    pub(super) fn infer_named_type_app_chirho(
        &mut self,
        expr_chirho: &ExprChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        let mut type_args_ast_chirho = Vec::new();
        let base_expr_chirho =
            Self::peel_type_app_chain_chirho(expr_chirho, &mut type_args_ast_chirho);
        if type_args_ast_chirho.is_empty() {
            return None;
        }

        if let ExprChirho::ConChirho(name_chirho) = base_expr_chirho
            && let Some(result_chirho) = self
                .instantiate_checked_constructor_type_application_chirho(
                    name_chirho,
                    &type_args_ast_chirho,
                    span_chirho,
                )
        {
            return Some(result_chirho);
        }

        let (carried_chirho, scheme_chirho) = match base_expr_chirho {
            ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => (
                SubstChirho::empty_chirho(),
                self.lookup_value_scheme_with_qualified_suffix_fallback_chirho(
                    &name_chirho.full_name_chirho(),
                    name_chirho.text_chirho(),
                )?,
            ),
            ExprChirho::AnnChirho {
                expr_chirho,
                ty_chirho,
                span_chirho,
            } => self.check_expression_annotation_chirho(expr_chirho, ty_chirho, *span_chirho),
            _ => return None,
        };

        // A name known only through a placeholder import scheme has no
        // quantifiers to apply type arguments to; its type stays unknown.
        if matches!(
            base_expr_chirho,
            ExprChirho::VarChirho(_) | ExprChirho::ConChirho(_)
        ) && is_placeholder_import_scheme_chirho(&scheme_chirho)
        {
            let fresh_chirho = self.fresh_var_chirho();
            return Some((SubstChirho::empty_chirho(), fresh_chirho));
        }
        let (explicit_tys_chirho, explicit_kinds_chirho): (Vec<_>, Vec<_>) = type_args_ast_chirho
            .into_iter()
            .map(|argument_chirho| self.check_body_type_argument_chirho(argument_chirho))
            .unzip();
        let Some((instantiated_ty_chirho, instantiated_preds_chirho)) = self
            .instantiate_scheme_parts_with_visible_type_args_chirho(
                &scheme_chirho,
                &explicit_tys_chirho,
                &explicit_kinds_chirho,
                span_chirho,
            )
        else {
            self.diagnostics_chirho.push_chirho(
                haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                    haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(
                        super::TYPE_MISMATCH_CODE_CHIRHO,
                    ),
                    "no specified type binder remains for visible type application",
                    span_chirho,
                ),
            );
            let result_chirho = self.fresh_var_chirho();
            return Some((carried_chirho, result_chirho));
        };
        for pred_inst_chirho in instantiated_preds_chirho {
            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
        }
        Some((carried_chirho, instantiated_ty_chirho))
    }

    /// The kind phase retains its checked result on the existing type syntax.
    /// An absent annotation supplies no ordinary kind proof.
    pub(super) fn checked_type_argument_kind_chirho(
        &mut self,
        argument_chirho: &TypeChirho,
    ) -> Option<TyChirho> {
        match argument_chirho {
            TypeChirho::KindAnnotationChirho { kind_chirho, .. } => {
                Some(self.ast_type_to_ty_in_scope_chirho(kind_chirho))
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.checked_type_argument_kind_chirho(inner_chirho)
            }
            _ => None,
        }
    }

    fn instantiate_checked_constructor_type_application_chirho(
        &mut self,
        name_chirho: &NameChirho,
        arguments_chirho: &[&TypeChirho],
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        use crate::ty_chirho::declarations_chirho::TelescopeVisibilityChirho;
        let resolution_chirho = self
            .promotion_scope_chirho
            .as_ref()?
            .quoted_chirho(name_chirho);
        let header_chirho = self
            .type_families_chirho
            .declaration_schemes_chirho()
            .get_chirho(resolution_chirho.key_chirho().as_str_chirho())
            .filter(|header_chirho| {
                header_chirho.owner_chirho.is_none()
                    && header_chirho.promoted_chirho.is_some()
                    && !header_chirho.held_chirho
            })?
            .clone();
        if !arguments_chirho.iter().all(|argument_chirho| {
            matches!(argument_chirho, TypeChirho::KindAnnotationChirho { .. })
        }) {
            return None;
        }
        if header_chirho.binders_chirho.iter().any(|binder_chirho| {
            binder_chirho.visibility_chirho != TelescopeVisibilityChirho::RequiredChirho
                && binder_chirho.term_specificity_chirho.is_none()
        }) {
            return None;
        }
        let mut parameters_chirho = SubstChirho::empty_chirho();
        for binder_chirho in &header_chirho.binders_chirho {
            parameters_chirho.insert_chirho(binder_chirho.variable_chirho, self.fresh_var_chirho());
        }
        let mut solver_chirho = SubstChirho::empty_chirho();
        let mut cursor_chirho = 0;
        for argument_chirho in arguments_chirho {
            while header_chirho
                .binders_chirho
                .get(cursor_chirho)
                .is_some_and(|binder_chirho| {
                    binder_chirho.term_specificity_chirho == Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho)
                })
            {
                cursor_chirho += 1;
            }
            let binder_chirho =
                header_chirho
                    .binders_chirho
                    .get(cursor_chirho)
                    .filter(|binder_chirho| {
                        binder_chirho.term_specificity_chirho
                            == Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho)
                    })?;
            let TypeChirho::KindAnnotationChirho { kind_chirho, .. } = argument_chirho else {
                return None;
            };
            let (actual_chirho, actual_kind_chirho) =
                self.check_body_type_argument_chirho(argument_chirho);
            let actual_kind_chirho = actual_kind_chirho
                .unwrap_or_else(|| self.ast_type_to_ty_in_scope_chirho(kind_chirho));
            let expected_kind_chirho = solver_chirho
                .apply_ty_chirho(&parameters_chirho.apply_ty_chirho(&binder_chirho.kind_chirho));
            let expected_chirho = solver_chirho
                .apply_ty_chirho(parameters_chirho.lookup_chirho(&binder_chirho.variable_chirho)?);
            for (left_chirho, right_chirho) in [
                (expected_kind_chirho, actual_kind_chirho),
                (expected_chirho, actual_chirho),
            ] {
                match self.unify_normalized_chirho(&left_chirho, &right_chirho, span_chirho) {
                    Ok(next_chirho) => {
                        solver_chirho = next_chirho.compose_chirho(&solver_chirho);
                        self.apply_subst_all_chirho(&next_chirho);
                    }
                    Err(error_chirho) => self.report_unify_error_chirho(&error_chirho),
                }
            }
            cursor_chirho += 1;
        }
        let mut result_chirho = solver_chirho
            .apply_ty_chirho(&parameters_chirho.apply_ty_chirho(&header_chirho.result_kind_chirho));
        for binder_chirho in header_chirho
            .binders_chirho
            .iter()
            .rev()
            .filter(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
            })
        {
            let field_chirho = solver_chirho
                .apply_ty_chirho(&parameters_chirho.apply_ty_chirho(&binder_chirho.kind_chirho));
            result_chirho = TyChirho::fun_chirho(field_chirho, result_chirho);
        }
        Some((solver_chirho, result_chirho))
    }

    pub(super) fn expr_to_required_type_arg_chirho(
        &self,
        expr_chirho: &ExprChirho,
    ) -> Option<TyChirho> {
        match expr_chirho {
            ExprChirho::VarChirho(name_chirho) | ExprChirho::ConChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                Some(
                    self.scoped_tyvars_chirho
                        .get(text_chirho)
                        .copied()
                        .map(TyChirho::VarChirho)
                        .unwrap_or_else(|| {
                            // A constructor here is promoted, as an unquoted name in a type
                            // is (row 686).
                            TyChirho::ConChirho(
                                unquoted_promoted_spelling_chirho(
                                    name_chirho,
                                    self.promotion_scope_chirho.as_deref(),
                                )
                                .unwrap_or_else(|| name_chirho.full_name_chirho()),
                            )
                        }),
                )
            }
            ExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => Some(TyChirho::AppChirho(
                Box::new(self.expr_to_required_type_arg_chirho(fun_chirho)?),
                Box::new(self.expr_to_required_type_arg_chirho(arg_chirho)?),
            )),
            ExprChirho::ListChirho {
                elements_chirho, ..
            } => {
                // Preserve the old reverse freshening order; the shared
                // builder receives source order and folds from the right.
                let converted_chirho = elements_chirho
                    .iter()
                    .rev()
                    .map(|element_chirho| self.expr_to_required_type_arg_chirho(element_chirho))
                    .collect::<Option<Vec<_>>>()?;
                Some(TyChirho::promoted_list_chirho(
                    converted_chirho.into_iter().rev(),
                    self.promotion_scope_chirho.is_some(),
                ))
            }
            ExprChirho::TupleChirho {
                elements_chirho, ..
            } => elements_chirho
                .iter()
                .map(|element_chirho| self.expr_to_required_type_arg_chirho(element_chirho))
                .collect::<Option<Vec<_>>>()
                .map(TyChirho::TupleChirho),
            ExprChirho::ParenChirho { inner_chirho, .. }
            | ExprChirho::AnnChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.expr_to_required_type_arg_chirho(inner_chirho),
            ExprChirho::LitChirho(LitChirho::IntChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(value_chirho.to_string()))
            }
            ExprChirho::LitChirho(LitChirho::StringChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(format!("\"{value_chirho}\"")))
            }
            ExprChirho::LitChirho(LitChirho::CharChirho(value_chirho, _, _)) => {
                Some(TyChirho::ConChirho(format!("'{value_chirho}'")))
            }
            _ => None,
        }
    }

    pub(super) fn infer_required_type_application_chirho(
        &mut self,
        function_chirho: &TyChirho,
        argument_chirho: &ExprChirho,
        span_chirho: SpanChirho,
    ) -> Option<(SubstChirho, TyChirho)> {
        let TyChirho::RequiredForallChirho { vars_chirho, .. } = function_chirho else {
            return None;
        };
        let result_chirho = (|| {
            let record_chirho = vars_chirho.record_chirho(0).ok_or_else(|| {
                crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                    mismatch_chirho: None,
                    message_chirho: "required application has an empty binder group".into(),
                    span_chirho,
                }
            })?;
            let (argument_chirho, links_chirho) =
                if self.elaboration_state_chirho.owner_chirho.is_some() {
                    let kind_chirho = record_chirho.kind_chirho.as_ref().ok_or_else(|| {
                        crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                            mismatch_chirho: None,
                            message_chirho: "required binder has no checked kind".into(),
                            span_chirho,
                        }
                    })?;
                    let (
                        Some(type_scope_chirho),
                        Some(family_scope_chirho),
                        Some(promotion_scope_chirho),
                    ) = (
                        self.type_constructor_scope_chirho.clone(),
                        self.family_scope_chirho.clone(),
                        self.promotion_scope_chirho.clone(),
                    )
                    else {
                        return Err(
                            crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                                mismatch_chirho: None,
                                message_chirho: "required argument lost its checked import scope"
                                    .into(),
                                span_chirho,
                            },
                        );
                    };
                    crate::kind_chirho::elaboration_chirho::elaborate_required_argument_chirho(
                        argument_chirho,
                        kind_chirho,
                        crate::kind_chirho::elaboration_chirho::ArgumentElaborationContextChirho {
                            state_chirho: &mut self.elaboration_state_chirho,
                            registry_chirho: &mut self.type_families_chirho,
                            type_scope_chirho: &type_scope_chirho,
                            family_scope_chirho: &family_scope_chirho,
                            promotion_scope_chirho: &promotion_scope_chirho,
                            scoped_variables_chirho: &self.scoped_tyvars_chirho,
                            images_chirho: &self.skolem_bindings_chirho,
                            refinements_chirho: &self.env_chirho,
                            refinement_mark_chirho: None,
                            equality_chirho: super::EqualityNormalizationViewChirho {
                                substitutions_chirho: &self.equality_substitutions_chirho,
                                legacy_synonyms_chirho: &self.type_synonyms_chirho,
                                type_scope_chirho: Some(&type_scope_chirho),
                                family_scope_chirho: Some(&family_scope_chirho),
                                family_module_chirho: &self.family_module_name_chirho,
                                promotion_present_chirho: true,
                                evidence_chirho: self.equality_scopes_chirho
                                    [self.active_equality_scope_chirho]
                                    .evidence_view_chirho(),
                            },
                        },
                    )?
                } else {
                    // Direct legacy APIs have no kind producer to upgrade. Their
                    // ordinary payload still follows the bounded substitution path.
                    (
                        self.expr_to_required_type_arg_chirho(argument_chirho)
                            .ok_or_else(|| {
                                crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                                    mismatch_chirho: None,
                                    message_chirho: "expression is not a required type argument"
                                        .into(),
                                    span_chirho,
                                }
                            })?,
                        SubstChirho::empty_chirho(),
                    )
                };
            self.apply_subst_all_chirho(&links_chirho);
            let mut budget_chirho =
                super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
            let function_chirho = links_chirho
                .apply_scheme_term_bounded_chirho(
                    function_chirho,
                    &mut self.elaboration_state_chirho.next_variable_chirho,
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                )
                .ok_or_else(
                    || crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                        mismatch_chirho: None,
                        message_chirho: "required function substitution resource bound exhausted"
                            .into(),
                        span_chirho,
                    },
                )?;
            let result_chirho = self
                .apply_required_type_arg_chirho(&function_chirho, argument_chirho)
                .ok_or_else(
                    || crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                        mismatch_chirho: None,
                        message_chirho: "required argument substitution resource bound exhausted"
                            .into(),
                        span_chirho,
                    },
                )?;
            let (body_chirho, predicates_chirho) =
                self.expose_qualified_parts_chirho(result_chirho, Vec::new());
            for predicate_chirho in predicates_chirho {
                self.defer_pred_chirho(predicate_chirho, span_chirho);
            }
            Ok((links_chirho, body_chirho))
        })();
        Some(result_chirho.unwrap_or_else(
            |error_chirho: crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho| {
                self.record_body_kind_error_chirho(error_chirho);
                (SubstChirho::empty_chirho(), self.fresh_var_chirho())
            },
        ))
    }

    fn apply_required_type_arg_chirho(
        &mut self,
        fun_ty_chirho: &TyChirho,
        arg_ty_chirho: TyChirho,
    ) -> Option<TyChirho> {
        let TyChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
        } = fun_ty_chirho
        else {
            return None;
        };
        let (first_var_chirho, remaining_vars_chirho) = vars_chirho.split_first()?;
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        budget_chirho.node_chirho(vars_chirho.len())?;
        let mut subst_chirho = SubstChirho::empty_chirho();
        subst_chirho.insert_chirho(*first_var_chirho, arg_ty_chirho);
        let remaining_chirho = if remaining_vars_chirho.is_empty() {
            super::equality_normalization_chirho::copy_term_with_budget_chirho(
                body_chirho,
                &mut budget_chirho,
            )?
        } else {
            TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.slice_chirho(1..vars_chirho.len()),
                body_chirho: Box::new(
                    super::equality_normalization_chirho::copy_term_with_budget_chirho(
                        body_chirho,
                        &mut budget_chirho,
                    )?,
                ),
            }
        };
        subst_chirho.apply_scheme_term_bounded_chirho(
            &remaining_chirho,
            &mut self.elaboration_state_chirho.next_variable_chirho,
            &mut |work_chirho, output_chirho| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            },
        )
    }

    pub(super) fn apply_visible_type_arg_chirho(
        &mut self,
        fun_ty_chirho: &TyChirho,
        arg_ty_chirho: TyChirho,
        argument_kind_chirho: Option<&TyChirho>,
        span_chirho: SpanChirho,
    ) -> Option<TyChirho> {
        let TyChirho::ForallChirho {
            vars_chirho,
            body_chirho,
        } = fun_ty_chirho
        else {
            return None;
        };
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        if budget_chirho.node_chirho(vars_chirho.len()).is_none() {
            self.type_normalization_exhausted_chirho.set(true);
            return None;
        }
        let mut substitution_chirho = SubstChirho::empty_chirho();
        let mut consumed_chirho = 0;
        for record_chirho in vars_chirho.records_chirho() {
            if record_chirho.specificity_chirho
                == Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho)
            {
                let Some(kind_chirho) = self.instantiate_quantifier_kind_chirho(
                    record_chirho,
                    &substitution_chirho,
                    &mut budget_chirho,
                ) else {
                    self.type_normalization_exhausted_chirho.set(true);
                    return None;
                };
                let fresh_chirho = self.fresh_var_chirho();
                self.note_fresh_quantifier_kind_chirho(&fresh_chirho, &kind_chirho);
                substitution_chirho.insert_chirho(record_chirho.variable_chirho, fresh_chirho);
                consumed_chirho += 1;
            } else {
                break;
            }
        }
        let selected_chirho = vars_chirho.record_chirho(consumed_chirho)?;
        let Some(expected_kind_chirho) = self.instantiate_quantifier_kind_chirho(
            selected_chirho,
            &substitution_chirho,
            &mut budget_chirho,
        ) else {
            self.type_normalization_exhausted_chirho.set(true);
            return None;
        };
        if let (Some(expected_chirho), Some(actual_chirho)) =
            (&expected_kind_chirho, argument_kind_chirho)
        {
            match crate::unify_chirho::unify_kind_terms_chirho(
                expected_chirho,
                actual_chirho,
                span_chirho,
            ) {
                Ok(solution_chirho) => {
                    substitution_chirho = solution_chirho.compose_chirho(&substitution_chirho);
                    self.apply_subst_all_chirho(&solution_chirho);
                }
                Err(error_chirho) => {
                    self.report_unify_error_chirho(&error_chirho);
                    return None;
                }
            }
        }
        substitution_chirho.insert_chirho(selected_chirho.variable_chirho, arg_ty_chirho);
        consumed_chirho += 1;
        let Some(copied_chirho) =
            super::equality_normalization_chirho::copy_term_with_budget_chirho(
                body_chirho,
                &mut budget_chirho,
            )
        else {
            self.type_normalization_exhausted_chirho.set(true);
            return None;
        };
        if budget_chirho
            .node_chirho(
                vars_chirho
                    .len()
                    .saturating_sub(consumed_chirho)
                    .saturating_mul(2),
            )
            .is_none()
        {
            self.type_normalization_exhausted_chirho.set(true);
            return None;
        }
        let remaining_chirho = if consumed_chirho == vars_chirho.len() {
            copied_chirho
        } else {
            TyChirho::ForallChirho {
                vars_chirho: vars_chirho.slice_chirho(consumed_chirho..vars_chirho.len()),
                body_chirho: Box::new(copied_chirho),
            }
        };
        let result_chirho = substitution_chirho.apply_bounded_chirho(
            &remaining_chirho,
            &mut |work_chirho, output_chirho| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            },
        );
        if result_chirho.is_none() {
            self.type_normalization_exhausted_chirho.set(true);
        }
        result_chirho
    }
}
