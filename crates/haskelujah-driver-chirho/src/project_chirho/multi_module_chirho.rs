// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Multi-module compilation and evaluation of sources given in memory (row 634).

use super::dependency_packages_chirho::insert_exported_schemes_into_imports_chirho;
use super::{
    CompileResultChirho, FrontendResultChirho, ImportedTypeSynonymsChirho, compile_backend_chirho,
    exported_type_synonyms_from_module_chirho, run_frontend_chirho,
    run_frontend_with_type_synonyms_and_type_families_chirho, seed_builtin_type_families_chirho,
    stg_lower_chirho,
};
use haskelujah_diagnostics_chirho::DiagnosticBundleChirho;
use haskelujah_naming_chirho::iface_chirho::{ModuleIfaceChirho, build_iface_with_imports_chirho};
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;

/// Compile multiple Haskell source files in dependency order.
///
/// Each module is compiled with access to previously compiled module interfaces,
/// so import declarations resolve against earlier modules in the list.
/// The caller is responsible for providing sources in dependency order.
pub fn compile_modules_chirho(
    sources_chirho: &[(&str, &str)], // (file_name, source_text)
    source_map_chirho: &mut SourceMapChirho,
) -> Result<Vec<CompileResultChirho>, DiagnosticBundleChirho> {
    let mut results_chirho = Vec::new();
    // Seed with synthetic interfaces for built-in modules (Data.Map, Data.Set, etc.)
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    // Accumulated type schemes from all previously-compiled modules,
    // keyed by unqualified name. Downstream modules receive all upstream
    // exports so that type inference can resolve cross-module references.
    let mut imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    > = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();

    for (file_name_chirho, source_chirho) in sources_chirho {
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            file_name_chirho,
            *source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();

        // Phases 1–4.5: shared front-end
        let frontend_result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
            source_chirho,
            file_id_chirho,
            &ifaces_chirho,
            &imported_types_chirho,
            &imported_type_synonyms_chirho,
            &imported_type_families_chirho,
        )?;

        let FrontendResultChirho {
            module_chirho,
            infer_result_chirho,
            resolved_imported_type_synonyms_chirho,
            warnings_chirho: _warnings_chirho,
        } = frontend_result_chirho;

        // Build interface for downstream modules before consuming module_chirho.
        // Use import-aware variant so `module Foo` re-exports work.
        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

        let extra_dict_param_names_chirho: std::collections::HashSet<String> =
            imported_types_chirho.keys().cloned().collect();

        // Carry both bare and module-qualified export names forward so
        // downstream modules can resolve selective and qualified imports.
        insert_exported_schemes_into_imports_chirho(
            &iface_chirho,
            &infer_result_chirho,
            &mut imported_types_chirho,
        );

        imported_type_synonyms_chirho.extend(exported_type_synonyms_from_module_chirho(
            &module_chirho,
            &iface_chirho,
            &resolved_imported_type_synonyms_chirho,
        ));
        imported_type_families_chirho = infer_result_chirho.type_families_chirho.clone();

        ifaces_chirho.push(iface_chirho);

        // Phases 5–7: back-end
        let compile_result_chirho = compile_backend_chirho(
            module_chirho,
            infer_result_chirho,
            extra_dict_param_names_chirho,
            &ifaces_chirho,
        )?;
        results_chirho.push(compile_result_chirho);
    }

    Ok(results_chirho)
}

