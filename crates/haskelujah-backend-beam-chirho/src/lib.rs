// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # haskelujah-backend-beam-chirho
//!
//! BEAM (Erlang Virtual Machine) bytecode backend for the Haskelujah compiler.
//! Translates Core IR into BEAM bytecode (.beam files) that can run on the
//! Erlang/OTP runtime, alongside Erlang, Elixir, and Gleam programs.
//!
//! ## Why BEAM?
//!
//! The BEAM VM provides:
//! - **Lightweight processes**: millions of concurrent processes with preemptive
//!   scheduling — a natural fit for Haskell's lightweight thread model
//! - **Fault tolerance**: supervision trees and process isolation
//! - **Hot code loading**: update running systems without downtime
//! - **Distribution**: transparent inter-node communication
//! - **Pattern matching**: native support for tagged tuples aligns with
//!   Haskell's algebraic data types
//!
//! ## Architecture
//!
//! ```text
//! Core IR → beam_module_chirho → BEAM instructions → .beam file
//! ```
//!
//! The BEAM file format uses External Term Format (ETF) for its chunk data.
//! Key chunks:
//! - **AtU8**: Atom table (constructor names, function names)
//! - **Code**: BEAM opcodes
//! - **StrT**: String literals
//! - **ImpT**: Import table (external function references)
//! - **ExpT**: Export table (public functions)
//! - **FunT**: Lambda/fun table
//! - **LitT**: Literal table (constants, compressed with zlib)
//!
//! ## Haskell → BEAM Mapping
//!
//! - **Closures**: BEAM funs (anonymous functions with environment capture)
//! - **Thunks**: Process-local lazy evaluation via spawn/receive or ETS
//! - **Data constructors**: Tagged tuples `{Tag, Field1, Field2, ...}`
//! - **Case dispatch**: BEAM `select_val` / `select_tuple_arity` instructions
//! - **IO monad**: Direct process I/O via BEAM message passing and ports
//! - **GC**: Delegated entirely to BEAM's per-process generational GC

pub mod beam_module_chirho;
pub mod etf_chirho;
pub mod opcodes_chirho;

pub use beam_module_chirho::compile_core_to_beam_chirho;

/// Configuration for BEAM bytecode generation.
#[derive(Debug, Clone)]
pub struct BeamConfigChirho {
    /// Module name in the BEAM system (atom).
    pub module_name_chirho: String,
    /// Target BEAM instruction set version.
    pub instruction_set_chirho: u32,
    /// Maximum BEAM opcode version to use.
    pub max_opcode_chirho: u32,
}

impl Default for BeamConfigChirho {
    fn default() -> Self {
        Self {
            module_name_chirho: "haskelujah_main".to_string(),
            instruction_set_chirho: 0,
            max_opcode_chirho: 181, // OTP 26
        }
    }
}

/// Result of BEAM compilation.
#[derive(Debug)]
pub struct BeamOutputChirho {
    /// Module name.
    pub module_name_chirho: String,
    /// Raw .beam file bytes.
    pub beam_bytes_chirho: Vec<u8>,
}
