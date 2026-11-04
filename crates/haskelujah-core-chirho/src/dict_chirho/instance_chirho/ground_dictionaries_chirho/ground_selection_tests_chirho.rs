// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Structural selection of a ground goal's instance: the unique head match with its substituted
//! context, a refusal by name when none or several match, the ground test, and the type bounds
//! that refuse a goal or a candidate by name before anything copies it.

use super::ground_budget_chirho::{
    GROUND_CANDIDATE_WEIGHT_LIMIT_CHIRHO, GROUND_GOAL_WEIGHT_LIMIT_CHIRHO, GroundBudgetChirho,
    ty_weight_capped_chirho,
};
use super::ground_selection_chirho::{
    GroundRefusalChirho, pred_is_ground_chirho, select_ground_instance_chirho,
};
use haskelujah_typing_chirho::class_chirho::{ClassEnvChirho, InstDeclChirho, PredChirho};
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

fn con_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ConChirho(name_chirho.to_string())
}

fn succ_chirho(inner_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(Box::new(con_chirho("SuccChirho")), Box::new(inner_chirho))
}

/// `Succ` applied `count_chirho` times to `Zero`.
fn chain_chirho(count_chirho: usize) -> TyChirho {
    (0..count_chirho).fold(con_chirho("ZeroChirho"), |inner_chirho, _| {
        succ_chirho(inner_chirho)
    })
}

fn known_nat_env_chirho() -> ClassEnvChirho {
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "KnownNatChirho".to_string(),
            head_ty_chirho: con_chirho("ZeroChirho"),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: Vec::new(),
        },
    ));
    let n_chirho = TyChirho::VarChirho(TyVarChirho(9001));
    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "KnownNatChirho".to_string(),
            head_ty_chirho: succ_chirho(n_chirho.clone()),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: vec![PredChirho::new_chirho("KnownNatChirho", n_chirho)],
        },
    ));
    env_chirho
}

/// Selection with a fresh budget, at the goal's own weight.
fn select_chirho<'env_chirho>(
    env_chirho: &'env_chirho ClassEnvChirho,
    goal_chirho: &PredChirho,
) -> Result<(&'env_chirho InstDeclChirho, Vec<PredChirho>), GroundRefusalChirho> {
    let mut budget_chirho = GroundBudgetChirho::default();
    let weight_chirho = budget_chirho.goal_weight_chirho(goal_chirho)?;
    select_ground_instance_chirho(env_chirho, goal_chirho, weight_chirho, &mut budget_chirho)
}

#[test]
fn a_ground_goal_selects_its_unique_instance_with_the_substituted_context_chirho() {
    let env_chirho = known_nat_env_chirho();
    let goal_chirho = PredChirho::new_chirho("KnownNatChirho", chain_chirho(2));
    let (instance_chirho, obligations_chirho) =
        select_chirho(&env_chirho, &goal_chirho).expect("one instance matches");
    assert!(!instance_chirho.context_chirho.is_empty());
    assert_eq!(
        obligations_chirho,
        vec![PredChirho::new_chirho("KnownNatChirho", chain_chirho(1))]
    );
}

#[test]
fn a_goal_no_head_matches_is_refused_by_name_chirho() {
    let env_chirho = known_nat_env_chirho();
    let goal_chirho = PredChirho::new_chirho("KnownNatChirho", con_chirho("BoolChirho"));
    assert_eq!(
        select_chirho(&env_chirho, &goal_chirho).map(|_| ()),
        Err(GroundRefusalChirho::UnresolvedChirho(goal_chirho))
    );
}

#[test]
fn several_matching_heads_are_refused_never_chosen_chirho() {
    let mut env_chirho = known_nat_env_chirho();
    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "KnownNatChirho".to_string(),
            head_ty_chirho: TyChirho::VarChirho(TyVarChirho(9002)),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: Vec::new(),
        },
    ));
    let goal_chirho = PredChirho::new_chirho("KnownNatChirho", con_chirho("ZeroChirho"));
    assert!(matches!(
        select_chirho(&env_chirho, &goal_chirho),
        Err(GroundRefusalChirho::OverlapChirho(_, heads_chirho)) if heads_chirho.len() == 2
    ));
}

#[test]
fn a_goal_with_a_meta_or_a_rigid_variable_is_not_ground_chirho() {
    let meta_chirho = PredChirho::new_chirho(
        "KnownNatChirho",
        succ_chirho(TyChirho::VarChirho(TyVarChirho(7))),
    );
    let rigid_chirho = PredChirho::new_chirho(
        "KnownNatChirho",
        succ_chirho(TyChirho::ForallVarChirho("n%1".to_string())),
    );
    let ground_chirho = PredChirho::new_chirho("KnownNatChirho", chain_chirho(1));
    assert!(!pred_is_ground_chirho(&meta_chirho));
    assert!(!pred_is_ground_chirho(&rigid_chirho));
    assert!(pred_is_ground_chirho(&ground_chirho));
}

#[test]
fn measuring_a_type_stops_at_its_cap_chirho() {
    let chain_ty_chirho = chain_chirho(100);
    let full_chirho = ty_weight_capped_chirho(&chain_ty_chirho, usize::MAX)
        .expect("an uncapped measure completes");
    assert_eq!(
        ty_weight_capped_chirho(&chain_ty_chirho, full_chirho),
        Some(full_chirho)
    );
    assert_eq!(
        ty_weight_capped_chirho(&chain_ty_chirho, full_chirho - 1),
        None
    );
}

