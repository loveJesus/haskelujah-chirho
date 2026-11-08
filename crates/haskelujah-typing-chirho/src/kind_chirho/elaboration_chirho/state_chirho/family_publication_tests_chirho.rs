// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Early family publication spends the original module's remaining allowance.

use super::*;
use crate::infer_chirho::FamilyHeadRoleChirho;

#[test]
fn full_equation_abstraction_shares_one_lhs_variable_scope_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let allowance_chirho = (
        state_chirho.work_left_chirho,
        state_chirho.output_left_chirho,
    );
    let kind_chirho = TyChirho::VarChirho(TyVarChirho(41));
    let visible_chirho = TyChirho::VarChirho(TyVarChirho(42));
    let (lhs_chirho, rhs_chirho, binders_chirho) = state_chirho
        .abstract_family_equation_chirho(
            &[kind_chirho.clone(), visible_chirho.clone()],
            &TyChirho::AppChirho(Box::new(visible_chirho), Box::new(kind_chirho)),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap()
        .unwrap();
    assert_eq!(binders_chirho.len(), 2);
    assert_eq!(
        rhs_chirho,
        TyChirho::AppChirho(
            Box::new(lhs_chirho[1].clone()),
            Box::new(lhs_chirho[0].clone())
        )
    );
    assert!(state_chirho.work_left_chirho < allowance_chirho.0);
    assert!(state_chirho.output_left_chirho < allowance_chirho.1);
    assert!(state_chirho.failure_chirho().is_none());
}

#[test]
fn abstraction_work_and_output_exhaustion_remain_terminal_chirho() {
    for output_chirho in [false, true] {
        let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
        if output_chirho {
            state_chirho.output_left_chirho = 1;
        } else {
            state_chirho.work_left_chirho = 1;
        }
        let owner_chirho = state_chirho.owner_chirho;
        let input_chirho = TyChirho::ConChirho("PayloadChirho".into());
        let failure_chirho = state_chirho
            .abstract_family_equation_chirho(
                std::slice::from_ref(&input_chirho),
                &input_chirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap_err();
        assert_eq!(state_chirho.failure_chirho(), Some(&failure_chirho));
        assert_eq!(state_chirho.owner_chirho, owner_chirho);
        assert_eq!(
            state_chirho
                .charge_chirho(0, 0, SpanChirho::DUMMY_CHIRHO)
                .unwrap_err(),
            failure_chirho
        );
        assert_eq!(
            state_chirho
                .abstract_family_equation_chirho(
                    &[],
                    &TyChirho::TupleChirho(vec![]),
                    SpanChirho::DUMMY_CHIRHO
                )
                .unwrap_err(),
            failure_chirho
        );
    }
}

#[test]
fn abstraction_depth_refusal_latches_at_the_producer_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let mut input_chirho = TyChirho::ConChirho("PayloadChirho".into());
    for _depth_chirho in 0..129 {
        input_chirho = TyChirho::ListChirho(Box::new(input_chirho));
    }
    let allowance_chirho = state_chirho.work_left_chirho;
    let failure_chirho = state_chirho
        .abstract_family_equation_chirho(
            &[input_chirho],
            &TyChirho::TupleChirho(vec![]),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap_err();
    assert_eq!(state_chirho.failure_chirho(), Some(&failure_chirho));
    assert!(state_chirho.work_left_chirho < allowance_chirho);
    assert_eq!(
        state_chirho
            .charge_chirho(0, 0, SpanChirho::DUMMY_CHIRHO)
            .unwrap_err(),
        failure_chirho
    );
}

#[test]
fn rhs_only_variables_are_refused_without_freshening_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let supply_chirho = state_chirho.next_variable_chirho;
    assert!(
        state_chirho
            .abstract_family_equation_chirho(
                &[],
                &TyChirho::VarChirho(TyVarChirho(8)),
                SpanChirho::DUMMY_CHIRHO
            )
            .is_err()
    );
    assert_eq!(state_chirho.next_variable_chirho, supply_chirho);
    assert!(state_chirho.failure_chirho().is_none());
}

