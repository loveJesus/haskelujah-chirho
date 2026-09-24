// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The operation a do statement selects.
//!
//! A do statement is not written with its operator: `x <- m` selects `>>=`,
//! a non-tail `m` selects `>>`, and a bind whose pattern can fail also selects
//! `fail`. Which binding those names stand for depends on the block
//! (QualifiedDo's `M.do` selects `M.>>=`, RebindableSyntax selects whatever is
//! in scope), so the choice is made ONCE, by lowering, and recorded on the
//! statement. Checking and desugaring both read that one record instead of
//! choosing again: two phases that select independently can disagree, and the
//! disagreement is invisible until something it emits fails to resolve.
//!
//! A statement that selects nothing - a tail expression, a `let`, a list
//! comprehension's qualifier - carries `None`, so "no operation selected" is a
//! shape rather than a rule to remember.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use crate::name_chirho::NameChirho;
use crate::provenance_chirho::{OccurrenceRoleChirho, OriginIdChirho};

/// One operation a statement selects: the binding chosen, carrying its own
/// origin, and the role that choice plays.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectedOperationChirho {
    /// The binding selected, as it must be looked up: `>>=` for an ordinary do,
    /// `M.>>=` under `M.do`. It carries the occurrence's origin.
    pub name_chirho: NameChirho,
    /// Which of the three operations this is.
    pub role_chirho: OccurrenceRoleChirho,
}

impl SelectedOperationChirho {
    /// The selection of `name_chirho` in `role_chirho`.
    pub fn new_chirho(name_chirho: NameChirho, role_chirho: OccurrenceRoleChirho) -> Self {
        Self {
            name_chirho,
            role_chirho,
        }
    }

    /// The origin of this selection's occurrence, if it carries one.
    pub fn origin_chirho(&self) -> Option<OriginIdChirho> {
        self.name_chirho.origin_chirho()
    }
}
