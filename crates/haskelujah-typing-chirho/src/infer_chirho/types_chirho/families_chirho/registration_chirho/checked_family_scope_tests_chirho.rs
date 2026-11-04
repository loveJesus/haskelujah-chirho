// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Fused typed default substitution retains simultaneous ownership semantics
//! and refuses growth before materializing repeated borrowed images.

use super::*;
use std::borrow::Cow as CowChirho;

#[test]
fn default_images_are_simultaneous_even_when_numeric_ids_overlap_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let actual_chirho = TyChirho::VarChirho(TyVarChirho(1));
    let other_chirho = TyChirho::ConChirho("Bool".to_owned());
    let images_chirho = HashMap::from([
        (TyVarChirho(0), CowChirho::Borrowed(&actual_chirho)),
        (TyVarChirho(1), CowChirho::Borrowed(&other_chirho)),
    ]);
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(0));
    let (lhs_chirho, rhs_chirho, names_chirho) = context_chirho
        .specialize_checked_family_terms_chirho(
            std::slice::from_ref(&variable_chirho),
            &variable_chirho,
            &images_chirho,
            &mut 100_000,
        )
        .expect("actual meta does not reenter template substitution");
    assert_eq!(
        lhs_chirho,
        vec![TyChirho::ForallVarChirho(names_chirho[0].clone())]
    );
    assert_eq!(lhs_chirho[0], rhs_chirho);
}

#[test]
fn repeated_borrowed_default_images_share_one_growth_allowance_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let actual_chirho = TyChirho::ConChirho("x".repeat(60_000));
    let images_chirho = HashMap::from([(TyVarChirho(0), CowChirho::Borrowed(&actual_chirho))]);
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(0));
    assert!(
        context_chirho
            .specialize_checked_family_terms_chirho(
                &[variable_chirho.clone(), variable_chirho.clone()],
                &variable_chirho,
                &images_chirho,
                &mut 100_000
            )
            .is_none()
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
}

#[test]
fn a_deep_default_image_is_refused_before_unbounded_recursion_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    let mut actual_chirho = TyChirho::ConChirho("Bool".to_owned());
    for _depth_chirho in 0..129 {
        actual_chirho = TyChirho::ListChirho(Box::new(actual_chirho));
    }
    let images_chirho = HashMap::from([(TyVarChirho(0), CowChirho::Borrowed(&actual_chirho))]);
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(0));
    assert!(
        context_chirho
            .specialize_checked_family_terms_chirho(
                std::slice::from_ref(&variable_chirho),
                &variable_chirho,
                &images_chirho,
                &mut 100_000
            )
            .is_none()
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
}

#[test]
fn default_rhs_variables_require_a_checked_lhs_binder_chirho() {
    let mut context_chirho = InferCtxChirho::new_chirho();
    assert!(
        context_chirho
            .abstract_checked_family_terms_chirho(
                &[TyChirho::ConChirho("Bool".to_owned())],
                &TyChirho::VarChirho(TyVarChirho(9))
            )
            .is_none()
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
}
