// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A directory-based project: module discovery, seeded imports and compilation (row 634).

use super::dependency_packages_chirho::insert_exported_schemes_into_imports_chirho;
use super::{
    CompileResultChirho, FrontendResultChirho, ImportedTypeSynonymsChirho, compile_backend_chirho,
    exported_type_synonyms_from_module_chirho, merge_stdlib_frontend_artifacts_chirho,
    read_haskell_source_file_chirho, run_frontend_chirho,
    run_frontend_with_type_synonyms_and_type_families_chirho, seed_builtin_type_families_chirho,
};
use haskelujah_naming_chirho::iface_chirho::{ModuleIfaceChirho, build_iface_with_imports_chirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Hierarchical project compilation from directory
// ---------------------------------------------------------------------------

/// Result of compiling a directory-based Haskell project.
#[derive(Debug)]
pub struct ProjectCompileResultChirho {
    /// Per-module compilation results, in dependency order.
    pub module_results_chirho: Vec<CompileResultChirho>,
    /// Module names in compilation order.
    pub compilation_order_chirho: Vec<String>,
    /// Non-fatal warnings across all modules.
    pub warnings_chirho: Vec<String>,
}

/// Walk a directory tree and find all `.hs` files recursively.
pub(crate) fn discover_hs_files_chirho(dir_chirho: &Path) -> Vec<PathBuf> {
    let mut files_chirho = Vec::new();
    if !dir_chirho.is_dir() {
        return files_chirho;
    }
    let mut stack_chirho = vec![dir_chirho.to_path_buf()];
    while let Some(current_chirho) = stack_chirho.pop() {
        let entries_chirho = match std::fs::read_dir(&current_chirho) {
            Ok(e_chirho) => e_chirho,
            Err(_) => continue,
        };
        for entry_chirho in entries_chirho.flatten() {
            let path_chirho = entry_chirho.path();
            if path_chirho.is_dir() {
                // Skip hidden directories and common non-source directories
                let name_chirho = entry_chirho.file_name();
                let name_str_chirho = name_chirho.to_string_lossy();
                if !name_str_chirho.starts_with('.')
                    && name_str_chirho != "dist-newstyle"
                    && name_str_chirho != "node_modules"
                    && name_str_chirho != ".stack-work"
                {
                    stack_chirho.push(path_chirho);
                }
            } else if path_chirho
                .extension()
                .is_some_and(|ext_chirho| ext_chirho == "hs")
            {
                files_chirho.push(path_chirho);
            }
        }
    }
    files_chirho.sort();
    files_chirho
}

/// Extract module name from a Haskell source string by scanning for `module Name where`.
/// Falls back to "Main" if no module header is found.
pub(crate) fn extract_module_name_chirho(source_chirho: &str) -> String {
    for line_chirho in source_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();
        if trimmed_chirho.is_empty() || trimmed_chirho.starts_with("--") {
            continue;
        }
        if trimmed_chirho.starts_with("{-") {
            continue; // skip block comment starts (simplified)
        }
        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("module ") {
            let rest_chirho = rest_chirho.trim();
            // Module name is everything before "(" or "where"
            // Check "(" first since export lists appear before "where"
            let name_chirho = rest_chirho
                .split_once('(')
                .or_else(|| rest_chirho.split_once(" where"))
                .or_else(|| rest_chirho.split_once("where"))
                .map_or(rest_chirho, |(n_chirho, _)| n_chirho);
            return name_chirho.trim().to_string();
        }
        break; // First non-comment, non-module line means implicit Main
    }
    "Main".to_string()
}

/// Extract imported module names from a Haskell source string.
pub(crate) fn extract_imports_chirho(source_chirho: &str) -> Vec<String> {
    let mut imports_chirho = Vec::new();
    for line_chirho in source_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();
        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("import ")
            && let Some(module_name_chirho) = parse_import_module_name_chirho(rest_chirho)
        {
            imports_chirho.push(module_name_chirho);
        }
    }
    imports_chirho
}

