// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which module files a Cabal package's components name (row 634).

use std::path::{Path, PathBuf};

/// Discover Haskell module files from a package description.
///
/// Returns `(module_name, file_path)` pairs. Searches `hs-source-dirs`
/// for each exposed/other module, converting dotted module names to paths.
pub fn discover_modules_chirho(
    package_chirho: &haskelujah_package_chirho::PackageDescChirho,
    project_dir_chirho: &Path,
) -> Vec<(String, PathBuf)> {
    let mut modules_chirho = discover_library_modules_chirho(package_chirho, project_dir_chirho);
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

        // Compile other-modules BEFORE main-is so dependencies are available
        for mod_name_chirho in &exe_chirho.other_modules_chirho {
            if seen_chirho.insert(format!("{}:{}", exe_chirho.name_chirho, mod_name_chirho))
                && let Some(path_chirho) =
                    find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
            {
                modules_chirho.push((mod_name_chirho.clone(), path_chirho));
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
            }
        }
    }

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
    let mut modules_chirho: Vec<(String, PathBuf)> = Vec::new();
    let mut seen_chirho = std::collections::HashSet::new();

    for library_chirho in iter_package_libraries_chirho(package_chirho) {
        for (module_name_chirho, path_chirho) in
            discover_modules_for_library_chirho(library_chirho, project_dir_chirho)
        {
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
        if let Some(path_chirho) =
            find_module_file_chirho(mod_name_chirho, &src_dirs_chirho, project_dir_chirho)
        {
            modules_chirho.push((mod_name_chirho.clone(), path_chirho));
        }
    }

    modules_chirho
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
