// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Consume the checked codomain in source order. Later universals and givens
//! open inside earlier value-pattern implications and close before them.
//! Workflow: language-features-chirho/rank-n-visible-type-application-chirho.

use super::body_chirho::BodyOrderChirho;
use super::*;
use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;

#[derive(Clone, Copy)]
struct OrderedPatternSiteChirho<'scope_chirho> {
    where_chirho: &'scope_chirho [LocalBindChirho],
    span_chirho: SpanChirho,
    parameter_scope_chirho:
        Option<&'scope_chirho super::super::annotations_chirho::ValueParameterScopeChirho>,
}

impl InferCtxChirho {
    pub(super) fn has_required_spine_chirho(&self, mut term_chirho: &TyChirho) -> bool {
        for _ in 0..128 {
            term_chirho = match term_chirho {
                TyChirho::RequiredForallChirho { .. } => return true,
                TyChirho::ForallChirho { body_chirho, .. }
                | TyChirho::QualChirho { body_chirho, .. } => body_chirho,
                TyChirho::FunChirho(_, result_chirho, _) => result_chirho,
                _ => return false,
            };
        }
        self.type_normalization_exhausted_chirho.set(true);
        true
    }

    pub(super) fn check_ordered_patterns_chirho(
        &mut self,
        patterns_chirho: &[PatChirho],
        where_chirho: &[LocalBindChirho],
        expected_chirho: &TyChirho,
        body_chirho: &mut impl FnMut(&mut Self, &TyChirho) -> (SubstChirho, TyChirho),
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let mut budget_chirho =
            super::super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let copied_chirho = SubstChirho::empty_chirho().apply_scheme_term_bounded_chirho(
            expected_chirho,
            &mut self.elaboration_state_chirho.next_variable_chirho,
            &mut |work_chirho, output_chirho| {
                budget_chirho.node_chirho(output_chirho)?;
                budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
            },
        );
        let Some(copied_chirho) = copied_chirho else {
            self.type_normalization_exhausted_chirho.set(true);
            return SubstChirho::empty_chirho();
        };
        let Some(entry_chirho) =
            super::super::equality_normalization_chirho::copy_term_with_budget_chirho(
                &copied_chirho,
                &mut budget_chirho,
            )
        else {
            self.type_normalization_exhausted_chirho.set(true);
            return SubstChirho::empty_chirho();
        };
        let parameter_scope_chirho = self.value_parameter_scope_chirho();
        self.check_ordered_parameter_entry_chirho(
            patterns_chirho.len(),
            entry_chirho,
            span_chirho,
            &mut budget_chirho,
            0,
        );
        self.ordered_pattern_cursor_chirho(
            patterns_chirho,
            OrderedPatternSiteChirho {
                where_chirho,
                span_chirho,
                parameter_scope_chirho: parameter_scope_chirho.as_ref(),
            },
            copied_chirho,
            body_chirho,
            &mut budget_chirho,
            0,
        )
    }

