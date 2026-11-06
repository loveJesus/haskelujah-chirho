// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Installing a source package: stage, check, publish, record (rows 634, 703).
//!
//! Install consumes a local source, a directory or the bytes of a release archive; fetching a
//! release from Hackage is the caller's separate step. Holding the store lock, it stages a
//! source-only copy, requires every listed library module (writing Cabal's `Paths_` module when
//! one is listed), resolves each library dependency to a builtin or to the exact source package
//! the store records, and compiles the whole library to Core against those dependencies. Only
//! then is the package directory published with one rename and its record with an atomic
//! database replace. Nothing installs or records an interface or object file: builds compile
//! the published source.
//!
//! Recovery boundary: until the directory rename nothing visible changes, and the next install
//! sweeps any staging left by a crash. If the database save fails after the rename, the directory
//! is moved back. A crash between the two renames leaves a package directory without a record,
//! which install refuses to replace until it is removed.

#[path = "package_source_tree_chirho.rs"]
mod package_source_tree_chirho;

use super::cabal_build_chirho::{
    collect_library_package_deps_chirho, compile_module_files_with_frontend_seed_chirho,
    package_module_cpp_options_map_chirho,
};
use super::dependency_packages_chirho::{
    collect_local_dependency_frontend_artifacts_chirho, find_local_dependency_package_dir_chirho,
};
use super::module_discovery_chirho::{
    discover_library_modules_chirho, discover_modules_for_library_chirho,
    iter_package_libraries_chirho,
};
use haskelujah_package_chirho::generated_modules_chirho::{
    is_safe_package_name_chirho, package_info_module_name_chirho, paths_module_name_chirho,
    render_paths_module_chirho,
};
use haskelujah_package_chirho::source_db_chirho::is_recordable_relative_path_chirho;
use haskelujah_package_chirho::{
    CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO, DependencyChirho, PackageDescChirho,
    ResolvedDependencyChirho, SourceInstallErrorChirho, SourceInstallInputChirho,
    SourceModuleChirho, SourceModuleOriginChirho, SourcePackageDbChirho, SourcePackageRecordChirho,
    SourceStoreChirho, StoreLockChirho, StoreStateAfterFailureChirho, VersionChirho,
    is_builtin_package_chirho,
};
use haskelujah_span_chirho::SourceMapChirho;
use package_source_tree_chirho::{
    ensure_runs_no_setup_code_chirho, read_single_cabal_file_chirho, stage_source_chirho,
};
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

/// What a successful install published and recorded.
#[derive(Debug)]
pub struct InstallResultChirho {
    /// The store's record for the package.
    pub record_chirho: SourcePackageRecordChirho,
    /// The published package directory.
    pub package_dir_chirho: PathBuf,
    /// The source package database holding the record.
    pub db_path_chirho: PathBuf,
    /// Library modules compiled to Core: every listed one.
    pub checked_module_count_chirho: usize,
    /// Build output the copy left out, relative to the package directory.
    pub excluded_paths_chirho: Vec<PathBuf>,
    /// Compiler warnings from the check.
    pub compile_warnings_chirho: Vec<String>,
}

fn unchanged_chirho(message_chirho: String) -> SourceInstallErrorChirho {
    SourceInstallErrorChirho::unchanged_chirho(message_chirho)
}

