// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Surface type conversion with lexical scope for both forall visibilities.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::TyVarChirho as AstTyVarChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};

use super::InferCtxChirho;
use crate::ty_chirho::{
    MultChirho, QuantifiedVariableChirho, QuantifiedVariablesChirho, TyChirho, TyVarChirho,
};
use haskelujah_ast_chirho::promotion_chirho::{
    quoted_promoted_spelling_chirho, unquoted_promoted_spelling_chirho,
};

impl InferCtxChirho {
    /// Producer-qualified identities take precedence over spelling. A foreign meta
    /// is an invariant failure and cannot be deferred or published.
    pub(super) fn surface_variable_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyVarChirho {
        // Opening a checked required binder can freshen its consumer ID. The
        // existing lexical image carries the exact producer correspondence;
        // an equal spelling without that proof cannot rekey a stamped name.
        if let (Some(source_chirho), Some(variable_chirho)) = (
            name_chirho.type_variable_chirho(),
            variables_chirho.get(name_chirho.text_chirho()),
        ) && self
            .skolem_bindings_chirho
            .lookup_image_chirho(variable_chirho)
            .and_then(|image_chirho| image_chirho.source_chirho.as_ref())
            == Some(source_chirho)
        {
            return *variable_chirho;
        }
        use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
        match name_chirho.type_variable_chirho() {
            Some(ScopedTypeVariableIdChirho::InferenceChirho {
                owner_chirho,
                variable_chirho,
            }) if Some(*owner_chirho) == self.elaboration_state_chirho.owner_chirho
                && variable_chirho.0 < self.elaboration_state_chirho.next_variable_chirho
                && self
                    .elaboration_state_chirho
                    .variable_kinds_chirho
                    .contains_key(variable_chirho) =>
            {
                variables_chirho.insert(name_chirho.text_chirho().to_owned(), *variable_chirho);
                *variable_chirho
            }
            Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
                declaration_chirho,
                index_chirho,
            }) => {
                let schema_chirho = self
                    .type_families_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(declaration_chirho);
                let top_chirho = schema_chirho.is_some_and(|schema_chirho| {
                    schema_chirho
                        .binders_chirho
                        .iter()
                        .any(|binder_chirho| binder_chirho.variable_chirho.0 == *index_chirho)
                });
                let local_chirho = schema_chirho.is_some_and(|schema_chirho| {
                    schema_chirho
                        .local_binders_chirho
                        .iter()
                        .any(|record_chirho| {
                            record_chirho.binder_chirho.variable_chirho.0 == *index_chirho
                        })
                });
                let key_chirho = format!(
                    "%quantifier:{}:{declaration_chirho}:{index_chirho}",
                    declaration_chirho.len()
                );
                if top_chirho || local_chirho {
                    if let Some(variable_chirho) = variables_chirho.get(&key_chirho) {
                        return *variable_chirho;
                    }
                    // Only the published head telescope is implicitly opened.
                    // A local forall occurrence needs its lexical binder action.
                    if top_chirho {
                        let variable_chirho =
                            TyVarChirho(self.elaboration_state_chirho.next_variable_chirho);
                        self.elaboration_state_chirho.next_variable_chirho += 1;
                        variables_chirho.insert(key_chirho, variable_chirho);
                        return variable_chirho;
                    }
                }
                self.invalid_surface_variable_chirho(name_chirho)
            }
            Some(_) => self.invalid_surface_variable_chirho(name_chirho),
            None => {
                if let Some(variable_chirho) = variables_chirho.get(name_chirho.text_chirho()) {
                    return *variable_chirho;
                }
                let variable_chirho =
                    TyVarChirho(self.elaboration_state_chirho.next_variable_chirho);
                self.elaboration_state_chirho.next_variable_chirho += 1;
                variables_chirho.insert(name_chirho.text_chirho().to_owned(), variable_chirho);
                variable_chirho
            }
        }
    }

    pub(super) fn surface_binder_variable_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyVarChirho {
        if name_chirho.type_variable_chirho().is_none() {
            let variable_chirho = TyVarChirho(self.elaboration_state_chirho.next_variable_chirho);
            self.elaboration_state_chirho.next_variable_chirho += 1;
            return variable_chirho;
        }
        use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
        if let Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
            declaration_chirho,
            index_chirho,
        }) = name_chirho.type_variable_chirho()
        {
            let proven_chirho = self
                .type_families_chirho
                .declaration_schemes_chirho()
                .get_chirho(declaration_chirho)
                .is_some_and(|schema_chirho| {
                    schema_chirho
                        .binders_chirho
                        .iter()
                        .any(|binder_chirho| binder_chirho.variable_chirho.0 == *index_chirho)
                        || schema_chirho
                            .local_binders_chirho
                            .iter()
                            .any(|record_chirho| {
                                record_chirho.binder_chirho.variable_chirho.0 == *index_chirho
                            })
                });
            if !proven_chirho {
                return self.invalid_surface_variable_chirho(name_chirho);
            }
            let variable_chirho = TyVarChirho(self.elaboration_state_chirho.next_variable_chirho);
            self.elaboration_state_chirho.next_variable_chirho += 1;
            return variable_chirho;
        }
        let previous_chirho = variables_chirho.get(name_chirho.text_chirho()).copied();
        let variable_chirho = self.surface_variable_chirho(name_chirho, variables_chirho);
        if let Some(previous_chirho) = previous_chirho {
            variables_chirho.insert(name_chirho.text_chirho().to_owned(), previous_chirho);
        } else {
            variables_chirho.remove(name_chirho.text_chirho());
        }
        variable_chirho
    }

    fn invalid_surface_variable_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
    ) -> TyVarChirho {
        self.non_deferrable_diagnostics_chirho.push_chirho(
            DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(300),
                "foreign or unbound canonical type variable at the kind/type inference boundary",
                name_chirho.span_chirho(),
            ),
        );
        let variable_chirho = TyVarChirho(self.elaboration_state_chirho.next_variable_chirho);
        self.elaboration_state_chirho.next_variable_chirho += 1;
        variable_chirho
    }

    /// Open every declaration-head binder lexically, but apply only visible
    /// parameters to the constructor result. Used by data and newtype schemes
    /// (and consequently their record selectors); see declaration-kinds-chirho.
    pub(super) fn data_head_type_chirho(
        &mut self,
        name_chirho: &str,
        binders_chirho: &[AstTyVarChirho],
    ) -> (TyChirho, HashMap<String, TyVarChirho>) {
        let mut variables_chirho = HashMap::with_capacity(binders_chirho.len());
        for binder_chirho in binders_chirho {
            let variable_chirho = self
                .surface_binder_variable_chirho(&binder_chirho.name_chirho, &mut variables_chirho);
            variables_chirho.insert(binder_chirho.text_chirho().to_owned(), variable_chirho);
        }
        let key_chirho = self
            .type_constructor_scope_chirho
            .as_ref()
            .and_then(|scope_chirho| scope_chirho.declaration_chirho(name_chirho))
            .map(|declaration_chirho| declaration_chirho.identity_chirho.key_chirho());
        let scheme_chirho = key_chirho
            .as_ref()
            .and_then(|key_chirho| {
                self.type_families_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(key_chirho)
            })
            .cloned();
        let mut result_chirho = TyChirho::ConChirho(
            self.type_constructor_scope_chirho
                .as_ref()
                .and_then(|scope_chirho| scope_chirho.declaration_chirho(name_chirho))
                .map_or_else(
                    || name_chirho.to_owned(),
                    |declaration_chirho| declaration_chirho.type_key_chirho(),
                ),
        );
        if let Some(scheme_chirho) = scheme_chirho {
            let mut mapping_chirho = crate::subst_chirho::SubstChirho::empty_chirho();
            for source_chirho in &scheme_chirho.source_variables_chirho {
                if let Some(binder_chirho) =
                    binders_chirho.get(source_chirho.header_index_chirho as usize)
                {
                    let variable_chirho = variables_chirho[binder_chirho.text_chirho()];
                    mapping_chirho.insert_chirho(
                        source_chirho.variable_chirho,
                        TyChirho::VarChirho(variable_chirho),
                    );
                    if let Some(actual_kind_chirho) = self
                        .elaboration_state_chirho
                        .variable_kinds_chirho
                        .get(&variable_chirho)
                        .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
                        && let Some(typed_binder_chirho) =
                            scheme_chirho.binders_chirho.iter().find(|binder_chirho| {
                                binder_chirho.variable_chirho == source_chirho.variable_chirho
                            })
                    {
                        crate::ty_chirho::declarations_chirho::map_telescope_kind_variables_chirho(
                            &typed_binder_chirho.kind_chirho,
                            actual_kind_chirho,
                            &mut mapping_chirho,
                        );
                    }
                }
            }
            for binder_chirho in &scheme_chirho.binders_chirho {
                let argument_chirho = if let Some(argument_chirho) =
                    mapping_chirho.lookup_chirho(&binder_chirho.variable_chirho)
                {
                    argument_chirho.clone()
                } else {
                    let variable_chirho =
                        TyVarChirho(self.elaboration_state_chirho.next_variable_chirho);
                    self.elaboration_state_chirho.next_variable_chirho += 1;
                    let argument_chirho = TyChirho::VarChirho(variable_chirho);
                    mapping_chirho
                        .insert_chirho(binder_chirho.variable_chirho, argument_chirho.clone());
                    argument_chirho
                };
                if let TyChirho::VarChirho(variable_chirho) = &argument_chirho {
                    variables_chirho.insert(
                        format!(
                            "%quantifier:{}:{}:{}",
                            scheme_chirho.identity_chirho.len(),
                            scheme_chirho.identity_chirho,
                            binder_chirho.variable_chirho.0
                        ),
                        *variable_chirho,
                    );
                }
                result_chirho =
                    TyChirho::AppChirho(Box::new(result_chirho), Box::new(argument_chirho));
            }
        } else {
            for binder_chirho in binders_chirho
                .iter()
                .filter(|binder_chirho| binder_chirho.is_visible_chirho())
            {
                result_chirho = TyChirho::AppChirho(
                    Box::new(result_chirho),
                    Box::new(TyChirho::VarChirho(
                        variables_chirho[binder_chirho.text_chirho()],
                    )),
                );
            }
        }
        (result_chirho, variables_chirho)
    }

    /// Convert an AST `TypeChirho` (surface syntax) to an internal `TyChirho`.
    ///
    /// Named type variables are mapped to fresh unification variables via
    /// `var_map_chirho`. This ensures that `a -> a` uses the same variable
    /// for both occurrences of `a`.
    pub fn ast_type_to_ty_chirho(
        &mut self,
        ast_ty_chirho: &TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> TyChirho {
        match ast_ty_chirho {
            TypeChirho::KindAnnotationChirho { ty_chirho, .. } => {
                self.ast_type_to_ty_chirho(ty_chirho, var_map_chirho)
            }
            TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let fun_chirho = self.ast_type_to_ty_chirho(fun_chirho, var_map_chirho);
                let arg_chirho = self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho);
                let applied_chirho =
                    TyChirho::AppChirho(Box::new(fun_chirho), Box::new(arg_chirho));
                if self.definition_type_conversion_chirho {
                    return applied_chirho;
                }
                let expanded_chirho = self.expand_type_synonyms_chirho(&applied_chirho);
                self.reduce_type_families_in_ty_chirho(&expanded_chirho)
            }
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho().to_string();
                let tv_chirho = self.surface_variable_chirho(name_chirho, var_map_chirho);
                self.tyvar_source_names_chirho
                    .entry(tv_chirho)
                    .or_insert(text_chirho);
                self.elaboration_state_chirho
                    .substitution_chirho
                    .apply_ty_chirho(&TyChirho::VarChirho(tv_chirho))
            }
            TypeChirho::ConChirho(name_chirho) => {
                if let Some(key_chirho) = self.family_reference_key_chirho(name_chirho) {
                    let typed_chirho = TyChirho::ConChirho(key_chirho);
                    return if self.definition_type_conversion_chirho {
                        typed_chirho
                    } else {
                        self.reduce_type_families_in_ty_chirho(&typed_chirho)
                    };
                }
                if let Some(declaration_chirho) = name_chirho.type_constructor_chirho() {
                    let typed_chirho = TyChirho::ConChirho(declaration_chirho.type_key_chirho());
                    return if self.definition_type_conversion_chirho {
                        typed_chirho
                    } else {
                        self.expand_type_synonyms_chirho(&typed_chirho)
                    };
                }
                // An unquoted promoted constructor keeps its identity key (row 686).
                if let Some(spelling_chirho) = unquoted_promoted_spelling_chirho(
                    name_chirho,
                    self.promotion_scope_chirho.as_deref(),
                ) {
                    return TyChirho::ConChirho(spelling_chirho);
                }
                let text_chirho =
                    self.normalize_imported_type_name_chirho(&name_chirho.full_name_chirho());
                let raw_chirho = TyChirho::ConChirho(text_chirho);
                // Eagerly expand nullary type synonyms (e.g. String → [Char])
                if self.definition_type_conversion_chirho {
                    raw_chirho
                } else {
                    self.expand_type_synonyms_chirho(&raw_chirho)
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                let f_chirho = self.ast_type_to_ty_chirho(fun_chirho, var_map_chirho);
                let a_chirho = self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho);
                let raw_chirho = TyChirho::AppChirho(Box::new(f_chirho), Box::new(a_chirho));
                if self.definition_type_conversion_chirho {
                    return raw_chirho;
                }
                // Expand parameterised type synonyms (e.g. Pair Int → (Int, Int))
                let expanded_chirho = self.expand_type_synonyms_chirho(&raw_chirho);
                // Reduce type family applications (e.g. F Int → Bool)
                self.reduce_type_families_in_ty_chirho(&expanded_chirho)
            }
            TypeChirho::FunChirho {
                arg_chirho,
                mult_chirho,
                result_chirho,
                ..
            } => {
                let a_chirho = self.ast_type_to_ty_chirho(arg_chirho, var_map_chirho);
                let r_chirho = self.ast_type_to_ty_chirho(result_chirho, var_map_chirho);
                let m_chirho = match mult_chirho {
                    Some(haskelujah_ast_chirho::ty_chirho::MultiplicityChirho::OneChirho) => {
                        MultChirho::OneChirho
                    }
                    _ => MultChirho::ManyChirho,
                };
                TyChirho::FunChirho(Box::new(a_chirho), Box::new(r_chirho), m_chirho)
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let elems_chirho: Vec<TyChirho> = elements_chirho
                    .iter()
                    .map(|e_chirho| self.ast_type_to_ty_chirho(e_chirho, var_map_chirho))
                    .collect();
                TyChirho::TupleChirho(elems_chirho)
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                let elem_chirho = self.ast_type_to_ty_chirho(element_chirho, var_map_chirho);
                TyChirho::ListChirho(Box::new(elem_chirho))
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                self.ast_type_to_ty_chirho(inner_chirho, var_map_chirho)
            }
            TypeChirho::QualChirho {
                context_chirho,
                body_chirho,
                ..
            } => TyChirho::QualChirho {
                context_chirho: context_chirho
                    .iter()
                    .map(|constraint_chirho| {
                        self.ast_constraint_to_term_chirho(constraint_chirho, var_map_chirho)
                    })
                    .collect(),
                body_chirho: Box::new(self.ast_type_to_ty_chirho(body_chirho, var_map_chirho)),
            },
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let (bound_vars_chirho, body_ty_chirho) =
                    self.ast_forall_body_chirho(vars_chirho, body_chirho, var_map_chirho);
                TyChirho::ForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(body_ty_chirho),
                }
            }
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => {
                let (bound_vars_chirho, body_ty_chirho) =
                    self.ast_forall_body_chirho(vars_chirho, body_chirho, var_map_chirho);
                TyChirho::RequiredForallChirho {
                    vars_chirho: bound_vars_chirho,
                    body_chirho: Box::new(body_ty_chirho),
                }
            }
            // DataKinds: promoted constructor is a type-level constant
            // A promoted constructor converts to its identity key under naming's scope
            // (row 686); a caller that never ran naming keeps the legacy spelling.
            TypeChirho::PromotedConChirho { name_chirho, .. } => TyChirho::ConChirho(
                quoted_promoted_spelling_chirho(
                    name_chirho,
                    self.promotion_scope_chirho.as_deref(),
                )
                .unwrap_or_else(|| name_chirho.full_name_chirho()),
            ),
            TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                let keyed_chirho = self.promotion_scope_chirho.is_some();
                TyChirho::promoted_list_chirho(
                    elements_chirho.iter().map(|element_chirho| {
                        self.ast_type_to_ty_chirho(element_chirho, var_map_chirho)
                    }),
                    keyed_chirho,
                )
            }
            // PartialTypeSignatures: `_` in a type signature is a wildcard that
            // becomes a fresh unification variable. Emit warning W4201 so the
            // programmer is informed of the inferred type position.
            TypeChirho::WildcardChirho { span_chirho } => {
                let fresh_ty_chirho = self.fresh_var_chirho();
                let diag_chirho = DiagnosticChirho::warning_with_code_chirho(
                    ErrorCodeChirho::warning_chirho(4201),
                    "found wildcard `_` in type signature (PartialTypeSignatures)".to_string(),
                    *span_chirho,
                );
                self.diagnostics_chirho.push_chirho(diag_chirho);
                fresh_ty_chirho
            }
            // Type-level literal (DataKinds): treat as a type-level constant.
            TypeChirho::LitChirho { value_chirho, .. } => TyChirho::ConChirho(value_chirho.clone()),
        }
    }

    /// Introduce a lexical scope without losing free variables discovered in its body.
    /// Workflow: language-features-chirho/rank-n-visible-type-application-chirho.md.
    ///
    /// Saving only the shadowed entries bounds extra work by the binder count,
    /// not by the accumulated environment. Restore in reverse order so repeated
    /// names also unwind as a stack. Fresh identities and their source names
    /// remain valid in the returned type after the lexical name scope closes.
    fn ast_forall_body_chirho(
        &mut self,
        vars_chirho: &[AstTyVarChirho],
        body_chirho: &TypeChirho,
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> (QuantifiedVariablesChirho, TyChirho) {
        self.with_ast_forall_bindings_chirho(
            vars_chirho,
            var_map_chirho,
            |context_chirho, map_chirho| {
                context_chirho.ast_type_to_ty_chirho(body_chirho, map_chirho)
            },
        )
    }

    pub(super) fn with_ast_forall_bindings_chirho<ResultChirho>(
        &mut self,
        vars_chirho: &[AstTyVarChirho],
        var_map_chirho: &mut HashMap<String, TyVarChirho>,
        action_chirho: impl FnOnce(&mut Self, &mut HashMap<String, TyVarChirho>) -> ResultChirho,
    ) -> (QuantifiedVariablesChirho, ResultChirho) {
        let mut budget_chirho =
            super::equality_normalization_chirho::EqualityBudgetChirho::new_chirho();
        let mut records_chirho = Vec::new();
        let mut undo_chirho = Vec::new();
        for binder_chirho in vars_chirho {
            records_chirho.push(self.open_ast_quantifier_chirho(
                binder_chirho,
                var_map_chirho,
                &mut undo_chirho,
                &mut budget_chirho,
            ));
        }
        let result_chirho = action_chirho(self, var_map_chirho);
        for (name_chirho, previous_chirho) in undo_chirho.into_iter().rev() {
            if let Some(variable_chirho) = previous_chirho {
                var_map_chirho.insert(name_chirho, variable_chirho);
            } else {
                var_map_chirho.remove(&name_chirho);
            }
        }
        (
            QuantifiedVariablesChirho::from_records_chirho(records_chirho),
            result_chirho,
        )
    }

    /// The source binder is the producer. Recover its full kind before admitting
    /// its lexical names; canonical free captures must already have a mapping.
    pub(super) fn open_ast_quantifier_chirho(
        &mut self,
        binder_chirho: &AstTyVarChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
        undo_chirho: &mut Vec<(String, Option<TyVarChirho>)>,
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
    ) -> QuantifiedVariableChirho {
        use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
        let variable_chirho =
            self.surface_binder_variable_chirho(&binder_chirho.name_chirho, variables_chirho);
        if let Err(error_chirho) = self
            .elaboration_state_chirho
            .mark_universal_chirho(variable_chirho, binder_chirho.name_chirho.span_chirho())
        {
            self.non_deferrable_diagnostics_chirho
                .push_chirho(error_chirho.into_diagnostic_chirho());
        }
        let kind_chirho = self.surface_quantifier_kind_chirho(
            &binder_chirho.name_chirho,
            variables_chirho,
            budget_chirho,
        );
        if budget_chirho
            .node_chirho(binder_chirho.text_chirho().len().saturating_add(2))
            .is_none()
        {
            self.note_type_normalization_exhausted_chirho();
        }
        let name_chirho = binder_chirho.text_chirho().to_owned();
        self.tyvar_source_names_chirho
            .insert(variable_chirho, name_chirho.clone());
        let previous_chirho = variables_chirho.insert(name_chirho.clone(), variable_chirho);
        undo_chirho.push((name_chirho, previous_chirho));
        if let Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
            declaration_chirho,
            index_chirho,
        }) = binder_chirho.name_chirho.type_variable_chirho()
        {
            if budget_chirho
                .node_chirho(declaration_chirho.len().saturating_add(64))
                .is_none()
            {
                self.note_type_normalization_exhausted_chirho();
            }
            let key_chirho = format!(
                "%quantifier:{}:{declaration_chirho}:{index_chirho}",
                declaration_chirho.len()
            );
            let previous_chirho = variables_chirho.insert(key_chirho.clone(), variable_chirho);
            undo_chirho.push((key_chirho, previous_chirho));
        }
        QuantifiedVariableChirho {
            variable_chirho,
            kind_chirho,
            source_chirho: binder_chirho.name_chirho.type_variable_chirho().cloned(),
            specificity_chirho: Some(binder_chirho.specificity_chirho),
        }
    }

    pub(super) fn surface_quantifier_kind_chirho(
        &mut self,
        name_chirho: &haskelujah_ast_chirho::name_chirho::NameChirho,
        variables_chirho: &HashMap<String, TyVarChirho>,
        budget_chirho: &mut super::equality_normalization_chirho::EqualityBudgetChirho,
    ) -> Option<std::sync::Arc<TyChirho>> {
        use crate::subst_chirho::SubstChirho;
        use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
        let mut charge_chirho = |work_chirho: usize, output_chirho: usize| {
            budget_chirho.node_chirho(output_chirho)?;
            budget_chirho.work_chirho(work_chirho.saturating_sub(output_chirho))
        };
        let result_chirho = match name_chirho.type_variable_chirho()? {
            ScopedTypeVariableIdChirho::InferenceChirho {
                owner_chirho,
                variable_chirho,
            } if Some(*owner_chirho) == self.elaboration_state_chirho.owner_chirho
                && variable_chirho.0 < self.elaboration_state_chirho.next_variable_chirho =>
            {
                let kind_chirho = self
                    .elaboration_state_chirho
                    .variable_kinds_chirho
                    .get(variable_chirho)?
                    .kind_chirho
                    .as_ref()?;
                self.elaboration_state_chirho
                    .substitution_chirho
                    .resolve_owned_bounded_chirho(kind_chirho, &mut charge_chirho)
            }
            ScopedTypeVariableIdChirho::QuantifiedChirho {
                declaration_chirho,
                index_chirho,
            } => {
                let schema_chirho = self
                    .type_families_chirho
                    .declaration_schemes_chirho()
                    .get_chirho(declaration_chirho)
                    .cloned()?;
                if schema_chirho.held_chirho || schema_chirho.owner_chirho.is_some() {
                    return None;
                }
                let kind_chirho = schema_chirho
                    .binders_chirho
                    .iter()
                    .find(|binder_chirho| binder_chirho.variable_chirho.0 == *index_chirho)
                    .map(|binder_chirho| &binder_chirho.kind_chirho)
                    .or_else(|| {
                        schema_chirho
                            .local_binders_chirho
                            .iter()
                            .find(|record_chirho| {
                                record_chirho.binder_chirho.variable_chirho.0 == *index_chirho
                            })
                            .map(|record_chirho| &record_chirho.binder_chirho.kind_chirho)
                    })?;
                let Some(free_chirho) =
                    SubstChirho::free_variables_bounded_chirho(kind_chirho, &mut charge_chirho)
                else {
                    self.note_type_normalization_exhausted_chirho();
                    return None;
                };
                let mut mapping_chirho = SubstChirho::empty_chirho();
                for source_chirho in free_chirho {
                    if charge_chirho(1, declaration_chirho.len().saturating_add(64)).is_none() {
                        self.note_type_normalization_exhausted_chirho();
                        return None;
                    }
                    let key_chirho = format!(
                        "%quantifier:{}:{declaration_chirho}:{}",
                        declaration_chirho.len(),
                        source_chirho.0
                    );
                    let Some(actual_chirho) = variables_chirho.get(&key_chirho) else {
                        self.non_deferrable_diagnostics_chirho.push_chirho(
                            DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(300),
                                "checked quantifier kind contains an unbound canonical capture",
                                name_chirho.span_chirho(),
                            ),
                        );
                        return None;
                    };
                    mapping_chirho
                        .insert_chirho(source_chirho, TyChirho::VarChirho(*actual_chirho));
                }
                mapping_chirho.apply_freshened_bounded_chirho(
                    kind_chirho,
                    &mut self.elaboration_state_chirho.next_variable_chirho,
                    &mut charge_chirho,
                )
            }
            _ => return None,
        };
        if result_chirho.is_none() {
            self.note_type_normalization_exhausted_chirho();
        }
        result_chirho.map(std::sync::Arc::new)
    }
}
