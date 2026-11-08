// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pure reduction controls for the measured B02 compatible overlap, T8044/B07 stuck
//! fallback boundary, and ordinary one-way matching. These do not claim frontend closure.

use super::closed_family_apartness_chirho::{
    FamilyHeadRoleChirho, GenerativeIdentityChirho, family_branches_compatible_chirho,
};
use super::family_definitions_chirho::{
    IntrinsicTypeFamilyChirho, TypeFamilyBranchChirho, TypeFamilyBranchIdChirho,
    TypeFamilyDefinitionChirho, TypeFamilyIdChirho, TypeFamilyReductionChirho,
};
use super::family_reduction_chirho::{
    FamilyReductionDecisionChirho, FamilyReductionLimitsChirho, decide_family_with_limits_chirho,
    reduce_family_definition_chirho, reduce_family_with_limits_chirho,
};
use crate::ty_chirho::{TyChirho, TyVarChirho};

fn con_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_owned())
}
fn var_chirho(index_chirho: u32) -> TyChirho {
    TyChirho::VarChirho(TyVarChirho(index_chirho))
}
fn named_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ForallVarChirho(name_chirho.to_owned())
}
fn app_chirho(function_chirho: TyChirho, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(Box::new(function_chirho), Box::new(argument_chirho))
}
fn list_chirho(element_chirho: TyChirho) -> TyChirho {
    TyChirho::ListChirho(Box::new(element_chirho))
}

fn classify_chirho(name_chirho: &str) -> FamilyHeadRoleChirho {
    match name_chirho {
        "GChirho" | "AliasGChirho" => FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: "GChirho".to_owned(),
            arity_chirho: 1,
        },
        "OpaqueChirho" => FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: "OpaqueChirho".to_owned(),
            arity_chirho: 1,
        },
        "AChirho.TChirho" | "AliasChirho.TChirho" => FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho("AChirho.TChirho".to_owned()),
        },
        "BChirho.TChirho" => FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho("BChirho.TChirho".to_owned()),
        },
        "Int" | "Bool" | "Char" | "Maybe" | "True" | "False" => {
            FamilyHeadRoleChirho::GenerativeChirho {
                identity_chirho: GenerativeIdentityChirho::NominalChirho(name_chirho.to_owned()),
            }
        }
        _ => FamilyHeadRoleChirho::UnknownChirho,
    }
}

fn family_id_chirho() -> TypeFamilyIdChirho {
    TypeFamilyIdChirho::SourceChirho {
        module_chirho: "DefinitionChirho".to_owned(),
        name_chirho: "FChirho".to_owned(),
    }
}

fn branch_chirho(
    index_chirho: u32,
    lhs_chirho: Vec<TyChirho>,
    rhs_chirho: TyChirho,
    blockers_chirho: Vec<usize>,
) -> TypeFamilyBranchChirho {
    TypeFamilyBranchChirho {
        id_chirho: TypeFamilyBranchIdChirho {
            family_chirho: family_id_chirho(),
            producer_module_chirho: "DefinitionChirho".to_owned(),
            declaration_start_chirho: 0,
            equation_index_chirho: index_chirho,
        },
        binders_chirho: Vec::new(),
        lhs_chirho,
        rhs_chirho,
        blockers_chirho,
        held_chirho: false,
    }
}

fn closed_chirho(
    arity_chirho: usize,
    branches_chirho: Vec<TypeFamilyBranchChirho>,
) -> TypeFamilyDefinitionChirho {
    TypeFamilyDefinitionChirho::new_chirho(
        family_id_chirho(),
        arity_chirho,
        TypeFamilyReductionChirho::ClosedChirho,
        branches_chirho,
    )
    .unwrap()
}

fn reduce_chirho(
    definition_chirho: &TypeFamilyDefinitionChirho,
    arguments_chirho: &[TyChirho],
) -> Option<TyChirho> {
    reduce_family_definition_chirho(definition_chirho, arguments_chirho, &classify_chirho)
}

