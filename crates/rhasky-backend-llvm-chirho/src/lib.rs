// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-backend-llvm-chirho
//!
//! LLVM IR backend for the Rhasky compiler. Translates Core IR into textual
//! LLVM IR that can be fed to `llc` / `opt` / `clang` for native code
//! generation.

pub mod codegen_chirho;

pub use codegen_chirho::compile_core_to_llvm_chirho;

/// Legacy stub — kept for backward compatibility with `check_source_file_chirho`.
pub fn compile_to_llvm_ir_stub_chirho(module_name_chirho: &str) -> String {
    format!("; rhasky llvm stub for module {module_name_chirho}\n")
}
