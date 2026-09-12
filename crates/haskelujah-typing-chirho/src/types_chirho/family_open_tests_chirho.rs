// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::family_open_chirho::{CompatibleOpenRowsChirho, OpenFamilyErrorChirho};
use super::{FamilyReductionChirho, reduce_equations_bounded_chirho};
use crate::ty_chirho::{TyChirho, TyVarChirho};

fn variable_chirho(index_chirho: u32) -> TyChirho {
    TyChirho::VarChirho(TyVarChirho(index_chirho))
}
fn atom_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.into())
}
fn check_chirho(
    rows_chirho: Vec<(Vec<TyChirho>, TyChirho)>,
) -> Result<CompatibleOpenRowsChirho<TyChirho>, OpenFamilyErrorChirho> {
    CompatibleOpenRowsChirho::check_chirho(rows_chirho, &|_| false, &mut 16_384)
}

#[test]
fn open_rows_do_not_share_same_spelled_variable_identities_chirho() {
    let rows_chirho = check_chirho(vec![
        (
            vec![variable_chirho(0), atom_chirho("Int")],
            variable_chirho(0),
        ),
        (
            vec![atom_chirho("Bool"), variable_chirho(0)],
            atom_chirho("Bool"),
        ),
    ])
    .unwrap();
    assert!(
        matches!(rows_chirho.reduce_chirho(&[atom_chirho("Bool"), atom_chirho("Int")], &|_| false, &mut 100), FamilyReductionChirho::ReducedChirho(result_chirho) if result_chirho == atom_chirho("Bool"))
    );
}

#[test]
fn infinite_unifiers_do_not_prove_apartness_chirho() {
    let patterns_chirho = [
        vec![variable_chirho(0), variable_chirho(0)],
        vec![
            TyChirho::ListChirho(Box::new(variable_chirho(0))),
            variable_chirho(0),
        ],
    ];
    assert_eq!(
        check_chirho(vec![
            (patterns_chirho[0].clone(), atom_chirho("Int")),
            (patterns_chirho[1].clone(), atom_chirho("Bool"))
        ])
        .unwrap_err(),
        OpenFamilyErrorChirho::ConflictChirho
    );
    // GHC9.14.1 also rejects this overlap with identical RHSs: infinite
    // unification refutes apartness but is not a finite compatibility witness.
    assert_eq!(
        check_chirho(vec![
            (patterns_chirho[0].clone(), atom_chirho("Int")),
            (patterns_chirho[1].clone(), atom_chirho("Int"))
        ])
        .unwrap_err(),
        OpenFamilyErrorChirho::ConflictChirho
    );
    let mut left_chirho = patterns_chirho[0].clone();
    let mut right_chirho = patterns_chirho[1].clone();
    left_chirho.push(atom_chirho("Int"));
    right_chirho.push(atom_chirho("Bool"));
    assert!(
        check_chirho(vec![
            (left_chirho, atom_chirho("Int")),
            (right_chirho, atom_chirho("Bool")),
        ])
        .is_ok(),
        "a clash in another input still proves apartness"
    );
}

#[test]
fn comparing_results_cannot_unify_them_into_agreement_chirho() {
    let inputs_chirho = vec![variable_chirho(0), variable_chirho(1)];
    assert_eq!(
        check_chirho(vec![
            (inputs_chirho.clone(), variable_chirho(0)),
            (inputs_chirho, variable_chirho(1))
        ])
        .unwrap_err(),
        OpenFamilyErrorChirho::ConflictChirho
    );
}

