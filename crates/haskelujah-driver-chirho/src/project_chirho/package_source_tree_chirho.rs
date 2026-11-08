// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Staging a source package: a bounded, link-free, source-only copy (row 703).
//!
//! Every install input becomes the same thing before any check runs: a fresh staging directory
//! holding only the package's regular files and directories. A top-level `dist-chirho` build
//! output is left out and reported. Links, special files, names a database line cannot hold and
//! a nested package store are refused, and the copy is bounded like a release archive. Packages
//! whose build would run Setup code are refused too, because source install runs none.

use haskelujah_package_chirho::hackage_chirho::{
    MAX_ARCHIVE_BYTES_CHIRHO, MAX_ARCHIVE_ENTRIES_CHIRHO, extract_tarball_bounded_chirho,
};
use haskelujah_package_chirho::source_db_chirho::is_recordable_relative_path_chirho;
use haskelujah_package_chirho::{
    PackageDescChirho, SOURCE_STORE_DIR_NAME_CHIRHO, SourceInstallInputChirho, parse_cabal_chirho,
};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The build output a source copy leaves out.
pub(super) const EXCLUDED_BUILD_DIR_CHIRHO: &str = "dist-chirho";

/// A staged source package.
pub(super) struct StagedSourceChirho {
    /// The staged package directory.
    pub(super) package_dir_chirho: PathBuf,
    /// What the copy left out, relative to the package directory.
    pub(super) excluded_paths_chirho: Vec<PathBuf>,
    /// For an archive, the name of its single top-level directory.
    pub(super) archive_root_name_chirho: Option<String>,
}

fn io_message_chirho(action_chirho: &str, path_chirho: &Path, error_chirho: io::Error) -> String {
    format!(
        "cannot {action_chirho} {}: {error_chirho}",
        path_chirho.display()
    )
}

/// Stage `input_chirho` as `<staging_dir_chirho>/package-chirho`.
/// workflow: development-chirho/package-source-install-chirho
pub(super) fn stage_source_chirho(
    input_chirho: &SourceInstallInputChirho,
    staging_dir_chirho: &Path,
) -> Result<StagedSourceChirho, String> {
    let package_dir_chirho = staging_dir_chirho.join("package-chirho");
    match input_chirho {
        SourceInstallInputChirho::DirectoryChirho(source_dir_chirho) => {
            let excluded_paths_chirho =
                copy_source_tree_chirho(source_dir_chirho, &package_dir_chirho)?;
            Ok(StagedSourceChirho {
                package_dir_chirho,
                excluded_paths_chirho,
                archive_root_name_chirho: None,
            })
        }
        SourceInstallInputChirho::TarballChirho(tarball_chirho) => {
            let archive_dir_chirho = staging_dir_chirho.join("archive-chirho");
            fs::create_dir(&archive_dir_chirho).map_err(|error_chirho| {
                io_message_chirho("create", &archive_dir_chirho, error_chirho)
            })?;
            extract_tarball_bounded_chirho(tarball_chirho, &archive_dir_chirho).map_err(
                |error_chirho| format!("cannot extract the release archive: {error_chirho}"),
            )?;
            let root_name_chirho = single_archive_root_chirho(&archive_dir_chirho)?;
            let excluded_paths_chirho = copy_source_tree_chirho(
                &archive_dir_chirho.join(&root_name_chirho),
                &package_dir_chirho,
            )?;
            Ok(StagedSourceChirho {
                package_dir_chirho,
                excluded_paths_chirho,
                archive_root_name_chirho: Some(root_name_chirho),
            })
        }
    }
}

