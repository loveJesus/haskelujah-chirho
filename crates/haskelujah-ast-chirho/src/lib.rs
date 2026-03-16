// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-ast-chirho
//!
//! Abstract Syntax Tree types for Haskell 2010. This is the "clean" AST
//! produced by lowering the lossless CST — it drops trivia and unnecessary
//! punctuation, and organizes nodes for semantic analysis.
//!
//! ## Design
//!
//! - Every AST node carries a `SpanChirho` for error reporting.
//! - Nodes use `Box<T>` for recursive types to keep the enum sizes small.
//! - The AST is parameterized by a name type `N` so that the same tree
//!   structure can be used before and after name resolution (with different
//!   name representations).

pub mod decl_chirho;
pub mod expr_chirho;
pub mod lit_chirho;
pub mod module_chirho;
pub mod name_chirho;
pub mod pat_chirho;
pub mod ty_chirho;

pub use decl_chirho::ForeignDirectionChirho;
pub use module_chirho::ModuleChirho;
pub use name_chirho::NameChirho;
