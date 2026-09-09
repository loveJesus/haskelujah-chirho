// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Shared Runtime System for Haskelujah
//!
//! This crate contains the backend-agnostic runtime primitives shared across
//! all execution backends (STG interpreter, LLVM, Cranelift, WebAssembly):
//!
//! - **`value_chirho`** — Runtime value types (`ValueChirho`, `ClosureChirho`,
//!   `InfoTableChirho`, `HeapAddrChirho`, `DataConTagChirho`)
//! - **`heap_chirho`** — Arena-based heap allocator (`HeapChirho`)
//! - **`gc_chirho`** — Mark-sweep garbage collector (`GcStateChirho`)
//!
//! The STG interpreter (`haskelujah-runtime-chirho`) adds the evaluation loop,
//! stack, primop dispatch, and FFI on top of these shared primitives.
//! Native-code backends link against this crate for heap management and GC
//! without pulling in interpreter-specific machinery.

pub mod ffi_chirho;
pub mod gc_chirho;
pub mod heap_chirho;
pub mod native_layout_chirho;
pub mod target_chirho;
pub mod value_chirho;

pub use gc_chirho::{
    GcConfigChirho, GcStateChirho, GcStatsChirho, extract_roots_from_values_chirho,
};
pub use heap_chirho::HeapChirho;
pub use native_layout_chirho::{
    CLOSURE_FV_OFFSET_CHIRHO, CON_FIELDS_OFFSET_CHIRHO, CON_TAG_MASK_CHIRHO, CON_TAG_SHIFT_CHIRHO,
    HEADER_SIZE_CHIRHO, OBJECT_KIND_MASK_CHIRHO, ObjectKindChirho, PAP_ARGS_OFFSET_CHIRHO,
    PAP_ARITY_OFFSET_CHIRHO, PAP_FUN_OFFSET_CHIRHO, THUNK_PAYLOAD_OFFSET_CHIRHO, WORD_SIZE_CHIRHO,
    pack_native_header_chirho, unpack_native_object_kind_chirho,
};
pub use value_chirho::{
    ClosureChirho, CodePtrChirho, DataConTagChirho, HeapAddrChirho, InfoTableChirho, InfoTagChirho,
    ValueChirho,
};