/// The name of an extracted archive's only top-level entry, which must be a directory.
fn single_archive_root_chirho(archive_dir_chirho: &Path) -> Result<String, String> {
    let mut names_chirho = Vec::new();
    for entry_chirho in fs::read_dir(archive_dir_chirho)
        .map_err(|error_chirho| io_message_chirho("list", archive_dir_chirho, error_chirho))?
    {
        let entry_chirho = entry_chirho
            .map_err(|error_chirho| io_message_chirho("list", archive_dir_chirho, error_chirho))?;
        names_chirho.push(entry_chirho.file_name());
    }
    let [root_chirho] = names_chirho.as_slice() else {
        return Err(format!(
            "a release archive must hold exactly one top-level directory; this one holds {} entries",
            names_chirho.len()
        ));
    };
    let root_name_chirho = root_chirho
        .to_str()
        .ok_or("the release archive's top-level directory name is not UTF-8")?
        .to_string();
    let metadata_chirho = fs::symlink_metadata(archive_dir_chirho.join(&root_name_chirho))
        .map_err(|error_chirho| io_message_chirho("inspect", archive_dir_chirho, error_chirho))?;
    if !metadata_chirho.is_dir() {
        return Err(format!(
            "the release archive's top-level entry {root_name_chirho} is not a directory"
        ));
    }
    Ok(root_name_chirho)
}

/// How large a staged source tree may grow: every entry, directories included, and the bytes of
/// its files.
#[derive(Debug, Clone, Copy)]
struct SourceTreeLimitsChirho {
    entries_chirho: usize,
    bytes_chirho: u64,
}

/// The limits a release archive has, applied to every install input.
const SOURCE_TREE_LIMITS_CHIRHO: SourceTreeLimitsChirho = SourceTreeLimitsChirho {
    entries_chirho: MAX_ARCHIVE_ENTRIES_CHIRHO,
    bytes_chirho: MAX_ARCHIVE_BYTES_CHIRHO,
};

/// Copy the package tree at `source_dir_chirho` into the new directory `dest_dir_chirho`,
/// returning what was left out. Only the root may be reached through a link the caller named.
fn copy_source_tree_chirho(
    source_dir_chirho: &Path,
    dest_dir_chirho: &Path,
) -> Result<Vec<PathBuf>, String> {
    copy_source_tree_within_chirho(
        source_dir_chirho,
        dest_dir_chirho,
        SOURCE_TREE_LIMITS_CHIRHO,
    )
}

