// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! S3 source-instance transport and S4 exact context-index contracts.

use super::*;
use crate::class_chirho::InstDeclChirho;
use crate::ty_chirho::TyVarChirho;
use haskelujah_ast_chirho::lit_chirho::LitChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho;
use haskelujah_span_chirho::SpanChirho;

fn skolem_chirho(name_chirho: &str) -> TyChirho {
    TyChirho::ForallVarChirho(name_chirho.to_owned())
}

fn context_chirho() -> InferCtxChirho {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let mut declaration_chirho = ctx_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("Eq")
        .expect("seeded Eq")
        .as_ref()
        .clone()
        .without_identity_chirho();
    declaration_chirho.name_chirho = "DescribeChirho".to_owned();
    ctx_chirho
        .class_env_chirho
        .add_class_chirho(declaration_chirho);
    ctx_chirho
        .instance_method_skolems_chirho
        .extend(["a%1".to_owned(), "b%2".to_owned()]);
    ctx_chirho
}

#[test]
fn full_predicates_keep_two_contexts_of_one_class_distinct_chirho() {
    let ctx_chirho = context_chirho();
    let a_chirho = TyChirho::ListChirho(Box::new(skolem_chirho("a%1")));
    let b_chirho = TyChirho::ListChirho(Box::new(skolem_chirho("b%2")));
    let predicates_chirho = vec![
        PredChirho::new_chirho("DescribeChirho", a_chirho.clone()),
        PredChirho::new_chirho("DescribeChirho", b_chirho.clone()),
    ];
    assert_eq!(
        ctx_chirho
            .instance_context_key_chirho(
                "DescribeChirho",
                &a_chirho,
                &[],
                Some(&predicates_chirho),
                &SubstChirho::empty_chirho()
            )
            .as_deref(),
        Some("$own:0")
    );
    assert_eq!(
        ctx_chirho
            .instance_context_key_chirho(
                "DescribeChirho",
                &b_chirho,
                &[],
                Some(&predicates_chirho),
                &SubstChirho::empty_chirho()
            )
            .as_deref(),
        Some("$own:1")
    );
}

#[test]
fn class_agreement_is_exact_and_duplicate_exact_contexts_are_ambiguous_chirho() {
    let ctx_chirho = context_chirho();
    let a_chirho = skolem_chirho("a%1");
    let ord_chirho = vec![PredChirho::new_chirho("Ord", a_chirho.clone())];
    assert!(
        ctx_chirho
            .instance_context_key_chirho(
                "Eq",
                &a_chirho,
                &[],
                Some(&ord_chirho),
                &SubstChirho::empty_chirho()
            )
            .is_none()
    );
    let duplicates_chirho = vec![PredChirho::new_chirho("DescribeChirho", a_chirho.clone()); 2];
    assert!(
        ctx_chirho
            .instance_context_key_chirho(
                "DescribeChirho",
                &a_chirho,
                &[],
                Some(&duplicates_chirho),
                &SubstChirho::empty_chirho()
            )
            .is_none()
    );
}

#[test]
fn incomplete_mptc_goals_and_unresolved_metas_prove_no_index_chirho() {
    let mut ctx_chirho = context_chirho();
    let mut declaration_chirho = ctx_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("Eq")
        .expect("seeded Eq")
        .as_ref()
        .clone()
        .without_identity_chirho();
    declaration_chirho.name_chirho = "PairChirho".to_owned();
    declaration_chirho.extra_vars_chirho = vec![TyVarChirho(10_000)];
    ctx_chirho
        .class_env_chirho
        .add_class_chirho(declaration_chirho);
    let a_chirho = skolem_chirho("a%1");
    let b_chirho = skolem_chirho("b%2");
    let predicates_chirho = vec![PredChirho::new_multi_chirho(
        "PairChirho",
        vec![a_chirho.clone(), b_chirho.clone()],
    )];
    assert!(
        ctx_chirho
            .instance_context_key_chirho(
                "PairChirho",
                &a_chirho,
                &[],
                Some(&predicates_chirho),
                &SubstChirho::empty_chirho()
            )
            .is_none()
    );
    assert_eq!(
        ctx_chirho
            .instance_context_key_chirho(
                "PairChirho",
                &a_chirho,
                &[b_chirho],
                Some(&predicates_chirho),
                &SubstChirho::empty_chirho()
            )
            .as_deref(),
        Some("$own:0")
    );
    let meta_chirho = TyChirho::VarChirho(TyVarChirho(3));
    let unresolved_chirho = vec![PredChirho::new_chirho(
        "DescribeChirho",
        meta_chirho.clone(),
    )];
    assert!(
        ctx_chirho
            .instance_context_key_chirho(
                "DescribeChirho",
                &meta_chirho,
                &[],
                Some(&unresolved_chirho),
                &SubstChirho::empty_chirho()
            )
            .is_none()
    );
}

