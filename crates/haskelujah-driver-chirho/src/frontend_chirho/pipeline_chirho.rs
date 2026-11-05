// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The frontend pipeline, moved from the driver root (row 691, L6): the phases one module runs
//! through against given interfaces, imported types, synonyms, families and class environment,
//! after an implicit `import Prelude` is injected unless the module opts out.

use super::*;

/// What one module is checked against: the interfaces name resolution reads and the
/// environment imported with them.
pub struct FrontendImportsChirho<'imports_chirho> {
    pub ifaces_chirho: &'imports_chirho [ModuleIfaceChirho],
    pub types_chirho:
        &'imports_chirho std::collections::HashMap<String, haskelujah_typing_chirho::SchemeChirho>,
    pub type_synonyms_chirho: &'imports_chirho ImportedTypeSynonymsChirho,
    pub type_families_chirho: &'imports_chirho ImportedTypeFamiliesChirho,
    pub class_env_chirho: &'imports_chirho haskelujah_typing_chirho::class_chirho::ClassEnvChirho,
}

/// The frontend with no caller options: only the module header's own `OPTIONS_GHC` pragmas
/// can defer an error.
pub fn run_frontend_with_type_synonyms_families_and_class_env_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
    imported_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    >,
    imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
    imported_type_families_chirho: &ImportedTypeFamiliesChirho,
    imported_class_env_chirho: &haskelujah_typing_chirho::class_chirho::ClassEnvChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    run_frontend_with_options_chirho(
        source_chirho,
        file_id_chirho,
        FrontendImportsChirho {
            ifaces_chirho,
            types_chirho: imported_types_chirho,
            type_synonyms_chirho: imported_type_synonyms_chirho,
            type_families_chirho: imported_type_families_chirho,
            class_env_chirho: imported_class_env_chirho,
        },
        &FrontendOptionsChirho::default(),
    )
}

/// The frontend phases under the caller's options (row 695). The caller's options apply
/// first and the `OPTIONS_GHC` pragmas in the header of `source_chirho` after them. Together
/// they decide which inference errors defer; every other phase's errors stop the module.
/// workflow: compiler-pipeline-chirho/error-deferral-chirho
pub fn run_frontend_with_options_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    imports_chirho: FrontendImportsChirho<'_>,
    options_chirho: &FrontendOptionsChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let deferral_chirho = DeferralPolicyChirho::resolve_chirho(
        &options_chirho.ghc_options_chirho,
        &haskelujah_parser_chirho::pragma_chirho::header_ghc_options_chirho(
            source_chirho,
            file_id_chirho,
        ),
    );
    run_frontend_under_deferral_chirho(
        source_chirho,
        file_id_chirho,
        imports_chirho,
        deferral_chirho,
    )
}