/// Install the source package `input_chirho` into the store at `store_root_chirho`.
///
/// On success the package directory is published and recorded. On failure the error says what
/// the store holds: unchanged, or (only when moving a published directory back failed) a package
/// directory without a record.
/// workflow: development-chirho/package-source-install-chirho
pub fn install_package_chirho(
    input_chirho: &SourceInstallInputChirho,
    store_root_chirho: &Path,
) -> Result<InstallResultChirho, SourceInstallErrorChirho> {
    let store_chirho =
        SourceStoreChirho::open_chirho(store_root_chirho).map_err(unchanged_chirho)?;
    let lock_chirho = store_chirho.lock_chirho().map_err(unchanged_chirho)?;
    let db_chirho = store_chirho.load_db_chirho().map_err(unchanged_chirho)?;
    store_chirho
        .sweep_staging_chirho(&lock_chirho)
        .map_err(unchanged_chirho)?;
    let staging_dir_chirho = store_chirho
        .new_staging_dir_chirho(&lock_chirho)
        .map_err(unchanged_chirho)?;
    let outcome_chirho = install_staged_chirho(
        input_chirho,
        &store_chirho,
        &lock_chirho,
        db_chirho,
        &staging_dir_chirho,
    );
    // A leftover that cannot be removed now is swept by the next install.
    let _ = std::fs::remove_dir_all(&staging_dir_chirho);
    outcome_chirho
}

fn install_staged_chirho(
    input_chirho: &SourceInstallInputChirho,
    store_chirho: &SourceStoreChirho,
    lock_chirho: &StoreLockChirho,
    db_chirho: SourcePackageDbChirho,
    staging_dir_chirho: &Path,
) -> Result<InstallResultChirho, SourceInstallErrorChirho> {
    let staged_chirho =
        stage_source_chirho(input_chirho, staging_dir_chirho).map_err(unchanged_chirho)?;
    let package_dir_chirho = &staged_chirho.package_dir_chirho;
    let (cabal_file_chirho, package_chirho) =
        read_single_cabal_file_chirho(package_dir_chirho).map_err(unchanged_chirho)?;
    let name_chirho = package_chirho.name_chirho.clone();
    if !is_safe_package_name_chirho(&name_chirho) {
        return Err(unchanged_chirho(format!(
            "the .cabal file names package `{name_chirho}`, which is not a valid package name"
        )));
    }
    let version_chirho = package_chirho.version_chirho.clone().ok_or_else(|| {
        unchanged_chirho(format!("the .cabal file for {name_chirho} has no version"))
    })?;
    let directory_name_chirho = format!("{name_chirho}-{version_chirho}");
    if let Some(root_name_chirho) = &staged_chirho.archive_root_name_chirho
        && *root_name_chirho != directory_name_chirho
    {
        return Err(unchanged_chirho(format!(
            "the archive's top-level directory is {root_name_chirho}, but its .cabal file describes {directory_name_chirho}"
        )));
    }
    ensure_runs_no_setup_code_chirho(&package_chirho, package_dir_chirho)
        .map_err(unchanged_chirho)?;
    if package_chirho.library_chirho.is_none() {
        return Err(unchanged_chirho(format!(
            "{directory_name_chirho} has no library; source install publishes libraries for other packages to build against"
        )));
    }
    ensure_new_package_name_chirho(
        &name_chirho,
        &directory_name_chirho,
        store_chirho,
        &db_chirho,
    )
    .map_err(unchanged_chirho)?;
    let (library_deps_chirho, dependencies_chirho) =
        resolve_recorded_dependencies_chirho(&package_chirho, store_chirho, &db_chirho)
            .map_err(unchanged_chirho)?;
    let modules_chirho =
        inventory_library_modules_chirho(&package_chirho, &version_chirho, package_dir_chirho)
            .map_err(unchanged_chirho)?;
    let (checked_module_count_chirho, compile_warnings_chirho) = check_library_compiles_chirho(
        &package_chirho,
        package_dir_chirho,
        &library_deps_chirho,
        modules_chirho.len(),
    )
    .map_err(unchanged_chirho)?;

    let record_chirho = SourcePackageRecordChirho {
        name_chirho,
        version_chirho,
        directory_name_chirho: directory_name_chirho.clone(),
        cabal_file_chirho,
        capability_chirho: CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO.to_string(),
        modules_chirho,
        dependencies_chirho,
    };
    let mut next_db_chirho = db_chirho;
    next_db_chirho
        .insert_new_chirho(record_chirho.clone())
        .map_err(unchanged_chirho)?;
    let target_dir_chirho = store_chirho.package_dir_chirho(&directory_name_chirho);
    std::fs::rename(package_dir_chirho, &target_dir_chirho).map_err(|error_chirho| {
        unchanged_chirho(format!(
            "cannot publish {}: {error_chirho}",
            target_dir_chirho.display()
        ))
    })?;
    if let Err(save_error_chirho) = store_chirho.save_db_chirho(&next_db_chirho, lock_chirho) {
        return Err(move_publication_back_chirho(
            &target_dir_chirho,
            package_dir_chirho,
            format!("cannot save the source package database: {save_error_chirho}"),
        ));
    }
    Ok(InstallResultChirho {
        record_chirho,
        package_dir_chirho: target_dir_chirho,
        db_path_chirho: store_chirho.db_path_chirho(),
        checked_module_count_chirho,
        excluded_paths_chirho: staged_chirho.excluded_paths_chirho,
        compile_warnings_chirho,
    })
}

