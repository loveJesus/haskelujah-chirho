// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::fmt;

use haskelujah_span_chirho::SpanChirho;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcErrorKindChirho {
    TypeMismatchChirho,
    KindMismatchChirho,
    InfiniteTypeChirho,
    SkolemEscapeChirho,
    UntouchableChirho,
    UnsupportedChirho,
    ResourceChirho,
    InvalidReferenceChirho,
}

#[derive(Debug, Clone)]
pub struct TcErrorChirho {
    pub kind_chirho: TcErrorKindChirho,
    pub message_chirho: String,
    pub span_chirho: SpanChirho,
}

impl TcErrorChirho {
    pub(crate) fn new_chirho(
        kind_chirho: TcErrorKindChirho,
        message_chirho: impl Into<String>,
    ) -> Self {
        Self {
            kind_chirho,
            message_chirho: message_chirho.into(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }
}

impl fmt::Display for TcErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter_chirho, "{}", self.message_chirho)
    }
}

impl std::error::Error for TcErrorChirho {}