/// The frontend phases under a deferral policy the caller already resolved: the source path
/// reads the module's header before CPP rewrites the text (row 695).
/// workflow: compiler-pipeline-chirho/error-deferral-chirho
pub(crate) fn run_frontend_under_deferral_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    imports_chirho: FrontendImportsChirho<'_>,
    deferral_chirho: DeferralPolicyChirho,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let FrontendImportsChirho {
        ifaces_chirho,
        types_chirho: imported_types_chirho,
        type_synonyms_chirho: imported_type_synonyms_chirho,
        type_families_chirho: imported_type_families_chirho,
        class_env_chirho: imported_class_env_chirho,
    } = imports_chirho;

    // Phase 1: CST parse (lex + layout + recursive-descent)
    let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
    let green_chirho = parser_chirho.parse_chirho();

    // Phase 2: CST → AST lowering
    let mut module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

    // Phase 2.1: Automatic Prelude import
    // Every Haskell module implicitly imports Prelude unless:
    //   - {-# LANGUAGE NoImplicitPrelude #-} is present
    //   - The module already has an explicit `import Prelude`
    inject_prelude_import_chirho(&mut module_chirho);

    // Phase 2.2: Template Haskell splice expansion
    // Process SpliceDeclChirho entries before name resolution so that
    // generated declarations participate in the normal compilation pipeline.
    let splice_result_chirho = splice_chirho::expand_splices_chirho(
        std::mem::take(&mut module_chirho.decls_chirho),
        &mut module_chirho.origin_supply_chirho,
    );
    module_chirho.decls_chirho = splice_result_chirho.decls_chirho;
    // The declarations written in source: deriving (Phase 2.5) appends its instances
    // after them, and the kind pass judges only written builtin instance heads (K01).
    // workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
    let written_decl_count_chirho = module_chirho.decls_chirho.len();
    let splice_warnings_chirho = splice_result_chirho.warnings_chirho;

    // The builtin classes the module's class spellings name, proven from the same
    // interfaces resolution reads (K01). Declaration validity and deriving judge which stock
    // class a derived class names by them, and deriving reads their parameter kinds to
    // eta-reduce a GND head (row 694); the kind pass binds them. Deriving appends only
    // instances, which declare no type name, so the scope is the same before and after it.
    // workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
    let builtin_class_scope_chirho =
        haskelujah_naming_chirho::build_builtin_class_scope_chirho(&module_chirho, ifaces_chirho);

    // Phase 2.4: Declaration validity (row 683), judged on the module as written,
    // before deriving appends generated instances. Reported after name resolution.
    // workflow: compiler-pipeline-chirho/declaration-validity-chirho
    let declaration_validity_chirho =
        haskelujah_typing_chirho::check_module_declaration_validity_with_builtins_diagnostics_chirho(
            &module_chirho,
            &builtin_class_scope_chirho,
        );

    // Phase 2.5: Deriving — generate instance declarations for `deriving` clauses
    let deriving_warnings_chirho =
        haskelujah_typing_chirho::deriving_chirho::apply_deriving_with_builtins_chirho(
            &mut module_chirho,
            &builtin_class_scope_chirho,
        );

    // Phase 2.6: Do-statement failability — clear the `fail` lowering RESERVED on
    // every bind wherever the pattern cannot fail, so what survives is a
    // selection that was really selected. Runs here because the module's own
    // declarations are final after splices and deriving, and it needs to know how
    // many constructors a pattern's type has. A constructor from another module
    // is not in this environment and stays failable, which is the safe direction.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    haskelujah_typing_chirho::do_failability_chirho::refine_do_selections_chirho(
        &mut module_chirho,
    );

    // Phase 3: Name resolution. Its errors never defer: GHC's out-of-scope switch reaches only
    // variables, and resolution's one out-of-scope value code also covers data constructors.
    // workflow: compiler-pipeline-chirho/error-deferral-chirho
    let resolve_result_chirho = resolve_module_with_imports_chirho(&module_chirho, ifaces_chirho);
    if resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(resolve_result_chirho.diagnostics_chirho);
    }
    // GHC never defers a declaration-shape error, so the defer flags do not apply.
    if declaration_validity_chirho
        .diagnostics_chirho
        .has_errors_chirho()
    {
        return Err(declaration_validity_chirho.diagnostics_chirho);
    }

    // Phase 3.1: Orphan instance detection
    let orphan_warnings_chirho =
        haskelujah_naming_chirho::check_orphan_instances_chirho(&module_chirho);

    // Phase 3.5: Kind inference, knowing the builtin classes the module's class spellings
    // name (the scope built before deriving).
    // workflow: compiler-pipeline-chirho/builtin-class-identity-chirho
    let kind_result_chirho = haskelujah_typing_chirho::infer_module_kinds_with_imports_chirho(
        &module_chirho,
        haskelujah_typing_chirho::KindImportsChirho {
            builtin_classes_chirho: &builtin_class_scope_chirho,
            written_decl_count_chirho,
        },
    );
    // A kind error is fatal under every deferral switch.
    if kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(kind_result_chirho.diagnostics_chirho);
    }

    // Phase 3.9: Seed the type environment with placeholder types for
    // imported values that don't have explicit type schemes. This allows
    // the type checker to see imported names as polymorphic variables
    // rather than reporting them as "unbound variable" (E0202).
    let mut merged_imported_types_chirho = imported_types_chirho.clone();
    let mut merged_imported_type_synonyms_chirho = imported_type_synonyms_chirho.clone();
    let merged_imported_type_families_chirho = imported_type_families_chirho.clone();
    let mut merged_imported_record_field_names_chirho: std::collections::HashMap<
        String,
        Vec<String>,
    > = std::collections::HashMap::new();
    let builtin_value_schemes_chirho =
        haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho();
    for (builtin_name_chirho, builtin_scheme_chirho) in &builtin_value_schemes_chirho {
        merged_imported_types_chirho
            .entry(builtin_name_chirho.clone())
            .or_insert_with(|| builtin_scheme_chirho.clone());
    }
    let safe_unqualified_imported_type_names_chirho =
        collect_safe_unqualified_imported_type_names_chirho(&module_chirho, ifaces_chirho);
    let preferred_qualified_type_names_chirho =
        collect_preferred_qualified_type_names_chirho(&module_chirho, ifaces_chirho);
    for import_chirho in &module_chirho.imports_chirho {
        let module_name_chirho = import_chirho.module_chirho.full_name_chirho();
        if let Some(iface_chirho) = ifaces_chirho
            .iter()
            .rev()
            .find(|m_chirho| m_chirho.name_chirho == module_name_chirho)
        {
            let qualifiable_type_names_chirho: std::collections::HashSet<String> = iface_chirho
                .exports_chirho
                .types_chirho
                .keys()
                .cloned()
                .chain(
                    imported_type_synonyms_chirho
                        .keys()
                        .filter_map(|name_chirho| {
                            name_chirho
                                .strip_prefix(&format!("{module_name_chirho}."))
                                .map(|suffix_chirho| suffix_chirho.to_string())
                        }),
                )
                .collect();
            // Collect which names this import brings in.
            let names_chirho =
                haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                    &iface_chirho.exports_chirho,
                    &import_chirho.spec_chirho,
                );
            let qualifier_chirho = import_chirho
                .alias_chirho
                .as_ref()
                .map(|a_chirho| a_chirho.text_chirho().to_string())
                .unwrap_or_else(|| module_name_chirho.clone());
            if !import_chirho.qualified_chirho {
                for type_info_chirho in iface_chirho.exports_chirho.types_chirho.values() {
                    if type_info_chirho.methods_chirho.is_empty()
                        || type_info_chirho.constructors_chirho.is_empty()
                    {
                        continue;
                    }
                    for constructor_name_chirho in &type_info_chirho.constructors_chirho {
                        merged_imported_record_field_names_chirho
                            .entry(constructor_name_chirho.clone())
                            .or_insert_with(|| type_info_chirho.methods_chirho.clone());
                        merged_imported_record_field_names_chirho
                            .entry(format!(
                                "{}.{}",
                                module_name_chirho, constructor_name_chirho
                            ))
                            .or_insert_with(|| type_info_chirho.methods_chirho.clone());
                        if qualifier_chirho != module_name_chirho {
                            merged_imported_record_field_names_chirho
                                .entry(format!("{}.{}", qualifier_chirho, constructor_name_chirho))
                                .or_insert_with(|| type_info_chirho.methods_chirho.clone());
                        }
                    }
                }
            }
            for (name_chirho, ns_chirho, _span_chirho) in &names_chirho {
                if ns_chirho == &haskelujah_naming_chirho::env_chirho::NamespaceChirho::TypeChirho {
                    let module_qualified_name_chirho =
                        format!("{module_name_chirho}.{name_chirho}");
                    if let Some((params_chirho, rhs_chirho)) = imported_type_synonyms_chirho
                        .get(&module_qualified_name_chirho)
                        .cloned()
                        .or_else(|| imported_type_synonyms_chirho.get(name_chirho).cloned())
                    {
                        let rewritten_rhs_chirho = qualify_imported_ast_type_chirho(
                            &rhs_chirho,
                            &qualifiable_type_names_chirho,
                            &qualifier_chirho,
                            &safe_unqualified_imported_type_names_chirho,
                            &preferred_qualified_type_names_chirho,
                        );
                        if !import_chirho.qualified_chirho {
                            merged_imported_type_synonyms_chirho.insert(
                                name_chirho.clone(),
                                (params_chirho.clone(), rewritten_rhs_chirho.clone()),
                            );
                        }
                        merged_imported_type_synonyms_chirho.insert(
                            format!("{qualifier_chirho}.{name_chirho}"),
                            (params_chirho, rewritten_rhs_chirho),
                        );
                    }
                }
            }
            for (name_chirho, ns_chirho, _span_chirho) in &names_chirho {
                if ns_chirho == &haskelujah_naming_chirho::env_chirho::NamespaceChirho::ValueChirho
                {
                    let module_qualified_name_chirho =
                        format!("{module_name_chirho}.{name_chirho}");
                    let base_scheme_chirho = merged_imported_types_chirho
                        .get(&module_qualified_name_chirho)
                        .cloned()
                        .or_else(|| {
                            if import_chirho.qualified_chirho {
                                // Row 608: a builtin alias takes its bare scheme only where
                                // GHC 9.14.1 types the export exactly so; otherwise the placeholder.
                                // workflow: compiler-pipeline-chirho/builtin-type-environment-chirho
                                qualified_import_alias_scheme_chirho(
                                    iface_chirho,
                                    name_chirho,
                                    &builtin_value_schemes_chirho,
                                )
                            } else {
                                merged_imported_types_chirho.get(name_chirho).cloned()
                            }
                        })
                        .unwrap_or_else(|| {
                            // Assign a fully polymorphic type: forall a. a
                            // This allows the type checker to accept the name
                            // without knowing the precise type.
                            let fresh_var_chirho = haskelujah_typing_chirho::TyVarChirho(
                                9000 + merged_imported_types_chirho.len() as u32,
                            );
                            haskelujah_typing_chirho::SchemeChirho {
                                vars_chirho: vec![fresh_var_chirho],
                                preds_chirho: vec![],
                                ty_chirho: haskelujah_typing_chirho::TyChirho::VarChirho(
                                    fresh_var_chirho,
                                ),
                            }
                        });
                    let in_scope_seed_scheme_chirho = qualify_imported_scheme_for_iface_chirho(
                        &base_scheme_chirho,
                        &qualifiable_type_names_chirho,
                        &qualifier_chirho,
                        &safe_unqualified_imported_type_names_chirho,
                        &preferred_qualified_type_names_chirho,
                    );
                    if !import_chirho.qualified_chirho {
                        // Current-module imports should win over broadly seeded
                        // dependency names. Otherwise a previously compiled
                        // module can pin an unqualified name like `choice` or
                        // `pack` to the wrong specialized scheme in later
                        // modules that explicitly import a different source.
                        merged_imported_types_chirho
                            .insert(name_chirho.clone(), in_scope_seed_scheme_chirho.clone());
                    }
                    let qualified_name_chirho = format!("{qualifier_chirho}.{name_chirho}");
                    if should_override_imported_scheme_chirho(
                        merged_imported_types_chirho.get(&qualified_name_chirho),
                        &in_scope_seed_scheme_chirho,
                    ) {
                        merged_imported_types_chirho
                            .insert(qualified_name_chirho, in_scope_seed_scheme_chirho);
                    }
                }
            }
        }
    }

    // Names spelled bare by their unique exporter keep the synonym key that spelling needs
    // (row 656).
    // workflow: compiler-pipeline-chirho/type-scope-resolution-chirho
    align_preferred_synonym_keys_chirho(
        &mut merged_imported_type_synonyms_chirho,
        &preferred_qualified_type_names_chirho,
    );

    // Phase 4: Type inference — use import-aware variant when upstream
    // type schemes are available, plain variant otherwise.
    let mut infer_result_chirho = if merged_imported_types_chirho.is_empty()
        && merged_imported_type_synonyms_chirho.is_empty()
        && merged_imported_type_families_chirho.is_empty()
        && merged_imported_record_field_names_chirho.is_empty()
        && imported_class_env_chirho.classes_chirho.is_empty()
        && imported_class_env_chirho.instances_chirho.is_empty()
    {
        infer_module_chirho(&module_chirho)
    } else {
        infer_module_with_imports_type_synonyms_families_and_class_env_chirho(
            &module_chirho,
            &merged_imported_types_chirho,
            &merged_imported_type_synonyms_chirho,
            &merged_imported_type_families_chirho,
            imported_class_env_chirho,
            &merged_imported_record_field_names_chirho,
            &safe_unqualified_imported_type_names_chirho,
            &preferred_qualified_type_names_chirho,
        )
    };
    // Only the inference errors the deferral policy defers become warnings. Any other error
    // stops the module, reported with the whole bundle.
    if !deferral_chirho.settle_inference_chirho(&mut infer_result_chirho.diagnostics_chirho) {
        return Err(infer_result_chirho.diagnostics_chirho);
    }

    // Phase 4.1: GND method coercibility (row 694). Every instance the module derives through
    // the newtype strategy needs a typed proof that its class's methods have one
    // representation at the newtype and at its representation, before the dictionary pass may
    // alias a row. A refusal is a declaration error and never defers.
    // workflow: language-features-chirho/deriving-strategies-chirho
    infer_result_chirho.gnd_coercion_proofs_chirho =
        haskelujah_typing_chirho::prove_gnd_coercions_chirho(
            &module_chirho,
            &builtin_class_scope_chirho,
            &infer_result_chirho.class_env_chirho,
        )?;

    // Phase 4.4: Source-type validity — reject type forms whose licensing
    // extension is not enabled (GHC's GHC-91510: illegal polymorphic /
    // qualified type).
    // A validity error is structural and never defers.
    let validity_result_chirho =
        haskelujah_typing_chirho::check_module_type_validity_diagnostics_chirho(&module_chirho);
    if validity_result_chirho
        .diagnostics_chirho
        .has_errors_chirho()
    {
        return Err(validity_result_chirho.diagnostics_chirho);
    }

    // Phase 4.5: Pattern match exhaustiveness and redundancy checking
    let exhaust_result_chirho =
        haskelujah_typing_chirho::check_module_exhaustiveness_chirho(&module_chirho);
    if exhaust_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(exhaust_result_chirho.diagnostics_chirho);
    }

    // Collect non-fatal warnings from exhaustiveness checking.
    let exhaust_warnings_chirho: Vec<String> = exhaust_result_chirho
        .diagnostics_chirho
        .diagnostics_chirho()
        .iter()
        .filter(|d_chirho| !d_chirho.is_error_chirho())
        .map(|d_chirho| d_chirho.to_string())
        .collect();

    // Collect orphan instance warnings as strings.
    let orphan_warning_strs_chirho: Vec<String> = orphan_warnings_chirho
        .diagnostics_chirho()
        .iter()
        .map(|d_chirho| d_chirho.to_string())
        .collect();

    // Phase 4.6: Linearity checking (only when LinearTypes extension is enabled)
    let linearity_warnings_chirho = if module_chirho
        .extensions_chirho
        .iter()
        .any(|e_chirho| e_chirho == "LinearTypes")
    {
        check_module_linearity_chirho(&module_chirho, &infer_result_chirho)
    } else {
        Vec::new()
    };

    // Collect non-fatal warnings from type inference, deferred errors included.
    let infer_warnings_chirho: Vec<String> = infer_result_chirho
        .diagnostics_chirho
        .diagnostics_chirho()
        .iter()
        .filter(|d_chirho| !d_chirho.is_error_chirho())
        .map(|d_chirho| d_chirho.to_string())
        .collect();

    // Merge splice, deriving, exhaustiveness, orphan, linearity, and type inference warnings.
    let mut warnings_chirho = splice_warnings_chirho;
    warnings_chirho.extend(deriving_warnings_chirho);
    warnings_chirho.extend(exhaust_warnings_chirho);
    warnings_chirho.extend(orphan_warning_strs_chirho);
    warnings_chirho.extend(linearity_warnings_chirho);
    warnings_chirho.extend(infer_warnings_chirho);

    Ok(FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        resolved_imported_type_synonyms_chirho: merged_imported_type_synonyms_chirho,
        warnings_chirho,
    })
}

/// Inject an implicit `import Prelude` into a module unless:
/// - `{-# LANGUAGE NoImplicitPrelude #-}` is present, or
/// - the module already has an explicit `import Prelude`.
fn inject_prelude_import_chirho(module_chirho: &mut ModuleChirho) {
    // Check for NoImplicitPrelude extension
    if module_chirho
        .extensions_chirho
        .iter()
        .any(|e_chirho| e_chirho == "NoImplicitPrelude")
    {
        return;
    }

    // Check if Prelude is already explicitly imported
    let has_prelude_import_chirho = module_chirho
        .imports_chirho
        .iter()
        .any(|imp_chirho| imp_chirho.module_chirho.text_chirho() == "Prelude");
    if has_prelude_import_chirho {
        return;
    }

    // Inject implicit Prelude import (unqualified, import everything)
    module_chirho
        .imports_chirho
        .push(haskelujah_ast_chirho::module_chirho::ImportDeclChirho {
            module_chirho: haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
                haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
                    "Prelude",
                    haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
                ),
            ),
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: None,
            span_chirho: haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO,
        });
}