/// Undo a publication whose record could not be saved.
fn move_publication_back_chirho(
    target_dir_chirho: &Path,
    staged_dir_chirho: &Path,
    reason_chirho: String,
) -> SourceInstallErrorChirho {
    match std::fs::rename(target_dir_chirho, staged_dir_chirho) {
        Ok(()) => unchanged_chirho(format!(
            "{reason_chirho}; the published directory was moved back"
        )),
        Err(error_chirho) => SourceInstallErrorChirho {
            message_chirho: format!(
                "{reason_chirho}; moving {} back failed: {error_chirho}",
                target_dir_chirho.display()
            ),
            store_state_chirho: StoreStateAfterFailureChirho::UnrecordedDirectoryChirho(
                target_dir_chirho.to_path_buf(),
            ),
        },
    }
}

/// Refuse a package name the store already holds in any form: builds select one directory per
/// name, so a second version or a replacement would change what recorded dependents compile.
fn ensure_new_package_name_chirho(
    name_chirho: &str,
    directory_name_chirho: &str,
    store_chirho: &SourceStoreChirho,
    db_chirho: &SourcePackageDbChirho,
) -> Result<(), String> {
    if let Some(record_chirho) = db_chirho.get_chirho(name_chirho) {
        return Err(format!(
            "this store already records {}; a second version or a reinstall is not supported",
            record_chirho.directory_name_chirho
        ));
    }
    if let Some(existing_chirho) =
        find_local_dependency_package_dir_chirho(store_chirho.root_chirho(), name_chirho)
    {
        return Err(format!(
            "{} already exists without a record (an interrupted install or an unrecorded copy); builds select one directory per package name, so remove it before installing {name_chirho}",
            existing_chirho.display()
        ));
    }
    let target_dir_chirho = store_chirho.package_dir_chirho(directory_name_chirho);
    if std::fs::symlink_metadata(&target_dir_chirho).is_ok() {
        return Err(format!("{} already exists", target_dir_chirho.display()));
    }
    Ok(())
}

