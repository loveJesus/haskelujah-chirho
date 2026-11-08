// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exercise the actual generalization/method-context coalescer and its shared
//! module budget. These are typed internal controls, not GHC-output claims.
use super::*;
use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, CompilerClassChirho, PredicateClassProofChirho,
    PredicateMetadataRefusalChirho,
};

fn predicate_chirho() -> SchemePredChirho {
    SchemePredChirho::from_compiler_chirho(
        CompilerClassChirho::EqChirho,
        crate::ty_chirho::SchemePredPartsChirho {
            kind_tys_chirho: vec![],
            class_name_chirho: "Eq".to_owned(),
            ty_chirho: TyChirho::VarChirho(TyVarChirho(20)),
            extra_tys_chirho: vec![],
        },
    )
}

#[test]
fn soft_metadata_exhaustion_never_adds_an_ordinary_error_or_skips_dedup_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let limit_chirho = ctx_chirho.class_env_chirho.registration_limit_chirho();
    assert!(
        ctx_chirho
            .class_env_chirho
            .charge_predicate_metadata_chirho(limit_chirho)
    );
    let old_chirho = predicate_chirho();
    let values_chirho =
        ctx_chirho.coalesce_scheme_predicates_chirho(vec![old_chirho.clone(), old_chirho.clone()]);
    assert_eq!(values_chirho, vec![old_chirho.clone()]);
    assert_eq!(
        values_chirho[0].class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(
            PredicateMetadataRefusalChirho::WorkChirho { limit_chirho }
        ))
    );
    assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(!ctx_chirho.has_non_deferrable_failure_chirho());
    let restored_chirho = ctx_chirho
        .class_env_chirho
        .try_snapshot_chirho()
        .expect("normal method restore point");
    ctx_chirho.class_env_chirho = restored_chirho;
    let restored_values_chirho =
        ctx_chirho.coalesce_scheme_predicates_chirho(vec![old_chirho.clone(), old_chirho.clone()]);
    assert_eq!(
        restored_values_chirho[0].class_identity_chirho(),
        values_chirho[0].class_identity_chirho()
    );
    let mut next_chirho =
        InferCtxChirho::new_with_imported_class_env_chirho(&ctx_chirho.class_env_chirho);
    let next_values_chirho =
        next_chirho.coalesce_scheme_predicates_chirho(vec![old_chirho.clone(), old_chirho.clone()]);
    assert_eq!(
        next_values_chirho[0].class_identity_chirho(),
        old_chirho.class_identity_chirho()
    );
}

#[test]
fn deep_and_large_method_contexts_keep_the_same_predicates_without_errors_chirho() {
    for deep_chirho in [false, true] {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let mut old_chirho = predicate_chirho();
        if deep_chirho {
            for _index_chirho in 0..256 {
                old_chirho.ty_chirho = TyChirho::ListChirho(Box::new(old_chirho.ty_chirho));
            }
        } else {
            old_chirho
                .extra_tys_chirho
                .push(TyChirho::ConChirho("x".repeat(5000)));
        }
        let values_chirho = ctx_chirho
            .coalesce_scheme_predicates_chirho(vec![old_chirho.clone(), old_chirho.clone()]);
        assert_eq!(values_chirho, vec![old_chirho]);
        assert!(matches!(
            values_chirho[0].class_identity_chirho(),
            PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(_))
        ));
        assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
        assert!(!ctx_chirho.has_non_deferrable_failure_chirho());
    }
}

#[test]
fn actual_generalization_conserves_ordinary_predicates_past_metadata_limits_chirho() {
    for case_chirho in 0..3 {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let variable_chirho = ctx_chirho.fresh_var_chirho();
        let mut argument_chirho = variable_chirho.clone();
        match case_chirho {
            0 => {
                argument_chirho = TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("x".repeat(5000))),
                    Box::new(argument_chirho),
                )
            }
            1 => {
                for _index_chirho in 0..256 {
                    argument_chirho = TyChirho::ListChirho(Box::new(argument_chirho));
                }
            }
            _ => {
                assert!(
                    ctx_chirho
                        .class_env_chirho
                        .charge_predicate_metadata_chirho(
                            ctx_chirho.class_env_chirho.registration_limit_chirho()
                        )
                );
            }
        }
        let wanted_chirho =
            PredChirho::compiler_chirho(CompilerClassChirho::EqChirho, argument_chirho.clone());
        ctx_chirho.deferred_preds_chirho.extend([
            (wanted_chirho.clone(), SpanChirho::DUMMY_CHIRHO),
            (wanted_chirho, SpanChirho::DUMMY_CHIRHO),
        ]);
        let body_chirho = TyChirho::fun_chirho(variable_chirho.clone(), variable_chirho);
        let scheme_chirho = ctx_chirho.generalize_chirho(&body_chirho);
        assert_eq!(scheme_chirho.ty_chirho, body_chirho);
        assert_eq!(scheme_chirho.preds_chirho.len(), 1);
        assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Eq");
        assert_eq!(scheme_chirho.preds_chirho[0].ty_chirho, argument_chirho);
        assert!(scheme_chirho.preds_chirho[0].kind_tys_chirho.is_empty());
        assert!(scheme_chirho.preds_chirho[0].extra_tys_chirho.is_empty());
        assert!(matches!(
            scheme_chirho.preds_chirho[0].class_identity_chirho(),
            PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(_))
        ));
        assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
        assert!(!ctx_chirho.has_non_deferrable_failure_chirho());
        let (instantiated_body_chirho, instantiated_predicates_chirho) =
            ctx_chirho.instantiate_scheme_parts_chirho(&scheme_chirho);
        assert_eq!(instantiated_predicates_chirho.len(), 1);
        assert_eq!(instantiated_predicates_chirho[0].class_name_chirho, "Eq");
        assert_eq!(
            instantiated_predicates_chirho[0].class_identity_chirho(),
            scheme_chirho.preds_chirho[0].class_identity_chirho()
        );
        let TyChirho::FunChirho(variable_chirho, _, _) = instantiated_body_chirho else {
            panic!("ordinary body keeps its function type");
        };
        let mut instantiated_argument_chirho = *variable_chirho;
        match case_chirho {
            0 => {
                instantiated_argument_chirho = TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("x".repeat(5000))),
                    Box::new(instantiated_argument_chirho),
                )
            }
            1 => {
                for _index_chirho in 0..256 {
                    instantiated_argument_chirho =
                        TyChirho::ListChirho(Box::new(instantiated_argument_chirho));
                }
            }
            _ => {}
        }
        assert_eq!(
            instantiated_predicates_chirho[0].ty_chirho,
            instantiated_argument_chirho
        );
        assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
        assert!(!ctx_chirho.has_non_deferrable_failure_chirho());
    }
}

