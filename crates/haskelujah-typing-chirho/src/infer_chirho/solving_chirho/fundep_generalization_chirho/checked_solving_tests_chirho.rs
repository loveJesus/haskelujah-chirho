// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Closed solving and failure-state preservation controls.

use super::super::InferCtxChirho;
use crate::class_chirho::{
    ClassDeclChirho, ClassDeclPartsChirho, InstDeclChirho, InstDeclPartsChirho, PredChirho,
    PredPartsChirho,
};
use crate::kind_chirho::elaboration_chirho::{
    ElaborationStateChirho, ElaborationVariableBindingChirho,
};
use crate::subst_chirho::SubstChirho;
use crate::ty_chirho::declarations_chirho::{
    DeclarationBinderChirho, DeclarationSchemeChirho, TelescopeVisibilityChirho,
};
use crate::ty_chirho::{
    QuantifiedVariableChirho, QuantifiedVariablesChirho, SchemeChirho, TyChirho, TyVarChirho,
};
use haskelujah_ast_chirho::name_chirho::{
    ClassIdentityUnknownChirho, CompilerTypeConstructorChirho, InstanceIdentityChirho,
    PredicateClassProofChirho, SourceUnitIdentityChirho, TypeConstructorDeclarationChirho,
};
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashMap;
use std::sync::Arc;

fn proof_chirho(class_chirho: &str) -> PredicateClassProofChirho {
    PredicateClassProofChirho::source_chirho(
        Some(
            SourceUnitIdentityChirho::from_producer_chirho(Arc::from("test-main-chirho")).unwrap(),
        ),
        Arc::from("CheckedFundepChirho"),
        Arc::from(class_chirho),
    )
}

fn context_chirho() -> InferCtxChirho {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.elaboration_state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    context_chirho.elaboration_state_chirho.next_variable_chirho = 1_000;
    // These are explicit checked compiler-producer premises of the small unit
    // fixture, not a replacement for the seven real frontend/native witnesses.
    for (identity_chirho, arity_chirho) in [
        (CompilerTypeConstructorChirho::IntChirho, 0),
        (CompilerTypeConstructorChirho::BoolChirho, 0),
        (CompilerTypeConstructorChirho::ListChirho, 1),
        (CompilerTypeConstructorChirho::MaybeChirho, 1),
        (CompilerTypeConstructorChirho::TupleChirho(2), 2),
        (CompilerTypeConstructorChirho::FunctionChirho, 2),
    ] {
        let declaration_chirho =
            TypeConstructorDeclarationChirho::compiler_data_chirho(identity_chirho);
        context_chirho
            .type_families_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(DeclarationSchemeChirho {
                injectivity_chirho: None,
                association_chirho: Default::default(),
                identity_chirho: Arc::from(identity_chirho.name_chirho()),
                owner_chirho: None,
                role_chirho: declaration_chirho.role_chirho,
                declaration_chirho: Some(declaration_chirho),
                binders_chirho: (0..arity_chirho)
                    .map(|index_chirho| DeclarationBinderChirho {
                        term_specificity_chirho: None,
                        variable_chirho: TyVarChirho(index_chirho),
                        kind_chirho: ElaborationStateChirho::type_universe_chirho(),
                        visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
                    })
                    .collect(),
                local_binders_chirho: Arc::from([]),
                source_variables_chirho: Vec::new(),
                result_kind_chirho: ElaborationStateChirho::type_universe_chirho(),
                synonym_rhs_chirho: None,
                synonym_syntax_chirho: None,
                visible_arity_chirho: arity_chirho as usize,
                reduction_arity_chirho: arity_chirho as usize,
                promoted_chirho: None,
                held_chirho: false,
            })
            .unwrap();
    }
    context_chirho
}

fn meta_chirho(context_chirho: &mut InferCtxChirho, variable_chirho: u32, kind_chirho: TyChirho) {
    context_chirho.note_fresh_quantifier_kind_chirho(
        &TyChirho::VarChirho(TyVarChirho(variable_chirho)),
        &Some(Arc::new(kind_chirho)),
    );
}

fn class_chirho(context_chirho: &mut InferCtxChirho, name_chirho: &str, kind_count_chirho: usize) {
    context_chirho
        .class_env_chirho
        .add_class_chirho(ClassDeclChirho::from_producer_chirho(
            ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: (0..kind_count_chirho)
                    .map(|index_chirho| TyVarChirho(40 + index_chirho as u32))
                    .collect(),
                name_chirho: name_chirho.into(),
                supers_chirho: Vec::new(),
                var_chirho: TyVarChirho(50),
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![TyVarChirho(51)],
                fundeps_chirho: vec![(vec![0], vec![1])],
                defaults_chirho: HashMap::new(),
            },
            proof_chirho(name_chirho),
        ));
}