#[test]
fn open_stuck_specific_row_does_not_shadow_a_compatible_covering_row_chirho() {
    let rows_chirho = vec![
        (vec![atom_chirho("Int")], atom_chirho("Bool")),
        (vec![variable_chirho(0)], atom_chirho("Bool")),
    ];
    assert!(matches!(
        reduce_equations_bounded_chirho(&rows_chirho, &[variable_chirho(90)], &|_| false, &mut 100),
        FamilyReductionChirho::StuckChirho
    ));
    for rows_chirho in [rows_chirho.clone(), rows_chirho.into_iter().rev().collect()] {
        assert!(
            matches!(check_chirho(rows_chirho).unwrap().reduce_chirho(&[variable_chirho(90)], &|_| false, &mut 100), FamilyReductionChirho::ReducedChirho(result_chirho) if result_chirho == atom_chirho("Bool"))
        );
    }
}

#[test]
fn hidden_inputs_participate_in_compatibility_and_reduction_chirho() {
    let rows_chirho = check_chirho(vec![
        (
            vec![atom_chirho("Type"), variable_chirho(0)],
            atom_chirho("Bool"),
        ),
        (
            vec![atom_chirho("Bool"), variable_chirho(0)],
            atom_chirho("Ordering"),
        ),
    ])
    .unwrap();
    assert!(
        matches!(rows_chirho.reduce_chirho(&[atom_chirho("Bool"), atom_chirho("False")], &|_| false, &mut 100), FamilyReductionChirho::ReducedChirho(result_chirho) if result_chirho == atom_chirho("Ordering"))
    );
}

#[test]
fn open_validation_fails_closed_on_unbound_opaque_and_exhausted_inputs_chirho() {
    assert_eq!(
        CompatibleOpenRowsChirho::check_chirho(
            vec![(
                vec![TyChirho::AppChirho(
                    Box::new(atom_chirho("FamilyChirho")),
                    Box::new(atom_chirho("Int"))
                )],
                atom_chirho("Bool")
            )],
            &|name_chirho| name_chirho == "FamilyChirho",
            &mut 100,
        )
        .unwrap_err(),
        OpenFamilyErrorChirho::UnprovedChirho,
        "a one-row family still requires valid inputs; pairwise checking alone proves nothing"
    );
    assert_eq!(
        check_chirho(vec![(vec![variable_chirho(0)], variable_chirho(1))]).unwrap_err(),
        OpenFamilyErrorChirho::UnprovedChirho
    );
    let opaque_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)],
        body_chirho: Box::new(variable_chirho(0)),
    };
    assert_eq!(
        check_chirho(vec![(vec![opaque_chirho], atom_chirho("Int"))]).unwrap_err(),
        OpenFamilyErrorChirho::UnprovedChirho
    );
    assert_eq!(
        CompatibleOpenRowsChirho::check_chirho(
            vec![(vec![variable_chirho(0)], atom_chirho("Int"))],
            &|_| false,
            &mut 0
        )
        .unwrap_err(),
        OpenFamilyErrorChirho::UnprovedChirho
    );
}

#[test]
fn open_reduction_accounts_for_expanded_result_size_chirho() {
    let rows_chirho = check_chirho(vec![(
        vec![variable_chirho(0)],
        TyChirho::TupleChirho(vec![variable_chirho(0); 16]),
    )])
    .unwrap();
    assert!(matches!(
        rows_chirho.reduce_chirho(&[atom_chirho("Int")], &|_| false, &mut 5),
        FamilyReductionChirho::LimitedChirho
    ));
}

#[test]
fn open_reduction_accounts_for_matching_before_a_constant_result_chirho() {
    let input_chirho = TyChirho::TupleChirho(vec![atom_chirho("Int"); 64]);
    let rows_chirho =
        check_chirho(vec![(vec![input_chirho.clone()], atom_chirho("Bool"))]).unwrap();
    assert!(matches!(
        rows_chirho.reduce_chirho(std::slice::from_ref(&input_chirho), &|_| false, &mut 8),
        FamilyReductionChirho::LimitedChirho
    ));
    assert!(
        matches!(rows_chirho.reduce_chirho(&[input_chirho], &|_| false, &mut 200), FamilyReductionChirho::ReducedChirho(result_chirho) if result_chirho == atom_chirho("Bool"))
    );
}
