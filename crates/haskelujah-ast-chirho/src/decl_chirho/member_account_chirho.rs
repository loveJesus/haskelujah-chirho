// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Small source accounts for class/instance members that have no semantic owner.

use crate::name_chirho::NameChirho;
use haskelujah_span_chirho::SpanChirho;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnattachedMemberKindChirho {
    UnmatchedDefaultChirho,
    RepeatedDefaultChirho,
    PatternBindingChirho,
    MinimalPragmaChirho,
    InlinePragmaChirho,
    UnsupportedChirho,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnattachedMemberChirho {
    pub kind_chirho: UnattachedMemberKindChirho,
    pub name_chirho: Option<String>,
    pub span_chirho: SpanChirho,
}

impl UnattachedMemberChirho {
    pub fn named_chirho(kind_chirho: UnattachedMemberKindChirho, name_chirho: &NameChirho) -> Self {
        Self {
            kind_chirho,
            name_chirho: Some(name_chirho.full_name_chirho()),
            span_chirho: name_chirho.span_chirho(),
        }
    }

    pub fn syntax_chirho(kind_chirho: UnattachedMemberKindChirho, span_chirho: SpanChirho) -> Self {
        Self {
            kind_chirho,
            name_chirho: None,
            span_chirho,
        }
    }
}
