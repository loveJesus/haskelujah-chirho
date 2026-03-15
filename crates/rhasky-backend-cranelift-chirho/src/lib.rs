// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-backend-cranelift-chirho
//!
//! Cranelift-based native code backend for the Rhasky compiler. Translates
//! Core IR into machine code via Cranelift's code generator, producing native
//! object files for any target architecture that Cranelift supports.
//!
//! ## Supported Targets
//!
//! Cranelift supports x86_64, aarch64, s390x, and riscv64. The target is
//! selectable at compile time via `target-lexicon` triple strings.
//!
//! ## Architecture
//!
//! ```text
//! Core IR → codegen_chirho → Cranelift IR (CLIF) → cranelift-object → .o file
//! ```
//!
//! The backend handles:
//! - **Function lowering**: Core lambdas/let-bindings → Cranelift functions
//! - **Closure representation**: Heap-allocated closures with environment capture
//! - **Thunk representation**: Lazy thunks with enter/update protocol
//! - **Primitive operations**: Arithmetic, comparison, string ops → native insns
//! - **Data constructors**: Tagged heap objects with constructor fields
//! - **Case dispatch**: Pattern matching → branch tables / conditional jumps
//! - **GC integration**: Stack maps and safe points for garbage collection
//! - **FFI**: Foreign function calls via Cranelift's call_indirect / call_extern

pub mod codegen_chirho;
pub mod lower_chirho;
pub mod runtime_layout_chirho;

pub use codegen_chirho::{compile_core_to_object_chirho, compile_core_to_object_executable_chirho};

/// Target triple for native code generation.
#[derive(Debug, Clone)]
pub struct TargetConfigChirho {
    /// Target triple string (e.g. "x86_64-unknown-linux-gnu", "aarch64-apple-darwin").
    pub triple_chirho: String,
    /// Whether to emit position-independent code.
    pub pic_chirho: bool,
    /// Optimization level: 0 = none, 1 = speed, 2 = speed+size.
    pub opt_level_chirho: u8,
}

impl Default for TargetConfigChirho {
    fn default() -> Self {
        Self {
            triple_chirho: target_lexicon::Triple::host().to_string(),
            pic_chirho: true,
            opt_level_chirho: 0,
        }
    }
}

/// Result of native compilation.
#[derive(Debug)]
pub struct NativeObjectChirho {
    /// Raw object file bytes (.o / .obj).
    pub object_bytes_chirho: Vec<u8>,
    /// Target triple used for compilation.
    pub target_triple_chirho: String,
}
