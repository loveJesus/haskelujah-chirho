// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Owned equality implications. Each frame stores only its new evidence and a
//! shared parent link; no growing type environment is cloned at a scope entry.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EqualityScopeKindChirho {
    ModuleChirho,
    BindingChirho,
    CheckedChirho,
    PatternChirho,
    GivenChirho,
}

#[derive(Clone, Debug)]
pub(super) struct WantedEqualityChirho {
    pub(super) lhs_chirho: TyChirho,
    pub(super) rhs_chirho: TyChirho,
    pub(super) span_chirho: SpanChirho,
    pub(super) owner_chirho: usize,
    pub(super) generalized_chirho: bool,
    generalized_by_chirho: HashSet<usize>,
}

#[derive(Debug)]
struct EqualityEvidenceChirho {
    parent_chirho: Option<Arc<EqualityEvidenceChirho>>,
    rewrites_chirho: Vec<(TyChirho, TyChirho)>,
}

#[derive(Debug)]
pub(super) struct EqualityScopeChirho {
    parent_chirho: Option<usize>,
    kind_chirho: EqualityScopeKindChirho,
    evidence_chirho: Option<Arc<EqualityEvidenceChirho>>,
    pub(super) skolems_chirho: HashSet<String>,
    pub(super) complete_chirho: bool,
}

impl EqualityScopeChirho {
    pub(super) fn root_chirho() -> Self {
        Self {
            parent_chirho: None,
            kind_chirho: EqualityScopeKindChirho::ModuleChirho,
            evidence_chirho: None,
            skolems_chirho: HashSet::new(),
            complete_chirho: true,
        }
    }
}

impl InferCtxChirho {
    pub(super) fn begin_equality_scope_chirho(
        &mut self,
        kind_chirho: EqualityScopeKindChirho,
    ) -> usize {
        let parent_chirho = self.active_equality_scope_chirho;
        let owner_chirho = self.equality_scopes_chirho.len();
        self.equality_scopes_chirho.push(EqualityScopeChirho {
            parent_chirho: Some(parent_chirho),
            kind_chirho,
            evidence_chirho: self.equality_scopes_chirho[parent_chirho]
                .evidence_chirho
                .clone(),
            skolems_chirho: HashSet::new(),
            complete_chirho: self.equality_scopes_chirho[parent_chirho].complete_chirho,
        });
        self.active_equality_scope_chirho = owner_chirho;
        owner_chirho
    }

    pub(super) fn end_equality_scope_chirho(&mut self, owner_chirho: usize) {
        debug_assert_eq!(self.active_equality_scope_chirho, owner_chirho);
        self.active_equality_scope_chirho = self.equality_scopes_chirho[owner_chirho]
            .parent_chirho
            .unwrap_or(0);
    }

    pub(super) fn record_equality_rewrites_chirho(
        &mut self,
        rewrites_chirho: Vec<(TyChirho, TyChirho)>,
    ) {
        if rewrites_chirho.is_empty() {
            return;
        }
        let scope_chirho = &mut self.equality_scopes_chirho[self.active_equality_scope_chirho];
        scope_chirho.evidence_chirho = Some(Arc::new(EqualityEvidenceChirho {
            parent_chirho: scope_chirho.evidence_chirho.clone(),
            rewrites_chirho,
        }));
    }

    pub(super) fn defer_equality_chirho(
        &mut self,
        lhs_chirho: TyChirho,
        rhs_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) {
        self.deferred_equalities_chirho.push(WantedEqualityChirho {
            lhs_chirho,
            rhs_chirho,
            span_chirho,
            owner_chirho: self.active_equality_scope_chirho,
            generalized_chirho: false,
            generalized_by_chirho: HashSet::new(),
        });
    }

    pub(super) fn normalize_equality_type_chirho(
        &self,
        ty_chirho: &TyChirho,
        owner_chirho: usize,
    ) -> Result<TyChirho, super::equality_normalization_chirho::EqualityNormalizationExhaustedChirho>
    {
        let evidence_chirho = std::iter::successors(
            self.equality_scopes_chirho[owner_chirho]
                .evidence_chirho
                .as_deref(),
            |node_chirho| node_chirho.parent_chirho.as_deref(),
        );
        super::equality_normalization_chirho::normalize_owned_equality_chirho(
            self,
            ty_chirho,
            evidence_chirho.map(|node_chirho| node_chirho.rewrites_chirho.as_slice()),
        )
    }