fn compatible_chirho(
    left_chirho: &TypeFamilyBranchChirho,
    right_chirho: &TypeFamilyBranchChirho,
) -> bool {
    family_branches_compatible_chirho(
        &left_chirho.lhs_chirho,
        &left_chirho.rhs_chirho,
        &right_chirho.lhs_chirho,
        &right_chirho.rhs_chirho,
        &classify_chirho,
    )
}

#[test]
fn measured_b02_compatible_overlap_can_pass_an_earlier_nonmatch_chirho() {
    let definition_chirho = closed_chirho(
        2,
        vec![
            branch_chirho(
                0,
                vec![con_chirho("True"), named_chirho("bChirho")],
                named_chirho("bChirho"),
                vec![],
            ),
            branch_chirho(
                1,
                vec![named_chirho("aChirho"), con_chirho("True")],
                named_chirho("aChirho"),
                vec![],
            ),
            branch_chirho(
                2,
                vec![var_chirho(0), var_chirho(1)],
                con_chirho("False"),
                vec![0, 1],
            ),
        ],
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[var_chirho(42), con_chirho("True")]),
        Some(var_chirho(42))
    );
    assert_eq!(
        reduce_chirho(
            &definition_chirho,
            &[con_chirho("False"), con_chirho("True")]
        ),
        Some(con_chirho("False"))
    );
    assert_eq!(
        reduce_chirho(
            &definition_chirho,
            &[con_chirho("True"), con_chirho("False")]
        ),
        Some(con_chirho("False"))
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[var_chirho(0), var_chirho(1)]),
        None
    );
}

#[test]
fn measured_c08_is_blocked_while_shared_c07_still_reduces_chirho() {
    let definition_chirho = closed_chirho(
        2,
        vec![
            branch_chirho(
                0,
                vec![con_chirho("Int"), con_chirho("Bool")],
                con_chirho("Char"),
                vec![],
            ),
            branch_chirho(
                1,
                vec![named_chirho("xChirho"), named_chirho("yChirho")],
                con_chirho("Int"),
                vec![0],
            ),
        ],
    );
    let first_chirho = app_chirho(con_chirho("OpaqueChirho"), named_chirho("aChirho%1"));
    let second_chirho = app_chirho(con_chirho("OpaqueChirho"), named_chirho("bChirho%2"));
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[first_chirho.clone(), second_chirho.clone()],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        FamilyReductionDecisionChirho::BlockedChirho
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[first_chirho.clone(), second_chirho]),
        None
    );
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[first_chirho.clone(), first_chirho],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        FamilyReductionDecisionChirho::ReducedChirho(con_chirho("Int"))
    );
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[con_chirho("UnknownChirho"), con_chirho("Bool")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        FamilyReductionDecisionChirho::UnknownChirho
    );
}

#[test]
fn t8044_b07_fallback_stays_blocked_until_the_argument_is_apart_chirho() {
    let definition_chirho = closed_chirho(
        1,
        vec![
            branch_chirho(0, vec![con_chirho("Int")], con_chirho("Int"), vec![]),
            branch_chirho(1, vec![var_chirho(0)], con_chirho("Char"), vec![0]),
        ],
    );
    assert_eq!(reduce_chirho(&definition_chirho, &[var_chirho(0)]), None);
    assert_eq!(
        reduce_chirho(&definition_chirho, &[named_chirho("rigidChirho")]),
        None
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Int")]),
        Some(con_chirho("Int"))
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Bool")]),
        Some(con_chirho("Char"))
    );
    assert_eq!(
        reduce_chirho(
            &definition_chirho,
            &[app_chirho(con_chirho("GChirho"), con_chirho("Int"))]
        ),
        None
    );
}

#[test]
fn matching_never_assigns_target_metas_or_named_rigids_chirho() {
    let specific_chirho = closed_chirho(
        1,
        vec![branch_chirho(
            0,
            vec![con_chirho("Int")],
            con_chirho("Bool"),
            vec![],
        )],
    );
    assert_eq!(reduce_chirho(&specific_chirho, &[var_chirho(7)]), None);
    assert_eq!(
        reduce_chirho(&specific_chirho, &[named_chirho("rigidChirho")]),
        None
    );
    let generic_chirho = closed_chirho(
        1,
        vec![branch_chirho(0, vec![var_chirho(7)], var_chirho(7), vec![])],
    );
    assert_eq!(
        reduce_chirho(&generic_chirho, &[var_chirho(7)]),
        Some(var_chirho(7))
    );
    assert_eq!(
        reduce_chirho(&generic_chirho, &[named_chirho("rigidChirho")]),
        Some(named_chirho("rigidChirho"))
    );
}

