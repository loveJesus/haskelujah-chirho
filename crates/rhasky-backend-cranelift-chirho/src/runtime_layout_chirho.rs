// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Runtime object layout constants for the Cranelift backend.
//!
//! Defines the in-memory representation of Haskell heap objects:
//! closures, thunks, constructors, and primitive values.

/// Word size in bytes for the target (default: 64-bit).
pub const WORD_SIZE_CHIRHO: usize = 8;

/// Object header layout.
///
/// Every heap object starts with a header word:
/// ```text
///  [63..8] info-table pointer / entry code address
///  [7..2]  GC tag bits (age, mark, forwarding)
///  [1..0]  object kind: 00 = thunk, 01 = fun, 10 = con, 11 = pap
/// ```
pub const HEADER_SIZE_CHIRHO: usize = WORD_SIZE_CHIRHO;

/// Object kinds stored in the low 2 bits of the header.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectKindChirho {
    ThunkChirho = 0b00,
    FunChirho = 0b01,
    ConChirho = 0b10,
    PapChirho = 0b11,
}

/// Constructor tag offset within the header (bits 8..15).
pub const CON_TAG_SHIFT_CHIRHO: u32 = 8;
pub const CON_TAG_MASK_CHIRHO: u64 = 0xFF00;

/// Closure layout: header + free variables.
///
/// ```text
/// [header] [fv_0] [fv_1] ... [fv_n]
/// ```
pub const CLOSURE_FV_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;

/// Thunk layout: header + payload (initially code pointer, after eval = value).
///
/// ```text
/// [header] [code_ptr / value]
/// ```
pub const THUNK_PAYLOAD_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;

/// Constructor layout: header + fields.
///
/// ```text
/// [header] [field_0] [field_1] ... [field_n]
/// ```
pub const CON_FIELDS_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;

/// PAP (partial application) layout: header + arity + fun_ptr + applied args.
///
/// ```text
/// [header] [remaining_arity] [fun_ptr] [arg_0] ... [arg_k]
/// ```
pub const PAP_ARITY_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;
pub const PAP_FUN_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO + WORD_SIZE_CHIRHO;
pub const PAP_ARGS_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO + 2 * WORD_SIZE_CHIRHO;

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn layout_constants_chirho() {
        assert_eq!(WORD_SIZE_CHIRHO, 8);
        assert_eq!(HEADER_SIZE_CHIRHO, 8);
        assert_eq!(CLOSURE_FV_OFFSET_CHIRHO, 8);
        assert_eq!(PAP_ARGS_OFFSET_CHIRHO, 24);
    }

    #[test]
    fn object_kind_values_chirho() {
        assert_eq!(ObjectKindChirho::ThunkChirho as u8, 0);
        assert_eq!(ObjectKindChirho::FunChirho as u8, 1);
        assert_eq!(ObjectKindChirho::ConChirho as u8, 2);
        assert_eq!(ObjectKindChirho::PapChirho as u8, 3);
    }
}
