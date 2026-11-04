// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared variable identity. Owner tokens establish which solver owns a meta;
//! they do not establish a nominal package or type-constructor identity.

use std::fmt;
use std::sync::Arc;

/// The existing inference-variable identifier, shared by type and kind phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TyVarChirho(pub u32);

impl fmt::Display for TyVarChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f_chirho, "t{}", self.0)
    }
}

/// A single elaboration/inference attempt's opaque owner token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ElaborationOwnerChirho(pub u64);

/// A source variable either belongs to this solver or is a stable quantified
/// slot of an exported declaration. No consumer resolves it by source spelling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopedTypeVariableIdChirho {
    InferenceChirho {
        owner_chirho: ElaborationOwnerChirho,
        variable_chirho: TyVarChirho,
    },
    QuantifiedChirho {
        declaration_chirho: Arc<str>,
        index_chirho: u32,
    },
}