fn parse_import_module_name_chirho(rest_chirho: &str) -> Option<String> {
    let mut remaining_chirho = rest_chirho.trim();

    'scan_modifiers_chirho: loop {
        if let Some(pragma_rest_chirho) = remaining_chirho.strip_prefix("{-#")
            && let Some(pragma_end_chirho) = pragma_rest_chirho.find("#-}")
        {
            remaining_chirho = pragma_rest_chirho[pragma_end_chirho + 3..].trim_start();
            continue;
        }

        for keyword_chirho in ["qualified", "safe", "unsafe", "interruptible"] {
            if let Some(after_keyword_chirho) = remaining_chirho.strip_prefix(keyword_chirho)
                && after_keyword_chirho
                    .chars()
                    .next()
                    .is_some_and(|char_chirho| char_chirho.is_whitespace())
            {
                remaining_chirho = after_keyword_chirho.trim_start();
                continue 'scan_modifiers_chirho;
            }
        }

        if let Some(package_rest_chirho) = remaining_chirho.strip_prefix('"')
            && let Some(package_end_chirho) = package_rest_chirho.find('"')
        {
            remaining_chirho = package_rest_chirho[package_end_chirho + 1..].trim_start();
            continue;
        }

        break;
    }

    let module_name_chirho = remaining_chirho
        .split(|char_chirho: char| char_chirho.is_whitespace() || char_chirho == '(')
        .next()
        .unwrap_or("")
        .trim();
    if module_name_chirho.is_empty() {
        None
    } else {
        Some(module_name_chirho.to_string())
    }
}

pub(crate) fn filter_seeded_type_synonyms_for_source_chirho(
    source_chirho: &str,
    imported_type_synonyms_chirho: &ImportedTypeSynonymsChirho,
) -> ImportedTypeSynonymsChirho {
    let imported_modules_chirho: std::collections::HashSet<String> =
        extract_imports_chirho(source_chirho).into_iter().collect();
    if imported_modules_chirho.is_empty() {
        return ImportedTypeSynonymsChirho::new();
    }

    imported_type_synonyms_chirho
        .iter()
        .filter(|(name_chirho, _synonym_chirho)| {
            imported_modules_chirho.iter().any(|module_name_chirho| {
                name_chirho
                    .strip_prefix(module_name_chirho)
                    .is_some_and(|suffix_chirho| suffix_chirho.starts_with('.'))
            })
        })
        .map(|(name_chirho, synonym_chirho)| (name_chirho.clone(), synonym_chirho.clone()))
        .collect()
}

