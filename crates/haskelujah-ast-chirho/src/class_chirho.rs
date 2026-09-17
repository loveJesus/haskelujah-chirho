// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class-body syntax that must survive into checked declaration contracts.

/// A monotone MINIMAL formula. Empty conjunction means no required methods.
/// Invalid recovery is never the same promise as an absent pragma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinimalFormulaChirho {
    MethodChirho(String),
    AllChirho(Vec<MinimalFormulaChirho>),
    AnyChirho(Vec<MinimalFormulaChirho>),
    InvalidChirho,
}
