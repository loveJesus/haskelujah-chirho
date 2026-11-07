// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Local dependency packages: their interfaces and frontend artifacts (row 634).

use super::cabal_build_chirho::{
    collect_library_package_deps_chirho, package_module_cpp_options_map_chirho,
};
use super::module_discovery_chirho::discover_library_modules_chirho;
use super::package_install_chirho::find_cabal_in_dir_chirho;
use super::project_dir_chirho::{
    ProjectCompileResultChirho, extract_imports_chirho,
    filter_seeded_imported_types_for_source_chirho, filter_seeded_type_synonyms_for_source_chirho,
    source_imports_stdlib_chirho,
};
use super::{
    CompileResultChirho, FrontendResultChirho, FrontendSeedArtifactsChirho,
    ImportedTypeFamiliesChirho, ImportedTypeSynonymsChirho, compile_backend_chirho,
    exported_type_synonyms_from_module_chirho, merge_stdlib_frontend_artifacts_chirho,
    preprocess_cpp_source_with_options_chirho, read_haskell_source_file_chirho,
    run_frontend_with_type_synonyms_families_and_class_env_chirho,
    seed_builtin_type_families_chirho,
};
use haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;
use haskelujah_typing_chirho::infer_chirho::InferResultChirho;
use std::path::{Path, PathBuf};

#[path = "dependency_scan_chirho.rs"]
mod dependency_scan_chirho;
#[cfg(test)]
pub(crate) use dependency_scan_chirho::scan_dependency_package_ifaces_chirho;
pub(crate) use dependency_scan_chirho::{
    DependencyInterfaceSourcesChirho, FrontendInterfaceSeedsChirho,
};

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

/// How the dependency collector treats a demanded dependency it cannot find or compile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DependencyDemandChirho {
    /// Warn and continue without it, as builds have done (row 634).
    BestEffortChirho,
    /// Every demanded direct and transitive dependency must be found and must compile; the
    /// first failure is the error. Source install uses this, so nothing it records was skipped
    /// (row 703).
    RequiredChirho,
}

pub(crate) fn collect_local_dependency_frontend_artifacts_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
) -> Result<FrontendSeedArtifactsChirho, String> {
    collect_dependency_frontend_artifacts_with_demand_chirho(
        project_dir_chirho,
        root_deps_chirho,
        DependencyDemandChirho::BestEffortChirho,
        None,
    )
}

/// The collector in its strict mode: an error for any demanded dependency that is missing or
/// fails to compile, never a warning and a skip (row 703).
pub(crate) fn collect_required_dependency_frontend_artifacts_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
) -> Result<FrontendSeedArtifactsChirho, String> {
    collect_dependency_frontend_artifacts_with_demand_chirho(
        project_dir_chirho,
        root_deps_chirho,
        DependencyDemandChirho::RequiredChirho,
        None,
    )
}

/// The strict collector that also keeps every dependency package's Core, in dependency order
/// (each package after the packages it depends on), so an executable can link the bodies it
/// calls (row 703). Each package is compiled once for both its seeds and its Core.
pub(crate) fn collect_linked_dependency_artifacts_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
) -> Result<(FrontendSeedArtifactsChirho, Vec<DependencyCoreChirho>), String> {
    let mut cores_chirho = Vec::new();
    let artifacts_chirho = collect_dependency_frontend_artifacts_with_demand_chirho(
        project_dir_chirho,
        root_deps_chirho,
        DependencyDemandChirho::RequiredChirho,
        Some(&mut cores_chirho),
    )?;
    Ok((artifacts_chirho, cores_chirho))
}

