// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Constructor pattern implications. Expression uses keep ordinary instantiation.
//! Workflow: language-features-chirho/pattern-matching-chirho.md.

use super::*;

pub(super) struct TypePatternAliasUndoChirho {
    name_chirho: String,
    variable_chirho: TyVarChirho,
    previous_name_chirho: Option<TyVarChirho>,
    previous_image_chirho: Option<crate::subst_chirho::SubstitutionImageChirho>,
}

pub(super) struct PatternImplicationChirho {
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
    pub(super) fn begin_pattern_implication_chirho(&mut self) -> PatternImplicationChirho {
        self.equality_pattern_depth_chirho += 1;
        PatternImplicationChirho {
            owner_chirho: self.begin_equality_scope_chirho(
                super::equality_scopes_chirho::EqualityScopeKindChirho::PatternChirho,
            ),
            given_start_chirho: self.given_preds_chirho.len(),
            rewrite_start_chirho: self.given_rewrites_chirho.len(),
            first_meta_chirho: self.next_var_chirho,
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
        let improvement_chirho = self.discharge_deferred_with_current_givens_chirho();
        let equalities_chirho = self.retry_deferred_equalities_chirho();
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

    pub(super) fn instantiate_constructor_pattern_with_variables_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (Vec<TyChirho>, TyChirho, Vec<PredChirho>, SubstChirho) {
        let mut binders_chirho = scheme_chirho.vars_chirho.to_vec();
        let mut fields_chirho = Vec::new();
        let mut remaining_chirho = &scheme_chirho.ty_chirho;
        loop {
            match remaining_chirho {
                TyChirho::ForallChirho {
                    vars_chirho,
                    body_chirho,
                }
                | TyChirho::RequiredForallChirho {
                    vars_chirho,
                    body_chirho,
                } => {
                    binders_chirho.extend(vars_chirho.iter().copied());
                    remaining_chirho = body_chirho;
                }
                TyChirho::FunChirho(field_chirho, result_chirho, _) => {
                    fields_chirho.push(field_chirho.as_ref());
                    remaining_chirho = result_chirho;
                }
                _ => break,
            }
        }
        binders_chirho.sort_by_key(|var_chirho| var_chirho.0);
        binders_chirho.dedup();
        let mut result_parameters_chirho = HashSet::new();
        let mut pending_chirho = Vec::new();
        let mut remaining_work_chirho = 16_384_usize;
        let mut head_chirho = remaining_chirho;
        while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
            let Some(next_work_chirho) = remaining_work_chirho.checked_sub(1) else {
                break;
            };
            remaining_work_chirho = next_work_chirho;
            pending_chirho.push(argument_chirho.as_ref());
            head_chirho = fun_chirho;
        }
        match remaining_chirho {
            TyChirho::ListChirho(element_chirho) => pending_chirho.push(element_chirho),
            TyChirho::TupleChirho(elements_chirho)
                if elements_chirho.len() <= remaining_work_chirho =>
            {
                remaining_work_chirho -= elements_chirho.len();
                pending_chirho.extend(elements_chirho);
            }
            _ => {}
        }
        while let Some(component_chirho) = pending_chirho.pop() {
            let Some(next_work_chirho) = remaining_work_chirho.checked_sub(1) else {
                break;
            };
            remaining_work_chirho = next_work_chirho;
            match component_chirho {
                TyChirho::VarChirho(var_chirho) => {
                    result_parameters_chirho.insert(*var_chirho);
                }
                TyChirho::TupleChirho(elements_chirho)
                    if elements_chirho.len() <= remaining_work_chirho =>
                {
                    remaining_work_chirho -= elements_chirho.len();
                    pending_chirho.extend(elements_chirho);
                }
                TyChirho::ListChirho(element_chirho) => pending_chirho.push(element_chirho),
                TyChirho::FunChirho(argument_chirho, result_chirho, _) => {
                    pending_chirho.extend([argument_chirho.as_ref(), result_chirho.as_ref()])
                }
                TyChirho::AppChirho(..) => {
                    let mut head_chirho = component_chirho;
                    let mut nested_chirho = Vec::new();
                    while let TyChirho::AppChirho(fun_chirho, argument_chirho) = head_chirho {
                        let Some(next_work_chirho) = remaining_work_chirho.checked_sub(1) else {
                            break;
                        };
                        remaining_work_chirho = next_work_chirho;
                        nested_chirho.push(argument_chirho.as_ref());
                        head_chirho = fun_chirho;
                    }
                    if let TyChirho::ConChirho(name_chirho) = head_chirho
                        && matches!(super::family_registration_chirho::family_head_role_chirho(&self.type_families_chirho, name_chirho), super::closed_family_apartness_chirho::FamilyHeadRoleChirho::GenerativeChirho { .. })
                    { pending_chirho.extend(nested_chirho); }
                }
                _ => {}
            }
        }
        let mut instantiation_chirho = SubstChirho::empty_chirho();
        for var_chirho in binders_chirho {
            let image_chirho = if result_parameters_chirho.contains(&var_chirho) {
                self.fresh_var_chirho()
            } else {
                let base_chirho = self
                    .tyvar_source_names_chirho
                    .get(&var_chirho)
                    .map(String::as_str)
                    .unwrap_or("existentialChirho");
                let name_chirho = skolem_name_chirho(base_chirho, self.next_skolem_chirho);
                self.next_skolem_chirho += 1;
                self.equality_scopes_chirho[self.active_equality_scope_chirho]
                    .skolems_chirho
                    .insert(name_chirho.clone());
                TyChirho::ForallVarChirho(name_chirho)
            };
            instantiation_chirho.insert_chirho(var_chirho, image_chirho);
        }
        let predicates_chirho = scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| {
                PredChirho::from_proof_chirho(
                    pred_chirho.class_identity_chirho().clone(),
                    crate::class_chirho::PredPartsChirho {
                        kind_tys_chirho: pred_chirho
                            .kind_tys_chirho
                            .iter()
                            .map(|kind_chirho| instantiation_chirho.apply_ty_chirho(kind_chirho))
                            .collect(),
                        class_name_chirho: pred_chirho.class_name_chirho.clone(),
                        ty_chirho: instantiation_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                        extra_tys_chirho: pred_chirho
                            .extra_tys_chirho
                            .iter()
                            .map(|ty_chirho| instantiation_chirho.apply_ty_chirho(ty_chirho))
                            .collect(),
                    },
                )
            })
            .collect();
        (
            fields_chirho
                .into_iter()
                .map(|field_chirho| instantiation_chirho.apply_ty_chirho(field_chirho))
                .collect(),
            instantiation_chirho.apply_ty_chirho(remaining_chirho),
            predicates_chirho,
            instantiation_chirho,
        )
    }