#[test]
fn repeated_patterns_preserve_target_variable_sharing_chirho() {
    let definition_chirho = closed_chirho(
        2,
        vec![
            branch_chirho(
                0,
                vec![var_chirho(0), var_chirho(0)],
                con_chirho("True"),
                vec![],
            ),
            branch_chirho(
                1,
                vec![var_chirho(1), var_chirho(2)],
                con_chirho("False"),
                vec![0],
            ),
        ],
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Int"), con_chirho("Bool")]),
        Some(con_chirho("False"))
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[var_chirho(42), var_chirho(42)]),
        Some(con_chirho("True"))
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[var_chirho(42), var_chirho(43)]),
        None
    );
}

#[test]
fn normalized_wildcards_are_distinct_and_named_numeric_keys_cannot_collide_chirho() {
    let wildcards_chirho = closed_chirho(
        2,
        vec![branch_chirho(
            0,
            vec![
                named_chirho("%wildcard_0_chirho"),
                named_chirho("%wildcard_1_chirho"),
            ],
            con_chirho("True"),
            vec![],
        )],
    );
    assert_eq!(
        reduce_chirho(&wildcards_chirho, &[con_chirho("Int"), con_chirho("Bool")]),
        Some(con_chirho("True"))
    );
    let distinct_chirho = closed_chirho(
        2,
        vec![branch_chirho(
            0,
            vec![var_chirho(0), named_chirho("tv0")],
            TyChirho::TupleChirho(vec![var_chirho(0), named_chirho("tv0")]),
            vec![],
        )],
    );
    assert_eq!(
        reduce_chirho(&distinct_chirho, &[con_chirho("Int"), con_chirho("Bool")]),
        Some(TyChirho::TupleChirho(vec![
            con_chirho("Int"),
            con_chirho("Bool")
        ]))
    );
}

#[test]
fn higher_kinded_pattern_matches_list_sugar_without_binding_the_target_chirho() {
    let definition_chirho = closed_chirho(
        1,
        vec![branch_chirho(
            0,
            vec![app_chirho(var_chirho(0), var_chirho(1))],
            var_chirho(1),
            vec![],
        )],
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[list_chirho(con_chirho("Int"))]),
        Some(con_chirho("Int"))
    );
    assert_eq!(
        reduce_chirho(
            &definition_chirho,
            &[app_chirho(con_chirho("[]"), var_chirho(42))]
        ),
        Some(var_chirho(42))
    );
    assert_eq!(reduce_chirho(&definition_chirho, &[var_chirho(42)]), None);
}

#[test]
fn tuple_and_arrow_views_preserve_structure_and_multiplicity_chirho() {
    let tuple_chirho = closed_chirho(
        1,
        vec![branch_chirho(
            0,
            vec![app_chirho(
                app_chirho(con_chirho("(,)"), var_chirho(0)),
                var_chirho(1),
            )],
            TyChirho::TupleChirho(vec![var_chirho(1), var_chirho(0)]),
            vec![],
        )],
    );
    assert_eq!(
        reduce_chirho(
            &tuple_chirho,
            &[TyChirho::TupleChirho(vec![
                con_chirho("Int"),
                con_chirho("Bool")
            ])]
        ),
        Some(TyChirho::TupleChirho(vec![
            con_chirho("Bool"),
            con_chirho("Int")
        ]))
    );
    let arrows_chirho = closed_chirho(
        1,
        vec![
            branch_chirho(
                0,
                vec![TyChirho::linear_fun_chirho(var_chirho(0), var_chirho(1))],
                list_chirho(var_chirho(0)),
                vec![],
            ),
            branch_chirho(1, vec![var_chirho(2)], con_chirho("Char"), vec![0]),
        ],
    );
    assert_eq!(
        reduce_chirho(
            &arrows_chirho,
            &[TyChirho::linear_fun_chirho(
                con_chirho("Int"),
                con_chirho("Bool")
            )]
        ),
        Some(list_chirho(con_chirho("Int")))
    );
    assert_eq!(
        reduce_chirho(
            &arrows_chirho,
            &[TyChirho::fun_chirho(con_chirho("Int"), con_chirho("Bool"))]
        ),
        Some(con_chirho("Char"))
    );
}