#[test]
fn unsupported_equations_stay_held_and_charge_the_original_owner_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let allowance_chirho = state_chirho.work_left_chirho;
    let input_chirho = TyChirho::ForallChirho {
        vars_chirho: Default::default(),
        body_chirho: Box::new(TyChirho::TupleChirho(vec![])),
    };
    assert!(
        state_chirho
            .abstract_family_equation_chirho(
                &[input_chirho],
                &TyChirho::TupleChirho(vec![]),
                SpanChirho::DUMMY_CHIRHO
            )
            .unwrap()
            .is_none()
    );
    assert!(state_chirho.work_left_chirho < allowance_chirho);
    assert!(state_chirho.failure_chirho().is_none());
}

#[test]
fn branch_compatibility_exhaustion_spends_and_latches_the_owner_chirho() {
    for output_chirho in [false, true] {
        let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
        if output_chirho {
            state_chirho.output_left_chirho = 1;
        } else {
            state_chirho.work_left_chirho = 1;
        }
        let before_chirho = (
            state_chirho.work_left_chirho,
            state_chirho.output_left_chirho,
        );
        let input_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(TyVarChirho(4)),
            TyChirho::VarChirho(TyVarChirho(5)),
        ]);
        let failure_chirho = state_chirho
            .family_branches_compatible_chirho(
                std::slice::from_ref(&input_chirho),
                &input_chirho,
                std::slice::from_ref(&input_chirho),
                &input_chirho,
                &|_| FamilyHeadRoleChirho::UnknownChirho,
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap_err();
        assert_eq!(state_chirho.failure_chirho(), Some(&failure_chirho));
        assert!(
            state_chirho.work_left_chirho < before_chirho.0
                || state_chirho.output_left_chirho < before_chirho.1
        );
        assert_eq!(
            state_chirho
                .family_branches_compatible_chirho(
                    &[],
                    &TyChirho::TupleChirho(vec![]),
                    &[],
                    &TyChirho::TupleChirho(vec![]),
                    &|_| FamilyHeadRoleChirho::UnknownChirho,
                    SpanChirho::DUMMY_CHIRHO
                )
                .unwrap_err(),
            failure_chirho
        );
    }
}

#[test]
fn uncertain_branch_heads_remain_blockers_without_positive_proof_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let input_chirho = TyChirho::ConChirho("UnknownChirho".into());
    let allowance_chirho = state_chirho.work_left_chirho;
    assert!(
        !state_chirho
            .family_branches_compatible_chirho(
                std::slice::from_ref(&input_chirho),
                &input_chirho,
                std::slice::from_ref(&input_chirho),
                &input_chirho,
                &|_| FamilyHeadRoleChirho::UnknownChirho,
                SpanChirho::DUMMY_CHIRHO
            )
            .unwrap()
    );
    assert!(state_chirho.work_left_chirho < allowance_chirho);
    assert!(state_chirho.failure_chirho().is_none());
}

#[test]
fn administrative_depth_refusal_does_not_become_owner_exhaustion_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    state_chirho.output_left_chirho = 16_383;
    let mut input_chirho = TyChirho::VarChirho(TyVarChirho(4));
    for _depth_chirho in 0..129 {
        input_chirho = TyChirho::ListChirho(Box::new(input_chirho));
    }
    assert!(
        !state_chirho
            .family_branches_compatible_chirho(
                std::slice::from_ref(&input_chirho),
                &input_chirho,
                std::slice::from_ref(&input_chirho),
                &input_chirho,
                &|_| FamilyHeadRoleChirho::UnknownChirho,
                SpanChirho::DUMMY_CHIRHO
            )
            .unwrap()
    );
    assert!(state_chirho.failure_chirho().is_none());
    state_chirho
        .charge_chirho(1, 1, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
}

#[test]
fn branch_name_preflight_failure_latches_with_unspent_work_chirho() {
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    state_chirho.work_left_chirho = 32;
    let input_chirho = TyChirho::ConChirho("x".repeat(64));
    let failure_chirho = state_chirho
        .family_branches_compatible_chirho(
            std::slice::from_ref(&input_chirho),
            &input_chirho,
            std::slice::from_ref(&input_chirho),
            &input_chirho,
            &|_| FamilyHeadRoleChirho::UnknownChirho,
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap_err();
    assert!(state_chirho.work_left_chirho > 0);
    assert!(state_chirho.work_left_chirho < 32);
    assert_eq!(state_chirho.failure_chirho(), Some(&failure_chirho));
}
