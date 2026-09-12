// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared frontend phases and explicit imported semantic inputs.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::*;
use haskelujah_typing_chirho::module_contracts_chirho::ModuleTypeContractsChirho;
mod contracts_chirho;
#[cfg(test)]
mod tests_chirho;

/// Modules are partitioned by defining interface; absence is not a guessed contract.
pub type ImportedTypeContractsChirho = std::collections::HashMap<String, ModuleTypeContractsChirho>;

pub struct FrontendInputsChirho<'input_chirho> {
    boot_chirho: bool,
    ifaces_chirho: &'input_chirho [ModuleIfaceChirho],
    imported_types_chirho:
        &'input_chirho std::collections::HashMap<String, haskelujah_typing_chirho::SchemeChirho>,
    imported_type_synonyms_chirho: &'input_chirho ImportedTypeSynonymsChirho,
    imported_type_families_chirho: &'input_chirho ImportedTypeFamiliesChirho,
    type_contracts_chirho: Option<&'input_chirho ImportedTypeContractsChirho>,
    class_env_chirho: Option<&'input_chirho haskelujah_typing_chirho::ClassEnvChirho>,
}

impl<'input_chirho> FrontendInputsChirho<'input_chirho> {
    /// Legacy raw-AST callers carry no checked provider metadata. Source/module
    /// compilation supplies the companion through with_type_contracts_chirho.
    pub fn new_chirho(
        ifaces_chirho: &'input_chirho [ModuleIfaceChirho],
        imported_types_chirho: &'input_chirho std::collections::HashMap<
            String,
            haskelujah_typing_chirho::SchemeChirho,
        >,
        imported_type_synonyms_chirho: &'input_chirho ImportedTypeSynonymsChirho,
        imported_type_families_chirho: &'input_chirho ImportedTypeFamiliesChirho,
    ) -> Self {
        Self {
            boot_chirho: false,
            ifaces_chirho,
            imported_types_chirho,
            imported_type_synonyms_chirho,
            imported_type_families_chirho,
            type_contracts_chirho: None,
            class_env_chirho: None,
        }
    }

    pub fn with_type_contracts_chirho(
        mut self,
        contracts_chirho: &'input_chirho ImportedTypeContractsChirho,
    ) -> Self {
        self.type_contracts_chirho = Some(contracts_chirho);
        self
    }

    pub fn with_class_env_chirho(
        mut self,
        env_chirho: &'input_chirho haskelujah_typing_chirho::ClassEnvChirho,
    ) -> Self {
        self.class_env_chirho = Some(env_chirho);
        self
    }

    pub(crate) fn with_boot_chirho(mut self, boot_chirho: bool) -> Self {
        self.boot_chirho = boot_chirho;
        self
    }
}