#[test]
fn substitution_restores_all_sugar_instead_of_leaking_application_views_chirho() {
    let identity_chirho = closed_chirho(
        1,
        vec![branch_chirho(0, vec![var_chirho(0)], var_chirho(0), vec![])],
    );
    for target_chirho in [
        list_chirho(con_chirho("Int")),
        TyChirho::TupleChirho(vec![con_chirho("Int"), con_chirho("Bool")]),
        TyChirho::linear_fun_chirho(con_chirho("Int"), con_chirho("Bool")),
        TyChirho::TupleChirho(Vec::new()),
    ] {
        assert_eq!(
            reduce_chirho(&identity_chirho, std::slice::from_ref(&target_chirho)),
            Some(target_chirho)
        );
    }
}

#[test]
fn fixed_arity_rejects_underapplication_and_reapplies_extra_arguments_once_chirho() {
    let definition_chirho = closed_chirho(
        1,
        vec![branch_chirho(
            0,
            vec![con_chirho("Int")],
            con_chirho("Maybe"),
            vec![],
        )],
    );
    assert_eq!(reduce_chirho(&definition_chirho, &[]), None);
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Int"), con_chirho("Bool")]),
        Some(app_chirho(con_chirho("Maybe"), con_chirho("Bool")))
    );
    let nullary_chirho = closed_chirho(
        0,
        vec![branch_chirho(0, vec![], con_chirho("Maybe"), vec![])],
    );
    assert_eq!(
        reduce_chirho(&nullary_chirho, &[]),
        Some(con_chirho("Maybe"))
    );
    assert_eq!(
        reduce_chirho(&nullary_chirho, &[con_chirho("Int")]),
        Some(app_chirho(con_chirho("Maybe"), con_chirho("Int")))
    );
}

#[test]
fn held_closed_branch_blocks_later_fallbacks_but_not_prior_exact_rows_chirho() {
    let mut held_chirho = branch_chirho(1, vec![var_chirho(0)], con_chirho("Char"), vec![0]);
    held_chirho.held_chirho = true;
    let definition_chirho = closed_chirho(
        1,
        vec![
            branch_chirho(0, vec![con_chirho("Bool")], con_chirho("Int"), vec![]),
            held_chirho,
            branch_chirho(2, vec![var_chirho(0)], con_chirho("Char"), vec![0, 1]),
        ],
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Bool")]),
        Some(con_chirho("Int"))
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Int")]),
        None
    );
}

#[test]
fn authoritative_aliases_match_and_same_spelled_nominal_ends_do_not_chirho() {
    let definition_chirho = closed_chirho(
        1,
        vec![
            branch_chirho(
                0,
                vec![con_chirho("AChirho.TChirho")],
                con_chirho("Int"),
                vec![],
            ),
            branch_chirho(1, vec![var_chirho(0)], con_chirho("Bool"), vec![0]),
        ],
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("AliasChirho.TChirho")]),
        Some(con_chirho("Int"))
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("BChirho.TChirho")]),
        Some(con_chirho("Bool"))
    );
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("LostChirho.TChirho")]),
        None
    );
}

