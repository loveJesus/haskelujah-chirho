// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Local bindings, right-hand sides and list-comprehension qualifiers.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{binding_groups_chirho, canonical_value_name_text_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{SchemeChirho, TyChirho};
use haskelujah_ast_chirho::expr_chirho::{MatchArmChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    /// Infer one sequential branch of a list comprehension in isolation.
    ///
    /// ParallelListComp branches share their outer environment, but a branch
    /// cannot see names introduced by a sibling. The returned bindings are
    /// reintroduced together only while inferring the result expression.
    ///
    /// workflow: parallel-list-comprehensions-chirho
    pub(super) fn infer_list_comp_qual_group_chirho(
        &mut self,
        quals_chirho: &[StmtChirho],
        span_chirho: SpanChirho,
    ) -> (
        SubstChirho,
        Vec<(String, SchemeChirho)>,
        super::constructor_patterns_chirho::TypePatternAliasesChirho,
    ) {
        let mut subst_chirho = SubstChirho::empty_chirho();
        let mut bound_names_chirho = Vec::new();
        let mut seen_names_chirho = HashSet::new();
        let type_pattern_scope_mark_chirho = self.type_pattern_alias_undo_chirho.len();
        self.env_chirho.push_scope_chirho();

        for qual_chirho in quals_chirho {
            match qual_chirho {
                StmtChirho::ExprChirho { expr_chirho, .. } => {
                    let (guard_subst_chirho, guard_ty_chirho) = self.infer_expr_chirho(expr_chirho);
                    subst_chirho = guard_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&guard_subst_chirho);
                    let guard_ty_sub_chirho = subst_chirho.apply_ty_chirho(&guard_ty_chirho);
                    if let Ok(unify_subst_chirho) = self.unify_normalized_chirho(
                        &guard_ty_sub_chirho,
                        &TyChirho::bool_chirho(),
                        span_chirho,
                    ) {
                        subst_chirho = unify_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&unify_subst_chirho);
                    }
                }
                StmtChirho::BindChirho {
                    pat_chirho,
                    expr_chirho,
                    ..
                } => {
                    let (source_subst_chirho, source_ty_chirho) =
                        self.infer_expr_chirho(expr_chirho);
                    subst_chirho = source_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&source_subst_chirho);

                    let elem_ty_chirho = self.fresh_var_chirho();
                    let list_ty_chirho = TyChirho::ListChirho(Box::new(elem_ty_chirho.clone()));
                    let source_ty_sub_chirho = subst_chirho.apply_ty_chirho(&source_ty_chirho);
                    if let Ok(unify_subst_chirho) = self.unify_normalized_chirho(
                        &source_ty_sub_chirho,
                        &list_ty_chirho,
                        span_chirho,
                    ) {
                        subst_chirho = unify_subst_chirho.compose_chirho(&subst_chirho);
                        self.apply_subst_all_chirho(&unify_subst_chirho);
                    }

                    let bound_ty_chirho = subst_chirho.apply_ty_chirho(&elem_ty_chirho);
                    let pat_subst_chirho = self.bind_pat_chirho(pat_chirho, &bound_ty_chirho);
                    subst_chirho = pat_subst_chirho.compose_chirho(&subst_chirho);
                    self.apply_subst_all_chirho(&pat_subst_chirho);
                    for name_chirho in crate::linearity_chirho::pat_bound_names_chirho(pat_chirho) {
                        if seen_names_chirho.insert(name_chirho.clone()) {
                            bound_names_chirho.push(name_chirho);
                        }
                    }
                }
                StmtChirho::LetChirho { binds_chirho, .. } => {
                    self.infer_local_binds_chirho(binds_chirho, &mut subst_chirho);
                    for bind_chirho in binds_chirho {
                        match bind_chirho {
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                                name_chirho,
                                ..
                            } => {
                                let name_chirho = canonical_value_name_text_chirho(
                                    name_chirho.text_chirho(),
                                );
                                if seen_names_chirho.insert(name_chirho.clone()) {
                                    bound_names_chirho.push(name_chirho);
                                }
                            }
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                                pat_chirho,
                                ..
                            } => {
                                for name_chirho in
                                    crate::linearity_chirho::pat_bound_names_chirho(pat_chirho)
                                {
                                    if seen_names_chirho.insert(name_chirho.clone()) {
                                        bound_names_chirho.push(name_chirho);
                                    }
                                }
                            }
                            haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                                ..
                            } => {}
                        }
                    }
                }
            }
        }

        let bindings_chirho = bound_names_chirho
            .into_iter()
            .filter_map(|name_chirho| {
                self.env_chirho
                    .lookup_chirho(&name_chirho)
                    .cloned()
                    .map(|scheme_chirho| {
                        (
                            name_chirho,
                            subst_chirho.apply_scheme_chirho(&scheme_chirho),
                        )
                    })
            })
            .collect();
        let aliases_chirho = self.type_pattern_aliases_since_chirho(type_pattern_scope_mark_chirho);
        self.restore_type_pattern_aliases_chirho(type_pattern_scope_mark_chirho);
        self.env_chirho.pop_scope_chirho();
        (subst_chirho, bindings_chirho, aliases_chirho)
    }

    /// Pre-bind all variable names in a pattern with fresh type variables.
    /// This enables mutually recursive `where`/`let` pattern bindings
    /// (letrec semantics), e.g. `(x,y) = (y+1, x+2)`.
    fn pre_bind_pat_vars_chirho(&mut self, pat_chirho: &PatChirho) {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                let fresh_chirho = self.fresh_var_chirho();
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(fresh_chirho),
                );
            }
            PatChirho::TupleChirho {
                elements_chirho, ..
            } => {
                for elem_chirho in elements_chirho {
                    self.pre_bind_pat_vars_chirho(elem_chirho);
                }
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                let fresh_chirho = self.fresh_var_chirho();
                self.env_chirho.bind_chirho(
                    name_chirho.text_chirho().to_string(),
                    SchemeChirho::mono_chirho(fresh_chirho),
                );
                self.pre_bind_pat_vars_chirho(pattern_chirho);
            }
            PatChirho::ConChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    self.pre_bind_pat_vars_chirho(arg_chirho);
                }
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                self.pre_bind_pat_vars_chirho(left_chirho);
                self.pre_bind_pat_vars_chirho(right_chirho);
            }
            PatChirho::ListChirho {
                elements_chirho, ..
            } => {
                for elem_chirho in elements_chirho {
                    self.pre_bind_pat_vars_chirho(elem_chirho);
                }
            }
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. } => {
                self.pre_bind_pat_vars_chirho(inner_chirho);
            }
            PatChirho::RecordChirho { fields_chirho, .. } => {
                for field_chirho in fields_chirho {
                    self.pre_bind_pat_vars_chirho(&field_chirho.pattern_chirho);
                }
            }
            PatChirho::ViewChirho {
                pat_chirho: inner_chirho,
                ..
            }
            | PatChirho::TypeAnnotChirho {
                pat_chirho: inner_chirho,
                ..
            } => {
                self.pre_bind_pat_vars_chirho(inner_chirho);
            }
            // Wildcards, literals, negated literals: no bindings
            _ => {}
        }
    }

    /// Infer the type of a right-hand side.
    pub(super) fn infer_rhs_chirho(&mut self, rhs_chirho: &RhsChirho) -> (SubstChirho, TyChirho) {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.infer_expr_chirho(expr_chirho),
            RhsChirho::GuardedChirho(guarded_chirho) => {
                self.infer_guard_arms_chirho(guarded_chirho)
            }
        }
    }

    pub(super) fn infer_rhs_against_expected_chirho(
        &mut self,
        rhs_chirho: &RhsChirho,
        expected_ty_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> (SubstChirho, TyChirho) {
        match rhs_chirho {
            RhsChirho::UnguardedChirho(expr_chirho) => self.infer_expr_against_expected_chirho(
                expr_chirho,
                expected_ty_chirho,
                span_chirho,
            ),
            RhsChirho::GuardedChirho(guarded_chirho) => {
                self.infer_guard_arms_against_expected_chirho(guarded_chirho, expected_ty_chirho)
            }
        }
    }

    pub(super) fn infer_local_binds_chirho(
        &mut self,
        binds_chirho: &[haskelujah_ast_chirho::expr_chirho::LocalBindChirho],
        subst_chirho: &mut SubstChirho,
    ) {
        let mut local_sigs_chirho: HashMap<String, haskelujah_ast_chirho::ty_chirho::TypeChirho> =
            HashMap::new();
        for bind_chirho in binds_chirho {
            if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } = bind_chirho
            {
                local_sigs_chirho.insert(
                    canonical_value_name_text_chirho(name_chirho.text_chirho()),
                    ty_chirho.clone(),
                );
            }
        }

        let mut fun_names_chirho: Vec<String> = Vec::new();
        let mut fun_matches_refs_chirho: Vec<&[MatchArmChirho]> = Vec::new();
        let mut fun_spans_chirho: Vec<SpanChirho> = Vec::new();
        let mut fun_pre_tys_chirho: Vec<TyChirho> = Vec::new();
        let mut local_skolemized_sigs_chirho: HashMap<String, TyChirho> = HashMap::new();

        for bind_chirho in binds_chirho {
            match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => {
                    let name_str_chirho =
                        canonical_value_name_text_chirho(name_chirho.text_chirho());
                    let pre_ty_chirho =
                        if let Some(sig_ast_chirho) = local_sigs_chirho.get(&name_str_chirho) {
                            let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                            let (sig_ty_chirho, _sig_preds_chirho) =
                                self.instantiate_scheme_parts_chirho(&sig_scheme_chirho);
                            self.env_chirho
                                .bind_chirho(name_str_chirho.clone(), sig_scheme_chirho);
                            sig_ty_chirho
                        } else {
                            let fresh_ty_chirho = self.fresh_var_chirho();
                            self.env_chirho.bind_chirho(
                                name_str_chirho.clone(),
                                SchemeChirho::mono_chirho(fresh_ty_chirho.clone()),
                            );
                            fresh_ty_chirho
                        };
                    fun_names_chirho.push(name_str_chirho);
                    fun_matches_refs_chirho.push(matches_chirho.as_slice());
                    fun_spans_chirho.push(*span_chirho);
                    fun_pre_tys_chirho.push(pre_ty_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    ..
                } => {
                    self.pre_bind_pat_vars_chirho(pat_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho { .. } => {}
            }
        }

        let fun_decls_for_scc_chirho: Vec<(usize, &[MatchArmChirho])> = fun_matches_refs_chirho
            .iter()
            .enumerate()
            .map(|(index_chirho, matches_chirho)| (index_chirho, *matches_chirho))
            .collect();
        let signed_names_chirho: HashSet<String> = if self.relaxed_poly_rec_chirho {
            local_sigs_chirho.keys().cloned().collect()
        } else {
            HashSet::new()
        };
        let groups_chirho = binding_groups_chirho(
            &fun_names_chirho,
            &fun_decls_for_scc_chirho,
            &signed_names_chirho,
        );

        for group_chirho in &groups_chirho {
            let mut equality_owners_chirho = Vec::new();
            let mut group_inferred_chirho: Vec<(String, TyChirho, SpanChirho)> = Vec::new();
            for &fun_index_chirho in group_chirho {
                let name_str_chirho = fun_names_chirho[fun_index_chirho].clone();
                let span_chirho = fun_spans_chirho[fun_index_chirho];
                let prev_scoped_chirho = self.scoped_tyvars_chirho.clone();
                let equality_owner_chirho = self.begin_equality_scope_chirho(
                    if local_sigs_chirho.contains_key(&name_str_chirho) {
                        super::equality_scopes_chirho::EqualityScopeKindChirho::CheckedChirho
                    } else {
                        super::equality_scopes_chirho::EqualityScopeKindChirho::BindingChirho
                    },
                );
                equality_owners_chirho.push(equality_owner_chirho);

                let (fun_subst_chirho, inferred_ty_chirho) =
                    if let Some(sig_ast_chirho) = local_sigs_chirho.get(&name_str_chirho) {
                        let (sig_scheme_chirho, sig_var_map_chirho) =
                            self.ast_type_to_scheme_with_var_map_chirho(sig_ast_chirho);
                        let (sig_ty_raw_chirho, sig_preds_chirho) =
                            self.skolemize_scheme_parts_chirho(&sig_scheme_chirho);
                        self.record_own_dictionary_indices_chirho(&sig_preds_chirho);
                        self.scope_signature_for_body_chirho(sig_ast_chirho, &sig_var_map_chirho);
                        local_skolemized_sigs_chirho
                            .insert(name_str_chirho.clone(), sig_ty_raw_chirho.clone());
                        self.with_signature_givens_chirho(
                            sig_preds_chirho,
                            &sig_ty_raw_chirho,
                            |self_chirho| {
                                self_chirho.infer_matches_against_expected_chirho(
                                    fun_matches_refs_chirho[fun_index_chirho],
                                    span_chirho,
                                    &sig_ty_raw_chirho,
                                )
                            },
                        )
                    } else {
                        self.infer_matches_chirho(
                            fun_matches_refs_chirho[fun_index_chirho],
                            span_chirho,
                        )
                    };
                *subst_chirho = fun_subst_chirho.compose_chirho(subst_chirho);
                self.apply_subst_all_chirho(&fun_subst_chirho);

                let pre_ty_sub_chirho =
                    subst_chirho.apply_ty_chirho(&fun_pre_tys_chirho[fun_index_chirho]);
                if !local_sigs_chirho.contains_key(&name_str_chirho)
                    && let Ok(unify_subst_chirho) = self.unify_normalized_chirho(
                        &pre_ty_sub_chirho,
                        &inferred_ty_chirho,
                        span_chirho,
                    )
                {
                    *subst_chirho = unify_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&unify_subst_chirho);
                }

                group_inferred_chirho.push((name_str_chirho, inferred_ty_chirho, span_chirho));
                self.scoped_tyvars_chirho = prev_scoped_chirho;
                self.end_equality_scope_chirho(equality_owner_chirho);
            }

            let inferred_equality_types_chirho = group_inferred_chirho
                .iter()
                .filter(|(name_chirho, _, _)| !local_sigs_chirho.contains_key(name_chirho))
                .map(|(_, ty_chirho, _)| subst_chirho.apply_ty_chirho(ty_chirho))
                .collect::<Vec<_>>();
            let previous_generalizing_chirho = std::mem::replace(
                &mut self.equality_generalizing_owners_chirho,
                equality_owners_chirho.iter().copied().collect(),
            );
            for ((name_str_chirho, inferred_ty_chirho, span_chirho), equality_owner_chirho) in
                group_inferred_chirho
                    .into_iter()
                    .zip(&equality_owners_chirho)
            {
                let previous_owner_chirho = self.active_equality_scope_chirho;
                self.active_equality_scope_chirho = *equality_owner_chirho;
                let mut binding_scheme_chirho = None;
                if let Some(sig_ast_chirho) = local_sigs_chirho.get(&name_str_chirho) {
                    let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                    let sig_full_chirho = if sig_scheme_chirho.vars_chirho.is_empty() {
                        sig_scheme_chirho.ty_chirho.clone()
                    } else {
                        TyChirho::ForallChirho {
                            vars_chirho: sig_scheme_chirho.vars_chirho.clone(),
                            body_chirho: Box::new(sig_scheme_chirho.ty_chirho.clone()),
                        }
                    };
                    let sig_ty_chirho = match local_skolemized_sigs_chirho.get(&name_str_chirho) {
                        Some(skolemized_chirho) => self
                            .normalize_ty_chirho(&subst_chirho.apply_ty_chirho(skolemized_chirho)),
                        None => self.normalize_ty_chirho(&sig_full_chirho),
                    };
                    let inferred_sub_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);
                    let inferred_sub_chirho = self.normalize_ty_chirho(&inferred_sub_chirho);
                    match self.subsume_normalized_chirho(
                        &inferred_sub_chirho,
                        &sig_ty_chirho,
                        span_chirho,
                    ) {
                        Ok(sig_subst_chirho) => {
                            *subst_chirho = sig_subst_chirho.compose_chirho(subst_chirho);
                            self.apply_subst_all_chirho(&sig_subst_chirho);
                            binding_scheme_chirho = Some(sig_scheme_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                self.env_chirho.remove_chirho(&name_str_chirho);
                let inferred_sub_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);
                let generalized_chirho = binding_scheme_chirho
                    .unwrap_or_else(|| self.generalize_local_chirho(&inferred_sub_chirho));
                self.local_generalized_vars_chirho
                    .extend(generalized_chirho.vars_chirho.iter().copied());
                self.env_chirho
                    .bind_chirho(name_str_chirho, generalized_chirho);
                self.active_equality_scope_chirho = previous_owner_chirho;
            }
            self.equality_generalizing_owners_chirho = previous_generalizing_chirho;
            self.finish_equality_binding_group_chirho(
                &equality_owners_chirho,
                &inferred_equality_types_chirho,
            );
        }

        for bind_chirho in binds_chirho {
            match bind_chirho {
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::PatBindChirho {
                    pat_chirho,
                    rhs_chirho,
                    ..
                } => {
                    let (rhs_subst_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(rhs_chirho);
                    *subst_chirho = rhs_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&rhs_subst_chirho);
                    let pat_subst_chirho = self.bind_binding_pat_chirho(pat_chirho, &rhs_ty_chirho);
                    *subst_chirho = pat_subst_chirho.compose_chirho(subst_chirho);
                    self.apply_subst_all_chirho(&pat_subst_chirho);
                }
                haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho { .. }
                | haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho { .. } => {}
            }
        }
    }
}
