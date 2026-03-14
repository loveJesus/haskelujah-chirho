// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-package-chirho
//!
//! Cabal file parser and Hackage package loader for the Rhasky compiler.
//!
//! Provides:
//! - Cabal file parsing (`.cabal` format) into structured `PackageDescChirho`
//! - Version constraint parsing and satisfaction checking
//! - Hackage index querying (tarball URL construction)
//! - Dependency resolution types

pub mod cabal_chirho;
pub mod version_chirho;
pub mod hackage_chirho;
pub mod resolve_chirho;

pub use cabal_chirho::{
    parse_cabal_chirho, BuildInfoChirho, DependencyChirho, ExecutableChirho,
    LibraryChirho, PackageDescChirho, TestSuiteChirho,
};
pub use version_chirho::{
    VersionChirho, VersionConstraintChirho, parse_version_chirho,
    parse_version_constraint_chirho,
};
pub use hackage_chirho::{hackage_tarball_url_chirho, hackage_cabal_url_chirho};
pub use resolve_chirho::{
    resolve_deps_chirho, BuildPlanChirho, BuildStepChirho,
    PackageIndexChirho, PackageMetaChirho, ResolveErrorChirho,
};
