// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Normal semantic controls for row 686b. The occurs and repeated-family cases are
//! GHC 9.14.1 `GHC.Core.Unify`'s infinitary-substitution and family-sharing examples;
//! the compatible overlap is the measured B02 `AndChirho` shape.

use super::closed_family_apartness_chirho::{
    FamilyHeadRoleChirho, FamilyPatternRelationChirho, FamilyUnificationChirho,
    FamilyUnificationLimitsChirho, GenerativeIdentityChirho, family_branches_compatible_chirho,
    family_patterns_relation_with_budget_chirho, family_patterns_relation_with_limits_chirho,
    family_patterns_unification_chirho, family_patterns_with_limits_chirho,
};

#[test]
fn repeated_apartness_checks_share_work_and_node_debt_chirho() {
    let mut budget_chirho = FamilyUnificationLimitsChirho {
        steps_chirho: 1024,
        nodes_chirho: 256,
        depth_chirho: 128,
    };
    let mut proved_chirho = 0;
    for _index_chirho in 0..128 {
        if family_patterns_relation_with_budget_chirho(
            &[constructor_chirho("AChirho.TChirho")],
            &[constructor_chirho("BChirho.TChirho")],
            &classify_chirho,
            &mut budget_chirho,
        ) != FamilyPatternRelationChirho::SurelyApartChirho
        {
            break;
        }
        proved_chirho += 1;
    }
    assert!(proved_chirho > 0 && proved_chirho < 128);
    assert!(budget_chirho.steps_chirho < 1024);
    assert!(budget_chirho.nodes_chirho < 256);
}

#[test]
fn an_unknown_apartness_result_keeps_its_used_allowance_chirho() {
    let mut budget_chirho = FamilyUnificationLimitsChirho::default();
    let before_chirho = budget_chirho;
    assert_eq!(
        family_patterns_relation_with_budget_chirho(
            &[constructor_chirho("MissingChirho")],
            &[constructor_chirho("BChirho.TChirho")],
            &classify_chirho,
            &mut budget_chirho,
        ),
        FamilyPatternRelationChirho::UnknownChirho
    );
    assert!(budget_chirho.steps_chirho < before_chirho.steps_chirho);
    assert!(budget_chirho.nodes_chirho < before_chirho.nodes_chirho);
}
use crate::ty_chirho::{TyChirho, TyVarChirho};

fn constructor_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_owned())
}

fn variable_chirho(index_chirho: u32) -> TyChirho {
    TyChirho::VarChirho(TyVarChirho(index_chirho))
}

fn named_variable_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ForallVarChirho(name_chirho.to_owned())
}

fn application_chirho(function_chirho: TyChirho, argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(Box::new(function_chirho), Box::new(argument_chirho))
}

fn family_chirho(name_chirho: &str, argument_chirho: TyChirho) -> TyChirho {
    application_chirho(constructor_chirho(name_chirho), argument_chirho)
}

fn list_chirho(element_chirho: TyChirho) -> TyChirho {
    TyChirho::ListChirho(Box::new(element_chirho))
}

fn forall_chirho(index_chirho: u32, body_chirho: TyChirho) -> TyChirho {
    TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(index_chirho)].into(),
        body_chirho: Box::new(body_chirho),
    }
}

/// A closed fixture identity table, not a guess that every unrecognized name is generative.
fn classify_chirho(name_chirho: &str) -> FamilyHeadRoleChirho {
    match name_chirho {
        "GChirho" | "AliasGChirho" => FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: "GChirho".to_owned(),
            arity_chirho: 1,
        },
        "HChirho" => FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: "HChirho".to_owned(),
            arity_chirho: 1,
        },
        "OpaqueChirho" => FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: "OpaqueChirho".to_owned(),
            arity_chirho: 1,
        },
        "NullChirho" => FamilyHeadRoleChirho::FamilyChirho {
            identity_chirho: "NullChirho".to_owned(),
            arity_chirho: 0,
        },
        "AChirho.TChirho" | "AliasChirho.TChirho" => FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(
                "source AChirho TChirho".to_owned(),
            ),
        },
        "BChirho.TChirho" => FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(
                "source BChirho TChirho".to_owned(),
            ),
        },
        "ListAliasChirho" => FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::ListChirho,
        },
        "PretendArrowChirho" => FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(
                "(linear arrow chirho)".to_owned(),
            ),
        },
        "Int" | "Bool" | "Char" | "Double" | "Maybe" | "True" | "False" | "TChirho"
        | "%promoted:TChirho" | "'[]" | ":" => FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(name_chirho.to_owned()),
        },
        _ => FamilyHeadRoleChirho::UnknownChirho,
    }
}