pub(crate) fn filter_seeded_imported_types_for_source_chirho(
    source_chirho: &str,
    imported_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::SchemeChirho,
    >,
) -> std::collections::HashMap<String, haskelujah_typing_chirho::SchemeChirho> {
    let imported_modules_chirho: std::collections::HashSet<String> =
        extract_imports_chirho(source_chirho).into_iter().collect();
    if imported_modules_chirho.is_empty() {
        return std::collections::HashMap::new();
    }

    let mut qualified_bare_schemes_chirho: std::collections::HashMap<
        String,
        Vec<haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    > = std::collections::HashMap::new();
    for (qualified_name_chirho, qualified_scheme_chirho) in imported_types_chirho {
        for module_name_chirho in &imported_modules_chirho {
            let Some(suffix_chirho) = qualified_name_chirho.strip_prefix(module_name_chirho) else {
                continue;
            };
            if !suffix_chirho.starts_with('.') {
                continue;
            }
            let remainder_chirho = &suffix_chirho[1..];
            if let Some((first_segment_chirho, _rest_chirho)) = remainder_chirho.split_once('.')
                && first_segment_chirho
                    .chars()
                    .next()
                    .is_some_and(|chirho| chirho.is_uppercase())
            {
                continue;
            }
            if !remainder_chirho.is_empty() && !remainder_chirho.contains('.') {
                qualified_bare_schemes_chirho
                    .entry(remainder_chirho.to_string())
                    .or_default()
                    .push(qualified_scheme_chirho.clone());
            }
        }
    }

    imported_types_chirho
        .iter()
        .filter(|(name_chirho, scheme_chirho)| {
            if !name_chirho.contains('.') {
                return qualified_bare_schemes_chirho.get(*name_chirho).is_some_and(
                    |schemes_chirho| {
                        schemes_chirho.len() == 1
                            && schemes_chirho
                                .first()
                                .is_some_and(|qualified_scheme_chirho| {
                                    qualified_scheme_chirho == *scheme_chirho
                                })
                    },
                );
            }
            imported_modules_chirho.iter().any(|module_name_chirho| {
                name_chirho
                    .strip_prefix(module_name_chirho)
                    .is_some_and(|suffix_chirho| {
                        if !suffix_chirho.starts_with('.') {
                            return false;
                        }
                        let remainder_chirho = &suffix_chirho[1..];
                        if let Some((first_segment_chirho, _rest_chirho)) =
                            remainder_chirho.split_once('.')
                            && first_segment_chirho
                                .chars()
                                .next()
                                .is_some_and(|chirho| chirho.is_uppercase())
                        {
                            return false;
                        }
                        true
                    })
            })
        })
        .map(|(name_chirho, scheme_chirho)| (name_chirho.clone(), scheme_chirho.clone()))
        .collect()
}

pub(crate) fn source_imports_stdlib_chirho(source_chirho: &str) -> bool {
    extract_imports_chirho(source_chirho)
        .iter()
        .any(|module_name_chirho| module_name_chirho.starts_with("Haskelujah."))
}

/// Parse a `.hs-boot` file to extract a minimal module interface.
///
/// Boot files provide enough type and value declarations to break circular
/// import cycles. This function runs the front-end pipeline on the boot file
/// to produce a `ModuleIfaceChirho` and extracted type schemes.
fn parse_boot_iface_chirho(
    module_name_chirho: &str,
    boot_source_chirho: &str,
    source_map_chirho: &mut SourceMapChirho,
    boot_path_chirho: &str,
    existing_ifaces_chirho: &[ModuleIfaceChirho],
    existing_types_chirho: &std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
) -> Result<
    (
        ModuleIfaceChirho,
        std::collections::HashMap<String, haskelujah_typing_chirho::ty_chirho::SchemeChirho>,
    ),
    String,
> {
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        source_map_chirho,
        boot_path_chirho,
        boot_source_chirho,
    );
    let file_id_chirho = source_file_chirho.file_id_chirho();

    let frontend_result_chirho = run_frontend_chirho(
        boot_source_chirho,
        file_id_chirho,
        existing_ifaces_chirho,
        existing_types_chirho,
    )
    .map_err(|e_chirho| format!("Boot file {} error: {}", module_name_chirho, e_chirho))?;

    let FrontendResultChirho {
        module_chirho,
        infer_result_chirho,
        resolved_imported_type_synonyms_chirho: _resolved_imported_type_synonyms_chirho,
        ..
    } = frontend_result_chirho;

    let iface_chirho = build_iface_with_imports_chirho(&module_chirho, existing_ifaces_chirho);

    // Extract type schemes from the boot file
    let mut types_chirho = std::collections::HashMap::new();
    for name_chirho in iface_chirho.exports_chirho.values_chirho.keys() {
        if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(name_chirho) {
            types_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
        }
    }
    for ty_info_chirho in iface_chirho.exports_chirho.types_chirho.values() {
        for con_name_chirho in &ty_info_chirho.constructors_chirho {
            if let Some(scheme_chirho) = infer_result_chirho
                .env_chirho
                .lookup_chirho(con_name_chirho)
            {
                types_chirho.insert(con_name_chirho.clone(), scheme_chirho.clone());
            }
        }
    }

    Ok((iface_chirho, types_chirho))
}