fn collect_dependency_frontend_artifacts_with_demand_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
    demand_chirho: DependencyDemandChirho,
    mut core_sink_chirho: Option<&mut Vec<DependencyCoreChirho>>,
) -> Result<FrontendSeedArtifactsChirho, String> {
    let builtin_deps_chirho = builtin_dependency_names_chirho();
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        if demand_chirho == DependencyDemandChirho::RequiredChirho
            && let Some(dep_chirho) = root_deps_chirho
                .iter()
                .find(|dep_chirho| !builtin_deps_chirho.contains(&dep_chirho.package_chirho))
        {
            return Err(format!(
                "dependency {} needs a package store, and none was found above {}",
                dep_chirho.package_chirho,
                project_dir_chirho.display()
            ));
        }
        return Ok(FrontendSeedArtifactsChirho::default());
    };

    // Seed PrimState type family equations so dependency packages like
    // primitive can resolve PrimState (ST s) = s during compilation.
    let mut artifacts_chirho = FrontendSeedArtifactsChirho {
        interface_sources_chirho: None,
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
            demand_chirho,
            core_sink_chirho.as_deref_mut(),
            &mut visited_chirho,
            &mut active_chirho,
            &mut artifacts_chirho,
            &mut source_map_chirho,
        ) {
            if demand_chirho == DependencyDemandChirho::RequiredChirho {
                return Err(error_chirho);
            }
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
    demand_chirho: DependencyDemandChirho,
    mut core_sink_chirho: Option<&mut Vec<DependencyCoreChirho>>,
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
        return match demand_chirho {
            DependencyDemandChirho::BestEffortChirho => Ok(()),
            DependencyDemandChirho::RequiredChirho => Err(format!(
                "dependency {package_name_chirho} has no package directory in {}",
                packages_dir_chirho.display()
            )),
        };
    };

    let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) else {
        active_chirho.remove(package_name_chirho);
        return match demand_chirho {
            DependencyDemandChirho::BestEffortChirho => Ok(()),
            DependencyDemandChirho::RequiredChirho => Err(format!(
                "dependency {package_name_chirho} has no .cabal file in {}",
                package_dir_chirho.display()
            )),
        };
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
            demand_chirho,
            core_sink_chirho.as_deref_mut(),
            visited_chirho,
            active_chirho,
            artifacts_chirho,
            source_map_chirho,
        ) {
            if demand_chirho == DependencyDemandChirho::RequiredChirho {
                return Err(format!(
                    "{package_name_chirho} depends on {}: {error_chirho}",
                    dep_chirho.package_chirho
                ));
            }
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

    // With a sink, the packages compiled so far are linked program modules; they include every
    // package this one depends on, since dependencies finish first.
    let linked_modules_chirho: Option<std::collections::HashSet<String>> =
        core_sink_chirho.as_deref().map(|cores_chirho| {
            cores_chirho
                .iter()
                .flat_map(|core_chirho| core_chirho.module_names_chirho.iter().cloned())
                .collect()
        });
    let CompiledModuleSourcesChirho {
        artifacts_chirho: mut compiled_artifacts_chirho,
        compiled_chirho,
    } = compile_module_sources_once_chirho(
        module_sources_chirho,
        source_map_chirho,
        FrontendSeedArtifactsChirho {
            ifaces_chirho: artifacts_chirho.ifaces_chirho.clone(),
            interface_sources_chirho: artifacts_chirho.interface_sources_chirho.clone(),
            imported_types_chirho: artifacts_chirho.imported_types_chirho.clone(),
            imported_type_synonyms_chirho: artifacts_chirho.imported_type_synonyms_chirho.clone(),
            imported_type_families_chirho: artifacts_chirho.imported_type_families_chirho.clone(),
        },
        true,
        linked_modules_chirho.as_ref(),
    )
    .map_err(|error_chirho| {
        format!(
            "dependency package '{}': {}",
            package_name_chirho, error_chirho
        )
    })?;
    if let Some(cores_chirho) = core_sink_chirho.as_deref_mut() {
        cores_chirho.push(DependencyCoreChirho {
            package_name_chirho: package_name_chirho.to_string(),
            module_names_chirho: compiled_chirho.compilation_order_chirho,
            results_chirho: compiled_chirho.module_results_chirho,
        });
    }

    // Checked home exports become authoritative package exports. First merge
    // only the retained fragments, then replace each exact admitted exposure;
    // a fallback must not keep exports/proof absent from the checked source.
    let mut checked_ifaces_chirho = Vec::new();
    let mut retained_ifaces_chirho = Vec::new();
    for mut iface_chirho in std::mem::take(&mut compiled_artifacts_chirho.ifaces_chirho) {
        if iface_chirho.exposure_chirho
            == Some(haskelujah_naming_chirho::iface_chirho::IfaceExposureChirho::HomeChirho)
        {
            iface_chirho.exposure_chirho = (!package_chirho.name_chirho.is_empty()).then(|| haskelujah_naming_chirho::iface_chirho::IfaceExposureChirho::packages_from_producer_chirho(&[&package_chirho.name_chirho]));
            checked_ifaces_chirho.push(iface_chirho);
        } else {
            retained_ifaces_chirho.push(iface_chirho);
        }
    }
    let mut admitted_ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(retained_ifaces_chirho);
    for iface_chirho in checked_ifaces_chirho {
        crate::module_search_chirho::admit_authoritative_iface_chirho(
            &mut admitted_ifaces_chirho,
            iface_chirho,
        );
    }
    artifacts_chirho.interface_sources_chirho = compiled_artifacts_chirho.interface_sources_chirho;
    artifacts_chirho.ifaces_chirho = admitted_ifaces_chirho;
    artifacts_chirho
        .imported_types_chirho
        .extend(compiled_artifacts_chirho.imported_types_chirho);
    artifacts_chirho
        .imported_type_synonyms_chirho
        .extend(compiled_artifacts_chirho.imported_type_synonyms_chirho);
    // The compiled package result already contains the initial registry plus
    // every newly checked module. Replace that cumulative snapshot once.
    artifacts_chirho.imported_type_families_chirho =
        compiled_artifacts_chirho.imported_type_families_chirho;

    active_chirho.remove(package_name_chirho);
    visited_chirho.insert(package_name_chirho.to_string());
    Ok(())
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

/// What one compile of a module set produces (row 703).
struct CompiledModuleSourcesChirho {
    artifacts_chirho: FrontendSeedArtifactsChirho,
    compiled_chirho: ProjectCompileResultChirho,
}

/// One dependency package's compiled bodies, ready to link into an executable (row 703).
pub(crate) struct DependencyCoreChirho {
    pub(crate) package_name_chirho: String,
    /// Its modules, in compilation order.
    pub(crate) module_names_chirho: Vec<String>,
    pub(crate) results_chirho: Vec<CompileResultChirho>,
}

/// The typed seeds of a module set, without Core.
pub(crate) fn collect_frontend_artifacts_from_module_sources_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    extra_ifaces_chirho: impl Into<FrontendInterfaceSeedsChirho>,
    initial_imported_types_chirho: std::collections::HashMap<
        String,
        haskelujah_typing_chirho::ty_chirho::SchemeChirho,
    >,
    initial_imported_type_synonyms_chirho: ImportedTypeSynonymsChirho,
    initial_imported_type_families_chirho: ImportedTypeFamiliesChirho,
    seed_stdlib_frontend_artifacts_chirho: bool,
) -> Result<FrontendSeedArtifactsChirho, String> {
    let interface_seed_chirho = extra_ifaces_chirho.into();
    compile_module_sources_once_chirho(
        module_sources_chirho,
        source_map_chirho,
        FrontendSeedArtifactsChirho {
            ifaces_chirho: interface_seed_chirho.ifaces_chirho,
            interface_sources_chirho: interface_seed_chirho.interface_sources_chirho,
            imported_types_chirho: initial_imported_types_chirho,
            imported_type_synonyms_chirho: initial_imported_type_synonyms_chirho,
            imported_type_families_chirho: initial_imported_type_families_chirho,
        },
        seed_stdlib_frontend_artifacts_chirho,
        None,
    )
    .map(|compiled_chirho| compiled_chirho.artifacts_chirho)
}