#[test]
fn repeated_stuck_family_results_compare_identity_without_reducing_them_chirho() {
    let definition_chirho = closed_chirho(
        2,
        vec![
            branch_chirho(
                0,
                vec![var_chirho(0), var_chirho(0)],
                con_chirho("True"),
                vec![],
            ),
            branch_chirho(
                1,
                vec![var_chirho(1), var_chirho(2)],
                con_chirho("False"),
                vec![0],
            ),
        ],
    );
    assert_eq!(
        reduce_chirho(
            &definition_chirho,
            &[
                app_chirho(con_chirho("GChirho"), con_chirho("Int")),
                app_chirho(con_chirho("AliasGChirho"), con_chirho("Int"))
            ]
        ),
        Some(con_chirho("True"))
    );
    assert_eq!(
        reduce_chirho(
            &definition_chirho,
            &[
                app_chirho(con_chirho("GChirho"), con_chirho("Int")),
                app_chirho(con_chirho("GChirho"), con_chirho("Bool"))
            ]
        ),
        None
    );
}

#[test]
fn unverified_or_conflicting_open_rows_never_reduce_first_wins_chirho() {
    let definition_chirho = TypeFamilyDefinitionChirho::new_chirho(
        family_id_chirho(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![
            branch_chirho(0, vec![var_chirho(0)], con_chirho("Int"), vec![]),
            branch_chirho(1, vec![con_chirho("Int")], con_chirho("Bool"), vec![]),
        ],
    )
    .unwrap();
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Int")]),
        None
    );
    assert!(
        definition_chirho
            .verify_open_coherence_chirho(&compatible_chirho, 10)
            .is_err()
    );
}

