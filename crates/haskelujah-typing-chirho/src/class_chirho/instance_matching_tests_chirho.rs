// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use crate::ty_chirho::MultChirho;

fn variable_chirho(index_chirho: u32) -> TyChirho {
    TyChirho::VarChirho(TyVarChirho(index_chirho))
}

fn application_chirho(name_chirho: &str, arguments_chirho: Vec<TyChirho>) -> TyChirho {
    arguments_chirho.into_iter().fold(
        TyChirho::ConChirho(name_chirho.to_owned()),
        |function_chirho, argument_chirho| {
            TyChirho::AppChirho(Box::new(function_chirho), Box::new(argument_chirho))
        },
    )
}

fn pair_chirho(left_chirho: TyChirho, right_chirho: TyChirho) -> TyChirho {
    application_chirho("PairChirho", vec![left_chirho, right_chirho])
}

fn instance_chirho(
    head_chirho: TyChirho,
    extras_chirho: Vec<TyChirho>,
    context_chirho: Vec<PredChirho>,
) -> InstDeclChirho {
    InstDeclChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: "ChoiceChirho".to_owned(),
        head_ty_chirho: head_chirho,
        extra_head_tys_chirho: extras_chirho,
        context_chirho,
    }
}

fn goal_chirho(head_chirho: TyChirho, extras_chirho: Vec<TyChirho>) -> PredChirho {
    PredChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: "ChoiceChirho".to_owned(),
        ty_chirho: head_chirho,
        extra_tys_chirho: extras_chirho,
    }
}

#[test]
fn a_repeated_nested_head_variable_cannot_choose_two_types_chirho() {
    let instance_chirho = instance_chirho(
        pair_chirho(
            variable_chirho(1),
            application_chirho("Maybe", vec![variable_chirho(1)]),
        ),
        vec![],
        vec![PredChirho::new_chirho("Eq", variable_chirho(1))],
    );
    let goal_chirho = goal_chirho(
        pair_chirho(
            TyChirho::int_chirho(),
            application_chirho("Maybe", vec![TyChirho::bool_chirho()]),
        ),
        vec![],
    );
    assert!(ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho).is_none());
}

#[test]
fn a_nonlinear_match_instantiates_every_context_argument_chirho() {
    let instance_chirho = instance_chirho(
        pair_chirho(variable_chirho(1), variable_chirho(1)),
        vec![application_chirho("Maybe", vec![variable_chirho(1)])],
        vec![
            PredChirho::new_chirho("Eq", variable_chirho(1)),
            PredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "RelatedChirho".to_owned(),
                ty_chirho: application_chirho("Maybe", vec![variable_chirho(1)]),
                extra_tys_chirho: vec![
                    variable_chirho(1),
                    TyChirho::ListChirho(Box::new(variable_chirho(1))),
                ],
            },
        ],
    );
    let goal_chirho = goal_chirho(
        pair_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
        vec![application_chirho("Maybe", vec![TyChirho::int_chirho()])],
    );
    assert_eq!(
        ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho),
        Some(vec![
            PredChirho::new_chirho("Eq", TyChirho::int_chirho()),
            PredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "RelatedChirho".to_owned(),
                ty_chirho: application_chirho("Maybe", vec![TyChirho::int_chirho()]),
                extra_tys_chirho: vec![
                    TyChirho::int_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                ],
            },
        ])
    );
}

#[test]
fn primary_and_extra_heads_share_one_variable_binding_chirho() {
    let instance_chirho = instance_chirho(
        application_chirho("Maybe", vec![variable_chirho(1)]),
        vec![variable_chirho(1)],
        vec![],
    );
    let goal_chirho = goal_chirho(
        application_chirho("Maybe", vec![TyChirho::int_chirho()]),
        vec![TyChirho::bool_chirho()],
    );
    assert!(ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho).is_none());
}

#[test]
fn repeats_inside_an_extra_head_are_consistent_too_chirho() {
    let instance_chirho = instance_chirho(
        TyChirho::int_chirho(),
        vec![pair_chirho(variable_chirho(1), variable_chirho(1))],
        vec![],
    );
    let goal_chirho = goal_chirho(
        TyChirho::int_chirho(),
        vec![pair_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho())],
    );
    assert!(ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho).is_none());
}

#[test]
fn two_target_metavariables_are_not_unified_by_instance_selection_chirho() {
    let instance_chirho = instance_chirho(
        pair_chirho(variable_chirho(1), variable_chirho(1)),
        vec![],
        vec![],
    );
    let goal_chirho = goal_chirho(
        pair_chirho(variable_chirho(41), variable_chirho(42)),
        vec![],
    );
    assert!(ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho).is_none());
    assert_eq!(
        goal_chirho.ty_chirho,
        pair_chirho(variable_chirho(41), variable_chirho(42))
    );
}

