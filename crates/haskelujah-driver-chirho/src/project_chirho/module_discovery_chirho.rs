// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which module files a Cabal package's components name (row 634), and which of them a build
//! requires (row 703).
//!
//! One probe per declared module looks for `<Module/Path>.{hs,lhs,hsc}` in the component's
//! `hs-source-dirs`. The found-only faces keep the original behavior, skipping what the probe
//! does not find, for option maps, best-effort dependency seeds and tests. The required faces
//! refuse a library's or an executable's declared module, or an executable's `main-is`, that no
//! source directory holds; a builtin module of the same name never satisfies it. Test suites and
//! benchmarks stay best effort. A package's own Cabal-generated `Paths_` and `PackageInfo_`
//! modules are not source obligations: install writes `Paths_`, and builds generate neither.

use std::path::{Path, PathBuf};

use haskelujah_package_chirho::generated_modules_chirho::{
    package_info_module_name_chirho, paths_module_name_chirho,
};

use super::dependency_packages_chirho::DependencyDemandChirho;

/// Discover Haskell module files from a package description.
///
/// Returns `(module_name, file_path)` pairs. Searches `hs-source-dirs`
/// for each exposed/other module, converting dotted module names to paths.
pub fn discover_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    collect_modules_chirho(package_chirho, project_dir_chirho, &mut Vec::new())
}

/// The package's library and executable modules for a build. Any declared one that is missing is
/// the error, by name.
/// workflow: development-chirho/package-source-install-chirho
pub(crate) fn require_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> Result<Vec<(String, PathBuf)>, String> {
    let mut missing_chirho = Vec::new();
    let modules_chirho =
        collect_modules_chirho(package_chirho, project_dir_chirho, &mut missing_chirho);
    required_chirho(modules_chirho, missing_chirho)
}

/// Every discovered module, recording each missing declared library or executable module in
/// `missing_chirho`.
fn collect_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
    missing_chirho: &mut Vec<String>,
) -> Vec<(String, PathBuf)> {
    let mut modules_chirho =
        collect_library_modules_chirho(package_chirho, project_dir_chirho, missing_chirho);
    let mut seen_chirho = modules_chirho
        .iter()
        .map(|(module_name_chirho, _path_chirho)| module_name_chirho.clone())
        .collect::<std::collections::HashSet<_>>();

    for exe_chirho in &package_chirho.executables_chirho {
        let src_dirs_chirho = if exe_chirho
            .build_info_chirho
            .hs_source_dirs_chirho
            .is_empty()
        {
            vec![".".to_string()]
        } else {
            exe_chirho.build_info_chirho.hs_source_dirs_chirho.clone()
        };
        let component_chirho = format!("executable {}", exe_chirho.name_chirho);

        // Compile other-modules BEFORE main-is so dependencies are available
        for mod_name_chirho in &exe_chirho.other_modules_chirho {
            if seen_chirho.insert(format!("{}:{}", exe_chirho.name_chirho, mod_name_chirho)) {
                match find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
                {
                    Some(path_chirho) => {
                        modules_chirho.push((mod_name_chirho.clone(), path_chirho))
                    }
                    None => record_missing_module_chirho(
                        missing_chirho,
                        &package_chirho.name_chirho,
                        &component_chirho,
                        mod_name_chirho,
                        &src_dirs_chirho,
                    ),
                }
            }
        }

        // Now add main-is AFTER other-modules
        if let Some(main_is_chirho) = &exe_chirho.main_is_chirho {
            let main_path_chirho = src_dirs_chirho
                .iter()
                .map(|d_chirho| project_dir_chirho.join(d_chirho).join(main_is_chirho))
                .find(|p_chirho| p_chirho.exists());

            if let Some(path_chirho) = main_path_chirho {
                let mod_name_chirho = "Main".to_string();
                if seen_chirho.insert(format!("{}:{}", exe_chirho.name_chirho, mod_name_chirho)) {
                    modules_chirho.push((mod_name_chirho, path_chirho));
                }
            } else {
                missing_chirho.push(format!(
                    "{component_chirho} lists main-is {main_is_chirho}, but no hs-source-dir holds it (searched [{}])",
                    src_dirs_chirho.join(", ")
                ));
            }
        }
    }

    // Test suites stay best effort: a default build does not select them.
    for test_chirho in &package_chirho.test_suites_chirho {
        let src_dirs_chirho = if test_chirho
            .build_info_chirho
            .hs_source_dirs_chirho
            .is_empty()
        {
            vec![".".to_string()]
        } else {
            test_chirho.build_info_chirho.hs_source_dirs_chirho.clone()
        };

        for mod_name_chirho in &test_chirho.other_modules_chirho {
            if seen_chirho.insert(format!(
                "test:{}:{}",
                test_chirho.name_chirho, mod_name_chirho
            )) && let Some(path_chirho) =
                find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
            {
                modules_chirho.push((mod_name_chirho.clone(), path_chirho));
            }
        }

        if let Some(main_is_chirho) = &test_chirho.main_is_chirho {
            let main_path_chirho = src_dirs_chirho
                .iter()
                .map(|d_chirho| project_dir_chirho.join(d_chirho).join(main_is_chirho))
                .find(|p_chirho| p_chirho.exists());

            if let Some(path_chirho) = main_path_chirho {
                let mod_name_chirho = "Main".to_string();
                if seen_chirho.insert(format!(
                    "test:{}:{}",
                    test_chirho.name_chirho, mod_name_chirho
                )) {
                    modules_chirho.push((mod_name_chirho, path_chirho));
                }
            }
        }
    }

    if package_chirho.library_chirho.is_none()
        && package_chirho.internal_libraries_chirho.is_empty()
        && package_chirho.executables_chirho.is_empty()
        && package_chirho.test_suites_chirho.is_empty()
        && package_chirho.benchmarks_chirho.is_empty()
    {
        for setup_file_chirho in ["Setup.hs", "Setup.lhs"] {
            let setup_path_chirho = project_dir_chirho.join(setup_file_chirho);
            if setup_path_chirho.exists() && seen_chirho.insert("setup:Setup".to_string()) {
                modules_chirho.push(("Setup".to_string(), setup_path_chirho));
                break;
            }
        }
    }

    modules_chirho
}

