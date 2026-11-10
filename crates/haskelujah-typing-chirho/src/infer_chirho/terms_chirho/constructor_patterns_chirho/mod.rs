// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor pattern implications. Expression uses keep ordinary instantiation.
//! Workflow: language-features-chirho/pattern-matching-chirho.md.

use super::*;
mod instantiation_chirho;
mod signatures_chirho;

pub(super) struct TypePatternAliasUndoChirho {
    name_chirho: Option<String>,
    variable_chirho: TyVarChirho,
    previous_name_chirho: Option<TyVarChirho>,
    previous_image_chirho: Option<crate::subst_chirho::SubstitutionImageChirho>,
}

pub(super) struct PatternImplicationChirho {
    entered_chirho: bool,
    owner_chirho: usize,
    given_start_chirho: usize,
    rewrite_start_chirho: usize,
    first_meta_chirho: u32,
    event_start_chirho: usize,
    alias_mark_chirho: usize,
}

/// A public image replacement is a domain update, not ordinary mathematical
/// substitution composition. Keep it separate until each caller accumulates
/// this implication's complete solution.
pub(super) struct PatternImplicationSolutionChirho {
    ordinary_chirho: SubstChirho,
    public_chirho: SubstChirho,
}

impl PatternImplicationSolutionChirho {
    pub(super) fn compose_chirho(&self, previous_chirho: &SubstChirho) -> SubstChirho {
        let mut step_chirho = self.public_chirho.compose_chirho(&self.ordinary_chirho);
        for (var_chirho, image_chirho) in self.public_chirho.iter_chirho() {
            step_chirho.insert_chirho(*var_chirho, image_chirho.clone());
        }
        let mut combined_chirho = step_chirho.compose_chirho(previous_chirho);
        for (var_chirho, image_chirho) in self.public_chirho.iter_chirho() {
            combined_chirho.insert_chirho(*var_chirho, image_chirho.clone());
        }
        combined_chirho
    }
}

pub(super) type TypePatternAliasesChirho = Vec<(
    String,
    TyVarChirho,
    crate::subst_chirho::SubstitutionImageChirho,
)>;

impl InferCtxChirho {
    #[cfg(test)]
    pub(super) fn begin_pattern_implication_chirho(&mut self) -> PatternImplicationChirho {
        self.begin_pattern_implication_with_roots_chirho(
            std::iter::empty(),
            SpanChirho::DUMMY_CHIRHO,
        )
    }

