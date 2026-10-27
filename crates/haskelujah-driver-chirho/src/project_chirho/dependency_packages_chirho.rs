// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Local dependency packages: their interfaces and frontend artifacts (row 634).

use super::cabal_build_chirho::{
    collect_library_package_deps_chirho, package_module_cpp_options_map_chirho,
};
use super::module_discovery_chirho::discover_library_modules_chirho;
#[cfg(test)]
use super::module_search_chirho::{
    extract_exported_names_from_source_chirho, extract_module_name_from_source_chirho,
};
use super::package_install_chirho::find_cabal_in_dir_chirho;
use super::project_dir_chirho::{
    ProjectCompileResultChirho, extract_imports_chirho,
    filter_seeded_imported_types_for_source_chirho, filter_seeded_type_synonyms_for_source_chirho,
    source_imports_stdlib_chirho,
};
use super::{
    CompileResultChirho, FrontendResultChirho, FrontendSeedArtifactsChirho,
    ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho, compile_backend_chirho,
    exported_type_synonyms_from_module_chirho, extend_imported_type_families_chirho,
    merge_stdlib_frontend_artifacts_chirho, preprocess_cpp_source_with_options_chirho,
    read_haskell_source_file_chirho, run_frontend_with_type_synonyms_families_and_class_env_chirho,
    seed_builtin_type_families_chirho,
};
use haskelujah_naming_chirho::iface_chirho::{ModuleIfaceChirho, build_iface_with_imports_chirho};
#[cfg(test)]
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
#[cfg(test)]
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;
use haskelujah_typing_chirho::infer_chirho::InferResultChirho;
use std::path::{Path, PathBuf};

/// Scan downloaded dependency packages in .haskelujah-packages-chirho/ for stub
/// module interfaces. This enables cross-package module resolution.
#[cfg(test)]
pub(crate) fn scan_dependency_package_ifaces_chirho(
    project_dir_chirho: &Path,
) -> Vec<ModuleIfaceChirho> {
    let mut ifaces_chirho = Vec::new();
    let current_package_dir_chirho = std::fs::canonicalize(project_dir_chirho)
        .unwrap_or_else(|_| project_dir_chirho.to_path_buf());

    // Look for .haskelujah-packages-chirho/ relative to project dir ancestors
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        return ifaces_chirho;
    };

    // Seed the current package first when the caller itself lives under
    // .haskelujah-packages-chirho/. This gives tests and package-scan callers
    // forward module stubs for same-package imports such as containers'
    // Utils.Containers.Internal.BitUtil.
    scan_package_hs_files_chirho(project_dir_chirho, &mut ifaces_chirho);

    // Scan each package directory
    let entries_chirho = match std::fs::read_dir(&packages_dir_chirho) {
        Ok(e_chirho) => e_chirho,
        Err(_) => return ifaces_chirho,
    };

    for entry_chirho in entries_chirho.flatten() {
        let pkg_dir_chirho = entry_chirho.path();
        if !pkg_dir_chirho.is_dir() {
            continue;
        }
        let canonical_pkg_dir_chirho =
            std::fs::canonicalize(&pkg_dir_chirho).unwrap_or_else(|_| pkg_dir_chirho.clone());
        if canonical_pkg_dir_chirho == current_package_dir_chirho {
            continue;
        }
        // Recursively find .hs files and generate stub interfaces
        scan_package_hs_files_chirho(&pkg_dir_chirho, &mut ifaces_chirho);
    }

    ifaces_chirho
}

pub(crate) fn find_dependency_packages_dir_chirho(project_dir_chirho: &Path) -> Option<PathBuf> {
    let mut search_dir_chirho = project_dir_chirho.to_path_buf();
    loop {
        let candidate_chirho = search_dir_chirho.join(".haskelujah-packages-chirho");
        if candidate_chirho.is_dir() {
            return Some(candidate_chirho);
        }
        if !search_dir_chirho.pop() {
            return None;
        }
    }
}

pub(crate) fn find_local_dependency_package_dir_chirho(
    packages_dir_chirho: &Path,
    package_name_chirho: &str,
) -> Option<PathBuf> {
    let mut candidates_chirho =
        find_local_dependency_package_dirs_chirho(packages_dir_chirho, package_name_chirho);
    candidates_chirho.sort();
    candidates_chirho.pop()
}

