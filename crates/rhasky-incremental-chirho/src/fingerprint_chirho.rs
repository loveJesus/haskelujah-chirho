// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Source and artifact fingerprinting
//!
//! A [`FingerprintChirho`] is a 128-bit content hash used to detect changes
//! in source files and intermediate artifacts. We use a simple, non-cryptographic
//! hash (FNV-1a extended to 128 bits via two independent 64-bit halves) that is
//! fast for the compiler's purposes while providing ample collision resistance
//! for a single-project namespace.

use std::fmt;

/// A 128-bit content hash.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FingerprintChirho {
    pub hi_chirho: u64,
    pub lo_chirho: u64,
}

impl FingerprintChirho {
    /// The zero fingerprint (used as a sentinel for "no fingerprint").
    pub const ZERO_CHIRHO: Self = Self {
        hi_chirho: 0,
        lo_chirho: 0,
    };

    /// Create from two halves.
    pub fn new_chirho(hi_chirho: u64, lo_chirho: u64) -> Self {
        Self {
            hi_chirho,
            lo_chirho,
        }
    }

    /// Compute a fingerprint from an arbitrary byte slice.
    ///
    /// Uses FNV-1a on two independent seeds to produce 128 bits.
    pub fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        // FNV-1a constants
        const FNV_OFFSET_A_CHIRHO: u64 = 0xcbf29ce484222325;
        const FNV_OFFSET_B_CHIRHO: u64 = 0x6c62272e07bb0142;
        const FNV_PRIME_CHIRHO: u64 = 0x00000100000001B3;

        let mut h_a_chirho = FNV_OFFSET_A_CHIRHO;
        let mut h_b_chirho = FNV_OFFSET_B_CHIRHO;

        for &byte_chirho in data_chirho {
            h_a_chirho ^= byte_chirho as u64;
            h_a_chirho = h_a_chirho.wrapping_mul(FNV_PRIME_CHIRHO);
            h_b_chirho ^= byte_chirho as u64;
            h_b_chirho = h_b_chirho.wrapping_mul(FNV_PRIME_CHIRHO);
        }

        Self {
            hi_chirho: h_a_chirho,
            lo_chirho: h_b_chirho,
        }
    }

    /// Compute a fingerprint from a string.
    pub fn from_str_chirho(s_chirho: &str) -> Self {
        Self::from_bytes_chirho(s_chirho.as_bytes())
    }

    /// Combine two fingerprints into one (order-dependent).
    pub fn combine_chirho(self, other_chirho: Self) -> Self {
        let mut buf_chirho = [0u8; 32];
        buf_chirho[0..8].copy_from_slice(&self.hi_chirho.to_le_bytes());
        buf_chirho[8..16].copy_from_slice(&self.lo_chirho.to_le_bytes());
        buf_chirho[16..24].copy_from_slice(&other_chirho.hi_chirho.to_le_bytes());
        buf_chirho[24..32].copy_from_slice(&other_chirho.lo_chirho.to_le_bytes());
        Self::from_bytes_chirho(&buf_chirho)
    }

    /// Combine a slice of fingerprints (e.g. all dependency fingerprints).
    pub fn combine_many_chirho(fps_chirho: &[Self]) -> Self {
        fps_chirho
            .iter()
            .copied()
            .fold(Self::ZERO_CHIRHO, |acc_chirho, fp_chirho| {
                acc_chirho.combine_chirho(fp_chirho)
            })
    }

    /// Hex string representation (32 hex digits).
    pub fn to_hex_chirho(&self) -> String {
        format!("{:016x}{:016x}", self.hi_chirho, self.lo_chirho)
    }

    /// Parse from a 32-character hex string.
    pub fn from_hex_chirho(s_chirho: &str) -> Option<Self> {
        if s_chirho.len() != 32 {
            return None;
        }
        let hi_chirho = u64::from_str_radix(&s_chirho[..16], 16).ok()?;
        let lo_chirho = u64::from_str_radix(&s_chirho[16..], 16).ok()?;
        Some(Self {
            hi_chirho,
            lo_chirho,
        })
    }

    /// Whether this is the zero sentinel.
    pub fn is_zero_chirho(&self) -> bool {
        self.hi_chirho == 0 && self.lo_chirho == 0
    }
}

impl fmt::Debug for FingerprintChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "Fp({})", self.to_hex_chirho())
    }
}

impl fmt::Display for FingerprintChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "{}", self.to_hex_chirho())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn empty_bytes_fingerprint_chirho() {
        let fp_chirho = FingerprintChirho::from_bytes_chirho(b"");
        assert!(!fp_chirho.is_zero_chirho());
        // The offset basis itself (no data mixed in) is the result.
        assert_eq!(fp_chirho.hi_chirho, 0xcbf29ce484222325);
        assert_eq!(fp_chirho.lo_chirho, 0x6c62272e07bb0142);
    }

    #[test]
    fn deterministic_chirho() {
        let a_chirho = FingerprintChirho::from_str_chirho("hello world");
        let b_chirho = FingerprintChirho::from_str_chirho("hello world");
        assert_eq!(a_chirho, b_chirho);
    }

    #[test]
    fn different_inputs_differ_chirho() {
        let a_chirho = FingerprintChirho::from_str_chirho("hello");
        let b_chirho = FingerprintChirho::from_str_chirho("world");
        assert_ne!(a_chirho, b_chirho);
    }

    #[test]
    fn hex_round_trip_chirho() {
        let fp_chirho = FingerprintChirho::from_str_chirho("module Main where");
        let hex_chirho = fp_chirho.to_hex_chirho();
        assert_eq!(hex_chirho.len(), 32);
        let parsed_chirho = FingerprintChirho::from_hex_chirho(&hex_chirho).unwrap();
        assert_eq!(fp_chirho, parsed_chirho);
    }

    #[test]
    fn invalid_hex_returns_none_chirho() {
        assert!(FingerprintChirho::from_hex_chirho("too_short").is_none());
        assert!(FingerprintChirho::from_hex_chirho("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz").is_none());
    }

    #[test]
    fn combine_is_order_dependent_chirho() {
        let a_chirho = FingerprintChirho::from_str_chirho("A");
        let b_chirho = FingerprintChirho::from_str_chirho("B");
        let ab_chirho = a_chirho.combine_chirho(b_chirho);
        let ba_chirho = b_chirho.combine_chirho(a_chirho);
        assert_ne!(ab_chirho, ba_chirho);
    }

    #[test]
    fn combine_many_chirho() {
        let fps_chirho = vec![
            FingerprintChirho::from_str_chirho("A"),
            FingerprintChirho::from_str_chirho("B"),
            FingerprintChirho::from_str_chirho("C"),
        ];
        let combined_chirho = FingerprintChirho::combine_many_chirho(&fps_chirho);
        assert!(!combined_chirho.is_zero_chirho());
        // Repeatable
        assert_eq!(
            combined_chirho,
            FingerprintChirho::combine_many_chirho(&fps_chirho)
        );
    }

    #[test]
    fn zero_sentinel_chirho() {
        assert!(FingerprintChirho::ZERO_CHIRHO.is_zero_chirho());
        assert!(!FingerprintChirho::from_str_chirho("x").is_zero_chirho());
    }

    #[test]
    fn display_and_debug_chirho() {
        let fp_chirho = FingerprintChirho::new_chirho(0xDEAD, 0xBEEF);
        assert_eq!(format!("{}", fp_chirho), "000000000000dead000000000000beef");
        assert_eq!(
            format!("{:?}", fp_chirho),
            "Fp(000000000000dead000000000000beef)"
        );
    }
}
