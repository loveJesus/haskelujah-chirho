// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::ty_chirho::{TyChirho, TyVarChirho};

fn reduce_equations_chirho<TermChirho: FamilyTermChirho>(
    equations_chirho: &[(Vec<TermChirho>, TermChirho)],
    arguments_chirho: &[TermChirho],
    family_chirho: &impl Fn(&str) -> bool,
) -> FamilyReductionChirho<TermChirho> {
    reduce_equations_bounded_chirho(
        equations_chirho,
        arguments_chirho,
        family_chirho,
        &mut 16_384,
    )
}

fn family_application_chirho(name_chirho: &str, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::application_chirho(TyChirho::ConChirho(name_chirho.into()), argument_chirho)
}

#[test]
fn imported_family_matching_preserves_kind_input_visibility_chirho() {
    use crate::infer_chirho::{InferCtxChirho, TypeFamilyClauseChirho, TypeFamilyEnvChirho};
    let equations_chirho = TypeFamilyEnvChirho::from([(
        "PickChirho".to_owned(),
        vec![
            TypeFamilyClauseChirho {
                kind_inputs_chirho: vec![TyChirho::ConChirho("Type".into())],
                type_inputs_chirho: vec![],
                result_chirho: TyChirho::ConChirho("Maybe".into()),
            },
            TypeFamilyClauseChirho {
                kind_inputs_chirho: vec![TyChirho::bool_chirho()],
                type_inputs_chirho: vec![],
                result_chirho: TyChirho::ConChirho("FlagChirho".into()),
            },
        ],
    )]);
    let mut context_chirho = InferCtxChirho::new_chirho();
    for (name_chirho, equations_chirho) in equations_chirho {
        context_chirho.register_elaborated_type_family_chirho(name_chirho, equations_chirho);
    }
    let application_chirho = |argument_chirho| {
        TyChirho::KindAppChirho(
            Box::new(TyChirho::ConChirho("PickChirho".into())),
            Box::new(argument_chirho),
        )
    };
    assert_eq!(
        context_chirho
            .reduce_type_families_in_ty_chirho(&application_chirho(TyChirho::bool_chirho())),
        TyChirho::ConChirho("FlagChirho".into())
    );
    let ordinary_chirho = family_application_chirho("PickChirho", TyChirho::bool_chirho());
    assert_eq!(
        context_chirho.reduce_type_families_in_ty_chirho(&ordinary_chirho),
        ordinary_chirho,
        "a transported hidden input cannot be consumed as an ordinary argument"
    );
    let unsolved_chirho = application_chirho(TyChirho::VarChirho(TyVarChirho(3100)));
    assert_eq!(
        context_chirho.reduce_type_families_in_ty_chirho(&unsolved_chirho),
        unsolved_chirho,
        "an unknown kind must not select whichever row was imported first"
    );
}

#[test]
fn a_covering_composition_needs_each_validated_injective_link_chirho() {
    let variable_chirho = TyChirho::ForallVarChirho("valueChirho".into());
    let rows_chirho = vec![(
        vec![variable_chirho.clone()],
        family_application_chirho(
            "OuterChirho",
            family_application_chirho("InnerChirho", variable_chirho.clone()),
        ),
    )];
    let family_chirho = |name_chirho: &str| matches!(name_chirho, "OuterChirho" | "InnerChirho");
    let proved_chirho = |name_chirho: &str, arity_chirho: usize| {
        (family_chirho(name_chirho) && arity_chirho == 1).then_some(&[0][..])
    };
    assert_eq!(
        family_injectivity_chirho::validate_injectivity_chirho(
            &rows_chirho,
            &[0],
            &family_chirho,
            &proved_chirho,
        ),
        Ok(true),
    );
    for missing_chirho in ["OuterChirho", "InnerChirho"] {
        assert!(
            family_injectivity_chirho::validate_injectivity_chirho(
                &rows_chirho,
                &[0],
                &family_chirho,
                &|name_chirho, arity_chirho| {
                    if name_chirho == missing_chirho {
                        None
                    } else {
                        proved_chirho(name_chirho, arity_chirho)
                    }
                },
            )
            .is_err(),
            "an unproved link must not be assumed injective",
        );
    }
    assert!(
        family_injectivity_chirho::validate_injectivity_chirho(
            &rows_chirho,
            &[0],
            &family_chirho,
            &|name_chirho, arity_chirho| {
                if name_chirho == "InnerChirho" {
                    Some(&[][..])
                } else {
                    proved_chirho(name_chirho, arity_chirho)
                }
            },
        )
        .is_err(),
        "a non-injective inner argument cannot be recovered through an injective outer family",
    );
    let oversaturated_chirho = vec![(
        vec![variable_chirho.clone()],
        TyChirho::application_chirho(rows_chirho[0].1.clone(), variable_chirho),
    )];
    assert!(
        family_injectivity_chirho::validate_injectivity_chirho(
            &oversaturated_chirho,
            &[0],
            &family_chirho,
            &proved_chirho,
        )
        .is_err(),
        "a proof of one saturated call says nothing about its function-valued result",
    );
}

