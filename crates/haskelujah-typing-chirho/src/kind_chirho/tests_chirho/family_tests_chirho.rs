// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn application_chirho(argument_chirho: KindChirho) -> KindChirho {
    KindChirho::app_chirho(
        KindChirho::ConChirho("FamilyChirho".into()),
        argument_chirho,
    )
}

#[test]
fn matching_an_occurrence_does_not_assert_family_injectivity_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    ctx_chirho
        .kind_family_names_chirho
        .insert("FamilyChirho".into());
    let written_chirho = ctx_chirho.fresh_kind_chirho();
    let target_chirho = application_chirho(KindChirho::RigidChirho(KindVarChirho(100)));
    assert!(
        ctx_chirho
            .unify_family_kinds_chirho(
                &application_chirho(written_chirho),
                &target_chirho,
                "test",
                SpanChirho::DUMMY_CHIRHO
            )
            .is_err()
    );
    let scheme_chirho = KindSchemeChirho::generalize_chirho(application_chirho(
        KindChirho::VarChirho(KindVarChirho(101)),
    ));
    let occurrence_chirho = ctx_chirho.open_kind_scheme_chirho(&scheme_chirho, false);
    let solution_chirho = ctx_chirho
        .unify_family_kinds_chirho(
            &occurrence_chirho,
            &target_chirho,
            "test",
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    assert_eq!(
        solution_chirho.apply_chirho(&occurrence_chirho),
        target_chirho
    );
}

#[test]
fn duplicating_family_reduction_reports_a_resource_limit_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    let variable_chirho = ctx_chirho.fresh_kind_chirho();
    ctx_chirho
        .kind_family_names_chirho
        .insert("FamilyChirho".into());
    ctx_chirho.kind_families_chirho.insert(
        "FamilyChirho".into(),
        families_chirho::KindFamilyChirho {
            equations_chirho: vec![(
                vec![variable_chirho.clone()],
                application_chirho(KindChirho::arrow_chirho(
                    variable_chirho.clone(),
                    variable_chirho,
                )),
            )],
            injective_chirho: Vec::new(),
        },
    );
    assert!(matches!(
        ctx_chirho.unify_family_kinds_chirho(
            &application_chirho(KindChirho::StarChirho),
            &KindChirho::StarChirho,
            "test",
            SpanChirho::DUMMY_CHIRHO
        ),
        Err(KindErrorChirho::ReductionLimitChirho { .. })
    ));
}