    /// A whole-meta equality may improve from an accessible GIVEN image only
    /// after BOTH complete wanted sides become equal. The meta must be absent
    /// from all given evidence, so trial substitution cannot alter a premise.
    /// Failed candidates never mutate inference state or become new givens.
    pub(super) fn improve_equality_from_owned_givens_chirho(
        &self,
        wanted_chirho: &WantedEqualityChirho,
    ) -> Result<
        Option<SubstChirho>,
        super::equality_normalization_chirho::EqualityNormalizationExhaustedChirho,
    > {
        use super::equality_normalization_chirho::{
            EqualityBudgetChirho, EqualityNormalizationExhaustedChirho,
            copy_term_with_budget_chirho, equal_chirho,
            normalize_owned_equality_with_budget_chirho, substitute_meta_with_budget_chirho,
        };
        let evidence_chirho = || {
            std::iter::successors(
                self.equality_scopes_chirho[wanted_chirho.owner_chirho]
                    .evidence_chirho
                    .as_deref(),
                |node_chirho| node_chirho.parent_chirho.as_deref(),
            )
            .map(|node_chirho| node_chirho.rewrites_chirho.as_slice())
        };
        let mut budget_chirho = EqualityBudgetChirho::new_chirho();
        let left_chirho = normalize_owned_equality_with_budget_chirho(
            self,
            &wanted_chirho.lhs_chirho,
            evidence_chirho(),
            &mut budget_chirho,
        )?;
        let right_chirho = normalize_owned_equality_with_budget_chirho(
            self,
            &wanted_chirho.rhs_chirho,
            evidence_chirho(),
            &mut budget_chirho,
        )?;
        let variable_chirho = match (&left_chirho, &right_chirho) {
            (TyChirho::VarChirho(variable_chirho), _)
            | (_, TyChirho::VarChirho(variable_chirho)) => *variable_chirho,
            _ => return Ok(None),
        };
        for chunk_chirho in evidence_chirho() {
            budget_chirho
                .work_chirho(chunk_chirho.len())
                .ok_or(EqualityNormalizationExhaustedChirho)?;
            for (left_chirho, right_chirho) in chunk_chirho {
                for term_chirho in [left_chirho, right_chirho] {
                    // Preflight includes depth, names and every predicate term.
                    let bounded_chirho =
                        copy_term_with_budget_chirho(term_chirho, &mut budget_chirho)
                            .ok_or(EqualityNormalizationExhaustedChirho)?;
                    if bounded_chirho.free_vars_chirho().contains(&variable_chirho) {
                        return Ok(None);
                    }
                }
            }
        }
        for chunk_chirho in evidence_chirho() {
            for (_, candidate_chirho) in chunk_chirho {
                budget_chirho
                    .work_chirho(1)
                    .ok_or(EqualityNormalizationExhaustedChirho)?;
                let image_chirho = normalize_owned_equality_with_budget_chirho(
                    self,
                    candidate_chirho,
                    evidence_chirho(),
                    &mut budget_chirho,
                )?;
                if image_chirho.free_vars_chirho().contains(&variable_chirho) {
                    continue;
                }
                let left_chirho = substitute_meta_with_budget_chirho(
                    &wanted_chirho.lhs_chirho,
                    variable_chirho,
                    &image_chirho,
                    &mut budget_chirho,
                )
                .ok_or(EqualityNormalizationExhaustedChirho)?;
                let right_chirho = substitute_meta_with_budget_chirho(
                    &wanted_chirho.rhs_chirho,
                    variable_chirho,
                    &image_chirho,
                    &mut budget_chirho,
                )
                .ok_or(EqualityNormalizationExhaustedChirho)?;
                let left_chirho = normalize_owned_equality_with_budget_chirho(
                    self,
                    &left_chirho,
                    evidence_chirho(),
                    &mut budget_chirho,
                )?;
                let right_chirho = normalize_owned_equality_with_budget_chirho(
                    self,
                    &right_chirho,
                    evidence_chirho(),
                    &mut budget_chirho,
                )?;
                if equal_chirho(&left_chirho, &right_chirho, &mut budget_chirho)
                    .ok_or(EqualityNormalizationExhaustedChirho)?
                {
                    return Ok(Some(SubstChirho::singleton_chirho(
                        variable_chirho,
                        image_chirho,
                    )));
                }
            }
        }
        Ok(None)
    }

    pub(super) fn equality_scope_contains_chirho(
        &self,
        ancestor_chirho: usize,
        mut owner_chirho: usize,
    ) -> bool {
        loop {
            if owner_chirho == ancestor_chirho {
                return true;
            }
            let Some(parent_chirho) = self.equality_scopes_chirho[owner_chirho].parent_chirho
            else {
                return false;
            };
            owner_chirho = parent_chirho;
        }
    }