#[test]
fn composed_injectivity_has_a_local_work_limit_chirho() {
    let variable_chirho = TyChirho::ForallVarChirho("valueChirho".into());
    let result_chirho = TyChirho::TupleChirho(vec![
        family_application_chirho(
            "InnerChirho",
            variable_chirho.clone()
        );
        16_384
    ]);
    assert_eq!(
        family_injectivity_chirho::validate_injectivity_chirho(
            &[(vec![variable_chirho], result_chirho)],
            &[0],
            &|name_chirho| name_chirho == "InnerChirho",
            &|name_chirho, arity_chirho| {
                (name_chirho == "InnerChirho" && arity_chirho == 1).then_some(&[0][..])
            },
        ),
        Ok(false),
        "exhausting the local proof budget is unproved, not permission to improve a kind",
    );
}

#[test]
fn a_duplicating_equation_spends_output_budget_before_allocation_chirho() {
    let rows_chirho = vec![(
        vec![TyChirho::ForallVarChirho("aChirho".into())],
        TyChirho::TupleChirho(vec![TyChirho::ForallVarChirho("aChirho".into()); 16]),
    )];
    let argument_chirho = TyChirho::TupleChirho(vec![TyChirho::int_chirho(); 16]);
    let mut budget_chirho = 64;
    assert!(matches!(
        reduce_equations_bounded_chirho(
            &rows_chirho,
            &[argument_chirho],
            &|_name_chirho| false,
            &mut budget_chirho
        ),
        FamilyReductionChirho::LimitedChirho
    ));
    assert_eq!(budget_chirho, 0);
}

#[test]
fn numeric_and_written_family_variables_are_distinct_chirho() {
    let rows_chirho = vec![(
        vec![
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::ForallVarChirho("tv0".into()),
        ],
        TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(TyVarChirho(0)),
            TyChirho::ForallVarChirho("tv0".into()),
        ]),
    )];
    let result_chirho = reduce_equations_chirho(
        &rows_chirho,
        &[TyChirho::int_chirho(), TyChirho::bool_chirho()],
        &|_name_chirho| false,
    );
    assert!(
        matches!(result_chirho, FamilyReductionChirho::ReducedChirho(result_chirho) if result_chirho == TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]))
    );
}

#[test]
fn a_blocked_earlier_equation_cannot_select_a_catchall_chirho() {
    let rows_chirho = vec![
        (vec![TyChirho::int_chirho()], TyChirho::bool_chirho()),
        (
            vec![TyChirho::ForallVarChirho("aChirho".into())],
            TyChirho::char_chirho(),
        ),
    ];
    assert!(matches!(
        reduce_equations_chirho(
            &rows_chirho,
            &[TyChirho::VarChirho(TyVarChirho(3))],
            &|_name_chirho| false
        ),
        FamilyReductionChirho::StuckChirho
    ));
    assert!(
        matches!(reduce_equations_chirho(&rows_chirho, &[TyChirho::bool_chirho()], &|_name_chirho| false), FamilyReductionChirho::ReducedChirho(result_chirho) if result_chirho == TyChirho::char_chirho())
    );
}

#[test]
fn opaque_terms_cannot_certify_injectivity_chirho() {
    let opaque_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(1)],
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(2))),
    };
    assert_eq!(
        family_injectivity_chirho::validate_injectivity_chirho(
            &[(vec![opaque_chirho], TyChirho::int_chirho())],
            &[0],
            &|_name_chirho| false,
            &|_name_chirho, _arity_chirho| None
        ),
        Ok(false)
    );
}