fn predicate_chirho(
    name_chirho: &str,
    head_chirho: TyChirho,
    output_chirho: TyChirho,
) -> PredChirho {
    PredChirho::from_proof_chirho(
        proof_chirho(name_chirho),
        PredPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: name_chirho.into(),
            ty_chirho: head_chirho,
            extra_tys_chirho: vec![output_chirho],
        },
    )
}

fn instance_chirho(
    context_chirho: &mut InferCtxChirho,
    name_chirho: &str,
    head_chirho: TyChirho,
    output_chirho: TyChirho,
    context_preds_chirho: Vec<PredChirho>,
    variables_chirho: &[u32],
) -> InstDeclChirho {
    let records_chirho = variables_chirho
        .iter()
        .map(|variable_chirho| {
            meta_chirho(
                context_chirho,
                *variable_chirho,
                ElaborationStateChirho::type_universe_chirho(),
            );
            context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho[&TyVarChirho(*variable_chirho)]
                .quantifier_chirho
                .clone()
        })
        .collect();
    InstDeclChirho::from_producer_chirho(
        InstDeclPartsChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: name_chirho.into(),
            head_ty_chirho: head_chirho,
            extra_head_tys_chirho: vec![output_chirho],
            context_chirho: context_preds_chirho,
        },
        proof_chirho(name_chirho),
        InstanceIdentityChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho),
    )
    .with_checked_binders_chirho(
        context_chirho
            .elaboration_state_chirho
            .owner_chirho
            .unwrap(),
        QuantifiedVariablesChirho::from_records_chirho(records_chirho),
    )
}

#[derive(Debug, PartialEq)]
struct FailureStateChirho {
    next_chirho: u32,
    substitutions_chirho: SubstChirho,
    records_chirho: Vec<(
        TyVarChirho,
        QuantifiedVariableChirho,
        ElaborationVariableBindingChirho,
    )>,
    predicates_chirho: Vec<(PredChirho, PredicateClassProofChirho)>,
}

fn snapshot_chirho(context_chirho: &InferCtxChirho) -> FailureStateChirho {
    let mut records_chirho: Vec<_> = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .iter()
        .map(|(variable_chirho, record_chirho)| {
            (
                *variable_chirho,
                record_chirho.quantifier_chirho.clone(),
                record_chirho.binding_chirho,
            )
        })
        .collect();
    records_chirho.sort_by_key(|(variable_chirho, _, _)| *variable_chirho);
    FailureStateChirho {
        next_chirho: context_chirho.elaboration_state_chirho.next_variable_chirho,
        substitutions_chirho: context_chirho.equality_substitutions_chirho.clone(),
        records_chirho,
        predicates_chirho: context_chirho
            .deferred_preds_chirho
            .iter()
            .map(|(predicate_chirho, _)| {
                (
                    predicate_chirho.clone(),
                    predicate_chirho.class_identity_chirho().clone(),
                )
            })
            .collect(),
    }
}

#[test]
fn a_closed_fd_tree_publishes_the_same_delta_to_types_and_wanteds_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "ConvertChirho", 0);
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "ConvertChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
        Vec::new(),
        &[],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    context_chirho.env_chirho.bind_chirho(
        "answerChirho".into(),
        SchemeChirho::mono_chirho(TyChirho::VarChirho(TyVarChirho(100))),
    );
    context_chirho.deferred_preds_chirho.push((
        predicate_chirho(
            "ConvertChirho",
            TyChirho::int_chirho(),
            TyChirho::VarChirho(TyVarChirho(100)),
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let next_chirho = context_chirho.elaboration_state_chirho.next_variable_chirho;
    let count_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .len();
    let delta_chirho = context_chirho.solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho());
    assert_eq!(
        delta_chirho.apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(100))),
        TyChirho::bool_chirho()
    );
    assert_eq!(
        context_chirho
            .env_chirho
            .lookup_chirho("answerChirho")
            .unwrap()
            .ty_chirho,
        TyChirho::bool_chirho()
    );
    assert!(context_chirho.deferred_preds_chirho.is_empty());
    assert_eq!(
        context_chirho.elaboration_state_chirho.next_variable_chirho,
        next_chirho
    );
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho
            .len(),
        count_chirho
    );
}

