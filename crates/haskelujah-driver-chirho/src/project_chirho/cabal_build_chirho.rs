// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A Cabal package's compile and executable build (row 634).

use super::core_merge_chirho::merge_compile_results_core_chirho;
use super::dependency_packages_chirho::{
    builtin_dependency_names_chirho, collect_local_dependency_frontend_artifacts_chirho,
    compile_module_sources_with_extra_ifaces_chirho, merge_local_dependency_package_index_chirho,
};
use super::module_discovery_chirho::{
    discover_modules_chirho, discover_modules_for_library_chirho,
    is_self_or_internal_library_dep_chirho, iter_package_libraries_chirho,
};
use super::project_dir_chirho::ProjectCompileResultChirho;
use super::{
    CompileResultChirho, ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho,
    preprocess_cpp_source_with_options_chirho, read_haskell_source_file_chirho,
};
use haskelujah_backend_llvm_chirho::try_compile_core_to_llvm_executable_chirho;
use haskelujah_core_chirho::CoreModuleChirho;
use haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho;
use haskelujah_span_chirho::SourceMapChirho;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Cabal project compilation
// ---------------------------------------------------------------------------

/// Result of compiling a Cabal-based project.
#[derive(Debug)]
pub struct CabalCompileResultChirho {
    /// The parsed package description.
    pub package_chirho: haskelujah_package_chirho::PackageDescChirho,
    /// The resolved build plan (external dependencies in topo order).
    pub build_plan_chirho: haskelujah_package_chirho::BuildPlanChirho,
    /// Per-module compilation results, in compilation order.
    pub module_results_chirho: Vec<CompileResultChirho>,
    /// Module names in compilation order.
    pub compilation_order_chirho: Vec<String>,
    /// Non-fatal warnings across all compiled modules.
    pub warnings_chirho: Vec<String>,
}

#[derive(Debug)]
pub struct CabalExecutableBuildResultChirho {
    pub name_chirho: String,
    pub core_chirho: CoreModuleChirho,
    pub llvm_ir_chirho: String,
    pub compilation_order_chirho: Vec<String>,
    pub warnings_chirho: Vec<String>,
}

#[derive(Debug)]
pub struct CabalBuildResultChirho {
    pub package_chirho: haskelujah_package_chirho::PackageDescChirho,
    pub build_plan_chirho: haskelujah_package_chirho::BuildPlanChirho,
    pub executables_chirho: Vec<CabalExecutableBuildResultChirho>,
}

