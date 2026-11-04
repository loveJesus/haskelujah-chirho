// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Owned residual equalities and real constructor pattern evidence.

use super::equality_scopes_chirho::EqualityScopeKindChirho;
use super::*;
use haskelujah_ast_chirho::expr_chirho::RhsChirho;

fn opaque_chirho(argument_chirho: TyChirho) -> TyChirho {
    TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho(
            TypeFamilyIdChirho::IntrinsicChirho(IntrinsicTypeFamilyChirho::NotChirho).key_chirho(),
        )),
        Box::new(argument_chirho),
    )
}

#[test]
fn an_inferred_binding_retains_its_generalizable_equality_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let owner_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::BindingChirho);
    let a_chirho = TyVarChirho(900);
    let b_chirho = TyVarChirho(901);
    ctx_chirho.defer_equality_chirho(
        opaque_chirho(TyChirho::VarChirho(a_chirho)),
        opaque_chirho(TyChirho::VarChirho(b_chirho)),
        SpanChirho::DUMMY_CHIRHO,
    );
    let ty_chirho =
        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho));
    let scheme_chirho = ctx_chirho.generalize_chirho(&ty_chirho);
    assert_eq!(scheme_chirho.preds_chirho.len(), 1);
    assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "~");
    ctx_chirho.end_equality_scope_chirho(owner_chirho);
    ctx_chirho.finish_equality_binding_group_chirho(&[owner_chirho], &[ty_chirho]);
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn a_checked_binding_without_skolems_cannot_acquire_an_equality_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let owner_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::CheckedChirho);
    ctx_chirho.defer_equality_chirho(
        opaque_chirho(TyChirho::VarChirho(TyVarChirho(900))),
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        ctx_chirho
            .generalizable_equality_preds_chirho(&[TyVarChirho(900)])
            .is_empty()
    );
    ctx_chirho.end_equality_scope_chirho(owner_chirho);
    ctx_chirho.finish_equality_binding_group_chirho(&[owner_chirho], &[]);
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn a_local_signature_can_use_its_parent_pattern_equality_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let pattern_chirho = ctx_chirho.begin_pattern_implication_chirho();
    let family_chirho = opaque_chirho(TyChirho::ForallVarChirho("aChirho%1".to_owned()));
    ctx_chirho.install_pattern_givens_chirho(vec![PredChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: "~".to_owned(),
        ty_chirho: family_chirho.clone(),
        extra_tys_chirho: vec![TyChirho::int_chirho()],
    }]);
    let child_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::CheckedChirho);
    ctx_chirho.defer_equality_chirho(
        family_chirho.clone(),
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    ctx_chirho.end_equality_scope_chirho(child_chirho);
    ctx_chirho.finish_equality_binding_group_chirho(&[child_chirho], &[]);
    ctx_chirho.end_pattern_implication_chirho(
        pattern_chirho,
        &TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        ctx_chirho.diagnostics_chirho
    );
    let sibling_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::CheckedChirho);
    ctx_chirho.defer_equality_chirho(
        family_chirho,
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    ctx_chirho.end_equality_scope_chirho(sibling_chirho);
    ctx_chirho.finish_equality_binding_group_chirho(&[sibling_chirho], &[]);
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn a_repeated_constructor_result_variable_supplies_a_family_given_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let pattern_chirho = ctx_chirho.begin_pattern_implication_chirho();
    let variable_chirho = TyVarChirho(900);
    let application_chirho = |left_chirho, right_chirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("EqualChirho".to_owned())),
                Box::new(left_chirho),
            )),
            Box::new(right_chirho),
        )
    };
    let result_chirho = application_chirho(
        TyChirho::VarChirho(variable_chirho),
        TyChirho::VarChirho(variable_chirho),
    );
    let family_chirho = opaque_chirho(TyChirho::ForallVarChirho("aChirho%1".to_owned()));
    let scrutinee_chirho = application_chirho(family_chirho.clone(), TyChirho::int_chirho());
    assert!(
        ctx_chirho
            .match_gadt_result_chirho(&result_chirho, &scrutinee_chirho, SpanChirho::DUMMY_CHIRHO)
            .is_some()
    );
    assert_eq!(
        ctx_chirho
            .normalize_equality_type_chirho(&family_chirho, ctx_chirho.active_equality_scope_chirho)
            .unwrap(),
        TyChirho::int_chirho()
    );
    assert!(ctx_chirho.deferred_equalities_chirho.is_empty());
    ctx_chirho.end_pattern_implication_chirho(
        pattern_chirho,
        &TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
}