#[test]
fn an_unclosed_child_discards_its_parents_tentative_improvement_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "ConvertChirho", 0);
    class_chirho(&mut context_chirho, "MissingChirho", 0);
    let child_chirho = predicate_chirho(
        "MissingChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
    );
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "ConvertChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
        vec![child_chirho],
        &[],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    context_chirho.deferred_preds_chirho.push((
        predicate_chirho(
            "ConvertChirho",
            TyChirho::int_chirho(),
            TyChirho::VarChirho(TyVarChirho(100)),
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let before_chirho = snapshot_chirho(&context_chirho);
    assert!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn a_kind_mismatch_cannot_publish_a_well_shaped_type_substitution_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "ConvertChirho", 0);
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "ConvertChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
        Vec::new(),
        &[],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    let type_chirho = ElaborationStateChirho::type_universe_chirho();
    meta_chirho(
        &mut context_chirho,
        100,
        TyChirho::fun_chirho(type_chirho.clone(), type_chirho),
    );
    context_chirho.deferred_preds_chirho.push((
        predicate_chirho(
            "ConvertChirho",
            TyChirho::int_chirho(),
            TyChirho::VarChirho(TyVarChirho(100)),
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let before_chirho = snapshot_chirho(&context_chirho);
    assert!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
}

#[test]
fn repeated_instance_variables_share_one_determinant_match_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "RepeatedChirho", 0);
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(10));
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "RepeatedChirho",
        TyChirho::TupleChirho(vec![variable_chirho.clone(), variable_chirho]),
        TyChirho::bool_chirho(),
        Vec::new(),
        &[10],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    context_chirho.deferred_preds_chirho.push((
        predicate_chirho(
            "RepeatedChirho",
            TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::bool_chirho()]),
            TyChirho::VarChirho(TyVarChirho(100)),
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let before_chirho = snapshot_chirho(&context_chirho);
    assert!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
    context_chirho.deferred_preds_chirho[0].0.ty_chirho =
        TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::int_chirho()]);
    assert_eq!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .lookup_chirho(&TyVarChirho(100)),
        Some(&TyChirho::bool_chirho())
    );
}

#[test]
fn a_same_text_unknown_class_never_borrows_the_checked_class_identity_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "ConvertChirho", 0);
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "ConvertChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
        Vec::new(),
        &[],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    context_chirho.deferred_preds_chirho.push((
        PredChirho::new_multi_chirho(
            "ConvertChirho",
            vec![
                TyChirho::int_chirho(),
                TyChirho::VarChirho(TyVarChirho(100)),
            ],
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let before_chirho = snapshot_chirho(&context_chirho);
    assert!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
}

#[test]
fn an_actual_hidden_kind_meta_is_checked_and_committed_with_the_conclusion_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "BazChirho", 1);
    let mut instance_chirho = instance_chirho(
        &mut context_chirho,
        "BazChirho",
        TyChirho::bool_chirho(),
        TyChirho::bool_chirho(),
        Vec::new(),
        &[],
    );
    instance_chirho
        .kind_tys_chirho
        .push(ElaborationStateChirho::type_universe_chirho());
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    meta_chirho(
        &mut context_chirho,
        101,
        ElaborationStateChirho::type_universe_chirho(),
    );
    let mut wanted_chirho = predicate_chirho(
        "BazChirho",
        TyChirho::bool_chirho(),
        TyChirho::VarChirho(TyVarChirho(100)),
    );
    wanted_chirho
        .kind_tys_chirho
        .push(TyChirho::VarChirho(TyVarChirho(101)));
    context_chirho
        .deferred_preds_chirho
        .push((wanted_chirho, SpanChirho::DUMMY_CHIRHO));
    let delta_chirho = context_chirho.solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho());
    assert_eq!(
        delta_chirho.lookup_chirho(&TyVarChirho(100)),
        Some(&TyChirho::bool_chirho())
    );
    assert_eq!(
        delta_chirho.lookup_chirho(&TyVarChirho(101)),
        Some(&ElaborationStateChirho::type_universe_chirho())
    );
}

#[test]
fn strict_monotype_equations_reject_occurs_cycles_polytypes_and_multiplicity_changes_chirho() {
    let variable_chirho = TyChirho::VarChirho(TyVarChirho(1));
    let mut charge_chirho = |_: usize, _: usize| Some(());
    assert!(
        crate::unify_chirho::unify_monomorphic_bounded_chirho(
            &variable_chirho,
            &TyChirho::ListChirho(Box::new(variable_chirho.clone())),
            &|_| true,
            &mut charge_chirho
        )
        .unwrap()
        .is_none()
    );
    let polytype_chirho = TyChirho::ForallChirho {
        vars_chirho: QuantifiedVariablesChirho::from_records_chirho(Vec::new()),
        body_chirho: Box::new(TyChirho::int_chirho()),
    };
    assert!(
        crate::unify_chirho::unify_monomorphic_bounded_chirho(
            &variable_chirho,
            &polytype_chirho,
            &|_| true,
            &mut charge_chirho
        )
        .unwrap()
        .is_none()
    );
    let linear_chirho = TyChirho::FunChirho(
        Box::new(TyChirho::int_chirho()),
        Box::new(TyChirho::int_chirho()),
        crate::ty_chirho::MultChirho::OneChirho,
    );
    assert!(
        crate::unify_chirho::unify_monomorphic_bounded_chirho(
            &linear_chirho,
            &TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            &|_| true,
            &mut charge_chirho
        )
        .unwrap()
        .is_none()
    );
    assert!(
        crate::unify_chirho::unify_monomorphic_bounded_chirho(
            &TyChirho::int_chirho(),
            &TyChirho::int_chirho(),
            &|_| true,
            &mut |_, _| None,
        )
        .is_err()
    );
}