/// Compile a Haskell project from a `.cabal` file path.
///
/// This function:
/// 1. Reads and parses the `.cabal` file
/// 2. Discovers exposed and internal modules from `hs-source-dirs`
/// 3. Resolves external dependencies against the provided package index
/// 4. Compiles all discovered modules in dependency order
pub fn compile_cabal_project_chirho(
    cabal_path_chirho: impl AsRef<Path>,
    index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<CabalCompileResultChirho, String> {
    use haskelujah_package_chirho::{parse_cabal_chirho, resolve_deps_chirho};

    // Read and parse the .cabal file.
    let cabal_content_chirho =
        std::fs::read_to_string(cabal_path_chirho.as_ref()).map_err(|e_chirho| {
            format!(
                "cannot read {}: {}",
                cabal_path_chirho.as_ref().display(),
                e_chirho
            )
        })?;
    let package_chirho = parse_cabal_chirho(&cabal_content_chirho);

    // Collect build-depends from the library (or all stanzas).
    let all_deps_chirho = collect_package_deps_chirho(&package_chirho);
    let project_dir_chirho = cabal_path_chirho
        .as_ref()
        .parent()
        .unwrap_or_else(|| Path::new("."));

    let builtins_chirho = builtin_dependency_names_chirho()
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    let merged_index_chirho = merge_local_dependency_package_index_chirho(
        project_dir_chirho,
        &all_deps_chirho,
        index_chirho,
    )?;

    // Resolve dependencies.
    let build_plan_chirho =
        resolve_deps_chirho(&all_deps_chirho, &merged_index_chirho, &builtins_chirho)
            .map_err(|e_chirho| format!("dependency resolution failed: {}", e_chirho))?;

    // Discover Haskell source files.
    let source_files_chirho = discover_modules_chirho(&package_chirho, project_dir_chirho);
    let module_cpp_options_chirho =
        package_module_cpp_options_map_chirho(&package_chirho, project_dir_chirho);

    let dep_frontend_artifacts_chirho =
        collect_local_dependency_frontend_artifacts_chirho(project_dir_chirho, &all_deps_chirho)?;

    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let project_compile_result_chirho = compile_module_files_with_frontend_seed_chirho(
        &source_files_chirho,
        &module_cpp_options_chirho,
        &mut source_map_chirho,
        dep_frontend_artifacts_chirho.ifaces_chirho.clone(),
        dep_frontend_artifacts_chirho.imported_types_chirho,
        dep_frontend_artifacts_chirho.imported_type_synonyms_chirho,
        dep_frontend_artifacts_chirho.imported_type_families_chirho,
    )?;

    Ok(CabalCompileResultChirho {
        package_chirho,
        build_plan_chirho,
        module_results_chirho: project_compile_result_chirho.module_results_chirho,
        compilation_order_chirho: project_compile_result_chirho.compilation_order_chirho,
        warnings_chirho: project_compile_result_chirho.warnings_chirho,
    })
}

pub fn build_cabal_project_chirho(
    cabal_path_chirho: impl AsRef<Path>,
    index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<CabalBuildResultChirho, String> {
    use haskelujah_package_chirho::{parse_cabal_chirho, resolve_deps_chirho};

    let cabal_content_chirho =
        std::fs::read_to_string(cabal_path_chirho.as_ref()).map_err(|e_chirho| {
            format!(
                "cannot read {}: {}",
                cabal_path_chirho.as_ref().display(),
                e_chirho
            )
        })?;
    let package_chirho = parse_cabal_chirho(&cabal_content_chirho);

    let all_deps_chirho = collect_package_deps_chirho(&package_chirho);
    let project_dir_chirho = cabal_path_chirho
        .as_ref()
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let builtins_chirho = builtin_dependency_names_chirho()
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    let merged_index_chirho = merge_local_dependency_package_index_chirho(
        project_dir_chirho,
        &all_deps_chirho,
        index_chirho,
    )?;
    let build_plan_chirho =
        resolve_deps_chirho(&all_deps_chirho, &merged_index_chirho, &builtins_chirho)
            .map_err(|e_chirho| format!("dependency resolution failed: {}", e_chirho))?;

    let mut executables_chirho = Vec::new();
    let dep_frontend_artifacts_chirho =
        collect_local_dependency_frontend_artifacts_chirho(project_dir_chirho, &all_deps_chirho)?;
    for executable_chirho in &package_chirho.executables_chirho {
        let target_modules_chirho = discover_executable_modules_chirho(
            &package_chirho,
            executable_chirho,
            project_dir_chirho,
        );
        let target_module_cpp_options_chirho = module_cpp_options_map_for_module_files_chirho(
            &target_modules_chirho,
            &executable_chirho.build_info_chirho.cpp_options_chirho,
        );
        let mut source_map_chirho = SourceMapChirho::new_chirho();
        let project_compile_result_chirho = compile_module_files_with_frontend_seed_chirho(
            &target_modules_chirho,
            &target_module_cpp_options_chirho,
            &mut source_map_chirho,
            dep_frontend_artifacts_chirho.ifaces_chirho.clone(),
            dep_frontend_artifacts_chirho.imported_types_chirho.clone(),
            dep_frontend_artifacts_chirho
                .imported_type_synonyms_chirho
                .clone(),
            dep_frontend_artifacts_chirho
                .imported_type_families_chirho
                .clone(),
        )?;
        let merged_core_chirho = merge_compile_results_core_chirho(
            &project_compile_result_chirho.module_results_chirho,
        )?;
        let llvm_ir_chirho = try_compile_core_to_llvm_executable_chirho(&merged_core_chirho)?;
        executables_chirho.push(CabalExecutableBuildResultChirho {
            name_chirho: executable_chirho.name_chirho.clone(),
            core_chirho: merged_core_chirho,
            llvm_ir_chirho,
            compilation_order_chirho: project_compile_result_chirho.compilation_order_chirho,
            warnings_chirho: project_compile_result_chirho.warnings_chirho,
        });
    }

    Ok(CabalBuildResultChirho {
        package_chirho,
        build_plan_chirho,
        executables_chirho,
    })
}

pub(crate) fn collect_library_package_deps_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
) -> Vec<haskelujah_package_chirho::DependencyChirho> {
    let mut deps_chirho = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();
    for lib_chirho in iter_package_libraries_chirho(package_chirho) {
        for dep_chirho in &lib_chirho.build_info_chirho.build_depends_chirho {
            if is_self_or_internal_library_dep_chirho(package_chirho, &dep_chirho.package_chirho) {
                continue;
            }
            if seen_chirho.insert(dep_chirho.package_chirho.clone()) {
                deps_chirho.push(dep_chirho.clone());
            }
        }
    }

    deps_chirho
}

/// Collect build-depends for the package targets we compile directly.
pub(crate) fn collect_package_deps_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
) -> Vec<haskelujah_package_chirho::DependencyChirho> {
    let mut deps_chirho = collect_library_package_deps_chirho(package_chirho);
    let mut seen_chirho = deps_chirho
        .iter()
        .map(|dep_chirho| dep_chirho.package_chirho.clone())
        .collect::<std::collections::HashSet<_>>();
    for exe_chirho in &package_chirho.executables_chirho {
        for dep_chirho in &exe_chirho.build_info_chirho.build_depends_chirho {
            if is_self_or_internal_library_dep_chirho(package_chirho, &dep_chirho.package_chirho) {
                continue;
            }
            if seen_chirho.insert(dep_chirho.package_chirho.clone()) {
                deps_chirho.push(dep_chirho.clone());
            }
        }
    }
    // Skip test-suite and benchmark dependencies here. They should not affect
    // package resolution for direct library/executable compilation, and they
    // especially must not leak into recursive local dependency indexing.

    deps_chirho
}

