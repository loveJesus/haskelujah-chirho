// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-typing-chirho
//!
//! Hindley-Milner type inference engine for the Rhasky compiler.
//! Implements Algorithm W with unification, type schemes, substitution,
//! and let-generalization.

pub mod env_chirho;
pub mod infer_chirho;
pub mod subst_chirho;
pub mod ty_chirho;
pub mod unify_chirho;

pub use infer_chirho::infer_module_chirho;
pub use ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};
