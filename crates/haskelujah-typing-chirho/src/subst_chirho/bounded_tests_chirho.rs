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
        vars_chirho: vec![TyVarChirho(0), TyVarChirho(1)],
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
            vars_chirho: vec![TyVarChirho(1)],
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
        vars_chirho: vec![TyVarChirho(0)],
        body_chirho: Box::new(TyChirho::QualChirho {
            context_chirho: vec![TyChirho::VarChirho(TyVarChirho(0))],
            body_chirho: Box::new(TyChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho(0)],
                body_chirho: Box::new(TyChirho::VarChirho(TyVarChirho(0))),
            }),
        }),
    };
    assert_eq!(
        reduce_chirho(&term_chirho, &TyChirho::int_chirho()),
        TyChirho::QualChirho {
            context_chirho: vec![TyChirho::int_chirho()],
            body_chirho: Box::new(TyChirho::ForallChirho {
                vars_chirho: vec![TyVarChirho(0)],
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
        vars_chirho: vec![TyVarChirho(0), TyVarChirho(1)],
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
        vars_chirho: vec![TyVarChirho(0)],
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
        source_binders_chirho: vec![],
        vars_chirho: vec![TyVarChirho(0)],
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
