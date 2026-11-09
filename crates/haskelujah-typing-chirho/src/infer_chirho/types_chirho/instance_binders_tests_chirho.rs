// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Owner-qualified instance binders and unpublished candidate openings.
//! Workflow: type-identity-chirho/instance-binders-chirho.md.

use super::*;
use crate::class_chirho::{InstDeclChirho, InstDeclPartsChirho, PredChirho, PredPartsChirho};
use crate::kind_chirho::elaboration_chirho::ElaborationStateChirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, NameChirho, PredicateClassProofChirho, RawNameChirho,
};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;
use haskelujah_span_chirho::SpanChirho;

fn named_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn new_checked_context_chirho() -> InferCtxChirho {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.adopt_elaboration_state_chirho(
        ElaborationStateChirho::new_chirho().expect("checked owner"),
    );
    context_chirho
}

fn owned_instance_chirho() -> (InferCtxChirho, InstDeclChirho) {
    let mut context_chirho = new_checked_context_chirho();
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let owner_chirho = context_chirho
        .elaboration_state_chirho
        .owner_chirho
        .expect("actual owner");
    let kind_chirho = context_chirho
        .elaboration_state_chirho
        .fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        )
        .expect("kind binder");
    let function_chirho = context_chirho
        .elaboration_state_chirho
        .fresh_chirho(
            Some(TyChirho::fun_chirho(
                TyChirho::VarChirho(kind_chirho),
                ElaborationStateChirho::type_universe_chirho(),
            )),
            span_chirho,
        )
        .expect("higher-kinded binder");
    let argument_chirho = context_chirho
        .elaboration_state_chirho
        .fresh_chirho(Some(TyChirho::VarChirho(kind_chirho)), span_chirho)
        .expect("dependent binder");
    let intermediate_chirho = context_chirho
        .elaboration_state_chirho
        .fresh_chirho(
            Some(ElaborationStateChirho::type_universe_chirho()),
            span_chirho,
        )
        .expect("context-only binder");
    for variable_chirho in [function_chirho, argument_chirho, intermediate_chirho] {
        let record_chirho = context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho
            .get_mut(&variable_chirho)
            .expect("actual checked record");
        record_chirho.source_chirho = Some(ScopedTypeVariableIdChirho::InferenceChirho {
            owner_chirho,
            variable_chirho,
        });
        record_chirho.specificity_chirho = Some(TyVarSpecificityChirho::SpecifiedChirho);
    }
    let parts_chirho = InstDeclPartsChirho {
        kind_tys_chirho: vec![TyChirho::VarChirho(kind_chirho)],
        class_name_chirho: "OwnedChirho".into(),
        head_ty_chirho: TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(function_chirho)),
            Box::new(TyChirho::VarChirho(argument_chirho)),
        ),
        extra_head_tys_chirho: vec![TyChirho::VarChirho(argument_chirho)],
        context_chirho: vec![PredChirho::from_proof_chirho(
            PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::MissingUnitChirho),
            PredPartsChirho {
                kind_tys_chirho: vec![TyChirho::VarChirho(kind_chirho)],
                class_name_chirho: "ContextChirho".into(),
                ty_chirho: TyChirho::VarChirho(intermediate_chirho),
                extra_tys_chirho: vec![TyChirho::VarChirho(argument_chirho)],
            },
        )],
    };
    let (owner_chirho, variables_chirho) = context_chirho
        .checked_instance_binders_chirho(&parts_chirho, span_chirho)
        .expect("bounded catalogue")
        .expect("checked records");
    let instance_chirho = InstDeclChirho::legacy_chirho(parts_chirho)
        .with_checked_binders_chirho(owner_chirho, variables_chirho);
    (context_chirho, instance_chirho)
}

fn solver_state_chirho(
    context_chirho: &InferCtxChirho,
) -> (u32, SubstChirho, SubstChirho, Vec<QuantifiedVariableChirho>) {
    let mut records_chirho: Vec<_> = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .values()
        .map(|record_chirho| record_chirho.quantifier_chirho.clone())
        .collect();
    records_chirho.sort_by_key(|record_chirho| record_chirho.variable_chirho);
    (
        context_chirho.elaboration_state_chirho.next_variable_chirho,
        context_chirho
            .elaboration_state_chirho
            .substitution_chirho
            .clone(),
        context_chirho.equality_substitutions_chirho.clone(),
        records_chirho,
    )
}

