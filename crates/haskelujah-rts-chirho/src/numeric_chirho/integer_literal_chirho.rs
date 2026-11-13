// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Opaque, pure Rust Integer values. No source constructor layout or machine-word narrowing.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use num_bigint::BigInt as BigIntChirho;
use std::{fmt, sync::Arc};

pub const INTEGER_BOX_NAME_CHIRHO: &str = "%runtime:opaque:IntegerChirho";
pub const MAX_INTEGER_LITERAL_BYTES_CHIRHO: usize = 16_384;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerValueChirho(Arc<BigIntChirho>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerLiteralErrorChirho {
    MalformedChirho,
    ResourceChirho,
}

impl fmt::Display for IntegerLiteralErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter_chirho.write_str(match self {
            Self::MalformedChirho => "malformed exact Integer literal payload",
            Self::ResourceChirho => "Integer literal byte allowance exhausted",
        })
    }
}

impl IntegerValueChirho {
    pub fn validate_literal_chirho(source_chirho: &str) -> Result<(), IntegerLiteralErrorChirho> {
        Self::literal_parts_chirho(source_chirho).map(|_| ())
    }

    fn literal_parts_chirho(
        source_chirho: &str,
    ) -> Result<(Vec<u8>, u32), IntegerLiteralErrorChirho> {
        if source_chirho.len() > MAX_INTEGER_LITERAL_BYTES_CHIRHO {
            return Err(IntegerLiteralErrorChirho::ResourceChirho);
        }
        if !source_chirho
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_digit)
            || source_chirho.ends_with('_')
        {
            return Err(IntegerLiteralErrorChirho::MalformedChirho);
        }
        // Recognize the prefix before removing separators: `0_x1` is not hex.
        let (source_digits_chirho, radix_chirho) = match source_chirho.as_bytes() {
            [b'0', b'x' | b'X', rest_chirho @ ..] => (rest_chirho, 16),
            [b'0', b'o' | b'O', rest_chirho @ ..] => (rest_chirho, 8),
            [b'0', b'b' | b'B', rest_chirho @ ..] => (rest_chirho, 2),
            rest_chirho => (rest_chirho, 10),
        };
        let digits_chirho: Vec<_> = source_digits_chirho
            .iter()
            .copied()
            .filter(|byte_chirho| *byte_chirho != b'_')
            .collect();
        if digits_chirho.is_empty()
            || !digits_chirho
                .iter()
                .all(|byte_chirho| char::from(*byte_chirho).is_digit(radix_chirho))
        {
            return Err(IntegerLiteralErrorChirho::MalformedChirho);
        }
        Ok((digits_chirho, radix_chirho))
    }

    pub fn from_literal_chirho(source_chirho: &str) -> Result<Self, IntegerLiteralErrorChirho> {
        let (digits_chirho, radix_chirho) = Self::literal_parts_chirho(source_chirho)?;
        BigIntChirho::parse_bytes(&digits_chirho, radix_chirho)
            .map(|value_chirho| Self(Arc::new(value_chirho)))
            .ok_or(IntegerLiteralErrorChirho::MalformedChirho)
    }
}

impl fmt::Display for IntegerValueChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter_chirho)
    }
}

#[cfg(test)]
#[path = "integer_literal_tests_chirho.rs"]
mod tests_chirho;