/// Every module's Core, compiled so references to `linked_modules_chirho` (dependency modules
/// merged into the same executable) are program references (row 703).
pub(crate) fn compile_module_sources_linked_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    seed_chirho: FrontendSeedArtifactsChirho,
    linked_modules_chirho: &std::collections::HashSet<String>,
) -> Result<ProjectCompileResultChirho, String> {
    compile_module_sources_once_chirho(
        module_sources_chirho,
        source_map_chirho,
        seed_chirho,
        true,
        Some(linked_modules_chirho),
    )
    .map(|compiled_chirho| compiled_chirho.compiled_chirho)
}

/// The one SCC compile of a module set behind both public paths (rows 629, 703): it always
/// yields the typed seeds dependents import, and with a Core request also every module's Core,
/// compiled against the request's linked modules as program modules.
fn compile_module_sources_once_chirho(
    module_sources_chirho: Vec<(String, String, String)>,
    source_map_chirho: &mut SourceMapChirho,
    seed_chirho: FrontendSeedArtifactsChirho,
    seed_stdlib_frontend_artifacts_chirho: bool,
    core_request_chirho: Option<&std::collections::HashSet<String>>,
) -> Result<CompiledModuleSourcesChirho, String> {
    let FrontendSeedArtifactsChirho {
        ifaces_chirho: seed_ifaces_chirho,
        mut interface_sources_chirho,
        imported_types_chirho: initial_imported_types_chirho,
        imported_type_synonyms_chirho: initial_imported_type_synonyms_chirho,
        imported_type_families_chirho: initial_imported_type_families_chirho,
    } = seed_chirho;
    if let Some(sources_chirho) = &interface_sources_chirho {
        sources_chirho
            .validate_sources_chirho(&module_sources_chirho)
            .map_err(|errors_chirho| errors_chirho.to_string())?;
        interface_sources_chirho = Some(std::sync::Arc::new(
            sources_chirho.including_compiled_sources_chirho(&module_sources_chirho),
        ));
    }
    if module_sources_chirho.is_empty() {
        return Ok(CompiledModuleSourcesChirho {
            artifacts_chirho: FrontendSeedArtifactsChirho {
                interface_sources_chirho,
                ifaces_chirho: haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(
                    haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
                ),
                imported_types_chirho: initial_imported_types_chirho,
                imported_type_synonyms_chirho: initial_imported_type_synonyms_chirho,
                imported_type_families_chirho: initial_imported_type_families_chirho,
            },
            compiled_chirho: ProjectCompileResultChirho {
                module_results_chirho: Vec::new(),
                compilation_order_chirho: Vec::new(),
                warnings_chirho: Vec::new(),
            },
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
        for imported_chirho in super::project_dir_chirho::extract_home_imports_chirho(source_chirho)
        {
            if known_modules_chirho.contains(&imported_chirho) {
                dep_graph_chirho.add_dep_chirho(module_name_chirho, &imported_chirho);
            }
        }
    }

    let sccs_chirho = dep_graph_chirho.topo_sort_sccs_chirho();
    let mut results_chirho: Vec<CompileResultChirho> = Vec::new();
    let mut ifaces_chirho: Vec<ModuleIfaceChirho> =
        haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let mut selected_source_ifaces_chirho = Vec::new();
    for iface_chirho in seed_ifaces_chirho {
        if iface_chirho.exports_chirho.origin_chirho
            == haskelujah_naming_chirho::IfaceOriginChirho::SourceChirho
        {
            selected_source_ifaces_chirho.push(iface_chirho);
        } else {
            ifaces_chirho.push(iface_chirho);
        }
    }
    let mut ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(ifaces_chirho);
    // This producer holds selected source snapshots, not partial builtin
    // fragments. Authoritative replacement preserves unrelated package views.
    for iface_chirho in selected_source_ifaces_chirho {
        crate::module_search_chirho::admit_authoritative_iface_chirho(
            &mut ifaces_chirho,
            iface_chirho,
        );
    }
    let mut all_warnings_chirho: Vec<String> = Vec::new();
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
        )?;
    }
    let mut order_chirho: Vec<String> = Vec::new();

    // The modules this package compiles and links together (row 629).
    let mut program_modules_chirho: std::collections::HashSet<String> = module_sources_chirho
        .iter()
        .map(|(module_name_chirho, _, _)| module_name_chirho.clone())
        .collect();
    // Dependency modules merged into the same executable are program modules too (row 703).
    if let Some(linked_modules_chirho) = core_request_chirho {
        program_modules_chirho.extend(linked_modules_chirho.iter().cloned());
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
                warnings_chirho,
            } = frontend_result_chirho;

            if infer_result_chirho
                .non_deferrable_diagnostics_chirho
                .has_errors_chirho()
            {
                return Err(format!(
                    "Error compiling {file_name_chirho}: {}",
                    infer_result_chirho.non_deferrable_diagnostics_chirho
                ));
            }

            all_warnings_chirho.extend(warnings_chirho);

            let iface_chirho = super::build_family_aware_iface_chirho(
                &module_chirho,
                &ifaces_chirho,
                &infer_result_chirho.type_families_chirho,
            )
            .map_err(|error_chirho| {
                error_chirho.source_message_chirho(file_name_chirho, source_map_chirho)
            })?;
            let extra_dict_param_names_chirho: Option<std::collections::HashSet<String>> =
                core_request_chirho.map(|_| imported_types_chirho.keys().cloned().collect());
            insert_exported_schemes_into_imports_chirho(
                &iface_chirho,
                &infer_result_chirho,
                &mut imported_types_chirho,
            );
            imported_type_synonyms_chirho.extend(
                exported_type_synonyms_from_module_chirho(
                    &module_chirho,
                    &iface_chirho,
                    &resolved_imported_type_synonyms_chirho,
                    &infer_result_chirho,
                )
                .map_err(|error_chirho| format!("{file_name_chirho}: {error_chirho}"))?,
            );
            imported_type_families_chirho = infer_result_chirho.type_families_chirho.clone();
            imported_class_env_chirho = infer_result_chirho.class_env_chirho.try_snapshot_chirho().map_err(|error_chirho| format!("{file_name_chirho}: class registry snapshot resource limit reached: {error_chirho:?}"))?;
            crate::module_search_chirho::admit_home_iface_chirho(&mut ifaces_chirho, iface_chirho);

            if let Some(extra_dict_param_names_chirho) = extra_dict_param_names_chirho {
                let compile_result_chirho = compile_backend_chirho(
                    module_chirho,
                    infer_result_chirho,
                    extra_dict_param_names_chirho,
                    &ifaces_chirho,
                    &program_modules_chirho,
                )
                .map_err(|e_chirho| {
                    format!("Backend error for {}: {}", module_name_chirho, e_chirho)
                })?;

                results_chirho.push(compile_result_chirho);
                order_chirho.push(module_name_chirho.clone());
            }
        }
    }

    Ok(CompiledModuleSourcesChirho {
        artifacts_chirho: FrontendSeedArtifactsChirho {
            interface_sources_chirho,
            ifaces_chirho,
            imported_types_chirho,
            imported_type_synonyms_chirho,
            imported_type_families_chirho,
        },
        compiled_chirho: ProjectCompileResultChirho {
            module_results_chirho: results_chirho,
            compilation_order_chirho: order_chirho,
            warnings_chirho: all_warnings_chirho,
        },
    })
}