fn polykinded_predicate_scheme_chirho() -> SchemeChirho {
    SchemeChirho {
        vars_chirho: vec![TyVarChirho(800), TyVarChirho(801)].into(),
        preds_chirho: vec![SchemePredChirho::from_compiler_chirho(
            CompilerClassChirho::TypeableChirho,
            crate::ty_chirho::SchemePredPartsChirho {
                kind_tys_chirho: vec![TyChirho::VarChirho(TyVarChirho(800))],
                class_name_chirho: "Typeable".to_owned(),
                ty_chirho: TyChirho::VarChirho(TyVarChirho(801)),
                extra_tys_chirho: Vec::new(),
            },
        )],
        ty_chirho: TyChirho::VarChirho(TyVarChirho(801)),
    }
}

#[test]
fn predicate_telescope_survives_fresh_explicit_and_rigid_instantiation_chirho() {
    let scheme_chirho = polykinded_predicate_scheme_chirho();
    let proof_chirho = scheme_chirho.preds_chirho[0]
        .class_identity_chirho()
        .clone();
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let (body_chirho, predicates_chirho) =
        ctx_chirho.instantiate_scheme_parts_chirho(&scheme_chirho);
    assert_eq!(predicates_chirho[0].ty_chirho, body_chirho);
    assert_eq!(predicates_chirho[0].kind_tys_chirho.len(), 1);
    assert_ne!(predicates_chirho[0].kind_tys_chirho[0], body_chirho);
    assert_eq!(predicates_chirho[0].class_identity_chirho(), &proof_chirho);
    let (_, predicates_chirho) = ctx_chirho
        .instantiate_scheme_parts_with_visible_type_args_chirho(
            &scheme_chirho,
            &[
                TyChirho::ConChirho("Nat".to_owned()),
                TyChirho::ConChirho("3".to_owned()),
            ],
            &[None, None],
            SpanChirho::DUMMY_CHIRHO,
        )
        .expect("full explicit telescope");
    assert_eq!(
        predicates_chirho[0].kind_tys_chirho,
        vec![TyChirho::ConChirho("Nat".to_owned())]
    );
    assert_eq!(
        predicates_chirho[0].ty_chirho,
        TyChirho::ConChirho("3".to_owned())
    );
    assert_eq!(predicates_chirho[0].class_identity_chirho(), &proof_chirho);
    ctx_chirho
        .tyvar_source_names_chirho
        .insert(TyVarChirho(800), "kChirho".to_owned());
    ctx_chirho
        .tyvar_source_names_chirho
        .insert(TyVarChirho(801), "aChirho".to_owned());
    let (body_chirho, predicates_chirho) = ctx_chirho.skolemize_scheme_parts_chirho(&scheme_chirho);
    assert_eq!(predicates_chirho[0].ty_chirho, body_chirho);
    assert_eq!(predicates_chirho[0].class_identity_chirho(), &proof_chirho);
    assert!(ctx_chirho.pred_mentions_rigid_chirho(&predicates_chirho[0]));
    assert!(!ctx_chirho.pred_fully_ground_chirho(&predicates_chirho[0]));
}

#[test]
fn generalization_closes_over_hidden_kinds_without_losing_metadata_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let argument_chirho = ctx_chirho.fresh_var_chirho();
    let kind_chirho = ctx_chirho.fresh_var_chirho();
    let mut predicate_chirho =
        PredChirho::compiler_chirho(CompilerClassChirho::TypeableChirho, argument_chirho.clone());
    predicate_chirho.kind_tys_chirho.push(kind_chirho.clone());
    let proof_chirho = predicate_chirho.class_identity_chirho().clone();
    assert!(!ctx_chirho.pred_fully_ground_chirho(&predicate_chirho));
    ctx_chirho
        .deferred_preds_chirho
        .push((predicate_chirho, SpanChirho::DUMMY_CHIRHO));
    let scheme_chirho = ctx_chirho.generalize_chirho(&argument_chirho);
    assert_eq!(scheme_chirho.preds_chirho.len(), 1);
    assert_eq!(
        scheme_chirho.preds_chirho[0].kind_tys_chirho,
        vec![kind_chirho.clone()]
    );
    assert_eq!(
        scheme_chirho.preds_chirho[0].class_identity_chirho(),
        &proof_chirho
    );
    for variable_chirho in kind_chirho
        .free_vars_chirho()
        .into_iter()
        .chain(argument_chirho.free_vars_chirho())
    {
        assert!(scheme_chirho.vars_chirho.contains(&variable_chirho));
    }
}
