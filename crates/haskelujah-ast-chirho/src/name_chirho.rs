// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Name representation
//!
//! Before name resolution, names are simple strings (`RawNameChirho`).
//! After resolution, they carry a unique ID (`ResolvedNameChirho`).

use std::fmt;
use std::hash::{Hash, Hasher};

use haskelujah_span_chirho::SpanChirho;

use crate::provenance_chirho::OriginIdChirho;

/// A name as written in source code, before resolution.
///
/// Equality and hashing cover the text, the qualifier and the span, exactly as
/// they did before occurrences carried an origin. The origin is use-site
/// metadata for evidence transport, not part of what the name is: every AST
/// comparison and name lookup keeps its meaning, and the evidence join reads
/// `origin_chirho` directly instead of comparing names.
#[derive(Clone)]
pub struct RawNameChirho {
    /// The textual name as it appears in source.
    pub text_chirho: String,
    /// Optional module qualifier (e.g. `Data.List` in `Data.List.sort`).
    pub qualifier_chirho: Option<String>,
    /// Source location of this name occurrence.
    pub span_chirho: SpanChirho,
    /// The producer-minted origin of this occurrence, on an expression reference
    /// whose use can demand class evidence. `None` identifies nothing: an
    /// occurrence without an origin receives no evidence by position or by span.
    /// Moving or cloning the same occurrence keeps its origin; a genuinely new
    /// occurrence takes a fresh one from the module's supply.
    /// workflow: language-features-chirho/dictionary-evidence-chirho
    pub origin_chirho: Option<OriginIdChirho>,
}

impl PartialEq for RawNameChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        self.text_chirho == other_chirho.text_chirho
            && self.qualifier_chirho == other_chirho.qualifier_chirho
            && self.span_chirho == other_chirho.span_chirho
    }
}

impl Eq for RawNameChirho {}

impl Hash for RawNameChirho {
    fn hash<StateChirho: Hasher>(&self, state_chirho: &mut StateChirho) {
        self.text_chirho.hash(state_chirho);
        self.qualifier_chirho.hash(state_chirho);
        self.span_chirho.hash(state_chirho);
    }
}

impl fmt::Debug for RawNameChirho {
    /// Renders as the derived form did, and names the origin only when there is
    /// one, so output that never involved an origin is unchanged.
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_chirho = f_chirho.debug_struct("RawNameChirho");
        debug_chirho
            .field("text_chirho", &self.text_chirho)
            .field("qualifier_chirho", &self.qualifier_chirho)
            .field("span_chirho", &self.span_chirho);
        if let Some(origin_chirho) = &self.origin_chirho {
            debug_chirho.field("origin_chirho", origin_chirho);
        }
        debug_chirho.finish()
    }
}

impl RawNameChirho {
    /// Construct an unqualified source name such as `map`.
    pub fn unqualified_chirho(text_chirho: impl Into<String>, span_chirho: SpanChirho) -> Self {
        Self {
            text_chirho: text_chirho.into(),
            qualifier_chirho: None,
            span_chirho,
            origin_chirho: None,
        }
    }

    /// Construct a qualified source name such as `Data.List.map`.
    pub fn qualified_chirho(
        qualifier_chirho: impl Into<String>,
        text_chirho: impl Into<String>,
        span_chirho: SpanChirho,
    ) -> Self {
        Self {
            text_chirho: text_chirho.into(),
            qualifier_chirho: Some(qualifier_chirho.into()),
            span_chirho,
            origin_chirho: None,
        }
    }

    /// The same name as an occurrence with a producer-minted origin.
    pub fn with_origin_chirho(mut self, origin_chirho: OriginIdChirho) -> Self {
        self.origin_chirho = Some(origin_chirho);
        self
    }

    /// Return the fully qualified textual form of the name.
    pub fn full_name_chirho(&self) -> String {
        match &self.qualifier_chirho {
            Some(q_chirho) => format!("{}.{}", q_chirho, self.text_chirho),
            None => self.text_chirho.clone(),
        }
    }
}

/// A unique identifier assigned during name resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DefIdChirho(pub u32);

impl DefIdChirho {
    /// Construct a definition identifier from its raw numeric value.
    pub fn new_chirho(id_chirho: u32) -> Self {
        Self(id_chirho)
    }
}

/// A name after resolution, carrying both source text and a unique ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedNameChirho {
    /// The original source spelling and span information.
    pub raw_chirho: RawNameChirho,
    /// The unique definition this occurrence resolves to.
    pub def_id_chirho: DefIdChirho,
}

/// Generic name type — before or after resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NameChirho {
    RawChirho(RawNameChirho),
    ResolvedChirho(ResolvedNameChirho),
}

impl NameChirho {
    /// Return the source span covering this name occurrence.
    pub fn span_chirho(&self) -> SpanChirho {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.span_chirho,
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.span_chirho,
        }
    }

    /// Return the unqualified text of the name as written in source.
    pub fn text_chirho(&self) -> &str {
        match self {
            Self::RawChirho(n_chirho) => &n_chirho.text_chirho,
            Self::ResolvedChirho(n_chirho) => &n_chirho.raw_chirho.text_chirho,
        }
    }

    /// Return the fully qualified name, including any module qualifier.
    pub fn full_name_chirho(&self) -> String {
        match self {
            Self::RawChirho(n_chirho) => n_chirho.full_name_chirho(),
            Self::ResolvedChirho(n_chirho) => n_chirho.raw_chirho.full_name_chirho(),
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn raw_name_unqualified_chirho() {
        let name_chirho = RawNameChirho::unqualified_chirho("foo", SpanChirho::DUMMY_CHIRHO);
        assert_eq!(name_chirho.full_name_chirho(), "foo");
        assert!(name_chirho.qualifier_chirho.is_none());
    }

    #[test]
    fn raw_name_qualified_chirho() {
        let name_chirho =
            RawNameChirho::qualified_chirho("Data.List", "sort", SpanChirho::DUMMY_CHIRHO);
        assert_eq!(name_chirho.full_name_chirho(), "Data.List.sort");
    }

    #[test]
    fn name_enum_text_chirho() {
        let raw_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "bar",
            SpanChirho::DUMMY_CHIRHO,
        ));
        assert_eq!(raw_chirho.text_chirho(), "bar");
    }
}