fn discover_executable_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    executable_chirho: &haskelujah_package_chirho::ExecutableChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    let mut target_package_chirho = package_chirho.clone();
    target_package_chirho.executables_chirho = vec![executable_chirho.clone()];
    target_package_chirho.test_suites_chirho.clear();
    target_package_chirho.benchmarks_chirho.clear();
    discover_modules_chirho(&target_package_chirho, project_dir_chirho)
}

fn module_cpp_options_map_for_module_files_chirho(
    module_files_chirho: &[(String, PathBuf)],
    cpp_options_chirho: &[String],
) -> std::collections::HashMap<PathBuf, Vec<String>> {
    let mut module_cpp_options_chirho = std::collections::HashMap::new();
    for (_, path_chirho) in module_files_chirho {
        module_cpp_options_chirho.insert(path_chirho.clone(), cpp_options_chirho.to_vec());
    }
    module_cpp_options_chirho
}

pub(crate) fn package_module_cpp_options_map_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> std::collections::HashMap<PathBuf, Vec<String>> {
    let mut module_cpp_options_chirho = std::collections::HashMap::new();

    for library_chirho in iter_package_libraries_chirho(package_chirho) {
        let library_modules_chirho =
            discover_modules_for_library_chirho(library_chirho, project_dir_chirho);
        for (_, path_chirho) in library_modules_chirho {
            module_cpp_options_chirho.insert(
                path_chirho,
                library_chirho.build_info_chirho.cpp_options_chirho.clone(),
            );
        }
    }

    for executable_chirho in &package_chirho.executables_chirho {
        let executable_modules_chirho = discover_executable_modules_chirho(
            package_chirho,
            executable_chirho,
            project_dir_chirho,
        );
        for (_, path_chirho) in executable_modules_chirho {
            module_cpp_options_chirho.insert(
                path_chirho,
                executable_chirho
                    .build_info_chirho
                    .cpp_options_chirho
                    .clone(),
            );
        }
    }

    module_cpp_options_chirho
}

fn compile_module_files_with_frontend_seed_chirho(
    module_files_chirho: &[(String, PathBuf)],
    module_cpp_options_chirho: &std::collections::HashMap<PathBuf, Vec<String>>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: Vec<ModuleIfaceChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
) -> Result<ProjectCompileResultChirho, String> {
    let mut module_sources_chirho: Vec<(String, String, String)> = Vec::new();
    for (module_name_chirho, path_chirho) in module_files_chirho {
        let raw_source_chirho =
            read_haskell_source_file_chirho(path_chirho).map_err(|e_chirho| {
                format!(
                    "cannot read module {} at {}: {}",
                    module_name_chirho,
                    path_chirho.display(),
                    e_chirho
                )
            })?;
        let cpp_options_for_module_chirho = module_cpp_options_chirho
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
                "cannot preprocess module {} at {}: {}",
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
    compile_module_sources_with_extra_ifaces_chirho(
        module_sources_chirho,
        source_map_chirho,
        extra_ifaces_chirho,
        initial_imported_types_chirho,
        initial_imported_type_synonyms_chirho,
        initial_imported_type_families_chirho,
    )
}