fn find_local_dependency_package_dirs_chirho(
    packages_dir_chirho: &Path,
    package_name_chirho: &str,
) -> Vec<PathBuf> {
    std::fs::read_dir(packages_dir_chirho)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry_chirho| entry_chirho.path())
        .filter(|path_chirho| {
            path_chirho.is_dir()
                && path_chirho.file_name().is_some_and(|file_name_chirho| {
                    package_dir_matches_dependency_chirho(
                        &file_name_chirho.to_string_lossy(),
                        package_name_chirho,
                    )
                })
        })
        .collect()
}

fn package_dir_matches_dependency_chirho(dir_name_chirho: &str, package_name_chirho: &str) -> bool {
    if dir_name_chirho == package_name_chirho {
        return true;
    }
    let prefix_chirho = format!("{package_name_chirho}-");
    let Some(version_suffix_chirho) = dir_name_chirho.strip_prefix(&prefix_chirho) else {
        return false;
    };
    version_suffix_chirho
        .chars()
        .next()
        .is_some_and(|char_chirho| char_chirho.is_ascii_digit())
}

pub(crate) fn builtin_dependency_names_chirho() -> std::collections::HashSet<String> {
    [
        "base",
        "ghc-prim",
        "ghc-bignum",
        "ghc-internal",
        "rts",
        "deepseq",
        "array",
        "bytestring",
        "containers",
        "text",
        "filepath",
        "directory",
        "process",
        "time",
        "template-haskell",
        "pretty",
        "binary",
        "integer-gmp",
        "ghc-boot-th",
        "ghc-boot",
        "os-string",
        "unix",
        "Win32",
    ]
    .into_iter()
    .map(|name_chirho| name_chirho.to_string())
    .collect()
}

pub(crate) fn merge_local_dependency_package_index_chirho(
    project_dir_chirho: &Path,
    _root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
    base_index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<haskelujah_package_chirho::PackageIndexChirho, String> {
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        return Ok(base_index_chirho.clone());
    };

    let builtin_deps_chirho = builtin_dependency_names_chirho();
    let mut merged_index_chirho = base_index_chirho.clone();
    let mut package_dirs_chirho: Vec<PathBuf> = std::fs::read_dir(&packages_dir_chirho)
        .map_err(|error_chirho| {
            format!(
                "cannot read local dependency directory {}: {}",
                packages_dir_chirho.display(),
                error_chirho
            )
        })?
        .filter_map(|entry_chirho| entry_chirho.ok().map(|entry_chirho| entry_chirho.path()))
        .filter(|path_chirho| path_chirho.is_dir())
        .collect();
    package_dirs_chirho.sort();

    for package_dir_chirho in package_dirs_chirho {
        let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) else {
            continue;
        };

        let Ok(cabal_content_chirho) = std::fs::read_to_string(&cabal_path_chirho) else {
            continue;
        };
        let package_chirho = haskelujah_package_chirho::parse_cabal_chirho(&cabal_content_chirho);
        if package_chirho.name_chirho.is_empty() {
            continue;
        }
        if builtin_deps_chirho.contains(&package_chirho.name_chirho) {
            continue;
        }

        let Some(package_version_chirho) = package_chirho.version_chirho.clone().or_else(|| {
            package_dir_chirho
                .file_name()
                .and_then(|file_name_chirho| file_name_chirho.to_str())
                .and_then(|dir_name_chirho| {
                    dir_name_chirho
                        .strip_prefix(&format!("{}-", package_chirho.name_chirho))
                        .and_then(haskelujah_package_chirho::parse_version_chirho)
                })
        }) else {
            continue;
        };
        let package_deps_chirho = collect_library_package_deps_chirho(&package_chirho);

        let already_present_chirho = merged_index_chirho
            .packages_chirho
            .get(&package_chirho.name_chirho)
            .is_some_and(|versions_chirho| {
                versions_chirho
                    .iter()
                    .any(|meta_chirho| meta_chirho.version_chirho == package_version_chirho)
            });
        if !already_present_chirho {
            merged_index_chirho.add_package_chirho(
                &package_chirho.name_chirho,
                package_version_chirho,
                package_deps_chirho,
            );
        }
    }

    Ok(merged_index_chirho)
}