    fn equality_has_checked_owner_chirho(&self, mut owner_chirho: usize) -> bool {
        loop {
            if self.equality_scopes_chirho[owner_chirho].kind_chirho
                == EqualityScopeKindChirho::CheckedChirho
            {
                return true;
            }
            // An inferred local binding owns its own generalization even
            // inside an enclosing checked function.
            if self.equality_scopes_chirho[owner_chirho].kind_chirho
                == EqualityScopeKindChirho::BindingChirho
            {
                return false;
            }
            let Some(parent_chirho) = self.equality_scopes_chirho[owner_chirho].parent_chirho
            else {
                return false;
            };
            owner_chirho = parent_chirho;
        }
    }

    pub(super) fn generalizable_equality_preds_chirho(
        &mut self,
        vars_chirho: &[TyVarChirho],
    ) -> Vec<SchemePredChirho> {
        let pending_chirho = std::mem::take(&mut self.deferred_equalities_chirho);
        let mut predicates_chirho = Vec::new();
        for mut wanted_chirho in pending_chirho {
            let admitted_chirho = if self.equality_generalizing_owners_chirho.is_empty() {
                self.equality_scope_contains_chirho(
                    self.active_equality_scope_chirho,
                    wanted_chirho.owner_chirho,
                )
            } else {
                let mut cursor_chirho = Some(wanted_chirho.owner_chirho);
                let mut admitted_chirho = false;
                while let Some(owner_chirho) = cursor_chirho {
                    if self
                        .equality_generalizing_owners_chirho
                        .contains(&owner_chirho)
                    {
                        admitted_chirho = true;
                        break;
                    }
                    cursor_chirho = self.equality_scopes_chirho[owner_chirho].parent_chirho;
                }
                admitted_chirho
            };
            if !admitted_chirho {
                self.deferred_equalities_chirho.push(wanted_chirho);
                continue;
            }
            let (Ok(lhs_chirho), Ok(rhs_chirho)) = (
                self.normalize_equality_type_chirho(
                    &wanted_chirho.lhs_chirho,
                    wanted_chirho.owner_chirho,
                ),
                self.normalize_equality_type_chirho(
                    &wanted_chirho.rhs_chirho,
                    wanted_chirho.owner_chirho,
                ),
            ) else {
                self.deferred_equalities_chirho.push(wanted_chirho);
                continue;
            };
            let free_chirho = lhs_chirho
                .free_vars_chirho()
                .into_iter()
                .chain(rhs_chirho.free_vars_chirho())
                .collect::<HashSet<_>>();
            if !self.equality_has_checked_owner_chirho(wanted_chirho.owner_chirho)
                && self.equality_scopes_chirho[wanted_chirho.owner_chirho].complete_chirho
                && !free_chirho.is_empty()
                && free_chirho
                    .iter()
                    .all(|var_chirho| vars_chirho.contains(var_chirho))
                && !self.type_contains_skolem_chirho(&lhs_chirho)
                && !self.type_contains_skolem_chirho(&rhs_chirho)
            {
                predicates_chirho.push(SchemePredChirho::legacy_chirho(
                    crate::ty_chirho::SchemePredPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: "~".to_owned(),
                        ty_chirho: lhs_chirho,
                        extra_tys_chirho: vec![rhs_chirho],
                    },
                ));
                wanted_chirho.generalized_chirho = true;
                wanted_chirho
                    .generalized_by_chirho
                    .insert(self.active_equality_scope_chirho);
            }
            self.deferred_equalities_chirho.push(wanted_chirho);
        }
        predicates_chirho
    }

    /// Called after all substitutions and generalizations of one SCC. Checked
    /// bindings cannot acquire an unproved equality; inferred ones retain only
    /// equalities every member of the recursive group can actually generalize.
    pub(super) fn finish_equality_binding_group_chirho(
        &mut self,
        owners_chirho: &[usize],
        inferred_types_chirho: &[TyChirho],
    ) {
        let _subst_chirho = self.retry_deferred_equalities_chirho();
        let pending_chirho = std::mem::take(&mut self.deferred_equalities_chirho);
        for wanted_chirho in pending_chirho {
            if !owners_chirho.iter().any(|owner_chirho| {
                self.equality_scope_contains_chirho(*owner_chirho, wanted_chirho.owner_chirho)
            }) || !self.equality_scopes_chirho[wanted_chirho.owner_chirho].complete_chirho
            {
                self.deferred_equalities_chirho.push(wanted_chirho);
                continue;
            }
            let (Ok(lhs_chirho), Ok(rhs_chirho)) = (
                self.normalize_equality_type_chirho(
                    &wanted_chirho.lhs_chirho,
                    wanted_chirho.owner_chirho,
                ),
                self.normalize_equality_type_chirho(
                    &wanted_chirho.rhs_chirho,
                    wanted_chirho.owner_chirho,
                ),
            ) else {
                self.report_equality_normalization_exhausted_chirho(wanted_chirho.span_chirho);
                continue;
            };
            if lhs_chirho == rhs_chirho {
                continue;
            }
            let free_chirho = lhs_chirho
                .free_vars_chirho()
                .into_iter()
                .chain(rhs_chirho.free_vars_chirho())
                .collect::<HashSet<_>>();
            let retained_chirho = wanted_chirho.generalized_chirho
                && !free_chirho.is_empty()
                && !self.equality_has_checked_owner_chirho(wanted_chirho.owner_chirho)
                && owners_chirho.iter().all(|owner_chirho| {
                    self.equality_scopes_chirho[*owner_chirho].kind_chirho
                        != EqualityScopeKindChirho::CheckedChirho
                        && wanted_chirho.generalized_by_chirho.contains(owner_chirho)
                })
                && inferred_types_chirho.iter().all(|ty_chirho| {
                    let type_vars_chirho = ty_chirho.free_vars_chirho();
                    free_chirho
                        .iter()
                        .all(|var_chirho| type_vars_chirho.contains(var_chirho))
                });
            if !retained_chirho {
                self.report_unify_error_chirho(&UnifyErrorChirho::MismatchChirho {
                    expected_chirho: lhs_chirho,
                    actual_chirho: rhs_chirho,
                    span_chirho: wanted_chirho.span_chirho,
                });
            }
        }
    }

    pub(super) fn report_equality_normalization_exhausted_chirho(
        &mut self,
        span_chirho: SpanChirho,
    ) {
        self.diagnostics_chirho.push_chirho(DiagnosticChirho::error_with_code_chirho(
            ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
            "equality normalization exceeded its bounded work or type-size allowance; equality remains unproved",
            span_chirho,
        ));
    }

    pub(super) fn check_pattern_equality_scope_chirho(
        &mut self,
        owner_chirho: usize,
        result_chirho: &TyChirho,
        first_meta_chirho: u32,
        event_start_chirho: usize,
        span_chirho: SpanChirho,
    ) -> SubstChirho {
        if self.equality_scopes_chirho[owner_chirho]
            .skolems_chirho
            .is_empty()
        {
            return SubstChirho::empty_chirho();
        }
        match self.pattern_equality_escape_chirho(
            owner_chirho,
            result_chirho,
            first_meta_chirho,
            event_start_chirho,
        ) {
            Ok((false, public_subst_chirho)) => {
                // These keys already occur in the delta. Materialization is
                // not another inference event; outer frames read their new
                // images through the same previously retained keys.
                let pattern_depth_chirho = self.equality_pattern_depth_chirho;
                self.equality_pattern_depth_chirho = 0;
                self.apply_subst_all_chirho(&public_subst_chirho);
                self.equality_pattern_depth_chirho = pattern_depth_chirho;
                return public_subst_chirho;
            }
            Ok((true, _)) => {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(TYPE_MISMATCH_CODE_CHIRHO),
                        "constructor pattern's existential type would escape its scope",
                        span_chirho,
                    ))
            }
            Err(_) => self.report_equality_normalization_exhausted_chirho(span_chirho),
        }
        SubstChirho::empty_chirho()
    }

    fn pattern_equality_escape_chirho(
        &self,
        owner_chirho: usize,
        result_chirho: &TyChirho,
        first_meta_chirho: u32,
        event_start_chirho: usize,
    ) -> Result<
        (bool, SubstChirho),
        super::equality_normalization_chirho::EqualityNormalizationExhaustedChirho,
    > {
        use super::equality_normalization_chirho::{
            EqualityBudgetChirho, EqualityNormalizationExhaustedChirho,
            normalize_owned_equality_with_budget_chirho,
        };
        if self.equality_substitution_events_exhausted_chirho {
            return Err(EqualityNormalizationExhaustedChirho);
        }
        let mut budget_chirho = EqualityBudgetChirho::new_chirho();
        let skolems_chirho = &self.equality_scopes_chirho[owner_chirho].skolems_chirho;
        let mut public_type_chirho = |ty_chirho: &TyChirho| -> Result<
            (TyChirho, bool),
            EqualityNormalizationExhaustedChirho,
        > {
            // Preserve an outer signature variable's public identity. A local
            // refinement may expose existential components without exporting
            // those components from the implication.
            let raw_chirho = normalize_owned_equality_with_budget_chirho(
                self,
                ty_chirho,
                std::iter::empty(),
                &mut budget_chirho,
            )?;
            if !type_mentions_owned_skolem_chirho(&raw_chirho, skolems_chirho, &mut budget_chirho)?
            {
                return Ok((raw_chirho, false));
            }
            let evidence_chirho = std::iter::successors(
                self.equality_scopes_chirho[owner_chirho]
                    .evidence_chirho
                    .as_deref(),
                |node_chirho| node_chirho.parent_chirho.as_deref(),
            );
            let normalized_chirho = normalize_owned_equality_with_budget_chirho(
                self,
                &raw_chirho,
                evidence_chirho.map(|node_chirho| node_chirho.rewrites_chirho.as_slice()),
                &mut budget_chirho,
            )?;
            let escaping_chirho = type_mentions_owned_skolem_chirho(
                &normalized_chirho,
                skolems_chirho,
                &mut budget_chirho,
            )?;
            Ok((normalized_chirho, escaping_chirho))
        };
        if public_type_chirho(result_chirho)?.1 {
            return Ok((true, SubstChirho::empty_chirho()));
        }
        let mut seen_chirho = HashSet::new();
        let mut public_subst_chirho = SubstChirho::empty_chirho();
        for var_chirho in &self.equality_substitution_events_chirho[event_start_chirho..] {
            // Storage is capped before append; normalize each older meta's
            // image once with this implication's shared work/output budget.
            if var_chirho.0 < first_meta_chirho
                && seen_chirho.insert(*var_chirho)
                && let Some(image_chirho) =
                    self.equality_substitutions_chirho.lookup_chirho(var_chirho)
            {
                let (image_chirho, escaping_chirho) = public_type_chirho(image_chirho)?;
                if escaping_chirho {
                    return Ok((true, SubstChirho::empty_chirho()));
                }
                public_subst_chirho.insert_chirho(*var_chirho, image_chirho);
            }
        }
        Ok((false, public_subst_chirho))
    }
}