/// Compile a Haskell project from a directory, automatically discovering `.hs`
/// files, computing dependency order from import declarations, and compiling
/// modules in topological order. Circular module imports are handled via
/// `.hs-boot` files.
///
/// Returns an error if:
/// - No `.hs` files are found in the directory
/// - Any module fails to compile
pub fn compile_project_dir_chirho(
    project_dir_chirho: &Path,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<ProjectCompileResultChirho, String> {
    // Step 1: Discover all .hs files
    let hs_files_chirho = discover_hs_files_chirho(project_dir_chirho);
    if hs_files_chirho.is_empty() {
        return Err(format!(
            "No .hs files found in {}",
            project_dir_chirho.display()
        ));
    }

    // Step 2: Read sources and extract module names + imports
    let mut module_sources_chirho: Vec<(String, String, String)> = Vec::new(); // (module_name, file_path, source)
    for path_chirho in &hs_files_chirho {
        let source_chirho = read_haskell_source_file_chirho(path_chirho).map_err(|e_chirho| {
            format!("Failed to read {}: {}", path_chirho.display(), e_chirho)
        })?;
        let module_name_chirho = extract_module_name_chirho(&source_chirho);
        let file_name_chirho = path_chirho.to_string_lossy().to_string();
        module_sources_chirho.push((module_name_chirho, file_name_chirho, source_chirho));
    }

    // Step 3: Build dependency graph
    let mut dep_graph_chirho = haskelujah_incremental_chirho::DepGraphChirho::new_chirho();
    let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();

    for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
        let fp_chirho =
            haskelujah_incremental_chirho::FingerprintChirho::from_str_chirho(source_chirho);
        dep_graph_chirho.add_module_chirho(module_name_chirho, fp_chirho);
        let imports_chirho = extract_imports_chirho(source_chirho);
        for imported_chirho in &imports_chirho {
            // Only add edges for local modules (skip external like Prelude, Data.Map, etc.)
            if known_modules_chirho.contains(imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, imported_chirho);
            }
        }
    }

    // Step 4: Topological sort via SCCs (handles circular imports)
    let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();

    // Flatten SCC order for the result, handling cycles via .hs-boot files.
    let mut order_chirho: Vec<String> = Vec::new();

    // Step 5: Compile in dependency order, respecting SCCs
    let mut results_chirho: Vec<CompileResultChirho> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut all_warnings_chirho: Vec<String> = Vec::new();
    let mut imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    > = std::collections::HashMap::new();
    let mut imported_type_synonyms_chirho = ImportedTypeSynonymsChirho::new();
    let mut imported_type_families_chirho = seed_builtin_type_families_chirho();
    if module_sources_chirho
        .iter()
        .any(|(_, _, source_chirho)| source_imports_stdlib_chirho(source_chirho))
    {
        merge_stdlib_frontend_artifacts_chirho(
            &mut ifaces_chirho,
            &mut imported_types_chirho,
            &mut imported_type_synonyms_chirho,
            &mut imported_type_families_chirho,
        );
    }

    for scc_chirho in &sccs_chirho {
        if scc_chirho.len() > 1 {
            // Circular import group — look for .hs-boot files to break the cycle.
            // Phase 1: For each SCC member, either compile its .hs-boot file
            // or create a minimal empty interface so that `import M` resolves.
            for module_name_chirho in scc_chirho {
                let (_, file_name_chirho, _source_chirho) = module_sources_chirho
                    .iter()
                    .find(|(n_chirho, _, _)| n_chirho == module_name_chirho)
                    .ok_or_else(|| format!("Module {} not found in sources", module_name_chirho))?;

                // Look for a .hs-boot file alongside the .hs file
                let boot_path_chirho = format!("{}-boot", file_name_chirho);
                if let Ok(boot_source_chirho) = read_haskell_source_file_chirho(&boot_path_chirho) {
                    // Parse the boot file to extract a minimal interface
                    let boot_iface_chirho = parse_boot_iface_chirho(
                        module_name_chirho,
                        &boot_source_chirho,
                        source_map_chirho,
                        &boot_path_chirho,
                        &ifaces_chirho,
                        &imported_types_chirho,
                    );
                    match boot_iface_chirho {
                        Ok((iface_chirho, types_chirho)) => {
                            ifaces_chirho.push(iface_chirho);
                            imported_types_chirho.extend(types_chirho);
                        }
                        Err(e_chirho) => {
                            all_warnings_chirho.push(format!(
                                "Warning: failed to parse {}: {}",
                                boot_path_chirho, e_chirho
                            ));
                            // Fall back to empty interface
                            ifaces_chirho.push(ModuleIfaceChirho {
                                name_chirho: module_name_chirho.clone(),
                                exports_chirho: Default::default(),
                            });
                        }
                    }
                } else {
                    // No boot file — create an empty interface so `import M`
                    // at least resolves (names won't be in scope without a boot
                    // file, matching GHC's behavior).
                    ifaces_chirho.push(ModuleIfaceChirho {
                        name_chirho: module_name_chirho.clone(),
                        exports_chirho: Default::default(),
                    });
                }
            }

            // Phase 2: Compile all modules in the SCC using boot interfaces.
            // The order within an SCC doesn't matter (boot files break the cycle).
        }

        // Compile each module in the SCC (or the single module if no cycle)
        for module_name_chirho in scc_chirho {
            let (_, file_name_chirho, source_chirho) = module_sources_chirho
                .iter()
                .find(|(n_chirho, _, _)| n_chirho == module_name_chirho)
                .ok_or_else(|| format!("Module {} not found in sources", module_name_chirho))?;

            let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                source_map_chirho,
                file_name_chirho,
                source_chirho,
            );
            let file_id_chirho = source_file_chirho.file_id_chirho();
            let filtered_imported_types_chirho = filter_seeded_imported_types_for_source_chirho(
                source_chirho,
                &imported_types_chirho,
            );
            let filtered_imported_type_synonyms_chirho =
                filter_seeded_type_synonyms_for_source_chirho(
                    source_chirho,
                    &imported_type_synonyms_chirho,
                );

            let frontend_result_chirho = run_frontend_with_type_synonyms_and_type_families_chirho(
                source_chirho,
                file_id_chirho,
                &ifaces_chirho,
                &filtered_imported_types_chirho,
                &filtered_imported_type_synonyms_chirho,
                &imported_type_families_chirho,
            )
            .map_err(|e_chirho| format!("Error compiling {}: {}", module_name_chirho, e_chirho))?;

            let FrontendResultChirho {
                module_chirho,
                infer_result_chirho,
                resolved_imported_type_synonyms_chirho,
                warnings_chirho,
            } = frontend_result_chirho;

            all_warnings_chirho.extend(warnings_chirho);

            // Build interface for downstream modules
            let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);

            let extra_dict_param_names_chirho: std::collections::HashSet<String> =
                imported_types_chirho.keys().cloned().collect();

            // Carry both bare and module-qualified export names forward so
            // downstream package modules can resolve imported signatures without
            // falling back to polymorphic placeholders.
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

            // Backend compilation
            let compile_result_chirho = compile_backend_chirho(
                module_chirho,
                infer_result_chirho,
                extra_dict_param_names_chirho,
                &ifaces_chirho,
            )
            .map_err(|e_chirho| {
                format!("Backend error for {}: {}", module_name_chirho, e_chirho)
            })?;

            results_chirho.push(compile_result_chirho);
            order_chirho.push(module_name_chirho.clone());
        }
    }

    Ok(ProjectCompileResultChirho {
        module_results_chirho: results_chirho,
        compilation_order_chirho: order_chirho,
        warnings_chirho: all_warnings_chirho,
    })
}
