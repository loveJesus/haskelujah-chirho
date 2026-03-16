// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-backend-jvm-chirho
//!
//! JVM bytecode backend for the Haskelujah compiler. Translates Core IR into
//! JVM class files (.class) that run on the Java Virtual Machine.
//!
//! ## Architecture
//!
//! ```text
//! Core IR → class_chirho → JVM bytecode → .class file
//! ```
//!
//! The approach follows Eta (formerly GHCJVM) and Frege's strategies:
//!
//! - **Closures**: Represented as objects implementing a `Closure` interface
//!   with an `enter()` method. Free variables are object fields.
//! - **Thunks**: Subclass of `Closure` that evaluates lazily on first `enter()`,
//!   then replaces itself with the computed value (indirection).
//! - **Data constructors**: Final classes with a tag field and payload fields.
//! - **Case dispatch**: Switch on constructor tag, or `instanceof` checks.
//! - **Primitive operations**: Direct JVM instructions (iadd, isub, etc.)
//!   for unboxed int/long/double; boxed via wrapper classes when needed.
//! - **Tail calls**: Trampoline pattern — `enter()` returns a continuation
//!   or a value, outer loop dispatches.
//! - **IO**: Modeled as `RealWorld` token threading (erased at runtime).
//!
//! ## JVM Target
//!
//! Class files target JVM 8+ bytecode (major version 52) for maximum
//! compatibility. Higher versions can be selected via configuration.

pub mod bytecode_chirho;
pub mod class_chirho;
pub mod constant_pool_chirho;

pub use class_chirho::compile_core_to_class_chirho;

/// Configuration for JVM bytecode generation.
#[derive(Debug, Clone)]
pub struct JvmConfigChirho {
    /// Target JVM major version (52 = Java 8, 61 = Java 17, 65 = Java 21).
    pub major_version_chirho: u16,
    /// Base package name for generated classes.
    pub package_chirho: String,
    /// Whether to generate debug info (line numbers, local variable tables).
    pub debug_info_chirho: bool,
}

impl Default for JvmConfigChirho {
    fn default() -> Self {
        Self {
            major_version_chirho: 52, // Java 8
            package_chirho: "haskelujah/generated".to_string(),
            debug_info_chirho: true,
        }
    }
}

/// Result of JVM compilation — one or more class files.
#[derive(Debug)]
pub struct JvmOutputChirho {
    /// Generated class files: (class_name, bytes).
    pub classes_chirho: Vec<ClassFileChirho>,
}

/// A single JVM class file.
#[derive(Debug)]
pub struct ClassFileChirho {
    /// Fully qualified class name (e.g. "haskelujah/generated/Main").
    pub name_chirho: String,
    /// Raw .class file bytes.
    pub bytes_chirho: Vec<u8>,
}