/// Resolve each library dependency to a builtin or to the exact recorded source package that
/// builds will select for its name.
fn resolve_recorded_dependencies_chirho(
    package_chirho: &PackageDescChirho,
    store_chirho: &SourceStoreChirho,
    db_chirho: &SourcePackageDbChirho,
) -> Result<(Vec<DependencyChirho>, Vec<ResolvedDependencyChirho>), String> {
    let library_deps_chirho = collect_library_package_deps_chirho(package_chirho);
    let mut resolved_chirho = Vec::new();
    for dependency_chirho in &library_deps_chirho {
        let name_chirho = &dependency_chirho.package_chirho;
        if is_builtin_package_chirho(name_chirho) {
            resolved_chirho.push(ResolvedDependencyChirho::BuiltinChirho(name_chirho.clone()));
            continue;
        }
        let record_chirho = db_chirho.get_chirho(name_chirho).ok_or_else(|| {
            format!(
                "dependency {name_chirho} {} is not a source package recorded in this store; install it first",
                dependency_chirho.constraint_chirho
            )
        })?;
        if !dependency_chirho
            .constraint_chirho
            .satisfied_by_chirho(&record_chirho.version_chirho)
        {
            return Err(format!(
                "this store records {}, which does not satisfy {name_chirho} {}",
                record_chirho.directory_name_chirho, dependency_chirho.constraint_chirho
            ));
        }
        let selected_chirho =
            find_local_dependency_package_dir_chirho(store_chirho.root_chirho(), name_chirho);
        if selected_chirho.as_deref().and_then(Path::file_name)
            != Some(OsStr::new(&record_chirho.directory_name_chirho))
        {
            return Err(format!(
                "builds would compile {} for {name_chirho}, not the recorded {}",
                selected_chirho.map_or_else(
                    || "nothing".to_string(),
                    |path_chirho| path_chirho.display().to_string()
                ),
                record_chirho.directory_name_chirho
            ));
        }
        resolved_chirho.push(ResolvedDependencyChirho::StoreChirho(
            name_chirho.clone(),
            record_chirho.version_chirho.clone(),
        ));
    }
    Ok((library_deps_chirho, resolved_chirho))
}

/// A module file's path relative to the package directory, refusing one that lies outside it.
fn package_relative_path_chirho(
    package_dir_chirho: &Path,
    path_chirho: &Path,
) -> Result<PathBuf, String> {
    let relative_chirho: PathBuf = path_chirho
        .strip_prefix(package_dir_chirho)
        .map_err(|_| format!("{} lies outside the package", path_chirho.display()))?
        .components()
        .filter(|component_chirho| !matches!(component_chirho, Component::CurDir))
        .collect();
    if is_recordable_relative_path_chirho(&relative_chirho) {
        Ok(relative_chirho)
    } else {
        Err(format!(
            "{} lies outside the package",
            path_chirho.display()
        ))
    }
}

/// Every listed library module, found exactly once. A listed `Paths_<pkg>` without a shipped
/// file is written from Cabal's contract into the listing library's first source directory,
/// where builds find it too; a listed `PackageInfo_<pkg>` is refused; any other missing module
/// is an error.
fn inventory_library_modules_chirho(
    package_chirho: &PackageDescChirho,
    version_chirho: &VersionChirho,
    package_dir_chirho: &Path,
) -> Result<Vec<SourceModuleChirho>, String> {
    let paths_module_chirho = paths_module_name_chirho(&package_chirho.name_chirho);
    let package_info_module_chirho = package_info_module_name_chirho(&package_chirho.name_chirho);
    let mut modules_chirho = Vec::new();
    let mut seen_chirho = std::collections::BTreeSet::new();
    let mut missing_chirho = Vec::new();
    for library_chirho in iter_package_libraries_chirho(package_chirho) {
        let found_chirho: std::collections::HashMap<String, PathBuf> =
            discover_modules_for_library_chirho(library_chirho, package_dir_chirho)
                .into_iter()
                .collect();
        let listed_chirho = library_chirho
            .exposed_modules_chirho
            .iter()
            .map(|module_name_chirho| (module_name_chirho, true))
            .chain(
                library_chirho
                    .other_modules_chirho
                    .iter()
                    .map(|module_name_chirho| (module_name_chirho, false)),
            );
        for (module_name_chirho, exposed_chirho) in listed_chirho {
            if !seen_chirho.insert(module_name_chirho.clone()) {
                continue;
            }
            if let Some(path_chirho) = found_chirho.get(module_name_chirho) {
                modules_chirho.push(SourceModuleChirho {
                    module_name_chirho: module_name_chirho.clone(),
                    exposed_chirho,
                    origin_chirho: SourceModuleOriginChirho::ShippedChirho,
                    relative_path_chirho: package_relative_path_chirho(
                        package_dir_chirho,
                        path_chirho,
                    )?,
                });
            } else if *module_name_chirho == paths_module_chirho {
                let source_dir_chirho = library_chirho
                    .build_info_chirho
                    .hs_source_dirs_chirho
                    .first()
                    .map_or(".", String::as_str);
                let file_path_chirho = package_dir_chirho
                    .join(source_dir_chirho)
                    .join(format!("{module_name_chirho}.hs"));
                let relative_path_chirho =
                    package_relative_path_chirho(package_dir_chirho, &file_path_chirho)?;
                let text_chirho =
                    render_paths_module_chirho(&package_chirho.name_chirho, version_chirho)?;
                if let Some(parent_chirho) = file_path_chirho.parent() {
                    std::fs::create_dir_all(parent_chirho).map_err(|error_chirho| {
                        format!("cannot create {}: {error_chirho}", parent_chirho.display())
                    })?;
                }
                std::fs::write(&file_path_chirho, text_chirho).map_err(|error_chirho| {
                    format!(
                        "cannot write {}: {error_chirho}",
                        file_path_chirho.display()
                    )
                })?;
                modules_chirho.push(SourceModuleChirho {
                    module_name_chirho: module_name_chirho.clone(),
                    exposed_chirho,
                    origin_chirho: SourceModuleOriginChirho::GeneratedChirho,
                    relative_path_chirho,
                });
            } else if *module_name_chirho == package_info_module_chirho {
                return Err(format!(
                    "{module_name_chirho} is Cabal's generated PackageInfo_ module, which source install does not generate"
                ));
            } else {
                missing_chirho.push(module_name_chirho.clone());
            }
        }
    }
    if !missing_chirho.is_empty() {
        return Err(format!(
            "listed library modules have no source file in their hs-source-dirs: {}",
            missing_chirho.join(", ")
        ));
    }
    Ok(modules_chirho)
}