#[test]
fn pattern_universals_stay_flexible_but_field_only_existentials_are_rigid_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let pattern_chirho = ctx_chirho.begin_pattern_implication_chirho();
    let universal_chirho = TyVarChirho(900);
    let existential_chirho = TyVarChirho(901);
    let result_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("IndexedChirho".to_owned())),
        Box::new(TyChirho::VarChirho(universal_chirho)),
    );
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![universal_chirho, existential_chirho],
        preds_chirho: vec![SchemePredChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Eq".to_owned(),
            ty_chirho: TyChirho::VarChirho(existential_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(TyChirho::VarChirho(existential_chirho), result_chirho),
    };
    let (fields_chirho, result_chirho, givens_chirho) =
        ctx_chirho.instantiate_constructor_pattern_chirho(&scheme_chirho);
    assert!(matches!(fields_chirho[0], TyChirho::ForallVarChirho(_)));
    assert!(!contains_skolem_chirho(&result_chirho));
    assert_eq!(givens_chirho[0].ty_chirho, fields_chirho[0]);
    assert!(ctx_chirho.deferred_preds_chirho.is_empty());
    ctx_chirho.end_pattern_implication_chirho(
        pattern_chirho,
        &fields_chirho[0],
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn constructor_expression_instantiation_stays_flexible_and_emits_wanteds_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let var_chirho = TyVarChirho(900);
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![SchemePredChirho {
            kind_tys_chirho: Vec::new(),
            class_name_chirho: "Eq".to_owned(),
            ty_chirho: TyChirho::VarChirho(var_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            TyChirho::ConChirho("SomeChirho".to_owned()),
        ),
    };
    let instantiated_chirho =
        ctx_chirho.instantiate_chirho(&scheme_chirho, SpanChirho::DUMMY_CHIRHO);
    assert!(!contains_skolem_chirho(&instantiated_chirho));
    assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 1);
}

#[test]
fn an_inferred_local_binding_can_generalize_inside_a_checked_parent_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let parent_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::CheckedChirho);
    let child_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::BindingChirho);
    let var_chirho = TyVarChirho(900);
    ctx_chirho.defer_equality_chirho(
        opaque_chirho(TyChirho::VarChirho(var_chirho)),
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        ctx_chirho
            .generalizable_equality_preds_chirho(&[var_chirho])
            .len(),
        1
    );
    ctx_chirho.end_equality_scope_chirho(child_chirho);
    ctx_chirho
        .finish_equality_binding_group_chirho(&[child_chirho], &[TyChirho::VarChirho(var_chirho)]);
    ctx_chirho.end_equality_scope_chirho(parent_chirho);
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn every_inferred_scc_member_retains_the_shared_equality_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let first_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::BindingChirho);
    let var_chirho = TyVarChirho(900);
    ctx_chirho.defer_equality_chirho(
        opaque_chirho(TyChirho::VarChirho(var_chirho)),
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    ctx_chirho.end_equality_scope_chirho(first_chirho);
    let second_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::BindingChirho);
    ctx_chirho.end_equality_scope_chirho(second_chirho);
    ctx_chirho
        .equality_generalizing_owners_chirho
        .extend([first_chirho, second_chirho]);
    ctx_chirho.active_equality_scope_chirho = first_chirho;
    let first_scheme_chirho = ctx_chirho.generalize_chirho(&TyChirho::VarChirho(var_chirho));
    ctx_chirho.active_equality_scope_chirho = second_chirho;
    let second_scheme_chirho = ctx_chirho.generalize_chirho(&TyChirho::VarChirho(var_chirho));
    assert_eq!(
        first_scheme_chirho.preds_chirho,
        second_scheme_chirho.preds_chirho
    );
    assert_eq!(second_scheme_chirho.preds_chirho[0].class_name_chirho, "~");
    ctx_chirho.active_equality_scope_chirho = 0;
    ctx_chirho.finish_equality_binding_group_chirho(
        &[first_chirho, second_chirho],
        &[
            TyChirho::VarChirho(var_chirho),
            TyChirho::VarChirho(var_chirho),
        ],
    );
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn one_scc_members_missing_predicate_cannot_be_certified_by_free_vars_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let first_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::BindingChirho);
    let var_chirho = TyVarChirho(900);
    ctx_chirho.defer_equality_chirho(
        opaque_chirho(TyChirho::VarChirho(var_chirho)),
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        ctx_chirho
            .generalizable_equality_preds_chirho(&[var_chirho])
            .len(),
        1
    );
    ctx_chirho.end_equality_scope_chirho(first_chirho);
    let second_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::BindingChirho);
    ctx_chirho.end_equality_scope_chirho(second_chirho);
    ctx_chirho.finish_equality_binding_group_chirho(
        &[first_chirho, second_chirho],
        &[
            TyChirho::VarChirho(var_chirho),
            TyChirho::VarChirho(var_chirho),
        ],
    );
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn an_existential_cannot_escape_through_an_older_function_parameter_meta_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let TyChirho::VarChirho(outer_chirho) = ctx_chirho.fresh_var_chirho() else {
        panic!("fresh meta");
    };
    let frame_chirho = ctx_chirho.begin_pattern_implication_chirho();
    let var_chirho = TyVarChirho(900);
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            TyChirho::ConChirho("ExChirho".to_owned()),
        ),
    };
    let (fields_chirho, _, _) = ctx_chirho.instantiate_constructor_pattern_chirho(&scheme_chirho);
    let mut substitution_chirho = SubstChirho::empty_chirho();
    substitution_chirho.insert_chirho(
        outer_chirho,
        TyChirho::fun_chirho(fields_chirho[0].clone(), TyChirho::int_chirho()),
    );
    ctx_chirho.apply_subst_all_chirho(&substitution_chirho);
    ctx_chirho.end_pattern_implication_chirho(
        frame_chirho,
        &TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn local_refinement_does_not_export_existentials_from_an_outer_rigid_type_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let frame_chirho = ctx_chirho.begin_pattern_implication_chirho();
    let var_chirho = TyVarChirho(900);
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            TyChirho::ConChirho("ExChirho".to_owned()),
        ),
    };
    let (fields_chirho, _, _) = ctx_chirho.instantiate_constructor_pattern_chirho(&scheme_chirho);
    let outer_chirho = TyChirho::ForallVarChirho("outerChirho%900".to_owned());
    ctx_chirho.record_equality_rewrites_chirho(vec![(
        outer_chirho.clone(),
        TyChirho::TupleChirho(fields_chirho),
    )]);
    ctx_chirho.end_pattern_implication_chirho(
        frame_chirho,
        &outer_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn constructor_result_tuple_parameters_are_universal_but_family_arguments_are_not_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let frame_chirho = ctx_chirho.begin_pattern_implication_chirho();
    let var_chirho = TyVarChirho(900);
    let result_chirho = |argument_chirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IndexChirho".to_owned())),
            Box::new(argument_chirho),
        )
    };
    let tuple_scheme_chirho = SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            result_chirho(TyChirho::TupleChirho(vec![TyChirho::VarChirho(var_chirho)])),
        ),
    };
    let (tuple_fields_chirho, _, _) =
        ctx_chirho.instantiate_constructor_pattern_chirho(&tuple_scheme_chirho);
    assert!(!contains_skolem_chirho(&tuple_fields_chirho[0]));
    let family_scheme_chirho = SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            result_chirho(opaque_chirho(TyChirho::VarChirho(var_chirho))),
        ),
    };
    let (family_fields_chirho, _, _) =
        ctx_chirho.instantiate_constructor_pattern_chirho(&family_scheme_chirho);
    assert!(contains_skolem_chirho(&family_fields_chirho[0]));
    ctx_chirho.end_pattern_implication_chirho(
        frame_chirho,
        &TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn an_existential_eliminated_by_local_evidence_is_materialized_before_pop_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let TyChirho::VarChirho(outer_chirho) = ctx_chirho.fresh_var_chirho() else {
        panic!("fresh meta");
    };
    let frame_chirho = ctx_chirho.begin_pattern_implication_chirho();
    let var_chirho = TyVarChirho(900);
    let scheme_chirho = SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            TyChirho::ConChirho("ExChirho".to_owned()),
        ),
    };
    let (fields_chirho, _, _) = ctx_chirho.instantiate_constructor_pattern_chirho(&scheme_chirho);
    let mut substitution_chirho = SubstChirho::empty_chirho();
    substitution_chirho.insert_chirho(
        outer_chirho,
        TyChirho::ListChirho(Box::new(fields_chirho[0].clone())),
    );
    ctx_chirho.apply_subst_all_chirho(&substitution_chirho);
    ctx_chirho.install_pattern_givens_chirho(vec![PredChirho {
        kind_tys_chirho: Vec::new(),
        class_name_chirho: "~".to_owned(),
        ty_chirho: fields_chirho[0].clone(),
        extra_tys_chirho: vec![TyChirho::int_chirho()],
    }]);
    let public_chirho = ctx_chirho.end_pattern_implication_chirho(
        frame_chirho,
        &TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    let expected_chirho = TyChirho::ListChirho(Box::new(TyChirho::int_chirho()));
    let accumulated_chirho = public_chirho.compose_chirho(&substitution_chirho);
    assert_eq!(
        accumulated_chirho.apply_ty_chirho(&TyChirho::VarChirho(outer_chirho)),
        expected_chirho
    );
    assert_eq!(
        ctx_chirho
            .normalize_equality_type_chirho(&TyChirho::VarChirho(outer_chirho), 0)
            .unwrap(),
        expected_chirho
    );
    assert!(ctx_chirho.given_rewrites_chirho.is_empty());
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn an_inferred_scc_constraint_cannot_disappear_into_a_checked_siblings_signature_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let first_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::BindingChirho);
    let var_chirho = TyVarChirho(900);
    ctx_chirho.defer_equality_chirho(
        opaque_chirho(TyChirho::VarChirho(var_chirho)),
        TyChirho::int_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    );
    ctx_chirho.end_equality_scope_chirho(first_chirho);
    let second_chirho =
        ctx_chirho.begin_equality_scope_chirho(EqualityScopeKindChirho::CheckedChirho);
    ctx_chirho.end_equality_scope_chirho(second_chirho);
    ctx_chirho
        .equality_generalizing_owners_chirho
        .extend([first_chirho, second_chirho]);
    ctx_chirho.active_equality_scope_chirho = first_chirho;
    assert_eq!(
        ctx_chirho
            .generalizable_equality_preds_chirho(&[var_chirho])
            .len(),
        1
    );
    ctx_chirho.active_equality_scope_chirho = second_chirho;
    assert_eq!(
        ctx_chirho
            .generalizable_equality_preds_chirho(&[var_chirho])
            .len(),
        1
    );
    ctx_chirho.active_equality_scope_chirho = 0;
    ctx_chirho.finish_equality_binding_group_chirho(
        &[first_chirho, second_chirho],
        &[TyChirho::VarChirho(var_chirho)],
    );
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn a_case_alternative_cannot_hide_its_mismatch_behind_an_earlier_given_chirho() {
    use super::test_support_chirho::dummy_name_chirho;
    use haskelujah_ast_chirho::expr_chirho::AltChirho;
    use haskelujah_ast_chirho::lit_chirho::LitChirho;
    for bad_chirho in [false, true] {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let var_chirho = TyVarChirho(900);
        let rigid_chirho = TyChirho::ForallVarChirho("aChirho%900".to_owned());
        let branch_chirho = |argument_chirho| {
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("BranchChirho".to_owned())),
                Box::new(argument_chirho),
            )
        };
        ctx_chirho.env_chirho.bind_chirho(
            "branchChirho".to_owned(),
            SchemeChirho::mono_chirho(branch_chirho(rigid_chirho.clone())),
        );
        ctx_chirho.env_chirho.bind_chirho(
            "valueChirho".to_owned(),
            SchemeChirho::mono_chirho(opaque_chirho(rigid_chirho)),
        );
        for (name_chirho, preds_chirho) in [
            (
                "HasChirho",
                vec![SchemePredChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "~".to_owned(),
                    ty_chirho: opaque_chirho(TyChirho::VarChirho(var_chirho)),
                    extra_tys_chirho: vec![TyChirho::int_chirho()],
                }],
            ),
            ("NoChirho", vec![]),
        ] {
            ctx_chirho.env_chirho.bind_chirho(
                name_chirho.to_owned(),
                SchemeChirho {
                    vars_chirho: vec![var_chirho],
                    preds_chirho,
                    ty_chirho: branch_chirho(TyChirho::VarChirho(var_chirho)),
                },
            );
        }
        let value_chirho = || ExprChirho::VarChirho(dummy_name_chirho("valueChirho"));
        let case_chirho = ExprChirho::CaseChirho {
            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("branchChirho"))),
            alts_chirho: vec![
                AltChirho {
                    pat_chirho: PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("HasChirho"),
                        args_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    rhs_chirho: RhsChirho::UnguardedChirho(value_chirho()),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                AltChirho {
                    pat_chirho: PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("NoChirho"),
                        args_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    rhs_chirho: RhsChirho::UnguardedChirho(if bad_chirho {
                        value_chirho()
                    } else {
                        ExprChirho::LitChirho(LitChirho::IntChirho(
                            0,
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        ctx_chirho.infer_expr_against_expected_chirho(
            &case_chirho,
            &TyChirho::int_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );
        assert_eq!(
            ctx_chirho.diagnostics_chirho.has_errors_chirho(),
            bad_chirho,
            "{:?}",
            ctx_chirho.diagnostics_chirho
        );
    }
}

#[test]
fn resolved_promoted_indices_determine_universals_but_unknown_ones_do_not_chirho() {
    use haskelujah_ast_chirho::promotion_chirho::PromotedKeyChirho;
    for (head_chirho, determined_chirho) in [
        (PromotedKeyChirho::builtin_cons_chirho(), true),
        (
            PromotedKeyChirho::source_chirho("SourceChirho", "NodeChirho"),
            true,
        ),
        (
            PromotedKeyChirho::unknown_chirho("MissingChirho", "NodeChirho"),
            false,
        ),
    ] {
        let mut ctx_chirho = InferCtxChirho::new_chirho();
        let frame_chirho = ctx_chirho.begin_pattern_implication_chirho();
        let var_chirho = TyVarChirho(900);
        let index_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho(head_chirho.as_str_chirho().to_owned())),
            Box::new(TyChirho::VarChirho(var_chirho)),
        );
        let result_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IndexedChirho".to_owned())),
            Box::new(index_chirho),
        );
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![var_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(TyChirho::VarChirho(var_chirho), result_chirho),
        };
        let (fields_chirho, _, _) =
            ctx_chirho.instantiate_constructor_pattern_chirho(&scheme_chirho);
        assert_eq!(
            !contains_skolem_chirho(&fields_chirho[0]),
            determined_chirho
        );
        ctx_chirho.end_pattern_implication_chirho(
            frame_chirho,
            &TyChirho::int_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        );
        assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
    }
}
