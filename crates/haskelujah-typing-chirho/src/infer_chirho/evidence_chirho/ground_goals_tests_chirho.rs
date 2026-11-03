// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! S1 transport contracts for complete goals, independently of their Core consumer.

use super::*;
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::TyVarChirho;
use haskelujah_span_chirho::SpanChirho;

fn maybe_int_chirho() -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Maybe".to_owned())),
        Box::new(TyChirho::int_chirho()),
    )
}

#[test]
fn structural_goals_keep_the_full_closed_type_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let ty_chirho = TyChirho::ListChirho(Box::new(maybe_int_chirho()));
    assert_eq!(
        ground_pred_chirho(&ctx_chirho, "Eq", &ty_chirho, &[]),
        Some(PredChirho::new_chirho("Eq", ty_chirho))
    );
}

#[test]
fn reference_transport_uses_the_final_substitution_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let ty_chirho = ctx_chirho.fresh_var_chirho();
    let mut supply_chirho =
        haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
    let origin_chirho = supply_chirho.fresh_chirho();
    ctx_chirho.capture_reference_evidence_chirho(
        SpanChirho::DUMMY_CHIRHO,
        Some(origin_chirho),
        &[PredChirho::new_chirho("Eq", ty_chirho.clone())],
    );
    let TyChirho::VarChirho(var_chirho) = ty_chirho else {
        panic!("fresh meta")
    };
    let mut subst_chirho = SubstChirho::empty_chirho();
    subst_chirho.insert_chirho(var_chirho, maybe_int_chirho());
    let (_, by_origin_chirho) = ctx_chirho.finalize_reference_evidence_chirho(&subst_chirho);
    assert_eq!(
        by_origin_chirho[&origin_chirho][0].ground_pred_chirho,
        Some(PredChirho::new_chirho("Eq", maybe_int_chirho()))
    );
}

#[test]
fn open_rigid_polymorphic_and_uncertain_heads_supply_no_ground_goal_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    for ty_chirho in [
        TyChirho::VarChirho(TyVarChirho(0)),
        TyChirho::ForallVarChirho("a%1".to_owned()),
        TyChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho(0)],
            body_chirho: Box::new(TyChirho::int_chirho()),
        },
        TyChirho::RequiredForallChirho {
            vars_chirho: vec![TyVarChirho(0)],
            body_chirho: Box::new(TyChirho::int_chirho()),
        },
        TyChirho::ConChirho("UnknownChirho".to_owned()),
        TyChirho::ConChirho("AliasChirho.Int".to_owned()),
        TyChirho::ConChirho("%family:unresolved:producer-chirho".to_owned()),
    ] {
        assert!(
            ground_pred_chirho(&ctx_chirho, "Eq", &ty_chirho, &[]).is_none(),
            "{ty_chirho:?}"
        );
    }
}

#[test]
fn family_results_normalize_but_stuck_applications_do_not_escape_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.family_module_name_chirho = "GroundGoalChirho".to_owned();
    ctx_chirho.register_type_family_chirho(
        "FChirho".to_owned(),
        1,
        vec![(vec![TyChirho::int_chirho()], TyChirho::bool_chirho())],
    );
    let key_chirho = ctx_chirho
        .family_name_key_chirho("FChirho")
        .expect("registered family");
    let app_chirho = |ty_chirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(key_chirho.clone())),
            Box::new(ty_chirho),
        )
    };
    assert_eq!(
        ground_pred_chirho(&ctx_chirho, "Eq", &app_chirho(TyChirho::int_chirho()), &[]),
        Some(PredChirho::new_chirho("Eq", TyChirho::bool_chirho()))
    );
    assert!(
        ground_pred_chirho(&ctx_chirho, "Eq", &app_chirho(TyChirho::char_chirho()), &[]).is_none()
    );
}

#[test]
fn mptc_goals_require_every_argument_and_capture_cannot_invent_extras_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let mut declaration_chirho = ctx_chirho.class_env_chirho.classes_chirho["Eq"].clone();
    declaration_chirho.name_chirho = "PairChirho".to_owned();
    declaration_chirho.extra_vars_chirho = vec![TyVarChirho(10_000)];
    ctx_chirho
        .class_env_chirho
        .add_class_chirho(declaration_chirho);
    assert!(ground_pred_chirho(&ctx_chirho, "PairChirho", &TyChirho::int_chirho(), &[]).is_none());
    assert_eq!(
        ground_pred_chirho(
            &ctx_chirho,
            "PairChirho",
            &TyChirho::int_chirho(),
            &[TyChirho::bool_chirho()]
        ),
        Some(PredChirho::new_multi_chirho(
            "PairChirho",
            vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]
        ))
    );
    assert!(
        ground_pred_chirho(
            &ctx_chirho,
            "PairChirho",
            &TyChirho::int_chirho(),
            &[TyChirho::VarChirho(TyVarChirho(1))]
        )
        .is_none()
    );
    assert!(
        ground_pred_chirho(
            &ctx_chirho,
            "UnknownClassChirho",
            &TyChirho::int_chirho(),
            &[]
        )
        .is_none()
    );
    let mut supply_chirho =
        haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
    let origin_chirho = supply_chirho.fresh_chirho();
    ctx_chirho.capture_reference_evidence_chirho(
        SpanChirho::DUMMY_CHIRHO,
        Some(origin_chirho),
        &[PredChirho::new_multi_chirho(
            "PairChirho",
            vec![TyChirho::int_chirho(), TyChirho::bool_chirho()],
        )],
    );
    let (_, by_origin_chirho) =
        ctx_chirho.finalize_reference_evidence_chirho(&SubstChirho::empty_chirho());
    assert!(
        by_origin_chirho[&origin_chirho][0]
            .ground_pred_chirho
            .is_none()
    );
}

#[test]
fn ambiguous_ordinary_constructor_names_cannot_authorize_new_dispatch_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    assert!(ground_pred_chirho(&ctx_chirho, "Eq", &maybe_int_chirho(), &[]).is_some());
    ctx_chirho
        .type_families_chirho
        .constructor_origins_mut_chirho()
        .replace_source_module_chirho("OtherChirho", &[("Maybe".to_owned(), true)])
        .unwrap();
    assert!(ground_pred_chirho(&ctx_chirho, "Eq", &maybe_int_chirho(), &[]).is_none());
}

#[test]
fn structural_disagreement_clears_only_the_new_goal_chirho() {
    let record_chirho = |ty_chirho| ReferenceEvidenceChirho {
        class_name_chirho: "Eq".to_owned(),
        ty_key_chirho: Some("Maybe".to_owned()),
        ground_pred_chirho: Some(PredChirho::new_chirho("Eq", ty_chirho)),
    };
    let captures_chirho = std::collections::HashMap::from([(
        1_u32,
        vec![
            vec![record_chirho(TyChirho::int_chirho())],
            vec![record_chirho(TyChirho::bool_chirho())],
        ],
    )]);
    let agreed_chirho = agreeing_reference_goals_chirho(captures_chirho);
    assert_eq!(agreed_chirho[&1][0].ty_key_chirho.as_deref(), Some("Maybe"));
    assert!(agreed_chirho[&1][0].ground_pred_chirho.is_none());
}
