// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Name representation
//!
//! Before name resolution, names are simple strings (`RawNameChirho`).
//! After resolution, they carry a unique ID (`ResolvedNameChirho`).

use haskelujah_span_chirho::SpanChirho;

/// A name as written in source code, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawNameChirho {
    /// The textual name as it appears in source.
    pub text_chirho: String,
    /// Optional module qualifier (e.g. `Data.List` in `Data.List.sort`).
    pub qualifier_chirho: Option<String>,
    /// Source location of this name occurrence.
    pub span_chirho: SpanChirho,
}

impl RawNameChirho {
    /// Construct an unqualified source name such as `map`.
    pub fn unqualified_chirho(text_chirho: impl Into<String>, span_chirho: SpanChirho) -> Self {
        Self {
            text_chirho: text_chirho.into(),
            qualifier_chirho: None,
            span_chirho,
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
        }
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
