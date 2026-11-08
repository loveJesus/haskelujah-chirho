// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The source package database: what a store holds and what install verified (row 703).
//!
//! A source package is the package's own source tree published under
//! `<store>/<name>-<version>/`, plus any module Cabal would generate for it. Each record says
//! which modules were compiled to Core when the package was installed and which exact
//! dependency each was checked against. No record names an interface or object file, because a
//! source install produces none; builds compile the recorded source.
//!
//! The text format is strict and versioned. Parsing refuses anything it does not recognise, so a
//! damaged database is an error rather than a smaller database.
//!
//! ```text
//! haskelujah-source-package-db-chirho 1
//! package haskell-say 1.0.0.0
//! directory haskell-say-1.0.0.0
//! cabal haskell-say.cabal
//! capability core-checked-source
//! module exposed shipped HaskellSay src/HaskellSay.hs
//! module other generated Paths_haskell_say src/Paths_haskell_say.hs
//! depends builtin base
//! end
//! ```

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::generated_modules_chirho::is_safe_package_name_chirho;
use crate::version_chirho::{VersionChirho, parse_version_chirho};

/// The first line of every source package database.
pub const SOURCE_DB_HEADER_CHIRHO: &str = "haskelujah-source-package-db-chirho 1";

/// The capability install records: every listed library module was compiled to Core.
pub const CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO: &str = "core-checked-source";

/// Where a recorded module's source came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceModuleOriginChirho {
    /// The package shipped the file.
    ShippedChirho,
    /// Install wrote it from Cabal's generated-module contract.
    GeneratedChirho,
}

/// One library module of a recorded source package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceModuleChirho {
    pub module_name_chirho: String,
    /// Listed in `exposed-modules` (otherwise `other-modules`).
    pub exposed_chirho: bool,
    pub origin_chirho: SourceModuleOriginChirho,
    /// The module's file, relative to the package directory.
    pub relative_path_chirho: PathBuf,
}

/// A library dependency as install resolved and checked it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedDependencyChirho {
    /// Provided by the compiler itself; no version bound is checked.
    BuiltinChirho(String),
    /// A source package recorded in the same store, at exactly this version.
    StoreChirho(String, VersionChirho),
}

/// What install verified for one source package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePackageRecordChirho {
    pub name_chirho: String,
    pub version_chirho: VersionChirho,
    /// The package directory's name inside the store, always `<name>-<version>`.
    pub directory_name_chirho: String,
    /// The `.cabal` file, relative to the package directory.
    pub cabal_file_chirho: PathBuf,
    pub capability_chirho: String,
    pub modules_chirho: Vec<SourceModuleChirho>,
    pub dependencies_chirho: Vec<ResolvedDependencyChirho>,
}

/// The recorded source packages of one store, one per package name: builds select a single
/// directory per name, so a second version could silently change what dependents compile.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourcePackageDbChirho {
    pub packages_chirho: BTreeMap<String, SourcePackageRecordChirho>,
}

/// A line of a source package database that could not be accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDbParseErrorChirho {
    /// One-based line number.
    pub line_chirho: usize,
    pub message_chirho: String,
}

impl std::fmt::Display for SourceDbParseErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f_chirho,
            "line {}: {}",
            self.line_chirho, self.message_chirho
        )
    }
}

impl std::error::Error for SourceDbParseErrorChirho {}

/// Whether a relative path stays inside its package directory and fits on one database line.
pub fn is_recordable_relative_path_chirho(path_chirho: &Path) -> bool {
    let Some(text_chirho) = path_chirho.to_str() else {
        return false;
    };
    !text_chirho.is_empty()
        && !text_chirho.contains(['\n', '\r'])
        && path_chirho
            .components()
            .all(|component_chirho| matches!(component_chirho, Component::Normal(_)))
}

fn is_module_name_chirho(text_chirho: &str) -> bool {
    !text_chirho.is_empty()
        && text_chirho.split('.').all(|part_chirho| {
            part_chirho
                .chars()
                .next()
                .is_some_and(|first_chirho| first_chirho.is_ascii_uppercase())
                && part_chirho.chars().all(|char_chirho| {
                    char_chirho.is_ascii_alphanumeric() || char_chirho == '_' || char_chirho == '\''
                })
        })
}