pub(crate) fn collect_local_dependency_frontend_artifacts_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
) -> Result<FrontendSeedArtifactsChirho, String> {
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        return Ok(FrontendSeedArtifactsChirho::default());
    };

    let builtin_deps_chirho = builtin_dependency_names_chirho();
    // Seed PrimState type family equations so dependency packages like
    // primitive can resolve PrimState (ST s) = s during compilation.
    let mut artifacts_chirho = FrontendSeedArtifactsChirho {
        imported_type_families_chirho: seed_builtin_type_families_chirho(),
        ..Default::default()
    };
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let mut visited_chirho = std::collections::HashSet::new();
    let mut active_chirho = std::collections::HashSet::new();

    for dep_chirho in root_deps_chirho {
        if let Err(error_chirho) = compile_local_dependency_package_frontend_recursive_chirho(
            &dep_chirho.package_chirho,
            &packages_dir_chirho,
            &builtin_deps_chirho,
            &mut visited_chirho,
            &mut active_chirho,
            &mut artifacts_chirho,
            &mut source_map_chirho,
        ) {
            eprintln!(
                "warning: local dependency '{}' frontend compile skipped: {}",
                dep_chirho.package_chirho, error_chirho
            );
        }
    }

    Ok(artifacts_chirho)
}

fn compile_local_dependency_package_frontend_recursive_chirho(
    package_name_chirho: &str,
    packages_dir_chirho: &Path,
    builtin_deps_chirho: &std::collections::HashSet<String>,
    visited_chirho: &mut std::collections::HashSet<String>,
    active_chirho: &mut std::collections::HashSet<String>,
    artifacts_chirho: &mut FrontendSeedArtifactsChirho,
    source_map_chirho: &mut SourceMapChirho,
) -> Result<(), String> {
    if builtin_deps_chirho.contains(package_name_chirho)
        || visited_chirho.contains(package_name_chirho)
    {
        return Ok(());
    }
    if !active_chirho.insert(package_name_chirho.to_string()) {
        return Ok(());
    }

    let Some(package_dir_chirho) =
        find_local_dependency_package_dir_chirho(packages_dir_chirho, package_name_chirho)
    else {
        active_chirho.remove(package_name_chirho);
        return Ok(());
    };

    let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) else {
        active_chirho.remove(package_name_chirho);
        return Ok(());
    };

    let cabal_content_chirho = std::fs::read_to_string(&cabal_path_chirho).map_err(|e_chirho| {
        format!(
            "cannot read dependency cabal {}: {}",
            cabal_path_chirho.display(),
            e_chirho
        )
    })?;
    let package_chirho = haskelujah_package_chirho::parse_cabal_chirho(&cabal_content_chirho);

    for dep_chirho in collect_library_package_deps_chirho(&package_chirho) {
        if let Err(error_chirho) = compile_local_dependency_package_frontend_recursive_chirho(
            &dep_chirho.package_chirho,
            packages_dir_chirho,
            builtin_deps_chirho,
            visited_chirho,
            active_chirho,
            artifacts_chirho,
            source_map_chirho,
        ) {
            eprintln!(
                "warning: transitive local dependency '{}' frontend compile skipped: {}",
                dep_chirho.package_chirho, error_chirho
            );
        }
    }

    let source_files_chirho = discover_library_modules_chirho(&package_chirho, &package_dir_chirho);
    let source_file_cpp_options_chirho =
        package_module_cpp_options_map_chirho(&package_chirho, &package_dir_chirho);
    let mut module_sources_chirho = Vec::new();
    for (module_name_chirho, path_chirho) in &source_files_chirho {
        let raw_source_chirho =
            read_haskell_source_file_chirho(path_chirho).map_err(|e_chirho| {
                format!(
                    "cannot read dependency module {} at {}: {}",
                    module_name_chirho,
                    path_chirho.display(),
                    e_chirho
                )
            })?;
        let cpp_options_for_module_chirho = source_file_cpp_options_chirho
            .get(path_chirho)
            .cloned()
            .unwrap_or_default();
        let source_chirho = preprocess_cpp_source_with_options_chirho(
            path_chirho,
            &raw_source_chirho,
            &cpp_options_for_module_chirho,
        )
        .map_err(|e_chirho| {
            format!(
                "cannot preprocess dependency module {} at {}: {}",
                module_name_chirho,
                path_chirho.display(),
                e_chirho
            )
        })?;
        module_sources_chirho.push((
            module_name_chirho.clone(),
            path_chirho.to_string_lossy().to_string(),
            source_chirho,
        ));
    }

    let compiled_artifacts_chirho = collect_frontend_artifacts_from_module_sources_chirho(
        module_sources_chirho,
        source_map_chirho,
        artifacts_chirho.ifaces_chirho.clone(),
        artifacts_chirho.imported_types_chirho.clone(),
        artifacts_chirho.imported_type_synonyms_chirho.clone(),
        artifacts_chirho.imported_type_families_chirho.clone(),
        true,
    )
    .map_err(|error_chirho| {
        format!(
            "dependency package '{}': {}",
            package_name_chirho, error_chirho
        )
    })?;

    artifacts_chirho.ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(
            compiled_artifacts_chirho.ifaces_chirho,
        );
    artifacts_chirho
        .imported_types_chirho
        .extend(compiled_artifacts_chirho.imported_types_chirho);
    artifacts_chirho
        .imported_type_synonyms_chirho
        .extend(compiled_artifacts_chirho.imported_type_synonyms_chirho);
    extend_imported_type_families_chirho(
        &mut artifacts_chirho.imported_type_families_chirho,
        compiled_artifacts_chirho.imported_type_families_chirho,
    );

    active_chirho.remove(package_name_chirho);
    visited_chirho.insert(package_name_chirho.to_string());
    Ok(())
}