#[test]
fn a_recursive_context_never_publishes_its_tentative_parent_delta_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "RecursiveChirho", 0);
    let child_chirho = predicate_chirho(
        "RecursiveChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
    );
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "RecursiveChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
        vec![child_chirho],
        &[],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    context_chirho.deferred_preds_chirho.push((
        predicate_chirho(
            "RecursiveChirho",
            TyChirho::int_chirho(),
            TyChirho::VarChirho(TyVarChirho(100)),
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let before_chirho = snapshot_chirho(&context_chirho);
    assert!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
}

#[test]
fn application_arguments_are_checked_even_when_the_projected_result_kind_agrees_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "ConvertChirho", 0);
    // Maybe Maybe projects to Type if its argument is never checked. Its
    // argument actually has kind Type -> Type, so the complete check refuses.
    let bad_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Maybe".into())),
        Box::new(TyChirho::ConChirho("Maybe".into())),
    );
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "ConvertChirho",
        TyChirho::int_chirho(),
        bad_chirho,
        Vec::new(),
        &[],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    context_chirho.deferred_preds_chirho.push((
        predicate_chirho(
            "ConvertChirho",
            TyChirho::int_chirho(),
            TyChirho::VarChirho(TyVarChirho(100)),
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let before_chirho = snapshot_chirho(&context_chirho);
    assert!(
        context_chirho
            .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
}

#[test]
fn universal_and_unknown_variables_cannot_become_fd_binding_domains_chirho() {
    for universal_chirho in [false, true] {
        let mut context_chirho = context_chirho();
        class_chirho(&mut context_chirho, "ConvertChirho", 0);
        let instance_chirho = instance_chirho(
            &mut context_chirho,
            "ConvertChirho",
            TyChirho::int_chirho(),
            TyChirho::bool_chirho(),
            Vec::new(),
            &[],
        );
        context_chirho
            .class_env_chirho
            .add_instance_chirho(instance_chirho);
        if universal_chirho {
            meta_chirho(
                &mut context_chirho,
                100,
                ElaborationStateChirho::type_universe_chirho(),
            );
            context_chirho
                .elaboration_state_chirho
                .mark_universal_chirho(TyVarChirho(100), SpanChirho::DUMMY_CHIRHO)
                .unwrap();
        }
        context_chirho.deferred_preds_chirho.push((
            predicate_chirho(
                "ConvertChirho",
                TyChirho::int_chirho(),
                TyChirho::VarChirho(TyVarChirho(100)),
            ),
            SpanChirho::DUMMY_CHIRHO,
        ));
        let before_chirho = snapshot_chirho(&context_chirho);
        assert!(
            context_chirho
                .solve_deferred_fundeps_chirho(&SubstChirho::empty_chirho())
                .is_empty_chirho()
        );
        assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
    }
}

#[test]
fn legacy_deferred_composition_retains_the_already_published_fd_delta_chirho() {
    let mut context_chirho = context_chirho();
    class_chirho(&mut context_chirho, "ConvertChirho", 0);
    let instance_chirho = instance_chirho(
        &mut context_chirho,
        "ConvertChirho",
        TyChirho::int_chirho(),
        TyChirho::bool_chirho(),
        Vec::new(),
        &[],
    );
    context_chirho
        .class_env_chirho
        .add_instance_chirho(instance_chirho);
    meta_chirho(
        &mut context_chirho,
        100,
        ElaborationStateChirho::type_universe_chirho(),
    );
    context_chirho.deferred_preds_chirho.push((
        predicate_chirho(
            "ConvertChirho",
            TyChirho::int_chirho(),
            TyChirho::VarChirho(TyVarChirho(100)),
        ),
        SpanChirho::DUMMY_CHIRHO,
    ));
    let (fundep_chirho, defaults_chirho) =
        context_chirho.check_deferred_solutions_chirho(&SubstChirho::empty_chirho());
    assert!(defaults_chirho.is_empty_chirho());
    assert_eq!(
        defaults_chirho
            .compose_chirho(&fundep_chirho)
            .apply_ty_chirho(&TyChirho::VarChirho(TyVarChirho(100))),
        TyChirho::bool_chirho()
    );
    assert_eq!(
        context_chirho
            .equality_substitutions_chirho
            .lookup_chirho(&TyVarChirho(100)),
        Some(&TyChirho::bool_chirho())
    );
}