impl SourcePackageDbChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    pub fn get_chirho(&self, name_chirho: &str) -> Option<&SourcePackageRecordChirho> {
        self.packages_chirho.get(name_chirho)
    }

    /// Add a record for a package name the database does not hold yet.
    pub fn insert_new_chirho(
        &mut self,
        record_chirho: SourcePackageRecordChirho,
    ) -> Result<(), String> {
        validate_record_chirho(&record_chirho)?;
        if self
            .packages_chirho
            .contains_key(&record_chirho.name_chirho)
        {
            return Err(format!(
                "the source database already records {}",
                record_chirho.name_chirho
            ));
        }
        self.packages_chirho
            .insert(record_chirho.name_chirho.clone(), record_chirho);
        Ok(())
    }

    /// The database as text, in a stable order.
    pub fn render_chirho(&self) -> String {
        let mut text_chirho = format!("{SOURCE_DB_HEADER_CHIRHO}\n");
        for record_chirho in self.packages_chirho.values() {
            text_chirho.push_str(&format!(
                "package {} {}\ndirectory {}\ncabal {}\ncapability {}\n",
                record_chirho.name_chirho,
                record_chirho.version_chirho,
                record_chirho.directory_name_chirho,
                record_chirho.cabal_file_chirho.display(),
                record_chirho.capability_chirho,
            ));
            for module_chirho in &record_chirho.modules_chirho {
                text_chirho.push_str(&format!(
                    "module {} {} {} {}\n",
                    if module_chirho.exposed_chirho {
                        "exposed"
                    } else {
                        "other"
                    },
                    match module_chirho.origin_chirho {
                        SourceModuleOriginChirho::ShippedChirho => "shipped",
                        SourceModuleOriginChirho::GeneratedChirho => "generated",
                    },
                    module_chirho.module_name_chirho,
                    module_chirho.relative_path_chirho.display(),
                ));
            }
            for dependency_chirho in &record_chirho.dependencies_chirho {
                match dependency_chirho {
                    ResolvedDependencyChirho::BuiltinChirho(name_chirho) => {
                        text_chirho.push_str(&format!("depends builtin {name_chirho}\n"));
                    }
                    ResolvedDependencyChirho::StoreChirho(name_chirho, version_chirho) => {
                        text_chirho
                            .push_str(&format!("depends store {name_chirho} {version_chirho}\n"));
                    }
                }
            }
            text_chirho.push_str("end\n");
        }
        text_chirho
    }

    /// Parse database text, refusing any line it does not recognise.
    pub fn parse_chirho(text_chirho: &str) -> Result<Self, SourceDbParseErrorChirho> {
        let mut lines_chirho = text_chirho.lines().enumerate();
        match lines_chirho.next() {
            Some((_, SOURCE_DB_HEADER_CHIRHO)) => {}
            _ => {
                return Err(SourceDbParseErrorChirho {
                    line_chirho: 1,
                    message_chirho: format!("expected the header `{SOURCE_DB_HEADER_CHIRHO}`"),
                });
            }
        }
        let mut db_chirho = Self::new_chirho();
        let mut open_chirho: Option<(usize, PartialRecordChirho)> = None;
        for (index_chirho, line_chirho) in lines_chirho {
            let number_chirho = index_chirho + 1;
            let error_chirho = |message_chirho: String| SourceDbParseErrorChirho {
                line_chirho: number_chirho,
                message_chirho,
            };
            let (keyword_chirho, rest_chirho) =
                line_chirho.split_once(' ').unwrap_or((line_chirho, ""));
            match (keyword_chirho, open_chirho.is_some()) {
                ("package", false) => {
                    let (name_chirho, version_text_chirho) =
                        rest_chirho.split_once(' ').ok_or_else(|| {
                            error_chirho("expected `package <name> <version>`".into())
                        })?;
                    let version_chirho =
                        parse_version_chirho(version_text_chirho).ok_or_else(|| {
                            error_chirho(format!("invalid version `{version_text_chirho}`"))
                        })?;
                    open_chirho = Some((
                        number_chirho,
                        PartialRecordChirho::new_chirho(name_chirho, version_chirho),
                    ));
                }
                ("package", true) => {
                    return Err(error_chirho(
                        "`package` before the previous record's `end`".into(),
                    ));
                }
                ("end", true) if rest_chirho.is_empty() => {
                    let (start_chirho, partial_chirho) =
                        open_chirho.take().expect("matched an open record");
                    let record_chirho =
                        partial_chirho.finish_chirho().map_err(|message_chirho| {
                            SourceDbParseErrorChirho {
                                line_chirho: start_chirho,
                                message_chirho,
                            }
                        })?;
                    db_chirho
                        .insert_new_chirho(record_chirho)
                        .map_err(|message_chirho| SourceDbParseErrorChirho {
                            line_chirho: start_chirho,
                            message_chirho,
                        })?;
                }
                (_, true) => {
                    let (_, partial_chirho) = open_chirho.as_mut().expect("matched an open record");
                    partial_chirho
                        .accept_line_chirho(keyword_chirho, rest_chirho)
                        .map_err(error_chirho)?;
                }
                (_, false) => {
                    return Err(error_chirho(format!(
                        "unexpected line outside a package record: `{line_chirho}`"
                    )));
                }
            }
        }
        if let Some((start_chirho, _)) = open_chirho {
            return Err(SourceDbParseErrorChirho {
                line_chirho: start_chirho,
                message_chirho: "package record has no `end`".into(),
            });
        }
        Ok(db_chirho)
    }
}

