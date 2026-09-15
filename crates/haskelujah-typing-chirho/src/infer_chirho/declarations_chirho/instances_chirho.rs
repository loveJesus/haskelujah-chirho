// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class and instance declaration ownership.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;

impl InferCtxChirho {
    /// Process an instance declaration from the AST and register it.
    pub(super) fn process_instance_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        if let DeclChirho::InstanceDeclChirho {
            context_chirho,
            class_chirho,
            types_chirho,
            assoc_tf_instances_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = class_chirho.text_chirho().to_string();

            let mut var_map_chirho = HashMap::new();

            // Instance head type (e.g., `Int` in `instance Eq Int`, or
            // `[a]` in `instance Eq a => Eq [a]`)
            let head_ty_chirho = if let Some(first_ty_chirho) = types_chirho.first() {
                self.ast_type_to_ty_chirho(first_ty_chirho, &mut var_map_chirho)
            } else {
                self.fresh_var_chirho()
            };

            // Context constraints (e.g., `Eq a` in `instance Eq a => Eq [a]`)
            let inst_context_chirho: Vec<PredChirho> = context_chirho
                .iter()
                .filter_map(|c_chirho| match c_chirho {
                    AstConstraintChirho::ClassChirho {
                        class_chirho,
                        args_chirho,
                        ..
                    } => {
                        let cn_chirho = class_chirho.text_chirho().to_string();
                        let ct_chirho = if let Some(arg_chirho) = args_chirho.first() {
                            self.ast_type_to_ty_chirho(arg_chirho, &mut var_map_chirho)
                        } else {
                            self.fresh_var_chirho()
                        };
                        let extra_tys_chirho = args_chirho
                            .iter()
                            .skip(1)
                            .map(|argument_chirho| {
                                self.ast_type_to_ty_chirho(argument_chirho, &mut var_map_chirho)
                            })
                            .collect();
                        Some(PredChirho {
                            class_name_chirho: cn_chirho,
                            ty_chirho: ct_chirho,
                            extra_tys_chirho,
                        })
                    }
                    AstConstraintChirho::QuantifiedChirho { .. } => None,
                })
                .collect();

            // For MPTCs, extra head types come from types_chirho[1..]
            let extra_head_tys_chirho: Vec<TyChirho> = types_chirho
                .iter()
                .skip(1)
                .map(|t_chirho| self.ast_type_to_ty_chirho(t_chirho, &mut var_map_chirho))
                .collect();

            // Expand type synonyms in the instance head (e.g. String → [Char])
            let head_ty_chirho = self.expand_type_synonyms_chirho(&head_ty_chirho);
            let extra_head_tys_chirho: Vec<TyChirho> = extra_head_tys_chirho
                .into_iter()
                .map(|t_chirho| self.expand_type_synonyms_chirho(&t_chirho))
                .collect();

            // Capture from this declaration before mixing it with imported
            // assumptions. Deriving-generated declarations use this same path.
            if types_chirho.is_empty() || context_chirho.iter().any(|constraint_chirho|
                !matches!(constraint_chirho, AstConstraintChirho::ClassChirho { args_chirho, .. } if !args_chirho.is_empty()))
            {
                self.declaration_contracts_chirho.unproved_instances_chirho.insert(class_name_chirho.clone());
            } else {
                let mut vars_chirho: Vec<_> = var_map_chirho.values().copied().collect();
                vars_chirho.sort_unstable_by_key(|identity_chirho| identity_chirho.0);
                vars_chirho.dedup();
                let mut head_chirho = vec![head_ty_chirho.clone()];
                head_chirho.extend(extra_head_tys_chirho.iter().cloned());
                let preds_chirho = inst_context_chirho.iter().map(|pred_chirho| SchemePredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho.clone(),
                    ty_chirho: self.expand_type_synonyms_chirho(&pred_chirho.ty_chirho),
                    extra_tys_chirho: pred_chirho.extra_tys_chirho.iter()
                        .map(|ty_chirho| self.expand_type_synonyms_chirho(ty_chirho)).collect(),
                }).collect();
                self.declaration_contracts_chirho.instances_chirho.entry(class_name_chirho.clone())
                    .or_default().push(SchemeChirho { vars_chirho, preds_chirho, ty_chirho: TyChirho::TupleChirho(head_chirho) });
            }
            self.class_env_chirho.add_instance_chirho(InstDeclChirho {
                class_name_chirho,
                head_ty_chirho,
                extra_head_tys_chirho,
                context_chirho: inst_context_chirho,
            });

            self.register_associated_family_equations_chirho(
                class_chirho.text_chirho(),
                types_chirho,
                assoc_tf_instances_chirho,
            );
        }
    }
}
