// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-core-chirho
//!
//! The typed Core intermediate representation for the Rhasky compiler.
//! Analogous to GHC's System FC Core: a small, explicitly typed language
//! that is the target of desugaring and the input to optimization passes.
//!
//! Core has these expression forms:
//! - `Var` — variable reference
//! - `Lit` — literal value
//! - `App` — function application
//! - `Lam` — lambda abstraction
//! - `Let` — let binding (recursive or non-recursive)
//! - `Case` — case expression (the only pattern matching form)
//! - `TyLam` / `TyApp` — type abstraction and application (polymorphism)

pub mod desugar_chirho;
pub mod expr_chirho;
pub mod pretty_chirho;

pub use desugar_chirho::desugar_module_chirho;
pub use expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho,
};
pub use pretty_chirho::pretty_module_chirho;