/// The copy under explicit limits. Each entry counts as it is listed, before any directory is
/// held in memory whole, so neither many files nor many directories pass the bound.
fn copy_source_tree_within_chirho(
    source_dir_chirho: &Path,
    dest_dir_chirho: &Path,
    limits_chirho: SourceTreeLimitsChirho,
) -> Result<Vec<PathBuf>, String> {
    let root_metadata_chirho = fs::metadata(source_dir_chirho)
        .map_err(|error_chirho| io_message_chirho("read", source_dir_chirho, error_chirho))?;
    if !root_metadata_chirho.is_dir() {
        return Err(format!(
            "{} is not a directory",
            source_dir_chirho.display()
        ));
    }
    fs::create_dir(dest_dir_chirho)
        .map_err(|error_chirho| io_message_chirho("create", dest_dir_chirho, error_chirho))?;
    let mut excluded_paths_chirho = Vec::new();
    let mut entry_count_chirho = 0usize;
    let mut byte_count_chirho = 0u64;
    let over_limit_chirho = || {
        format!(
            "the package exceeds {} entries or {} bytes",
            limits_chirho.entries_chirho, limits_chirho.bytes_chirho
        )
    };
    let mut pending_dirs_chirho = vec![PathBuf::new()];
    while let Some(relative_dir_chirho) = pending_dirs_chirho.pop() {
        let listed_dir_chirho = source_dir_chirho.join(&relative_dir_chirho);
        let mut entries_chirho = Vec::new();
        for entry_chirho in fs::read_dir(&listed_dir_chirho)
            .map_err(|error_chirho| io_message_chirho("list", &listed_dir_chirho, error_chirho))?
        {
            let entry_chirho = entry_chirho.map_err(|error_chirho| {
                io_message_chirho("list", &listed_dir_chirho, error_chirho)
            })?;
            entry_count_chirho += 1;
            if entry_count_chirho > limits_chirho.entries_chirho {
                return Err(over_limit_chirho());
            }
            entries_chirho.push((entry_chirho.file_name(), entry_chirho.path()));
        }
        entries_chirho.sort();
        let at_root_chirho = relative_dir_chirho.as_os_str().is_empty();
        for (file_name_chirho, source_path_chirho) in entries_chirho {
            let relative_path_chirho = relative_dir_chirho.join(&file_name_chirho);
            if !is_recordable_relative_path_chirho(&relative_path_chirho) {
                return Err(format!(
                    "{} has a name that is not UTF-8 or holds a line break",
                    source_path_chirho.display()
                ));
            }
            let metadata_chirho =
                fs::symlink_metadata(&source_path_chirho).map_err(|error_chirho| {
                    io_message_chirho("inspect", &source_path_chirho, error_chirho)
                })?;
            let file_type_chirho = metadata_chirho.file_type();
            if at_root_chirho
                && file_name_chirho.as_os_str() == OsStr::new(SOURCE_STORE_DIR_NAME_CHIRHO)
            {
                return Err(format!(
                    "the package contains {SOURCE_STORE_DIR_NAME_CHIRHO}, which would redirect its dependency lookup"
                ));
            }
            if at_root_chirho
                && file_name_chirho.as_os_str() == OsStr::new(EXCLUDED_BUILD_DIR_CHIRHO)
                && file_type_chirho.is_dir()
            {
                excluded_paths_chirho.push(relative_path_chirho);
                continue;
            }
            let dest_path_chirho = dest_dir_chirho.join(&relative_path_chirho);
            if file_type_chirho.is_dir() {
                fs::create_dir(&dest_path_chirho).map_err(|error_chirho| {
                    io_message_chirho("create", &dest_path_chirho, error_chirho)
                })?;
                pending_dirs_chirho.push(relative_path_chirho);
            } else if file_type_chirho.is_file() {
                byte_count_chirho = byte_count_chirho.saturating_add(metadata_chirho.len());
                if byte_count_chirho > limits_chirho.bytes_chirho {
                    return Err(over_limit_chirho());
                }
                fs::copy(&source_path_chirho, &dest_path_chirho).map_err(|error_chirho| {
                    io_message_chirho("copy", &source_path_chirho, error_chirho)
                })?;
            } else {
                return Err(format!(
                    "{} is a link or special file; a source package may hold only files and directories",
                    relative_path_chirho.display()
                ));
            }
        }
    }
    Ok(excluded_paths_chirho)
}

