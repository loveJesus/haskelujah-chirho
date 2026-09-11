// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::ty_chirho::{TyChirho, TyVarChirho};

fn family_application_chirho(name_chirho: &str, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::application_chirho(TyChirho::ConChirho(name_chirho.into()), argument_chirho)
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
