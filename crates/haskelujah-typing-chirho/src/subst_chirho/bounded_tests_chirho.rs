// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn reduce_chirho(term_chirho: &TyChirho, argument_chirho: &TyChirho) -> TyChirho {
    SubstChirho::reduce_checked_argument_chirho(term_chirho, argument_chirho, true, &mut |_, _| {
        Some(())
    })
    .expect("bounded checked substitution")
    .expect("checked forall lambda")
    .0
}

#[test]
fn checked_beta_keeps_context_and_partial_required_prefix_chirho() {
    let term_chirho = TyChirho::RequiredForallChirho {
        vars_chirho: vec![TyVarChirho(0), TyVarChirho(1)].into(),
        body_chirho: Box::new(TyChirho::QualChirho {
            context_chirho: vec![TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("CChirho".into())),
                Box::new(TyChirho::VarChirho(TyVarChirho(0))),
            )],
            body_chirho: Box::new(TyChirho::fun_chirho(
                TyChirho::VarChirho(TyVarChirho(0)),
                TyChirho::VarChirho(TyVarChirho(1)),
            )),
        }),
    };
    assert_eq!(
        reduce_chirho(&term_chirho, &TyChirho::int_chirho()),
        TyChirho::RequiredForallChirho {
            vars_chirho: vec![TyVarChirho(1)].into(),
            body_chirho: Box::new(TyChirho::QualChirho {
                context_chirho: vec![TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("CChirho".into())),
                    Box::new(TyChirho::int_chirho())
                )],
                body_chirho: Box::new(TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::VarChirho(TyVarChirho(1))
                )),
            }),
        }
    );
}

#[test]
fn checked_beta_keeps_outer_context_and_nested_binder_mask_chirho() {
    let term_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        body_chirho: Box::new(TyChirho::QualChirho {
            context_chirho: vec![TyChirho::VarChirho(TyVarChirho(0))],
            body_chirho: Box::new(TyChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho(0)].into(),
                body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(0))),
            }),
        }),
    };
    assert_eq!(
        reduce_chirho(&term_chirho, &TyChirho::int_chirho()),
        TyChirho::QualChirho {
            context_chirho: vec![TyChirho::int_chirho()],
            body_chirho: Box::new(TyChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho(0)].into(),
                body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(0)))
            }),
        }
    );
}

#[test]
fn bounded_images_are_simultaneous_chirho() {
    let mut replacement_chirho =
        SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::VarChirho(TyVarChirho(1)));
    replacement_chirho.insert_chirho(TyVarChirho(1), TyChirho::bool_chirho());
    assert_eq!(
        replacement_chirho
            .apply_bounded_chirho(&TyChirho::VarChirho(TyVarChirho(0)), &mut |_, _| Some(())),
        Some(TyChirho::VarChirho(TyVarChirho(1)))
    );
}

#[test]
fn checked_beta_refuses_capture_chirho() {
    let term_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0), TyVarChirho(1)].into(),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(0))),
    };
    assert!(
        SubstChirho::reduce_checked_argument_chirho(
            &term_chirho,
            &TyChirho::VarChirho(TyVarChirho(1)),
            true,
            &mut |_, _| Some(())
        )
        .is_none()
    );
}

#[test]
fn ordinary_forall_shape_does_not_authorize_beta_chirho() {
    let term_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(0))),
    };
    assert_eq!(
        SubstChirho::reduce_checked_argument_chirho(
            &term_chirho,
            &TyChirho::int_chirho(),
            false,
            &mut |_, _| Some(())
        ),
        Some(None)
    );
}

#[test]
fn checked_beta_respects_shared_allowance_and_depth_chirho() {
    let mut term_chirho = TyChirho::VarChirho(TyVarChirho(0));
    for _index_chirho in 0..130 {
        term_chirho = TyChirho::ListChirho(Box::new(term_chirho));
    }
    let replacement_chirho = SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::int_chirho());
    assert!(
        replacement_chirho
            .apply_bounded_chirho(&term_chirho, &mut |_, _| Some(()))
            .is_none()
    );
    assert!(
        replacement_chirho
            .apply_bounded_chirho(&TyChirho::VarChirho(TyVarChirho(0)), &mut |_, _| None)
            .is_none()
    );
}