/// The package's only `.cabal` file, relative to `package_dir_chirho`, and its description.
pub(super) fn read_single_cabal_file_chirho(
    package_dir_chirho: &Path,
) -> Result<(PathBuf, PackageDescChirho), String> {
    let mut cabal_names_chirho = Vec::new();
    for entry_chirho in fs::read_dir(package_dir_chirho)
        .map_err(|error_chirho| io_message_chirho("list", package_dir_chirho, error_chirho))?
    {
        let entry_chirho = entry_chirho
            .map_err(|error_chirho| io_message_chirho("list", package_dir_chirho, error_chirho))?;
        let path_chirho = entry_chirho.path();
        if path_chirho.extension() == Some(OsStr::new("cabal")) && path_chirho.is_file() {
            cabal_names_chirho.push(entry_chirho.file_name());
        }
    }
    cabal_names_chirho.sort();
    let [cabal_name_chirho] = cabal_names_chirho.as_slice() else {
        return Err(if cabal_names_chirho.is_empty() {
            "the package root holds no .cabal file".to_string()
        } else {
            format!(
                "the package root holds several .cabal files: {}",
                cabal_names_chirho
                    .iter()
                    .map(|name_chirho| name_chirho.to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        });
    };
    let cabal_path_chirho = package_dir_chirho.join(cabal_name_chirho);
    let cabal_text_chirho = fs::read_to_string(&cabal_path_chirho)
        .map_err(|error_chirho| io_message_chirho("read", &cabal_path_chirho, error_chirho))?;
    Ok((
        PathBuf::from(cabal_name_chirho),
        parse_cabal_chirho(&cabal_text_chirho),
    ))
}

/// Refuse a package whose build would run code before compiling: a configure or make step,
/// or a Setup script other than Cabal's default `main = defaultMain`.
pub(super) fn ensure_runs_no_setup_code_chirho(
    package_chirho: &PackageDescChirho,
    package_dir_chirho: &Path,
) -> Result<(), String> {
    match package_chirho.build_type_chirho.as_deref().map(str::trim) {
        Some(build_type_chirho) if build_type_chirho.eq_ignore_ascii_case("Simple") => Ok(()),
        None => ensure_default_setup_script_chirho(package_dir_chirho, false),
        Some(build_type_chirho) if build_type_chirho.eq_ignore_ascii_case("Custom") => {
            ensure_default_setup_script_chirho(package_dir_chirho, true)
        }
        Some(build_type_chirho) => Err(format!(
            "build-type {build_type_chirho} runs a configure or make step, which source install does not run"
        )),
    }
}

fn ensure_default_setup_script_chirho(
    package_dir_chirho: &Path,
    setup_required_chirho: bool,
) -> Result<(), String> {
    for (file_name_chirho, literate_chirho) in [("Setup.hs", false), ("Setup.lhs", true)] {
        let setup_path_chirho = package_dir_chirho.join(file_name_chirho);
        match fs::read_to_string(&setup_path_chirho) {
            Ok(text_chirho) if is_default_setup_script_chirho(&text_chirho, literate_chirho) => {
                return Ok(());
            }
            Ok(_) => {
                return Err(format!(
                    "{file_name_chirho} is a custom Setup script; source install runs no Setup code"
                ));
            }
            Err(error_chirho) if error_chirho.kind() == io::ErrorKind::NotFound => {}
            Err(error_chirho) => {
                return Err(io_message_chirho("read", &setup_path_chirho, error_chirho));
            }
        }
    }
    if setup_required_chirho {
        Err("build-type Custom needs a Setup script, and the package has none".to_string())
    } else {
        Ok(())
    }
}

/// Whether a Setup script is Cabal's default one: `main = defaultMain` from
/// `Distribution.Simple`, with nothing else but comments, pragmas and a `Main` header.
fn is_default_setup_script_chirho(text_chirho: &str, literate_chirho: bool) -> bool {
    let mut has_default_main_chirho = false;
    let mut in_block_comment_chirho = false;
    for raw_line_chirho in text_chirho.lines() {
        let code_chirho = if literate_chirho {
            match raw_line_chirho.strip_prefix('>') {
                Some(code_chirho) => code_chirho,
                None => continue,
            }
        } else {
            raw_line_chirho
        };
        let code_chirho = code_chirho.split("--").next().unwrap_or_default();
        let normalized_chirho = code_chirho.split_whitespace().collect::<Vec<_>>().join(" ");
        if in_block_comment_chirho {
            in_block_comment_chirho = !normalized_chirho.ends_with("-}");
            continue;
        }
        if normalized_chirho.is_empty() || normalized_chirho.starts_with("#!") {
            continue;
        }
        if normalized_chirho.starts_with("{-") {
            in_block_comment_chirho = !normalized_chirho.ends_with("-}");
            continue;
        }
        match normalized_chirho.as_str() {
            "main = defaultMain" => has_default_main_chirho = true,
            "import Distribution.Simple"
            | "import Distribution.Simple (defaultMain)"
            | "main :: IO ()"
            | "module Main where"
            | "module Main (main) where" => {}
            _ => return false,
        }
    }
    has_default_main_chirho
}

#[cfg(test)]
#[path = "package_source_tree_tests_chirho.rs"]
mod tests_chirho;