#[test]
fn a_goal_heavier_than_one_goal_may_be_is_refused_before_matching_chirho() {
    let goal_chirho = PredChirho::new_chirho(
        "KnownNatChirho",
        chain_chirho(GROUND_GOAL_WEIGHT_LIMIT_CHIRHO),
    );
    assert_eq!(
        GroundBudgetChirho::default().goal_weight_chirho(&goal_chirho),
        Err(GroundRefusalChirho::BudgetChirho(
            "goal weight",
            GROUND_GOAL_WEIGHT_LIMIT_CHIRHO
        ))
    );
}

#[test]
fn a_candidate_that_could_copy_past_its_limit_is_refused_before_matching_chirho() {
    // `instance C (a, .., a) => C a`, 32 occurrences: each may copy the whole goal.
    let a_chirho = TyChirho::VarChirho(TyVarChirho(9003));
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "GrowChirho".to_string(),
            head_ty_chirho: a_chirho.clone(),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: vec![PredChirho::new_chirho(
                "GrowChirho",
                TyChirho::TupleChirho(vec![a_chirho; 32]),
            )],
        },
    ));
    // The heaviest chain one goal may weigh; 32 copies of it cannot fit one candidate.
    let goal_chirho = (1..)
        .map(|count_chirho| PredChirho::new_chirho("GrowChirho", chain_chirho(count_chirho)))
        .take_while(|goal_chirho| {
            GroundBudgetChirho::default()
                .goal_weight_chirho(goal_chirho)
                .is_ok()
        })
        .last()
        .expect("a short chain is within one goal's weight");
    let mut budget_chirho = GroundBudgetChirho::default();
    let weight_chirho = budget_chirho
        .goal_weight_chirho(&goal_chirho)
        .expect("the goal itself is within its own weight");
    assert_eq!(
        select_ground_instance_chirho(&env_chirho, &goal_chirho, weight_chirho, &mut budget_chirho)
            .map(|_| ()),
        Err(GroundRefusalChirho::BudgetChirho(
            "candidate weight",
            GROUND_CANDIDATE_WEIGHT_LIMIT_CHIRHO
        ))
    );
}

#[test]
fn an_exhausted_budget_refuses_selection_by_name_chirho() {
    let env_chirho = known_nat_env_chirho();
    let goal_chirho = PredChirho::new_chirho("KnownNatChirho", chain_chirho(1));
    let mut budget_chirho = GroundBudgetChirho::exhausted_chirho();
    assert!(matches!(
        select_ground_instance_chirho(&env_chirho, &goal_chirho, 1, &mut budget_chirho),
        Err(GroundRefusalChirho::BudgetChirho("type work", _))
    ));
}

#[test]
fn repeated_head_variables_choose_only_a_consistent_goal_chirho() {
    // The checker shares one substitution across the complete head. Repetition
    // accepts the equal pair and rejects the unequal pair.
    let a_chirho = TyChirho::VarChirho(TyVarChirho(9004));
    let pair_chirho = |left_chirho: TyChirho, right_chirho: TyChirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(con_chirho("PairChirho")),
                Box::new(left_chirho),
            )),
            Box::new(right_chirho),
        )
    };
    let mut env_chirho = ClassEnvChirho::new_chirho();
    env_chirho.add_instance_chirho(InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "SameChirho".to_string(),
            head_ty_chirho: pair_chirho(a_chirho.clone(), a_chirho.clone()),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: vec![PredChirho::new_chirho("Show", a_chirho)],
        },
    ));
    let goal_chirho = PredChirho::new_chirho(
        "SameChirho",
        pair_chirho(con_chirho("Int"), con_chirho("Bool")),
    );
    assert!(matches!(
        select_chirho(&env_chirho, &goal_chirho),
        Err(GroundRefusalChirho::UnresolvedChirho(_))
    ));
    let consistent_chirho = PredChirho::new_chirho(
        "SameChirho",
        pair_chirho(con_chirho("Int"), con_chirho("Int")),
    );
    let (_, context_chirho) = select_chirho(&env_chirho, &consistent_chirho)
        .expect("the consistent repeated head selects its actual instance");
    assert_eq!(
        context_chirho,
        vec![PredChirho::new_chirho("Show", con_chirho("Int"))]
    );
}

#[test]
fn a_large_context_class_name_is_bounded_before_substitution_chirho() {
    let instance_chirho = InstDeclChirho::legacy_chirho(
        haskelujah_typing_chirho::class_chirho::InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "CChirho".to_owned(),
            head_ty_chirho: TyChirho::VarChirho(TyVarChirho(9020)),
            extra_head_tys_chirho: Vec::new(),
            context_chirho: vec![PredChirho::new_chirho(
                &"LongClassChirho".repeat(GROUND_CANDIDATE_WEIGHT_LIMIT_CHIRHO),
                con_chirho("Int"),
            )],
        },
    );
    let mut budget_chirho = GroundBudgetChirho::default();
    assert!(matches!(
        budget_chirho.candidate_chirho(&instance_chirho, 4),
        Err(GroundRefusalChirho::BudgetChirho("candidate weight", _))
    ));
}
