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
    }
}
