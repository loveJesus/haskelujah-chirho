// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Occurrence provenance: the identity one class-method USE carries from the
//! producer that created it to the evidence join that consumes it.
//!
//! An origin is minted ONCE, by the producer of the construct, from the one
//! supply owned by the module compilation. Lowering, Template Haskell
//! expansion, early deriving and the late GND pass all BORROW that supply;
//! none of them starts a counter, and nothing downstream re-enumerates an
//! origin. A span is kept for diagnostics and is never the identity: the
//! deriving pass gives every reference it generates the same placeholder span.
//!
//! Absence is meaningful. A use with no origin has NO identity evidence, and a
//! use that demands evidence while lacking an identity is a named compiler
//! failure, never a guessed proof.
//! workflow: language-features-chirho/dictionary-evidence-chirho

/// An opaque identity for one occurrence-producing construct within one
/// module compilation. It is use-site metadata, never a binding's `DefId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OriginIdChirho(u32);

impl OriginIdChirho {
    /// The raw number, for diagnostics and serialization only.
    pub fn as_raw_chirho(self) -> u32 {
        self.0
    }
}

/// Which use a construct makes of a method, for the constructs that make more
/// than one, or that make one without naming it (a literal's `fromInteger`, a
/// do statement's `>>=`). An explicit name reference is `Reference`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OccurrenceRoleChirho {
    /// A method named in the source (`show x`, `x == y`).
    Reference,
    /// `fromInteger` for an integer literal.
    IntegerLiteral,
    /// `fromString` for a string literal under OverloadedStrings.
    StringLiteral,
    /// `fromList` for a list literal under OverloadedLists.
    ListLiteral,
    /// `enumFrom` for `[a ..]`.
    EnumFrom,
    /// `enumFromThen` for `[a, b ..]`.
    EnumFromThen,
    /// `enumFromTo` for `[a .. c]`.
    EnumFromTo,
    /// `enumFromThenTo` for `[a, b .. c]`.
    EnumFromThenTo,
    /// `>>=` for a do-statement bind.
    Bind,
    /// `>>` for do-statement sequencing.
    Then,
    /// `fail` for a failable pattern in a do bind.
    Fail,
}

/// The identity of one method use: its construct's origin and the role of this
/// use within that construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProvenanceChirho {
    pub origin_chirho: OriginIdChirho,
    pub role_chirho: OccurrenceRoleChirho,
}

impl ProvenanceChirho {
    pub fn new_chirho(origin_chirho: OriginIdChirho, role_chirho: OccurrenceRoleChirho) -> Self {
        Self {
            origin_chirho,
            role_chirho,
        }
    }
}

/// Mints origins for one module compilation. It is owned at the module
/// lifetime and borrowed, never copied, by every producer, so two producers
/// that run at different times can never mint the same origin.
#[derive(Debug, Default)]
pub struct OriginSupplyChirho {
    next_chirho: u32,
}

impl OriginSupplyChirho {
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// A fresh origin, distinct from every origin this supply has minted.
    pub fn fresh_chirho(&mut self) -> OriginIdChirho {
        let origin_chirho = OriginIdChirho(self.next_chirho);
        self.next_chirho = self
            .next_chirho
            .checked_add(1)
            .expect("origin supply exhausted within one module compilation");
        origin_chirho
    }

    /// How many origins this supply has minted.
    pub fn minted_chirho(&self) -> u32 {
        self.next_chirho
    }
}