    pub(super) fn begin_pattern_implication_with_roots_chirho<'roots_chirho>(
        &mut self,
        roots_chirho: impl IntoIterator<Item = &'roots_chirho TyChirho>,
        span_chirho: SpanChirho,
    ) -> PatternImplicationChirho {
        let mut additional_chirho = Vec::new();
        for root_chirho in roots_chirho {
            if let Err(error_chirho) =
                self.elaboration_state_chirho
                    .charge_chirho(1, 1, span_chirho)
            {
                self.record_body_kind_error_chirho(error_chirho);
                break;
            }
            additional_chirho.push(root_chirho);
        }
        let captured_chirho = self
            .capture_scope_metas_chirho(None, &additional_chirho, span_chirho)
            .and_then(|captured_chirho| captured_chirho.into_kind_capture_chirho());
        if captured_chirho.is_none() {
            self.type_normalization_exhausted_chirho.set(true);
            return PatternImplicationChirho {
                entered_chirho: false,
                owner_chirho: self.active_equality_scope_chirho,
                given_start_chirho: self.given_preds_chirho.len(),
                rewrite_start_chirho: self.given_rewrites_chirho.len(),
                first_meta_chirho: self.elaboration_state_chirho.next_variable_chirho,
                event_start_chirho: self.equality_substitution_events_chirho.len(),
                alias_mark_chirho: self.type_pattern_alias_undo_chirho.len(),
            };
        }
        self.equality_pattern_depth_chirho += 1;
        let owner_chirho = self.begin_equality_scope_chirho(
            super::equality_scopes_chirho::EqualityScopeKindChirho::PatternChirho,
        );
        self.equality_scopes_chirho[owner_chirho].captured_kinds_chirho = captured_chirho;
        PatternImplicationChirho {
            entered_chirho: true,
            owner_chirho,
            given_start_chirho: self.given_preds_chirho.len(),
            rewrite_start_chirho: self.given_rewrites_chirho.len(),
            first_meta_chirho: self.elaboration_state_chirho.next_variable_chirho,
            event_start_chirho: self.equality_substitution_events_chirho.len(),
            alias_mark_chirho: self.type_pattern_alias_undo_chirho.len(),
        }
    }

    /// Solve while pattern evidence and environment refinements still exist.
    /// Pending equalities keep their owner/evidence links after the frame ends.
    pub(super) fn end_pattern_implication_chirho(
        &mut self,
        frame_chirho: PatternImplicationChirho,
        result_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> PatternImplicationSolutionChirho {
        if !frame_chirho.entered_chirho {
            return PatternImplicationSolutionChirho {
                ordinary_chirho: SubstChirho::empty_chirho(),
                public_chirho: SubstChirho::empty_chirho(),
            };
        }
        let improvement_chirho = if self.has_non_deferrable_failure_chirho() {
            SubstChirho::empty_chirho()
        } else {
            self.discharge_deferred_with_current_givens_chirho()
        };
        let equalities_chirho = if self.has_non_deferrable_failure_chirho() {
            SubstChirho::empty_chirho()
        } else {
            self.retry_deferred_equalities_chirho()
        };
        let public_subst_chirho = self.check_pattern_equality_scope_chirho(
            frame_chirho.owner_chirho,
            result_chirho,
            frame_chirho.first_meta_chirho,
            frame_chirho.event_start_chirho,
            span_chirho,
        );
        self.given_preds_chirho
            .truncate(frame_chirho.given_start_chirho);
        self.given_rewrites_chirho
            .truncate(frame_chirho.rewrite_start_chirho);
        self.restore_type_pattern_aliases_chirho(frame_chirho.alias_mark_chirho);
        self.end_equality_scope_chirho(frame_chirho.owner_chirho);
        self.equality_pattern_depth_chirho -= 1;
        if self.equality_pattern_depth_chirho == 0 {
            self.equality_substitution_events_chirho.clear();
            self.equality_substitution_events_exhausted_chirho = false;
        }
        PatternImplicationSolutionChirho {
            ordinary_chirho: equalities_chirho.compose_chirho(&improvement_chirho),
            public_chirho: public_subst_chirho,
        }
    }

    /// Result parameters exposed through generative structure determine a
    /// universal. Field-only binders, and binders hidden under a non-injective
    /// family or unknown head, remain rigid in the pattern implication.
    pub(super) fn instantiate_constructor_pattern_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (Vec<TyChirho>, TyChirho, Vec<PredChirho>) {
        let (fields_chirho, result_chirho, predicates_chirho, _) =
            self.instantiate_constructor_pattern_with_variables_chirho(scheme_chirho);
        (fields_chirho, result_chirho, predicates_chirho)
    }

    pub(super) fn type_pattern_aliases_since_chirho(
        &self,
        mark_chirho: usize,
    ) -> TypePatternAliasesChirho {
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let mut aliases_chirho = Vec::new();
        for undo_chirho in &self.type_pattern_alias_undo_chirho[mark_chirho..] {
            let Some(name_chirho) = &undo_chirho.name_chirho else {
                continue;
            };
            if let Some(image_chirho) = self
                .skolem_bindings_chirho
                .lookup_image_chirho(&undo_chirho.variable_chirho)
            {
                let copied_chirho = SubstChirho::empty_chirho().apply_bounded_chirho(
                    &image_chirho.image_chirho,
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                );
                if let Some(copied_chirho) = copied_chirho.filter(|_| {
                    budget_chirho
                        .node_chirho(name_chirho.len().saturating_add(1))
                        .is_some()
                }) {
                    aliases_chirho.push((
                        name_chirho.clone(),
                        undo_chirho.variable_chirho,
                        crate::subst_chirho::SubstitutionImageChirho {
                            image_chirho: copied_chirho,
                            kind_chirho: image_chirho.kind_chirho.clone(),
                            source_chirho: image_chirho.source_chirho.clone(),
                        },
                    ));
                } else {
                    self.type_normalization_exhausted_chirho.set(true);
                    return Vec::new();
                }
            }
        }
        aliases_chirho
    }

    pub(super) fn restore_type_pattern_aliases_chirho(&mut self, mark_chirho: usize) {
        while self.type_pattern_alias_undo_chirho.len() > mark_chirho {
            let undo_chirho = self
                .type_pattern_alias_undo_chirho
                .pop()
                .expect("type pattern scope mark");
            if let Some(name_chirho) = undo_chirho.name_chirho {
                if let Some(previous_chirho) = undo_chirho.previous_name_chirho {
                    self.scoped_tyvars_chirho
                        .insert(name_chirho, previous_chirho);
                } else {
                    self.scoped_tyvars_chirho.remove(&name_chirho);
                }
            }
            if let Some(previous_chirho) = undo_chirho.previous_image_chirho {
                self.skolem_bindings_chirho
                    .insert_image_chirho(undo_chirho.variable_chirho, previous_chirho);
            } else {
                self.skolem_bindings_chirho
                    .remove_chirho(&undo_chirho.variable_chirho);
            }
        }
    }

    pub(super) fn install_type_pattern_alias_image_chirho(
        &mut self,
        name_chirho: String,
        variable_chirho: TyVarChirho,
        image_chirho: crate::subst_chirho::SubstitutionImageChirho,
    ) {
        self.install_type_pattern_binding_chirho(Some(name_chirho), variable_chirho, image_chirho);
    }

    pub(super) fn install_anonymous_type_pattern_image_chirho(
        &mut self,
        variable_chirho: TyVarChirho,
        image_chirho: crate::subst_chirho::SubstitutionImageChirho,
    ) {
        self.install_type_pattern_binding_chirho(None, variable_chirho, image_chirho);
    }

    fn install_type_pattern_binding_chirho(
        &mut self,
        name_chirho: Option<String>,
        variable_chirho: TyVarChirho,
        image_chirho: crate::subst_chirho::SubstitutionImageChirho,
    ) {
        let previous_name_chirho = name_chirho.as_ref().and_then(|name_chirho| {
            self.scoped_tyvars_chirho
                .insert(name_chirho.clone(), variable_chirho)
        });
        let previous_image_chirho = self
            .skolem_bindings_chirho
            .lookup_image_chirho(&variable_chirho)
            .cloned();
        self.skolem_bindings_chirho
            .insert_image_chirho(variable_chirho, image_chirho);
        self.type_pattern_alias_undo_chirho
            .push(TypePatternAliasUndoChirho {
                name_chirho,
                variable_chirho,
                previous_name_chirho,
                previous_image_chirho,
            });
    }

    pub(super) fn specialize_type_pattern_image_chirho(
        &mut self,
        image_chirho: &crate::subst_chirho::SubstitutionImageChirho,
        solution_chirho: &SubstChirho,
        span_chirho: SpanChirho,
    ) -> Option<crate::subst_chirho::SubstitutionImageChirho> {
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return None;
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let result_chirho = (|| {
            let mut apply_chirho = |term_chirho: &TyChirho| {
                solution_chirho.apply_scheme_term_bounded_chirho(
                    term_chirho,
                    &mut self.elaboration_state_chirho.next_variable_chirho,
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                )
            };
            let term_chirho = apply_chirho(&image_chirho.image_chirho)?;
            let kind_chirho = match &image_chirho.kind_chirho {
                Some(kind_chirho) => Some(Arc::new(apply_chirho(kind_chirho)?)),
                None => None,
            };
            Some(crate::subst_chirho::SubstitutionImageChirho {
                image_chirho: term_chirho,
                kind_chirho,
                source_chirho: image_chirho.source_chirho.clone(),
            })
        })();
        if let Err(error_chirho) = self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            result_chirho.is_none(),
            span_chirho,
        ) {
            self.record_body_kind_error_chirho(error_chirho);
            return None;
        }
        result_chirho
    }

    pub(super) fn bind_constructor_type_pattern_chirho(
        &mut self,
        pattern_chirho: &TypeChirho,
        expected_chirho: &TyChirho,
        expected_kind_chirho: Option<&TyChirho>,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let mut budget_chirho = match self
            .elaboration_state_chirho
            .lend_equality_budget_chirho(span_chirho)
        {
            Ok(budget_chirho) => budget_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return SubstChirho::empty_chirho();
            }
        };
        let initial_chirho = budget_chirho.remaining_chirho();
        let mut seen_chirho = HashSet::new();
        let mut unknown_origin_chirho = false;
        let mark_chirho = self.type_pattern_alias_undo_chirho.len();
        let prepared_chirho = self
            .prepare_type_pattern_variables_chirho(
                pattern_chirho,
                &mut seen_chirho,
                true,
                &mut unknown_origin_chirho,
                &mut budget_chirho,
                0,
            )
            .and_then(|()| {
                if unknown_origin_chirho {
                    return None;
                }
                self.complete_type_pattern_kinds_chirho(mark_chirho, &mut budget_chirho)
            });
        if let Err(error_chirho) = self.elaboration_state_chirho.settle_equality_budget_chirho(
            initial_chirho,
            &budget_chirho,
            prepared_chirho.is_none(),
            span_chirho,
        ) {
            self.type_normalization_exhausted_chirho.set(true);
            self.restore_type_pattern_aliases_chirho(mark_chirho);
            self.record_body_kind_error_chirho(error_chirho);
            return SubstChirho::empty_chirho();
        }
        let checked_pattern_chirho = expected_kind_chirho.and_then(|kind_chirho| {
            self.check_body_pattern_type_chirho(pattern_chirho, kind_chirho)
        });
        let pattern_chirho = checked_pattern_chirho
            .as_ref()
            .map_or(pattern_chirho, |checked_chirho| {
                &checked_chirho.syntax_chirho
            });
        self.bind_prepared_constructor_type_pattern_chirho(
            pattern_chirho,
            expected_chirho,
            span_chirho,
        )
    }

    /// Every free pattern variable has an image before any dependent kind is
    /// mapped. The same current prefix also includes earlier pattern arguments.
    fn complete_type_pattern_kinds_chirho(
        &mut self,
        mark_chirho: usize,
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
    ) -> Option<()> {
        for index_chirho in mark_chirho..self.type_pattern_alias_undo_chirho.len() {
            budget_chirho.node_chirho(1)?;
            let variable_chirho = self.type_pattern_alias_undo_chirho[index_chirho].variable_chirho;
            let image_chirho = self
                .skolem_bindings_chirho
                .lookup_image_chirho(&variable_chirho)?;
            let term_chirho = super::equality_normalization_chirho::copy_term_with_budget_chirho(
                &image_chirho.image_chirho,
                budget_chirho,
            )?;
            let source_chirho = image_chirho.source_chirho.clone();
            let kind_chirho = match image_chirho.kind_chirho.clone() {
                Some(kind_chirho) => Some(Arc::new(
                    self.skolem_bindings_chirho.apply_freshened_bounded_chirho(
                        &kind_chirho,
                        &mut self.elaboration_state_chirho.next_variable_chirho,
                        &mut |work_chirho, output_chirho| {
                            budget_chirho.node_chirho(output_chirho)?;
                            budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                        },
                    )?,
                )),
                None => None,
            };
            self.note_fresh_quantifier_kind_chirho(&term_chirho, &kind_chirho);
            self.skolem_bindings_chirho.insert_image_chirho(
                variable_chirho,
                crate::subst_chirho::SubstitutionImageChirho {
                    image_chirho: term_chirho,
                    kind_chirho,
                    source_chirho,
                },
            );
        }
        Some(())
    }

    fn bind_prepared_constructor_type_pattern_chirho(
        &mut self,
        pattern_chirho: &TypeChirho,
        expected_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        match pattern_chirho {
            TypeChirho::ParenChirho { inner_chirho, .. } => self
                .bind_prepared_constructor_type_pattern_chirho(
                    inner_chirho,
                    expected_chirho,
                    span_chirho,
                ),
            TypeChirho::KindAnnotationChirho { ty_chirho, .. } => self
                .bind_prepared_constructor_type_pattern_chirho(
                    ty_chirho,
                    expected_chirho,
                    span_chirho,
                ),
            TypeChirho::WildcardChirho { .. } => SubstChirho::empty_chirho(),
            TypeChirho::VarChirho(name_chirho) => {
                let mut variables_chirho = HashMap::new();
                let variable_chirho =
                    self.surface_variable_chirho(name_chirho, &mut variables_chirho);
                let kind_chirho = self
                    .skolem_bindings_chirho
                    .lookup_image_chirho(&variable_chirho)
                    .and_then(|image_chirho| image_chirho.kind_chirho.clone());
                let term_chirho = match self
                    .elaboration_state_chirho
                    .copy_chirho(expected_chirho, span_chirho)
                {
                    Ok(term_chirho) => term_chirho,
                    Err(error_chirho) => {
                        self.record_body_kind_error_chirho(error_chirho);
                        return SubstChirho::empty_chirho();
                    }
                };
                self.install_type_pattern_alias_image_chirho(
                    name_chirho.text_chirho().to_owned(),
                    variable_chirho,
                    crate::subst_chirho::SubstitutionImageChirho {
                        image_chirho: term_chirho,
                        kind_chirho,
                        source_chirho: name_chirho.type_variable_chirho().cloned(),
                    },
                );
                SubstChirho::empty_chirho()
            }
            _ => {
                let matched_chirho = self.ast_type_to_ty_in_scope_chirho(pattern_chirho);
                match self.unify_normalized_chirho(expected_chirho, &matched_chirho, span_chirho) {
                    Ok(substitution_chirho) => substitution_chirho,
                    Err(error_chirho) => {
                        self.report_unify_error_chirho(&error_chirho);
                        SubstChirho::empty_chirho()
                    }
                }
            }
        }
    }

    fn prepare_type_pattern_variables_chirho(
        &mut self,
        pattern_chirho: &TypeChirho,
        seen_chirho: &mut HashSet<haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho>,
        bind_variables_chirho: bool,
        unknown_origin_chirho: &mut bool,
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
        depth_chirho: usize,
    ) -> Option<()> {
        budget_chirho.work_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        match pattern_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let Some(source_chirho) = name_chirho.type_variable_chirho() else {
                    *unknown_origin_chirho = true;
                    return Some(());
                };
                if seen_chirho.insert(source_chirho.clone()) {
                    budget_chirho.node_chirho(name_chirho.text_chirho().len().saturating_add(1))?;
                    if !bind_variables_chirho {
                        return Some(());
                    }
                    let mut variables_chirho = HashMap::new();
                    let variable_chirho =
                        self.surface_variable_chirho(name_chirho, &mut variables_chirho);
                    let kind_chirho = self.surface_quantifier_kind_chirho(
                        name_chirho,
                        &variables_chirho,
                        budget_chirho,
                    );
                    let fresh_chirho = self.fresh_var_chirho();
                    self.install_type_pattern_alias_image_chirho(
                        name_chirho.text_chirho().to_owned(),
                        variable_chirho,
                        crate::subst_chirho::SubstitutionImageChirho {
                            image_chirho: fresh_chirho,
                            kind_chirho,
                            source_chirho: name_chirho.type_variable_chirho().cloned(),
                        },
                    );
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                self.prepare_type_pattern_variables_chirho(
                    fun_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    arg_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                self.prepare_type_pattern_variables_chirho(
                    arg_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    result_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                for element_chirho in elements_chirho {
                    self.prepare_type_pattern_variables_chirho(
                        element_chirho,
                        seen_chirho,
                        bind_variables_chirho,
                        unknown_origin_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?;
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => self
                .prepare_type_pattern_variables_chirho(
                    element_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?,
            TypeChirho::ParenChirho { inner_chirho, .. } => self
                .prepare_type_pattern_variables_chirho(
                    inner_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?,
            TypeChirho::KindAnnotationChirho {
                ty_chirho,
                kind_chirho,
                ..
            } => {
                self.prepare_type_pattern_variables_chirho(
                    ty_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    kind_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let mut bound_chirho = Vec::new();
                for binder_chirho in vars_chirho {
                    if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                        self.prepare_type_pattern_variables_chirho(
                            kind_chirho,
                            seen_chirho,
                            bind_variables_chirho,
                            unknown_origin_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )?;
                    }
                    budget_chirho.node_chirho(1)?;
                    let Some(source_chirho) = binder_chirho.name_chirho.type_variable_chirho()
                    else {
                        *unknown_origin_chirho = true;
                        return Some(());
                    };
                    if seen_chirho.insert(source_chirho.clone()) {
                        bound_chirho.push(source_chirho);
                    }
                }
                let result_chirho = self.prepare_type_pattern_variables_chirho(
                    body_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
                for source_chirho in bound_chirho {
                    seen_chirho.remove(source_chirho);
                }
                result_chirho?;
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    self.prepare_type_pattern_constraint_chirho(
                        constraint_chirho,
                        seen_chirho,
                        bind_variables_chirho,
                        unknown_origin_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?;
                }
                self.prepare_type_pattern_variables_chirho(
                    body_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
            }
            _ => {}
        }
        Some(())
    }

    fn prepare_type_pattern_constraint_chirho(
        &mut self,
        constraint_chirho: &haskelujah_ast_chirho::ty_chirho::ConstraintChirho,
        seen_chirho: &mut HashSet<haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho>,
        bind_variables_chirho: bool,
        unknown_origin_chirho: &mut bool,
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
        depth_chirho: usize,
    ) -> Option<()> {
        use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;
        budget_chirho.work_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        match constraint_chirho {
            ConstraintChirho::KindAnnotationChirho {
                constraint_chirho,
                kind_chirho,
                ..
            } => {
                self.prepare_type_pattern_constraint_chirho(
                    constraint_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    kind_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?;
            }
            ConstraintChirho::ClassChirho {
                args_chirho,
                kind_args_chirho,
                ..
            } => {
                for term_chirho in kind_args_chirho.iter().chain(args_chirho) {
                    self.prepare_type_pattern_variables_chirho(
                        term_chirho,
                        seen_chirho,
                        bind_variables_chirho,
                        unknown_origin_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?;
                }
            }
            ConstraintChirho::QuantifiedChirho {
                vars_chirho,
                context_chirho,
                body_chirho,
                ..
            } => {
                let mut bound_chirho = Vec::new();
                for binder_chirho in vars_chirho {
                    if let Some(kind_chirho) = &binder_chirho.kind_annotation_chirho {
                        self.prepare_type_pattern_variables_chirho(
                            kind_chirho,
                            seen_chirho,
                            bind_variables_chirho,
                            unknown_origin_chirho,
                            budget_chirho,
                            depth_chirho + 1,
                        )?;
                    }
                    budget_chirho.node_chirho(1)?;
                    let Some(source_chirho) = binder_chirho.name_chirho.type_variable_chirho()
                    else {
                        *unknown_origin_chirho = true;
                        return Some(());
                    };
                    if seen_chirho.insert(source_chirho.clone()) {
                        bound_chirho.push(source_chirho);
                    }
                }
                for constraint_chirho in context_chirho {
                    self.prepare_type_pattern_constraint_chirho(
                        constraint_chirho,
                        seen_chirho,
                        bind_variables_chirho,
                        unknown_origin_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?;
                }
                let result_chirho = self.prepare_type_pattern_constraint_chirho(
                    body_chirho,
                    seen_chirho,
                    bind_variables_chirho,
                    unknown_origin_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                );
                for source_chirho in bound_chirho {
                    seen_chirho.remove(source_chirho);
                }
                result_chirho?;
            }
        }
        Some(())
    }

    pub(super) fn install_pattern_givens_chirho(&mut self, predicates_chirho: Vec<PredChirho>) {
        let start_chirho = self.given_rewrites_chirho.len();
        self.install_given_equalities_chirho(&predicates_chirho);
        self.record_equality_rewrites_chirho(self.given_rewrites_chirho[start_chirho..].to_vec());
        self.given_preds_chirho.extend(predicates_chirho);
    }

    /// Meet only equal nominal datatype heads. Their result arguments are
    /// evidence of the match, not wanted equalities. A family component stays
    /// whole: no injectivity is inferred from its spelling or arguments.
    pub(super) fn match_gadt_result_chirho(
        &mut self,
        result_chirho: &TyChirho,
        scrutinee_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Option<SubstChirho> {
        let result_chirho = self.normalize_ty_chirho(result_chirho);
        let scrutinee_chirho = self.normalize_ty_chirho(scrutinee_chirho);
        let result_head_chirho = Self::ty_outer_head_name_chirho(&result_chirho)?;
        if Self::ty_outer_head_name_chirho(&scrutinee_chirho) != Some(result_head_chirho)
            || self.ty_is_family_app_chirho(&result_chirho)
        {
            return None;
        }
        let (_, result_arguments_chirho) = collect_app_spine_chirho(&result_chirho);
        let (_, scrutinee_arguments_chirho) = collect_app_spine_chirho(&scrutinee_chirho);
        if result_arguments_chirho.len() != scrutinee_arguments_chirho.len() {
            return None;
        }
        let mut combined_chirho = SubstChirho::empty_chirho();
        for (left_chirho, right_chirho) in result_arguments_chirho
            .into_iter()
            .zip(scrutinee_arguments_chirho)
        {
            let left_chirho = combined_chirho.apply_ty_chirho(&left_chirho);
            let right_chirho = combined_chirho.apply_ty_chirho(&right_chirho);
            if left_chirho == right_chirho {
                continue;
            }
            let ordinary_chirho = if self.ty_mentions_family_chirho(&left_chirho)
                || self.ty_mentions_family_chirho(&right_chirho)
            {
                // Binding a whole flexible variable is safe; decomposing a
                // family is not. No wanted is created by result matching.
                if matches!(left_chirho, TyChirho::VarChirho(_))
                    || matches!(right_chirho, TyChirho::VarChirho(_))
                {
                    self.unify_ordinary_chirho(&left_chirho, &right_chirho, span_chirho)
                        .ok()
                } else {
                    None
                }
            } else {
                self.unify_ordinary_chirho(&left_chirho, &right_chirho, span_chirho)
                    .ok()
                    .or_else(|| {
                        self.refine_skolems_by_unification_chirho(&left_chirho, &right_chirho)
                    })
            };
            if let Some(mut subst_chirho) = ordinary_chirho {
                if self.apply_subst_all_chirho(&mut subst_chirho).is_err() {
                    return None;
                };
                combined_chirho = subst_chirho.compose_chirho(&combined_chirho);
            } else {
                self.install_pattern_givens_chirho(vec![PredChirho::legacy_chirho(
                    crate::class_chirho::PredPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: "~".to_owned(),
                        ty_chirho: left_chirho,
                        extra_tys_chirho: vec![right_chirho],
                    },
                )]);
            }
        }
        Some(combined_chirho)
    }
}
