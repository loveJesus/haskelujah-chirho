// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The written quantification of an H98 constructor, shared with role inference.
//!
//! Prefix, infix and record constructors retain the same actual binders and context.
//! Binder kinds and specificity remain in the existing complete AST binder vocabulary;
//! consumers must not recover a telescope from names or treat an unread prefix as empty.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use super::TyVarChirho;
use crate::ty_chirho::ConstraintChirho;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConPrefixChirho {
    /// Written binders in lexical order, including kinds and invisible-binder specificity.
    pub existential_vars_chirho: Vec<TyVarChirho>,
    /// Constraints stored by this constructor, with their complete type syntax.
    pub context_chirho: Vec<ConstraintChirho>,
    /// Distinguish an explicit empty `forall .` from the absence of a quantifier.
    pub forall_written_chirho: bool,
    /// The producer encountered syntax it could not faithfully retain.
    pub unread_chirho: bool,
}

impl ConPrefixChirho {
    /// Missing syntax is an unknown quantification, never a proof of no constraints.
    pub fn unread_quantification_chirho() -> Self {
        Self {
            unread_chirho: true,
            ..Self::default()
        }
    }

    pub fn is_empty_chirho(&self) -> bool {
        self.existential_vars_chirho.is_empty()
            && self.context_chirho.is_empty()
            && !self.forall_written_chirho
            && !self.unread_chirho
    }
}
