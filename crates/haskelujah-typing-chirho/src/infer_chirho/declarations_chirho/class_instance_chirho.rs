// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class and instance declarations.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::{
    AssocTypeDefaultChirho, ast_type_to_syn_rhs_chirho, collect_free_type_vars_from_ast_chirho,
    instance_obligations_chirho, substitute_type_vars_chirho,
};
use crate::class_chirho::{ClassDeclChirho, InstDeclChirho, PredChirho};
use crate::ty_chirho::{SchemePredChirho, TyChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use std::collections::HashMap;

impl InferCtxChirho {
    /// Process a class declaration from the AST and register it in the class env.
    pub(super) fn process_class_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        if let DeclChirho::ClassDeclChirho {
            context_chirho,
            name_chirho,
            type_vars_chirho,
            methods_chirho,
            associated_tfs_chirho,
            fundeps_chirho: ast_fundeps_chirho,
            ..
        } = decl_chirho
        {
            let class_name_chirho = name_chirho.text_chirho().to_string();

            // Generate fresh type variables for all class params
            let class_tv_chirho = TyVarChirho(self.next_var_chirho);
            self.next_var_chirho += 1;

            let extra_vars_chirho: Vec<TyVarChirho> = type_vars_chirho
                .iter()
                .skip(1)
                .map(|_| {
                    let v_chirho = TyVarChirho(self.next_var_chirho);
                    self.next_var_chirho += 1;
                    v_chirho
                })
                .collect();

            let mut class_scoped_tyvars_chirho = HashMap::new();
            if let Some(first_var_chirho) = type_vars_chirho.first() {
                class_scoped_tyvars_chirho
                    .insert(first_var_chirho.text_chirho().to_string(), class_tv_chirho);
            }
            for (ast_var_chirho, ty_var_chirho) in type_vars_chirho
                .iter()
                .skip(1)
                .zip(extra_vars_chirho.iter().copied())
            {
                class_scoped_tyvars_chirho
                    .insert(ast_var_chirho.text_chirho().to_string(), ty_var_chirho);
            }

            // Superclasses from context
            let supers_chirho: Vec<String> = context_chirho
                .iter()
                .filter_map(|c_chirho| {
                    c_chirho
                        .simple_class_chirho()
                        .map(|class_chirho| class_chirho.text_chirho().to_string())
                })
                .collect();

            // Method signatures and optional default implementations
            let mut method_map_chirho = HashMap::new();
            let mut defaults_map_chirho = HashMap::new();
            for method_chirho in methods_chirho {
                let method_name_chirho = method_chirho.name_chirho.text_chirho().to_string();
                let (mut scheme_chirho, _method_var_map_chirho) = self
                    .ast_type_to_scheme_seeded_chirho(
                        &method_chirho.ty_chirho,
                        &class_scoped_tyvars_chirho,
                        false,
                    );

                let class_pred_chirho = SchemePredChirho {
                    class_name_chirho: class_name_chirho.clone(),
                    ty_chirho: TyChirho::VarChirho(class_tv_chirho),
                    extra_tys_chirho: extra_vars_chirho
                        .iter()
                        .copied()
                        .map(TyChirho::VarChirho)
                        .collect(),
                };
                if !scheme_chirho.preds_chirho.contains(&class_pred_chirho) {
                    scheme_chirho.preds_chirho.push(class_pred_chirho);
                }
                // A method's type is `forall <class vars>. C <class vars> =>
                // forall <own vars>. ty`: the class variables are quantified
                // first (visible type application `take @n` reaches `n` even
                // when the method type itself never mentions it), then the
                // method's own variables in their order of appearance.
                let mut ordered_vars_chirho: Vec<TyVarChirho> = vec![class_tv_chirho];
                ordered_vars_chirho.extend(extra_vars_chirho.iter().copied());
                for var_chirho in &scheme_chirho.vars_chirho {
                    if !ordered_vars_chirho.contains(var_chirho) {
                        ordered_vars_chirho.push(*var_chirho);
                    }
                }
                scheme_chirho.vars_chirho = ordered_vars_chirho;
                method_map_chirho.insert(method_name_chirho.clone(), scheme_chirho.clone());

                // Also add method to the type environment so it can be used
                self.env_chirho
                    .bind_chirho(method_name_chirho.clone(), scheme_chirho);

                // Capture default implementation if present
                if let Some(ref default_arms_chirho) = method_chirho.default_chirho {
                    defaults_map_chirho.insert(method_name_chirho, default_arms_chirho.clone());
                }
            }

            // Convert AST fundeps (variable names) to indices into type_vars_chirho
            let var_names_chirho: Vec<String> = type_vars_chirho
                .iter()
                .map(|v_chirho| v_chirho.text_chirho().to_string())
                .collect();
            let resolved_fundeps_chirho: Vec<(Vec<usize>, Vec<usize>)> = ast_fundeps_chirho
                .iter()
                .map(|(from_chirho, to_chirho)| {
                    let from_idx_chirho: Vec<usize> = from_chirho
                        .iter()
                        .filter_map(|n_chirho| {
                            var_names_chirho
                                .iter()
                                .position(|v_chirho| v_chirho == n_chirho)
                        })
                        .collect();
                    let to_idx_chirho: Vec<usize> = to_chirho
                        .iter()
                        .filter_map(|n_chirho| {
                            var_names_chirho
                                .iter()
                                .position(|v_chirho| v_chirho == n_chirho)
                        })
                        .collect();
                    (from_idx_chirho, to_idx_chirho)
                })
                .collect();

            self.fully_known_class_names_chirho
                .insert(class_name_chirho.clone());
            self.class_env_chirho.add_class_chirho(ClassDeclChirho {
                name_chirho: class_name_chirho.clone(),
                supers_chirho,
                var_chirho: class_tv_chirho,
                methods_chirho: method_map_chirho,
                extra_vars_chirho,
                fundeps_chirho: resolved_fundeps_chirho,
                defaults_chirho: defaults_map_chirho,
            });

            // Register associated type families as open type families. A
            // default equation is NOT a general equation of the family (that
            // would reduce `F a` for an abstract `a`); it is applied per
            // instance that leaves the family undefined.
            let class_params_chirho: Vec<String> = type_vars_chirho
                .iter()
                .map(|param_chirho| param_chirho.text_chirho().to_string())
                .collect();
            for atf_chirho in associated_tfs_chirho {
                let tf_name_chirho = atf_chirho.name_chirho.text_chirho().to_string();
                self.register_open_family_chirho(
                    tf_name_chirho.clone(),
                    atf_chirho.type_vars_chirho.len(),
                    atf_chirho.span_chirho,
                );
                self.assoc_type_declared_params_chirho.insert(
                    tf_name_chirho.clone(),
                    atf_chirho
                        .type_vars_chirho
                        .iter()
                        .map(|param_chirho| param_chirho.text_chirho().to_string())
                        .collect(),
                );
                if let Some(default_rhs_chirho) = &atf_chirho.default_rhs_chirho {
                    let binders_chirho = if atf_chirho.default_params_chirho.len()
                        == atf_chirho.type_vars_chirho.len()
                    {
                        &atf_chirho.default_params_chirho
                    } else {
                        &atf_chirho.type_vars_chirho
                    };
                    let param_names_chirho: Vec<String> = binders_chirho
                        .iter()
                        .map(|param_chirho| param_chirho.text_chirho().to_string())
                        .collect();
                    self.assoc_type_defaults_chirho
                        .entry(class_name_chirho.clone())
                        .or_default()
                        .push(AssocTypeDefaultChirho {
                            family_chirho: tf_name_chirho,
                            family_params_chirho: param_names_chirho,
                            class_params_chirho: class_params_chirho.clone(),
                            rhs_chirho: default_rhs_chirho.clone(),
                        });
                }
            }
        }
    }

    /// Process an instance declaration from the AST and register it.
    pub(super) fn process_instance_decl_chirho(&mut self, decl_chirho: &DeclChirho) {
        if let DeclChirho::InstanceDeclChirho {
            context_chirho,
            class_chirho,
            types_chirho,
            assoc_tf_instances_chirho,
            span_chirho,
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

            // Context constraints (e.g., `Eq a` in `instance Eq a => Eq [a]`), through
            // the SAME conversion a signature's context uses: every argument is kept, so
            // `Convert a String` does not behave as `Convert a`, and `a ~ Int` keeps both
            // sides. A quantified premise is still unmodelled and makes the context
            // incomplete rather than empty.
            // workflow: language-features-chirho/instance-obligations-chirho
            let (context_preds_chirho, context_equalities_chirho, _unmodelled_chirho) =
                self.convert_written_context_chirho(context_chirho, &mut var_map_chirho);
            let mut inst_context_chirho: Vec<PredChirho> = context_preds_chirho
                .into_iter()
                .map(|pred_chirho| PredChirho {
                    class_name_chirho: pred_chirho.class_name_chirho,
                    ty_chirho: pred_chirho.ty_chirho,
                    extra_tys_chirho: pred_chirho.extra_tys_chirho,
                })
                .collect();
            inst_context_chirho.extend(context_equalities_chirho.into_iter().map(
                |(lhs_chirho, rhs_chirho)| {
                    PredChirho::new_multi_chirho("~", vec![lhs_chirho, rhs_chirho])
                },
            ));

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

            let instance_chirho = InstDeclChirho {
                class_name_chirho,
                head_ty_chirho,
                extra_head_tys_chirho,
                context_chirho: inst_context_chirho,
            };
            // workflow: language-features-chirho/instance-obligations-chirho
            self.local_instances_chirho
                .push(instance_obligations_chirho::LocalInstanceChirho {
                    instance_chirho: instance_chirho.clone(),
                    nullary_chirho: types_chirho.is_empty(),
                    written_names_chirho: instance_obligations_chirho::written_head_names_chirho(
                        class_chirho,
                        types_chirho,
                    ),
                    span_chirho: *span_chirho,
                });
            self.class_env_chirho.add_instance_chirho(instance_chirho);

            // Register associated type family instances from this instance decl
            for atfi_chirho in assoc_tf_instances_chirho {
                let fname_chirho = atfi_chirho.family_name_chirho.full_name_chirho();
                let inst_params_chirho =
                    collect_free_type_vars_from_ast_chirho(&atfi_chirho.lhs_types_chirho);
                let lhs_chirho: Vec<TyChirho> = atfi_chirho
                    .lhs_types_chirho
                    .iter()
                    .map(|t_chirho| {
                        ast_type_to_syn_rhs_chirho(
                            t_chirho,
                            &inst_params_chirho,
                            self.promotion_scope_chirho.as_deref(),
                        )
                    })
                    .collect();
                let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(
                    &atfi_chirho.rhs_chirho,
                    &inst_params_chirho,
                    self.promotion_scope_chirho.as_deref(),
                );
                self.register_family_instance_at_chirho(
                    fname_chirho,
                    lhs_chirho,
                    rhs_ty_chirho,
                    atfi_chirho.span_chirho,
                    Some(0),
                );
            }

            // Associated families this instance leaves undefined take the
            // class's default, specialized to the instance head.
            if let Some(defaults_chirho) = self
                .assoc_type_defaults_chirho
                .get(class_chirho.text_chirho())
                .cloned()
            {
                let inst_free_vars_chirho = collect_free_type_vars_from_ast_chirho(types_chirho);
                for (default_index_chirho, default_chirho) in
                    defaults_chirho.into_iter().enumerate()
                {
                    let defined_here_chirho = assoc_tf_instances_chirho.iter().any(|atfi_chirho| {
                        atfi_chirho.family_name_chirho.text_chirho() == default_chirho.family_chirho
                    });
                    if defined_here_chirho {
                        continue;
                    }
                    // Family parameter `i` names class parameter `j` when the
                    // family DECLARATION spells position `i` with the class
                    // parameter's name; the default equation may use other
                    // spellings, so the mapping goes through positions.
                    let declared_names_chirho: Vec<String> = self
                        .assoc_type_declared_params_chirho
                        .get(&default_chirho.family_chirho)
                        .cloned()
                        .unwrap_or_else(|| default_chirho.family_params_chirho.clone());
                    let mut bindings_chirho: HashMap<String, TyChirho> = HashMap::new();
                    let mut lhs_chirho = Vec::new();
                    for (position_chirho, param_chirho) in
                        default_chirho.family_params_chirho.iter().enumerate()
                    {
                        let declared_chirho = declared_names_chirho
                            .get(position_chirho)
                            .unwrap_or(param_chirho);
                        match default_chirho
                            .class_params_chirho
                            .iter()
                            .position(|class_param_chirho| class_param_chirho == declared_chirho)
                        {
                            Some(index_chirho) if index_chirho < types_chirho.len() => {
                                let head_ty_chirho = ast_type_to_syn_rhs_chirho(
                                    &types_chirho[index_chirho],
                                    &inst_free_vars_chirho,
                                    self.promotion_scope_chirho.as_deref(),
                                );
                                bindings_chirho
                                    .insert(param_chirho.clone(), head_ty_chirho.clone());
                                lhs_chirho.push(head_ty_chirho);
                            }
                            _ => lhs_chirho.push(TyChirho::ForallVarChirho(param_chirho.clone())),
                        }
                    }
                    let rhs_pattern_chirho = ast_type_to_syn_rhs_chirho(
                        &default_chirho.rhs_chirho,
                        &default_chirho.family_params_chirho,
                        self.promotion_scope_chirho.as_deref(),
                    );
                    let rhs_chirho =
                        substitute_type_vars_chirho(&rhs_pattern_chirho, &bindings_chirho);
                    self.register_family_instance_at_chirho(
                        default_chirho.family_chirho.clone(),
                        lhs_chirho,
                        rhs_chirho,
                        *span_chirho,
                        u32::try_from(default_index_chirho).ok(),
                    );
                }
            }
        }
    }
}
