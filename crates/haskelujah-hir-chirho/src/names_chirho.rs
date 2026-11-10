// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use haskelujah_span_chirho::SpanChirho;

/// A module-local index; the owning module retains its defining unit and module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NameIdChirho(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitIdentityChirho {
    /// Minted by the standalone driver mode, never inferred from a path.
    StandaloneHomeChirho,
    CompilerChirho,
}

/// Only wired-in language entities, not a replacement Prelude catalogue.
/// Reference: GHC.Builtin.Types, boolTyCon/trueDataCon/falseDataCon/charTyCon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WiredNameChirho {
    BoolChirho,
    CharChirho,
    TrueChirho,
    FalseChirho,
}

#[derive(Debug, Clone)]
pub struct NameInfoChirho {
    pub unit_chirho: UnitIdentityChirho,
    pub module_chirho: String,
    pub occurrence_chirho: String,
    pub local_chirho: bool,
    pub wired_chirho: Option<WiredNameChirho>,
    pub span_chirho: SpanChirho,
}