/// A record whose lines are still being read.
struct PartialRecordChirho {
    name_chirho: String,
    version_chirho: VersionChirho,
    directory_name_chirho: Option<String>,
    cabal_file_chirho: Option<PathBuf>,
    capability_chirho: Option<String>,
    modules_chirho: Vec<SourceModuleChirho>,
    dependencies_chirho: Vec<ResolvedDependencyChirho>,
}

impl PartialRecordChirho {
    fn new_chirho(name_chirho: &str, version_chirho: VersionChirho) -> Self {
        Self {
            name_chirho: name_chirho.to_string(),
            version_chirho,
            directory_name_chirho: None,
            cabal_file_chirho: None,
            capability_chirho: None,
            modules_chirho: Vec::new(),
            dependencies_chirho: Vec::new(),
        }
    }

    fn accept_line_chirho(
        &mut self,
        keyword_chirho: &str,
        rest_chirho: &str,
    ) -> Result<(), String> {
        match keyword_chirho {
            "directory" => {
                set_once_chirho(&mut self.directory_name_chirho, rest_chirho, "directory")
            }
            "cabal" => set_once_chirho(&mut self.cabal_file_chirho, rest_chirho, "cabal"),
            "capability" => set_once_chirho(&mut self.capability_chirho, rest_chirho, "capability"),
            "module" => {
                let mut fields_chirho = rest_chirho.splitn(4, ' ');
                let listing_chirho = fields_chirho.next().unwrap_or_default();
                let origin_chirho = fields_chirho.next().unwrap_or_default();
                let module_name_chirho = fields_chirho.next().unwrap_or_default();
                let path_chirho = fields_chirho.next().unwrap_or_default();
                let exposed_chirho = match listing_chirho {
                    "exposed" => true,
                    "other" => false,
                    _ => {
                        return Err(format!(
                            "module listing must be exposed or other, not `{listing_chirho}`"
                        ));
                    }
                };
                let origin_chirho = match origin_chirho {
                    "shipped" => SourceModuleOriginChirho::ShippedChirho,
                    "generated" => SourceModuleOriginChirho::GeneratedChirho,
                    _ => {
                        return Err(format!(
                            "module origin must be shipped or generated, not `{origin_chirho}`"
                        ));
                    }
                };
                self.modules_chirho.push(SourceModuleChirho {
                    module_name_chirho: module_name_chirho.to_string(),
                    exposed_chirho,
                    origin_chirho,
                    relative_path_chirho: PathBuf::from(path_chirho),
                });
                Ok(())
            }
            "depends" => {
                let fields_chirho: Vec<&str> = rest_chirho.split(' ').collect();
                let dependency_chirho =
                    match fields_chirho.as_slice() {
                        ["builtin", name_chirho] => {
                            ResolvedDependencyChirho::BuiltinChirho(name_chirho.to_string())
                        }
                        ["store", name_chirho, version_text_chirho] => {
                            ResolvedDependencyChirho::StoreChirho(
                                name_chirho.to_string(),
                                parse_version_chirho(version_text_chirho).ok_or_else(|| {
                                    format!("invalid version `{version_text_chirho}`")
                                })?,
                            )
                        }
                        _ => return Err(
                            "expected `depends builtin <name>` or `depends store <name> <version>`"
                                .into(),
                        ),
                    };
                self.dependencies_chirho.push(dependency_chirho);
                Ok(())
            }
            _ => Err(format!(
                "unknown record line `{keyword_chirho} {rest_chirho}`"
            )),
        }
    }

