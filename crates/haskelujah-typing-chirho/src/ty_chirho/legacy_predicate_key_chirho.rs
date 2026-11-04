// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Borrowed iterative implementation of the existing public predicate equality
//! relation. It changes neither public Eq/Hash nor type representation.
use super::{PredicateRecordChirho, TyChirho};
use std::hash::{Hash, Hasher};

pub(super) fn fingerprint_chirho(predicate_chirho: &impl PredicateRecordChirho) -> u64 {
    let mut hasher_chirho = std::collections::hash_map::DefaultHasher::new();
    predicate_chirho
        .class_name_chirho()
        .hash(&mut hasher_chirho);
    let terms_chirho = predicate_chirho.terms_chirho();
    terms_chirho.kind_tys_chirho.len().hash(&mut hasher_chirho);
    for term_chirho in terms_chirho.kind_tys_chirho {
        hash_term_chirho(term_chirho, &mut hasher_chirho);
    }
    hash_term_chirho(terms_chirho.ty_chirho, &mut hasher_chirho);
    terms_chirho.extra_tys_chirho.len().hash(&mut hasher_chirho);
    for term_chirho in terms_chirho.extra_tys_chirho {
        hash_term_chirho(term_chirho, &mut hasher_chirho);
    }
    hasher_chirho.finish()
}

pub(super) fn equal_chirho(
    left_chirho: &impl PredicateRecordChirho,
    right_chirho: &impl PredicateRecordChirho,
) -> bool {
    if left_chirho.class_name_chirho() != right_chirho.class_name_chirho() {
        return false;
    }
    let left_chirho = left_chirho.terms_chirho();
    let right_chirho = right_chirho.terms_chirho();
    if left_chirho.kind_tys_chirho.len() != right_chirho.kind_tys_chirho.len()
        || left_chirho.extra_tys_chirho.len() != right_chirho.extra_tys_chirho.len()
    {
        return false;
    }
    left_chirho
        .iter_chirho()
        .zip(right_chirho.iter_chirho())
        .all(|(left_chirho, right_chirho)| equal_term_chirho(left_chirho, right_chirho))
}

fn hash_term_chirho(term_chirho: &TyChirho, hasher_chirho: &mut impl Hasher) {
    let mut pending_chirho = vec![term_chirho];
    while let Some(term_chirho) = pending_chirho.pop() {
        match term_chirho {
            TyChirho::VarChirho(variable_chirho) => {
                0u8.hash(hasher_chirho);
                variable_chirho.hash(hasher_chirho);
            }
            TyChirho::ConChirho(name_chirho) => {
                1u8.hash(hasher_chirho);
                name_chirho.hash(hasher_chirho);
            }
            TyChirho::AppChirho(function_chirho, argument_chirho) => {
                2u8.hash(hasher_chirho);
                pending_chirho.push(argument_chirho);
                pending_chirho.push(function_chirho);
            }
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                3u8.hash(hasher_chirho);
                multiplicity_chirho.hash(hasher_chirho);
                pending_chirho.push(result_chirho);
                pending_chirho.push(argument_chirho);
            }
            TyChirho::TupleChirho(elements_chirho) => {
                4u8.hash(hasher_chirho);
                elements_chirho.len().hash(hasher_chirho);
                pending_chirho.extend(elements_chirho.iter().rev());
            }
            TyChirho::ListChirho(element_chirho) => {
                5u8.hash(hasher_chirho);
                pending_chirho.push(element_chirho);
            }
            TyChirho::ForallVarChirho(name_chirho) => {
                6u8.hash(hasher_chirho);
                name_chirho.hash(hasher_chirho);
            }
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                7u8.hash(hasher_chirho);
                vars_chirho.hash(hasher_chirho);
                pending_chirho.push(body_chirho);
            }
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => {
                8u8.hash(hasher_chirho);
                vars_chirho.hash(hasher_chirho);
                pending_chirho.push(body_chirho);
            }
        }
    }
}

fn equal_term_chirho(left_chirho: &TyChirho, right_chirho: &TyChirho) -> bool {
    let mut pending_chirho = vec![(left_chirho, right_chirho)];
    while let Some((left_chirho, right_chirho)) = pending_chirho.pop() {
        match (left_chirho, right_chirho) {
            (TyChirho::VarChirho(left_chirho), TyChirho::VarChirho(right_chirho))
                if left_chirho == right_chirho => {}
            (TyChirho::ConChirho(left_chirho), TyChirho::ConChirho(right_chirho))
            | (TyChirho::ForallVarChirho(left_chirho), TyChirho::ForallVarChirho(right_chirho))
                if left_chirho == right_chirho => {}
            (
                TyChirho::AppChirho(left_function_chirho, left_argument_chirho),
                TyChirho::AppChirho(right_function_chirho, right_argument_chirho),
            ) => {
                pending_chirho.push((left_argument_chirho, right_argument_chirho));
                pending_chirho.push((left_function_chirho, right_function_chirho));
            }
            (
                TyChirho::FunChirho(left_argument_chirho, left_result_chirho, left_mult_chirho),
                TyChirho::FunChirho(right_argument_chirho, right_result_chirho, right_mult_chirho),
            ) if left_mult_chirho == right_mult_chirho => {
                pending_chirho.push((left_argument_chirho, right_argument_chirho));
                pending_chirho.push((left_result_chirho, right_result_chirho));
            }
            (TyChirho::TupleChirho(left_chirho), TyChirho::TupleChirho(right_chirho))
                if left_chirho.len() == right_chirho.len() =>
            {
                pending_chirho.extend(left_chirho.iter().zip(right_chirho))
            }
            (TyChirho::ListChirho(left_chirho), TyChirho::ListChirho(right_chirho)) => {
                pending_chirho.push((left_chirho, right_chirho))
            }
            (
                TyChirho::ForallChirho {
                    vars_chirho: left_vars_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::ForallChirho {
                    vars_chirho: right_vars_chirho,
                    body_chirho: right_body_chirho,
                },
            )
            | (
                TyChirho::RequiredForallChirho {
                    vars_chirho: left_vars_chirho,
                    body_chirho: left_body_chirho,
                },
                TyChirho::RequiredForallChirho {
                    vars_chirho: right_vars_chirho,
                    body_chirho: right_body_chirho,
                },
            ) if left_vars_chirho == right_vars_chirho => {
                pending_chirho.push((left_body_chirho, right_body_chirho))
            }
            _ => return false,
        }
    }
    true
}
