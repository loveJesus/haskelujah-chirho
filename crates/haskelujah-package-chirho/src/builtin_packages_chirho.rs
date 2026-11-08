// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The packages the compiler provides itself (row 703).
//!
//! A dependency on one of these is served by the compiler's own module interfaces and runtime,
//! never by compiling that package's source from a store. The compiler declares no version for
//! any of them: the `MIN_VERSION_*` values its C preprocessor defines are compatibility seeds that
//! a store directory of the same name can raise, not a capability. So a name match proves nothing
//! about a version bound, and source install accepts a builtin dependency only when its bound is
//! trivial (see [`is_trivial_constraint_chirho`]).
//! This is the single list: source installation consumes it, and the build path's
//! `builtin_dependency_names_chirho` is to consume it as well (kind handoff, row 703). Until
//! then a driver test pins the two lists equal.

use crate::version_chirho::VersionConstraintChirho;

/// The package names the compiler provides, in the build path's order.
pub const BUILTIN_PACKAGE_NAMES_CHIRHO: [&str; 23] = [
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
];

/// Whether `package_name_chirho` is provided by the compiler rather than by a store.
pub fn is_builtin_package_chirho(package_name_chirho: &str) -> bool {
    BUILTIN_PACKAGE_NAMES_CHIRHO.contains(&package_name_chirho)
}

/// Whether every version satisfies `constraint_chirho`: `-any`, `>= 0`, or a combination that
/// cannot exclude a version. Only such a bound is accepted on a package with no declared version.
pub fn is_trivial_constraint_chirho(constraint_chirho: &VersionConstraintChirho) -> bool {
    match constraint_chirho {
        VersionConstraintChirho::AnyChirho => true,
        VersionConstraintChirho::GeChirho(version_chirho) => version_chirho
            .components_chirho
            .iter()
            .all(|component_chirho| *component_chirho == 0),
        VersionConstraintChirho::AndChirho(left_chirho, right_chirho) => {
            is_trivial_constraint_chirho(left_chirho) && is_trivial_constraint_chirho(right_chirho)
        }
        VersionConstraintChirho::OrChirho(left_chirho, right_chirho) => {
            is_trivial_constraint_chirho(left_chirho) || is_trivial_constraint_chirho(right_chirho)
        }
        _ => false,
    }
}
