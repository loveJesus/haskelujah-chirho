// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Module desugaring, moved whole from the desugar root (row 691, D7): the module pass that
//! records constructors, classes and instance rows and then desugars every binding, and the
//! type keys that name instance rows. Its later split by pass is a separate measured row.

use super::*;

impl DesugarCtxChirho {
    /// Derive the type key string from an AST `TypeChirho`, matching
    /// the format produced by `Display for TyChirho` in the typing layer
    /// so that the `$prim_` binding names align with `dict_chirho.rs`.
    pub fn type_key_from_ast_chirho(
        ty_chirho: &TypeChirho,
        scope_chirho: Option<&PromotionScopeChirho>,
    ) -> String {
        match ty_chirho {
            TypeChirho::KindAnnotationChirho { ty_chirho, .. } => {
                Self::type_key_from_ast_chirho(ty_chirho, scope_chirho)
            }
            TypeChirho::KindApplicationChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => format!(
                "({} {})",
                Self::type_key_from_ast_chirho(fun_chirho, scope_chirho),
                Self::type_key_from_ast_chirho(arg_chirho, scope_chirho)
            ),
            TypeChirho::ConChirho(name_chirho) => {
                if let Some(declaration_chirho) = name_chirho.type_constructor_chirho() {
                    return declaration_chirho.type_key_chirho();
                }
                unquoted_promoted_spelling_chirho(name_chirho, scope_chirho)
                    .unwrap_or_else(|| name_chirho.text_chirho().to_string())
            }
            TypeChirho::PromotedConChirho { name_chirho, .. } => {
                quoted_promoted_spelling_chirho(name_chirho, scope_chirho)
                    .unwrap_or_else(|| "_".to_string())
            }
            TypeChirho::VarChirho(name_chirho) => name_chirho.text_chirho().to_string(),
            TypeChirho::ListChirho { element_chirho, .. } => {
                format!(
                    "[{}]",
                    Self::type_key_from_ast_chirho(element_chirho, scope_chirho)
                )
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => {
                let inner_chirho: Vec<String> = elements_chirho
                    .iter()
                    .map(|e_chirho| Self::type_key_from_ast_chirho(e_chirho, scope_chirho))
                    .collect();
                format!("({})", inner_chirho.join(", "))
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                format!(
                    "{} {}",
                    Self::type_key_from_ast_chirho(fun_chirho, scope_chirho),
                    Self::type_key_from_ast_chirho(arg_chirho, scope_chirho)
                )
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                Self::type_key_from_ast_chirho(inner_chirho, scope_chirho)
            }
            _ => "_".to_string(),
        }
    }

    /// Build a type key for an instance declaration from its AST types.
    /// For single-parameter classes: just the first type key.
    /// For multi-parameter classes: head_key + "_" + extra keys joined by "_".
    /// Matches the format used in `dict_chirho.rs::generate_instance_dicts_chirho`.
    pub fn instance_type_key_chirho(
        types_chirho: &[TypeChirho],
        scope_chirho: Option<&PromotionScopeChirho>,
    ) -> String {
        if types_chirho.is_empty() {
            "_".to_string()
        } else if types_chirho.len() == 1 {
            Self::type_key_from_ast_chirho(&types_chirho[0], scope_chirho)
        } else {
            let keys_chirho: Vec<String> = types_chirho
                .iter()
                .map(|t_chirho| Self::type_key_from_ast_chirho(t_chirho, scope_chirho))
                .collect();
            keys_chirho.join("_")
        }
    }

    /// Desugar an entire AST module into a Core module with name map.
    pub fn desugar_module_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) -> Result<DesugarOutputChirho, DesugarInputErrorChirho> {
        haskelujah_ast_chirho::decl_chirho::check_module_constructor_signatures_chirho(
            module_chirho,
        )?;
        let mut projection_budget_chirho = GadtProjectionBudgetChirho::with_limits_chirho(
            haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitsChirho {
                work_chirho: 1 << 27,
                output_chirho: 1 << 24,
                depth_chirho: 512,
            },
        );
        check_module_projection_chirho(module_chirho, &mut projection_budget_chirho, 3)?;
        let expanded_rows_chirho = input_chirho::check_executable_patterns_chirho(
            module_chirho,
            &mut projection_budget_chirho,
            None,
        )?;
        let record_selector_plans_chirho =
            self.record_selector_plans_chirho(module_chirho, &mut projection_budget_chirho)?;
        self.current_module_name_chirho = Some(module_chirho.name_chirho.text_chirho().to_string());

        // Pass -1: Collect constructor strictness annotations for strict field enforcement
        self.collect_con_strictness_chirho(module_chirho);

        // Pass -0.5: Collect pattern synonym definitions for expansion during desugaring
        self.collect_pat_syns_chirho(module_chirho);
        if expanded_rows_chirho {
            // The original syntax gate precedes metadata collection. The SAME
            // remaining allowance now reserves complete matcher-owned columns.
            input_chirho::check_executable_patterns_chirho(
                module_chirho,
                &mut projection_budget_chirho,
                Some(self),
            )?;
        }

        // Pass -0.25: Seed aliases for built-in qualified imports whose Core
        // bodies use internal Prelude wrapper names.
        self.seed_builtin_import_aliases_chirho(module_chirho);
        // Unit 517: every `toList` an import of Data.List.NonEmpty or Data.Foldable
        // exposes reaches the dual-shape route, ahead of the module's own names.
        self.seed_to_list_routes_chirho(module_chirho);
        // QualifiedDo methods are selected through the source module qualifier,
        // while today's Core linker addresses imported values by bare export
        // name. Snapshot Prelude's methods before local bindings shadow them.
        // Other modules stay qualified and fail loudly until the linker can
        // distinguish same-named exports by defining module.
        // workflow: language-features-chirho/qualified-do-chirho
        self.seed_prelude_qualified_do_aliases_chirho(module_chirho);

        // Pass 0: Build map of class → (method_name → default MatchArms) from class declarations
        let mut class_defaults_chirho: HashMap<String, HashMap<String, Vec<MatchArmChirho>>> =
            HashMap::new();
        // Every row an instance takes from its class's default, by name (row 694).
        let mut default_method_rows_chirho: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let mut class_method_names_chirho: HashMap<String, Vec<String>> = HashMap::new();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                ..
            } = decl_chirho
            {
                let cn_chirho = name_chirho.text_chirho().to_string();
                let mut defaults_chirho = HashMap::new();
                let mut method_names_chirho = Vec::new();
                for method_chirho in methods_chirho {
                    let mn_chirho = method_chirho.name_chirho.text_chirho().to_string();
                    method_names_chirho.push(mn_chirho.clone());
                    if let Some(ref arms_chirho) = method_chirho.default_chirho {
                        defaults_chirho.insert(mn_chirho, arms_chirho.clone());
                    }
                }
                class_defaults_chirho.insert(cn_chirho.clone(), defaults_chirho);
                class_method_names_chirho.insert(cn_chirho, method_names_chirho);
            }
        }

        // Pass 1: Pre-bind all top-level names so they can reference each other
        let mut pre_binders_chirho: Vec<(usize, BinderChirho)> = Vec::new();
        for (idx_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
            match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    span_chirho,
                    ..
                } => {
                    let binder_chirho = self.fresh_binder_chirho(
                        name_chirho.text_chirho(),
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                            self.next_id_chirho,
                        )),
                        *span_chirho,
                    );
                    self.bind_in_scope_chirho(name_chirho.text_chirho(), binder_chirho.id_chirho);
                    pre_binders_chirho.push((idx_chirho, binder_chirho));
                }
                DeclChirho::InstanceDeclChirho {
                    class_chirho,
                    types_chirho,
                    methods_chirho,
                    span_chirho,
                    ..
                } => {
                    let class_name_chirho = class_chirho.text_chirho();
                    let type_key_chirho = Self::instance_type_key_chirho(
                        types_chirho,
                        self.promotion_scope_chirho.as_deref(),
                    );
                    // The rows below are named by this key; the dictionary pass pairs the
                    // declaration with the checker's instance by class and span (S3).
                    // workflow: language-features-chirho/dictionary-evidence-chirho
                    self.instance_row_keys_chirho
                        .push(crate::dict_chirho::InstanceRowKeyChirho {
                            class_name_chirho: class_name_chirho.to_string(),
                            span_chirho: *span_chirho,
                            row_key_chirho: type_key_chirho.clone(),
                        });
                    // Collect provided method names
                    let mut provided_chirho: std::collections::HashSet<String> =
                        std::collections::HashSet::new();
                    for method_chirho in methods_chirho {
                        if let LocalBindChirho::FunBindChirho {
                            name_chirho: method_name_chirho,
                            ..
                        } = method_chirho
                        {
                            let mn_chirho = method_name_chirho.text_chirho().to_string();
                            provided_chirho.insert(mn_chirho.clone());
                            let prim_name_chirho = format!(
                                "$prim_{}_{}_{}",
                                class_name_chirho, mn_chirho, type_key_chirho
                            );
                            let binder_chirho = self.fresh_binder_chirho(
                                &prim_name_chirho,
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                *span_chirho,
                            );
                            self.bind_in_scope_chirho(&prim_name_chirho, binder_chirho.id_chirho);
                            pre_binders_chirho.push((idx_chirho, binder_chirho));
                        }
                    }
                    // Pre-bind default methods not provided by the instance
                    if let Some(defaults_chirho) = class_defaults_chirho.get(class_name_chirho) {
                        for default_name_chirho in defaults_chirho.keys() {
                            if !provided_chirho.contains(default_name_chirho) {
                                let prim_name_chirho = format!(
                                    "$prim_{}_{}_{}",
                                    class_name_chirho, default_name_chirho, type_key_chirho
                                );
                                let binder_chirho = self.fresh_binder_chirho(
                                    &prim_name_chirho,
                                    TyChirho::VarChirho(
                                        haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                            self.next_id_chirho,
                                        ),
                                    ),
                                    *span_chirho,
                                );
                                self.bind_in_scope_chirho(
                                    &prim_name_chirho,
                                    binder_chirho.id_chirho,
                                );
                                pre_binders_chirho.push((idx_chirho, binder_chirho));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        self.prebind_record_accessors_chirho(&record_selector_plans_chirho);

        // Convert AST inline pragmas to Core inline annotations
        let inline_pragmas_chirho = &module_chirho.inline_pragmas_chirho;

        // Pass 2: Desugar bodies
        let mut bindings_chirho = Vec::new();
        let mut pre_iter_chirho = pre_binders_chirho.into_iter().peekable();

        for (idx_chirho, decl_chirho) in module_chirho.decls_chirho.iter().enumerate() {
            match decl_chirho {
                DeclChirho::FunBindChirho {
                    name_chirho,
                    matches_chirho,
                    span_chirho,
                } => {
                    let core_rhs_chirho = self.desugar_matches_chirho(matches_chirho, *span_chirho);
                    // Use the pre-bound binder
                    let binder_chirho = if let Some((pre_idx, _)) = pre_iter_chirho.peek() {
                        if *pre_idx == idx_chirho {
                            pre_iter_chirho.next().unwrap().1
                        } else {
                            // Shouldn't happen, but fall back
                            self.fresh_binder_chirho(
                                "_unknown",
                                TyChirho::VarChirho(
                                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                        self.next_id_chirho,
                                    ),
                                ),
                                *span_chirho,
                            )
                        }
                    } else {
                        self.fresh_binder_chirho(
                            "_unknown",
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            *span_chirho,
                        )
                    };
                    // Look up inline pragma for this binding
                    let inline_annotation_chirho = match inline_pragmas_chirho.get(&name_chirho.text_chirho().to_string()) {
                        Some(haskelujah_ast_chirho::module_chirho::InlinePragmaChirho::InlineChirho) => InlineAnnotationChirho::AlwaysChirho,
                        Some(haskelujah_ast_chirho::module_chirho::InlinePragmaChirho::NoInlineChirho) => InlineAnnotationChirho::NeverChirho,
                        Some(haskelujah_ast_chirho::module_chirho::InlinePragmaChirho::InlinableChirho) => InlineAnnotationChirho::InlinableChirho,
                        None => InlineAnnotationChirho::NoneChirho,
                    };
                    bindings_chirho.push(CoreBindingChirho {
                        binder_chirho,
                        rhs_chirho: core_rhs_chirho,
                        is_rec_chirho: true,
                        inline_chirho: inline_annotation_chirho,
                    });
                }
                DeclChirho::PatBindChirho {
                    pat_chirho,
                    rhs_chirho,
                    span_chirho,
                } => {
                    let core_rhs_chirho = self.desugar_rhs_chirho(rhs_chirho);
                    // Simple variable pattern: just create a direct binding
                    if let PatChirho::VarChirho(name_chirho) = pat_chirho {
                        let binder_chirho = self.fresh_binder_chirho(
                            name_chirho.text_chirho(),
                            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                self.next_id_chirho,
                            )),
                            *span_chirho,
                        );
                        self.bind_in_scope_chirho(
                            name_chirho.text_chirho(),
                            binder_chirho.id_chirho,
                        );
                        bindings_chirho.push(CoreBindingChirho {
                            binder_chirho,
                            rhs_chirho: core_rhs_chirho,
                            is_rec_chirho: false,
                            inline_chirho: InlineAnnotationChirho::NoneChirho,
                        });
                    } else {
                        // A top-level pattern binding, as the Report defines it:
                        // each variable is bound through one shared match of the
                        // WHOLE pattern, performed when any of them is demanded.
                        // (A top-level binding cannot be banged, so there is no
                        // witness to force.)
                        // workflow: language-features-chirho/pattern-matching-chirho
                        let binding_chirho = self.pattern_binding_chirho(
                            pat_chirho,
                            core_rhs_chirho,
                            &HashMap::new(),
                        );
                        for (binder_chirho, rhs_chirho) in binding_chirho.bindings_chirho {
                            bindings_chirho.push(CoreBindingChirho {
                                binder_chirho,
                                rhs_chirho,
                                is_rec_chirho: false,
                                inline_chirho: InlineAnnotationChirho::NoneChirho,
                            });
                        }
                    }
                }
                DeclChirho::InstanceDeclChirho {
                    class_chirho,
                    types_chirho,
                    methods_chirho,
                    span_chirho,
                    ..
                } => {
                    let class_name_chirho = class_chirho.text_chirho();
                    let type_key_chirho = Self::instance_type_key_chirho(
                        types_chirho,
                        self.promotion_scope_chirho.as_deref(),
                    );
                    let mut provided_chirho: std::collections::HashSet<String> =
                        std::collections::HashSet::new();
                    for method_chirho in methods_chirho {
                        if let LocalBindChirho::FunBindChirho {
                            name_chirho: method_name_chirho,
                            matches_chirho: method_matches_chirho,
                            span_chirho: method_span_chirho,
                        } = method_chirho
                        {
                            provided_chirho.insert(method_name_chirho.text_chirho().to_string());
                            let core_rhs_chirho = self
                                .desugar_matches_chirho(method_matches_chirho, *method_span_chirho);
                            let prim_name_chirho = format!(
                                "$prim_{}_{}_{}",
                                class_name_chirho,
                                method_name_chirho.text_chirho(),
                                type_key_chirho
                            );
                            // Consume pre-bound binder for this instance
                            // (multiple methods share the same idx_chirho)
                            let binder_chirho =
                                if let Some((pre_idx_chirho, _)) = pre_iter_chirho.peek() {
                                    if *pre_idx_chirho == idx_chirho {
                                        pre_iter_chirho.next().unwrap().1
                                    } else {
                                        self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                    }
                                } else {
                                    self.fresh_binder_chirho(
                                        &prim_name_chirho,
                                        TyChirho::VarChirho(
                                            haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                self.next_id_chirho,
                                            ),
                                        ),
                                        *span_chirho,
                                    )
                                };
                            bindings_chirho.push(CoreBindingChirho {
                                binder_chirho,
                                rhs_chirho: core_rhs_chirho,
                                is_rec_chirho: false,
                                inline_chirho: InlineAnnotationChirho::NoneChirho,
                            });
                        }
                    }
                    // Generate bindings for default methods not provided
                    if let Some(defaults_chirho) = class_defaults_chirho.get(class_name_chirho) {
                        for (default_name_chirho, default_arms_chirho) in defaults_chirho {
                            if !provided_chirho.contains(default_name_chirho) {
                                let core_rhs_chirho =
                                    self.desugar_matches_chirho(default_arms_chirho, *span_chirho);
                                let prim_name_chirho = format!(
                                    "$prim_{}_{}_{}",
                                    class_name_chirho, default_name_chirho, type_key_chirho
                                );
                                // A row the class default fills, never one the instance
                                // wrote: GND may replace it (row 694).
                                default_method_rows_chirho.insert(prim_name_chirho.clone());
                                let binder_chirho =
                                    if let Some((pre_idx_chirho, _)) = pre_iter_chirho.peek() {
                                        if *pre_idx_chirho == idx_chirho {
                                            pre_iter_chirho.next().unwrap().1
                                        } else {
                                            self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                        }
                                    } else {
                                        self.fresh_binder_chirho(
                                            &prim_name_chirho,
                                            TyChirho::VarChirho(
                                                haskelujah_typing_chirho::ty_chirho::TyVarChirho(
                                                    self.next_id_chirho,
                                                ),
                                            ),
                                            *span_chirho,
                                        )
                                    };
                                bindings_chirho.push(CoreBindingChirho {
                                    binder_chirho,
                                    rhs_chirho: core_rhs_chirho,
                                    is_rec_chirho: false,
                                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                                });
                            }
                        }
                    }
                }
                // Data, type, class declarations don't produce Core bindings directly
                _ => {}
            }
        }

        // Generate record field accessor functions.
        self.generate_record_accessors_chirho(&record_selector_plans_chirho, &mut bindings_chirho);

        Ok(DesugarOutputChirho {
            module_chirho: CoreModuleChirho {
                name_chirho: module_chirho.name_chirho.text_chirho().to_string(),
                bindings_chirho,
                names_chirho: self.names_chirho.clone(),
                specialize_pragmas_chirho: module_chirho.specialize_pragmas_chirho.clone(),
                foreign_exports_chirho: module_chirho
                    .foreign_exports_chirho
                    .iter()
                    .map(|(h_chirho, c_chirho, cc_chirho)| {
                        crate::expr_chirho::ForeignExportChirho {
                            haskell_name_chirho: h_chirho.clone(),
                            foreign_name_chirho: c_chirho.clone(),
                            calling_conv_chirho: cc_chirho.clone(),
                        }
                    })
                    .collect(),
                provenance_chirho: crate::expr_chirho::CoreProvenanceChirho {
                    imported_references_chirho: self.imported_references_chirho.clone(),
                    ..Default::default()
                },
            },
            names_chirho: self.names_chirho.clone(),
            method_occurrences_chirho: self.method_occurrences_chirho.clone(),
            literal_occurrence_spans_chirho: self.literal_occurrence_spans_chirho.clone(),
            range_occurrence_spans_chirho: self.range_occurrence_spans_chirho.clone(),
            reference_occurrence_spans_chirho: self.reference_occurrence_spans_chirho.clone(),
            method_occurrence_spans_chirho: self.method_occurrence_spans_chirho.clone(),
            occurrence_provenance_chirho: self.occurrence_provenance_chirho.clone(),
            operator_occurrences_chirho: self.operator_occurrences_chirho.clone(),
            instance_row_keys_chirho: self.instance_row_keys_chirho.clone(),
            default_method_rows_chirho,
        })
    }
}