    fn finish_chirho(self) -> Result<SourcePackageRecordChirho, String> {
        let record_chirho = SourcePackageRecordChirho {
            directory_name_chirho: self
                .directory_name_chirho
                .ok_or_else(|| format!("record for {} has no `directory`", self.name_chirho))?,
            cabal_file_chirho: self
                .cabal_file_chirho
                .ok_or_else(|| format!("record for {} has no `cabal`", self.name_chirho))?,
            capability_chirho: self
                .capability_chirho
                .ok_or_else(|| format!("record for {} has no `capability`", self.name_chirho))?,
            name_chirho: self.name_chirho,
            version_chirho: self.version_chirho,
            modules_chirho: self.modules_chirho,
            dependencies_chirho: self.dependencies_chirho,
        };
        validate_record_chirho(&record_chirho)?;
        Ok(record_chirho)
    }
}

fn set_once_chirho<T: From<String>>(
    slot_chirho: &mut Option<T>,
    value_chirho: &str,
    keyword_chirho: &str,
) -> Result<(), String> {
    if value_chirho.is_empty() {
        return Err(format!("`{keyword_chirho}` needs a value"));
    }
    if slot_chirho.is_some() {
        return Err(format!("`{keyword_chirho}` appears twice in one record"));
    }
    *slot_chirho = Some(T::from(value_chirho.to_string()));
    Ok(())
}

/// The invariants every stored record keeps, checked when it is added and when it is read.
fn validate_record_chirho(record_chirho: &SourcePackageRecordChirho) -> Result<(), String> {
    let name_chirho = &record_chirho.name_chirho;
    if !is_safe_package_name_chirho(name_chirho) {
        return Err(format!("`{name_chirho}` is not a valid package name"));
    }
    let expected_directory_chirho = format!("{name_chirho}-{}", record_chirho.version_chirho);
    if record_chirho.directory_name_chirho != expected_directory_chirho {
        return Err(format!(
            "record for {name_chirho} names directory `{}`, not `{expected_directory_chirho}`",
            record_chirho.directory_name_chirho
        ));
    }
    if !is_recordable_relative_path_chirho(&record_chirho.cabal_file_chirho) {
        return Err(format!(
            "record for {name_chirho} has an invalid cabal path"
        ));
    }
    if record_chirho.capability_chirho != CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO {
        return Err(format!(
            "record for {name_chirho} has unknown capability `{}`",
            record_chirho.capability_chirho
        ));
    }
    let mut seen_modules_chirho = std::collections::BTreeSet::new();
    for module_chirho in &record_chirho.modules_chirho {
        if !is_module_name_chirho(&module_chirho.module_name_chirho) {
            return Err(format!(
                "record for {name_chirho} has invalid module name `{}`",
                module_chirho.module_name_chirho
            ));
        }
        if !seen_modules_chirho.insert(module_chirho.module_name_chirho.as_str()) {
            return Err(format!(
                "record for {name_chirho} lists module {} twice",
                module_chirho.module_name_chirho
            ));
        }
        if !is_recordable_relative_path_chirho(&module_chirho.relative_path_chirho) {
            return Err(format!(
                "record for {name_chirho} has an invalid path for module {}",
                module_chirho.module_name_chirho
            ));
        }
    }
    for dependency_chirho in &record_chirho.dependencies_chirho {
        let (ResolvedDependencyChirho::BuiltinChirho(dependency_name_chirho)
        | ResolvedDependencyChirho::StoreChirho(dependency_name_chirho, _)) = dependency_chirho;
        if !is_safe_package_name_chirho(dependency_name_chirho)
            || dependency_name_chirho == name_chirho
        {
            return Err(format!(
                "record for {name_chirho} has invalid dependency `{dependency_name_chirho}`"
            ));
        }
    }
    Ok(())
}
