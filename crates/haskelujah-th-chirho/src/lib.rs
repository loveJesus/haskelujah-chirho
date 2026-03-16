// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Template Haskell support for Haskelujah
//!
//! This crate provides:
//! - TH AST types (`ThExpChirho`, `ThDecChirho`, `ThPatChirho`, `ThTypeChirho`, etc.)
//!   that mirror `Language.Haskell.TH.Syntax` from GHC's `template-haskell` package
//! - The Q monad (`QStateChirho`) for compile-time metaprogramming
//! - Reification (`reify_chirho`) to inspect compiler state from TH code
//! - Splice evaluation and conversion back to haskelujah AST
//!
//! The TH types are deliberately separate from `haskelujah-ast-chirho` types because
//! TH represents the stable Haskell-visible API surface while the compiler AST
//! may evolve independently.

pub mod th_ast_chirho;
pub mod q_monad_chirho;
pub mod reify_chirho;
pub mod convert_chirho;