/// Compile every library module to Core against the recorded dependencies, discovered the way
/// builds discover them, and count the modules actually compiled.
fn check_library_compiles_chirho(
    package_chirho: &PackageDescChirho,
    package_dir_chirho: &Path,
    library_deps_chirho: &[DependencyChirho],
    listed_module_count_chirho: usize,
) -> Result<(usize, Vec<String>), String> {
    let module_files_chirho = discover_library_modules_chirho(package_chirho, package_dir_chirho);
    if module_files_chirho.len() != listed_module_count_chirho {
        return Err(format!(
            "builds would find {} library module files, but the package lists {listed_module_count_chirho}",
            module_files_chirho.len()
        ));
    }
    let module_cpp_options_chirho =
        package_module_cpp_options_map_chirho(package_chirho, package_dir_chirho);
    let seeds_chirho = collect_local_dependency_frontend_artifacts_chirho(
        package_dir_chirho,
        library_deps_chirho,
    )?;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let compiled_chirho = compile_module_files_with_frontend_seed_chirho(
        &module_files_chirho,
        &module_cpp_options_chirho,
        &mut source_map_chirho,
        seeds_chirho.ifaces_chirho,
        seeds_chirho.imported_types_chirho,
        seeds_chirho.imported_type_synonyms_chirho,
        seeds_chirho.imported_type_families_chirho,
    )
    .map_err(|error_chirho| format!("the library does not compile: {error_chirho}"))?;
    let compiled_count_chirho = compiled_chirho.module_results_chirho.len();
    if compiled_count_chirho != module_files_chirho.len() {
        return Err(format!(
            "compiled {compiled_count_chirho} of {} library modules",
            module_files_chirho.len()
        ));
    }
    Ok((compiled_count_chirho, compiled_chirho.warnings_chirho))
}

/// Find any `.cabal` file in a directory.
pub(crate) fn find_cabal_in_dir_chirho(dir_chirho: &Path) -> Option<PathBuf> {
    if let Ok(entries_chirho) = std::fs::read_dir(dir_chirho) {
        for entry_chirho in entries_chirho.flatten() {
            if entry_chirho
                .path()
                .extension()
                .is_some_and(|ext_chirho| ext_chirho == "cabal")
            {
                return Some(entry_chirho.path());
            }
        }
    }
    None
}
