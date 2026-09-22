// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class and instance declaration ownership.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use super::*;

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

            let kind_vars_chirho = self.seed_class_kind_variables_chirho(
                &class_name_chirho,
                &mut class_scoped_tyvars_chirho,
            );

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
            let mut method_var_names_chirho = HashMap::new();
            for method_chirho in methods_chirho {
                let method_name_chirho = method_chirho.name_chirho.text_chirho().to_string();
                let (mut scheme_chirho, _method_var_map_chirho) = self
                    .ast_type_to_scheme_seeded_chirho(
                        &method_chirho.ty_chirho,
                        &class_scoped_tyvars_chirho,
                        false,
                    );

                for variable_chirho in &scheme_chirho.vars_chirho {
                    if let Some(name_chirho) = self.tyvar_source_names_chirho.get(variable_chirho) {
                        method_var_names_chirho.insert(*variable_chirho, name_chirho.clone());
                    }
                }

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
            let contract_fundeps_chirho = ast_fundeps_chirho
                .iter()
                .map(|(from_chirho, to_chirho)| {
                    let positions_chirho = |names_chirho: &[String]| {
                        names_chirho
                            .iter()
                            .map(|name_chirho| {
                                var_names_chirho
                                    .iter()
                                    .position(|bound_chirho| bound_chirho == name_chirho)
                            })
                            .collect::<Option<Vec<_>>>()
                    };
                    Some((positions_chirho(from_chirho)?, positions_chirho(to_chirho)?))
                })
                .collect::<Option<Vec<_>>>();
            let parameters_chirho: Vec<_> = std::iter::once(class_tv_chirho)
                .chain(extra_vars_chirho.iter().copied())
                .collect();
            let contract_chirho = self.capture_class_contract_chirho(
                decl_chirho,
                &class_scoped_tyvars_chirho,
                &parameters_chirho,
                &method_map_chirho,
                contract_fundeps_chirho,
            );
            self.declaration_contracts_chirho
                .classes_chirho
                .insert(class_name_chirho.clone(), contract_chirho);
            self.class_env_chirho.add_class_chirho(ClassDeclChirho {
                name_chirho: class_name_chirho.clone(),
                supers_chirho,
                var_chirho: class_tv_chirho,
                kind_vars_chirho,
                method_var_names_chirho,
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
                if atf_chirho.data_chirho {
                    continue;
                }
                let tf_name_chirho = atf_chirho.name_chirho.text_chirho().to_string();
                self.register_type_family_chirho(tf_name_chirho.clone(), vec![]);
                self.assoc_type_declared_params_chirho.insert(
                    tf_name_chirho.clone(),
                    atf_chirho
                        .type_vars_chirho
                        .iter()
                        .map(|param_chirho| param_chirho.text_chirho().to_string())
                        .collect(),
                );
                for equation_chirho in &atf_chirho.defaults_chirho {
                    let Some(param_names_chirho) = equation_chirho
                        .lhs_types_chirho
                        .iter()
                        .map(|param_chirho| {
                            if let TypeChirho::VarChirho(name_chirho) =
                                param_chirho.unannotated_chirho()
                            {
                                Some(name_chirho.text_chirho().to_owned())
                            } else {
                                None
                            }
                        })
                        .collect::<Option<Vec<_>>>()
                    else {
                        continue;
                    };
                    self.assoc_type_defaults_chirho
                        .entry(class_name_chirho.clone())
                        .or_default()
                        .push(AssocTypeDefaultChirho {
                            family_chirho: tf_name_chirho.clone(),
                            family_params_chirho: param_names_chirho,
                            class_params_chirho: class_params_chirho.clone(),
                            rhs_chirho: equation_chirho.rhs_chirho.clone(),
                            span_chirho: equation_chirho.span_chirho,
                        });
                }
            }
        }
    }
}
