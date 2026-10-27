// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The `Eq`, `Show` and `Ord` instances of tuples, at every arity GHC's base gives them
//! (2 to 15), and the arity bound the dictionary pass composes tuple rows to (row 604).
//! Before, only pairs and triples had instances: `print` of a 4-tuple, or `==` on one,
//! failed to type-check.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{ClassEnvChirho, InstDeclChirho, PredChirho};
use crate::ty_chirho::{TyChirho, TyVarChirho};

/// The largest tuple GHC's base gives `Eq`, `Show` and `Ord` instances.
pub const MAX_TUPLE_INSTANCE_ARITY_CHIRHO: usize = 15;

/// The first of the type variables the tuple instance heads use, one per component.
const TUPLE_COMPONENT_VAR_BASE_CHIRHO: u32 = 9910;

impl ClassEnvChirho {
    /// `instance (C a1, …, C an) => C (a1, …, an)` for `Eq`, `Show` and `Ord`, at every
    /// arity from 2 to [`MAX_TUPLE_INSTANCE_ARITY_CHIRHO`].
    pub(super) fn seed_tuple_instances_chirho(&mut self) {
        for arity_chirho in 2..=MAX_TUPLE_INSTANCE_ARITY_CHIRHO {
            let components_chirho = (0..arity_chirho as u32)
                .map(|index_chirho| {
                    TyChirho::VarChirho(TyVarChirho(TUPLE_COMPONENT_VAR_BASE_CHIRHO + index_chirho))
                })
                .collect::<Vec<_>>();
            for class_chirho in ["Eq", "Show", "Ord"] {
                self.add_instance_chirho(InstDeclChirho {
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::TupleChirho(components_chirho.clone()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: components_chirho
                        .iter()
                        .map(|component_chirho| {
                            PredChirho::new_chirho(class_chirho, component_chirho.clone())
                        })
                        .collect(),
                });
            }
        }
    }
}

#[cfg(test)]
#[path = "tuple_instances_tests_chirho.rs"]
mod tests_chirho;
