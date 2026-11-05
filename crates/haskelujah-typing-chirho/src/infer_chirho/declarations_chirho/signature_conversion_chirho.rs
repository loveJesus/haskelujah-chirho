// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Signature schemes, predicate conversion, and signature-level binder scope.
//! Workflow: language-features-chirho/rank-n-visible-type-application-chirho.md.

use std::collections::HashMap;

use haskelujah_ast_chirho::ty_chirho::{ConstraintChirho as AstConstraintChirho, TypeChirho};
use haskelujah_span_chirho::SpanChirho;

use super::{InferCtxChirho, signature_binders_chirho};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{
    QuantifiedVariableChirho, QuantifiedVariablesChirho, SchemeChirho, SchemePredChirho, TyChirho,
    TyVarChirho,
};

impl InferCtxChirho {
    /// Convert an AST `TypeChirho` to a `SchemeChirho` (with quantified variables
    /// and constraints extracted).
    pub fn ast_type_to_scheme_chirho(&mut self, ast_ty_chirho: &TypeChirho) -> SchemeChirho {
        self.ast_type_to_scheme_with_var_map_chirho(ast_ty_chirho).0
    }

    /// `ast_type_to_scheme_chirho`, also returning a name-to-variable map.
    /// The first syntactically outermost forall group's identities are retained
    /// for `scope_signature_for_body_chirho`; callers must use that operation,
    /// not export this entire map into the definition's lexical scope.
    pub fn ast_type_to_scheme_with_var_map_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
    ) -> (SchemeChirho, HashMap<String, TyVarChirho>) {
        let seed_chirho = self.scoped_tyvars_chirho.clone();
        self.ast_type_to_scheme_seeded_chirho(ast_ty_chirho, &seed_chirho, true)
    }

    /// Convert a signature to a scheme with `seed_chirho` pre-binding some
    /// type-variable names. When `seeded_are_bound_chirho` is true the seeded
    /// variables belong to an enclosing scope (ScopedTypeVariables) and are
    /// never quantified here; when false they are ordinary binders of this
    /// scheme (a class method signature quantifies its class's variables).
    pub(super) fn ast_type_to_scheme_seeded_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        seed_chirho: &HashMap<String, TyVarChirho>,
        seeded_are_bound_chirho: bool,
    ) -> (SchemeChirho, HashMap<String, TyVarChirho>) {
        // Seed var_map with scoped type variables so that where-clause annotations
        // referring to the enclosing function's forall-bound vars reuse the same TyVarChirho.
        let mut var_map_chirho: HashMap<String, TyVarChirho> = seed_chirho.clone();
        let mut source_binders_chirho = Vec::new();
        let mut scheme_preds_chirho = Vec::new();
        let mut equality_pairs_chirho = Vec::new();
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let body_ast_chirho = self.prepare_signature_body_chirho(
            ast_ty_chirho,
            &mut var_map_chirho,
            &mut source_binders_chirho,
            &mut scheme_preds_chirho,
            &mut equality_pairs_chirho,
            &mut budget_chirho,
        );
        let explicit_forall_vars_chirho: Vec<_> = source_binders_chirho
            .iter()
            .map(|record_chirho| record_chirho.variable_chirho)
            .collect();
        // Only the first syntactically outermost forall scopes the RHS. A
        // later leading forall can shadow its spelling within the signature,
        // so retain the first binder identities before the returned map is used.
        let body_bindings_chirho: Vec<_> = match ast_ty_chirho {
            TypeChirho::ForallChirho { vars_chirho, .. } => vars_chirho
                .iter()
                .zip(explicit_forall_vars_chirho.iter())
                .map(|(name_chirho, var_chirho)| {
                    (name_chirho.text_chirho().to_string(), *var_chirho)
                })
                .collect(),
            _ => Vec::new(),
        };
        // Keep family applications opaque in the published term scheme. Quick Look
        // may expand synonyms, but reducing a family here would invent information
        // that ordinary implicit instantiation is not allowed to use.
        let previous_conversion_chirho = self.definition_type_conversion_chirho;
        self.definition_type_conversion_chirho = true;
        let ty_chirho = self.ast_type_to_ty_chirho(body_ast_chirho, &mut var_map_chirho);
        self.definition_type_conversion_chirho = previous_conversion_chirho;
        let ty_chirho = self.expand_type_synonyms_chirho(&ty_chirho);
        if let Some(license_chirho) = self.type_validity_license_chirho {
            let expanded_validity_chirho = crate::validity_chirho::check_expanded_type_chirho(
                &ty_chirho,
                license_chirho,
                self.type_families_chirho.declaration_schemes_chirho(),
                ast_ty_chirho.span_chirho(),
            );
            for diagnostic_chirho in expanded_validity_chirho.diagnostics_chirho() {
                self.non_deferrable_diagnostics_chirho
                    .push_chirho(diagnostic_chirho.clone());
            }
        }
        let (outer_forall_vars_chirho, inner_ty_chirho) = self.split_typed_scheme_prefix_chirho(
            ty_chirho,
            &mut scheme_preds_chirho,
            &mut equality_pairs_chirho,
        );

        // Apply equality constraints: for `(a ~ b) => T`, unify `a` and `b`
        // so the scheme body reflects the equality. Keep a local substitution
        // because these variables can be bound by the outer source `forall`;
        // applying only to the global inference state does not rewrite the
        // scheme we are about to return.
        let mut equality_subst_chirho = SubstChirho::empty_chirho();
        for (lhs_chirho, rhs_chirho) in &equality_pairs_chirho {
            let lhs_sub_chirho = equality_subst_chirho.apply_ty_chirho(lhs_chirho);
            let rhs_sub_chirho = equality_subst_chirho.apply_ty_chirho(rhs_chirho);
            // An equality that plain unification cannot decide here — one side
            // is a stuck type-family application (`F a ~ Bool`) — is kept as
            // a `~` predicate: a given rewrite rule when the signature is
            // checked, a deferred equality when it is used.
            if self.ty_is_family_app_chirho(&self.normalize_ty_chirho(&lhs_sub_chirho))
                || self.ty_is_family_app_chirho(&self.normalize_ty_chirho(&rhs_sub_chirho))
                || self.ty_is_family_app_chirho(&lhs_sub_chirho)
                || self.ty_is_family_app_chirho(&rhs_sub_chirho)
            {
                scheme_preds_chirho.push(SchemePredChirho::legacy_chirho(
                    crate::ty_chirho::SchemePredPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: "~".to_string(),
                        ty_chirho: lhs_sub_chirho,
                        extra_tys_chirho: vec![rhs_sub_chirho],
                    },
                ));
                continue;
            }
            match self.unify_normalized_chirho(
                &lhs_sub_chirho,
                &rhs_sub_chirho,
                SpanChirho::DUMMY_CHIRHO,
            ) {
                Ok(subst_chirho) => {
                    equality_subst_chirho = subst_chirho.compose_chirho(&equality_subst_chirho);
                    self.apply_subst_all_chirho(&subst_chirho);
                }
                // `(Int ~ Bool) =>`: an insoluble given. GHC accepts the
                // binding (its body is unreachable); keep the equality so
                // the body is checked under the rewrite it states.
                Err(_err_chirho) => scheme_preds_chirho.push(SchemePredChirho::legacy_chirho(
                    crate::ty_chirho::SchemePredPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: "~".to_string(),
                        ty_chirho: lhs_sub_chirho,
                        extra_tys_chirho: vec![rhs_sub_chirho],
                    },
                )),
            }
        }

        // Peel off the outermost ForallChirho produced by ast_type_to_ty_chirho.
        // The vars in a top-level ForallChirho become scheme vars; nested ForallChirho
        // (inside function args) are preserved for rank-N polymorphism.
        let outer_records_chirho = outer_forall_vars_chirho.clone();
        let outer_forall_vars_chirho = if explicit_forall_vars_chirho.is_empty() {
            outer_forall_vars_chirho.to_vec()
        } else {
            let mut all_vars_chirho = explicit_forall_vars_chirho;
            for var_chirho in &outer_forall_vars_chirho {
                if !all_vars_chirho.contains(var_chirho) {
                    all_vars_chirho.push(*var_chirho);
                }
            }
            all_vars_chirho
        };
        let inner_ty_chirho = equality_subst_chirho.apply_ty_chirho(&inner_ty_chirho);
        let scheme_preds_chirho: Vec<SchemePredChirho> = scheme_preds_chirho
            .into_iter()
            .map(|pred_chirho| {
                SchemePredChirho::from_proof_chirho(
                    pred_chirho.class_identity_chirho().clone(),
                    crate::ty_chirho::SchemePredPartsChirho {
                        kind_tys_chirho: pred_chirho
                            .kind_tys_chirho
                            .iter()
                            .map(|kind_chirho| equality_subst_chirho.apply_ty_chirho(kind_chirho))
                            .collect(),
                        class_name_chirho: pred_chirho.class_name_chirho,
                        ty_chirho: equality_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                        extra_tys_chirho: pred_chirho
                            .extra_tys_chirho
                            .iter()
                            .map(|ty_chirho| equality_subst_chirho.apply_ty_chirho(ty_chirho))
                            .collect(),
                    },
                )
            })
            .collect();

        // Only quantify over vars that are free in the resulting type.
        // Vars bound by inner ForallChirho are NOT free and should not be in scheme vars.
        let mut free_in_ty_chirho = inner_ty_chirho.free_vars_chirho();
        for predicate_chirho in &scheme_preds_chirho {
            for term_chirho in predicate_chirho
                .kind_tys_chirho
                .iter()
                .chain(std::iter::once(&predicate_chirho.ty_chirho))
                .chain(&predicate_chirho.extra_tys_chirho)
            {
                free_in_ty_chirho.extend(term_chirho.free_vars_chirho());
            }
        }
        let mut existing_records_chirho = source_binders_chirho;
        existing_records_chirho.extend(outer_records_chirho.records_chirho().iter().cloned());
        let mut mapped_records_chirho = Vec::new();
        for mut record_chirho in existing_records_chirho {
            if let Some(kind_chirho) = &record_chirho.kind_chirho {
                let mapped_chirho = equality_subst_chirho.apply_bounded_chirho(
                    kind_chirho,
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                );
                match mapped_chirho {
                    Some(kind_chirho) => {
                        record_chirho.kind_chirho = Some(std::sync::Arc::new(kind_chirho))
                    }
                    None => self.type_normalization_exhausted_chirho.set(true),
                }
            }
            mapped_records_chirho.push(record_chirho);
        }
        let mapped_records_chirho =
            QuantifiedVariablesChirho::from_records_chirho(mapped_records_chirho);
        for record_chirho in mapped_records_chirho.records_chirho() {
            if let Some(kind_chirho) = &record_chirho.kind_chirho {
                let free_chirho = SubstChirho::free_variables_bounded_chirho(
                    kind_chirho,
                    &mut |work_chirho, output_chirho| {
                        budget_chirho.node_chirho(output_chirho)?;
                        budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
                    },
                );
                match free_chirho {
                    Some(free_chirho) => free_in_ty_chirho.extend(free_chirho),
                    None => self.type_normalization_exhausted_chirho.set(true),
                }
            }
        }
        free_in_ty_chirho.sort_unstable();
        free_in_ty_chirho.dedup();
        // Source order is not allocation order: infix lowering moves the
        // operator before its left operand, and predicates convert after the body.
        // Workflow: language-features-chirho/rank-n-visible-type-application-chirho
        let vars_chirho = signature_binders_chirho::ordered_scheme_vars_chirho(
            ast_ty_chirho,
            &var_map_chirho,
            outer_forall_vars_chirho,
            &free_in_ty_chirho,
        );

        // ScopedTypeVariables: a variable bound by an enclosing signature (or
        // instance head) is never re-quantified here, and one already made
        // rigid is that rigid constant here too.
        // workflow: language-features-chirho/rigid-type-variables-chirho
        let scoped_ids_chirho: Vec<TyVarChirho> = if seeded_are_bound_chirho {
            seed_chirho.values().copied().collect()
        } else {
            Vec::new()
        };
        // Canonical signature IDs survive repeated conversion. Only an
        // enclosing lexical seed is already rigid here; this signature's own
        // prior checking skolems must never replace its published quantifiers.
        let mut skolem_bindings_chirho = SubstChirho::empty_chirho();
        for variable_chirho in &scoped_ids_chirho {
            if let Some(image_chirho) = self.skolem_bindings_chirho.lookup_chirho(variable_chirho) {
                skolem_bindings_chirho.insert_chirho(*variable_chirho, image_chirho.clone());
            }
        }
        let vars_chirho: Vec<TyVarChirho> = vars_chirho
            .into_iter()
            .filter(|var_chirho| !scoped_ids_chirho.contains(var_chirho))
            .collect();
        let vars_chirho = self.quantify_variables_chirho(
            vars_chirho,
            mapped_records_chirho.records_chirho(),
            &scoped_ids_chirho,
            &SubstChirho::empty_chirho(),
            haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho,
            &mut budget_chirho,
        );
        var_map_chirho.extend(body_bindings_chirho);
        let scheme_chirho = SchemeChirho {
            vars_chirho,
            preds_chirho: scheme_preds_chirho,
            ty_chirho: inner_ty_chirho,
        };
        (
            skolem_bindings_chirho.apply_scheme_chirho(&scheme_chirho),
            var_map_chirho,
        )
    }

    /// One conversion for every WRITTEN context, a signature's and a declaration's
    /// alike: class predicates keep every argument, `~` keeps both sides, and a
    /// constraint synonym is expanded. The `bool` is true when the context carries a
    /// premise this layer does not model (a quantified constraint), so a caller that
    /// reasons about the context knows it is incomplete rather than empty.
    /// workflow: language-features-chirho/instance-obligations-chirho
    pub(super) fn convert_written_context_chirho(
        &mut self,
        context_chirho: &[AstConstraintChirho],
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> (Vec<SchemePredChirho>, Vec<(TyChirho, TyChirho)>, bool) {
        let mut preds_chirho = Vec::new();
        let mut equalities_chirho = Vec::new();
        let mut unmodelled_chirho = false;
        for constraint_chirho in context_chirho {
            if matches!(
                constraint_chirho.unannotated_chirho(),
                AstConstraintChirho::QuantifiedChirho { .. }
            ) {
                unmodelled_chirho = true;
                continue;
            }
            self.convert_signature_constraint_chirho(
                constraint_chirho,
                var_map_chirho,
                &mut preds_chirho,
                &mut equalities_chirho,
            );
        }
        (preds_chirho, equalities_chirho, unmodelled_chirho)
    }

    fn convert_signature_constraint_chirho(
        &mut self,
        constraint_chirho: &AstConstraintChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        scheme_preds_chirho: &mut Vec<SchemePredChirho>,
        equality_pairs_chirho: &mut Vec<(TyChirho, TyChirho)>,
    ) {
        let term_chirho = self.ast_constraint_to_term_chirho(constraint_chirho, var_map_chirho);
        let expanded_chirho = self.expand_type_synonyms_chirho(&term_chirho);
        self.scheme_preds_from_constraint_ty_chirho(
            &expanded_chirho,
            scheme_preds_chirho,
            equality_pairs_chirho,
        );
    }

    pub(super) fn scope_signature_for_body_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        var_map_chirho: &HashMap<String, TyVarChirho>,
    ) {
        if self.scoped_type_variables_chirho
            && let TypeChirho::ForallChirho { vars_chirho, .. } = ast_ty_chirho
        {
            for name_chirho in vars_chirho {
                if let Some(var_chirho) = var_map_chirho.get(name_chirho.text_chirho()) {
                    self.scoped_tyvars_chirho
                        .insert(name_chirho.text_chirho().to_string(), *var_chirho);
                }
            }
        }
    }

    /// Prepare leading quantifiers and predicates in lexical order. Parentheses
    /// are transparent for scheme quantification, but not for RHS scope export.
    /// Converting a context before entering the next forall prevents a later
    /// same-spelling binder from capturing that context's variables.
    fn prepare_signature_body_chirho<'a>(
        &mut self,
        ast_ty_chirho: &'a TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        source_binders_chirho: &mut Vec<QuantifiedVariableChirho>,
        scheme_preds_chirho: &mut Vec<SchemePredChirho>,
        equality_pairs_chirho: &mut Vec<(TyChirho, TyChirho)>,
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
    ) -> &'a TypeChirho {
        match ast_ty_chirho {
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => {
                for constraint_chirho in context_chirho {
                    self.convert_signature_constraint_chirho(
                        constraint_chirho,
                        var_map_chirho,
                        scheme_preds_chirho,
                        equality_pairs_chirho,
                    );
                }
                self.prepare_signature_body_chirho(
                    body_chirho,
                    var_map_chirho,
                    source_binders_chirho,
                    scheme_preds_chirho,
                    equality_pairs_chirho,
                    budget_chirho,
                )
            }
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let mut undo_chirho = Vec::new();
                for binder_chirho in vars_chirho {
                    let record_chirho = self.open_ast_quantifier_chirho(
                        binder_chirho,
                        var_map_chirho,
                        &mut undo_chirho,
                        budget_chirho,
                    );
                    source_binders_chirho.push(record_chirho);
                }
                self.prepare_signature_body_chirho(
                    body_chirho,
                    var_map_chirho,
                    source_binders_chirho,
                    scheme_preds_chirho,
                    equality_pairs_chirho,
                    budget_chirho,
                )
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => self.prepare_signature_body_chirho(
                inner_chirho,
                var_map_chirho,
                source_binders_chirho,
                scheme_preds_chirho,
                equality_pairs_chirho,
                budget_chirho,
            ),
            other_chirho => other_chirho,
        }
    }
}
