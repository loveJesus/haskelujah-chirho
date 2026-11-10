// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Scope-owned dictionaries and the proof-selected superclass projection shared
//! with binding signatures. Scoped keys never select an enclosing class default.
//! workflow: language-features-chirho/rank-n-visible-type-application-chirho

use super::*;
use haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho;

#[derive(Debug, Default)]
pub(in crate::dict_chirho) struct ScopedDictionaryEnvironmentChirho {
    dictionaries_chirho: HashMap<String, CoreIdChirho>,
    classes_chirho: HashSet<String>,
    projections_chirho: Vec<(BinderChirho, CoreExprChirho)>,
}

impl DictPassCtxChirho {
    pub(super) fn scoped_method_head_chirho(
        &self,
        id_chirho: CoreIdChirho,
        dictionaries_chirho: &HashMap<String, CoreIdChirho>,
        classes_chirho: &HashSet<String>,
    ) -> Option<CoreExprChirho> {
        let canonical_chirho = self.canonical_id_chirho(id_chirho);
        if self
            .local_shadow_ids_chirho
            .borrow()
            .contains(&canonical_chirho)
        {
            return None;
        }
        let evidence_chirho = self.occurrence_evidence_chirho.get(&id_chirho)?;
        if !haskelujah_typing_chirho::infer_chirho::is_sigma_dictionary_key_chirho(
            &evidence_chirho.ty_key_chirho,
        ) {
            return None;
        }
        let name_chirho = self
            .method_occurrence_canon_chirho
            .get(&id_chirho)
            .map(|(name_chirho, _)| name_chirho)
            .or_else(|| self.names_chirho.get(&id_chirho))?;
        let namespace_chirho = self.proof_namespace_chirho(
            &evidence_chirho.class_name_chirho,
            &evidence_chirho.class_identity_chirho,
        );
        let method_chirho = name_chirho
            .rsplit('.')
            .next()
            .unwrap_or(name_chirho)
            .trim_start_matches('(')
            .trim_end_matches(')');
        let selector_chirho =
            match self.displaced_identity_chirho(&evidence_chirho.class_identity_chirho) {
                Some(identity_chirho) => self
                    .compiler_selectors_chirho
                    .get(&(identity_chirho, method_chirho.to_string()))
                    .copied(),
                None => self
                    .class_method_selector_for_name_chirho(name_chirho)
                    .filter(|(class_chirho, _)| *class_chirho == namespace_chirho)
                    .map(|(_, selector_chirho)| selector_chirho),
            }?;
        let Some(dictionary_chirho) = Self::own_dict_chirho(
            &namespace_chirho,
            &evidence_chirho.ty_key_chirho,
            dictionaries_chirho,
            classes_chirho,
        ) else {
            return Some(self.named_dictionary_refusal_chirho(
                None,
                format!(
                    "checked sigma method {method_chirho} has no dictionary in its lexical scope"
                ),
            ));
        };
        Some(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::VarChirho(selector_chirho)),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(dictionary_chirho)),
        })
    }

    /// Dictionary positions come from the actual checked application spine.
    /// No result-spine context is guessed from a type spelling in Core.
    pub(super) fn rewrite_positioned_reference_chirho(
        &self,
        expression_chirho: &CoreExprChirho,
        dictionaries_chirho: &HashMap<String, CoreIdChirho>,
        classes_chirho: &HashSet<String>,
        type_keys_chirho: &HashMap<CoreIdChirho, String>,
        instances_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let (written_chirho, arguments_chirho) =
            Self::collect_app_head_var_chirho(expression_chirho)?;
        let proofs_chirho = self.reference_evidence_chirho.get(&written_chirho)?;
        if !proofs_chirho
            .iter()
            .any(|proof_chirho| proof_chirho.dictionary_position_chirho != Some(0))
        {
            return None;
        }
        if proofs_chirho.iter().any(|proof_chirho| {
            proof_chirho
                .dictionary_position_chirho
                .is_none_or(|position_chirho| position_chirho > arguments_chirho.len())
        }) || proofs_chirho.windows(2).any(|pair_chirho| {
            pair_chirho[0].dictionary_position_chirho > pair_chirho[1].dictionary_position_chirho
        }) {
            return Some(self.named_dictionary_refusal_chirho(
                None,
                "checked dictionary positions disagree with the written application".to_owned(),
            ));
        }
        if let Some(refusal_chirho) = self.displaced_helper_refusal_chirho(written_chirho) {
            return Some(refusal_chirho);
        }
        let mut result_chirho = CoreExprChirho::VarChirho(self.canonical_id_chirho(written_chirho));
        let mut proofs_chirho = proofs_chirho
            .iter()
            .filter(|proof_chirho| proof_chirho.class_name_chirho != "~")
            .peekable();
        for position_chirho in 0..=arguments_chirho.len() {
            while proofs_chirho.peek().is_some_and(|proof_chirho| {
                proof_chirho.dictionary_position_chirho == Some(position_chirho)
            }) {
                let proof_chirho = proofs_chirho.next()?;
                if proof_chirho.ty_key_chirho.is_none() {
                    return Some(self.named_dictionary_refusal_chirho(
                        None,
                        "a result-spine dictionary has no checked reference evidence".to_owned(),
                    ));
                }
                let namespace_chirho = self.reference_namespace_chirho(proof_chirho);
                let argument_chirho = self.reference_dictionary_argument_chirho(
                    written_chirho,
                    super::constrained_references_chirho::ReferenceDictionarySlotChirho {
                        class_name_chirho: &namespace_chirho,
                        proof_chirho: Some(proof_chirho),
                    },
                    None,
                    dictionaries_chirho,
                    classes_chirho,
                    instances_chirho,
                );
                result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(argument_chirho),
                };
            }
            if let Some(argument_chirho) = arguments_chirho.get(position_chirho) {
                result_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(result_chirho),
                    arg_chirho: Box::new(self.rewrite_method_refs_with_locals_chirho(
                        argument_chirho,
                        dictionaries_chirho,
                        classes_chirho,
                        type_keys_chirho,
                        instances_chirho,
                    )),
                };
            }
        }
        Some(result_chirho)
    }

    pub(super) fn scoped_class_key_chirho(key_chirho: &str, class_chirho: &str) -> String {
        format!("{key_chirho}:{}:{class_chirho}", class_chirho.len())
    }

    pub(super) fn prepare_scoped_dictionaries_chirho(&mut self) {
        for (id_chirho, binding_chirho) in &self.sigma_local_schemes_chirho {
            let classes_chirho = binding_chirho
                .scheme_chirho
                .preds_chirho
                .iter()
                .filter(|predicate_chirho| predicate_chirho.class_name_chirho != "~")
                .map(|predicate_chirho| self.predicate_namespace_chirho(predicate_chirho))
                .collect::<Vec<_>>();
            if !classes_chirho.is_empty() {
                self.dict_param_bindings_chirho
                    .insert(*id_chirho, classes_chirho);
            }
        }
        let mut parameters_chirho = self
            .sigma_dictionary_binders_chirho
            .iter()
            .map(|(id_chirho, parameter_chirho)| (*id_chirho, parameter_chirho.clone()))
            .collect::<Vec<_>>();
        parameters_chirho.sort_by_key(|(id_chirho, _)| *id_chirho);
        for (id_chirho, parameter_chirho) in parameters_chirho {
            let predicate_chirho = parameter_chirho.predicate_chirho();
            let namespace_chirho = self.proof_namespace_chirho(
                &predicate_chirho.class_name_chirho,
                predicate_chirho.class_identity_chirho(),
            );
            let key_chirho = parameter_chirho
                .scope_chirho
                .id_chirho
                .parameter_key_chirho(parameter_chirho.index_chirho);
            let mut environment_chirho = ScopedDictionaryEnvironmentChirho::default();
            environment_chirho.dictionaries_chirho.insert(
                Self::scoped_class_key_chirho(&key_chirho, &namespace_chirho),
                id_chirho,
            );
            environment_chirho
                .classes_chirho
                .insert(namespace_chirho.clone());
            environment_chirho.projections_chirho = self.project_dictionary_superclasses_chirho(
                vec![(
                    namespace_chirho,
                    predicate_chirho.class_identity_chirho().clone(),
                    id_chirho,
                )],
                Some(&key_chirho),
                &mut environment_chirho.dictionaries_chirho,
                &mut environment_chirho.classes_chirho,
            );
            self.scoped_dictionary_environments_chirho
                .insert(id_chirho, environment_chirho);
        }
    }

    pub(super) fn rewrite_scoped_dictionary_lambda_chirho(
        &self,
        binder_chirho: &BinderChirho,
        body_chirho: &CoreExprChirho,
        dict_vars_chirho: &HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &HashSet<String>,
        local_type_keys_chirho: &HashMap<CoreIdChirho, String>,
        local_instance_dicts_chirho: &HashMap<(String, String), CoreIdChirho>,
    ) -> Option<CoreExprChirho> {
        let environment_chirho = self
            .scoped_dictionary_environments_chirho
            .get(&binder_chirho.id_chirho)?;
        let mut dictionaries_chirho = dict_vars_chirho.clone();
        dictionaries_chirho.extend(
            environment_chirho
                .dictionaries_chirho
                .iter()
                .map(|(key_chirho, id_chirho)| (key_chirho.clone(), *id_chirho)),
        );
        let mut classes_chirho = evidence_classes_chirho.clone();
        classes_chirho.extend(environment_chirho.classes_chirho.iter().cloned());
        let mut body_chirho = self.rewrite_method_refs_with_locals_chirho(
            body_chirho,
            &dictionaries_chirho,
            &classes_chirho,
            local_type_keys_chirho,
            local_instance_dicts_chirho,
        );
        let mut used_chirho = crate::simplify_chirho::free_vars_chirho(&body_chirho);
        // Keep the value-parameter spine intact. These lazy projections depend
        // only on dictionaries already in scope, so they belong inside its
        // leading lambdas, not between two runtime parameters.
        let mut target_chirho = &mut body_chirho;
        loop {
            target_chirho = match target_chirho {
                CoreExprChirho::LamChirho { body_chirho, .. }
                | CoreExprChirho::TyLamChirho { body_chirho, .. } => body_chirho,
                _ => break,
            };
        }
        for (projection_chirho, rhs_chirho) in environment_chirho.projections_chirho.iter().rev() {
            if used_chirho.contains(&projection_chirho.id_chirho) {
                used_chirho.extend(crate::simplify_chirho::free_vars_chirho(rhs_chirho));
                let previous_chirho = std::mem::replace(
                    target_chirho,
                    CoreExprChirho::LitChirho(crate::expr_chirho::CoreLitChirho::IntChirho(0)),
                );
                *target_chirho = CoreExprChirho::LetChirho {
                    rec_chirho: false,
                    binds_chirho: vec![(projection_chirho.clone(), rhs_chirho.clone())],
                    body_chirho: Box::new(previous_chirho),
                };
            }
        }
        Some(CoreExprChirho::LamChirho {
            binder_chirho: binder_chirho.clone(),
            body_chirho: Box::new(body_chirho),
        })
    }

    pub(super) fn project_dictionary_superclasses_chirho(
        &mut self,
        roots_chirho: Vec<(String, PredicateClassProofChirho, CoreIdChirho)>,
        scope_key_chirho: Option<&str>,
        dict_vars_chirho: &mut HashMap<String, CoreIdChirho>,
        evidence_classes_chirho: &mut HashSet<String>,
    ) -> Vec<(BinderChirho, CoreExprChirho)> {
        let mut super_let_binds_chirho = Vec::new();
        let mut available_dicts_chirho: HashMap<String, CoreIdChirho> = HashMap::new();
        let mut pending_classes_chirho: Vec<(String, PredicateClassProofChirho)> = Vec::new();
        for (namespace_chirho, proof_chirho, dict_id_chirho) in roots_chirho {
            available_dicts_chirho.insert(namespace_chirho.clone(), dict_id_chirho);
            pending_classes_chirho.push((namespace_chirho, proof_chirho));
        }

        while let Some((class_name_chirho, proof_chirho)) = pending_classes_chirho.pop() {
            let Some(sub_dict_id_chirho) = available_dicts_chirho.get(&class_name_chirho).copied()
            else {
                continue;
            };
            let super_slots_chirho = self
                .proof_layout_chirho(&class_name_chirho, &proof_chirho)
                .map(|layout_chirho| {
                    layout_chirho
                        .super_slots_chirho
                        .iter()
                        .map(|slot_chirho| {
                            (
                                slot_chirho.namespace_chirho.clone(),
                                slot_chirho.class_proof_chirho.clone(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            for (super_name_chirho, super_proof_chirho) in super_slots_chirho {
                let unlaid_displaced_chirho = self
                    .displaced_identity_chirho(&super_proof_chirho)
                    .is_some()
                    && self
                        .proof_layout_chirho(&super_name_chirho, &super_proof_chirho)
                        .is_none();
                if unlaid_displaced_chirho
                    || available_dicts_chirho.contains_key(&super_name_chirho)
                {
                    continue;
                }
                let Some(sel_id_chirho) = self
                    .super_selectors_chirho
                    .get(&(class_name_chirho.clone(), super_name_chirho.clone()))
                    .copied()
                else {
                    continue;
                };

                let super_dict_binder_chirho = self.fresh_binder_chirho(
                    &format!("$d{}", super_name_chirho),
                    TyChirho::ConChirho(format!("$Dict_{}", super_name_chirho)),
                );
                let extraction_chirho = CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(sel_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(sub_dict_id_chirho)),
                };
                evidence_classes_chirho.insert(super_name_chirho.clone());
                let key_chirho = scope_key_chirho.map_or_else(
                    || super_name_chirho.clone(),
                    |scope_key_chirho| {
                        Self::scoped_class_key_chirho(scope_key_chirho, &super_name_chirho)
                    },
                );
                dict_vars_chirho.insert(key_chirho, super_dict_binder_chirho.id_chirho);
                available_dicts_chirho.insert(
                    super_name_chirho.clone(),
                    super_dict_binder_chirho.id_chirho,
                );
                pending_classes_chirho.push((super_name_chirho, super_proof_chirho));
                super_let_binds_chirho.push((super_dict_binder_chirho, extraction_chirho));
            }
        }
        super_let_binds_chirho
    }
}