/// Recursively scan a package directory for .hs files and generate stub interfaces.
#[cfg(test)]
fn scan_package_hs_files_chirho(dir_chirho: &Path, ifaces_chirho: &mut Vec<ModuleIfaceChirho>) {
    let entries_chirho = match std::fs::read_dir(dir_chirho) {
        Ok(e_chirho) => e_chirho,
        Err(_) => return,
    };

    for entry_chirho in entries_chirho.flatten() {
        let path_chirho = entry_chirho.path();
        if path_chirho.is_dir() {
            let dir_name_chirho = path_chirho
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            // Skip hidden dirs, dist, test, bench directories
            if dir_name_chirho.starts_with('.')
                || dir_name_chirho == "dist-chirho"
                || dir_name_chirho == "tests"
                || dir_name_chirho == "test"
                || dir_name_chirho == "bench"
                || dir_name_chirho == "benchmarks"
            {
                continue;
            }
            scan_package_hs_files_chirho(&path_chirho, ifaces_chirho);
        } else if path_chirho.extension().is_some_and(|extension_chirho| {
            extension_chirho == "hs" || extension_chirho == "lhs" || extension_chirho == "hsc"
        }) {
            let file_name_chirho = path_chirho
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            if file_name_chirho == "Setup.hs"
                || file_name_chirho == "Setup.lhs"
                || file_name_chirho == "Setup.hsc"
            {
                continue;
            }
            // Read source and extract module name + exports
            if let Ok(preprocessed_source_chirho) = read_haskell_source_file_chirho(&path_chirho) {
                let mut stub_iface_chirho = None;
                if let Some(mod_name_chirho) =
                    extract_module_name_from_source_chirho(&preprocessed_source_chirho)
                {
                    use haskelujah_naming_chirho::iface_chirho::{
                        IfaceExportsChirho, IfaceTypeChirho, IfaceValueChirho,
                    };
                    let span_chirho = haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO;
                    let mut exports_chirho = IfaceExportsChirho::default();
                    for name_chirho in
                        extract_exported_names_from_source_chirho(&preprocessed_source_chirho)
                    {
                        let first_char_chirho = name_chirho.chars().next().unwrap_or('a');
                        if first_char_chirho.is_uppercase() {
                            exports_chirho.types_chirho.insert(
                                name_chirho.clone(),
                                IfaceTypeChirho {
                                    name_chirho: name_chirho.clone(),
                                    constructors_chirho: vec![],
                                    methods_chirho: vec![],
                                    span_chirho,
                                },
                            );
                            exports_chirho.values_chirho.insert(
                                name_chirho.clone(),
                                IfaceValueChirho {
                                    name_chirho: name_chirho.clone(),
                                    span_chirho,
                                },
                            );
                        } else {
                            exports_chirho.values_chirho.insert(
                                name_chirho.clone(),
                                IfaceValueChirho {
                                    name_chirho: name_chirho.clone(),
                                    span_chirho,
                                },
                            );
                        }
                    }
                    stub_iface_chirho = Some(ModuleIfaceChirho {
                        name_chirho: mod_name_chirho,
                        exports_chirho,
                    });
                }

                let real_iface_result_chirho =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut source_map_chirho = SourceMapChirho::new_chirho();
                        let source_file_chirho = SourceFileChirho::from_source_map_chirho(
                            &mut source_map_chirho,
                            &path_chirho,
                            &preprocessed_source_chirho,
                        );
                        let file_id_chirho = source_file_chirho.file_id_chirho();
                        let parser_chirho =
                            ParserChirho::new_chirho(&preprocessed_source_chirho, file_id_chirho);
                        let green_chirho = parser_chirho.parse_chirho();
                        let module_chirho = lower_module_chirho(&green_chirho, file_id_chirho);
                        build_iface_with_imports_chirho(&module_chirho, ifaces_chirho)
                    }));

                if let Ok(real_iface_chirho) = real_iface_result_chirho {
                    ifaces_chirho.push(real_iface_chirho);
                } else if let Some(stub_iface_chirho) = stub_iface_chirho {
                    ifaces_chirho.push(stub_iface_chirho);
                }
            }
        }
    }
}

