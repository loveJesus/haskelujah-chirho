// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pure shared telescope visibility and specificity; source AST fields re-export these same types.

/// Whether a declaration-head binder consumes an ordinary type argument.
/// This is not the specified/inferred distinction on invisible forall binders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TyVarVisibilityChirho {
    /// An ordinary head parameter, written `a` or `(a :: k)`.
    VisibleChirho,
    /// A scoped invisible parameter, written `@a` or `@(a :: k)`.
    InvisibleChirho,
}

/// Whether an invisible quantifier can be selected by written `@` syntax.
/// This is independent of declaration-head/required argument visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TyVarSpecificityChirho {
    SpecifiedChirho,
    InferredChirho,
}