pub(crate) fn discover_library_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    collect_library_modules_chirho(package_chirho, project_dir_chirho, &mut Vec::new())
}

/// A dependency library's modules as the walk's demand allows: found-only for best-effort seeds,
/// and for a required dependency an error naming any missing declared module.
/// workflow: development-chirho/package-source-install-chirho
pub(crate) fn library_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
    demand_chirho: DependencyDemandChirho,
) -> Result<Vec<(String, PathBuf)>, String> {
    let mut missing_chirho = Vec::new();
    let modules_chirho =
        collect_library_modules_chirho(package_chirho, project_dir_chirho, &mut missing_chirho);
    match demand_chirho {
        DependencyDemandChirho::BestEffortChirho => Ok(modules_chirho),
        DependencyDemandChirho::RequiredChirho => required_chirho(modules_chirho, missing_chirho)
            .map_err(|error_chirho| {
                format!("package {}: {error_chirho}", package_chirho.name_chirho)
            }),
    }
}

fn collect_library_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
    missing_chirho: &mut Vec<String>,
) -> Vec<(String, PathBuf)> {
    let mut modules_chirho: Vec<(String, PathBuf)> = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();

    for library_chirho in iter_package_libraries_chirho(package_chirho) {
        let component_chirho = library_chirho.name_chirho.as_deref().map_or_else(
            || "library".to_string(),
            |name_chirho| format!("library {name_chirho}"),
        );
        let found_chirho = collect_modules_for_library_chirho(
            library_chirho,
            project_dir_chirho,
            &mut |module_chirho, src_dirs_chirho| {
                record_missing_module_chirho(
                    missing_chirho,
                    &package_chirho.name_chirho,
                    &component_chirho,
                    module_chirho,
                    src_dirs_chirho,
                )
            },
        );
        for (module_name_chirho, path_chirho) in found_chirho {
            if seen_chirho.insert(module_name_chirho.clone()) {
                modules_chirho.push((module_name_chirho, path_chirho));
            }
        }
    }

    modules_chirho
}