#[test]
fn scheme_environment_keeps_predicate_kind_and_extra_free_variables_chirho() {
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![TyVarChirho(0)].into(),
        preds_chirho: vec![crate::ty_chirho::SchemePredChirho::legacy_chirho(
            crate::ty_chirho::SchemePredPartsChirho {
                kind_tys_chirho: vec![TyChirho::VarChirho(TyVarChirho(1))],
                class_name_chirho: "SourceClassChirho".into(),
                ty_chirho: TyChirho::VarChirho(TyVarChirho(2)),
                extra_tys_chirho: vec![
                    TyChirho::VarChirho(TyVarChirho(3)),
                    TyChirho::VarChirho(TyVarChirho(0)),
                ],
            },
        )],
        ty_chirho: TyChirho::bool_chirho(),
    };
    assert_eq!(
        scheme_chirho.free_vars_chirho(),
        vec![TyVarChirho(1), TyVarChirho(2), TyVarChirho(3)]
    );
    let mut environment_chirho = crate::env_chirho::TyEnvChirho::new_chirho();
    environment_chirho.bind_chirho("capturedChirho".into(), scheme_chirho);
    let mut variables_chirho = environment_chirho.free_vars_chirho();
    variables_chirho.sort_unstable();
    assert_eq!(
        variables_chirho,
        vec![TyVarChirho(1), TyVarChirho(2), TyVarChirho(3)]
    );
}

fn record_chirho(
    variable_chirho: u32,
    kind_chirho: TyChirho,
) -> crate::ty_chirho::QuantifiedVariableChirho {
    crate::ty_chirho::QuantifiedVariableChirho {
        variable_chirho: TyVarChirho(variable_chirho), kind_chirho: Some(std::sync::Arc::new(kind_chirho)),
        source_chirho: Some(haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho::QuantifiedChirho {
            declaration_chirho: "sourceChirho".into(), index_chirho: variable_chirho }),
        specificity_chirho: Some(haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho),
    }
}

#[test]
fn alpha_freshening_maps_kind_dependencies_with_the_same_prefix_chirho() {
    let source_chirho = vec![
        record_chirho(0, TyChirho::ConChirho("Type".into())),
        record_chirho(1, TyChirho::VarChirho(TyVarChirho(0))),
    ];
    let term_chirho = TyChirho::ForallChirho {
        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(
            source_chirho.clone(),
        ),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(1))),
    };
    let mut next_chirho = 0;
    let TyChirho::ForallChirho {
        vars_chirho,
        body_chirho,
    } = SubstChirho::empty_chirho()
        .apply_freshened_bounded_chirho(&term_chirho, &mut next_chirho, &mut |_, _| Some(()))
        .expect("bounded freshening")
    else {
        panic!("forall retained")
    };
    assert_ne!(vars_chirho[0], TyVarChirho(0));
    assert_eq!(
        vars_chirho.records_chirho()[1].kind_chirho.as_deref(),
        Some(&TyChirho::VarChirho(vars_chirho[0]))
    );
    assert_eq!(*body_chirho, TyChirho::VarChirho(vars_chirho[1]));
    for (before_chirho, after_chirho) in source_chirho.iter().zip(vars_chirho.records_chirho()) {
        assert_eq!(before_chirho.source_chirho, after_chirho.source_chirho);
        assert_eq!(
            before_chirho.specificity_chirho,
            after_chirho.specificity_chirho
        );
    }
}

#[test]
fn binder_kinds_see_only_preceding_bindings_chirho() {
    let term_chirho = TyChirho::ForallChirho {
        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
            record_chirho(0, TyChirho::VarChirho(TyVarChirho(1))),
            record_chirho(1, TyChirho::VarChirho(TyVarChirho(0))),
        ]),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(1))),
    };
    let mut substitution_chirho =
        SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::bool_chirho());
    substitution_chirho.insert_chirho(TyVarChirho(1), TyChirho::int_chirho());
    let TyChirho::ForallChirho {
        vars_chirho,
        body_chirho,
    } = substitution_chirho
        .apply_bounded_chirho(&term_chirho, &mut |_, _| Some(()))
        .expect("sequential substitution")
    else {
        panic!("forall retained")
    };
    assert_eq!(
        vars_chirho.records_chirho()[0].kind_chirho.as_deref(),
        Some(&TyChirho::int_chirho())
    );
    assert_eq!(
        vars_chirho.records_chirho()[1].kind_chirho.as_deref(),
        Some(&TyChirho::VarChirho(TyVarChirho(0)))
    );
    assert_eq!(*body_chirho, TyChirho::VarChirho(TyVarChirho(1)));
}