    /// Open the same checked mixed spine without matching value patterns.
    /// Later parameter kinds cannot borrow givens from earlier patterns in
    /// this list; enclosing implications and written contexts remain available.
    fn check_ordered_parameter_entry_chirho(
        &mut self,
        remaining_chirho: usize,
        current_chirho: TyChirho,
        span_chirho: SpanChirho,
        budget_chirho: &mut super::super::equality_normalization_chirho::EqualityBudgetChirho,
        depth_chirho: usize,
    ) {
        if remaining_chirho == 0 {
            return;
        }
        if self.has_non_deferrable_failure_chirho() {
            return;
        }
        if depth_chirho >= 128 || budget_chirho.work_chirho(1).is_none() {
            self.type_normalization_exhausted_chirho.set(true);
            return;
        }
        let required_chirho = matches!(&current_chirho, TyChirho::RequiredForallChirho { .. });
        match current_chirho {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            }
            | TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                let consumed_chirho = if required_chirho {
                    remaining_chirho.min(vars_chirho.len())
                } else {
                    0
                };
                let scheme_chirho = SchemeChirho {
                    vars_chirho,
                    preds_chirho: Vec::new(),
                    ty_chirho: *body_chirho,
                };
                self.with_rigid_signature_chirho(&scheme_chirho, span_chirho, |context_chirho, opened_chirho| {
                    if let Some(copied_chirho) = super::super::equality_normalization_chirho::copy_term_with_budget_chirho(opened_chirho, budget_chirho) {
                        context_chirho.check_ordered_parameter_entry_chirho(remaining_chirho - consumed_chirho, copied_chirho, span_chirho, budget_chirho, depth_chirho + 1);
                    } else { context_chirho.type_normalization_exhausted_chirho.set(true); }
                });
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho,
            } => {
                let qualified_chirho = TyChirho::QualChirho {
                    context_chirho,
                    body_chirho,
                };
                let (next_chirho, givens_chirho) =
                    self.expose_qualified_parts_chirho(qualified_chirho, Vec::new());
                self.with_signature_givens_chirho(givens_chirho, &next_chirho, |context_chirho| {
                    if let Some(copied_chirho) =
                        super::super::equality_normalization_chirho::copy_term_with_budget_chirho(
                            &next_chirho,
                            budget_chirho,
                        )
                    {
                        context_chirho.check_ordered_parameter_entry_chirho(
                            remaining_chirho,
                            copied_chirho,
                            span_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        );
                    } else {
                        context_chirho.type_normalization_exhausted_chirho.set(true);
                    }
                });
            }
            TyChirho::FunChirho(parameter_chirho, next_chirho, _) => {
                self.check_value_parameters_chirho(
                    &[],
                    std::slice::from_ref(parameter_chirho.as_ref()),
                    span_chirho,
                );
                self.check_ordered_parameter_entry_chirho(
                    remaining_chirho - 1,
                    *next_chirho,
                    span_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
            }
            _ => {}
        }
    }

    fn with_ordered_signature_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        site_chirho: OrderedPatternSiteChirho<'_>,
        check_chirho: impl FnOnce(&mut Self, &TyChirho, OrderedPatternSiteChirho<'_>) -> SubstChirho,
    ) -> (SubstChirho, SubstChirho) {
        let given_start_chirho = self.given_preds_chirho.len();
        self.with_rigid_signature_at_parameter_entry_chirho(
            scheme_chirho,
            site_chirho.span_chirho,
            site_chirho.parameter_scope_chirho,
            |context_chirho, opened_chirho, parameter_scope_chirho| {
                if context_chirho.has_non_deferrable_failure_chirho() {
                    return SubstChirho::empty_chirho();
                }
                if site_chirho.parameter_scope_chirho.is_some() && parameter_scope_chirho.is_none()
                {
                    return SubstChirho::empty_chirho();
                }
                if context_chirho.given_preds_chirho[given_start_chirho..]
                    .iter()
                    .any(|predicate_chirho| predicate_chirho.class_name_chirho != "~")
                {
                    context_chirho.note_required_sigma_gap_chirho(site_chirho.span_chirho);
                }
                check_chirho(
                    context_chirho,
                    opened_chirho,
                    OrderedPatternSiteChirho {
                        parameter_scope_chirho,
                        ..site_chirho
                    },
                )
            },
        )
    }

    fn ordered_pattern_cursor_chirho(
        &mut self,
        patterns_chirho: &[PatChirho],
        site_chirho: OrderedPatternSiteChirho<'_>,
        current_chirho: TyChirho,
        body_chirho: &mut impl FnMut(&mut Self, &TyChirho) -> (SubstChirho, TyChirho),
        budget_chirho: &mut super::super::equality_normalization_chirho::EqualityBudgetChirho,
        depth_chirho: usize,
    ) -> SubstChirho {
        let where_chirho = site_chirho.where_chirho;
        let span_chirho = site_chirho.span_chirho;
        if self.has_non_deferrable_failure_chirho() {
            return SubstChirho::empty_chirho();
        }
        if depth_chirho >= 128 || budget_chirho.work_chirho(1).is_none() {
            self.type_normalization_exhausted_chirho.set(true);
            return SubstChirho::empty_chirho();
        }
        if patterns_chirho.is_empty() {
            return self.check_pattern_body_chirho(
                where_chirho,
                (&current_chirho, &current_chirho),
                SubstChirho::empty_chirho(),
                BodyOrderChirho::ExpectedFirstChirho,
                body_chirho,
                span_chirho,
            );
        }
        match current_chirho {
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho: next_chirho,
            } => {
                let scheme_chirho = SchemeChirho {
                    vars_chirho,
                    preds_chirho: Vec::new(),
                    ty_chirho: *next_chirho,
                };
                let (checked_chirho, public_chirho) = self.with_ordered_signature_chirho(&scheme_chirho, site_chirho, |context_chirho, opened_chirho, site_chirho| {
                    let Some(opened_chirho) = super::super::equality_normalization_chirho::copy_term_with_budget_chirho(opened_chirho, budget_chirho) else {
                        context_chirho.type_normalization_exhausted_chirho.set(true);
                        return SubstChirho::empty_chirho();
                    };
                    context_chirho.ordered_pattern_cursor_chirho(patterns_chirho, site_chirho, opened_chirho, body_chirho, budget_chirho, depth_chirho + 1)
                });
                self.publish_signature_solution_chirho(checked_chirho, public_chirho)
            }
            TyChirho::QualChirho {
                context_chirho,
                body_chirho: next_chirho,
            } => {
                let qualified_chirho = TyChirho::QualChirho {
                    context_chirho,
                    body_chirho: next_chirho,
                };
                let (next_chirho, givens_chirho) =
                    self.expose_qualified_parts_chirho(qualified_chirho, Vec::new());
                if givens_chirho
                    .iter()
                    .any(|predicate_chirho| predicate_chirho.class_name_chirho != "~")
                {
                    self.note_required_sigma_gap_chirho(span_chirho);
                }
                let parameter_scope_chirho = match site_chirho.parameter_scope_chirho {
                    Some(entry_chirho) => {
                        let Some(extended_chirho) = self.extend_value_parameter_scope_chirho(
                            entry_chirho,
                            &givens_chirho,
                            span_chirho,
                        ) else {
                            return SubstChirho::empty_chirho();
                        };
                        Some(extended_chirho)
                    }
                    None => None,
                };
                self.with_signature_givens_chirho(givens_chirho, &next_chirho, |context_chirho| {
                    let Some(copied_chirho) =
                        super::super::equality_normalization_chirho::copy_term_with_budget_chirho(
                            &next_chirho,
                            budget_chirho,
                        )
                    else {
                        context_chirho.type_normalization_exhausted_chirho.set(true);
                        return SubstChirho::empty_chirho();
                    };
                    context_chirho.ordered_pattern_cursor_chirho(
                        patterns_chirho,
                        OrderedPatternSiteChirho {
                            parameter_scope_chirho: parameter_scope_chirho.as_ref(),
                            ..site_chirho
                        },
                        copied_chirho,
                        body_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )
                })
            }
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho: next_chirho,
            } => {
                let Some(record_chirho) = vars_chirho.record_chirho(0).cloned() else {
                    self.non_deferrable_diagnostics_chirho.push_chirho(
                        crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                            mismatch_chirho: None,
                            message_chirho: "checked required signature has an empty binder group"
                                .into(),
                            span_chirho,
                        }
                        .into_diagnostic_chirho(),
                    );
                    return SubstChirho::empty_chirho();
                };
                let next_chirho = if vars_chirho.len() == 1 {
                    *next_chirho
                } else {
                    TyChirho::RequiredForallChirho {
                        vars_chirho: vars_chirho.slice_chirho(1..vars_chirho.len()),
                        body_chirho: next_chirho,
                    }
                };
                let scheme_chirho = SchemeChirho {
                    vars_chirho: vars_chirho.slice_chirho(0..1),
                    preds_chirho: Vec::new(),
                    ty_chirho: next_chirho,
                };
                let (checked_chirho, public_chirho) = self.with_ordered_signature_chirho(&scheme_chirho, site_chirho, |context_chirho, opened_chirho, site_chirho| {
                    let Some(image_chirho) = context_chirho.skolem_bindings_chirho.lookup_image_chirho(&record_chirho.variable_chirho) else {
                        context_chirho.non_deferrable_diagnostics_chirho.push_chirho(
                            crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                                mismatch_chirho: None,
                    message_chirho: "checked required binder has no rigid image".into(),
                                span_chirho,
                            }.into_diagnostic_chirho(),
                        );
                        return SubstChirho::empty_chirho();
                    };
                    let Some(term_chirho) = super::super::equality_normalization_chirho::copy_term_with_budget_chirho(&image_chirho.image_chirho, budget_chirho) else {
                        context_chirho.type_normalization_exhausted_chirho.set(true);
                        return SubstChirho::empty_chirho();
                    };
                    let image_chirho = crate::subst_chirho::SubstitutionImageChirho { image_chirho: term_chirho, kind_chirho: image_chirho.kind_chirho.clone(), source_chirho: image_chirho.source_chirho.clone() };
                    let mark_chirho = context_chirho.type_pattern_alias_undo_chirho.len();
                    let mut pattern_chirho = &patterns_chirho[0];
                    let mut parens_chirho = depth_chirho;
                    while let PatChirho::ParenChirho { inner_chirho, .. } = pattern_chirho {
                        parens_chirho += 1;
                        if parens_chirho >= 128 || budget_chirho.work_chirho(1).is_none() {
                            context_chirho.type_normalization_exhausted_chirho.set(true);
                            return SubstChirho::empty_chirho();
                        }
                        pattern_chirho = inner_chirho;
                    }
                    match pattern_chirho {
                        PatChirho::VarChirho(name_chirho) if name_chirho.type_variable_chirho().is_none() || name_chirho.type_variable_chirho() == record_chirho.source_chirho.as_ref() => {
                            if budget_chirho.node_chirho(name_chirho.text_chirho().len()).is_none() {
                                context_chirho.type_normalization_exhausted_chirho.set(true);
                                return SubstChirho::empty_chirho();
                            }
                            context_chirho.install_type_pattern_alias_image_chirho(name_chirho.text_chirho().to_owned(), record_chirho.variable_chirho, image_chirho);
                        }
                        PatChirho::WildcardChirho(_) => context_chirho.install_anonymous_type_pattern_image_chirho(record_chirho.variable_chirho, image_chirho),
                        _ => {
                            context_chirho.non_deferrable_diagnostics_chirho.push_chirho(
                                crate::kind_chirho::elaboration_chirho::ElaborationErrorChirho {
                                    mismatch_chirho: None,
                    message_chirho: "required pattern does not bind its checked type argument".into(),
                                    span_chirho: pattern_chirho.span_chirho(),
                                }.into_diagnostic_chirho(),
                            );
                            return SubstChirho::empty_chirho();
                        }
                    }
                    let result_chirho = if let Some(copied_chirho) = super::super::equality_normalization_chirho::copy_term_with_budget_chirho(opened_chirho, budget_chirho) {
                        context_chirho.ordered_pattern_cursor_chirho(&patterns_chirho[1..], site_chirho, copied_chirho, body_chirho, budget_chirho, depth_chirho + 1)
                    } else { context_chirho.type_normalization_exhausted_chirho.set(true); SubstChirho::empty_chirho() };
                    context_chirho.restore_type_pattern_aliases_chirho(mark_chirho);
                    result_chirho
                });
                self.publish_signature_solution_chirho(checked_chirho, public_chirho)
            }
            TyChirho::FunChirho(parameter_chirho, next_chirho, _) => {
                self.env_chirho.push_scope_chirho();
                let frame_chirho = self.begin_pattern_implication_with_roots_chirho(
                    [parameter_chirho.as_ref(), next_chirho.as_ref()],
                    span_chirho,
                );
                if self.has_non_deferrable_failure_chirho() {
                    return SubstChirho::empty_chirho();
                }
                let mut first_chirho = self.bind_pat_chirho(&patterns_chirho[0], &parameter_chirho);
                if self.apply_subst_all_chirho(&mut first_chirho).is_err() {
                    return SubstChirho::empty_chirho();
                };
                let next_chirho = first_chirho.apply_scheme_term_bounded_chirho(
                    &next_chirho,
                    &mut self.elaboration_state_chirho.next_variable_chirho,
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                );
                let mut result_chirho = first_chirho;
                if let Some(next_chirho) = next_chirho {
                    let exit_chirho =
                        super::super::equality_normalization_chirho::copy_term_with_budget_chirho(
                            &next_chirho,
                            budget_chirho,
                        );
                    let child_chirho = self.ordered_pattern_cursor_chirho(
                        &patterns_chirho[1..],
                        site_chirho,
                        next_chirho,
                        body_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    );
                    self.check_solved_value_parameters_chirho(
                        site_chirho.parameter_scope_chirho,
                        std::slice::from_ref(parameter_chirho.as_ref()),
                        span_chirho,
                    );
                    result_chirho = child_chirho.compose_chirho(&result_chirho);
                    let exit_chirho = exit_chirho.unwrap_or_else(|| {
                        self.type_normalization_exhausted_chirho.set(true);
                        Self::terminal_commit_type_chirho()
                    });
                    let solution_chirho = self.end_pattern_implication_chirho(
                        frame_chirho,
                        &result_chirho.apply_ty_chirho(&exit_chirho),
                        span_chirho,
                    );
                    result_chirho = solution_chirho.compose_chirho(&result_chirho);
                } else {
                    self.type_normalization_exhausted_chirho.set(true);
                    let result_ty_chirho = Self::terminal_commit_type_chirho();
                    let solution_chirho = self.end_pattern_implication_chirho(
                        frame_chirho,
                        &result_ty_chirho,
                        span_chirho,
                    );
                    result_chirho = solution_chirho.compose_chirho(&result_chirho);
                }
                self.env_chirho.pop_scope_chirho();
                result_chirho
            }
            _ => {
                self.diagnostics_chirho.push_chirho(
                    haskelujah_diagnostics_chirho::DiagnosticChirho::error_with_code_chirho(
                        haskelujah_diagnostics_chirho::ErrorCodeChirho::error_chirho(
                            super::super::TYPE_MISMATCH_CODE_CHIRHO,
                        ),
                        "function equation has more patterns than its checked argument spine",
                        span_chirho,
                    ),
                );
                SubstChirho::empty_chirho()
            }
        }
    }
}
