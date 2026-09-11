// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::ty_chirho::{TyChirho, TyVarChirho};

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
            &|_name_chirho| false
        ),
        Ok(false)
    );
}
