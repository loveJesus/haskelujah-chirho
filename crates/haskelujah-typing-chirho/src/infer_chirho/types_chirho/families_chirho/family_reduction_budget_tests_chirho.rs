// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The real matcher shares one continuation allowance across completed and
//! refused attempts; old value-taking callers keep their original contract.

use super::closed_family_apartness_chirho::{FamilyHeadRoleChirho, GenerativeIdentityChirho};
use super::family_definitions_chirho::{
    TypeFamilyBranchChirho, TypeFamilyBranchIdChirho, TypeFamilyDefinitionChirho,
    TypeFamilyIdChirho, TypeFamilyReductionChirho,
};
use super::family_reduction_chirho::{
    FamilyReductionDecisionChirho, FamilyReductionLimitsChirho, MatchFailureChirho,
    decide_family_with_budget_chirho, decide_family_with_limits_chirho,
};
use crate::ty_chirho::{TyChirho, TyVarChirho};
use haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitChirho as MatchLimitChirho;

fn con_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_owned())
}
fn classify_chirho(name_chirho: &str) -> FamilyHeadRoleChirho {
    if name_chirho == "UnknownChirho" {
        FamilyHeadRoleChirho::UnknownChirho
    } else {
        FamilyHeadRoleChirho::GenerativeChirho {
            identity_chirho: GenerativeIdentityChirho::NominalChirho(name_chirho.to_owned()),
        }
    }
}
fn definition_chirho(lhs_chirho: TyChirho, rhs_chirho: TyChirho) -> TypeFamilyDefinitionChirho {
    let family_chirho = TypeFamilyIdChirho::SourceChirho {
        module_chirho: "BudgetChirho".into(),
        name_chirho: "FChirho".into(),
    };
    TypeFamilyDefinitionChirho::new_chirho(
        family_chirho.clone(),
        1,
        TypeFamilyReductionChirho::ClosedChirho,
        vec![TypeFamilyBranchChirho {
            id_chirho: TypeFamilyBranchIdChirho {
                family_chirho,
                producer_module_chirho: "BudgetChirho".into(),
                declaration_start_chirho: 0,
                equation_index_chirho: 0,
            },
            binders_chirho: vec![],
            lhs_chirho: vec![lhs_chirho],
            rhs_chirho,
            blockers_chirho: vec![],
            held_chirho: false,
        }],
    )
    .expect("well-shaped component definition")
}
fn limits_chirho() -> FamilyReductionLimitsChirho {
    FamilyReductionLimitsChirho {
        steps_chirho: 4096,
        nodes_chirho: 4096,
        depth_chirho: 128,
    }
}

#[test]
fn completed_nonmatches_consume_one_shared_allowance_chirho() {
    let definition_chirho = definition_chirho(con_chirho("Int"), con_chirho("Bool"));
    let mut limits_chirho = limits_chirho();
    let original_chirho = limits_chirho;
    let mut attempts_chirho = 0;
    loop {
        let before_chirho = limits_chirho;
        let result_chirho = decide_family_with_budget_chirho(
            &definition_chirho,
            &[con_chirho("Char")],
            &classify_chirho,
            &mut limits_chirho,
        );
        assert!(limits_chirho.steps_chirho <= before_chirho.steps_chirho);
        assert!(limits_chirho.nodes_chirho <= before_chirho.nodes_chirho);
        match result_chirho {
            Ok(FamilyReductionDecisionChirho::NoMatchChirho) => {
                assert!(limits_chirho.steps_chirho < before_chirho.steps_chirho);
                attempts_chirho += 1;
                assert!(attempts_chirho < original_chirho.steps_chirho);
            }
            Err(MatchFailureChirho::LimitChirho(
                MatchLimitChirho::WorkChirho | MatchLimitChirho::OutputChirho,
            )) => break,
            other_chirho => panic!("unexpected bounded decision: {other_chirho:?}"),
        }
    }
    assert!(attempts_chirho > 1);
    assert!(limits_chirho.steps_chirho < original_chirho.steps_chirho);
    assert!(limits_chirho.nodes_chirho < original_chirho.nodes_chirho);
}

#[test]
fn unsupported_comparison_retains_consumed_target_work_chirho() {
    let definition_chirho = definition_chirho(con_chirho("UnknownChirho"), con_chirho("Bool"));
    let mut limits_chirho = limits_chirho();
    let initial_chirho = limits_chirho;
    assert_eq!(
        decide_family_with_budget_chirho(
            &definition_chirho,
            &[con_chirho("Int")],
            &classify_chirho,
            &mut limits_chirho,
        ),
        Ok(FamilyReductionDecisionChirho::UnknownChirho)
    );
    assert!(limits_chirho.steps_chirho < initial_chirho.steps_chirho);
    assert!(limits_chirho.nodes_chirho < initial_chirho.nodes_chirho);
}

#[test]
fn successful_materialization_retains_both_consumptions_chirho() {
    let definition_chirho =
        definition_chirho(TyChirho::VarChirho(TyVarChirho(0)), con_chirho("Bool"));
    let mut limits_chirho = limits_chirho();
    let initial_chirho = limits_chirho;
    assert_eq!(
        decide_family_with_budget_chirho(
            &definition_chirho,
            &[con_chirho("Int")],
            &classify_chirho,
            &mut limits_chirho,
        ),
        Ok(FamilyReductionDecisionChirho::ReducedChirho(con_chirho(
            "Bool"
        )))
    );
    assert!(limits_chirho.steps_chirho < initial_chirho.steps_chirho);
    assert!(limits_chirho.nodes_chirho < initial_chirho.nodes_chirho);
}

