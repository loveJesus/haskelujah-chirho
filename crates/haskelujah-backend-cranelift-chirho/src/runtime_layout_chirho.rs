// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Shared native runtime layout re-export for the Cranelift backend.

pub use haskelujah_rts_chirho::native_layout_chirho::*;

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
