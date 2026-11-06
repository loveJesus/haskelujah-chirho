// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `haskelujah install`: fetch or read a source package and install it checked (row 703).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// `haskelujah install` — install a checked source package into a package store.
///
/// `install <package> [version] [--store <dir>]` fetches the release archive from Hackage, the
/// command's only network step, and installs it. `install --from <dir|archive.tar.gz>
/// [--store <dir>]` installs a local source without the network. Either way the library must
/// compile to Core before anything is published, and no interface or object file is installed:
/// builds compile the published source. The store defaults to `./.haskelujah-packages-chirho`.
pub(crate) fn install_command_chirho(
    program_name_chirho: &str,
    positional_chirho: &[&str],
) -> ExitCode {
    let usage_chirho = format!(
        "usage: {program_name_chirho} install (<package> [version] | --from <dir|archive.tar.gz>) [--store <dir>]"
    );
    let mut from_chirho: Option<&str> = None;
    let mut store_chirho: Option<&str> = None;
    let mut names_chirho: Vec<&str> = Vec::new();
    let mut args_chirho = positional_chirho.iter().skip(1).copied();
    while let Some(arg_chirho) = args_chirho.next() {
        match arg_chirho {
            "--from" | "--store" => {
                let Some(value_chirho) = args_chirho.next() else {
                    eprintln!("`{arg_chirho}` needs a value");
                    eprintln!("{usage_chirho}");
                    return ExitCode::from(2);
                };
                let slot_chirho = if arg_chirho == "--from" {
                    &mut from_chirho
                } else {
                    &mut store_chirho
                };
                if slot_chirho.replace(value_chirho).is_some() {
                    eprintln!("`{arg_chirho}` was given twice");
                    return ExitCode::from(2);
                }
            }
            option_chirho if option_chirho.starts_with("--") => {
                eprintln!("unknown install option `{option_chirho}`");
                eprintln!("{usage_chirho}");
                return ExitCode::from(2);
            }
            name_chirho => names_chirho.push(name_chirho),
        }
    }

    let input_chirho = match (from_chirho, names_chirho.as_slice()) {
        (Some(from_path_chirho), []) => local_install_input_chirho(Path::new(from_path_chirho)),
        (None, [package_chirho]) => fetch_install_input_chirho(package_chirho, None),
        (None, [package_chirho, version_text_chirho]) => {
            fetch_install_input_chirho(package_chirho, Some(*version_text_chirho))
        }
        _ => {
            eprintln!("{usage_chirho}");
            return ExitCode::from(2);
        }
    };
    let input_chirho = match input_chirho {
        Ok(input_chirho) => input_chirho,
        Err(error_chirho) => {
            eprintln!("error: {error_chirho}");
            eprintln!("nothing was installed");
            return ExitCode::from(1);
        }
    };

    let store_root_chirho = PathBuf::from(
        store_chirho.unwrap_or(haskelujah_package_chirho::SOURCE_STORE_DIR_NAME_CHIRHO),
    );
    match haskelujah_driver_chirho::install_package_chirho(&input_chirho, &store_root_chirho) {
        Ok(result_chirho) => {
            report_source_install_chirho(&result_chirho);
            ExitCode::SUCCESS
        }
        Err(error_chirho) => {
            eprintln!("error: {}", error_chirho.message_chirho);
            match &error_chirho.store_state_chirho {
                haskelujah_package_chirho::StoreStateAfterFailureChirho::UnchangedChirho => {
                    eprintln!(
                        "nothing was installed: no package directory was published and the source package database is unchanged"
                    );
                }
                haskelujah_package_chirho::StoreStateAfterFailureChirho::UnrecordedDirectoryChirho(
                    dir_chirho,
                ) => {
                    eprintln!(
                        "the store now holds {} without a record; remove it before installing again",
                        dir_chirho.display()
                    );
                }
            }
            ExitCode::from(1)
        }
    }
}

/// The largest local release archive `install --from` reads into memory.
const MAX_LOCAL_ARCHIVE_BYTES_CHIRHO: u64 = 256 * 1024 * 1024;

