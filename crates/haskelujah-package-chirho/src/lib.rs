// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-package-chirho
//!
//! Cabal file parser and Hackage package loader for the Haskelujah compiler.
//!
//! Provides:
//! - Cabal file parsing (`.cabal` format) into structured `PackageDescChirho`
//! - Version constraint parsing and satisfaction checking
//! - Hackage index querying (tarball URL construction)
//! - Dependency resolution types
//! - Source package stores: the compiler-provided package list, Cabal-generated modules, the
//!   strict source package database and atomic, locked store publication (row 703)

pub mod builtin_packages_chirho;
pub mod cabal_chirho;
pub mod generated_modules_chirho;
pub mod hackage_chirho;
pub mod pkgdb_chirho;
pub mod resolve_chirho;
pub mod source_db_chirho;
pub mod source_store_chirho;
#[cfg(test)]
mod source_store_tests_chirho;
pub mod version_chirho;

pub use builtin_packages_chirho::{
    BUILTIN_PACKAGE_NAMES_CHIRHO, is_builtin_package_chirho, is_trivial_constraint_chirho,
};

pub use cabal_chirho::{
    BuildInfoChirho, DependencyChirho, ExecutableChirho, LibraryChirho, PackageDescChirho,
    TestSuiteChirho, parse_cabal_chirho,
};
pub use hackage_chirho::{hackage_cabal_url_chirho, hackage_tarball_url_chirho};
pub use pkgdb_chirho::{
    InstalledModuleChirho, InstalledPkgChirho, InstalledPkgDbChirho, pkg_id_chirho,
};
pub use resolve_chirho::{
    BuildPlanChirho, BuildStepChirho, PackageIndexChirho, PackageMetaChirho, ResolveErrorChirho,
    resolve_build_deps_chirho, resolve_deps_chirho,
};
pub use source_db_chirho::{
    CORE_CHECKED_SOURCE_CAPABILITY_CHIRHO, ResolvedDependencyChirho, SourceModuleChirho,
    SourceModuleOriginChirho, SourcePackageDbChirho, SourcePackageRecordChirho,
};
pub use source_store_chirho::{
    SOURCE_STORE_DIR_NAME_CHIRHO, SourceInstallErrorChirho, SourceInstallInputChirho,
    SourceStoreChirho, StoreLockChirho, StoreStateAfterFailureChirho,
};
pub use version_chirho::{
    VersionChirho, VersionConstraintChirho, parse_version_chirho, parse_version_constraint_chirho,
};
