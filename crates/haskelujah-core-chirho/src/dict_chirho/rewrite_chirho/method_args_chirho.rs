// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A method var or argument rewritten under a known key: the method-var entry, typed
//! arguments and the typed rewrite.

use std::collections::{HashMap, HashSet};

use super::DictPassCtxChirho;
use crate::expr_chirho::{CoreAltChirho, CoreExprChirho, CoreIdChirho};

impl DictPassCtxChirho {
    /// Try to rewrite a method Var reference into a dictionary projection.
    /// If `type_key_override_chirho` is provided, use it to select a
    /// type-specific instance dictionary instead of the default.
    pub(super) fn try_rewrite_method_var_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_override_chirho: Option<&str>,
    ) -> CoreExprChirho {
        // Skip rewriting for locally-bound variables that shadow class methods
        if self.local_shadow_ids_chirho.borrow().contains(&id_chirho) {
            return CoreExprChirho::VarChirho(id_chirho);
        }
        // Dictionary evidence: an occurrence the checker proved dispatches by
        // that proof, before any syntactic guess — the engine's prim row when
        // it has one, else the keyed selector path below with the proven key.
        // workflow: language-features-chirho/dictionary-evidence-chirho
        let evidence_chirho = self.occurrence_evidence_chirho.get(&id_chirho);
        // A proof that is not the binding's own dictionary is never answered by
        // the ambient context dictionary below; unresolved, it takes its
        // canonical method's route with the caller's key.
        // workflow: language-features-chirho/own-class-method-occurrences-chirho
        let foreign_proof_chirho = evidence_chirho.is_some_and(|(_class_chirho, key_chirho)| {
            !Self::proof_admits_context_chirho(key_chirho)
        });
        let caller_key_chirho = type_key_override_chirho;
        if let Some((evidence_class_chirho, evidence_key_chirho)) = evidence_chirho {
            if let Some(self_dict_head_chirho) = self.agreeing_self_dictionary_chirho(
                id_chirho,
                &|| caller_key_chirho.map(str::to_string),
                local_instance_dicts_chirho,
            ) {
                return self_dict_head_chirho;
            }
            if let Some((method_name_chirho, _canonical_chirho)) =
                self.method_occurrence_canon_chirho.get(&id_chirho)
            {
                // Literal wrappers take the selector form (see
                // `occurrence_head_replacement_chirho`).
                if !matches!(method_name_chirho.as_str(), "fromInteger" | "fromString") {
                    for key_spelling_chirho in Self::key_spellings_chirho(evidence_key_chirho) {
                        let prim_name_chirho = format!(
                            "$prim_{evidence_class_chirho}_{method_name_chirho}_{key_spelling_chirho}"
                        );
                        if let Some(prim_chirho) = self.proof_row_chirho(&prim_name_chirho) {
                            return prim_chirho;
                        }
                    }
                }
            }
        }
        // R2 at every entry (R2b): an `Eq` or `Ord` occurrence the checker
        // proved at a whole list key, with no method row, takes the proven-list
        // dispatch here too. The caller's own key, before the proof replaces it
        // below, is offered only for agreement with the proof. Occurrences in
        // typed higher-order arguments arrive here and never through
        // `occurrence_head_replacement_chirho`: `map (== [[1]]) ..` came back a
        // bare `==`, and inside `Eq a =>` the enclosing dictionary answered.
        // workflow: language-features-chirho/dictionary-evidence-chirho
        if let Some(dispatched_chirho) = self.proven_list_occurrence_chirho(
            id_chirho,
            &|| type_key_override_chirho.map(str::to_string),
            local_instance_dicts_chirho,
        ) {
            return dispatched_chirho;
        }
        let type_key_override_chirho = evidence_chirho
            .map(|(_class_chirho, key_chirho)| key_chirho.as_str())
            .or(type_key_override_chirho);
        if let Some(name_chirho) = self.names_chirho.get(&id_chirho) {
            if let Some((class_name_chirho, sel_id_chirho)) =
                self.class_method_selector_for_name_chirho(name_chirho)
            {
                // The checker proved this occurrence is at a rigid variable of
                // the enclosing signature: the binding's own dictionary
                // parameter for the class is the evidence, no instance is.
                // workflow: language-features-chirho/dictionary-evidence-chirho
                if let Some(own_key_chirho) = type_key_override_chirho.filter(|key_chirho| {
                    haskelujah_typing_chirho::infer_chirho::is_own_dictionary_key_chirho(key_chirho)
                }) {
                    if let Some(dict_id_chirho) = Self::own_dict_chirho(
                        &class_name_chirho,
                        own_key_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                    ) {
                        return CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(dict_id_chirho)),
                        };
                    }
                }
                // Try type-specific instance dict first. For higher-kinded
                // classes, value inference sees keys like `Either Int Int`
                // while generated dictionaries are keyed by the type head
                // (`Either`, `Maybe`, `[]`).
                if let Some(type_key_chirho) = type_key_override_chirho {
                    let candidate_keys_chirho =
                        Self::registry_spellings_chirho(&class_name_chirho, type_key_chirho);
                    for candidate_key_chirho in candidate_keys_chirho {
                        if let Some(dict_id_chirho) = local_instance_dicts_chirho
                            .get(&(class_name_chirho.clone(), candidate_key_chirho.clone()))
                        {
                            return CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                            };
                        }
                        if let Some(dict_id_chirho) = self
                            .instance_dicts_chirho
                            .get(&(class_name_chirho.clone(), candidate_key_chirho))
                        {
                            return CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                            };
                        }
                    }
                    if self
                        .class_param_count_chirho
                        .get(&class_name_chirho)
                        .copied()
                        .unwrap_or(1)
                        > 1
                    {
                        let prefix_chirho = format!("{type_key_chirho}_");
                        let mut dict_ids_chirho = Vec::new();
                        for ((dict_class_chirho, dict_key_chirho), dict_id_chirho) in
                            local_instance_dicts_chirho
                        {
                            if dict_class_chirho == &class_name_chirho
                                && dict_key_chirho.starts_with(&prefix_chirho)
                                && !dict_ids_chirho.contains(dict_id_chirho)
                            {
                                dict_ids_chirho.push(*dict_id_chirho);
                            }
                        }
                        for ((dict_class_chirho, dict_key_chirho), dict_id_chirho) in
                            &self.instance_dicts_chirho
                        {
                            if dict_class_chirho == &class_name_chirho
                                && dict_key_chirho.starts_with(&prefix_chirho)
                                && !dict_ids_chirho.contains(dict_id_chirho)
                            {
                                dict_ids_chirho.push(*dict_id_chirho);
                            }
                        }
                        if let [dict_id_chirho] = dict_ids_chirho.as_slice() {
                            return CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                            };
                        }
                    }
                }

                // Fall back only to dictionaries backed by real local evidence.
                // Ambient seeded defaults are not proof for an unresolved method
                // use and used to pick arbitrary semantics.
                if evidence_classes_chirho.contains(&class_name_chirho)
                    && !foreign_proof_chirho
                    && let Some(dict_id_chirho) = dict_vars_chirho.get(&class_name_chirho)
                {
                    return CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(*dict_id_chirho)),
                    };
                }
            }
        }
        if foreign_proof_chirho
            && let Some((_method_name_chirho, canonical_id_chirho)) =
                self.method_occurrence_canon_chirho.get(&id_chirho)
        {
            return self.try_rewrite_method_var_chirho(
                *canonical_id_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                caller_key_chirho,
            );
        }
        CoreExprChirho::VarChirho(id_chirho)
    }

    // Rewrites typed method arguments such as `(empty :: [Int])` after an
    // outer method application has selected a concrete instance key.
    // workflow: monadic-dispatch-chirho
    pub(super) fn try_rewrite_typed_method_arg_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_chirho: &str,
    ) -> Option<CoreExprChirho> {
        match expr_chirho {
            CoreExprChirho::VarChirho(arg_id_chirho) => Some(self.try_rewrite_method_var_chirho(
                *arg_id_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                Some(type_key_chirho),
            )),
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => self.try_rewrite_typed_method_arg_chirho(
                inner_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_instance_dicts_chirho,
                type_key_chirho,
            ),
            _ => None,
        }
    }

    pub(super) fn try_rewrite_typed_dict_param_arg_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_chirho: &str,
    ) -> Option<CoreExprChirho> {
        let (written_id_chirho, classes_chirho, args_chirho) =
            self.collect_dict_param_app_chirho(expr_chirho)?;
        // The surrounding expression's result type is not the type of this
        // callee's predicate (e.g. Eq a for dedup :: [a] -> [a]). A captured
        // reference must take the same evidence-first path in every context.
        if self
            .reference_evidence_chirho
            .contains_key(&written_id_chirho)
        {
            return Some(self.rewrite_method_refs_with_locals_chirho(
                expr_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
            ));
        }
        // Every predicate slot, or none at all (the general path decides).
        let mut result_chirho = self.typed_reference_dicts_chirho(
            written_id_chirho,
            classes_chirho,
            type_key_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
        )?;

        for a_chirho in &args_chirho {
            let rewritten_arg_chirho = self
                .try_rewrite_typed_dict_param_arg_chirho(
                    a_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .or_else(|| {
                    self.try_rewrite_typed_method_arg_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_instance_dicts_chirho,
                        type_key_chirho,
                    )
                })
                .unwrap_or_else(|| {
                    self.rewrite_method_refs_with_locals_chirho(
                        a_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                });
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        Some(result_chirho)
    }

    // See spec-chirho/workflows-chirho/stg-forcing-core-chirho.md: prove nested
    // PAP-looking residues are missing evidence before changing runtime forcing.
    pub(super) fn rewrite_expr_with_type_key_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
        type_key_chirho: &str,
    ) -> Option<CoreExprChirho> {
        if let Some(rewritten_chirho) = self
            .try_rewrite_typed_dict_param_arg_chirho(
                expr_chirho,
                dict_vars_chirho,
                evidence_classes_chirho,
                local_type_keys_chirho,
                local_instance_dicts_chirho,
                type_key_chirho,
            )
            .or_else(|| {
                self.try_rewrite_typed_method_arg_chirho(
                    expr_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
            })
        {
            return Some(rewritten_chirho);
        }

        match expr_chirho {
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                let rewritten_fun_chirho = self.rewrite_expr_with_type_key_chirho(
                    fun_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                let rewritten_arg_chirho = self.rewrite_expr_with_type_key_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                if rewritten_fun_chirho.is_none() && rewritten_arg_chirho.is_none() {
                    return None;
                }
                let rebuilt_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(rewritten_fun_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            fun_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                    arg_chirho: Box::new(rewritten_arg_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                };
                Some(
                    self.try_dispatch_semigroup_monoid_body_chirho(
                        &rebuilt_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    )
                    .unwrap_or(rebuilt_chirho),
                )
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ty_chirho,
            } => self
                .rewrite_expr_with_type_key_chirho(
                    inner_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .map(|rewritten_inner_chirho| CoreExprChirho::TyAppChirho {
                    expr_chirho: Box::new(rewritten_inner_chirho),
                    ty_chirho: ty_chirho.clone(),
                }),
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                let lambda_type_keys_chirho = Self::extend_first_lambda_type_key_chirho(
                    expr_chirho,
                    type_key_chirho,
                    local_type_keys_chirho,
                );
                self.rewrite_expr_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    &lambda_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .map(|rewritten_body_chirho| CoreExprChirho::LamChirho {
                    binder_chirho: binder_chirho.clone(),
                    body_chirho: Box::new(rewritten_body_chirho),
                })
            }
            CoreExprChirho::TyLamChirho {
                ty_var_chirho,
                body_chirho,
            } => self
                .rewrite_expr_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                )
                .map(|rewritten_body_chirho| CoreExprChirho::TyLamChirho {
                    ty_var_chirho: ty_var_chirho.clone(),
                    body_chirho: Box::new(rewritten_body_chirho),
                }),
            CoreExprChirho::LetChirho {
                rec_chirho,
                binds_chirho,
                body_chirho,
            } => {
                let mut changed_chirho = false;
                let rewritten_binds_chirho = binds_chirho
                    .iter()
                    .map(|(binder_chirho, rhs_chirho)| {
                        let rewritten_rhs_chirho = self.rewrite_expr_with_type_key_chirho(
                            rhs_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_rhs_chirho.is_some() {
                            changed_chirho = true;
                        }
                        (
                            binder_chirho.clone(),
                            rewritten_rhs_chirho.unwrap_or_else(|| {
                                self.rewrite_method_refs_with_locals_chirho(
                                    rhs_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                )
                            }),
                        )
                    })
                    .collect();
                let rewritten_body_chirho = self.rewrite_expr_with_type_key_chirho(
                    body_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                if rewritten_body_chirho.is_some() {
                    changed_chirho = true;
                }
                if !changed_chirho {
                    return None;
                }
                Some(CoreExprChirho::LetChirho {
                    rec_chirho: *rec_chirho,
                    binds_chirho: rewritten_binds_chirho,
                    body_chirho: Box::new(rewritten_body_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            body_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                })
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                result_ty_chirho,
                alts_chirho,
            } => {
                let rewritten_scrutinee_chirho = self.rewrite_expr_with_type_key_chirho(
                    scrutinee_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                    type_key_chirho,
                );
                let mut changed_chirho = rewritten_scrutinee_chirho.is_some();
                let rewritten_alts_chirho = alts_chirho
                    .iter()
                    .map(|alt_chirho| {
                        let rewritten_rhs_chirho = self.rewrite_expr_with_type_key_chirho(
                            &alt_chirho.rhs_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_rhs_chirho.is_some() {
                            changed_chirho = true;
                        }
                        CoreAltChirho {
                            con_chirho: alt_chirho.con_chirho.clone(),
                            binders_chirho: alt_chirho.binders_chirho.clone(),
                            rhs_chirho: rewritten_rhs_chirho.unwrap_or_else(|| {
                                self.rewrite_method_refs_with_locals_chirho(
                                    &alt_chirho.rhs_chirho,
                                    dict_vars_chirho,
                                    evidence_classes_chirho,
                                    local_type_keys_chirho,
                                    local_instance_dicts_chirho,
                                )
                            }),
                        }
                    })
                    .collect();
                if !changed_chirho {
                    return None;
                }
                Some(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(rewritten_scrutinee_chirho.unwrap_or_else(|| {
                        self.rewrite_method_refs_with_locals_chirho(
                            scrutinee_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                        )
                    })),
                    bind_chirho: bind_chirho.clone(),
                    result_ty_chirho: result_ty_chirho.clone(),
                    alts_chirho: rewritten_alts_chirho,
                })
            }
            CoreExprChirho::PrimOpChirho {
                name_chirho,
                args_chirho,
            } => {
                let mut changed_chirho = false;
                let rewritten_args_chirho = args_chirho
                    .iter()
                    .map(|arg_chirho| {
                        let rewritten_arg_chirho = self.rewrite_expr_with_type_key_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_arg_chirho.is_some() {
                            changed_chirho = true;
                        }
                        rewritten_arg_chirho.unwrap_or_else(|| {
                            self.rewrite_method_refs_with_locals_chirho(
                                arg_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )
                        })
                    })
                    .collect();
                // At the exact key Double, the desugarer's blind Int primop takes row
                // 547's float counterpart: this traversal has just resolved its operands
                // at that key, as `(fromInteger 3)` in `[-3 / 2, 1] :: [Double]` (row 553).
                // workflow: language-features-chirho/dictionary-evidence-chirho
                changed_chirho.then(|| CoreExprChirho::PrimOpChirho {
                    name_chirho: (type_key_chirho == "Double")
                        .then(|| {
                            super::double_primops_chirho::double_counterpart_chirho(name_chirho)
                        })
                        .flatten()
                        .map_or_else(|| name_chirho.clone(), str::to_string),
                    args_chirho: rewritten_args_chirho,
                })
            }
            CoreExprChirho::ConAppChirho {
                con_name_chirho,
                args_chirho,
            } => {
                let mut changed_chirho = false;
                let rewritten_args_chirho = args_chirho
                    .iter()
                    .map(|arg_chirho| {
                        let rewritten_arg_chirho = self.rewrite_expr_with_type_key_chirho(
                            arg_chirho,
                            dict_vars_chirho,
                            evidence_classes_chirho,
                            local_type_keys_chirho,
                            local_instance_dicts_chirho,
                            type_key_chirho,
                        );
                        if rewritten_arg_chirho.is_some() {
                            changed_chirho = true;
                        }
                        rewritten_arg_chirho.unwrap_or_else(|| {
                            self.rewrite_method_refs_with_locals_chirho(
                                arg_chirho,
                                dict_vars_chirho,
                                evidence_classes_chirho,
                                local_type_keys_chirho,
                                local_instance_dicts_chirho,
                            )
                        })
                    })
                    .collect();
                changed_chirho.then(|| CoreExprChirho::ConAppChirho {
                    con_name_chirho: con_name_chirho.clone(),
                    args_chirho: rewritten_args_chirho,
                })
            }
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => None,
        }
    }

    pub(super) fn try_rewrite_typed_higher_order_args_chirho(
        &self,
        expr_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let mut args_chirho = Vec::new();
        let mut head_chirho = expr_chirho;
        while let CoreExprChirho::AppChirho {
            fun_chirho,
            arg_chirho,
        } = head_chirho
        {
            args_chirho.push(arg_chirho.as_ref());
            head_chirho = fun_chirho.as_ref();
        }
        if args_chirho.len() < 2 {
            return None;
        }
        args_chirho.reverse();
        let CoreExprChirho::VarChirho(head_id_chirho) = head_chirho else {
            return None;
        };
        let head_name_chirho = self.names_chirho.get(head_id_chirho);
        if head_name_chirho.is_some_and(|name_chirho| {
            name_chirho.starts_with("$prim_") || name_chirho.starts_with("$sel_")
        }) || head_name_chirho
            .and_then(|name_chirho| self.class_method_selector_for_name_chirho(name_chirho))
            .is_some()
            || self
                .dict_param_bindings_chirho
                .contains_key(&self.canonical_id_chirho(*head_id_chirho))
        {
            return None;
        }

        let type_key_chirho = args_chirho
            .iter()
            .rev()
            .find_map(|arg_chirho| {
                self.infer_strict_dispatch_key_for_rewrite_chirho(
                    arg_chirho,
                    local_type_keys_chirho,
                )
            })
            .or_else(|| {
                args_chirho
                    .iter()
                    .any(|arg_chirho| self.expr_contains_numeric_default_marker_chirho(arg_chirho))
                    .then(|| "Int".to_string())
            })?;
        let payload_type_key_chirho =
            Self::payload_type_key_from_value_key_chirho(&type_key_chirho);

        let mut changed_chirho = false;
        let mut rewritten_args_chirho = Vec::with_capacity(args_chirho.len());
        for arg_chirho in &args_chirho {
            // A reference the checker proved keeps its own evidence: the
            // variable arm dispatches it; a key guessed from its siblings
            // must not.
            // workflow: language-features-chirho/dictionary-evidence-chirho
            if let CoreExprChirho::VarChirho(arg_id_chirho) = arg_chirho {
                if self.reference_evidence_chirho.contains_key(arg_id_chirho) {
                    rewritten_args_chirho.push(self.rewrite_method_refs_with_locals_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                    ));
                    changed_chirho = true;
                    continue;
                }
            }
            let rewritten_typed_arg_chirho = payload_type_key_chirho
                .as_deref()
                .and_then(|payload_key_chirho| {
                    self.rewrite_expr_with_type_key_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                        payload_key_chirho,
                    )
                })
                .or_else(|| {
                    self.rewrite_expr_with_type_key_chirho(
                        arg_chirho,
                        dict_vars_chirho,
                        evidence_classes_chirho,
                        local_type_keys_chirho,
                        local_instance_dicts_chirho,
                        &type_key_chirho,
                    )
                });
            if let Some(rewritten_chirho) = rewritten_typed_arg_chirho {
                changed_chirho = true;
                rewritten_args_chirho.push(rewritten_chirho);
            } else if Self::first_value_lambda_binder_id_chirho(arg_chirho).is_some() {
                let payload_type_key_chirho = payload_type_key_chirho
                    .clone()
                    .unwrap_or_else(|| type_key_chirho.clone());
                let lambda_type_keys_chirho = Self::extend_first_lambda_type_key_chirho(
                    arg_chirho,
                    &payload_type_key_chirho,
                    local_type_keys_chirho,
                );
                changed_chirho = true;
                rewritten_args_chirho.push(self.rewrite_method_refs_with_locals_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    &lambda_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            } else {
                rewritten_args_chirho.push(self.rewrite_method_refs_with_locals_chirho(
                    arg_chirho,
                    dict_vars_chirho,
                    evidence_classes_chirho,
                    local_type_keys_chirho,
                    local_instance_dicts_chirho,
                ));
            }
        }
        if !changed_chirho {
            return None;
        }

        let mut result_chirho = self.rewrite_method_refs_with_locals_chirho(
            head_chirho,
            dict_vars_chirho,
            evidence_classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
        );
        for rewritten_arg_chirho in rewritten_args_chirho {
            result_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(result_chirho),
                arg_chirho: Box::new(rewritten_arg_chirho),
            };
        }
        Some(result_chirho)
    }
}