#[test]
fn repeated_instance_openings_share_each_prefix_but_never_each_others_variables_chirho() {
    let (mut context_chirho, instance_chirho) = owned_instance_chirho();
    let before_chirho = solver_state_chirho(&context_chirho);
    let mut next_chirho = before_chirho.0;
    let (first_chirho, first_prefix_chirho) = context_chirho
        .stage_instance_opening_chirho(&instance_chirho, &mut next_chirho, SpanChirho::DUMMY_CHIRHO)
        .expect("bounded first opening")
        .expect("checked first opening");
    let (second_chirho, _) = context_chirho
        .stage_instance_opening_chirho(&instance_chirho, &mut next_chirho, SpanChirho::DUMMY_CHIRHO)
        .expect("bounded second opening")
        .expect("checked second opening");
    let (_, original_chirho) = instance_chirho
        .checked_binders_chirho()
        .expect("producer catalogue");
    let (_, first_variables_chirho) = first_chirho
        .checked_binders_chirho()
        .expect("first catalogue");
    let (_, second_variables_chirho) = second_chirho
        .checked_binders_chirho()
        .expect("second catalogue");
    assert!(
        first_variables_chirho
            .iter()
            .all(|variable_chirho| !second_variables_chirho.contains(variable_chirho))
    );
    for (original_chirho, first_chirho) in original_chirho
        .records_chirho()
        .iter()
        .zip(first_variables_chirho.records_chirho())
    {
        let image_chirho = first_prefix_chirho
            .lookup_image_chirho(&original_chirho.variable_chirho)
            .expect("one image for each producer variable");
        assert_eq!(
            image_chirho.image_chirho,
            TyChirho::VarChirho(first_chirho.variable_chirho)
        );
        assert_eq!(image_chirho.kind_chirho, first_chirho.kind_chirho);
        assert_eq!(first_chirho.source_chirho, original_chirho.source_chirho);
        assert_eq!(
            first_chirho.specificity_chirho,
            original_chirho.specificity_chirho
        );
        assert_eq!(
            first_chirho.kind_chirho.as_deref(),
            original_chirho
                .kind_chirho
                .as_deref()
                .map(|kind_chirho| first_prefix_chirho.apply_ty_chirho(kind_chirho))
                .as_ref()
        );
    }
    assert_eq!(
        first_chirho.kind_tys_chirho[0],
        first_prefix_chirho.apply_ty_chirho(&instance_chirho.kind_tys_chirho[0])
    );
    assert_eq!(
        first_chirho.head_ty_chirho,
        first_prefix_chirho.apply_ty_chirho(&instance_chirho.head_ty_chirho)
    );
    assert_eq!(
        first_chirho.extra_head_tys_chirho[0],
        first_prefix_chirho.apply_ty_chirho(&instance_chirho.extra_head_tys_chirho[0])
    );
    assert_eq!(
        first_chirho.context_chirho[0].kind_tys_chirho[0],
        first_chirho.kind_tys_chirho[0]
    );
    assert_eq!(
        first_chirho.context_chirho[0].extra_tys_chirho[0],
        first_chirho.extra_head_tys_chirho[0]
    );
    assert_eq!(
        first_chirho.context_chirho[0].ty_chirho,
        first_prefix_chirho.apply_ty_chirho(&instance_chirho.context_chirho[0].ty_chirho)
    );
    assert_eq!(
        first_chirho.context_chirho[0].class_identity_chirho(),
        instance_chirho.context_chirho[0].class_identity_chirho()
    );
    assert_eq!(
        first_chirho.class_identity_chirho(),
        instance_chirho.class_identity_chirho()
    );
    assert_eq!(
        first_chirho.instance_identity_chirho(),
        instance_chirho.instance_identity_chirho()
    );
    assert_eq!(solver_state_chirho(&context_chirho), before_chirho);
    assert!(
        context_chirho
            .class_env_chirho
            .instances_chirho()
            .get_chirho("OwnedChirho")
            .is_none()
    );
}

