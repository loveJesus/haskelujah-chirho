// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Schemes: fresh variables, instantiation, deferred predicates, givens and generalization.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{FundepGeneralizationChirho, GeneralizationScopeChirho};
use crate::class_chirho::PredChirho;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    pub(super) fn coalesce_scheme_predicates_chirho(
        &mut self,
        predicates_chirho: Vec<SchemePredChirho>,
    ) -> Vec<SchemePredChirho> {
        crate::ty_chirho::OrderedPredicateSetChirho::with_metadata_work_chirho(
            predicates_chirho,
            self.class_env_chirho.registration_limit_chirho(),
            |weight_chirho| {
                self.class_env_chirho
                    .charge_predicate_metadata_chirho(weight_chirho)
            },
        )
        .into_vec_chirho()
    }

    /// Generate a fresh unification variable.
    pub fn fresh_var_chirho(&mut self) -> TyChirho {
        let var_chirho = TyVarChirho(self.elaboration_state_chirho.next_variable_chirho);
        self.elaboration_state_chirho.next_variable_chirho += 1;
        TyChirho::VarChirho(var_chirho)
    }

    /// Actual value producers share the checked TYPE r producer. Holes, error
    /// recovery and unknown imports keep fresh_var_chirho's unsorted behavior.
    /// workflow: type-identity-chirho/committed-meta-kinds-chirho
    pub(super) fn fresh_value_meta_chirho(&mut self, span_chirho: SpanChirho) -> TyChirho {
        if self.has_non_deferrable_failure_chirho() {
            return Self::terminal_commit_type_chirho();
        }
        if self.elaboration_state_chirho.owner_chirho.is_none() {
            return self.fresh_var_chirho();
        }
        let result_chirho = self
            .elaboration_state_chirho
            .fresh_value_kind_chirho(span_chirho)
            .and_then(|kind_chirho| {
                self.elaboration_state_chirho
                    .fresh_chirho(Some(kind_chirho), span_chirho)
            });
        match result_chirho {
            Ok(variable_chirho) => TyChirho::VarChirho(variable_chirho),
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                Self::terminal_commit_type_chirho()
            }
        }
    }

    pub(super) fn fresh_value_parameter_chirho(&mut self, span_chirho: SpanChirho) -> TyChirho {
        self.fresh_value_meta_chirho(span_chirho)
    }

    /// List expression syntax selects the intrinsic compiler List constructor,
    /// whose checked required parameter has kind Type. This is not a default
    /// for an unknown type variable or a lookup by the written name of a type.
    /// workflow: type-identity-chirho/committed-meta-kinds-chirho
    pub(super) fn fresh_list_element_meta_chirho(&mut self, span_chirho: SpanChirho) -> TyChirho {
        if self.has_non_deferrable_failure_chirho() {
            return Self::terminal_commit_type_chirho();
        }
        if self.elaboration_state_chirho.owner_chirho.is_none() {
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

    pub(super) fn instantiate_scheme_parts_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
    ) -> (TyChirho, Vec<PredChirho>) {
        if self.has_non_deferrable_failure_chirho() {
            return (Self::terminal_commit_type_chirho(), Vec::new());
        }
        let mut subst_chirho = SubstChirho::empty_chirho();
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        for record_chirho in scheme_chirho.vars_chirho.records_chirho() {
            let Some(kind_chirho) = self.instantiate_quantifier_kind_chirho(
                record_chirho,
                &subst_chirho,
                &mut budget_chirho,
            ) else {
                self.type_normalization_exhausted_chirho.set(true);
                return (Self::terminal_commit_type_chirho(), Vec::new());
            };
            let v_chirho = &record_chirho.variable_chirho;
            let fresh_chirho = self.fresh_var_chirho();
            self.note_fresh_quantifier_kind_chirho(&fresh_chirho, &kind_chirho);
            if self.local_generalized_vars_chirho.contains(v_chirho)
                && let TyChirho::VarChirho(fresh_var_chirho) = fresh_chirho
            {
                self.local_instantiations_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(fresh_var_chirho);
            }
            subst_chirho.insert_chirho(*v_chirho, fresh_chirho);
        }

        let Some((body_chirho, predicates_chirho)) = self.instantiate_scheme_terms_chirho(
            scheme_chirho,
            &scheme_chirho.ty_chirho,
            &subst_chirho,
            &mut budget_chirho,
        ) else {
            self.type_normalization_exhausted_chirho.set(true);
            return (self.fresh_var_chirho(), Vec::new());
        };
        self.expose_qualified_parts_chirho(body_chirho, predicates_chirho)
    }

    /// Align written @ arguments only with source-proven specified binders.
    /// Hidden/inferred binders are freshened without consuming a written slot.
    pub(super) fn instantiate_scheme_parts_with_visible_type_args_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        explicit_tys_chirho: &[TyChirho],
        explicit_kinds_chirho: &[Option<TyChirho>],
        span_chirho: SpanChirho,
    ) -> Option<(TyChirho, Vec<PredChirho>)> {
        let mut prefix_chirho = SubstChirho::empty_chirho();
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let mut groups_chirho = vec![&scheme_chirho.vars_chirho];
        let mut body_chirho = &scheme_chirho.ty_chirho;
        while let TyChirho::ForallChirho {
            vars_chirho,
            body_chirho: nested_body_chirho,
        } = body_chirho
        {
            if budget_chirho.node_chirho(1).is_none() {
                self.type_normalization_exhausted_chirho.set(true);
                return None;
            }
            groups_chirho.push(vars_chirho);
            body_chirho = nested_body_chirho;
        }
        let mut cursor_chirho = 0;
        for record_chirho in groups_chirho
            .into_iter()
            .flat_map(|group_chirho| group_chirho.records_chirho())
        {
            let specified_chirho = record_chirho.specificity_chirho
                != Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho);
            let Some(expected_kind_chirho) = self.instantiate_quantifier_kind_chirho(
                record_chirho,
                &prefix_chirho,
                &mut budget_chirho,
            ) else {
                self.type_normalization_exhausted_chirho.set(true);
                return None;
            };
            let argument_chirho = if specified_chirho && cursor_chirho < explicit_tys_chirho.len() {
                let argument_chirho = explicit_tys_chirho[cursor_chirho].clone();
                if let (Some(expected_chirho), Some(Some(actual_chirho))) = (
                    &expected_kind_chirho,
                    explicit_kinds_chirho.get(cursor_chirho),
                ) {
                    match crate::unify_chirho::unify_kind_terms_chirho(
                        expected_chirho,
                        actual_chirho,
                        span_chirho,
                    ) {
                        Ok(mut solution_chirho) => {
                            if self.apply_subst_all_chirho(&mut solution_chirho).is_err() {
                                return None;
                            };
                            prefix_chirho = solution_chirho.compose_chirho(&prefix_chirho);
                        }
                        Err(error_chirho) => {
                            self.report_unify_error_chirho(&error_chirho);
                            return None;
                        }
                    }
                }
                cursor_chirho += 1;
                argument_chirho
            } else {
                let argument_chirho = self.fresh_var_chirho();
                self.note_fresh_quantifier_kind_chirho(&argument_chirho, &expected_kind_chirho);
                argument_chirho
            };
            prefix_chirho.insert_chirho(record_chirho.variable_chirho, argument_chirho.clone());
        }
        if cursor_chirho != explicit_tys_chirho.len() {
            return None;
        }
        let Some((body_chirho, predicates_chirho)) = self.instantiate_scheme_terms_chirho(
            scheme_chirho,
            body_chirho,
            &prefix_chirho,
            &mut budget_chirho,
        ) else {
            self.type_normalization_exhausted_chirho.set(true);
            return None;
        };
        Some(self.expose_qualified_parts_chirho(body_chirho, predicates_chirho))
    }

    /// Instantiate a type scheme with fresh unification variables.
    /// Any predicates on the scheme are also instantiated and deferred.
    pub fn instantiate_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        span_chirho: SpanChirho,
    ) -> TyChirho {
        let (instantiated_ty_chirho, instantiated_preds_chirho) =
            self.instantiate_scheme_parts_chirho(scheme_chirho);
        for pred_inst_chirho in instantiated_preds_chirho {
            self.defer_pred_chirho(pred_inst_chirho, span_chirho);
        }
        instantiated_ty_chirho
    }

    /// Defer a wanted predicate: an equality (`~`) becomes a deferred
    /// equality to be unified once its stuck side is known, anything else a
    /// deferred class constraint. Every instantiation site goes through here.
    /// workflow: language-features-chirho/stuck-family-equalities-chirho
    pub(super) fn defer_pred_chirho(&mut self, pred_chirho: PredChirho, span_chirho: SpanChirho) {
        if pred_chirho.class_name_chirho == "~" {
            if let Some(rhs_chirho) = pred_chirho.extra_tys_chirho.first() {
                self.defer_equality_chirho(
                    pred_chirho.ty_chirho.clone(),
                    rhs_chirho.clone(),
                    span_chirho,
                );
            }
            return;
        }
        self.deferred_preds_chirho.push((pred_chirho, span_chirho));
    }

    pub(super) fn with_given_preds_chirho<TChirho, FChirho>(
        &mut self,
        preds_chirho: Vec<PredChirho>,
        f_chirho: FChirho,
    ) -> TChirho
    where
        FChirho: FnOnce(&mut Self) -> TChirho,
    {
        self.with_given_scope_chirho(preds_chirho, None, f_chirho, |_| ())
    }

    /// A type signature's scope: the givens of `with_given_preds_chirho`, and before they leave
    /// scope, a wanted over the signature's own rigid variables that nothing in scope can serve
    /// is reported (row 686, C08).
    /// workflow: language-features-chirho/stuck-family-equalities-chirho
    pub(super) fn with_signature_givens_chirho<TChirho, FChirho>(
        &mut self,
        preds_chirho: Vec<PredChirho>,
        signature_ty_chirho: &TyChirho,
        f_chirho: FChirho,
    ) -> TChirho
    where
        FChirho: FnOnce(&mut Self) -> TChirho,
    {
        self.with_signature_givens_and_finish_chirho(
            preds_chirho,
            signature_ty_chirho,
            f_chirho,
            |_| (),
        )
    }

    /// The finish operation sees the final committed solution while the same
    /// checked signature's equality evidence and givens remain in scope.
    pub(super) fn with_signature_givens_and_finish_chirho<TChirho>(
        &mut self,
        preds_chirho: Vec<PredChirho>,
        signature_ty_chirho: &TyChirho,
        f_chirho: impl FnOnce(&mut Self) -> TChirho,
        finish_chirho: impl FnOnce(&mut Self),
    ) -> TChirho {
        let skolems_chirho =
            self.signature_scope_skolems_chirho(signature_ty_chirho, &preds_chirho);
        self.with_given_scope_chirho(preds_chirho, Some(&skolems_chirho), f_chirho, finish_chirho)
    }

    fn with_given_scope_chirho<TChirho, FChirho>(
        &mut self,
        preds_chirho: Vec<PredChirho>,
        signature_skolems_chirho: Option<&HashSet<String>>,
        f_chirho: FChirho,
        finish_chirho: impl FnOnce(&mut Self),
    ) -> TChirho
    where
        FChirho: FnOnce(&mut Self) -> TChirho,
    {
        let given_start_chirho = self.given_preds_chirho.len();
        let rewrites_start_chirho = self.given_rewrites_chirho.len();
        let equality_owner_chirho =
            self.begin_equality_scope_chirho(if signature_skolems_chirho.is_some() {
                super::equality_scopes_chirho::EqualityScopeKindChirho::CheckedChirho
            } else {
                super::equality_scopes_chirho::EqualityScopeKindChirho::GivenChirho
            });
        self.install_given_equalities_chirho(&preds_chirho);
        self.record_equality_rewrites_chirho(
            self.given_rewrites_chirho[rewrites_start_chirho..].to_vec(),
        );
        self.given_preds_chirho.extend(preds_chirho);
        let result_chirho = f_chirho(self);
        let _improvement_chirho = self.discharge_deferred_with_current_givens_chirho();
        let _equalities_chirho = self.retry_deferred_equalities_chirho();
        if let Some(skolems_chirho) = signature_skolems_chirho {
            self.report_stuck_signature_wanteds_chirho(skolems_chirho);
        }
        finish_chirho(self);
        self.given_preds_chirho.truncate(given_start_chirho);
        self.given_rewrites_chirho.truncate(rewrites_start_chirho);
        self.end_equality_scope_chirho(equality_owner_chirho);
        result_chirho
    }

    /// Generalize a type into a scheme by quantifying over all free variables
    /// not free in the environment. Also partitions deferred predicates:
    /// predicates whose type variables are all being generalized become part
    /// of the scheme; the rest remain deferred.
    ///
    /// Deferred predicates are assumed to already have substitutions applied
    /// (via `apply_subst_all_chirho` during inference).
    pub fn generalize_chirho(&mut self, ty_chirho: &TyChirho) -> SchemeChirho {
        self.generalize_with_io_defaulting_chirho(
            ty_chirho,
            true,
            GeneralizationScopeChirho::TopLevelChirho,
        )
    }

    pub(super) fn generalize_local_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        publication_chirho: &mut SubstChirho,
    ) -> SchemeChirho {
        self.generalize_group_member_chirho(
            ty_chirho,
            false,
            GeneralizationScopeChirho::NestedChirho,
            false,
            publication_chirho,
        )
    }

    pub(super) fn generalize_with_io_defaulting_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        allow_io_defaulting_chirho: bool,
        scope_chirho: GeneralizationScopeChirho,
    ) -> SchemeChirho {
        let mut publication_chirho = SubstChirho::empty_chirho();
        let scheme_chirho = self.generalize_group_member_chirho(
            ty_chirho,
            allow_io_defaulting_chirho,
            scope_chirho,
            false,
            &mut publication_chirho,
        );
        publication_chirho.apply_scheme_chirho(&scheme_chirho)
    }

    /// Generalize one member of a declaration group. A member of a group the monomorphism
    /// restriction restricts never quantifies a constrained variable (Haskell 2010 section
    /// 4.5.5, Rule 1, row 692). Every variable of a predicate connected to its type stays
    /// monomorphic for a later use to fix, and the module's end resolves what is left.
    /// workflow: language-features-chirho/fundep-generalization-chirho
    pub(super) fn generalize_group_member_chirho(
        &mut self,
        ty_chirho: &TyChirho,
        allow_io_defaulting_chirho: bool,
        scope_chirho: GeneralizationScopeChirho,
        restricted_chirho: bool,
        publication_chirho: &mut SubstChirho,
    ) -> SchemeChirho {
        let mut env_fvs_chirho = self.env_chirho.free_vars_chirho();
        for pred_chirho in &self.given_preds_chirho {
            for kind_chirho in &pred_chirho.kind_tys_chirho {
                env_fvs_chirho.extend(kind_chirho.free_vars_chirho());
            }
            env_fvs_chirho.extend(pred_chirho.ty_chirho.free_vars_chirho());
            for extra_ty_chirho in &pred_chirho.extra_tys_chirho {
                env_fvs_chirho.extend(extra_ty_chirho.free_vars_chirho());
            }
        }
        // workflow: language-features-chirho/fundep-generalization-chirho
        let newly_mono_chirho = self.fundep_newly_mono_vars_chirho(&env_fvs_chirho);
        let mut ty_fvs_chirho = ty_chirho.free_vars_chirho();
        // Visit each connected predicate once: kinds may add variables absent
        // from the result term, while unrelated ambiguous wanteds stay outside.
        let mut owners_chirho: HashMap<TyVarChirho, Vec<usize>> = HashMap::new();
        let mut predicate_variables_chirho = Vec::new();
        for (index_chirho, (predicate_chirho, _)) in self.deferred_preds_chirho.iter().enumerate() {
            let variables_chirho = predicate_chirho
                .kind_tys_chirho
                .iter()
                .chain(std::iter::once(&predicate_chirho.ty_chirho))
                .chain(&predicate_chirho.extra_tys_chirho)
                .flat_map(TyChirho::free_vars_chirho)
                .collect::<Vec<_>>();
            for variable_chirho in &variables_chirho {
                owners_chirho
                    .entry(*variable_chirho)
                    .or_default()
                    .push(index_chirho);
            }
            predicate_variables_chirho.push(variables_chirho);
        }
        let mut seen_variables_chirho = ty_fvs_chirho.iter().copied().collect::<HashSet<_>>();
        let mut seen_predicates_chirho = HashSet::new();
        let mut index_chirho = 0;
        while let Some(variable_chirho) = ty_fvs_chirho.get(index_chirho).copied() {
            index_chirho += 1;
            for owner_chirho in owners_chirho.get(&variable_chirho).into_iter().flatten() {
                if seen_predicates_chirho.insert(*owner_chirho) {
                    for variable_chirho in &predicate_variables_chirho[*owner_chirho] {
                        if seen_variables_chirho.insert(*variable_chirho) {
                            ty_fvs_chirho.push(*variable_chirho);
                        }
                    }
                }
            }
        }
        // Rule 1 (row 692): a restricted member keeps every constrained variable monomorphic.
        // They are neither scoped nor rigid, so no representation or skolem protection applies.
        let withheld_chirho: HashSet<TyVarChirho> = if restricted_chirho {
            seen_predicates_chirho
                .iter()
                .flat_map(|owner_chirho| predicate_variables_chirho[*owner_chirho].iter().copied())
                .collect()
        } else {
            HashSet::new()
        };
        self.restriction_chirho
            .withhold_chirho(withheld_chirho.iter().copied());
        let vars_chirho: Vec<TyVarChirho> = ty_fvs_chirho
            .into_iter()
            .filter(|v_chirho| {
                !env_fvs_chirho.contains(v_chirho)
                    && !newly_mono_chirho.contains(v_chirho)
                    && !withheld_chirho.contains(v_chirho)
            })
            .collect();
        let mut scoped_chirho = env_fvs_chirho.clone();
        scoped_chirho.extend(newly_mono_chirho.iter().copied());
        let representation_defaults_chirho = (|| {
            if self.elaboration_state_chirho.owner_chirho.is_none() {
                return Ok(SubstChirho::empty_chirho());
            }
            let candidates_chirho = self
                .elaboration_state_chirho
                .kind_dependency_closure_chirho(&vars_chirho, SpanChirho::DUMMY_CHIRHO)?;
            let mut protected_chirho = scoped_chirho.to_vec();
            protected_chirho.extend(self.scoped_tyvars_chirho.values().copied());
            let protected_chirho = self
                .elaboration_state_chirho
                .kind_dependency_closure_chirho(&protected_chirho, SpanChirho::DUMMY_CHIRHO)?;
            self.elaboration_state_chirho
                .default_representation_variables_chirho(
                    &candidates_chirho,
                    &protected_chirho,
                    SpanChirho::DUMMY_CHIRHO,
                )
        })();
        let mut representation_defaults_chirho = match representation_defaults_chirho {
            Ok(defaults_chirho) => defaults_chirho,
            Err(error_chirho) => {
                self.record_body_kind_error_chirho(error_chirho);
                return SchemeChirho::mono_chirho(Self::terminal_commit_type_chirho());
            }
        };
        if self
            .apply_subst_all_chirho(&mut representation_defaults_chirho)
            .is_err()
        {
            return SchemeChirho::mono_chirho(Self::terminal_commit_type_chirho());
        };
        *publication_chirho = representation_defaults_chirho.compose_chirho(publication_chirho);
        let defaulted_ty_chirho = if representation_defaults_chirho.is_empty_chirho() {
            None
        } else {
            match self.elaboration_state_chirho.resolve_chirho(
                &representation_defaults_chirho.apply_ty_chirho(ty_chirho),
                SpanChirho::DUMMY_CHIRHO,
            ) {
                Ok(term_chirho) => Some(term_chirho),
                Err(error_chirho) => {
                    self.record_body_kind_error_chirho(error_chirho);
                    None
                }
            }
        };
        let ty_chirho = defaulted_ty_chirho.as_ref().unwrap_or(ty_chirho);
        let vars_chirho = vars_chirho
            .into_iter()
            .filter(|variable_chirho| {
                representation_defaults_chirho
                    .lookup_chirho(variable_chirho)
                    .is_none()
            })
            .collect();
        let vars_chirho = self.quantify_variables_chirho(
            vars_chirho,
            &[],
            &scoped_chirho,
            &SubstChirho::empty_chirho(),
            haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::InferredChirho,
            &mut super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho(),
        );
        let fundeps_chirho = FundepGeneralizationChirho {
            newly_mono_chirho,
            quantified_chirho: &vars_chirho,
            scope_chirho,
        };

        // Partition deferred predicates: those whose type vars are all being
        // generalized go into the scheme; the rest stay deferred.
        let mut scheme_preds_chirho = self.generalizable_equality_preds_chirho(&vars_chirho);
        let mut remaining_chirho = Vec::new();
        let drained_preds_chirho: Vec<(PredChirho, SpanChirho)> =
            self.deferred_preds_chirho.drain(..).collect();
        for wanted_chirho in drained_preds_chirho {
            let Some((pred_chirho, span_chirho)) = self.place_fundep_wanted_chirho(
                wanted_chirho,
                &fundeps_chirho,
                &mut scheme_preds_chirho,
                &mut remaining_chirho,
            ) else {
                continue;
            };
            let pred_fvs_chirho = pred_chirho
                .kind_tys_chirho
                .iter()
                .chain(std::iter::once(&pred_chirho.ty_chirho))
                .chain(&pred_chirho.extra_tys_chirho)
                .flat_map(TyChirho::free_vars_chirho)
                .collect::<Vec<_>>();
            // A predicate over a rigid signature variable belongs to the
            // enclosing signature's context: it is discharged by those givens
            // when their scope ends, or reported. It is never quantified away.
            let mentions_rigid_chirho = self.pred_mentions_rigid_chirho(&pred_chirho);
            if !mentions_rigid_chirho
                && pred_fvs_chirho
                    .iter()
                    .all(|v_chirho| vars_chirho.contains(v_chirho))
            {
                // Preserve unresolved contexts, but do not quantify away a
                // wanted whose fixed heads cannot match any actual instance of
                // a complete class. This includes Show (a -> a), even though a
                // remains open; later refinement cannot change the arrow head.
                if self.report_certainly_unsolvable_wanted_chirho(&pred_chirho, span_chirho) {
                    continue;
                }
                scheme_preds_chirho.push(SchemePredChirho::from_proof_chirho(
                    pred_chirho.class_identity_chirho().clone(),
                    crate::ty_chirho::SchemePredPartsChirho {
                        kind_tys_chirho: pred_chirho.kind_tys_chirho,
                        class_name_chirho: pred_chirho.class_name_chirho,
                        ty_chirho: pred_chirho.ty_chirho,
                        extra_tys_chirho: pred_chirho.extra_tys_chirho,
                    },
                ));
            } else {
                remaining_chirho.push((pred_chirho, span_chirho));
            }
        }
        self.deferred_preds_chirho = remaining_chirho;

        // Keep the first occurrence of each whole predicate, including extra MPTC types.
        // Adjacent-only dedup left Num b, Integral a, Num b with two dictionary indices.
        scheme_preds_chirho = self.coalesce_scheme_predicates_chirho(scheme_preds_chirho);

        // Context reduction (row 692): a predicate leaves only when another one entails it along
        // proven compiler superclass edges at the same complete arguments (`(Eq a, Ord a)`
        // keeps `Ord a`). An unknown or source participant stays, and every survivor keeps its
        // own proof. Num has no superclass in the seed.
        let keep_chirho: Vec<bool> = scheme_preds_chirho
            .iter()
            .map(|pred_chirho| {
                !scheme_preds_chirho.iter().any(|other_chirho| {
                    self.compiler_superclass_entails_chirho(other_chirho, pred_chirho)
                })
            })
            .collect();
        scheme_preds_chirho = scheme_preds_chirho
            .into_iter()
            .zip(keep_chirho)
            .filter(|(_, keep_chirho)| *keep_chirho)
            .map(|(p_chirho, _)| p_chirho)
            .collect();

        // Evidence-threading P2: top-level generalization can absorb
        // `(Num a, Fractional a)` into the binding scheme before module-end
        // defaulting sees it. Preserve the fractional evidence for any method
        // occurrence captured on the same bare variable so `negate 5.5`
        // dispatches through the Double row instead of the class-only Int
        // fallback used for truly unconstrained methods.
        let fractional_default_classes_chirho: &[&str] =
            &["Fractional", "Floating", "RealFrac", "RealFloat"];
        let mut pred_classes_by_var_chirho: HashMap<TyVarChirho, Vec<&str>> = HashMap::new();
        for pred_chirho in &scheme_preds_chirho {
            if let TyChirho::VarChirho(var_chirho) = &pred_chirho.ty_chirho {
                pred_classes_by_var_chirho
                    .entry(*var_chirho)
                    .or_default()
                    .push(pred_chirho.class_name_chirho.as_str());
            }
        }
        for (var_chirho, classes_chirho) in pred_classes_by_var_chirho {
            if classes_chirho
                .iter()
                .any(|class_chirho| fractional_default_classes_chirho.contains(class_chirho))
            {
                self.occurrence_default_hints_chirho
                    .insert(var_chirho, "Double".to_string());
            } else {
                self.occurrence_generalized_vars_chirho.insert(var_chirho);
            }
        }

        // IO defaulting: when a predicate is Monad/Applicative/Functor on a
        // bare type variable with no other non-IO-defaultable constraints,
        // default the variable to IO (like GHC's ExtendedDefaultRules).
        // This ensures `f = return 42` gets type `IO Int` rather than
        // `forall m. Monad m => m Int`, avoiding dictionary abstraction.
        let io_defaultable_chirho: &[&str] = &[
            "Monad",
            "Applicative",
            "Functor",
            "MonadIO",
            "MonadFail",
            "MonadFix",
        ];
        let mut io_default_subst_chirho = SubstChirho::empty_chirho();
        // Group predicates by type variable
        let mut var_pred_classes_chirho: std::collections::HashMap<TyVarChirho, Vec<&str>> =
            std::collections::HashMap::new();
        for pred_chirho in &scheme_preds_chirho {
            if let TyChirho::VarChirho(v_chirho) = &pred_chirho.ty_chirho {
                var_pred_classes_chirho
                    .entry(*v_chirho)
                    .or_default()
                    .push(&pred_chirho.class_name_chirho);
            }
        }
        // DETERMINISM: builds the IO-defaulting substitution — same hazard as the
        // defaulting loop in check_deferred_preds_chirho. Sort so the substitution is a
        // function of the program, not of the hasher.
        let mut sorted_var_pred_classes_chirho: Vec<_> = var_pred_classes_chirho.iter().collect();
        sorted_var_pred_classes_chirho.sort_by_key(|(var_chirho, _)| **var_chirho);
        for (var_chirho, classes_chirho) in sorted_var_pred_classes_chirho {
            let all_io_chirho = classes_chirho
                .iter()
                .all(|c_chirho| io_defaultable_chirho.contains(c_chirho));
            if all_io_chirho && !classes_chirho.is_empty() {
                io_default_subst_chirho
                    .insert_chirho(*var_chirho, TyChirho::ConChirho("IO".to_string()));
            }
        }
        if allow_io_defaulting_chirho && !io_default_subst_chirho.is_empty_chirho() {
            // Apply IO defaulting: substitute in the type, remove defaulted
            // vars, and remove satisfied predicates.
            let defaulted_ty_chirho = io_default_subst_chirho.apply_ty_chirho(ty_chirho);
            let defaulted_vars_chirho = vars_chirho.filter_chirho(|record_chirho| {
                io_default_subst_chirho
                    .lookup_chirho(&record_chirho.variable_chirho)
                    .is_none()
            });
            let defaulted_preds_chirho: Vec<SchemePredChirho> = scheme_preds_chirho
                .into_iter()
                .filter(|p_chirho| {
                    if let TyChirho::VarChirho(v_chirho) = &p_chirho.ty_chirho {
                        io_default_subst_chirho.lookup_chirho(v_chirho).is_none()
                    } else {
                        true
                    }
                })
                .collect();
            let scheme_chirho = SchemeChirho {
                vars_chirho: defaulted_vars_chirho,
                preds_chirho: defaulted_preds_chirho,
                ty_chirho: defaulted_ty_chirho,
            };
            let scheme_chirho = io_default_subst_chirho.apply_scheme_chirho(&scheme_chirho);
            let scheme_chirho = representation_defaults_chirho.apply_scheme_chirho(&scheme_chirho);
            if scope_chirho == GeneralizationScopeChirho::TopLevelChirho {
                self.record_inferred_dictionary_owners_chirho(&scheme_chirho);
            }
            return scheme_chirho;
        }

        let scheme_chirho = SchemeChirho {
            vars_chirho,
            preds_chirho: scheme_preds_chirho,
            ty_chirho: ty_chirho.clone(),
        };
        let scheme_chirho = representation_defaults_chirho.apply_scheme_chirho(&scheme_chirho);
        if scope_chirho == GeneralizationScopeChirho::TopLevelChirho {
            self.record_inferred_dictionary_owners_chirho(&scheme_chirho);
        }
        scheme_chirho
    }

    /// Whether `given_chirho` entails `wanted_chirho` along proven compiler superclass edges
    /// (row 692). Both must name a compiler class whose active record is that compiler record.
    /// From the given, each step instantiates the proven seed templates of an active compiler
    /// record at that predicate's complete arguments; a derived superclass counts only when it
    /// is itself an active compiler record, and the wanted is reached only with the same class,
    /// complete arguments and proof. An unknown, source or shadowed class, a template that is
    /// not proven, or an exhausted search proves nothing, so the wanted stays.
    /// workflow: language-features-chirho/fundep-generalization-chirho
    pub(super) fn compiler_superclass_entails_chirho(
        &self,
        given_chirho: &SchemePredChirho,
        wanted_chirho: &SchemePredChirho,
    ) -> bool {
        const STEPS_CHIRHO: usize = 64;
        let wanted_chirho = PredChirho::from_scheme_chirho(wanted_chirho);
        if self
            .class_env_chirho
            .active_compiler_record_chirho(&wanted_chirho)
            .is_none()
        {
            return false;
        }
        let given_chirho = PredChirho::from_scheme_chirho(given_chirho);
        let mut seen_chirho = HashSet::from([given_chirho.clone()]);
        let mut pending_chirho = vec![given_chirho];
        let mut steps_chirho = 0usize;
        while let Some(current_chirho) = pending_chirho.pop() {
            if steps_chirho == STEPS_CHIRHO {
                return false;
            }
            steps_chirho += 1;
            let Some(record_chirho) = self
                .class_env_chirho
                .active_compiler_record_chirho(&current_chirho)
            else {
                continue;
            };
            let Some(superclasses_chirho) = record_chirho.superclass_preds_at_chirho(
                &current_chirho.kind_tys_chirho,
                &current_chirho.all_tys_chirho(),
            ) else {
                continue;
            };
            for superclass_chirho in superclasses_chirho {
                if self
                    .class_env_chirho
                    .active_compiler_record_chirho(&superclass_chirho)
                    .is_none()
                {
                    continue;
                }
                if superclass_chirho == wanted_chirho
                    && superclass_chirho.class_identity_chirho()
                        == wanted_chirho.class_identity_chirho()
                {
                    return true;
                }
                if seen_chirho.insert(superclass_chirho.clone()) {
                    pending_chirho.push(superclass_chirho);
                }
            }
        }
        false
    }

    /// Check if `potential_super_chirho` is a (transitive) superclass of
    /// `sub_chirho`. For example, `Eq` is a superclass of `Ord`, and
    /// transitively of `Real`. Each class is
    /// visited once, so a superclass cycle, legal under
    /// UndecidableSuperClasses (T16414's `All` and `All2`), ends the walk.
    /// workflow: language-features-chirho/rigid-type-variables-chirho
    pub(super) fn is_transitive_superclass_chirho(
        &self,
        potential_super_chirho: &str,
        sub_chirho: &str,
    ) -> bool {
        let mut visited_chirho = HashSet::from([sub_chirho.to_string()]);
        let mut pending_chirho = vec![sub_chirho.to_string()];
        while let Some(class_name_chirho) = pending_chirho.pop() {
            for super_chirho in self
                .class_env_chirho
                .superclasses_chirho(&class_name_chirho)
            {
                if super_chirho == potential_super_chirho {
                    return true;
                }
                if visited_chirho.insert(super_chirho.clone()) {
                    pending_chirho.push(super_chirho);
                }
            }
        }
        false
    }
}

#[cfg(test)]
#[path = "schemes_resource_tests_chirho.rs"]
mod resource_tests_chirho;

#[cfg(test)]
#[path = "../types_chirho/meta_producer_tests_chirho.rs"]
mod meta_producer_tests_chirho;
