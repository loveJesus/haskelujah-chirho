// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The applied type a constrained reference's predicate resolved to (row 694, P1, P2).
//!
//! A reference's evidence names the instance head key its predicate solved to: `BoxChirho`
//! for `Show (BoxChirho Int)`. The key names a conditional instance but not its arguments.
//! When the predicate's type is a ground application, the evidence also carries that type, so
//! the dictionary pass can apply the instance's factory to its context's dictionaries at the
//! actual arguments. The type is structure the checker solved, never recovered from the key,
//! and it never decides whether a reference's evidence agrees: captures that agree on every
//! class and key keep their evidence, and only an applied type every capture resolved alike
//! survives.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;
use std::hash::Hash;

use super::ReferenceEvidenceChirho;
use crate::ty_chirho::TyChirho;

/// The applied type evidence carries for a predicate resolved to `ty_chirho`: the type itself
/// when it applies a type constructor to arguments and names no type variable.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(super) fn applied_evidence_ty_chirho(ty_chirho: &TyChirho) -> Option<TyChirho> {
    (matches!(ty_chirho, TyChirho::AppChirho(..)) && ground_ty_chirho(ty_chirho))
        .then(|| ty_chirho.clone())
}

/// Whether `ty_chirho` names no type variable, rigid or not.
fn ground_ty_chirho(ty_chirho: &TyChirho) -> bool {
    match ty_chirho {
        TyChirho::ConChirho(_) => true,
        TyChirho::AppChirho(left_chirho, right_chirho)
        | TyChirho::FunChirho(left_chirho, right_chirho, _) => {
            ground_ty_chirho(left_chirho) && ground_ty_chirho(right_chirho)
        }
        TyChirho::ListChirho(element_chirho) => ground_ty_chirho(element_chirho),
        TyChirho::TupleChirho(elements_chirho) => elements_chirho.iter().all(ground_ty_chirho),
        TyChirho::VarChirho(_)
        | TyChirho::ForallVarChirho(_)
        | TyChirho::ForallChirho { .. }
        | TyChirho::RequiredForallChirho { .. } => false,
    }
}

/// Keep a reference's evidence only when every capture agrees on each predicate's class and
/// key. A predicate keeps its applied type only where every capture resolved the same one.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(super) fn agreeing_references_chirho<KeyChirho: Hash + Eq>(
    captures_chirho: HashMap<KeyChirho, Vec<Vec<ReferenceEvidenceChirho>>>,
) -> HashMap<KeyChirho, Vec<ReferenceEvidenceChirho>> {
    captures_chirho
        .into_iter()
        .filter_map(|(key_chirho, records_chirho)| {
            let (first_chirho, rest_chirho) = records_chirho.split_first()?;
            let proves_alike_chirho = |other_chirho: &Vec<ReferenceEvidenceChirho>| {
                other_chirho.len() == first_chirho.len()
                    && other_chirho.iter().zip(first_chirho).all(
                        |(other_record_chirho, record_chirho)| {
                            other_record_chirho.class_name_chirho == record_chirho.class_name_chirho
                                && other_record_chirho.ty_key_chirho == record_chirho.ty_key_chirho
                        },
                    )
            };
            if !rest_chirho.iter().all(proves_alike_chirho) {
                return None;
            }
            let mut agreed_chirho = first_chirho.clone();
            for (index_chirho, record_chirho) in agreed_chirho.iter_mut().enumerate() {
                if rest_chirho.iter().any(|other_chirho| {
                    other_chirho[index_chirho].applied_ty_chirho != record_chirho.applied_ty_chirho
                }) {
                    record_chirho.applied_ty_chirho = None;
                }
            }
            Some((key_chirho, agreed_chirho))
        })
        .collect()
}

#[cfg(test)]
#[path = "applied_evidence_tests_chirho.rs"]
mod tests_chirho;
