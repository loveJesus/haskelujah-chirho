// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The packages the compiler provides itself (row 703).
//!
//! A dependency on one of these is served by the compiler's own module interfaces and runtime,
//! never by compiling that package's source from a store, and its version bound is not checked.
//! This is the single list: source installation consumes it, and the build path's
//! `builtin_dependency_names_chirho` is to consume it as well (kind handoff, row 703). Until
//! then a driver test pins the two lists equal.

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
