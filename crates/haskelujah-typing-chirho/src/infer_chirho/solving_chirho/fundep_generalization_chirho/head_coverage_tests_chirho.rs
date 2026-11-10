// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Uncovered and unsuccessfully checked arguments cannot become instance matches.

use super::*;

fn partial_context_chirho(fundeps_chirho: Vec<(Vec<usize>, Vec<usize>)>) -> InferCtxChirho {
    let mut context_chirho = context_chirho();
    let name_chirho = "PartialChirho";
    context_chirho
        .class_env_chirho
        .add_class_chirho(ClassDeclChirho::from_producer_chirho(
            ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: name_chirho.into(),
                supers_chirho: Vec::new(),
                var_chirho: TyVarChirho(50),
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![TyVarChirho(51), TyVarChirho(52)],
                fundeps_chirho,
                defaults_chirho: HashMap::new(),
            },
            proof_chirho(name_chirho),
        ));
    let owner_chirho = context_chirho
        .elaboration_state_chirho
        .owner_chirho
        .unwrap();
    let instance_chirho = InstDeclChirho::from_producer_chirho(
        InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: name_chirho.into(),
            head_ty_chirho: TyChirho::int_chirho(),
            extra_head_tys_chirho: vec![
                TyChirho::bool_chirho(),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".into())),
                    Box::new(TyChirho::bool_chirho()),
                ),
            ],
            context_chirho: Vec::new(),
        },
        proof_chirho(name_chirho),
        InstanceIdentityChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
    )
    .with_checked_binders_chirho(
        owner_chirho,
        QuantifiedVariablesChirho::from_records_chirho(Vec::new()),
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    for variable_chirho in [100, 101] {
        meta_chirho(
            &mut context_chirho,
            variable_chirho,
            ElaborationStateChirho::type_universe_chirho(),
        );
    }
    context_chirho.deferred_preds_chirho.push((
        PredChirho::from_proof_chirho(
            proof_chirho(name_chirho),
            PredPartsChirho {
                class_name_chirho: name_chirho.into(),
                kind_tys_chirho: Vec::new(),
                ty_chirho: TyChirho::int_chirho(),
                extra_tys_chirho: vec![
                    TyChirho::VarChirho(TyVarChirho(100)),
                    TyChirho::VarChirho(TyVarChirho(101)),
                ],
            },
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    context_chirho
}

fn assert_unselected_chirho(context_chirho: &mut InferCtxChirho) {
    let before_chirho = snapshot_chirho(context_chirho);
    assert!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert_eq!(snapshot_chirho(context_chirho), before_chirho);
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
}

#[test]
fn an_argument_outside_every_fundep_is_still_matched_without_improvement_chirho() {
    let mut context_chirho = partial_context_chirho(vec![(vec![0], vec![1])]);
    assert_unselected_chirho(&mut context_chirho);
    context_chirho.deferred_preds_chirho[0].0.extra_tys_chirho[1] = TyChirho::bool_chirho();
    assert_unselected_chirho(&mut context_chirho);
    context_chirho.deferred_preds_chirho[0].0.extra_tys_chirho[1] = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Maybe".into())),
        Box::new(TyChirho::bool_chirho()),
    );
    let delta_chirho = context_chirho.solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho());
    assert_eq!(
        delta_chirho.lookup_chirho(&TyVarChirho(100)),
        Some(&TyChirho::bool_chirho())
    );
    assert!(context_chirho.deferred_preds_chirho.is_empty());
}

#[test]
fn a_failed_group_never_covers_its_unmatched_determinant_chirho() {
    let mut context_chirho = partial_context_chirho(vec![(vec![0], vec![1]), (vec![2], vec![0])]);
    assert_unselected_chirho(&mut context_chirho);
    context_chirho.deferred_preds_chirho[0].0.extra_tys_chirho[1] = TyChirho::bool_chirho();
    assert_unselected_chirho(&mut context_chirho);
    context_chirho.deferred_preds_chirho[0].0.extra_tys_chirho[1] = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Maybe".into())),
        Box::new(TyChirho::bool_chirho()),
    );
    let delta_chirho = context_chirho.solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho());
    assert_eq!(
        delta_chirho.lookup_chirho(&TyVarChirho(100)),
        Some(&TyChirho::bool_chirho())
    );
    assert!(context_chirho.deferred_preds_chirho.is_empty());
}
