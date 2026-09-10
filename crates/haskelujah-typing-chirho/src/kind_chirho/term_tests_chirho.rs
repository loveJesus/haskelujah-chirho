// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Binder equality and substitution are semantic contracts, independent of source spelling.
use super::*;

fn dependent_chirho(result_chirho: KindChirho) -> KindChirho {
    KindChirho::DependentChirho {
        argument_chirho: Box::new(KindChirho::StarChirho),
        result_chirho: Box::new(result_chirho),
    }
}

#[test]
fn dependent_bound_terms_survive_defaulting_and_ambient_substitution_chirho() {
    let bound_chirho = KindChirho::BoundChirho(0);
    let identity_chirho = dependent_chirho(KindChirho::arrow_chirho(
        bound_chirho.clone(),
        bound_chirho.clone(),
    ));
    assert!(
        KindSchemeChirho::generalize_chirho(identity_chirho.clone())
            .quantified_chirho
            .is_empty()
    );
    let ambient_chirho =
        KindSubstChirho::singleton_chirho(KindVarChirho(0), KindChirho::StarChirho);
    assert_eq!(
        ambient_chirho.apply_chirho(&identity_chirho),
        identity_chirho
    );
    assert_eq!(default_kind_vars_chirho(&identity_chirho), identity_chirho);
    let body_chirho = KindChirho::arrow_chirho(bound_chirho.clone(), bound_chirho);
    let supplied_chirho = KindChirho::ConChirho("Bool".into());
    assert_eq!(
        body_chirho.substitute_bound_chirho(&supplied_chirho),
        KindChirho::arrow_chirho(supplied_chirho.clone(), supplied_chirho)
    );
}

#[test]
fn dependent_equality_is_alpha_equivalent_but_does_not_export_local_binders_chirho() {
    let left_chirho = dependent_chirho(
        KindChirho::arrow_chirho(
            KindChirho::VarChirho(KindVarChirho(10)),
            KindChirho::VarChirho(KindVarChirho(10)),
        )
        .abstract_variable_chirho(KindVarChirho(10)),
    );
    let right_chirho = dependent_chirho(
        KindChirho::arrow_chirho(
            KindChirho::VarChirho(KindVarChirho(20)),
            KindChirho::VarChirho(KindVarChirho(20)),
        )
        .abstract_variable_chirho(KindVarChirho(20)),
    );
    assert!(
        unify_kind_chirho(
            &left_chirho,
            &right_chirho,
            "alpha",
            SpanChirho::DUMMY_CHIRHO
        )
        .is_ok()
    );
    for outside_chirho in [
        KindChirho::StarChirho,
        KindChirho::VarChirho(KindVarChirho(30)),
    ] {
        let invalid_chirho = dependent_chirho(KindChirho::arrow_chirho(
            KindChirho::BoundChirho(0),
            outside_chirho,
        ));
        for (first_chirho, second_chirho) in [
            (&left_chirho, &invalid_chirho),
            (&invalid_chirho, &left_chirho),
        ] {
            assert!(
                unify_kind_chirho(
                    first_chirho,
                    second_chirho,
                    "no escape",
                    SpanChirho::DUMMY_CHIRHO
                )
                .is_err(),
                "a local bound term cannot solve a metavariable outside its forall"
            );
        }
    }
}

#[test]
fn bound_substitution_respects_nested_binders_and_shifts_free_arguments_chirho() {
    let nested_chirho = KindChirho::DependentChirho {
        argument_chirho: Box::new(KindChirho::BoundChirho(0)),
        result_chirho: Box::new(KindChirho::arrow_chirho(
            KindChirho::BoundChirho(1),
            KindChirho::BoundChirho(0),
        )),
    };
    assert_eq!(
        nested_chirho.substitute_bound_chirho(&KindChirho::StarChirho),
        dependent_chirho(KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::BoundChirho(0)
        ))
    );
    // A reference to an enclosing binder must not be captured by the nested
    // binder. A supplied quantified argument keeps its own binding positions.
    assert_eq!(
        nested_chirho.substitute_bound_chirho(&KindChirho::BoundChirho(0)),
        nested_chirho
    );
    let argument_chirho = dependent_chirho(KindChirho::BoundChirho(0));
    let body_chirho = dependent_chirho(KindChirho::BoundChirho(1));
    assert_eq!(
        body_chirho.substitute_bound_chirho(&argument_chirho),
        dependent_chirho(argument_chirho)
    );
}
