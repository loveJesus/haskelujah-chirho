// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Shared native heap object layout for compiled backends.
//!
//! This is the target layout for compiled LLVM/Cranelift heap objects used by
//! lazy evaluation, closures, constructors, and PAPs.

/// Word size in bytes for the target (default: 64-bit).
pub const WORD_SIZE_CHIRHO: usize = 8;

/// Every heap object begins with a single header word.
pub const HEADER_SIZE_CHIRHO: usize = WORD_SIZE_CHIRHO;

/// Low bits used to discriminate the heap object kind.
pub const OBJECT_KIND_MASK_CHIRHO: u64 = 0b11;

/// Native object kinds stored in the low two bits of the header word.
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
pub const CLOSURE_FV_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;

/// Thunk layout: header + captured free variables.
pub const THUNK_PAYLOAD_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;

/// Constructor layout: header + fields.
pub const CON_FIELDS_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;

/// PAP layout: header + remaining arity + fun pointer + applied args.
pub const PAP_ARITY_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO;
pub const PAP_FUN_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO + WORD_SIZE_CHIRHO;
pub const PAP_ARGS_OFFSET_CHIRHO: usize = HEADER_SIZE_CHIRHO + 2 * WORD_SIZE_CHIRHO;

/// Pack an entry pointer or metadata word with a native object kind.
pub const fn pack_native_header_chirho(
    header_payload_bits_chirho: u64,
    object_kind_chirho: ObjectKindChirho,
) -> u64 {
    (header_payload_bits_chirho << 2) | (object_kind_chirho as u64)
}

/// Unpack the low-bit kind from a native header word.
pub const fn unpack_native_object_kind_chirho(
    header_word_chirho: u64,
) -> ObjectKindChirho {
    match header_word_chirho & OBJECT_KIND_MASK_CHIRHO {
        0b00 => ObjectKindChirho::ThunkChirho,
        0b01 => ObjectKindChirho::FunChirho,
        0b10 => ObjectKindChirho::ConChirho,
        _ => ObjectKindChirho::PapChirho,
    }
}

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

    #[test]
    fn pack_and_unpack_header_kind_chirho() {
        let header_word_chirho = pack_native_header_chirho(0x1234, ObjectKindChirho::ThunkChirho);
        assert_eq!(header_word_chirho, 0x48d0);
        assert_eq!(
            unpack_native_object_kind_chirho(header_word_chirho),
            ObjectKindChirho::ThunkChirho
        );
    }
}