#[test]
fn returned_name_is_bounded_before_its_clone_chirho() {
    let name_chirho = "LongNameChirho".repeat(16);
    let definition_chirho = definition_chirho(
        TyChirho::VarChirho(TyVarChirho(0)),
        con_chirho(&name_chirho),
    );
    let initial_chirho = FamilyReductionLimitsChirho {
        steps_chirho: 100_000,
        nodes_chirho: 32,
        depth_chirho: 128,
    };
    let mut limits_chirho = initial_chirho;
    assert_eq!(
        decide_family_with_budget_chirho(
            &definition_chirho,
            &[con_chirho("Int")],
            &classify_chirho,
            &mut limits_chirho,
        ),
        Err(MatchFailureChirho::LimitChirho(
            MatchLimitChirho::OutputChirho
        ))
    );
    assert!(limits_chirho.steps_chirho < initial_chirho.steps_chirho);
    assert!(limits_chirho.nodes_chirho < initial_chirho.nodes_chirho);
    // The compatibility API's existing node-only limit is deliberately distinct.
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[con_chirho("Int")],
            &classify_chirho,
            initial_chirho,
        ),
        FamilyReductionDecisionChirho::ReducedChirho(con_chirho(&name_chirho))
    );
}

#[test]
fn physical_depth_failure_is_typed_and_does_not_refund_chirho() {
    let definition_chirho =
        definition_chirho(TyChirho::VarChirho(TyVarChirho(0)), con_chirho("Bool"));
    let argument_chirho = TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
        TyChirho::ListChirho(Box::new(con_chirho("Int"))),
    ))));
    let initial_chirho = FamilyReductionLimitsChirho {
        depth_chirho: 1,
        ..limits_chirho()
    };
    let mut limits_chirho = initial_chirho;
    assert_eq!(
        decide_family_with_budget_chirho(
            &definition_chirho,
            std::slice::from_ref(&argument_chirho),
            &classify_chirho,
            &mut limits_chirho,
        ),
        Err(MatchFailureChirho::LimitChirho(
            MatchLimitChirho::DepthChirho
        ))
    );
    assert!(limits_chirho.steps_chirho < initial_chirho.steps_chirho);
    assert_eq!(
        decide_family_with_limits_chirho(
            &definition_chirho,
            &[argument_chirho],
            &classify_chirho,
            initial_chirho,
        ),
        FamilyReductionDecisionChirho::UnknownChirho
    );
}

#[test]
fn ordinary_siblings_share_consumed_work_without_fixed_reservations_chirho() {
    let mut context_chirho = super::test_support_chirho::family_fixture_context_chirho();
    context_chirho.register_type_family_chirho(
        "StepChirho".into(),
        1,
        vec![(vec![TyChirho::int_chirho()], TyChirho::bool_chirho())],
    );
    let input_chirho = TyChirho::TupleChirho(vec![
        super::test_support_chirho::type_family_app1_chirho(
            "StepChirho",
            TyChirho::int_chirho(),
        );
        128
    ]);
    assert_eq!(
        context_chirho.reduce_type_families_in_ty_chirho(&input_chirho),
        TyChirho::TupleChirho(vec![TyChirho::bool_chirho(); 128])
    );
}

#[test]
fn normalizer_stuck_attempts_settle_against_the_same_outer_allowance_chirho() {
    let mut context_chirho = super::test_support_chirho::family_fixture_context_chirho();
    context_chirho.register_type_family_chirho(
        "BlockedChirho".into(),
        1,
        vec![(vec![TyChirho::int_chirho()], TyChirho::bool_chirho())],
    );
    let input_chirho = super::test_support_chirho::type_family_app1_chirho(
        "BlockedChirho",
        TyChirho::char_chirho(),
    );
    let mut work_chirho = 4096;
    let mut output_chirho = 4096;
    let mut completed_chirho = 0;
    loop {
        let before_chirho = (work_chirho, output_chirho);
        let result_chirho = context_chirho.reduce_families_bounded_chirho(
            &input_chirho,
            0,
            &mut work_chirho,
            &mut output_chirho,
        );
        assert!(work_chirho <= before_chirho.0);
        assert!(output_chirho <= before_chirho.1);
        if let Some(result_chirho) = result_chirho {
            assert_eq!(result_chirho, input_chirho);
            assert!(work_chirho < before_chirho.0);
            assert!(output_chirho < before_chirho.1);
            completed_chirho += 1;
            assert!(completed_chirho < 4096);
        } else {
            break;
        }
    }
    assert!(completed_chirho > 1);
    assert!(work_chirho < 4096);
    assert!(output_chirho < 4096);
}

#[test]
fn normalizer_output_and_depth_refusals_keep_partial_consumption_chirho() {
    let mut context_chirho = super::test_support_chirho::family_fixture_context_chirho();
    context_chirho.register_type_family_chirho(
        "StepChirho".into(),
        1,
        vec![(vec![TyChirho::int_chirho()], TyChirho::bool_chirho())],
    );
    let application_chirho =
        super::test_support_chirho::type_family_app1_chirho("StepChirho", TyChirho::int_chirho());
    let input_chirho = TyChirho::TupleChirho(vec![application_chirho; 64]);
    let mut work_chirho = 4096;
    let mut output_chirho = 64;
    assert!(
        context_chirho
            .reduce_families_bounded_chirho(&input_chirho, 0, &mut work_chirho, &mut output_chirho,)
            .is_none()
    );
    assert!(work_chirho < 4096);
    assert!(output_chirho < 64);
    let mut work_chirho = 4096;
    let mut output_chirho = 4096;
    assert!(
        context_chirho
            .reduce_families_bounded_chirho(
                &TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                128,
                &mut work_chirho,
                &mut output_chirho,
            )
            .is_none()
    );
    assert!(work_chirho < 4096);
    assert!(output_chirho < 4096);
}
