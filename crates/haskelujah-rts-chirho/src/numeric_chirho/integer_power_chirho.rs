// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's `(^)` at `Int` (row 574). The interpreter and both native backends share this
//! wrapping machine-Int implementation and its negative-exponent boundary.
//! Workflow: testing-chirho/native-fail-loud-chirho.md.

/// `base_chirho ^ exponent_chirho`, wrapping at the machine-Int boundary, or `None` for
/// a negative exponent, which GHC rejects with `Negative exponent`.
pub fn pow_int_chirho(base_chirho: i64, exponent_chirho: i64) -> Option<i64> {
    if exponent_chirho < 0 {
        None
    } else {
        Some(base_chirho.wrapping_pow(exponent_chirho as u32))
    }
}