fn type_mentions_owned_skolem_chirho(
    ty_chirho: &TyChirho,
    skolems_chirho: &HashSet<String>,
    budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
) -> Result<bool, super::equality_normalization_chirho::EqualityNormalizationExhaustedChirho> {
    use super::equality_normalization_chirho::EqualityNormalizationExhaustedChirho;
    let mut pending_chirho = vec![ty_chirho];
    while let Some(ty_chirho) = pending_chirho.pop() {
        budget_chirho
            .work_chirho(1)
            .ok_or(EqualityNormalizationExhaustedChirho)?;
        match ty_chirho {
            TyChirho::ForallVarChirho(name_chirho) => {
                budget_chirho
                    .work_chirho(name_chirho.len())
                    .ok_or(EqualityNormalizationExhaustedChirho)?;
                if skolems_chirho.contains(name_chirho) {
                    return Ok(true);
                }
            }
            TyChirho::AppChirho(a_chirho, b_chirho)
            | TyChirho::FunChirho(a_chirho, b_chirho, _) => {
                pending_chirho.extend([a_chirho.as_ref(), b_chirho.as_ref()])
            }
            TyChirho::TupleChirho(elements_chirho) => pending_chirho.extend(elements_chirho),
            TyChirho::ListChirho(element_chirho)
            | TyChirho::ForallChirho {
                body_chirho: element_chirho,
                ..
            }
            | TyChirho::RequiredForallChirho {
                body_chirho: element_chirho,
                ..
            } => pending_chirho.push(element_chirho),
            _ => {}
        }
    }
    Ok(false)
}

impl InferCtxChirho {
    /// Preserve checked implication evidence while exporting a public image.
    /// The same allowance covers evidence copies, rewrites and normalization.
    pub(super) fn normalize_signature_public_image_chirho(
        &self,
        ty_chirho: &TyChirho,
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
    ) -> Result<TyChirho, super::equality_normalization_chirho::EqualityNormalizationExhaustedChirho>
    {
        let evidence_chirho = std::iter::successors(
            self.equality_scopes_chirho[self.active_equality_scope_chirho]
                .evidence_chirho
                .as_deref(),
            |node_chirho| node_chirho.parent_chirho.as_deref(),
        );
        super::equality_normalization_chirho::normalize_owned_equality_with_budget_chirho(
            self,
            ty_chirho,
            evidence_chirho.map(|node_chirho| node_chirho.rewrites_chirho.as_slice()),
            budget_chirho,
        )
    }
}