#[test]
fn instance_registration_joins_owned_ast_ids_and_keeps_context_only_variables_chirho() {
    let (mut context_chirho, instance_chirho) = owned_instance_chirho();
    let (owner_chirho, variables_chirho) = instance_chirho
        .checked_binders_chirho()
        .expect("producer catalogue");
    let surface_chirho = |text_chirho: &str, variable_chirho: TyVarChirho| {
        let mut name_chirho = named_chirho(text_chirho);
        name_chirho.set_type_variable_chirho(Some(ScopedTypeVariableIdChirho::InferenceChirho {
            owner_chirho,
            variable_chirho,
        }));
        TypeChirho::VarChirho(name_chirho)
    };
    let [
        kind_chirho,
        function_chirho,
        argument_chirho,
        intermediate_chirho,
    ] = &**variables_chirho
    else {
        panic!("four live producer binders");
    };
    let declaration_chirho = DeclChirho::InstanceDeclChirho {
        kind_args_chirho: vec![surface_chirho("kChirho", *kind_chirho)],
        context_chirho: vec![
            haskelujah_ast_chirho::ty_chirho::ConstraintChirho::ClassChirho {
                class_chirho: Box::new(named_chirho("ContextChirho")),
                kind_args_chirho: vec![surface_chirho("kChirho", *kind_chirho)],
                args_chirho: vec![
                    surface_chirho("middleChirho", *intermediate_chirho),
                    surface_chirho("aChirho", *argument_chirho),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        class_chirho: named_chirho("OwnedChirho"),
        types_chirho: vec![
            TypeChirho::AppChirho {
                fun_chirho: Box::new(surface_chirho("fChirho", *function_chirho)),
                arg_chirho: Box::new(surface_chirho("aChirho", *argument_chirho)),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            surface_chirho("aChirho", *argument_chirho),
        ],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let next_chirho = context_chirho.elaboration_state_chirho.next_variable_chirho;
    context_chirho.process_instance_decl_chirho(&declaration_chirho);
    let registered_chirho = context_chirho
        .class_env_chirho
        .instances_chirho()
        .get_chirho("OwnedChirho")
        .expect("source instance registered");
    let registered_chirho = registered_chirho.first_chirho().expect("one instance");
    let (registered_owner_chirho, registered_variables_chirho) = registered_chirho
        .checked_binders_chirho()
        .expect("the actual ID join produces a catalogue");
    assert_eq!(registered_owner_chirho, owner_chirho);
    assert_eq!(
        registered_variables_chirho.records_chirho(),
        variables_chirho.records_chirho()
    );
    assert!(registered_variables_chirho.contains(intermediate_chirho));
    assert_eq!(
        context_chirho.elaboration_state_chirho.next_variable_chirho,
        next_chirho
    );
    assert!(!context_chirho.has_non_deferrable_failure_chirho());
}

#[test]
fn legacy_and_foreign_instance_catalogues_publish_no_fresh_state_chirho() {
    let (mut context_chirho, instance_chirho) = owned_instance_chirho();
    let legacy_chirho = InstDeclChirho::legacy_chirho(InstDeclPartsChirho {
        kind_tys_chirho: instance_chirho.kind_tys_chirho.clone(),
        class_name_chirho: instance_chirho.class_name_chirho.clone(),
        head_ty_chirho: instance_chirho.head_ty_chirho.clone(),
        extra_head_tys_chirho: instance_chirho.extra_head_tys_chirho.clone(),
        context_chirho: instance_chirho.context_chirho.clone(),
    });
    let before_chirho = solver_state_chirho(&context_chirho);
    let mut next_chirho = before_chirho.0;
    assert!(
        context_chirho
            .stage_instance_opening_chirho(
                &legacy_chirho,
                &mut next_chirho,
                SpanChirho::DUMMY_CHIRHO
            )
            .expect("unknown is not a resource error")
            .is_none()
    );
    assert_eq!(next_chirho, before_chirho.0);
    assert_eq!(solver_state_chirho(&context_chirho), before_chirho);
    let mut foreign_chirho = new_checked_context_chirho();
    let before_foreign_chirho = solver_state_chirho(&foreign_chirho);
    let mut foreign_next_chirho = before_foreign_chirho.0;
    assert!(
        foreign_chirho
            .stage_instance_opening_chirho(
                &instance_chirho,
                &mut foreign_next_chirho,
                SpanChirho::DUMMY_CHIRHO
            )
            .expect("foreign owner stays untransported")
            .is_none()
    );
    assert_eq!(foreign_next_chirho, before_foreign_chirho.0);
    assert_eq!(solver_state_chirho(&foreign_chirho), before_foreign_chirho);
}

#[test]
fn a_missing_kind_is_not_a_type_default_or_a_checked_instance_catalogue_chirho() {
    let (mut context_chirho, instance_chirho) = owned_instance_chirho();
    let variable_chirho = instance_chirho
        .checked_binders_chirho()
        .expect("producer catalogue")
        .1[0];
    context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .get_mut(&variable_chirho)
        .expect("actual record")
        .kind_chirho = None;
    let parts_chirho = InstDeclPartsChirho {
        kind_tys_chirho: instance_chirho.kind_tys_chirho.clone(),
        class_name_chirho: instance_chirho.class_name_chirho.clone(),
        head_ty_chirho: instance_chirho.head_ty_chirho.clone(),
        extra_head_tys_chirho: instance_chirho.extra_head_tys_chirho.clone(),
        context_chirho: instance_chirho.context_chirho.clone(),
    };
    let before_chirho = solver_state_chirho(&context_chirho);
    assert!(
        context_chirho
            .checked_instance_binders_chirho(&parts_chirho, SpanChirho::DUMMY_CHIRHO)
            .expect("unknown kind")
            .is_none()
    );
    assert_eq!(solver_state_chirho(&context_chirho), before_chirho);
}

#[test]
fn a_foreign_record_origin_cannot_be_rekeyed_by_a_local_catalogue_owner_chirho() {
    let (mut context_chirho, instance_chirho) = owned_instance_chirho();
    let foreign_chirho = new_checked_context_chirho();
    let foreign_owner_chirho = foreign_chirho
        .elaboration_state_chirho
        .owner_chirho
        .expect("different actual owner");
    let (owner_chirho, variables_chirho) = instance_chirho
        .checked_binders_chirho()
        .expect("producer catalogue");
    let foreign_variables_chirho = variables_chirho.map_records_chirho(|record_chirho| {
        let mut record_chirho = record_chirho.clone();
        record_chirho.source_chirho = Some(ScopedTypeVariableIdChirho::InferenceChirho {
            owner_chirho: foreign_owner_chirho,
            variable_chirho: record_chirho.variable_chirho,
        });
        record_chirho
    });
    let foreign_instance_chirho =
        instance_chirho.with_checked_binders_chirho(owner_chirho, foreign_variables_chirho);
    let before_chirho = solver_state_chirho(&context_chirho);
    let mut next_chirho = before_chirho.0;
    assert!(
        context_chirho
            .stage_instance_opening_chirho(
                &foreign_instance_chirho,
                &mut next_chirho,
                SpanChirho::DUMMY_CHIRHO
            )
            .expect("foreign correspondence stays unknown")
            .is_none()
    );
    assert_eq!(next_chirho, before_chirho.0);
    assert_eq!(solver_state_chirho(&context_chirho), before_chirho);
}

#[test]
fn a_partly_allocated_local_opening_is_discarded_on_supply_exhaustion_chirho() {
    let (mut context_chirho, instance_chirho) = owned_instance_chirho();
    let before_chirho = solver_state_chirho(&context_chirho);
    let mut next_chirho = u32::MAX - 1;
    assert!(
        context_chirho
            .stage_instance_opening_chirho(
                &instance_chirho,
                &mut next_chirho,
                SpanChirho::DUMMY_CHIRHO
            )
            .is_err()
    );
    assert_eq!(next_chirho, u32::MAX - 1);
    assert_eq!(solver_state_chirho(&context_chirho), before_chirho);
    assert!(
        context_chirho
            .elaboration_state_chirho
            .failure_chirho()
            .is_some()
    );
    assert!(
        context_chirho
            .class_env_chirho
            .instances_chirho()
            .get_chirho("OwnedChirho")
            .is_none()
    );
}