#[test]
fn only_certified_coherent_open_rows_can_reduce_chirho() {
    let definition_chirho = TypeFamilyDefinitionChirho::new_chirho(
        family_id_chirho(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![
            branch_chirho(0, vec![var_chirho(0)], var_chirho(0), vec![]),
            branch_chirho(1, vec![con_chirho("Int")], con_chirho("Int"), vec![]),
        ],
    )
    .unwrap();
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Bool")]),
        None
    );
    let certified_chirho = definition_chirho
        .verify_open_coherence_chirho(&compatible_chirho, 10)
        .unwrap();
    assert_eq!(
        reduce_chirho(&certified_chirho, &[con_chirho("Bool")]),
        Some(con_chirho("Bool"))
    );
    assert_eq!(
        reduce_chirho(&certified_chirho, &[con_chirho("Int")]),
        Some(con_chirho("Int"))
    );
    let one_chirho = TypeFamilyDefinitionChirho::new_chirho(
        family_id_chirho(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![branch_chirho(
            0,
            vec![con_chirho("Int")],
            con_chirho("Bool"),
            vec![],
        )],
    )
    .unwrap()
    .verify_open_coherence_chirho(&compatible_chirho, 0)
    .unwrap();
    assert_eq!(
        reduce_chirho(&one_chirho, &[con_chirho("Int")]),
        Some(con_chirho("Bool"))
    );
}

#[test]
fn held_open_rows_and_exhausted_coherence_checks_cannot_certify_chirho() {
    let mut held_chirho = branch_chirho(0, vec![var_chirho(0)], con_chirho("Int"), vec![]);
    held_chirho.held_chirho = true;
    let definition_chirho = TypeFamilyDefinitionChirho::new_chirho(
        family_id_chirho(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![held_chirho],
    )
    .unwrap();
    assert_eq!(
        reduce_chirho(&definition_chirho, &[con_chirho("Int")]),
        None
    );
    assert!(
        definition_chirho
            .verify_open_coherence_chirho(&compatible_chirho, 10)
            .is_err()
    );
    let pairs_chirho = TypeFamilyDefinitionChirho::new_chirho(
        family_id_chirho(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        vec![
            branch_chirho(0, vec![var_chirho(0)], var_chirho(0), vec![]),
            branch_chirho(1, vec![con_chirho("Int")], con_chirho("Int"), vec![]),
        ],
    )
    .unwrap();
    assert!(
        pairs_chirho
            .verify_open_coherence_chirho(&compatible_chirho, 0)
            .is_err()
    );
}

#[test]
fn intrinsics_empty_families_and_unsupported_syntax_remain_stuck_chirho() {
    let intrinsic_chirho = TypeFamilyDefinitionChirho::new_chirho(
        TypeFamilyIdChirho::IntrinsicChirho(IntrinsicTypeFamilyChirho::AddNatChirho),
        2,
        TypeFamilyReductionChirho::IntrinsicChirho(IntrinsicTypeFamilyChirho::AddNatChirho),
        vec![],
    )
    .unwrap();
    assert_eq!(
        reduce_chirho(&intrinsic_chirho, &[con_chirho("Int"), con_chirho("Bool")]),
        None
    );
    assert_eq!(
        reduce_chirho(&closed_chirho(1, vec![]), &[con_chirho("Int")]),
        None
    );
    let identity_chirho = closed_chirho(
        1,
        vec![branch_chirho(0, vec![var_chirho(0)], var_chirho(0), vec![])],
    );
    assert_eq!(
        reduce_chirho(
            &identity_chirho,
            &[TyChirho::TupleChirho(vec![con_chirho("Int")])]
        ),
        None
    );
}

#[test]
fn matching_substitution_and_output_expansion_are_all_bounded_chirho() {
    let identity_chirho = closed_chirho(
        1,
        vec![branch_chirho(0, vec![var_chirho(0)], var_chirho(0), vec![])],
    );
    for limits_chirho in [
        FamilyReductionLimitsChirho {
            steps_chirho: 0,
            nodes_chirho: 100,
            depth_chirho: 100,
        },
        FamilyReductionLimitsChirho {
            steps_chirho: 100,
            nodes_chirho: 0,
            depth_chirho: 100,
        },
        FamilyReductionLimitsChirho {
            steps_chirho: 100,
            nodes_chirho: 100,
            depth_chirho: 0,
        },
    ] {
        assert!(
            reduce_family_with_limits_chirho(
                &identity_chirho,
                &[list_chirho(con_chirho("Int"))],
                &classify_chirho,
                limits_chirho
            )
            .is_none()
        );
    }
    let duplicating_chirho = closed_chirho(
        1,
        vec![branch_chirho(
            0,
            vec![var_chirho(0)],
            TyChirho::TupleChirho(vec![var_chirho(0), var_chirho(0)]),
            vec![],
        )],
    );
    let mut target_chirho = con_chirho("Int");
    for _index_chirho in 0..8 {
        target_chirho = list_chirho(target_chirho);
    }
    assert!(
        reduce_family_with_limits_chirho(
            &duplicating_chirho,
            &[target_chirho],
            &classify_chirho,
            FamilyReductionLimitsChirho {
                steps_chirho: 10_000,
                nodes_chirho: 24,
                depth_chirho: 128
            }
        )
        .is_none()
    );
    assert_eq!(
        reduce_chirho(&identity_chirho, &vec![con_chirho("Int"); 150]),
        None
    );
}

#[test]
fn application_spines_charge_the_physical_depth_of_their_first_argument_chirho() {
    let identity_chirho = closed_chirho(
        1,
        vec![branch_chirho(0, vec![var_chirho(0)], var_chirho(0), vec![])],
    );
    let nested_chirho = list_chirho(list_chirho(list_chirho(con_chirho("Int"))));
    let target_chirho = app_chirho(
        app_chirho(con_chirho("(,)"), nested_chirho),
        con_chirho("Bool"),
    );
    assert!(
        reduce_family_with_limits_chirho(
            &identity_chirho,
            &[target_chirho],
            &classify_chirho,
            FamilyReductionLimitsChirho {
                steps_chirho: 10_000,
                nodes_chirho: 100,
                depth_chirho: 4
            }
        )
        .is_none()
    );
}

#[test]
fn completed_stuck_comparisons_are_distinct_from_unknown_work_chirho() {
    use FamilyReductionDecisionChirho as DChirho;
    let definition_chirho = closed_chirho(
        1,
        vec![
            branch_chirho(0, vec![con_chirho("Int")], con_chirho("Bool"), vec![]),
            branch_chirho(
                1,
                vec![named_chirho("aChirho")],
                con_chirho("Char"),
                vec![0],
            ),
        ],
    );
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[con_chirho("Int")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        DChirho::ReducedChirho(con_chirho("Bool"))
    );
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[named_chirho("rigidChirho")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        DChirho::BlockedChirho
    );
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[con_chirho("UnknownChirho")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        DChirho::UnknownChirho
    );
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[con_chirho("Int")],
            &classify_chirho,
            FamilyReductionLimitsChirho {
                steps_chirho: 0,
                ..FamilyReductionLimitsChirho::default()
            }
        ),
        DChirho::UnknownChirho
    );
    let empty_chirho = closed_chirho(1, Vec::new());
    assert_eq!(
        decide_family_with_limits_chirho(
            &empty_chirho,
            &[con_chirho("Int")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        DChirho::NoMatchChirho
    );
}

#[test]
fn held_and_unverified_definitions_never_certify_a_stuck_result_chirho() {
    use FamilyReductionDecisionChirho as DChirho;
    let mut held_chirho =
        branch_chirho(0, vec![named_chirho("aChirho")], con_chirho("Int"), vec![]);
    held_chirho.held_chirho = true;
    let closed_chirho = closed_chirho(1, vec![held_chirho]);
    assert!(!closed_chirho.reduction_metadata_complete_chirho());
    assert_eq!(
        decide_family_with_limits_chirho(
            &closed_chirho,
            &[con_chirho("Bool")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        DChirho::UnknownChirho
    );
    let open_chirho = TypeFamilyDefinitionChirho::new_chirho(
        family_id_chirho(),
        1,
        TypeFamilyReductionChirho::OpenChirho,
        Vec::new(),
    )
    .unwrap();
    assert!(!open_chirho.reduction_metadata_complete_chirho());
    assert_eq!(
        decide_family_with_limits_chirho(
            &open_chirho,
            &[con_chirho("Bool")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        DChirho::UnknownChirho
    );
    let verified_chirho = open_chirho
        .verify_open_coherence_chirho(&|_left_chirho, _right_chirho| true, 0)
        .unwrap();
    assert!(verified_chirho.reduction_metadata_complete_chirho());
    assert_eq!(
        decide_family_with_limits_chirho(
            &verified_chirho,
            &[con_chirho("Bool")],
            &classify_chirho,
            FamilyReductionLimitsChirho::default()
        ),
        DChirho::NoMatchChirho
    );
}

#[test]
fn equation_variables_carry_checked_polytypes_without_decomposing_them_chirho() {
    // The measured ExplicitFamilyLengthChirho accepts an explicit polymorphic
    // family argument. Its bound variable is payload, never an equation meta.
    let forall_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        body_chirho: Box::new(var_chirho(0)),
    };
    let identity_chirho = closed_chirho(
        1,
        vec![branch_chirho(0, vec![var_chirho(0)], var_chirho(0), vec![])],
    );
    assert_eq!(
        reduce_chirho(&identity_chirho, std::slice::from_ref(&forall_chirho)),
        Some(forall_chirho.clone())
    );
    let repeated_chirho = closed_chirho(
        2,
        vec![branch_chirho(
            0,
            vec![var_chirho(0), var_chirho(0)],
            var_chirho(0),
            vec![],
        )],
    );
    assert_eq!(
        reduce_chirho(
            &repeated_chirho,
            &[forall_chirho.clone(), forall_chirho.clone()]
        ),
        Some(forall_chirho.clone())
    );
    let concrete_chirho = closed_chirho(
        1,
        vec![branch_chirho(
            0,
            vec![con_chirho("Int")],
            con_chirho("Bool"),
            vec![],
        )],
    );
    assert_eq!(
        reduce_chirho(&concrete_chirho, std::slice::from_ref(&forall_chirho)),
        None
    );
    let forbidden_pattern_chirho = closed_chirho(
        1,
        vec![branch_chirho(
            0,
            vec![forall_chirho.clone()],
            con_chirho("Bool"),
            vec![],
        )],
    );
    assert_eq!(
        reduce_chirho(
            &forbidden_pattern_chirho,
            std::slice::from_ref(&forall_chirho)
        ),
        None
    );
    assert_eq!(
        reduce_family_with_limits_chirho(
            &identity_chirho,
            &[forall_chirho],
            &classify_chirho,
            FamilyReductionLimitsChirho {
                steps_chirho: 5,
                nodes_chirho: 100,
                depth_chirho: 128
            }
        ),
        None
    );
}