pub(crate) fn iter_package_libraries_chirho<'a>(
    package_chirho: &'a haskelujah_package_chirho::PackageDescChirho,
) -> impl Iterator<Item = &'a haskelujah_package_chirho::LibraryChirho> + 'a {
    package_chirho
        .library_chirho
        .iter()
        .chain(package_chirho.internal_libraries_chirho.iter())
}

pub(crate) fn is_self_or_internal_library_dep_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    dep_name_chirho: &str,
) -> bool {
    dep_name_chirho == package_chirho.name_chirho
        || package_chirho
            .internal_libraries_chirho
            .iter()
            .any(|library_chirho| library_chirho.name_chirho.as_deref() == Some(dep_name_chirho))
}

pub(crate) fn discover_modules_for_library_chirho(
    library_chirho: &haskelujah_package_chirho::LibraryChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    collect_modules_for_library_chirho(library_chirho, project_dir_chirho, &mut |_, _| {})
}

/// One library's exposed and other modules, in that order, calling `missing_chirho` with each
/// declared module the probe does not find and its source directories.
fn collect_modules_for_library_chirho(
    library_chirho: &haskelujah_package_chirho::LibraryChirho,
    project_dir_chirho: &Path,
    missing_chirho: &mut dyn FnMut(&str, &[String]),
) -> Vec<(String, PathBuf)> {
    let src_dirs_chirho = if library_chirho
        .build_info_chirho
        .hs_source_dirs_chirho
        .is_empty()
    {
        vec![".".to_string()]
    } else {
        library_chirho
            .build_info_chirho
            .hs_source_dirs_chirho
            .clone()
    };

    let mut modules_chirho = Vec::new();
    for mod_name_chirho in library_chirho
        .exposed_modules_chirho
        .iter()
        .chain(library_chirho.other_modules_chirho.iter())
    {
        match find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho) {
            Some(path_chirho) => modules_chirho.push((mod_name_chirho.clone(), path_chirho)),
            None => missing_chirho(mod_name_chirho, &src_dirs_chirho),
        }
    }

    modules_chirho
}

/// Record a declared module no source directory holds, unless it is one of the package's own
/// Cabal-generated modules, which are not source obligations.
fn record_missing_module_chirho(
    missing_chirho: &mut Vec<String>,
    package_name_chirho: &str,
    component_chirho: &str,
    module_chirho: &str,
    src_dirs_chirho: &[String],
) {
    if module_chirho == paths_module_name_chirho(package_name_chirho)
        || module_chirho == package_info_module_name_chirho(package_name_chirho)
    {
        return;
    }
    let relative_chirho = module_chirho.replace('.', "/");
    missing_chirho.push(format!(
        "{component_chirho} lists module {module_chirho}, but no {relative_chirho}.hs, \
         {relative_chirho}.lhs or {relative_chirho}.hsc exists under hs-source-dirs [{}]",
        src_dirs_chirho.join(", ")
    ));
}

/// The found modules, or one error naming every missing declared module.
fn required_chirho(
    modules_chirho: Vec<(String, PathBuf)>,
    missing_chirho: Vec<String>,
) -> Result<Vec<(String, PathBuf)>, String> {
    if missing_chirho.is_empty() {
        return Ok(modules_chirho);
    }
    Err(format!(
        "declared source modules are missing: {}; a builtin module of the same name does not \
         satisfy a declared module",
        missing_chirho.join("; ")
    ))
}

/// Find the file for a dotted module name (e.g. `Data.Map` → `Data/Map.hs`)
/// in the given source directories.
fn find_module_file_chirho(
    module_name_chirho: &str,
    src_dirs_chirho: &[String],
    project_dir_chirho: &Path,
) -> Option<PathBuf> {
    let module_rel_dir_chirho = module_name_chirho.replace('.', "/");

    for dir_chirho in src_dirs_chirho {
        for extension_chirho in ["hs", "lhs", "hsc"] {
            let candidate_path_chirho = project_dir_chirho
                .join(dir_chirho)
                .join(format!("{module_rel_dir_chirho}.{extension_chirho}"));
            if candidate_path_chirho.exists() {
                return Some(candidate_path_chirho);
            }
        }
    }

    None
}