/// Compile and evaluate multiple Haskell source modules through the full
/// pipeline, merging all Core IR into a single program, then running the
/// entry point (default `"main"`) from the last module.
pub fn eval_modules_chirho(
    sources_chirho: &[(&str, &str)],
    source_map_chirho: &mut SourceMapChirho,
    entry_name_chirho: Option<&str>,
) -> Result<haskelujah_runtime_chirho::ValueChirho, String> {
    use haskelujah_core_chirho::expr_chirho::{
        CoreExprChirho, CoreIdChirho, CoreModuleChirho as CoreModChirho,
    };

    let results_chirho = compile_modules_chirho(sources_chirho, source_map_chirho)
        .map_err(|diag_chirho| format!("compilation error: {:?}", diag_chirho))?;

    if results_chirho.is_empty() {
        return Err("no modules to evaluate".to_string());
    }

    // Each module's desugarer starts CoreIds from 0, so IDs collide across
    // modules. Offset each module's IDs into non-overlapping ranges, then
    // remap cross-module variable references to point at the correct
    // definition IDs.

    fn offset_id_chirho(id_chirho: CoreIdChirho, off_chirho: u32) -> CoreIdChirho {
        CoreIdChirho(id_chirho.0 + off_chirho)
    }

    fn max_id_in_expr_chirho(expr_chirho: &CoreExprChirho, max_id_chirho: &mut u32) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                *max_id_chirho = (*max_id_chirho).max(id_chirho.0);
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                max_id_in_expr_chirho(fun_chirho, max_id_chirho);
                max_id_in_expr_chirho(arg_chirho, max_id_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                max_id_in_expr_chirho(body_chirho, max_id_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (binder_chirho, rhs_chirho) in binds_chirho {
                    *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                    max_id_in_expr_chirho(rhs_chirho, max_id_chirho);
                }
                max_id_in_expr_chirho(body_chirho, max_id_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                *max_id_chirho = (*max_id_chirho).max(bind_chirho.id_chirho.0);
                max_id_in_expr_chirho(scrutinee_chirho, max_id_chirho);
                for alt_chirho in alts_chirho {
                    for binder_chirho in &alt_chirho.binders_chirho {
                        *max_id_chirho = (*max_id_chirho).max(binder_chirho.id_chirho.0);
                    }
                    max_id_in_expr_chirho(&alt_chirho.rhs_chirho, max_id_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                max_id_in_expr_chirho(body_chirho, max_id_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                max_id_in_expr_chirho(inner_chirho, max_id_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. }
            | CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for arg_chirho in args_chirho {
                    max_id_in_expr_chirho(arg_chirho, max_id_chirho);
                }
            }
        }
    }

    fn max_id_in_module_chirho(module_chirho: &CoreModChirho) -> u32 {
        let mut max_id_chirho = module_chirho
            .names_chirho
            .keys()
            .map(|id_chirho| id_chirho.0)
            .max()
            .unwrap_or(0);
        for binding_chirho in &module_chirho.bindings_chirho {
            max_id_chirho = max_id_chirho.max(binding_chirho.binder_chirho.id_chirho.0);
            max_id_in_expr_chirho(&binding_chirho.rhs_chirho, &mut max_id_chirho);
        }
        max_id_chirho
    }

    fn offset_binder_chirho(
        b_chirho: &mut haskelujah_core_chirho::expr_chirho::BinderChirho,
        off_chirho: u32,
    ) {
        b_chirho.id_chirho = offset_id_chirho(b_chirho.id_chirho, off_chirho);
    }

    fn offset_expr_chirho(expr_chirho: &mut CoreExprChirho, off_chirho: u32) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                *id_chirho = offset_id_chirho(*id_chirho, off_chirho);
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                offset_expr_chirho(fun_chirho, off_chirho);
                offset_expr_chirho(arg_chirho, off_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                offset_binder_chirho(binder_chirho, off_chirho);
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (b_chirho, rhs_chirho) in binds_chirho {
                    offset_binder_chirho(b_chirho, off_chirho);
                    offset_expr_chirho(rhs_chirho, off_chirho);
                }
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                offset_binder_chirho(bind_chirho, off_chirho);
                offset_expr_chirho(scrutinee_chirho, off_chirho);
                for alt_chirho in alts_chirho {
                    for ab_chirho in &mut alt_chirho.binders_chirho {
                        offset_binder_chirho(ab_chirho, off_chirho);
                    }
                    offset_expr_chirho(&mut alt_chirho.rhs_chirho, off_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                offset_expr_chirho(body_chirho, off_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                offset_expr_chirho(inner_chirho, off_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    offset_expr_chirho(a_chirho, off_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    offset_expr_chirho(a_chirho, off_chirho);
                }
            }
        }
    }

    fn remap_expr_chirho(
        expr_chirho: &mut CoreExprChirho,
        remap_chirho: &std::collections::HashMap<CoreIdChirho, CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(id_chirho) => {
                if let Some(&new_id_chirho) = remap_chirho.get(id_chirho) {
                    *id_chirho = new_id_chirho;
                }
            }
            CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                remap_expr_chirho(fun_chirho, remap_chirho);
                remap_expr_chirho(arg_chirho, remap_chirho);
            }
            CoreExprChirho::LamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (_b_chirho, rhs_chirho) in binds_chirho {
                    remap_expr_chirho(rhs_chirho, remap_chirho);
                }
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                alts_chirho,
                ..
            } => {
                remap_expr_chirho(scrutinee_chirho, remap_chirho);
                for alt_chirho in alts_chirho {
                    remap_expr_chirho(&mut alt_chirho.rhs_chirho, remap_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                remap_expr_chirho(body_chirho, remap_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                remap_expr_chirho(inner_chirho, remap_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    remap_expr_chirho(a_chirho, remap_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    remap_expr_chirho(a_chirho, remap_chirho);
                }
            }
        }
    }

    // Phase 1: Offset each module's IDs into non-overlapping ranges.
    let mut merged_bindings_chirho = Vec::new();
    let mut merged_names_chirho: std::collections::HashMap<CoreIdChirho, String> =
        std::collections::HashMap::new();
    let mut merged_newtype_cons_chirho = std::collections::HashSet::new();
    let mut id_offset_chirho: u32 = 0;

    for result_chirho in &results_chirho {
        let mut bindings_chirho = result_chirho.core_chirho.bindings_chirho.clone();

        if id_offset_chirho > 0 {
            // Offset all CoreIds in this module's bindings and names
            for binding_chirho in &mut bindings_chirho {
                offset_binder_chirho(&mut binding_chirho.binder_chirho, id_offset_chirho);
                offset_expr_chirho(&mut binding_chirho.rhs_chirho, id_offset_chirho);
            }
        }

        // Merge names with offset
        for (id_chirho, name_chirho) in &result_chirho.core_chirho.names_chirho {
            merged_names_chirho.insert(
                offset_id_chirho(*id_chirho, id_offset_chirho),
                name_chirho.clone(),
            );
        }

        merged_bindings_chirho.extend(bindings_chirho);
        merged_newtype_cons_chirho.extend(result_chirho.newtype_cons_chirho.clone());

        // Advance the offset past this module's ID range.
        let max_id_chirho = max_id_in_module_chirho(&result_chirho.core_chirho);
        id_offset_chirho += max_id_chirho + 1;
    }

    // Phase 2: Build canonical name → definition CoreId (first wins).
    let mut def_name_to_id_chirho: std::collections::HashMap<String, CoreIdChirho> =
        std::collections::HashMap::new();
    for binding_chirho in &merged_bindings_chirho {
        def_name_to_id_chirho
            .entry(binding_chirho.binder_chirho.name_chirho.clone())
            .or_insert(binding_chirho.binder_chirho.id_chirho);
    }

    // Phase 3: Build remap table for cross-module references.
    // An ID needs remapping if it has a name that maps to a different
    // definition ID (i.e., a downstream module references an upstream name).
    // Only remap IDs that are NOT binder sites.
    let mut all_binder_ids_chirho: std::collections::HashSet<CoreIdChirho> =
        std::collections::HashSet::new();
    fn collect_binder_ids_chirho(
        expr_chirho: &CoreExprChirho,
        set_chirho: &mut std::collections::HashSet<CoreIdChirho>,
    ) {
        match expr_chirho {
            CoreExprChirho::VarChirho(_) | CoreExprChirho::LitChirho(_) => {}
            CoreExprChirho::AppChirho {
                fun_chirho,
                arg_chirho,
            } => {
                collect_binder_ids_chirho(fun_chirho, set_chirho);
                collect_binder_ids_chirho(arg_chirho, set_chirho);
            }
            CoreExprChirho::LamChirho {
                binder_chirho,
                body_chirho,
            } => {
                set_chirho.insert(binder_chirho.id_chirho);
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::LetChirho {
                binds_chirho,
                body_chirho,
                ..
            } => {
                for (b_chirho, rhs_chirho) in binds_chirho {
                    set_chirho.insert(b_chirho.id_chirho);
                    collect_binder_ids_chirho(rhs_chirho, set_chirho);
                }
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::CaseChirho {
                scrutinee_chirho,
                bind_chirho,
                alts_chirho,
                ..
            } => {
                set_chirho.insert(bind_chirho.id_chirho);
                collect_binder_ids_chirho(scrutinee_chirho, set_chirho);
                for alt_chirho in alts_chirho {
                    for ab_chirho in &alt_chirho.binders_chirho {
                        set_chirho.insert(ab_chirho.id_chirho);
                    }
                    collect_binder_ids_chirho(&alt_chirho.rhs_chirho, set_chirho);
                }
            }
            CoreExprChirho::TyLamChirho { body_chirho, .. } => {
                collect_binder_ids_chirho(body_chirho, set_chirho);
            }
            CoreExprChirho::TyAppChirho {
                expr_chirho: inner_chirho,
                ..
            } => {
                collect_binder_ids_chirho(inner_chirho, set_chirho);
            }
            CoreExprChirho::PrimOpChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    collect_binder_ids_chirho(a_chirho, set_chirho);
                }
            }
            CoreExprChirho::ConAppChirho { args_chirho, .. } => {
                for a_chirho in args_chirho {
                    collect_binder_ids_chirho(a_chirho, set_chirho);
                }
            }
        }
    }
    for binding_chirho in &merged_bindings_chirho {
        all_binder_ids_chirho.insert(binding_chirho.binder_chirho.id_chirho);
        collect_binder_ids_chirho(&binding_chirho.rhs_chirho, &mut all_binder_ids_chirho);
    }

    let mut remap_chirho: std::collections::HashMap<CoreIdChirho, CoreIdChirho> =
        std::collections::HashMap::new();
    for (id_chirho, name_chirho) in &merged_names_chirho {
        if all_binder_ids_chirho.contains(id_chirho) {
            continue;
        }
        if let Some(&def_id_chirho) = def_name_to_id_chirho.get(name_chirho)
            && def_id_chirho != *id_chirho
        {
            remap_chirho.insert(*id_chirho, def_id_chirho);
        }
    }

    // Phase 4: Apply the remap to all bindings.
    if !remap_chirho.is_empty() {
        for binding_chirho in &mut merged_bindings_chirho {
            remap_expr_chirho(&mut binding_chirho.rhs_chirho, &remap_chirho);
        }
    }

    let merged_core_chirho = CoreModChirho {
        name_chirho: results_chirho
            .last()
            .unwrap()
            .module_chirho
            .name_chirho
            .text_chirho()
            .to_string(),
        bindings_chirho: merged_bindings_chirho,
        names_chirho: merged_names_chirho,
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        // Only the STG interpreter reads this module, and it reads no native provenance.
        provenance_chirho: Default::default(),
    };

    let (value_chirho, _machine_chirho) = stg_lower_chirho::lower_and_run_chirho(
        &merged_core_chirho,
        entry_name_chirho,
        merged_newtype_cons_chirho,
    )?;
    Ok(value_chirho)
}

/// Compile multiple Haskell source files with incremental recompilation avoidance.
///
/// Uses an [`IncrementalSessionChirho`] to track source fingerprints and
/// dependency-interface fingerprints. Only modules whose source or dependencies
/// have changed are recompiled; the rest reuse cached results.
pub fn compile_modules_incremental_chirho(
    sources_chirho: &[(&str, &str)], // (file_name, source_text)
    source_map_chirho: &mut SourceMapChirho,
    session_chirho: &mut haskelujah_incremental_chirho::IncrementalSessionChirho,
) -> Result<Vec<(CompileResultChirho, bool)>, DiagnosticBundleChirho> {
    use haskelujah_incremental_chirho::{FingerprintChirho, PhaseTagChirho};

    // Phase 0: Register all modules and compute fingerprints.
    // Parse module headers to extract names and imports.
    let mut module_infos_chirho: Vec<(String, String, &str, haskelujah_span_chirho::FileIdChirho)> =
        Vec::new(); // (module_name, file_name, source, file_id)

    for (file_name_chirho, source_chirho) in sources_chirho {
        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
            source_map_chirho,
            *file_name_chirho,
            *source_chirho,
        );
        let file_id_chirho = source_file_chirho.file_id_chirho();
        let parser_chirho = ParserChirho::new_chirho(source_chirho, file_id_chirho);
        let green_chirho = parser_chirho.parse_chirho();
        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);

        let module_name_chirho = module_chirho.name_chirho.text_chirho().to_string();
        let imports_chirho: Vec<String> = module_chirho
            .imports_chirho
            .iter()
            .map(|i_chirho| i_chirho.module_chirho.text_chirho().to_string())
            .collect();

        let import_refs_chirho: Vec<&str> = imports_chirho
            .iter()
            .map(|s_chirho| s_chirho.as_str())
            .collect();

        session_chirho.register_module_chirho(
            &module_name_chirho,
            source_chirho,
            &import_refs_chirho,
        );

        module_infos_chirho.push((
            module_name_chirho,
            file_name_chirho.to_string(),
            source_chirho,
            file_id_chirho,
        ));
    }

    // Determine compilation order.
    let order_chirho = session_chirho
        .compilation_order_chirho()
        .unwrap_or_else(|| {
            module_infos_chirho
                .iter()
                .map(|(n_chirho, _, _, _)| n_chirho.clone())
                .collect()
        });

    let mut results_chirho: Vec<(CompileResultChirho, bool)> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    // TODO: cache serialized CompileResultChirho in the artifact store
    // so the cache-hit branch can skip recompilation entirely.

    for module_name_chirho in &order_chirho {
        let info_chirho = module_infos_chirho
            .iter()
            .find(|(n_chirho, _, _, _)| n_chirho == module_name_chirho);

        let Some((_, _file_name_chirho, source_chirho, file_id_chirho)) = info_chirho else {
            continue;
        };

        let source_fp_chirho = FingerprintChirho::from_str_chirho(source_chirho);
        let needs_rebuild_chirho =
            session_chirho.needs_rebuild_chirho(module_name_chirho, source_fp_chirho);
        let recompiled_chirho = needs_rebuild_chirho.is_some();

        // Phases 1–4.5: shared front-end (same path for both recompile and cache-hit,
        // since Core/AST are not yet serialised to the artifact store).
        let empty_imported_types_chirho = std::collections::HashMap::new();
        let frontend_result_chirho = run_frontend_chirho(
            source_chirho,
            *file_id_chirho,
            &ifaces_chirho,
            &empty_imported_types_chirho,
        )?;
        let FrontendResultChirho {
            module_chirho,
            infer_result_chirho,
            resolved_imported_type_synonyms_chirho: _resolved_imported_type_synonyms_chirho,
            warnings_chirho: _warnings_chirho,
        } = frontend_result_chirho;

        // Build and register the module interface for downstream modules.
        let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

        if recompiled_chirho {
            // Record compilation fingerprint for incremental tracking.
            let iface_fp_chirho =
                FingerprintChirho::from_str_chirho(&format!("{:?}", iface_chirho));
            let record_chirho =
                session_chirho.record_compilation_chirho(module_name_chirho, source_fp_chirho);
            record_chirho.set_phase_chirho(PhaseTagChirho::IfaceChirho, iface_fp_chirho);
        }
        // Cache-hit branch skips fingerprint bookkeeping but still pushes the
        // interface so downstream modules resolve correctly.

        ifaces_chirho.push(iface_chirho);

        // Phases 5–7: back-end
        let compile_result_chirho = compile_backend_chirho(
            module_chirho,
            infer_result_chirho,
            std::collections::HashSet::new(),
            &ifaces_chirho,
        )?;

        results_chirho.push((compile_result_chirho, recompiled_chirho));
    }

    Ok(results_chirho)
}