pub fn run_frontend_with_inputs_chirho(
    source_chirho: &str,
    file_id_chirho: haskelujah_span_chirho::FileIdChirho,
    inputs_chirho: FrontendInputsChirho<'_>,
) -> Result<FrontendResultChirho, DiagnosticBundleChirho> {
    let ifaces_chirho = inputs_chirho.ifaces_chirho;
    let imported_types_chirho = inputs_chirho.imported_types_chirho;
    let imported_type_synonyms_chirho = inputs_chirho.imported_type_synonyms_chirho;
    let imported_type_families_chirho = inputs_chirho.imported_type_families_chirho;
    let empty_class_env_chirho = haskelujah_typing_chirho::ClassEnvChirho::new_chirho();
    let imported_class_env_chirho = inputs_chirho
        .class_env_chirho
        .unwrap_or(&empty_class_env_chirho);
    // Check for -fdefer-type-errors / -fdefer-out-of-scope-variables
    // These GHC flags cause type errors to be deferred as warnings.
    let defer_errors_chirho = source_chirho.contains("-fdefer-type-errors")
        || source_chirho.contains("-fdefer-out-of-scope-variables");

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
    let splice_result_chirho =
        splice_chirho::expand_splices_chirho(std::mem::take(&mut module_chirho.decls_chirho));
    module_chirho.decls_chirho = splice_result_chirho.decls_chirho;
    let splice_warnings_chirho = splice_result_chirho.warnings_chirho;

    // Phase 2.5: Deriving — generate instance declarations for `deriving` clauses
    let deriving_warnings_chirho =
        haskelujah_typing_chirho::deriving_chirho::apply_deriving_chirho(&mut module_chirho);

    // Phase 3: Name resolution
    let resolve_result_chirho = resolve_module_with_imports_chirho(&module_chirho, ifaces_chirho);
    if !defer_errors_chirho && resolve_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(resolve_result_chirho.diagnostics_chirho);
    }

    // Phase 3.1: Orphan instance detection
    let orphan_warnings_chirho =
        haskelujah_naming_chirho::check_orphan_instances_chirho(&module_chirho);

    // Phase 3.5: Kind inference
    let imported_contracts_chirho = contracts_chirho::select_imported_contracts_chirho(
        &module_chirho,
        ifaces_chirho,
        inputs_chirho.type_contracts_chirho,
        imported_types_chirho,
    );
    let kind_result_chirho =
        haskelujah_typing_chirho::kind_chirho::infer_module_kinds_with_imports_chirho(
            &module_chirho,
            &imported_contracts_chirho.kinds_chirho,
        );
    if !defer_errors_chirho && kind_result_chirho.diagnostics_chirho.has_errors_chirho() {
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
    for (builtin_name_chirho, builtin_scheme_chirho) in
        haskelujah_typing_chirho::infer_chirho::builtin_value_schemes_chirho()
    {
        merged_imported_types_chirho
            .entry(builtin_name_chirho)
            .or_insert(builtin_scheme_chirho);
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
                    inputs_chirho
                        .type_contracts_chirho
                        .and_then(|contracts_chirho| contracts_chirho.get(&module_name_chirho))
                        .into_iter()
                        .flat_map(contracts_chirho::contract_type_names_chirho),
                )
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
                                None
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
                    let in_scope_seed_scheme_chirho = if let Some(contracts_chirho) = inputs_chirho
                        .type_contracts_chirho
                        .and_then(|modules_chirho| modules_chirho.get(&module_name_chirho))
                    {
                        base_scheme_chirho.map_constructor_names_chirho(&mut |name_chirho| {
                            contracts_chirho::private_contract_name_chirho(
                                name_chirho,
                                iface_chirho,
                                contracts_chirho,
                            )
                            .unwrap_or_else(|| {
                                qualify_imported_type_name_chirho(
                                    name_chirho,
                                    &qualifiable_type_names_chirho,
                                    &qualifier_chirho,
                                    &safe_unqualified_imported_type_names_chirho,
                                    &preferred_qualified_type_names_chirho,
                                )
                            })
                        })
                    } else {
                        qualify_imported_scheme_for_iface_chirho(
                            &base_scheme_chirho,
                            &qualifiable_type_names_chirho,
                            &qualifier_chirho,
                            &safe_unqualified_imported_type_names_chirho,
                            &preferred_qualified_type_names_chirho,
                        )
                    };
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

    // Phase 4: Pass import contracts and the solved kind arguments together.
    // Workflow: language-features-chirho/declaration-kinds-chirho.
    let infer_result_chirho = infer_module_with_inputs_chirho(
        &module_chirho,
        InferInputsChirho {
            boot_chirho: inputs_chirho.boot_chirho,
            imported_types_chirho: &merged_imported_types_chirho,
            imported_type_synonyms_chirho: &merged_imported_type_synonyms_chirho,
            imported_closed_synonyms_chirho: &imported_contracts_chirho.synonyms_chirho,
            imported_type_families_chirho: &merged_imported_type_families_chirho,
            imported_class_env_chirho,
            imported_record_field_names_chirho: &merged_imported_record_field_names_chirho,
            safe_unqualified_imported_type_names_chirho:
                &safe_unqualified_imported_type_names_chirho,
            preferred_qualified_type_names_chirho: &preferred_qualified_type_names_chirho,
            kind_elaboration_chirho: Some(kind_result_chirho.elaboration_chirho),
        },
    );
    if !defer_errors_chirho && infer_result_chirho.diagnostics_chirho.has_errors_chirho() {
        return Err(infer_result_chirho.diagnostics_chirho);
    }

    // Phase 4.4: Source-type validity — reject type forms whose licensing
    // extension is not enabled (GHC's GHC-91510: illegal polymorphic /
    // qualified type).
    let validity_result_chirho =
        haskelujah_typing_chirho::check_module_type_validity_diagnostics_chirho(&module_chirho);
    if !defer_errors_chirho
        && validity_result_chirho
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

    // Collect non-fatal warnings from type inference (e.g. typed holes W4200).
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

    let mut type_contracts_chirho = imported_contracts_chirho;
    type_contracts_chirho
        .kinds_chirho
        .extend(kind_result_chirho.contracts_chirho);
    type_contracts_chirho.synonyms_chirho = infer_result_chirho.type_synonyms_chirho.clone();
    let iface_chirho = if inputs_chirho.boot_chirho {
        haskelujah_naming_chirho::iface_chirho::build_boot_iface_with_imports_chirho(
            &module_chirho,
            ifaces_chirho,
        )
    } else {
        build_iface_with_imports_chirho(&module_chirho, ifaces_chirho)
    };
    let type_contracts_chirho = contracts_chirho::export_contracts_chirho(
        &module_chirho,
        &iface_chirho,
        &type_contracts_chirho,
        &infer_result_chirho,
    );
    Ok(FrontendResultChirho {
        type_contracts_chirho,
        module_chirho,
        infer_result_chirho,
        resolved_imported_type_synonyms_chirho: merged_imported_type_synonyms_chirho,
        warnings_chirho,
    })
}
