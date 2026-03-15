// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Shared Runtime System for Rhasky
//!
//! This crate contains the backend-agnostic runtime primitives shared across
//! all execution backends (STG interpreter, LLVM, Cranelift, WebAssembly):
//!
//! - **`value_chirho`** — Runtime value types (`ValueChirho`, `ClosureChirho`,
//!   `InfoTableChirho`, `HeapAddrChirho`, `DataConTagChirho`)
//! - **`heap_chirho`** — Arena-based heap allocator (`HeapChirho`)
//! - **`gc_chirho`** — Mark-sweep garbage collector (`GcStateChirho`)
//!
//! The STG interpreter (`rhasky-runtime-chirho`) adds the evaluation loop,
//! stack, primop dispatch, and FFI on top of these shared primitives.
//! Native-code backends link against this crate for heap management and GC
//! without pulling in interpreter-specific machinery.

pub mod value_chirho;
pub mod heap_chirho;
pub mod gc_chirho;

pub use value_chirho::{
    ValueChirho, ClosureChirho, InfoTableChirho, InfoTagChirho,
    CodePtrChirho, DataConTagChirho, HeapAddrChirho,
};
pub use heap_chirho::HeapChirho;
pub use gc_chirho::{
    GcConfigChirho, GcStateChirho, GcStatsChirho,
    extract_roots_from_values_chirho,
};
