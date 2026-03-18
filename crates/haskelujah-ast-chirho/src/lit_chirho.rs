// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Literal values

use haskelujah_span_chirho::SpanChirho;

/// A literal value in source code.
#[derive(Debug, Clone, PartialEq)]
pub enum LitChirho {
    /// Integer literal and its source span.
    IntChirho(i64, SpanChirho),
    /// Floating-point literal and its source span.
    FloatChirho(f64, SpanChirho),
    /// Character literal and its source span.
    CharChirho(char, SpanChirho),
    /// String literal contents and its source span.
    StringChirho(String, SpanChirho),
}

impl LitChirho {
    /// Return the source span covering this literal.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::IntChirho(_, s_chirho) => *s_chirho,
            Self::FloatChirho(_, s_chirho) => *s_chirho,
            Self::CharChirho(_, s_chirho) => *s_chirho,
            Self::StringChirho(_, s_chirho) => *s_chirho,
        }
    }
}