pub(crate) fn insert_exported_schemes_into_imports_chirho(
    iface_chirho: &ModuleIfaceChirho,
    infer_result_chirho: &InferResultChirho,
    imported_types_chirho: &mut std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
) {
    for name_chirho in iface_chirho.exports_chirho.values_chirho.keys() {
        if let Some(scheme_chirho) = infer_result_chirho.env_chirho.lookup_chirho(name_chirho) {
            imported_types_chirho.insert(name_chirho.clone(), scheme_chirho.clone());
            imported_types_chirho.insert(
                format!("{}.{}", iface_chirho.name_chirho, name_chirho),
                scheme_chirho.clone(),
            );
        }
    }
    for ty_info_chirho in iface_chirho.exports_chirho.types_chirho.values() {
        for con_name_chirho in &ty_info_chirho.constructors_chirho {
            if let Some(scheme_chirho) = infer_result_chirho
                .env_chirho
                .lookup_chirho(con_name_chirho)
            {
                imported_types_chirho.insert(con_name_chirho.clone(), scheme_chirho.clone());
                imported_types_chirho.insert(
                    format!("{}.{}", iface_chirho.name_chirho, con_name_chirho),
                    scheme_chirho.clone(),
                );
            }
        }
        for method_name_chirho in &ty_info_chirho.methods_chirho {
            if let Some(scheme_chirho) = infer_result_chirho
                .env_chirho
                .lookup_chirho(method_name_chirho)
            {
                imported_types_chirho.insert(method_name_chirho.clone(), scheme_chirho.clone());
                imported_types_chirho.insert(
                    format!("{}.{}", iface_chirho.name_chirho, method_name_chirho),
                    scheme_chirho.clone(),
                );
            }
        }
    }
}

pub(crate) fn collect_frontend_artifacts_from_module_sources_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
    seed_stdlib_frontend_artifacts_chirho: bool,
) -> Result<FrontendSeedArtifactsChirho, String> {
    if module_sources_chirho.is_empty() {
        return Ok(FrontendSeedArtifactsChirho {
            ifaces_chirho: haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(
                haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
            ),
            imported_types_chirho: initial_imported_types_chirho,
            imported_type_synonyms_chirho: initial_imported_type_synonyms_chirho,
            imported_type_families_chirho: initial_imported_type_families_chirho,
        });
    }

    let mut dep_graph_chirho = haskelujah_incremental_chirho::DepGraphChirho::new_chirho();
    let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();

    for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
        let fp_chirho =
            haskelujah_incremental_chirho::FingerprintChirho::from_str_chirho(source_chirho);
        dep_graph_chirho.add_module_chirho(module_name_chirho, fp_chirho);
        for imported_chirho in extract_imports_chirho(source_chirho) {
            if known_modules_chirho.contains(&imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_chirho);
            }
        }
    }

    let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    // Add cross-package dependency interfaces
    ifaces_chirho.extend(extra_ifaces_chirho);
    let mut ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(ifaces_chirho);
    let mut imported_types_chirho = initial_imported_types_chirho;
    let mut imported_type_synonyms_chirho = initial_imported_type_synonyms_chirho;
    let mut imported_type_families_chirho = initial_imported_type_families_chirho;
    let mut imported_class_env_chirho =
        haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();
    if seed_stdlib_frontend_artifacts_chirho
        && module_sources_chirho
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
        for module_name_chirho in scc_chirho {
            let (_, file_name_chirho, source_chirho) = module_sources_chirho
                .iter()
                .find(|(name_chirho, _, _)| name_chirho == module_name_chirho)
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

            let frontend_result_chirho =
                run_frontend_with_type_synonyms_families_and_class_env_chirho(
                    source_chirho,
                    file_id_chirho,
                    &ifaces_chirho,
                    &filtered_imported_types_chirho,
                    &filtered_imported_type_synonyms_chirho,
                    &imported_type_families_chirho,
                    &imported_class_env_chirho,
                )
                .map_err(|e_chirho| {
                    format!("Error compiling {}: {}", module_name_chirho, e_chirho)
                })?;

            let FrontendResultChirho {
                module_chirho,
                infer_result_chirho,
                resolved_imported_type_synonyms_chirho,
                warnings_chirho: _warnings_chirho,
            } = frontend_result_chirho;

            let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);
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
            imported_class_env_chirho = infer_result_chirho.class_env_chirho.clone();
            ifaces_chirho.push(iface_chirho);
        }
    }

    Ok(FrontendSeedArtifactsChirho {
        ifaces_chirho,
        imported_types_chirho,
        imported_type_synonyms_chirho,
        imported_type_families_chirho,
    })
}

