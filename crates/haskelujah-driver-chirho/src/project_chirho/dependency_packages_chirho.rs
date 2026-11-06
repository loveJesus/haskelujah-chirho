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

/// A package directory in the store as the build index sees it: its name, its version (from its
/// .cabal file, else its directory suffix) and its library dependencies (row 703).
pub(crate) struct StorePackageChirho {
    pub(crate) name_chirho: String,
    pub(crate) version_chirho: haskelujah_package_chirho::VersionChirho,
    pub(crate) dir_chirho: PathBuf,
    pub(crate) dependencies_chirho: Vec<haskelujah_package_chirho::DependencyChirho>,
}

/// Every non-builtin package directory in the store with a readable .cabal file and a version,
/// in directory-name order. Anything else is left out, as the build index always has.
fn scan_store_packages_chirho(
    packages_dir_chirho: &Path,
) -> Result<Vec<StorePackageChirho>, String> {
    let builtin_deps_chirho = builtin_dependency_names_chirho();
    let mut package_dirs_chirho: Vec<PathBuf> = std::fs::read_dir(packages_dir_chirho)
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

    let mut store_packages_chirho = Vec::new();
    for package_dir_chirho in package_dirs_chirho {
        let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) else {
            continue;
        };
        let Ok(cabal_content_chirho) = std::fs::read_to_string(&cabal_path_chirho) else {
            continue;
        };
        let package_chirho = haskelujah_package_chirho::parse_cabal_chirho(&cabal_content_chirho);
        if package_chirho.name_chirho.is_empty()
            || builtin_deps_chirho.contains(&package_chirho.name_chirho)
        {
            continue;
        }
        let Some(version_chirho) = package_chirho.version_chirho.clone().or_else(|| {
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
        store_packages_chirho.push(StorePackageChirho {
            dependencies_chirho: collect_library_package_deps_chirho(&package_chirho),
            name_chirho: package_chirho.name_chirho,
            version_chirho,
            dir_chirho: package_dir_chirho,
        });
    }
    Ok(store_packages_chirho)
}

pub(crate) fn merge_local_dependency_package_index_chirho(
    project_dir_chirho: &Path,
    _root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
    base_index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<haskelujah_package_chirho::PackageIndexChirho, String> {
    Ok(merge_local_dependency_store_chirho(project_dir_chirho, base_index_chirho)?.0)
}

/// The build index with every store package added, and the store packages themselves, so a
/// strict build can map each planned version to the directory it was indexed from (row 703).
/// workflow: development-chirho/package-source-install-chirho
pub(crate) fn merge_local_dependency_store_chirho(
    project_dir_chirho: &Path,
    base_index_chirho: &haskelujah_package_chirho::PackageIndexChirho,
) -> Result<
    (
        haskelujah_package_chirho::PackageIndexChirho,
        Vec<StorePackageChirho>,
    ),
    String,
> {
    let Some(packages_dir_chirho) = find_dependency_packages_dir_chirho(project_dir_chirho) else {
        return Ok((base_index_chirho.clone(), Vec::new()));
    };
    let store_packages_chirho = scan_store_packages_chirho(&packages_dir_chirho)?;
    let mut merged_index_chirho = base_index_chirho.clone();
    for store_package_chirho in &store_packages_chirho {
        let already_present_chirho = merged_index_chirho
            .packages_chirho
            .get(&store_package_chirho.name_chirho)
            .is_some_and(|versions_chirho| {
                versions_chirho.iter().any(|meta_chirho| {
                    meta_chirho.version_chirho == store_package_chirho.version_chirho
                })
            });
        if !already_present_chirho {
            merged_index_chirho.add_package_chirho(
                &store_package_chirho.name_chirho,
                store_package_chirho.version_chirho.clone(),
                store_package_chirho.dependencies_chirho.clone(),
            );
        }
    }
    Ok((merged_index_chirho, store_packages_chirho))
}

/// The exact store packages an executable build links: each step of the strict build plan,
/// mapped to the one store directory that holds that name and version (row 703).
pub(crate) struct StoreSelectionChirho {
    packages_chirho: std::collections::HashMap<String, StorePackageChirho>,
}

impl StoreSelectionChirho {
    /// Map every planned package to its store directory. A version that no directory holds, or
    /// that two directories hold, is refused, since either would link a body the plan did not
    /// choose.
    /// workflow: development-chirho/package-source-install-chirho
    pub(crate) fn from_plan_chirho(
        plan_chirho: &haskelujah_package_chirho::BuildPlanChirho,
        store_packages_chirho: Vec<StorePackageChirho>,
    ) -> Result<Self, String> {
        let mut by_identity_chirho: std::collections::HashMap<
            (String, haskelujah_package_chirho::VersionChirho),
            Vec<StorePackageChirho>,
        > = std::collections::HashMap::new();
        for store_package_chirho in store_packages_chirho {
            by_identity_chirho
                .entry((
                    store_package_chirho.name_chirho.clone(),
                    store_package_chirho.version_chirho.clone(),
                ))
                .or_default()
                .push(store_package_chirho);
        }
        let mut packages_chirho = std::collections::HashMap::new();
        for step_chirho in &plan_chirho.steps_chirho {
            let found_chirho = by_identity_chirho
                .remove(&(
                    step_chirho.package_chirho.clone(),
                    step_chirho.version_chirho.clone(),
                ))
                .unwrap_or_default();
            if found_chirho.len() > 1 {
                return Err(format!(
                    "the build plan selects {} {}, and {} store directories hold that version: {}",
                    step_chirho.package_chirho,
                    step_chirho.version_chirho,
                    found_chirho.len(),
                    found_chirho
                        .iter()
                        .map(|store_package_chirho| store_package_chirho
                            .dir_chirho
                            .display()
                            .to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            let Some(store_package_chirho) = found_chirho.into_iter().next() else {
                return Err(format!(
                    "the build plan selects {} {}, but no store directory holds that version",
                    step_chirho.package_chirho, step_chirho.version_chirho
                ));
            };
            packages_chirho.insert(step_chirho.package_chirho.clone(), store_package_chirho);
        }
        Ok(Self { packages_chirho })
    }
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
        None,
    )
}

/// The strict collector that also keeps every dependency package's Core, in dependency order
/// (each package after the packages it depends on), so an executable can link the bodies it
/// calls (row 703). Each package is compiled once for both its seeds and its Core. It compiles
/// exactly the store directories `selection_chirho` planned, and refuses any reached bound that a
/// planned version does not satisfy.
/// workflow: development-chirho/package-source-install-chirho
pub(crate) fn collect_linked_dependency_artifacts_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
    selection_chirho: &StoreSelectionChirho,
) -> Result<(FrontendSeedArtifactsChirho, Vec<DependencyCoreChirho>), String> {
    let mut cores_chirho = Vec::new();
    let artifacts_chirho = collect_dependency_frontend_artifacts_with_demand_chirho(
        project_dir_chirho,
        root_deps_chirho,
        DependencyDemandChirho::RequiredChirho,
        Some(&mut cores_chirho),
        Some(selection_chirho),
    )?;
    Ok((artifacts_chirho, cores_chirho))
}

fn collect_dependency_frontend_artifacts_with_demand_chirho(
    project_dir_chirho: &Path,
    root_deps_chirho: &[haskelujah_package_chirho::DependencyChirho],
    demand_chirho: DependencyDemandChirho,
    core_sink_chirho: Option<&mut Vec<DependencyCoreChirho>>,
    selection_chirho: Option<&StoreSelectionChirho>,
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

    let mut walk_chirho = DependencyWalkChirho {
        packages_dir_chirho: &packages_dir_chirho,
        builtin_deps_chirho: &builtin_deps_chirho,
        demand_chirho,
        selection_chirho,
        core_sink_chirho,
        visited_chirho: std::collections::HashSet::new(),
        active_chirho: std::collections::HashSet::new(),
        // Seed PrimState type family equations so dependency packages like
        // primitive can resolve PrimState (ST s) = s during compilation.
        artifacts_chirho: FrontendSeedArtifactsChirho {
            interface_sources_chirho: None,
            imported_type_families_chirho: seed_builtin_type_families_chirho(),
            ..Default::default()
        },
        source_map_chirho: SourceMapChirho::new_chirho(),
    };
    for dep_chirho in root_deps_chirho {
        if let Err(error_chirho) = walk_chirho.compile_package_chirho(
            &dep_chirho.package_chirho,
            "the project",
            &dep_chirho.constraint_chirho,
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

    Ok(walk_chirho.artifacts_chirho)
}

/// One walk over the store's dependency graph: the store it reads, how it treats a failure, the
/// exact versions a strict build planned (if any), and what it has compiled so far
/// (rows 634, 703).
struct DependencyWalkChirho<'a> {
    packages_dir_chirho: &'a Path,
    builtin_deps_chirho: &'a std::collections::HashSet<String>,
    demand_chirho: DependencyDemandChirho,
    selection_chirho: Option<&'a StoreSelectionChirho>,
    core_sink_chirho: Option<&'a mut Vec<DependencyCoreChirho>>,
    visited_chirho: std::collections::HashSet<String>,
    active_chirho: std::collections::HashSet<String>,
    artifacts_chirho: FrontendSeedArtifactsChirho,
    source_map_chirho: SourceMapChirho,
}

impl DependencyWalkChirho<'_> {
    /// Compile one dependency package after the packages it depends on. With a selection, the
    /// reaching bound (`required_by_chirho` asks for `constraint_chirho`) must hold for the
    /// planned version, checked even when the package was compiled already, and the planned
    /// directory is the only one read.
    fn compile_package_chirho(
        &mut self,
        package_name_chirho: &str,
        required_by_chirho: &str,
        constraint_chirho: &haskelujah_package_chirho::VersionConstraintChirho,
    ) -> Result<(), String> {
        if self.builtin_deps_chirho.contains(package_name_chirho) {
            return Ok(());
        }
        let planned_dir_chirho = match self.selection_chirho {
            None => None,
            Some(selection_chirho) => {
                let Some(selected_chirho) =
                    selection_chirho.packages_chirho.get(package_name_chirho)
                else {
                    return Err(format!(
                        "{required_by_chirho} needs {package_name_chirho}, which the build plan does not select"
                    ));
                };
                if !constraint_chirho.satisfied_by_chirho(&selected_chirho.version_chirho) {
                    return Err(format!(
                        "{required_by_chirho} requires {package_name_chirho} {constraint_chirho}, but the build plan selects {}",
                        selected_chirho.version_chirho
                    ));
                }
                Some(selected_chirho.dir_chirho.clone())
            }
        };
        if self.visited_chirho.contains(package_name_chirho) {
            return Ok(());
        }
        if !self.active_chirho.insert(package_name_chirho.to_string()) {
            return Ok(());
        }

        let Some(package_dir_chirho) = planned_dir_chirho.or_else(|| {
            find_local_dependency_package_dir_chirho(self.packages_dir_chirho, package_name_chirho)
        }) else {
            self.active_chirho.remove(package_name_chirho);
            return match self.demand_chirho {
                DependencyDemandChirho::BestEffortChirho => Ok(()),
                DependencyDemandChirho::RequiredChirho => Err(format!(
                    "dependency {package_name_chirho} has no package directory in {}",
                    self.packages_dir_chirho.display()
                )),
            };
        };

        let Some(cabal_path_chirho) = find_cabal_in_dir_chirho(&package_dir_chirho) else {
            self.active_chirho.remove(package_name_chirho);
            return match self.demand_chirho {
                DependencyDemandChirho::BestEffortChirho => Ok(()),
                DependencyDemandChirho::RequiredChirho => Err(format!(
                    "dependency {package_name_chirho} has no .cabal file in {}",
                    package_dir_chirho.display()
                )),
            };
        };

        let cabal_content_chirho =
            std::fs::read_to_string(&cabal_path_chirho).map_err(|e_chirho| {
                format!(
                    "cannot read dependency cabal {}: {}",
                    cabal_path_chirho.display(),
                    e_chirho
                )
            })?;
        let package_chirho = haskelujah_package_chirho::parse_cabal_chirho(&cabal_content_chirho);

        for dep_chirho in collect_library_package_deps_chirho(&package_chirho) {
            if let Err(error_chirho) = self.compile_package_chirho(
                &dep_chirho.package_chirho,
                package_name_chirho,
                &dep_chirho.constraint_chirho,
            ) {
                if self.demand_chirho == DependencyDemandChirho::RequiredChirho {
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

        let source_files_chirho =
            discover_library_modules_chirho(&package_chirho, &package_dir_chirho);
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
            self.core_sink_chirho.as_deref().map(|cores_chirho| {
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
            &mut self.source_map_chirho,
            FrontendSeedArtifactsChirho {
                ifaces_chirho: self.artifacts_chirho.ifaces_chirho.clone(),
                interface_sources_chirho: self.artifacts_chirho.interface_sources_chirho.clone(),
                imported_types_chirho: self.artifacts_chirho.imported_types_chirho.clone(),
                imported_type_synonyms_chirho: self
                    .artifacts_chirho
                    .imported_type_synonyms_chirho
                    .clone(),
                imported_type_families_chirho: self
                    .artifacts_chirho
                    .imported_type_families_chirho
                    .clone(),
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
        if let Some(cores_chirho) = self.core_sink_chirho.as_deref_mut() {
            cores_chirho.push(DependencyCoreChirho {
                package_name_chirho: package_name_chirho.to_string(),
                module_names_chirho: compiled_chirho.compilation_order_chirho,
                results_chirho: compiled_chirho.module_results_chirho,
            });
        }

        for iface_chirho in &mut compiled_artifacts_chirho.ifaces_chirho {
            if iface_chirho.exposure_chirho
                == Some(haskelujah_naming_chirho::iface_chirho::IfaceExposureChirho::HomeChirho)
            {
                iface_chirho.exposure_chirho = (!package_chirho.name_chirho.is_empty()).then(|| haskelujah_naming_chirho::iface_chirho::IfaceExposureChirho::packages_from_producer_chirho(&[&package_chirho.name_chirho]));
            }
        }

        self.artifacts_chirho.interface_sources_chirho =
            compiled_artifacts_chirho.interface_sources_chirho;
        self.artifacts_chirho.ifaces_chirho =
            haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(
                compiled_artifacts_chirho.ifaces_chirho,
            );
        self.artifacts_chirho
            .imported_types_chirho
            .extend(compiled_artifacts_chirho.imported_types_chirho);
        self.artifacts_chirho
            .imported_type_synonyms_chirho
            .extend(compiled_artifacts_chirho.imported_type_synonyms_chirho);
        // The compiled package result already contains the initial registry plus
        // every newly checked module. Replace that cumulative snapshot once.
        self.artifacts_chirho.imported_type_families_chirho =
            compiled_artifacts_chirho.imported_type_families_chirho;

        self.active_chirho.remove(package_name_chirho);
        self.visited_chirho.insert(package_name_chirho.to_string());
        Ok(())
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
    ifaces_chirho.extend(seed_ifaces_chirho);
    let mut ifaces_chirho =
        haskelujah_naming_chirho::iface_chirho::merge_module_ifaces_chirho(ifaces_chirho);
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
