// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! What an H98 constructor writes before its name (row 694, #31811).
//!
//! `data T a = forall b. (Show b, a ~ Int) => MkT a b` gives `MkT` the existential variable `b`
//! and a context. GHC's role inference reads both: the fields' `b` is not the parameter, and
//! every type variable the context mentions is nominal. Root's measured
//! `ConstructorContextRoleChirho` shows it: `forall . (aChirho ~ Int) =>` makes `HoldChirho`'s
//! parameter nominal, and GND through it is GHC-18872.
//!
//! The lowering keeps both here. A prefix it could not read, such as a kinded binder, is
//! unread, and so is a GADT record's signature, which no shape of this carrier holds: a
//! consumer then knows nothing about the constructor's quantification, never that it has none.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use crate::name_chirho::NameChirho;
use crate::ty_chirho::ConstraintChirho;

/// The `forall vs. ctx =>` an H98 constructor writes before its name.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConPrefixChirho {
    /// The variables the `forall` binds, which the fields and the context may use.
    pub existential_vars_chirho: Vec<NameChirho>,
    /// The constraints before `=>`, which the constructor stores.
    pub context_chirho: Vec<ConstraintChirho>,
    /// Whether the constructor's quantification holds syntax the lowering did not read.
    pub unread_chirho: bool,
}

impl ConPrefixChirho {
    /// A quantification the lowering could not read: nothing about it is known.
    pub fn unread_quantification_chirho() -> Self {
        Self {
            unread_chirho: true,
            ..Self::default()
        }
    }

    /// Whether the constructor writes no quantification at all.
    pub fn is_empty_chirho(&self) -> bool {
        self.existential_vars_chirho.is_empty()
            && self.context_chirho.is_empty()
            && !self.unread_chirho
    }
}
