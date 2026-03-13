// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Literal values

use rhasky_span_chirho::SpanChirho;

/// A literal value in source code.
#[derive(Debug, Clone, PartialEq)]
pub enum LitChirho {
    IntChirho(i64, SpanChirho),
    FloatChirho(f64, SpanChirho),
    CharChirho(char, SpanChirho),
    StringChirho(String, SpanChirho),
}

impl LitChirho {
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::IntChirho(_, s_chirho) => *s_chirho,
            Self::FloatChirho(_, s_chirho) => *s_chirho,
            Self::CharChirho(_, s_chirho) => *s_chirho,
            Self::StringChirho(_, s_chirho) => *s_chirho,
        }
    }
}