#[test]
fn unrelated_binder_does_not_capture_an_outside_replacement_chirho() {
    let replacement_chirho =
        SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::VarChirho(TyVarChirho(1)));
    let scoped_chirho = |body_chirho| TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(1)].into(),
        body_chirho: Box::new(body_chirho),
    };
    let term_chirho = TyChirho::TupleChirho(vec![
        TyChirho::VarChirho(TyVarChirho(0)),
        scoped_chirho(TyChirho::int_chirho()),
    ]);
    assert_eq!(
        replacement_chirho.apply_bounded_chirho(&term_chirho, &mut |_, _| Some(())),
        Some(TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(TyVarChirho(1)),
            scoped_chirho(TyChirho::int_chirho())
        ]))
    );
    assert!(
        replacement_chirho
            .apply_bounded_chirho(
                &scoped_chirho(TyChirho::VarChirho(TyVarChirho(0))),
                &mut |_, _| Some(())
            )
            .is_none()
    );
}

#[test]
fn owned_zonking_preserves_scope_and_refuses_cycles_chirho() {
    let mut substitution_chirho =
        SubstChirho::singleton_chirho(TyVarChirho(0), TyChirho::VarChirho(TyVarChirho(1)));
    substitution_chirho.insert_chirho(TyVarChirho(1), TyChirho::int_chirho());
    assert_eq!(
        substitution_chirho
            .resolve_owned_bounded_chirho(&TyChirho::VarChirho(TyVarChirho(0)), &mut |_, _| Some(
                ()
            )),
        Some(TyChirho::int_chirho())
    );
    let term_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho(1)].into(),
        body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(0))),
    };
    assert_eq!(
        substitution_chirho.resolve_owned_bounded_chirho(&term_chirho, &mut |_, _| Some(())),
        Some(TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(1)].into(),
            body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(1)))
        })
    );
    substitution_chirho.insert_chirho(TyVarChirho(1), TyChirho::VarChirho(TyVarChirho(0)));
    assert!(
        substitution_chirho
            .resolve_owned_bounded_chirho(
                &TyChirho::VarChirho(TyVarChirho(0)),
                &mut |_, _| Some(())
            )
            .is_none()
    );
}

#[test]
fn closed_kind_cost_is_independent_of_unused_prefix_images_chirho() {
    let mut prefix_chirho = SubstChirho::empty_chirho();
    for index_chirho in 0..2000 {
        prefix_chirho.insert_chirho(TyVarChirho(index_chirho), TyChirho::int_chirho());
    }
    let mut left_chirho = 24usize;
    assert_eq!(
        prefix_chirho.apply_bounded_chirho(
            &TyChirho::ConChirho("Type".into()),
            &mut |work_chirho, output_chirho| {
                left_chirho = left_chirho.checked_sub(work_chirho.max(output_chirho))?;
                Some(())
            }
        ),
        Some(TyChirho::ConChirho("Type".into()))
    );
}

#[test]
fn kind_only_depth_uses_the_same_substitution_allowance_chirho() {
    let mut kind_chirho = TyChirho::int_chirho();
    for _index_chirho in 0..130 {
        kind_chirho = TyChirho::ListChirho(Box::new(kind_chirho));
    }
    let term_chirho = TyChirho::ForallChirho {
        vars_chirho: crate::ty_chirho::QuantifiedVariablesChirho::from_records_chirho(vec![
            record_chirho(0, kind_chirho),
        ]),
        body_chirho: Box::new(TyChirho::int_chirho()),
    };
    assert!(
        SubstChirho::empty_chirho()
            .apply_bounded_chirho(&term_chirho, &mut |_, _| Some(()))
            .is_none()
    );
}