fn unify_chirho(left_chirho: &[TyChirho], right_chirho: &[TyChirho]) -> FamilyUnificationChirho {
    family_patterns_unification_chirho(left_chirho, right_chirho, &classify_chirho)
}

fn compatible_chirho(
    left_lhs_chirho: &[TyChirho],
    left_rhs_chirho: &TyChirho,
    right_lhs_chirho: &[TyChirho],
    right_rhs_chirho: &TyChirho,
) -> bool {
    family_branches_compatible_chirho(
        left_lhs_chirho,
        left_rhs_chirho,
        right_lhs_chirho,
        right_rhs_chirho,
        &classify_chirho,
    )
}

#[test]
fn authoritative_nominal_identity_preserves_aliases_and_namesakes_chirho() {
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("AChirho.TChirho")],
            &[constructor_chirho("AliasChirho.TChirho")]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("AChirho.TChirho")],
            &[constructor_chirho("BChirho.TChirho")]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("Int")],
            &[constructor_chirho("Double")]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn unknown_heads_never_supply_a_nominal_apartness_proof_chirho() {
    for right_chirho in ["UnknownChirho", "OtherChirho", "Int"] {
        assert_eq!(
            unify_chirho(
                &[constructor_chirho("UnknownChirho")],
                &[constructor_chirho(right_chirho)]
            ),
            FamilyUnificationChirho::UncertainChirho
        );
    }
    assert_eq!(
        unify_chirho(
            &[variable_chirho(0)],
            &[constructor_chirho("UnknownChirho")]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn unknown_heads_do_not_hide_an_independent_known_clash_chirho() {
    assert_eq!(
        unify_chirho(
            &[
                constructor_chirho("UnknownChirho"),
                constructor_chirho("Int")
            ],
            &[
                constructor_chirho("OtherChirho"),
                constructor_chirho("Bool")
            ]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn unknown_applications_are_not_assumed_injective_chirho() {
    for head_chirho in ["UnknownChirho", "OtherChirho"] {
        assert_eq!(
            unify_chirho(
                &[application_chirho(
                    constructor_chirho("UnknownChirho"),
                    constructor_chirho("Int")
                )],
                &[application_chirho(
                    constructor_chirho(head_chirho),
                    constructor_chirho("Bool")
                )]
            ),
            FamilyUnificationChirho::UncertainChirho
        );
    }
}

#[test]
fn substitutions_cannot_turn_unknown_applications_into_injective_heads_chirho() {
    let left_chirho = [
        variable_chirho(0),
        application_chirho(variable_chirho(0), constructor_chirho("Int")),
    ];
    let right_chirho = [
        constructor_chirho("UnknownChirho"),
        application_chirho(variable_chirho(1), constructor_chirho("Bool")),
    ];
    assert_eq!(
        unify_chirho(&left_chirho, &right_chirho),
        FamilyUnificationChirho::UncertainChirho
    );
    // Queue the application before the substitution too. A merely eager head check
    // would have already scheduled Int versus Bool when the Unknown binding arrives.
    assert_eq!(
        unify_chirho(
            &[left_chirho[1].clone(), left_chirho[0].clone()],
            &[right_chirho[1].clone(), right_chirho[0].clone()]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn variable_application_heads_need_positive_injectivity_evidence_chirho() {
    assert_eq!(
        unify_chirho(
            &[application_chirho(
                variable_chirho(0),
                constructor_chirho("Int")
            )],
            &[application_chirho(
                variable_chirho(0),
                constructor_chirho("Bool")
            )]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn list_tuple_and_function_sugar_share_their_generative_views_chirho() {
    let list_application_chirho =
        application_chirho(constructor_chirho("[]"), constructor_chirho("Int"));
    assert_eq!(
        unify_chirho(
            &[list_chirho(constructor_chirho("Int"))],
            &[list_application_chirho]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
    let tuple_application_chirho = application_chirho(
        application_chirho(constructor_chirho("(,)"), constructor_chirho("Int")),
        constructor_chirho("Bool"),
    );
    assert_eq!(
        unify_chirho(
            &[TyChirho::TupleChirho(vec![
                constructor_chirho("Int"),
                constructor_chirho("Bool")
            ])],
            &[tuple_application_chirho]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
    let function_application_chirho = application_chirho(
        application_chirho(constructor_chirho("->"), constructor_chirho("Int")),
        constructor_chirho("Bool"),
    );
    assert_eq!(
        unify_chirho(
            &[TyChirho::fun_chirho(
                constructor_chirho("Int"),
                constructor_chirho("Bool")
            )],
            &[function_application_chirho]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
}

#[test]
fn a_higher_kinded_pattern_matches_list_sugar_without_false_apartness_chirho() {
    assert_eq!(
        unify_chirho(
            &[application_chirho(variable_chirho(0), variable_chirho(1))],
            &[list_chirho(constructor_chirho("Int"))]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
    assert_eq!(
        unify_chirho(
            &[list_chirho(variable_chirho(0))],
            &[application_chirho(
                constructor_chirho("Maybe"),
                variable_chirho(0)
            )]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn unit_and_singleton_tuple_cannot_collapse_to_one_constructor_chirho() {
    assert_eq!(
        unify_chirho(
            &[TyChirho::TupleChirho(Vec::new())],
            &[constructor_chirho("()")]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
    assert_eq!(
        unify_chirho(
            &[TyChirho::TupleChirho(vec![constructor_chirho("Int")])],
            &[constructor_chirho("()")]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn arrow_multiplicity_is_part_of_the_generative_identity_chirho() {
    assert_eq!(
        unify_chirho(
            &[TyChirho::linear_fun_chirho(
                constructor_chirho("Int"),
                constructor_chirho("Bool")
            )],
            &[TyChirho::fun_chirho(
                constructor_chirho("Int"),
                constructor_chirho("Bool")
            )]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn syntax_tags_cannot_collide_with_opaque_nominal_identity_strings_chirho() {
    let arrow_chirho =
        TyChirho::linear_fun_chirho(constructor_chirho("Int"), constructor_chirho("Bool"));
    let nominal_chirho = application_chirho(
        application_chirho(
            constructor_chirho("PretendArrowChirho"),
            constructor_chirho("Int"),
        ),
        constructor_chirho("Bool"),
    );
    assert_eq!(
        unify_chirho(&[arrow_chirho], &[nominal_chirho]),
        FamilyUnificationChirho::SurelyApartChirho
    );
    assert_eq!(
        unify_chirho(
            &[list_chirho(constructor_chirho("Int"))],
            &[application_chirho(
                constructor_chirho("ListAliasChirho"),
                constructor_chirho("Int")
            )]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
}

#[test]
fn promoted_and_ordinary_constructor_identities_stay_distinct_chirho() {
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("TChirho")],
            &[constructor_chirho("%promoted:TChirho")]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
    assert_eq!(
        unify_chirho(&[constructor_chirho("'[]")], &[constructor_chirho("[]")]),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn source_and_target_variable_numbers_are_separate_namespaces_chirho() {
    assert_eq!(
        unify_chirho(&[variable_chirho(0)], &[list_chirho(variable_chirho(0))]),
        FamilyUnificationChirho::UnifiableChirho
    );
    assert_eq!(
        unify_chirho(
            &[named_variable_chirho("aChirho")],
            &[list_chirho(named_variable_chirho("aChirho"))]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
}

#[test]
fn numeric_and_named_variables_never_collide_by_rendered_name_chirho() {
    assert_eq!(
        unify_chirho(
            &[variable_chirho(0), named_variable_chirho("tv0")],
            &[constructor_chirho("Int"), constructor_chirho("Bool")]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
}

#[test]
fn repeated_variables_retain_their_sharing_chirho() {
    assert_eq!(
        unify_chirho(
            &[variable_chirho(0), variable_chirho(0)],
            &[constructor_chirho("Int"), constructor_chirho("Bool")]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn occurs_failure_is_uncertainty_and_never_proves_compatibility_chirho() {
    let left_chirho = [variable_chirho(0), variable_chirho(0)];
    let right_chirho = [list_chirho(variable_chirho(0)), variable_chirho(0)];
    assert_eq!(
        unify_chirho(&left_chirho, &right_chirho),
        FamilyUnificationChirho::UncertainChirho
    );
    assert!(!compatible_chirho(
        &left_chirho,
        &constructor_chirho("Int"),
        &right_chirho,
        &constructor_chirho("Bool")
    ));
}

#[test]
fn a_later_clash_survives_an_earlier_occurs_uncertainty_chirho() {
    assert_eq!(
        unify_chirho(
            &[
                variable_chirho(0),
                variable_chirho(0),
                constructor_chirho("Int")
            ],
            &[
                list_chirho(variable_chirho(0)),
                variable_chirho(0),
                constructor_chirho("Bool")
            ]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn lhs_and_rhs_are_freshened_together_but_branches_are_separate_chirho() {
    // F a b = a versus F b a = a is first-projection versus second-projection.
    assert!(!compatible_chirho(
        &[variable_chirho(0), variable_chirho(1)],
        &variable_chirho(0),
        &[variable_chirho(1), variable_chirho(0)],
        &variable_chirho(0)
    ));
}

#[test]
fn compatibility_chases_transitive_substitutions_on_both_rhs_chirho() {
    assert!(compatible_chirho(
        &[variable_chirho(0), variable_chirho(0)],
        &variable_chirho(0),
        &[variable_chirho(1), constructor_chirho("Int")],
        &variable_chirho(1)
    ));
    assert!(!compatible_chirho(
        &[variable_chirho(0)],
        &variable_chirho(0),
        &[constructor_chirho("Int")],
        &constructor_chirho("Bool")
    ));
}

#[test]
fn measured_and_overlap_has_equal_rhs_after_the_finite_unifier_chirho() {
    assert!(compatible_chirho(
        &[constructor_chirho("True"), named_variable_chirho("bChirho")],
        &named_variable_chirho("bChirho"),
        &[named_variable_chirho("aChirho"), constructor_chirho("True")],
        &named_variable_chirho("aChirho")
    ));
}

#[test]
fn disjoint_branches_are_compatible_without_comparing_rhs_chirho() {
    assert!(compatible_chirho(
        &[constructor_chirho("Int")],
        &constructor_chirho("UnknownChirho"),
        &[constructor_chirho("Bool")],
        &constructor_chirho("OtherChirho")
    ));
}

#[test]
fn a_stuck_family_cannot_be_treated_as_a_rigid_head_chirho() {
    assert_eq!(
        unify_chirho(
            &[family_chirho("GChirho", constructor_chirho("Int"))],
            &[constructor_chirho("Bool")]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
    assert!(!compatible_chirho(
        &[family_chirho("GChirho", constructor_chirho("Int"))],
        &constructor_chirho("Int"),
        &[constructor_chirho("Bool")],
        &constructor_chirho("Bool")
    ));
}

#[test]
fn identical_ground_family_applications_are_already_equal_chirho() {
    assert_eq!(
        unify_chirho(
            &[family_chirho("GChirho", constructor_chirho("Int"))],
            &[family_chirho("AliasGChirho", constructor_chirho("Int"))]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
}

#[test]
fn one_shared_family_result_cannot_be_both_int_and_bool_chirho() {
    let target_chirho = family_chirho("GChirho", variable_chirho(0));
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("Int"), constructor_chirho("Bool")],
            &[target_chirho.clone(), target_chirho]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn different_family_arguments_and_heads_do_not_share_flatten_variables_chirho() {
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("Int"), constructor_chirho("Bool")],
            &[
                family_chirho("GChirho", constructor_chirho("Int")),
                family_chirho("GChirho", constructor_chirho("Bool"))
            ]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("Int"), constructor_chirho("Bool")],
            &[
                family_chirho("GChirho", constructor_chirho("Int")),
                family_chirho("HChirho", constructor_chirho("Int"))
            ]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn family_sharing_tracks_argument_substitutions_chirho() {
    assert_eq!(
        unify_chirho(
            &[
                family_chirho("GChirho", variable_chirho(0)),
                family_chirho("GChirho", variable_chirho(1)),
                variable_chirho(0),
                variable_chirho(1)
            ],
            &[
                constructor_chirho("Int"),
                constructor_chirho("Bool"),
                variable_chirho(0),
                variable_chirho(0)
            ]
        ),
        FamilyUnificationChirho::SurelyApartChirho
    );
}

#[test]
fn rhs_family_applications_compare_their_substituted_identity_not_flatten_ids_chirho() {
    assert!(compatible_chirho(
        &[constructor_chirho("Int")],
        &family_chirho("GChirho", constructor_chirho("Int")),
        &[variable_chirho(0)],
        &family_chirho("GChirho", variable_chirho(0))
    ));
    assert!(!compatible_chirho(
        &[constructor_chirho("Int")],
        &family_chirho("GChirho", constructor_chirho("Int")),
        &[variable_chirho(0)],
        &family_chirho("HChirho", variable_chirho(0))
    ));
}

#[test]
fn forall_variables_are_alpha_renamed_without_capture_chirho() {
    assert_eq!(
        unify_chirho(
            &[forall_chirho(
                0,
                TyChirho::fun_chirho(variable_chirho(0), variable_chirho(0))
            )],
            &[forall_chirho(
                42,
                TyChirho::fun_chirho(variable_chirho(42), variable_chirho(42))
            )]
        ),
        FamilyUnificationChirho::UnifiableChirho
    );
    assert_eq!(
        unify_chirho(
            &[forall_chirho(0, variable_chirho(1))],
            &[forall_chirho(42, variable_chirho(42))]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn malformed_forall_binders_are_unsupported_instead_of_aliasing_chirho() {
    let invalid_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0), TyVarChirho(0)].into(),
        body_chirho: Box::new(variable_chirho(0)),
    };
    assert_eq!(
        unify_chirho(&[invalid_chirho], &[constructor_chirho("Int")]),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn family_flattening_does_not_identify_different_forall_scopes_chirho() {
    let left_chirho = TyChirho::TupleChirho(vec![
        forall_chirho(0, family_chirho("GChirho", variable_chirho(0))),
        forall_chirho(0, family_chirho("GChirho", variable_chirho(0))),
    ]);
    let right_chirho = TyChirho::TupleChirho(vec![
        forall_chirho(1, constructor_chirho("Int")),
        forall_chirho(1, constructor_chirho("Bool")),
    ]);
    assert_eq!(
        unify_chirho(&[left_chirho], &[right_chirho]),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn alpha_equivalent_forall_rhs_keep_structured_family_identity_chirho() {
    assert!(compatible_chirho(
        &[constructor_chirho("Int")],
        &forall_chirho(0, family_chirho("GChirho", variable_chirho(0))),
        &[variable_chirho(1)],
        &forall_chirho(42, family_chirho("GChirho", variable_chirho(42)))
    ));
}

#[test]
fn undersaturation_and_vector_arity_mismatch_are_not_apartness_proofs_chirho() {
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("GChirho")],
            &[constructor_chirho("Int")]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
    assert_eq!(
        unify_chirho(&[constructor_chirho("Int")], &[]),
        FamilyUnificationChirho::UncertainChirho
    );
    assert_eq!(
        unify_chirho(
            &[constructor_chirho("NullChirho")],
            &[constructor_chirho("Int")]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
}

#[test]
fn exhausting_each_work_budget_is_conservative_chirho() {
    let left_chirho = [list_chirho(constructor_chirho("Int"))];
    for limits_chirho in [
        FamilyUnificationLimitsChirho {
            steps_chirho: 0,
            nodes_chirho: 100,
            depth_chirho: 100,
        },
        FamilyUnificationLimitsChirho {
            steps_chirho: 100,
            nodes_chirho: 0,
            depth_chirho: 100,
        },
        FamilyUnificationLimitsChirho {
            steps_chirho: 100,
            nodes_chirho: 100,
            depth_chirho: 0,
        },
    ] {
        assert_eq!(
            family_patterns_with_limits_chirho(
                &left_chirho,
                &left_chirho,
                &classify_chirho,
                limits_chirho
            ),
            FamilyUnificationChirho::UncertainChirho
        );
    }
}

#[test]
fn measured_c08_has_completed_known_family_overlap_but_c07_is_apart_chirho() {
    let earlier_chirho = [constructor_chirho("Int"), constructor_chirho("Bool")];
    let first_chirho = family_chirho("OpaqueChirho", named_variable_chirho("aChirho%1"));
    let second_chirho = family_chirho("OpaqueChirho", named_variable_chirho("bChirho%2"));
    assert_eq!(
        family_patterns_relation_with_limits_chirho(
            &earlier_chirho,
            &[first_chirho.clone(), second_chirho.clone()],
            &classify_chirho,
            FamilyUnificationLimitsChirho::default()
        ),
        FamilyPatternRelationChirho::KnownFamilyOverlapChirho
    );
    assert_eq!(
        family_patterns_relation_with_limits_chirho(
            &earlier_chirho,
            &[first_chirho.clone(), first_chirho.clone()],
            &classify_chirho,
            FamilyUnificationLimitsChirho::default()
        ),
        FamilyPatternRelationChirho::SurelyApartChirho
    );
    // The old relation and branch compatibility retain their conservative contract.
    assert_eq!(
        unify_chirho(
            &earlier_chirho,
            &[first_chirho.clone(), second_chirho.clone()]
        ),
        FamilyUnificationChirho::UncertainChirho
    );
    assert!(!compatible_chirho(
        &earlier_chirho,
        &constructor_chirho("Int"),
        &[first_chirho, second_chirho],
        &constructor_chirho("Int")
    ));
}

#[test]
fn known_family_overlap_does_not_hide_other_uncertainty_chirho() {
    let family_chirho = family_chirho("OpaqueChirho", named_variable_chirho("aChirho%1"));
    assert_eq!(
        family_patterns_relation_with_limits_chirho(
            &[
                constructor_chirho("Int"),
                constructor_chirho("UnknownChirho")
            ],
            &[family_chirho.clone(), constructor_chirho("Bool")],
            &classify_chirho,
            FamilyUnificationLimitsChirho::default()
        ),
        FamilyPatternRelationChirho::UnknownChirho
    );
    assert_eq!(
        family_patterns_relation_with_limits_chirho(
            &[
                variable_chirho(0),
                variable_chirho(0),
                constructor_chirho("Int")
            ],
            &[
                list_chirho(variable_chirho(0)),
                variable_chirho(0),
                family_chirho.clone()
            ],
            &classify_chirho,
            FamilyUnificationLimitsChirho::default()
        ),
        FamilyPatternRelationChirho::UnknownChirho
    );
    assert_eq!(
        family_patterns_relation_with_limits_chirho(
            &[constructor_chirho("Int")],
            &[family_chirho],
            &classify_chirho,
            FamilyUnificationLimitsChirho {
                steps_chirho: 0,
                ..FamilyUnificationLimitsChirho::default()
            }
        ),
        FamilyPatternRelationChirho::UnknownChirho
    );
}
