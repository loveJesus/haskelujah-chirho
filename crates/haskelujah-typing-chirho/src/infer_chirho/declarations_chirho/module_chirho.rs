// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A whole module, inferred declaration by declaration.
//! Moved from `infer_chirho.rs` in row 661.

use super::InferCtxChirho;
use super::gadt_constructors_chirho::LocalConstructorFieldContextChirho;
use super::restriction_chirho::{effective_flag_chirho, group_is_restricted_chirho};
use super::{
    GeneralizationScopeChirho, SIGNATURE_MISMATCH_CODE_CHIRHO, ast_type_to_syn_rhs_chirho,
    binding_groups_chirho, canonical_value_name_text_chirho, collect_module_shadowed_names_chirho,
    records_chirho,
};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::{MultChirho, SchemeChirho, TyChirho};
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{MatchArmChirho, RhsChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_diagnostics_chirho::{DiagnosticChirho, ErrorCodeChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::{HashMap, HashSet};

impl InferCtxChirho {
    /// Checked fields wait for the existing local/associated family producer.
    /// Data-layout validity uses no GADT givens and is never a deferred type error.
    pub(super) fn finish_constructor_field_kinds_chirho(&mut self) -> bool {
        let view_chirho = super::EqualityNormalizationViewChirho {
            substitutions_chirho: &self.equality_substitutions_chirho,
            legacy_synonyms_chirho: &self.type_synonyms_chirho,
            type_scope_chirho: self.type_constructor_scope_chirho.as_deref(),
            family_scope_chirho: self.family_scope_chirho.as_deref(),
            family_module_chirho: &self.family_module_name_chirho,
            promotion_present_chirho: self.promotion_scope_chirho.is_some(),
            evidence_chirho: Default::default(),
        };
        match self
            .elaboration_state_chirho
            .finish_constructor_field_kinds_chirho(&self.type_families_chirho, view_chirho)
        {
            Ok(()) => true,
            Err(error_chirho) => {
                self.non_deferrable_diagnostics_chirho
                    .push_chirho(error_chirho.into_diagnostic_chirho());
                false
            }
        }
    }

    /// Run type inference on a module. Returns the result with environment,
    /// substitution, and diagnostics.
    pub fn infer_module_chirho(&mut self, module_chirho: &ModuleChirho) -> SubstChirho {
        self.type_validity_license_chirho = Some(
            crate::validity_chirho::LicenseChirho::from_module_chirho(module_chirho),
        );
        let mut subst_chirho = SubstChirho::empty_chirho();
        self.prepare_family_scope_chirho(module_chirho, self.family_scope_chirho.clone());

        // A non-Prelude import can bring instances of SEEDED classes into
        // scope (e.g. a sibling module declaring `instance Show I`) that the
        // class env does not model, so the seeded instance lists stop being
        // the whole universe. Only classes declared in THIS module then keep a
        // fully-known universe — an imported module can never instance a class
        // it cannot see — and the decl walk below re-populates those.
        // Prelude is exempt: it is the instance set `seed_standard_chirho`
        // models, and the driver injects it implicitly into every module.
        let has_non_prelude_import_chirho = module_chirho
            .imports_chirho
            .iter()
            .any(|import_chirho| import_chirho.module_chirho.text_chirho() != "Prelude");
        if has_non_prelude_import_chirho {
            self.fully_known_class_names_chirho.clear();
        }

        self.constraint_generation_faithful_chirho =
            !module_chirho
                .extensions_chirho
                .iter()
                .any(|extension_chirho| {
                    Self::UNFAITHFUL_CONSTRAINT_EXTENSIONS_CHIRHO
                        .contains(&extension_chirho.as_str())
                });
        // A literal names the compiler's own class unless the module rebinds its translation
        // or may write primitive literals, whose `#` the AST erases (row 692).
        self.compiler_literals_chirho =
            !effective_flag_chirho(&module_chirho.extensions_chirho, "RebindableSyntax", false)
                && !effective_flag_chirho(&module_chirho.extensions_chirho, "MagicHash", false);
        self.scoped_type_variables_chirho =
            Self::scoped_type_variables_enabled_chirho(module_chirho);
        self.relaxed_poly_rec_chirho =
            !module_chirho
                .extensions_chirho
                .iter()
                .any(|extension_chirho| {
                    extension_chirho == "Haskell98" || extension_chirho == "NoRelaxedPolyRec"
                });

        // Phase -1: Register type synonyms so they can be expanded during
        // type inference. Process in declaration order (handles chains
        // like type FilePath = String where String is already registered).
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                type_vars_chirho,
                rhs_chirho,
                ..
            } = decl_chirho
            {
                let syn_name_chirho = name_chirho.text_chirho().to_string();
                if self.uses_checked_declaration_chirho(&syn_name_chirho) {
                    continue;
                }
                let params_chirho: Vec<String> = type_vars_chirho
                    .iter()
                    .map(|v_chirho| v_chirho.text_chirho().to_string())
                    .collect();
                let mut stamped_rhs_chirho = rhs_chirho.clone();
                if let Some(scope_chirho) = self.family_scope_chirho.as_deref() {
                    super::stamp_family_references_chirho(
                        &mut stamped_rhs_chirho,
                        scope_chirho,
                        &self.type_families_chirho,
                        &self.family_module_name_chirho,
                    );
                }
                let rhs_ty_chirho = ast_type_to_syn_rhs_chirho(
                    &stamped_rhs_chirho,
                    &params_chirho,
                    self.promotion_scope_chirho.as_deref(),
                );
                self.register_type_synonym_chirho(syn_name_chirho, params_chirho, rhs_ty_chirho);
            }
        }

        // Deriving mentions become potential-instance suppressions for the
        // certainly-unsolvable check: `deriving Eq` on a local data decl is a
        // real `instance Eq T` in GHC even though the class env does not model
        // derived instances yet.
        for decl_chirho in &module_chirho.decls_chirho {
            match decl_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    deriving_chirho,
                    ..
                }
                | DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    deriving_chirho,
                    ..
                } => {
                    for derived_chirho in deriving_chirho {
                        self.derived_instance_heads_chirho.insert((
                            derived_chirho.class_text_chirho().to_string(),
                            name_chirho.text_chirho().to_string(),
                        ));
                    }
                }
                DeclChirho::StandaloneDerivingDeclChirho { class_chirho, .. } => {
                    self.derived_instance_heads_chirho
                        .insert((class_chirho.text_chirho().to_string(), "*".to_string()));
                }
                _ => {}
            }
        }
        for (first_chirho, second_chirho, _via_chirho) in &module_chirho.deriving_via_chirho {
            // Field order of the via tuple is not load-bearing here: insert
            // both orientations so the suppression works either way.
            self.derived_instance_heads_chirho.insert((
                first_chirho.text_chirho().to_string(),
                second_chirho.text_chirho().to_string(),
            ));
            self.derived_instance_heads_chirho.insert((
                second_chirho.text_chirho().to_string(),
                first_chirho.text_chirho().to_string(),
            ));
        }

        self.module_default_types_chirho.clear();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::DefaultDeclChirho { types_chirho, .. } = decl_chirho {
                let mut default_var_map_chirho = HashMap::new();
                for default_type_chirho in types_chirho {
                    let default_ty_chirho = self
                        .ast_type_to_ty_chirho(default_type_chirho, &mut default_var_map_chirho);
                    self.module_default_types_chirho.push(default_ty_chirho);
                }
            }
        }

        // Phase -0.5: immutable family declarations and coherent open rows.
        self.register_module_families_chirho(module_chirho);

        for shadowed_name_chirho in collect_module_shadowed_names_chirho(module_chirho) {
            self.env_chirho.remove_chirho(&shadowed_name_chirho);
        }

        // Phase 0: Collect type signatures for later checking
        let mut type_sigs_chirho: HashMap<String, TypeChirho> = HashMap::new();
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::TypeSigChirho {
                name_chirho,
                ty_chirho,
                ..
            } = decl_chirho
            {
                self.check_source_signature_ambiguity_chirho(ty_chirho);
                type_sigs_chirho.insert(
                    canonical_value_name_text_chirho(name_chirho.text_chirho()),
                    ty_chirho.clone(),
                );
            }
        }

        // Phase 0b: Seed foreign imports so later top-level bindings can use
        // them just like ordinary top-level definitions.
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::ForeignDeclChirho {
                direction_chirho:
                    haskelujah_ast_chirho::decl_chirho::ForeignDirectionChirho::ImportChirho,
                name_chirho,
                ty_chirho,
                ..
            } = decl_chirho
            {
                let foreign_scheme_chirho = self.ast_type_to_scheme_chirho(ty_chirho);
                self.env_chirho.bind_chirho(
                    canonical_value_name_text_chirho(name_chirho.text_chirho()),
                    foreign_scheme_chirho,
                );
            }
        }

        // Phase 1a: Process class declarations (before data/instance so methods
        // are available in the environment)
        for decl_chirho in &module_chirho.decls_chirho {
            self.process_class_decl_chirho(decl_chirho);
        }

        // Phase 1b: Process instance declarations
        for decl_chirho in &module_chirho.decls_chirho {
            self.process_instance_decl_chirho(decl_chirho);
        }
        if !self.finish_constructor_field_kinds_chirho() {
            return subst_chirho;
        }
        // workflow: language-features-chirho/instance-obligations-chirho
        self.check_local_instance_obligations_chirho(module_chirho);

        let type_constructor_scope_chirho = self.type_constructor_scope_chirho.clone();
        let module_name_chirho = module_chirho.name_chirho.full_name_chirho();

        let mut projection_budget_chirho =
            haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho();

        // Phase 2: Bind data and newtype constructor types
        for decl_chirho in &module_chirho.decls_chirho {
            if let Err(error_chirho) =
                haskelujah_ast_chirho::decl_chirho::check_declaration_projection_chirho(
                    decl_chirho,
                    &mut projection_budget_chirho,
                    1,
                )
            {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_chirho(
                        error_chirho.to_string(),
                        error_chirho.span_chirho,
                    ));
                continue;
            }
            match decl_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    constructors_chirho,
                    ..
                } => {
                    let field_context_chirho = LocalConstructorFieldContextChirho {
                        module_name_chirho: &module_name_chirho,
                        owner_declaration_chirho: type_constructor_scope_chirho.as_ref().and_then(
                            |scope_chirho| {
                                scope_chirho.declaration_chirho(&name_chirho.full_name_chirho())
                            },
                        ),
                    };
                    let (result_ty_chirho, mut tv_map_chirho) =
                        self.data_head_type_chirho(name_chirho.text_chirho(), type_vars_chirho);
                    let data_type_name_chirho = name_chirho.text_chirho().to_string();
                    for con_chirho in constructors_chirho {
                        self.data_constructors_chirho
                            .entry(data_type_name_chirho.clone())
                            .or_default()
                            .push(
                                records_chirho::con_decl_name_chirho(con_chirho)
                                    .text_chirho()
                                    .to_string(),
                            );
                    }
                    for con_chirho in constructors_chirho {
                        if self.bind_written_h98_constructor_chirho(
                            con_chirho,
                            (name_chirho, type_vars_chirho),
                            &tv_map_chirho,
                            type_constructor_scope_chirho.as_deref(),
                            field_context_chirho,
                            &mut projection_budget_chirho,
                        ) {
                            continue;
                        }
                        match con_chirho {
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                                name_chirho,
                                fields_chirho,
                                ..
                            } => {
                                self.bind_constructor_field_names_chirho(
                                    field_context_chirho,
                                    name_chirho.text_chirho(),
                                    Vec::new(),
                                );
                                if fields_chirho.iter().any(|(strictness_chirho, _ty_chirho)| {
                                    strictness_chirho.is_strict_chirho()
                                }) {
                                    self.strict_positional_constructors_chirho
                                        .insert(name_chirho.text_chirho().to_string());
                                }
                                let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                    .iter()
                                    .map(|(_s_chirho, ty_chirho)| {
                                        self.ast_type_to_ty_chirho(ty_chirho, &mut tv_map_chirho)
                                    })
                                    .collect();
                                let con_ty_chirho = TyChirho::fun_n_chirho(
                                    field_tys_chirho,
                                    result_ty_chirho.clone(),
                                );
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                            }
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                                name_chirho,
                                fields_chirho,
                                ..
                            } => {
                                let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                    .iter()
                                    .flat_map(|fd_chirho| {
                                        let ty_chirho = self.ast_type_to_ty_chirho(
                                            &fd_chirho.ty_chirho,
                                            &mut tv_map_chirho,
                                        );
                                        std::iter::repeat_n(ty_chirho, fd_chirho.names_chirho.len())
                                    })
                                    .collect();
                                let con_ty_chirho = TyChirho::fun_n_chirho(
                                    field_tys_chirho.clone(),
                                    result_ty_chirho.clone(),
                                );
                                let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                                self.env_chirho.bind_chirho(
                                    name_chirho.text_chirho().to_string(),
                                    gen_scheme_chirho,
                                );
                                // Store field names for RecordWildCards expansion
                                let field_names_chirho: Vec<String> = fields_chirho
                                    .iter()
                                    .flat_map(|fd_chirho| {
                                        fd_chirho
                                            .names_chirho
                                            .iter()
                                            .map(|n_chirho| n_chirho.text_chirho().to_string())
                                    })
                                    .collect();
                                self.bind_constructor_field_names_chirho(
                                    field_context_chirho,
                                    name_chirho.text_chirho(),
                                    field_names_chirho,
                                );
                                self.con_strict_fields_chirho.insert(
                                    name_chirho.text_chirho().to_string(),
                                    records_chirho::strict_field_names_chirho(fields_chirho),
                                );
                                // Bind field accessor functions: fieldName :: T -> FieldType
                                // Use a running index into field_tys_chirho (which is
                                // flattened by field names, not by field declarations).
                                let mut ft_idx_chirho = 0;
                                for fd_chirho in fields_chirho {
                                    for fname_chirho in &fd_chirho.names_chirho {
                                        if ft_idx_chirho < field_tys_chirho.len() {
                                            let accessor_ty_chirho = TyChirho::FunChirho(
                                                Box::new(result_ty_chirho.clone()),
                                                Box::new(field_tys_chirho[ft_idx_chirho].clone()),
                                                MultChirho::ManyChirho,
                                            );
                                            let accessor_scheme_chirho =
                                                self.generalize_chirho(&accessor_ty_chirho);
                                            self.env_chirho.bind_chirho(
                                                fname_chirho.text_chirho().to_string(),
                                                accessor_scheme_chirho,
                                            );
                                        }
                                        ft_idx_chirho += 1;
                                    }
                                }
                            }
                            haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                                name_chirho,
                                signature_chirho,
                                ..
                            } => {
                                self.bind_gadt_constructor_chirho(
                                    name_chirho,
                                    signature_chirho,
                                    &tv_map_chirho,
                                    type_constructor_scope_chirho.as_deref(),
                                    field_context_chirho,
                                    &mut projection_budget_chirho,
                                );
                            }
                        }
                    }
                }
                DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    constructor_chirho,
                    ..
                } => {
                    self.data_constructors_chirho
                        .entry(name_chirho.text_chirho().to_string())
                        .or_default()
                        .push(
                            records_chirho::con_decl_name_chirho(constructor_chirho)
                                .text_chirho()
                                .to_string(),
                        );
                    let field_context_chirho = LocalConstructorFieldContextChirho {
                        module_name_chirho: &module_name_chirho,
                        owner_declaration_chirho: type_constructor_scope_chirho.as_ref().and_then(
                            |scope_chirho| {
                                scope_chirho.declaration_chirho(&name_chirho.full_name_chirho())
                            },
                        ),
                    };
                    let (result_ty_chirho, mut nt_tv_map_chirho) =
                        self.data_head_type_chirho(name_chirho.text_chirho(), type_vars_chirho);
                    if self.bind_written_h98_constructor_chirho(
                        constructor_chirho,
                        (name_chirho, type_vars_chirho),
                        &nt_tv_map_chirho,
                        type_constructor_scope_chirho.as_deref(),
                        field_context_chirho,
                        &mut projection_budget_chirho,
                    ) {
                        continue;
                    }
                    match constructor_chirho.as_ref() {
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                            name_chirho: con_name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            self.bind_constructor_field_names_chirho(
                                field_context_chirho,
                                con_name_chirho.text_chirho(),
                                Vec::new(),
                            );
                            // Newtype constructor: field type → result type
                            let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                .iter()
                                .map(|(_s_chirho, ty_chirho)| {
                                    self.ast_type_to_ty_chirho(ty_chirho, &mut nt_tv_map_chirho)
                                })
                                .collect();
                            let con_ty_chirho =
                                TyChirho::fun_n_chirho(field_tys_chirho, result_ty_chirho);
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::RecordChirho {
                            name_chirho: con_name_chirho,
                            fields_chirho,
                            ..
                        } => {
                            // Store field names for RecordWildCards expansion
                            let field_names_chirho: Vec<String> = fields_chirho
                                .iter()
                                .flat_map(|fd_chirho| {
                                    fd_chirho
                                        .names_chirho
                                        .iter()
                                        .map(|n_chirho| n_chirho.text_chirho().to_string())
                                })
                                .collect();
                            self.bind_constructor_field_names_chirho(
                                field_context_chirho,
                                con_name_chirho.text_chirho(),
                                field_names_chirho,
                            );
                            self.con_strict_fields_chirho.insert(
                                con_name_chirho.text_chirho().to_string(),
                                records_chirho::strict_field_names_chirho(fields_chirho),
                            );
                            let field_tys_chirho: Vec<TyChirho> = fields_chirho
                                .iter()
                                .flat_map(|fd_chirho| {
                                    let ty_chirho = self.ast_type_to_ty_chirho(
                                        &fd_chirho.ty_chirho,
                                        &mut nt_tv_map_chirho,
                                    );
                                    std::iter::repeat_n(ty_chirho, fd_chirho.names_chirho.len())
                                })
                                .collect();
                            let con_ty_chirho = TyChirho::fun_n_chirho(
                                field_tys_chirho.clone(),
                                result_ty_chirho.clone(),
                            );
                            let gen_scheme_chirho = self.generalize_chirho(&con_ty_chirho);
                            self.env_chirho.bind_chirho(
                                con_name_chirho.text_chirho().to_string(),
                                gen_scheme_chirho,
                            );
                            // Bind field accessor functions: fieldName :: T -> FieldType
                            let mut nt_ft_idx_chirho = 0;
                            for fd_chirho in fields_chirho {
                                for fname_chirho in &fd_chirho.names_chirho {
                                    if nt_ft_idx_chirho < field_tys_chirho.len() {
                                        let accessor_ty_chirho = TyChirho::FunChirho(
                                            Box::new(result_ty_chirho.clone()),
                                            Box::new(field_tys_chirho[nt_ft_idx_chirho].clone()),
                                            MultChirho::ManyChirho,
                                        );
                                        let accessor_scheme_chirho =
                                            self.generalize_chirho(&accessor_ty_chirho);
                                        self.env_chirho.bind_chirho(
                                            fname_chirho.text_chirho().to_string(),
                                            accessor_scheme_chirho,
                                        );
                                    }
                                    nt_ft_idx_chirho += 1;
                                }
                            }
                        }
                        haskelujah_ast_chirho::decl_chirho::ConDeclChirho::GadtChirho {
                            name_chirho: con_name_chirho,
                            signature_chirho,
                            ..
                        } => {
                            self.bind_gadt_constructor_chirho(
                                con_name_chirho,
                                signature_chirho,
                                &nt_tv_map_chirho,
                                type_constructor_scope_chirho.as_deref(),
                                field_context_chirho,
                                &mut projection_budget_chirho,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        // Phase 3: Infer function bindings using SCC-based binding groups
        // for proper let-polymorphism (forward references, mutual recursion).
        //
        // Phase 3a: Pre-bind ALL top-level function names with fresh type
        // variables so every function can reference every other function.
        // Phase 3b: Process groups in SCC topological order so that leaf
        // functions get generalized first, and functions that depend on
        // them see their polymorphic types.
        // 3a: Collect FunBindChirho declarations and pre-bind ALL of them
        let mut fun_names_chirho: Vec<String> = Vec::new();
        let mut fun_matches_refs_chirho: Vec<&[MatchArmChirho]> = Vec::new();
        let mut fun_spans_chirho: Vec<SpanChirho> = Vec::new();
        let mut fun_pre_tys_chirho: Vec<TyChirho> = Vec::new();

        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::FunBindChirho {
                name_chirho,
                matches_chirho,
                span_chirho,
                ..
            } = decl_chirho
            {
                let binding_name_chirho =
                    canonical_value_name_text_chirho(name_chirho.text_chirho());
                let pre_ty_chirho =
                    if let Some(sig_ast_chirho) = type_sigs_chirho.get(&binding_name_chirho) {
                        let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                        let (sig_ty_chirho, _sig_preds_chirho) =
                            self.instantiate_scheme_parts_chirho(&sig_scheme_chirho);
                        self.env_chirho
                            .bind_chirho(binding_name_chirho.clone(), sig_scheme_chirho);
                        sig_ty_chirho
                    } else {
                        let fresh_ty_chirho = self.fresh_value_meta_chirho(*span_chirho);
                        self.env_chirho.bind_chirho(
                            binding_name_chirho.clone(),
                            SchemeChirho::mono_chirho(fresh_ty_chirho.clone()),
                        );
                        self.recursive_references_chirho
                            .assumptions_chirho
                            .insert(binding_name_chirho.clone(), fresh_ty_chirho.clone());
                        fresh_ty_chirho
                    };
                fun_names_chirho.push(binding_name_chirho);
                fun_matches_refs_chirho.push(matches_chirho.as_slice());
                fun_spans_chirho.push(*span_chirho);
                fun_pre_tys_chirho.push(pre_ty_chirho);
            }
        }

        // Also pre-bind PatBindChirho variables. When a type signature
        // exists for a pattern-bound variable, use the signature scheme
        // for proper polymorphism (e.g. `id1 :: a -> a; (id1) = id`).
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::PatBindChirho {
                pat_chirho,
                span_chirho: _,
                ..
            } = decl_chirho
            {
                let names_chirho = crate::linearity_chirho::pat_bound_names_chirho(pat_chirho);
                for name_chirho in names_chirho {
                    if let Some(sig_ast_chirho) = type_sigs_chirho.get(&name_chirho) {
                        let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                        self.env_chirho
                            .bind_chirho(name_chirho.clone(), sig_scheme_chirho);
                    } else {
                        let pre_ty_chirho = self.fresh_var_chirho();
                        self.env_chirho.bind_chirho(
                            name_chirho.clone(),
                            SchemeChirho::mono_chirho(pre_ty_chirho),
                        );
                    }
                }
            }
        }

        // Compute SCC binding groups (topological order: leaves first)
        let fun_decls_for_scc_chirho: Vec<(usize, &[MatchArmChirho])> = fun_matches_refs_chirho
            .iter()
            .enumerate()
            .map(|(i_chirho, m_chirho)| (i_chirho, *m_chirho))
            .collect();
        let signed_names_chirho: HashSet<String> = if self.relaxed_poly_rec_chirho {
            type_sigs_chirho.keys().cloned().collect()
        } else {
            HashSet::new()
        };
        let groups_chirho = binding_groups_chirho(
            &fun_names_chirho,
            &fun_decls_for_scc_chirho,
            &signed_names_chirho,
        );

        // 3b: Process each SCC group: infer → generalize
        // The rigid form of each checked signature, for the Phase 3c check.
        let mut skolemized_sigs_chirho: HashMap<String, TyChirho> = HashMap::new();
        // The monomorphism restriction reads this module's facts once (row 692).
        // workflow: language-features-chirho/fundep-generalization-chirho
        self.restriction_chirho.begin_module_chirho(module_chirho);
        for group_chirho in &groups_chirho {
            // Rule 1, decided once for the whole group: one unsigned simple pattern binding
            // restricts every unsigned member, function syntax included.
            let group_restricted_chirho = self.restriction_chirho.enabled_chirho()
                && group_is_restricted_chirho(group_chirho.iter().map(|&member_chirho| {
                    (
                        fun_matches_refs_chirho[member_chirho]
                            .first()
                            .is_none_or(|arm_chirho| arm_chirho.pats_chirho.is_empty()),
                        type_sigs_chirho.contains_key(&fun_names_chirho[member_chirho]),
                    )
                }));
            // Every variable this group's inference creates is at or above this mark.
            let group_first_variable_chirho = self.group_variable_mark_chirho();
            let mut equality_owners_chirho = Vec::new();
            // Infer each function body in the group
            let mut group_inferred_chirho: Vec<(usize, String, TyChirho, SpanChirho)> = Vec::new();
            for &fi_chirho in group_chirho {
                let binding_name_chirho = &fun_names_chirho[fi_chirho];
                let pre_ty_chirho = &fun_pre_tys_chirho[fi_chirho];
                let span_chirho = fun_spans_chirho[fi_chirho];
                let equality_owner_chirho = self.begin_equality_scope_chirho(
                    if type_sigs_chirho.contains_key(binding_name_chirho) {
                        super::equality_scopes_chirho::EqualityScopeKindChirho::CheckedChirho
                    } else {
                        super::equality_scopes_chirho::EqualityScopeKindChirho::BindingChirho
                    },
                );
                equality_owners_chirho.push(equality_owner_chirho);

                // ScopedTypeVariables: an explicit `forall` scopes the
                // signature's own binders (the same variables the rigid seed
                // was built from) over the body; restored afterwards.
                let prev_scoped_chirho = self.scoped_tyvars_chirho.clone();

                let (mut s_chirho, inferred_ty_chirho) =
                    if let Some(sig_ast_chirho) = type_sigs_chirho.get(binding_name_chirho) {
                        let (sig_scheme_chirho, sig_var_map_chirho) =
                            self.ast_type_to_scheme_with_var_map_chirho(sig_ast_chirho);
                        let (sig_ty_raw_chirho, sig_preds_chirho) =
                            self.skolemize_scheme_parts_chirho(&sig_scheme_chirho);
                        self.record_own_dictionary_indices_chirho(&sig_preds_chirho);
                        self.scope_signature_for_body_chirho(sig_ast_chirho, &sig_var_map_chirho);
                        skolemized_sigs_chirho
                            .insert(binding_name_chirho.clone(), sig_ty_raw_chirho.clone());
                        self.with_signature_givens_chirho(
                            sig_preds_chirho,
                            &sig_ty_raw_chirho,
                            |self_chirho| {
                                self_chirho.infer_matches_against_expected_chirho(
                                    fun_matches_refs_chirho[fi_chirho],
                                    span_chirho,
                                    &sig_ty_raw_chirho,
                                )
                            },
                        )
                    } else {
                        self.infer_matches_chirho(fun_matches_refs_chirho[fi_chirho], span_chirho)
                    };
                if self.apply_subst_all_chirho(&mut s_chirho).is_err() {
                    return SubstChirho::empty_chirho();
                };
                subst_chirho = s_chirho.compose_chirho(&subst_chirho);

                // Unsigned recursive placeholders join their inferred result.
                // A signed binding already uses its full declared scheme and
                // was checked under that SAME signature's givens. An unrelated
                // fresh pretype supplies no additional family equality.
                if !type_sigs_chirho.contains_key(binding_name_chirho) {
                    let pre_sub_chirho = subst_chirho.apply_ty_chirho(pre_ty_chirho);
                    match self.unify_normalized_chirho(
                        &pre_sub_chirho,
                        &inferred_ty_chirho,
                        span_chirho,
                    ) {
                        Ok(mut su_chirho) => {
                            if self.apply_subst_all_chirho(&mut su_chirho).is_err() {
                                return SubstChirho::empty_chirho();
                            };
                            subst_chirho = su_chirho.compose_chirho(&subst_chirho);
                        }
                        Err(err_chirho) => {
                            self.report_unify_error_chirho(&err_chirho);
                        }
                    }
                }

                group_inferred_chirho.push((
                    fi_chirho,
                    binding_name_chirho.clone(),
                    inferred_ty_chirho,
                    span_chirho,
                ));

                // Restore scoped type variables
                self.scoped_tyvars_chirho = prev_scoped_chirho;
                self.end_equality_scope_chirho(equality_owner_chirho);
            }

            let inferred_equality_types_chirho = group_inferred_chirho
                .iter()
                .filter(|(_, name_chirho, _, _)| !type_sigs_chirho.contains_key(name_chirho))
                .map(|(_, _, ty_chirho, _)| subst_chirho.apply_ty_chirho(ty_chirho))
                .collect::<Vec<_>>();
            let previous_generalizing_chirho = std::mem::replace(
                &mut self.equality_generalizing_owners_chirho,
                equality_owners_chirho.iter().copied().collect(),
            );
            // Generalize all functions in the group together
            for (
                (fi_chirho, binding_name_chirho, inferred_ty_chirho, span_chirho),
                equality_owner_chirho,
            ) in group_inferred_chirho.iter().zip(&equality_owners_chirho)
            {
                let previous_owner_chirho = self.active_equality_scope_chirho;
                self.active_equality_scope_chirho = *equality_owner_chirho;
                self.env_chirho.remove_chirho(binding_name_chirho);

                let final_ty_chirho = subst_chirho.apply_ty_chirho(inferred_ty_chirho);
                // IO defaulting of a lone Monad/Functor-constrained variable is a
                // `main`-style convenience for argument-less bindings (where the
                // monomorphism restriction would block generalization anyway); a
                // function binding keeps its polymorphism.
                let argument_less_chirho = fun_matches_refs_chirho[*fi_chirho]
                    .first()
                    .is_none_or(|arm_chirho| arm_chirho.pats_chirho.is_empty());
                let inferred_scheme_chirho = self.generalize_group_member_chirho(
                    &final_ty_chirho,
                    argument_less_chirho,
                    GeneralizationScopeChirho::TopLevelChirho,
                    group_restricted_chirho && !type_sigs_chirho.contains_key(binding_name_chirho),
                    &mut subst_chirho,
                );
                let mut binding_scheme_chirho = inferred_scheme_chirho.clone();

                // Phase 3c: Check against type signature if one exists
                if let Some(sig_ast_chirho) = type_sigs_chirho.get(binding_name_chirho) {
                    let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                    // Build the full sig type preserving forall structure
                    let sig_full_chirho = if sig_scheme_chirho.vars_chirho.is_empty() {
                        sig_scheme_chirho.ty_chirho.clone()
                    } else {
                        TyChirho::ForallChirho {
                            vars_chirho: sig_scheme_chirho.vars_chirho.clone(),
                            body_chirho: Box::new(sig_scheme_chirho.ty_chirho.clone()),
                        }
                    };
                    // Check against the SAME rigid signature the body was
                    // checked with: a signature variable the body forced to a
                    // concrete type is a mismatch, not an instantiation.
                    // workflow: language-features-chirho/rigid-type-variables-chirho
                    let sig_ty_chirho = match skolemized_sigs_chirho.get(binding_name_chirho) {
                        Some(skolemized_chirho) => self
                            .normalize_ty_chirho(&subst_chirho.apply_ty_chirho(skolemized_chirho)),
                        None => self.normalize_ty_chirho(&sig_full_chirho),
                    };
                    let inferred_sub_chirho = subst_chirho.apply_ty_chirho(inferred_ty_chirho);
                    let inferred_sub_chirho = self.normalize_ty_chirho(&inferred_sub_chirho);
                    // Use subsumption checking to handle higher-rank types
                    match self.subsume_normalized_chirho(
                        &inferred_sub_chirho,
                        &sig_ty_chirho,
                        *span_chirho,
                    ) {
                        Ok(mut sig_s_chirho) => {
                            if self.apply_subst_all_chirho(&mut sig_s_chirho).is_err() {
                                return SubstChirho::empty_chirho();
                            };
                            subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                            binding_scheme_chirho = sig_scheme_chirho;
                        }
                        Err(_err_chirho) => {
                            self.diagnostics_chirho.push_chirho(
                                DiagnosticChirho::error_with_code_chirho(
                                    ErrorCodeChirho::error_chirho(SIGNATURE_MISMATCH_CODE_CHIRHO),
                                    format!(
                                        "type signature mismatch for `{binding_name_chirho}`: \
                                         inferred `{inferred_sub_chirho}`, \
                                         declared `{sig_ty_chirho}`"
                                    ),
                                    *span_chirho,
                                ),
                            );
                        }
                    }
                }

                self.finish_recursive_references_chirho(
                    binding_name_chirho,
                    &binding_scheme_chirho,
                );
                self.env_chirho
                    .bind_chirho(binding_name_chirho.clone(), binding_scheme_chirho);
                self.active_equality_scope_chirho = previous_owner_chirho;
            }
            self.equality_generalizing_owners_chirho = previous_generalizing_chirho;
            self.finish_equality_binding_group_chirho(
                &equality_owners_chirho,
                &inferred_equality_types_chirho,
            );
            // Rule 1 reaches every wanted the restricted group left behind, not only those
            // its result type mentions (row 692).
            if group_restricted_chirho {
                self.withhold_group_residuals_chirho(group_first_variable_chirho);
            }
        }

        // Phase 3d: Handle top-level pattern bindings
        for decl_chirho in &module_chirho.decls_chirho {
            if let DeclChirho::PatBindChirho {
                pat_chirho,
                rhs_chirho,
                span_chirho: _,
            } = decl_chirho
            {
                if matches!(rhs_chirho, RhsChirho::GuardedChirho(arms_chirho) if arms_chirho.is_empty())
                {
                    continue;
                }
                // The WHOLE right-hand side: every arm's qualifiers and body,
                // not the first arm's body alone.
                let (mut s1_chirho, rhs_ty_chirho) = self.infer_rhs_chirho(rhs_chirho);
                if self.apply_subst_all_chirho(&mut s1_chirho).is_err() {
                    return SubstChirho::empty_chirho();
                };
                subst_chirho = s1_chirho.compose_chirho(&subst_chirho);

                let mut sp_chirho = self.bind_binding_pat_chirho(pat_chirho, &rhs_ty_chirho);
                if self.apply_subst_all_chirho(&mut sp_chirho).is_err() {
                    return SubstChirho::empty_chirho();
                };
                subst_chirho = sp_chirho.compose_chirho(&subst_chirho);

                let names_chirho = crate::linearity_chirho::pat_bound_names_chirho(pat_chirho);
                // The whole pattern binding is generalized together: every
                // bound name leaves the environment before any of them is
                // generalized or checked, so a sibling's monomorphic type does
                // not pin the shared variables.
                let resolved_types_chirho: Vec<(String, TyChirho)> = names_chirho
                    .iter()
                    .filter_map(|name_chirho| {
                        self.env_chirho
                            .lookup_chirho(name_chirho)
                            .map(|scheme_chirho| {
                                (
                                    name_chirho.clone(),
                                    subst_chirho.apply_ty_chirho(&scheme_chirho.ty_chirho),
                                )
                            })
                    })
                    .collect();
                for (name_chirho, _resolved_chirho) in &resolved_types_chirho {
                    self.env_chirho.remove_chirho(name_chirho);
                }
                let env_fvs_chirho = self.env_chirho.free_vars_chirho();
                for (name_chirho, resolved_chirho) in resolved_types_chirho {
                    let name_chirho = &name_chirho;
                    {
                        // Check against type signature if one exists
                        if let Some(sig_ast_chirho) = type_sigs_chirho.get(name_chirho) {
                            let sig_scheme_chirho = self.ast_type_to_scheme_chirho(sig_ast_chirho);
                            let (sig_ty_raw_chirho, sig_preds_chirho) =
                                self.skolemize_scheme_parts_chirho(&sig_scheme_chirho);
                            self.record_own_dictionary_indices_chirho(&sig_preds_chirho);
                            let sig_ty_chirho = self.normalize_ty_chirho(&sig_ty_raw_chirho);
                            // Generalize the monomorphic pattern type over the
                            // variables not free in the environment, instantiate
                            // that afresh, and check it against the rigid
                            // signature: sibling pattern variables share
                            // variables, and each signature is checked on its own.
                            let mut instantiation_chirho = SubstChirho::empty_chirho();
                            for var_chirho in resolved_chirho.free_vars_chirho() {
                                if !env_fvs_chirho.contains(&var_chirho) {
                                    let fresh_chirho = self.fresh_var_chirho();
                                    instantiation_chirho.insert_chirho(var_chirho, fresh_chirho);
                                }
                            }
                            let inferred_inst_chirho =
                                instantiation_chirho.apply_ty_chirho(&resolved_chirho);
                            let inferred_sub_chirho =
                                self.normalize_ty_chirho(&inferred_inst_chirho);
                            // The signature's context is available as givens
                            // while the RHS's wanteds are discharged.
                            let sig_subst_opt_chirho =
                                self.with_given_preds_chirho(sig_preds_chirho, |self_chirho| {
                                    match self_chirho.subsume_normalized_chirho(
                                        &inferred_sub_chirho,
                                        &sig_ty_chirho,
                                        SpanChirho::DUMMY_CHIRHO,
                                    ) {
                                        Ok(mut sig_s_chirho) => {
                                            if self_chirho
                                                .apply_subst_all_chirho(&mut sig_s_chirho)
                                                .is_err()
                                            {
                                                return None;
                                            };
                                            Some(sig_s_chirho)
                                        }
                                        Err(_err_chirho) => {
                                            self_chirho.diagnostics_chirho.push_chirho(
                                            DiagnosticChirho::error_with_code_chirho(
                                                ErrorCodeChirho::error_chirho(
                                                    SIGNATURE_MISMATCH_CODE_CHIRHO,
                                                ),
                                                format!(
                                                    "type signature mismatch for `{name_chirho}`: \
                                                         inferred `{inferred_sub_chirho}`, \
                                                         declared `{sig_ty_chirho}`"
                                                ),
                                                SpanChirho::DUMMY_CHIRHO,
                                            ),
                                        );
                                            None
                                        }
                                    }
                                });
                            if let Some(sig_s_chirho) = sig_subst_opt_chirho {
                                subst_chirho = sig_s_chirho.compose_chirho(&subst_chirho);
                            }
                            // Use the signature scheme directly for proper
                            // polymorphism (e.g. `id1 :: a -> a; (id1) = id`)
                            self.env_chirho
                                .bind_chirho(name_chirho.clone(), sig_scheme_chirho);
                        } else {
                            let gen_chirho = self.generalize_chirho(&resolved_chirho);
                            self.env_chirho.bind_chirho(name_chirho.clone(), gen_chirho);
                        }
                    }
                }
            }
        }

        // Phase 3e: Check instance method implementations against their
        // class method types specialized to the instance head. This gives
        // instance bodies the same expected-type guidance that top-level
        // signature-annotated wrappers already receive.
        self.check_instance_methods_against_class_chirho(module_chirho);
        self.check_class_default_methods_chirho(module_chirho);

        // Deferred equalities whose stuck side has been solved by now are
        // decided here; one still stuck against a closed type is reported,
        // one open on both sides is left alone.
        let equalities_chirho = self.retry_deferred_equalities_chirho();
        self.report_unsolved_deferred_equalities_chirho();
        if equalities_chirho.is_empty_chirho() {
            subst_chirho
        } else {
            equalities_chirho.compose_chirho(&subst_chirho)
        }
    }
}
