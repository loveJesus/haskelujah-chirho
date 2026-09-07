// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-naming-chirho
//!
//! Name resolution for Haskell. Takes an AST with raw names (`RawNameChirho`)
//! and produces an AST with resolved names (`ResolvedNameChirho`), where each
//! name occurrence is linked to its definition site via a `DefIdChirho`.
//!
//! ## Responsibilities
//!
//! - **Scope analysis**: Track which names are in scope at each point.
//! - **Binding sites**: Assign unique `DefIdChirho` to each definition.
//! - **Use sites**: Resolve each name reference to its binding's `DefIdChirho`.
//! - **Import resolution**: Process import declarations to populate module scopes.
//! - **Error reporting**: Diagnose undefined names, ambiguous names, etc.

pub mod env_chirho;
pub mod iface_chirho;
pub mod resolve_chirho;
mod type_exports_chirho;
mod type_scope_chirho;

pub use env_chirho::NameEnvChirho;
pub use iface_chirho::{
    build_iface_chirho, build_iface_with_imports_chirho, builtin_module_ifaces_chirho,
    ModuleIfaceChirho,
};
pub use resolve_chirho::{
    check_orphan_instances_chirho, resolve_module_chirho, resolve_module_with_imports_chirho,
};
