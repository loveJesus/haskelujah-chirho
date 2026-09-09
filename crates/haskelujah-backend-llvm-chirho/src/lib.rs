// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-backend-llvm-chirho
//!
//! LLVM IR backend for the Haskelujah compiler. Translates Core IR into textual
//! LLVM IR that can be fed to `llc` / `opt` / `clang` for native code
//! generation.

pub mod codegen_chirho;
pub use haskelujah_rts_chirho::target_chirho::native_target_chirho;

pub use codegen_chirho::compile_core_to_llvm_chirho;
pub use codegen_chirho::compile_core_to_llvm_executable_chirho;
pub use codegen_chirho::try_compile_core_to_llvm_executable_chirho;

/// Legacy stub — kept for backward compatibility with `check_source_file_chirho`.
pub fn compile_to_llvm_ir_stub_chirho(module_name_chirho: &str) -> String {
    // TODO(codex-audit): remove this compatibility stub once all callers use
    // the real LLVM entry points.
    format!("; haskelujah llvm stub for module {module_name_chirho}\n")
}