#[test]
fn method_reference_and_literal_captures_retain_their_own_context_chirho() {
    let mut ctx_chirho = context_chirho();
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let origin_chirho = supply_chirho.fresh_chirho();
    let literal_origin_chirho = supply_chirho.fresh_chirho();
    let predicates_chirho = vec![
        PredChirho::new_chirho("Num", skolem_chirho("a%1")),
        PredChirho::new_chirho("Num", skolem_chirho("b%2")),
    ];
    ctx_chirho.with_instance_context_evidence_chirho(&predicates_chirho, |ctx_chirho| {
        let method_chirho = NameChirho::RawChirho(
            RawNameChirho::unqualified_chirho("+", SpanChirho::DUMMY_CHIRHO)
                .with_origin_chirho(origin_chirho),
        );
        ctx_chirho
            .capture_occurrence_predicates_chirho(&method_chirho, &[predicates_chirho[1].clone()]);
        let literal_chirho =
            LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, Some(literal_origin_chirho));
        ctx_chirho.capture_literal_evidence_chirho(&literal_chirho, "Num", &skolem_chirho("a%1"));
    });
    assert!(ctx_chirho.active_instance_context_chirho.is_none());
    let subst_chirho = SubstChirho::empty_chirho();
    let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&subst_chirho);
    assert_eq!(records_chirho[0].ty_key_chirho, "$own:1");
    assert!(records_chirho[0].ground_pred_chirho.is_none());
    let (_, references_chirho) = ctx_chirho.finalize_reference_evidence_chirho(&subst_chirho);
    assert_eq!(
        references_chirho[&origin_chirho][0]
            .ty_key_chirho
            .as_deref(),
        Some("$own:1")
    );
    assert!(
        references_chirho[&origin_chirho][0]
            .ground_pred_chirho
            .is_none()
    );
    let (_, literals_chirho) = ctx_chirho.finalize_literal_evidence_chirho(&subst_chirho);
    assert_eq!(
        literals_chirho[&literal_origin_chirho].ty_key_chirho,
        "$own:0"
    );
}

#[test]
fn captures_match_after_final_substitution_and_context_restores_on_nesting_chirho() {
    let mut ctx_chirho = context_chirho();
    let a_chirho = skolem_chirho("a%1");
    let outer_chirho = vec![PredChirho::new_chirho("DescribeChirho", a_chirho.clone())];
    ctx_chirho.with_instance_context_evidence_chirho(&outer_chirho, |ctx_chirho| {
        let previous_chirho = ctx_chirho.active_instance_context_chirho.clone();
        ctx_chirho.with_instance_context_evidence_chirho(&[], |ctx_chirho| {
            assert!(
                ctx_chirho
                    .active_instance_context_chirho
                    .as_deref()
                    .is_some_and(|context_chirho| context_chirho.is_empty())
            )
        });
        assert_eq!(ctx_chirho.active_instance_context_chirho, previous_chirho);
    });
    let mut subst_chirho = SubstChirho::empty_chirho();
    subst_chirho.insert_chirho(TyVarChirho(4), a_chirho);
    assert_eq!(
        ctx_chirho
            .instance_context_key_chirho(
                "DescribeChirho",
                &TyChirho::VarChirho(TyVarChirho(4)),
                &[],
                Some(&outer_chirho),
                &subst_chirho
            )
            .as_deref(),
        Some("$own:0")
    );
}

#[test]
fn source_instance_transport_keeps_colliding_spans_and_actual_declarations_chirho() {
    let mut ctx_chirho = context_chirho();
    let instances_chirho = [TyChirho::int_chirho(), TyChirho::char_chirho()]
        .into_iter()
        .map(|head_ty_chirho| {
            InstDeclChirho::legacy_chirho(crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "DescribeChirho".to_owned(),
                head_ty_chirho,
                extra_head_tys_chirho: Vec::new(),
                context_chirho: Vec::new(),
            })
        })
        .collect::<Vec<_>>();
    for instance_chirho in &instances_chirho {
        ctx_chirho.local_instances_chirho.push(
            crate::infer_chirho::instance_obligations_chirho::LocalInstanceChirho {
                instance_chirho: std::sync::Arc::new(instance_chirho.clone()),
                nullary_chirho: false,
                written_names_chirho: Vec::new(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
    }
    let transported_chirho = ctx_chirho.finish_chirho().source_instances_chirho;
    assert_eq!(transported_chirho.len(), instances_chirho.len());
    for ((span_chirho, actual_chirho), expected_chirho) in
        transported_chirho.iter().zip(instances_chirho)
    {
        assert_eq!(*span_chirho, SpanChirho::DUMMY_CHIRHO);
        assert_eq!(
            actual_chirho.class_name_chirho,
            expected_chirho.class_name_chirho
        );
        assert_eq!(actual_chirho.head_ty_chirho, expected_chirho.head_ty_chirho);
        assert_eq!(
            actual_chirho.extra_head_tys_chirho,
            expected_chirho.extra_head_tys_chirho
        );
        assert_eq!(actual_chirho.context_chirho, expected_chirho.context_chirho);
    }
}