pub(crate) fn compile_module_sources_with_extra_ifaces_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
) -> Result<ProjectCompileResultChirho, String> {
    if module_sources_chirho.is_empty() {
        return Ok(ProjectCompileResultChirho {
            module_results_chirho: Vec::new(),
            compilation_order_chirho: Vec::new(),
            warnings_chirho: Vec::new(),
        });
    }

    let mut dep_graph_chirho = haskelujah_incremental_chirho::DepGraphChirho::new_chirho();
    let known_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(name_chirho, _, _)| name_chirho.clone())
        .collect();

    for (module_name_chirho, _, source_chirho) in &module_sources_chirho {
        let fp_chirho =
            haskelujah_incremental_chirho::FingerprintChirho::from_str_chirho(source_chirho);
        dep_graph_chirho.add_module_chirho(module_name_chirho, fp_chirho);
        for imported_chirho in extract_imports_chirho(source_chirho) {
            if known_modules_chirho.contains(&imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_chirho);
            }
        }
    }

    let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();
    let mut results_chirho: Vec<CompileResultChirho> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    ifaces_chirho.extend(extra_ifaces_chirho);
    let mut ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(ifaces_chirho);
    let mut all_warnings_chirho: Vec<String> = Vec::new();
    let mut imported_types_chirho = initial_imported_types_chirho;
    let mut imported_type_synonyms_chirho = initial_imported_type_synonyms_chirho;
    let mut imported_type_families_chirho = initial_imported_type_families_chirho;
    let mut imported_class_env_chirho =
        haskelujah_typing_chirho::class_chirho::ClassEnvChirho::new_chirho();
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
    let mut order_chirho: Vec<String> = Vec::new();

    for scc_chirho in &sccs_chirho {
        for module_name_chirho in scc_chirho {
            let (_, file_name_chirho, source_chirho) = module_sources_chirho
                .iter()
                .find(|(name_chirho, _, _)| name_chirho == module_name_chirho)
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

            let frontend_result_chirho =
                run_frontend_with_type_synonyms_families_and_class_env_chirho(
                    source_chirho,
                    file_id_chirho,
                    &ifaces_chirho,
                    &filtered_imported_types_chirho,
                    &filtered_imported_type_synonyms_chirho,
                    &imported_type_families_chirho,
                    &imported_class_env_chirho,
                )
                .map_err(|e_chirho| {
                    format!("Error compiling {}: {}", module_name_chirho, e_chirho)
                })?;

            let FrontendResultChirho {
                module_chirho,
                infer_result_chirho,
                resolved_imported_type_synonyms_chirho,
                warnings_chirho,
            } = frontend_result_chirho;

            all_warnings_chirho.extend(warnings_chirho);

            let iface_chirho = build_iface_with_imports_chirho(&module_chirho, &ifaces_chirho);
            let extra_dict_param_names_chirho: std::collections::HashSet<String> =
                imported_types_chirho.keys().cloned().collect();
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
            imported_class_env_chirho = infer_result_chirho.class_env_chirho.clone();
            ifaces_chirho.push(iface_chirho);

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
