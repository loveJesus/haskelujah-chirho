// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-backend-wasm-chirho
//!
//! WebAssembly backend for the Rhasky compiler. Translates Core IR into
//! a valid Wasm binary module that can be executed by any conforming
//! WebAssembly runtime.

pub mod codegen_chirho;

pub use codegen_chirho::{compile_core_to_wasm_chirho, compile_core_to_wasm_executable_chirho};

/// Legacy stub — kept for backward compatibility with `check_source_file_chirho`.
pub fn compile_to_wasm_stub_chirho(module_name_chirho: &str) -> Vec<u8> {
    format!("rhasky-wasm-stub:{module_name_chirho}").into_bytes()
}
