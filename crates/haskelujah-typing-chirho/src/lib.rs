// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-typing-chirho
//!
//! Hindley-Milner type inference engine for the Haskelujah compiler.
//! Implements Algorithm W with unification, type schemes, substitution,
//! and let-generalization.

pub mod class_chirho;
mod dependency_chirho;
pub mod deriving_chirho;
pub mod env_chirho;
pub mod exhaust_chirho;
#[path = "types_chirho/families_chirho.rs"]
mod families_chirho;
pub mod infer_chirho;
pub mod kind_chirho;
pub mod linearity_chirho;
#[path = "types_chirho/module_contracts_chirho.rs"]
pub mod module_contracts_chirho;
#[path = "types_chirho/scheme_equality_chirho.rs"]
mod scheme_equality_chirho;
pub mod skolem_chirho;
#[path = "types_chirho/subst_chirho.rs"]
pub mod subst_chirho;
#[path = "types_chirho/ty_chirho.rs"]
pub mod ty_chirho;
pub mod unify_chirho;
pub mod validity_chirho;

pub use class_chirho::{ClassDeclChirho, ClassEnvChirho, InstDeclChirho, PredChirho, QualTyChirho};
pub use deriving_chirho::{DerivingResultChirho, apply_deriving_chirho, derive_instances_chirho};
pub use exhaust_chirho::{
    ExhaustResultChirho, TypeConEnvChirho, check_module_exhaustiveness_chirho,
};
pub use infer_chirho::infer_module_chirho;
pub use infer_chirho::infer_module_with_imports_chirho;
pub use kind_chirho::{KindChirho, KindEnvChirho, KindResultChirho, infer_module_kinds_chirho};
pub use ty_chirho::{MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};
pub use validity_chirho::{
    ValidityErrorChirho, ValidityResultChirho, check_module_type_validity_chirho,
    check_module_type_validity_diagnostics_chirho,
};
