// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class-bound method specialization. Workflow: declaration-kinds-chirho.
use super::*;

impl InferCtxChirho {
    /// Bind source names and their checked kind identities in the same lexical
    /// map. A name already bound by the class head denotes that parameter;
    /// a method's own forall may subsequently shadow the source name.
    pub(super) fn seed_class_kind_variables_chirho(
        &mut self,
        class_name_chirho: &str,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> Vec<TyVarChirho> {
        let binders_chirho = self
            .kind_elaboration_chirho
            .as_ref()
            .and_then(|elaboration_chirho| {
                elaboration_chirho.class_heads_chirho.get(class_name_chirho)
            })
            .cloned()
            .unwrap_or_default();
        binders_chirho
            .iter()
            .map(|binder_chirho| {
                let variable_chirho = *variables_chirho
                    .entry(binder_chirho.parameter_name_chirho())
                    .or_insert_with(|| {
                        let variable_chirho = TyVarChirho(self.next_var_chirho);
                        self.next_var_chirho += 1;
                        variable_chirho
                    });
                variables_chirho.insert(binder_chirho.identity_key_chirho(), variable_chirho);
                variable_chirho
            })
            .collect()
    }

    /// The class's binder order and the instance occurrence are both owned by
    /// kind inference. Never replace a missing classifier with Type or erase
    /// the slot merely to make an associated family equation match.
    fn instance_method_kind_arguments_chirho(
        &mut self,
        class_decl_chirho: &ClassDeclChirho,
        declaration_index_chirho: usize,
        span_chirho: SpanChirho,
        variables_chirho: &mut HashMap<String, TyVarChirho>,
    ) -> Option<Vec<TyChirho>> {
        if class_decl_chirho.kind_vars_chirho.is_empty() {
            return Some(Vec::new());
        }
        let arguments_chirho = self
            .kind_elaboration_chirho
            .as_ref()
            .and_then(|elaboration_chirho| {
                elaboration_chirho
                    .class_instances_chirho
                    .get(&declaration_index_chirho)
            })
            .filter(|(name_chirho, _)| name_chirho == &class_decl_chirho.name_chirho)
            .map(|(_, arguments_chirho)| arguments_chirho)
            .filter(|arguments_chirho| {
                arguments_chirho.len() == class_decl_chirho.kind_vars_chirho.len()
            })
            .cloned();
        let Some(arguments_chirho) = arguments_chirho else {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(300),
                    format!(
                        "instance method has no checked kind arguments for class `{}`",
                        class_decl_chirho.name_chirho
                    ),
                    span_chirho,
                ));
            return None;
        };
        Some(
            arguments_chirho
                .iter()
                .map(|argument_chirho| {
                    self.kind_term_type_chirho(argument_chirho, variables_chirho, &mut Vec::new())
                })
                .collect(),
        )
    }

    pub(super) fn instantiate_instance_method_expected_parts_chirho(
        &mut self,
        class_decl_chirho: &ClassDeclChirho,
        method_scheme_chirho: &SchemeChirho,
        instance_head_tys_chirho: &[TyChirho],
        instance_kind_tys_chirho: &[TyChirho],
    ) -> (Vec<PredChirho>, TyChirho) {
        let mut method_subst_chirho = SubstChirho::empty_chirho();
        if let Some(head_ty_chirho) = instance_head_tys_chirho.first() {
            method_subst_chirho.insert_chirho(class_decl_chirho.var_chirho, head_ty_chirho.clone());
        }
        for (extra_var_chirho, head_ty_chirho) in class_decl_chirho
            .extra_vars_chirho
            .iter()
            .zip(instance_head_tys_chirho.iter().skip(1))
        {
            method_subst_chirho.insert_chirho(*extra_var_chirho, head_ty_chirho.clone());
        }
        for (kind_var_chirho, kind_ty_chirho) in class_decl_chirho
            .kind_vars_chirho
            .iter()
            .zip(instance_kind_tys_chirho)
        {
            method_subst_chirho.insert_chirho(*kind_var_chirho, kind_ty_chirho.clone());
        }
        for method_var_chirho in &method_scheme_chirho.vars_chirho {
            if method_subst_chirho
                .lookup_chirho(method_var_chirho)
                .is_none()
            {
                // Match ordinary signature checking: only a source binder is
                // a universal promise. Synthetic inference holes (including
                // the current builtin Generic representation) remain holes.
                let Some(name_chirho) = class_decl_chirho
                    .method_var_names_chirho
                    .get(method_var_chirho)
                    .filter(|name_chirho| !name_chirho.is_empty())
                else {
                    let fresh_chirho = self.fresh_var_chirho();
                    method_subst_chirho.insert_chirho(*method_var_chirho, fresh_chirho);
                    continue;
                };
                // Include the identity in the displayed base, not only the
                // internal suffix, so distinct skolems never print as X ~ X.
                let label_chirho = format!("{name_chirho}_{}", self.next_skolem_chirho);
                let skolem_chirho = TyChirho::ForallVarChirho(skolem_name_chirho(
                    &label_chirho,
                    self.next_skolem_chirho,
                ));
                self.next_skolem_chirho += 1;
                method_subst_chirho.insert_chirho(*method_var_chirho, skolem_chirho);
            }
        }
        let specialized_preds_chirho: Vec<PredChirho> = method_scheme_chirho
            .preds_chirho
            .iter()
            .map(|pred_chirho| PredChirho {
                class_name_chirho: pred_chirho.class_name_chirho.clone(),
                ty_chirho: self.normalize_ty_chirho(
                    &method_subst_chirho.apply_ty_chirho(&pred_chirho.ty_chirho),
                ),
                extra_tys_chirho: pred_chirho
                    .extra_tys_chirho
                    .iter()
                    .map(|ty_chirho| {
                        self.normalize_ty_chirho(&method_subst_chirho.apply_ty_chirho(ty_chirho))
                    })
                    .collect(),
            })
            .collect();
        let specialized_expected_ty_chirho =
            method_subst_chirho.apply_ty_chirho(&method_scheme_chirho.ty_chirho);
        (
            specialized_preds_chirho,
            self.normalize_ty_chirho(&specialized_expected_ty_chirho),
        )
    }

    #[cfg(test)]
    pub(super) fn instantiate_instance_method_expected_ty_chirho(
        &mut self,
        class_decl_chirho: &ClassDeclChirho,
        method_scheme_chirho: &SchemeChirho,
        instance_head_tys_chirho: &[TyChirho],
    ) -> TyChirho {
        let (_specialized_preds_chirho, specialized_ty_chirho) = self
            .instantiate_instance_method_expected_parts_chirho(
                class_decl_chirho,
                method_scheme_chirho,
                instance_head_tys_chirho,
                &[],
            );
        specialized_ty_chirho
    }

    pub(super) fn check_instance_methods_against_class_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) {
        for (declaration_index_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate()
        {
            let DeclChirho::InstanceDeclChirho {
                class_chirho,
                types_chirho,
                methods_chirho,
                span_chirho: instance_span_chirho,
                ..
            } = decl_chirho
            else {
                continue;
            };

            let class_name_chirho = class_chirho.text_chirho().to_string();
            let Some(class_decl_chirho) = self
                .class_env_chirho
                .classes_chirho
                .get(&class_name_chirho)
                .cloned()
            else {
                continue;
            };

            let mut instance_var_map_chirho = HashMap::new();
            let instance_head_tys_chirho: Vec<TyChirho> = types_chirho
                .iter()
                .map(|ty_chirho| {
                    self.ast_type_to_ty_chirho(ty_chirho, &mut instance_var_map_chirho)
                })
                .collect();
            let Some(instance_kind_tys_chirho) = self.instance_method_kind_arguments_chirho(
                &class_decl_chirho,
                declaration_index_chirho,
                *instance_span_chirho,
                &mut instance_var_map_chirho,
            ) else {
                continue;
            };
            for method_bind_chirho in methods_chirho {
                let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } = method_bind_chirho
                else {
                    continue;
                };

                let method_name_chirho = name_chirho.text_chirho().to_string();
                let Some(method_scheme_chirho) =
                    class_decl_chirho.methods_chirho.get(&method_name_chirho)
                else {
                    continue;
                };

                let env_snapshot_chirho = self.env_chirho.clone();
                let class_env_snapshot_chirho = self.class_env_chirho.clone();
                let scoped_tyvars_snapshot_chirho = self.scoped_tyvars_chirho.clone();
                let deferred_checkpoint_chirho = self.deferred_preds_chirho.len();
                let mut instance_scoped_tyvars_chirho = scoped_tyvars_snapshot_chirho.clone();
                if self.scoped_type_variables_chirho {
                    instance_scoped_tyvars_chirho.extend(instance_var_map_chirho.clone());
                }
                self.scoped_tyvars_chirho = instance_scoped_tyvars_chirho;
                let (mut specialized_given_preds_chirho, mut specialized_expected_ty_chirho) = self
                    .instantiate_instance_method_expected_parts_chirho(
                        &class_decl_chirho,
                        method_scheme_chirho,
                        &instance_head_tys_chirho,
                        &instance_kind_tys_chirho,
                    );
                if let Some(signature_chirho) = methods_chirho.iter().find_map(|binding_chirho| {
                    if let haskelujah_ast_chirho::expr_chirho::LocalBindChirho::TypeSigChirho {
                        name_chirho,
                        ty_chirho,
                        ..
                    } = binding_chirho
                        && name_chirho.text_chirho() == method_name_chirho
                    {
                        Some(ty_chirho)
                    } else {
                        None
                    }
                }) {
                    let (scheme_chirho, names_chirho) =
                        self.ast_type_to_scheme_with_var_map_chirho(signature_chirho);
                    // The written instance signature may be MORE general than
                    // the class obligation. Check that relation flexibly, then
                    // check its body against its own rigid, lexically scoped
                    // binders rather than the class signature's independent ones.
                    let (instance_ty_chirho, _) =
                        self.instantiate_scheme_parts_chirho(&scheme_chirho);
                    if let Err(error_chirho) = self.subsume_normalized_chirho(
                        &instance_ty_chirho,
                        &specialized_expected_ty_chirho,
                        *span_chirho,
                    ) {
                        self.report_unify_error_chirho(&error_chirho);
                    }
                    let (instance_ty_chirho, instance_givens_chirho) =
                        self.skolemize_scheme_parts_chirho(&scheme_chirho);
                    self.scope_signature_for_body_chirho(signature_chirho, &names_chirho);
                    specialized_expected_ty_chirho = instance_ty_chirho;
                    specialized_given_preds_chirho.extend(instance_givens_chirho);
                }
                let expected_ty_chirho = self.normalize_ty_chirho(&specialized_expected_ty_chirho);
                let (method_subst_chirho, inferred_ty_chirho) =
                    self.with_given_preds_chirho(specialized_given_preds_chirho, |self_chirho| {
                        self_chirho.infer_matches_against_expected_chirho(
                            matches_chirho,
                            *span_chirho,
                            &expected_ty_chirho,
                        )
                    });

                let inferred_norm_chirho = self
                    .normalize_ty_chirho(&method_subst_chirho.apply_ty_chirho(&inferred_ty_chirho));
                let expected_norm_chirho = self
                    .normalize_ty_chirho(&method_subst_chirho.apply_ty_chirho(&expected_ty_chirho));
                if let Err(err_chirho) = self.subsume_normalized_chirho(
                    &inferred_norm_chirho,
                    &expected_norm_chirho,
                    *span_chirho,
                ) {
                    self.report_unify_error_chirho(&err_chirho);
                }

                self.deferred_preds_chirho
                    .truncate(deferred_checkpoint_chirho);
                self.env_chirho = env_snapshot_chirho;
                self.class_env_chirho = class_env_snapshot_chirho;
                self.scoped_tyvars_chirho = scoped_tyvars_snapshot_chirho;
            }
        }
    }
}