#[test]
fn arrow_multiplicity_must_match_in_both_directions_chirho() {
    for (pattern_mult_chirho, target_mult_chirho) in [
        (MultChirho::OneChirho, MultChirho::ManyChirho),
        (MultChirho::ManyChirho, MultChirho::OneChirho),
    ] {
        let instance_chirho = instance_chirho(
            TyChirho::FunChirho(
                Box::new(variable_chirho(1)),
                Box::new(variable_chirho(1)),
                pattern_mult_chirho,
            ),
            vec![],
            vec![],
        );
        let goal_chirho = goal_chirho(
            TyChirho::FunChirho(
                Box::new(TyChirho::int_chirho()),
                Box::new(TyChirho::int_chirho()),
                target_mult_chirho,
            ),
            vec![],
        );
        assert!(
            ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho).is_none()
        );
    }
}

#[test]
fn matching_arrow_multiplicity_preserves_the_context_chirho() {
    for mult_chirho in [MultChirho::OneChirho, MultChirho::ManyChirho] {
        let instance_chirho = instance_chirho(
            TyChirho::FunChirho(
                Box::new(variable_chirho(1)),
                Box::new(variable_chirho(1)),
                mult_chirho,
            ),
            vec![],
            vec![PredChirho::new_chirho("Eq", variable_chirho(1))],
        );
        let goal_chirho = goal_chirho(
            TyChirho::FunChirho(
                Box::new(TyChirho::bool_chirho()),
                Box::new(TyChirho::bool_chirho()),
                mult_chirho,
            ),
            vec![],
        );
        assert_eq!(
            ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho),
            Some(vec![PredChirho::new_chirho("Eq", TyChirho::bool_chirho())])
        );
    }
}

#[test]
fn list_sugar_matches_the_same_list_constructor_chirho() {
    for (pattern_chirho, target_chirho) in [
        (
            TyChirho::ListChirho(Box::new(variable_chirho(1))),
            application_chirho("[]", vec![TyChirho::bool_chirho()]),
        ),
        (
            application_chirho("[]", vec![variable_chirho(1)]),
            TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
        ),
    ] {
        let instance_chirho = instance_chirho(
            pattern_chirho,
            vec![],
            vec![PredChirho::new_chirho("Eq", variable_chirho(1))],
        );
        assert_eq!(
            ClassEnvChirho::instance_sub_goals_chirho(
                &instance_chirho,
                &goal_chirho(target_chirho, vec![])
            ),
            Some(vec![PredChirho::new_chirho("Eq", TyChirho::bool_chirho())])
        );
    }
}

#[test]
fn only_the_same_class_and_full_argument_count_can_match_chirho() {
    let instance_chirho = instance_chirho(variable_chirho(1), vec![variable_chirho(2)], vec![]);
    let mut goal_chirho = goal_chirho(TyChirho::int_chirho(), vec![TyChirho::bool_chirho()]);
    goal_chirho.class_name_chirho = "OtherChirho".to_owned();
    assert!(ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho).is_none());
    goal_chirho.class_name_chirho = "ChoiceChirho".to_owned();
    goal_chirho.extra_tys_chirho.clear();
    assert!(ClassEnvChirho::instance_sub_goals_chirho(&instance_chirho, &goal_chirho).is_none());
}

#[test]
fn alpha_duplicate_detection_uses_the_same_arrow_multiplicity_chirho() {
    let left_chirho = instance_chirho(
        TyChirho::FunChirho(
            Box::new(variable_chirho(1)),
            Box::new(variable_chirho(1)),
            MultChirho::OneChirho,
        ),
        vec![],
        vec![],
    );
    let mut right_chirho = left_chirho.clone();
    right_chirho.head_ty_chirho = TyChirho::FunChirho(
        Box::new(variable_chirho(4)),
        Box::new(variable_chirho(4)),
        MultChirho::OneChirho,
    );
    assert!(ClassEnvChirho::instance_heads_alpha_equal_chirho(
        &left_chirho,
        &right_chirho
    ));
    right_chirho.head_ty_chirho = TyChirho::FunChirho(
        Box::new(variable_chirho(4)),
        Box::new(variable_chirho(4)),
        MultChirho::ManyChirho,
    );
    assert!(!ClassEnvChirho::instance_heads_alpha_equal_chirho(
        &left_chirho,
        &right_chirho
    ));
}