    pub(super) fn type_pattern_aliases_since_chirho(
        &self,
        mark_chirho: usize,
    ) -> TypePatternAliasesChirho {
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let mut aliases_chirho = Vec::new();
        for undo_chirho in &self.type_pattern_alias_undo_chirho[mark_chirho..] {
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
                        .node_chirho(undo_chirho.name_chirho.len().saturating_add(1))
                        .is_some()
                }) {
                    aliases_chirho.push((
                        undo_chirho.name_chirho.clone(),
                        undo_chirho.variable_chirho,
                        crate::subst_chirho::SubstitutionImageChirho {
                            image_chirho: copied_chirho,
                            kind_chirho: image_chirho.kind_chirho.clone(),
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
            if let Some(previous_chirho) = undo_chirho.previous_name_chirho {
                self.scoped_tyvars_chirho
                    .insert(undo_chirho.name_chirho, previous_chirho);
            } else {
                self.scoped_tyvars_chirho.remove(&undo_chirho.name_chirho);
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

    pub(super) fn bind_checked_type_pattern_alias_chirho(
        &mut self,
        name_chirho: &NameChirho,
        image_chirho: TyChirho,
    ) {
        let mut variables_chirho = HashMap::new();
        let variable_chirho = self.surface_variable_chirho(name_chirho, &mut variables_chirho);
        self.install_type_pattern_alias_chirho(
            name_chirho.text_chirho().to_owned(),
            variable_chirho,
            image_chirho,
        );
    }

    pub(super) fn install_type_pattern_alias_chirho(
        &mut self,
        name_chirho: String,
        variable_chirho: TyVarChirho,
        image_chirho: TyChirho,
    ) {
        let kind_chirho = match &image_chirho {
            TyChirho::VarChirho(variable_chirho) => self
                .elaboration_variable_kinds_chirho
                .get(variable_chirho)
                .and_then(|record_chirho| record_chirho.kind_chirho.clone()),
            _ => None,
        };
        self.install_type_pattern_alias_image_chirho(
            name_chirho,
            variable_chirho,
            crate::subst_chirho::SubstitutionImageChirho {
                image_chirho,
                kind_chirho,
            },
        );
    }

    pub(super) fn install_type_pattern_alias_image_chirho(
        &mut self,
        name_chirho: String,
        variable_chirho: TyVarChirho,
        image_chirho: crate::subst_chirho::SubstitutionImageChirho,
    ) {
        let previous_name_chirho = self
            .scoped_tyvars_chirho
            .insert(name_chirho.clone(), variable_chirho);
        let previous_image_chirho = self
            .skolem_bindings_chirho
            .lookup_image_chirho(&variable_chirho)
            .cloned();
        self.skolem_bindings_chirho
            .insert_image_chirho(variable_chirho, image_chirho);
        self.type_pattern_alias_undo_chirho
            .push(TypePatternAliasUndoChirho {
                name_chirho: name_chirho.clone(),
                variable_chirho,
                previous_name_chirho,
                previous_image_chirho,
            });
    }

    pub(super) fn bind_constructor_type_pattern_chirho(
        &mut self,
        pattern_chirho: &TypeChirho,
        expected_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let mut seen_chirho = HashSet::new();
        if self
            .prepare_type_pattern_variables_chirho(
                pattern_chirho,
                &mut seen_chirho,
                &mut budget_chirho,
                0,
            )
            .is_none()
        {
            self.type_normalization_exhausted_chirho.set(true);
            return SubstChirho::empty_chirho();
        }
        self.bind_prepared_constructor_type_pattern_chirho(
            pattern_chirho,
            expected_chirho,
            span_chirho,
        )
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
                self.bind_checked_type_pattern_alias_chirho(name_chirho, expected_chirho.clone());
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
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
        depth_chirho: usize,
    ) -> Option<()> {
        budget_chirho.work_chirho(1)?;
        if depth_chirho > 128 {
            return None;
        }
        match pattern_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let source_chirho = name_chirho.type_variable_chirho()?;
                if seen_chirho.insert(source_chirho.clone()) {
                    budget_chirho.node_chirho(name_chirho.text_chirho().len().saturating_add(1))?;
                    let variable_chirho = self.fresh_var_chirho();
                    self.bind_checked_type_pattern_alias_chirho(name_chirho, variable_chirho);
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
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    arg_chirho,
                    seen_chirho,
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
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    result_chirho,
                    seen_chirho,
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
                        budget_chirho,
                        depth_chirho + 1,
                    )?;
                }
            }
            TypeChirho::ListChirho { element_chirho, .. } => self
                .prepare_type_pattern_variables_chirho(
                    element_chirho,
                    seen_chirho,
                    budget_chirho,
                    depth_chirho + 1,
                )?,
            TypeChirho::ParenChirho { inner_chirho, .. } => self
                .prepare_type_pattern_variables_chirho(
                    inner_chirho,
                    seen_chirho,
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
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    kind_chirho,
                    seen_chirho,
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
                            budget_chirho,
                            depth_chirho + 1,
                        )?;
                    }
                    budget_chirho.node_chirho(1)?;
                    let source_chirho = binder_chirho.name_chirho.type_variable_chirho()?;
                    if seen_chirho.insert(source_chirho.clone()) {
                        bound_chirho.push(source_chirho);
                    }
                }
                let result_chirho = self.prepare_type_pattern_variables_chirho(
                    body_chirho,
                    seen_chirho,
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
                        budget_chirho,
                        depth_chirho + 1,
                    )?;
                }
                self.prepare_type_pattern_variables_chirho(
                    body_chirho,
                    seen_chirho,
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
                    budget_chirho,
                    depth_chirho + 1,
                )?;
                self.prepare_type_pattern_variables_chirho(
                    kind_chirho,
                    seen_chirho,
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
                            budget_chirho,
                            depth_chirho + 1,
                        )?;
                    }
                    budget_chirho.node_chirho(1)?;
                    let source_chirho = binder_chirho.name_chirho.type_variable_chirho()?;
                    if seen_chirho.insert(source_chirho.clone()) {
                        bound_chirho.push(source_chirho);
                    }
                }
                for constraint_chirho in context_chirho {
                    self.prepare_type_pattern_constraint_chirho(
                        constraint_chirho,
                        seen_chirho,
                        budget_chirho,
                        depth_chirho + 1,
                    )?;
                }
                let result_chirho = self.prepare_type_pattern_constraint_chirho(
                    body_chirho,
                    seen_chirho,
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
            if let Some(subst_chirho) = ordinary_chirho {
                self.apply_subst_all_chirho(&subst_chirho);
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