/// A local install source: a package directory, or a release archive file read into memory.
fn local_install_input_chirho(
    path_chirho: &Path,
) -> Result<haskelujah_package_chirho::SourceInstallInputChirho, String> {
    let metadata_chirho = fs::metadata(path_chirho)
        .map_err(|error_chirho| format!("cannot read {}: {error_chirho}", path_chirho.display()))?;
    if metadata_chirho.is_dir() {
        return Ok(
            haskelujah_package_chirho::SourceInstallInputChirho::DirectoryChirho(
                path_chirho.to_path_buf(),
            ),
        );
    }
    if metadata_chirho.len() > MAX_LOCAL_ARCHIVE_BYTES_CHIRHO {
        return Err(format!(
            "{} is larger than {MAX_LOCAL_ARCHIVE_BYTES_CHIRHO} bytes",
            path_chirho.display()
        ));
    }
    let bytes_chirho = fs::read(path_chirho)
        .map_err(|error_chirho| format!("cannot read {}: {error_chirho}", path_chirho.display()))?;
    Ok(haskelujah_package_chirho::SourceInstallInputChirho::TarballChirho(bytes_chirho))
}

/// Fetch a release archive from Hackage: the only network step of `install`.
fn fetch_install_input_chirho(
    package_chirho: &str,
    version_text_chirho: Option<&str>,
) -> Result<haskelujah_package_chirho::SourceInstallInputChirho, String> {
    let version_chirho = match version_text_chirho {
        Some(version_text_chirho) => {
            haskelujah_package_chirho::parse_version_chirho(version_text_chirho)
                .ok_or_else(|| format!("invalid version `{version_text_chirho}`"))?
        }
        None => {
            eprintln!("Looking up the preferred version of {package_chirho} on Hackage...");
            haskelujah_package_chirho::hackage_chirho::fetch_latest_version_chirho(package_chirho)
                .map_err(|error_chirho| {
                format!("cannot look up the preferred version of {package_chirho}: {error_chirho}")
            })?
        }
    };
    eprintln!(
        "Fetching {}",
        haskelujah_package_chirho::hackage_tarball_url_chirho(package_chirho, &version_chirho)
    );
    let bytes_chirho = haskelujah_package_chirho::hackage_chirho::download_tarball_chirho(
        package_chirho,
        &version_chirho,
    )
    .map_err(|error_chirho| {
        format!("cannot fetch {package_chirho}-{version_chirho}: {error_chirho}")
    })?;
    Ok(haskelujah_package_chirho::SourceInstallInputChirho::TarballChirho(bytes_chirho))
}

fn report_source_install_chirho(result_chirho: &haskelujah_driver_chirho::InstallResultChirho) {
    let record_chirho = &result_chirho.record_chirho;
    eprintln!(
        "Installed source package {} in {}",
        record_chirho.directory_name_chirho,
        result_chirho.package_dir_chirho.display()
    );
    let generated_chirho: Vec<&str> = record_chirho
        .modules_chirho
        .iter()
        .filter(|module_chirho| {
            module_chirho.origin_chirho
                == haskelujah_package_chirho::SourceModuleOriginChirho::GeneratedChirho
        })
        .map(|module_chirho| module_chirho.module_name_chirho.as_str())
        .collect();
    eprintln!(
        "  checked: {} library modules compiled to Core{}",
        result_chirho.checked_module_count_chirho,
        if generated_chirho.is_empty() {
            String::new()
        } else {
            format!(
                ", generated by Cabal's contract: {}",
                generated_chirho.join(", ")
            )
        }
    );
    let dependencies_chirho: Vec<String> = record_chirho
        .dependencies_chirho
        .iter()
        .map(|dependency_chirho| match dependency_chirho {
            haskelujah_package_chirho::ResolvedDependencyChirho::BuiltinChirho(name_chirho) => {
                format!("{name_chirho} (builtin)")
            }
            haskelujah_package_chirho::ResolvedDependencyChirho::StoreChirho(
                name_chirho,
                version_chirho,
            ) => format!("{name_chirho}-{version_chirho}"),
        })
        .collect();
    eprintln!(
        "  dependencies: {}",
        if dependencies_chirho.is_empty() {
            "none".to_string()
        } else {
            dependencies_chirho.join(", ")
        }
    );
    for excluded_chirho in &result_chirho.excluded_paths_chirho {
        eprintln!("  left out build output: {}", excluded_chirho.display());
    }
    eprintln!(
        "  recorded in {} as {}",
        result_chirho.db_path_chirho.display(),
        record_chirho.capability_chirho
    );
    eprintln!(
        "  no interface or object files are installed; builds compile this package from source"
    );
}
