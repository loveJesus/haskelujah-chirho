// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Actual effective carriers, pre-entry scope permissions and terminal failure.
//! Workflow: type-identity-chirho/committed-meta-kinds-chirho.md.

use super::*;
use crate::infer_chirho::{InferenceImportContextChirho, infer_elaborated_module_chirho};
use crate::kind_chirho::elaboration_chirho::{
    ElaborationImportsChirho, ElaborationVariableBindingChirho, elaborate_module_chirho,
};
use crate::ty_chirho::{QuantifiedVariablesChirho, SchemeChirho};
use haskelujah_ast_chirho::expr_chirho::{ExprChirho, MatchArmChirho, RhsChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassScopeChirho, NameChirho, RawNameChirho, TypeConstructorScopeChirho,
};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::promotion_chirho::PromotionScopeChirho;

fn module_chirho() -> ModuleChirho {
    ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "EffectiveCommitChirho",
            SpanChirho::DUMMY_CHIRHO,
        )),
        exports_chirho: None,
        imports_chirho: Vec::new(),
        decls_chirho: Vec::new(),
        extensions_chirho: Vec::new(),
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
        deriving_via_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn context_chirho() -> InferCtxChirho {
    let checked_chirho = elaborate_module_chirho(
        &mut module_chirho(),
        ElaborationImportsChirho {
            registry_chirho: &crate::infer_chirho::TypeFamilyEnvChirho::default(),
            type_scope_chirho: &TypeConstructorScopeChirho::default(),
            family_scope_chirho: &crate::infer_chirho::TypeFamilyScopeChirho::default(),
            promotion_scope_chirho: &PromotionScopeChirho::default(),
            builtin_classes_chirho: &BuiltinClassScopeChirho::default(),
            source_unit_chirho: None,
        },
    )
    .expect("the real elaborator supplies the owner and compiler kind catalogue");
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.adopt_elaboration_state_chirho(checked_chirho.state_chirho);
    context_chirho.type_families_chirho = checked_chirho.registry_chirho;
    context_chirho
}

fn variable_chirho(term_chirho: &TyChirho) -> TyVarChirho {
    let TyChirho::VarChirho(variable_chirho) = term_chirho else {
        panic!("actual meta")
    };
    *variable_chirho
}

fn value_representation_chirho(
    context_chirho: &InferCtxChirho,
    term_chirho: &TyChirho,
) -> TyVarChirho {
    let kind_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(term_chirho)]
        .kind_chirho
        .as_deref()
        .expect("actual checked value kind");
    let TyChirho::AppChirho(_, representation_chirho) = kind_chirho else {
        panic!("checked TYPE r")
    };
    variable_chirho(representation_chirho)
}

fn outer_value_chirho(context_chirho: &mut InferCtxChirho) -> TyChirho {
    let outer_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    context_chirho.env_chirho.bind_chirho(
        "outerChirho".into(),
        SchemeChirho::mono_chirho(outer_chirho.clone()),
    );
    outer_chirho
}

fn exhaust_work_chirho(context_chirho: &mut InferCtxChirho) {
    loop {
        let remaining_chirho = context_chirho
            .elaboration_state_chirho
            .lend_equality_budget_chirho(SpanChirho::DUMMY_CHIRHO)
            .expect("original owner allowance")
            .remaining_chirho()
            .0;
        if remaining_chirho <= 1 {
            break;
        }
        context_chirho
            .elaboration_state_chirho
            .charge_chirho(remaining_chirho - 1, 0, SpanChirho::DUMMY_CHIRHO)
            .expect("spend rather than reset the original owner allowance");
    }
}

#[test]
fn the_owned_return_carrier_contains_the_same_kind_solution_as_live_views_chirho() {
    let mut context_chirho = context_chirho();
    let value_chirho = outer_value_chirho(&mut context_chirho);
    let representation_chirho = value_representation_chirho(&context_chirho, &value_chirho);
    let element_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let mut step_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&value_chirho), element_chirho.clone());
    context_chirho
        .apply_subst_all_chirho(&mut step_chirho)
        .expect("closed effective commit");
    let lifted_chirho = crate::ty_chirho::representation_chirho::boxed_representation_chirho(true);
    assert_eq!(
        step_chirho.lookup_chirho(&representation_chirho),
        Some(lifted_chirho)
    );
    assert_eq!(
        step_chirho.lookup_chirho(&representation_chirho),
        context_chirho
            .equality_substitutions_chirho
            .lookup_chirho(&representation_chirho)
    );
    assert_eq!(
        context_chirho
            .env_chirho
            .lookup_chirho("outerChirho")
            .unwrap()
            .ty_chirho,
        element_chirho
    );
    let public_chirho = context_chirho
        .finish_public_substitution_chirho(&step_chirho)
        .expect("bounded outward closure");
    assert_eq!(
        public_chirho.lookup_chirho(&representation_chirho),
        Some(lifted_chirho)
    );
}

#[test]
fn commit_precedes_composition_and_closes_only_explicit_outward_domains_chirho() {
    let mut context_chirho = context_chirho();
    let first_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let first_rep_chirho = value_representation_chirho(&context_chirho, &first_chirho);
    let second_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let second_rep_chirho = value_representation_chirho(&context_chirho, &second_chirho);
    let element_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let mut before_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&first_chirho), second_chirho.clone());
    context_chirho
        .apply_subst_all_chirho(&mut before_chirho)
        .unwrap();
    let mut after_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&second_chirho), element_chirho.clone());
    context_chirho
        .apply_subst_all_chirho(&mut after_chirho)
        .unwrap();
    let combined_chirho = after_chirho.compose_chirho(&before_chirho);
    // An unrelated working equality is not an outward-carrier domain.
    let unrelated_chirho = variable_chirho(&context_chirho.fresh_var_chirho());
    context_chirho
        .equality_substitutions_chirho
        .insert_chirho(unrelated_chirho, TyChirho::bool_chirho());
    let returned_chirho = context_chirho
        .finish_public_substitution_chirho(&combined_chirho)
        .unwrap();
    assert_eq!(
        returned_chirho.lookup_chirho(&variable_chirho(&first_chirho)),
        Some(&element_chirho)
    );
    let lifted_chirho = crate::ty_chirho::representation_chirho::boxed_representation_chirho(true);
    assert_eq!(
        returned_chirho.lookup_chirho(&first_rep_chirho),
        Some(lifted_chirho)
    );
    assert_eq!(
        returned_chirho.lookup_chirho(&second_rep_chirho),
        Some(lifted_chirho)
    );
    assert!(returned_chirho.lookup_chirho(&unrelated_chirho).is_none());
}

#[test]
fn unsigned_match_results_return_their_intrinsic_list_kind_solution_chirho() {
    let mut context_chirho = context_chirho();
    let (returned_chirho, ty_chirho) = context_chirho.infer_matches_chirho(
        &[MatchArmChirho {
            pats_chirho: vec![PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)],
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::ListChirho {
                elements_chirho: Vec::new(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            where_binds_chirho: Vec::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(!context_chirho.has_non_deferrable_failure_chirho());
    assert!(
        matches!(ty_chirho, TyChirho::FunChirho(_, result_chirho, _) if matches!(result_chirho.as_ref(), TyChirho::ListChirho(_)))
    );
    let lifted_chirho = crate::ty_chirho::representation_chirho::boxed_representation_chirho(true);
    let solved_chirho: Vec<_> = returned_chirho
        .iter_chirho()
        .filter(|(_, image_chirho)| *image_chirho == lifted_chirho)
        .collect();
    assert!(
        !solved_chirho.is_empty(),
        "the returned result carrier, not merely live metadata, retains K"
    );
    for (variable_chirho, image_chirho) in solved_chirho {
        assert_eq!(
            context_chirho
                .equality_substitutions_chirho
                .lookup_chirho(variable_chirho),
            Some(image_chirho)
        );
    }
}

#[test]
fn signature_kind_only_escape_is_terminal_and_restores_the_pre_entry_edge_chirho() {
    let mut context_chirho = context_chirho();
    let outer_chirho = outer_value_chirho(&mut context_chirho);
    let representation_chirho = value_representation_chirho(&context_chirho, &outer_chirho);
    let before_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
        .quantifier_chirho
        .clone();
    let private_value_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let private_rep_chirho = value_representation_chirho(&context_chirho, &private_value_chirho);
    context_chirho
        .elaboration_state_chirho
        .mark_universal_chirho(private_rep_chirho, SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    let scheme_chirho = SchemeChirho {
        vars_chirho: QuantifiedVariablesChirho::from_records_chirho(vec![
            context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho[&private_rep_chirho]
                .quantifier_chirho
                .clone(),
        ]),
        preds_chirho: Vec::new(),
        ty_chirho: TyChirho::bool_chirho(),
    };
    let (checked_chirho, public_chirho) = context_chirho.with_rigid_signature_chirho(
        &scheme_chirho,
        SpanChirho::DUMMY_CHIRHO,
        |context_chirho, _expected_chirho| {
            let rigid_chirho = context_chirho
                .skolem_bindings_chirho
                .lookup_chirho(&private_rep_chirho)
                .unwrap()
                .clone();
            let mut step_chirho =
                SubstChirho::singleton_chirho(representation_chirho, rigid_chirho);
            context_chirho
                .apply_subst_all_chirho(&mut step_chirho)
                .unwrap();
            step_chirho
        },
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
    assert!(
        context_chirho
            .publish_signature_solution_chirho(checked_chirho, public_chirho)
            .is_empty_chirho()
    );
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
            .quantifier_chirho,
        before_chirho
    );
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_binding_chirho(variable_chirho(&outer_chirho)),
        ElaborationVariableBindingChirho::MetaChirho
    );
    let result_chirho = context_chirho.finish_chirho();
    assert!(
        result_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert!(result_chirho.subst_chirho.is_empty_chirho());
}

#[test]
fn private_alias_role_poisoning_does_not_remove_a_safe_outer_meta_permission_chirho() {
    let mut context_chirho = context_chirho();
    let outer_chirho = outer_value_chirho(&mut context_chirho);
    let outer_rep_chirho = value_representation_chirho(&context_chirho, &outer_chirho);
    let frame_chirho = context_chirho.begin_pattern_implication_with_roots_chirho(
        std::iter::once(&outer_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    let private_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    context_chirho
        .elaboration_state_chirho
        .mark_universal_chirho(variable_chirho(&private_chirho), SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    let mut private_step_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&private_chirho), outer_chirho.clone());
    context_chirho
        .apply_subst_all_chirho(&mut private_step_chirho)
        .unwrap();
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_binding_chirho(variable_chirho(&outer_chirho)),
        ElaborationVariableBindingChirho::UniversalChirho
    );
    let solved_chirho = context_chirho.end_pattern_implication_chirho(
        frame_chirho,
        &outer_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    let public_chirho = solved_chirho.compose_chirho(&SubstChirho::empty_chirho());
    assert!(!context_chirho.has_non_deferrable_failure_chirho());
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_binding_chirho(variable_chirho(&outer_chirho)),
        ElaborationVariableBindingChirho::MetaChirho
    );
    assert!(
        public_chirho
            .lookup_chirho(&variable_chirho(&private_chirho))
            .is_none()
    );
    assert_eq!(
        public_chirho.lookup_chirho(&outer_rep_chirho),
        Some(crate::ty_chirho::representation_chirho::boxed_representation_chirho(true))
    );
}

#[test]
fn complete_staging_rejects_a_new_private_kind_domain_before_any_publication_chirho() {
    let mut context_chirho = context_chirho();
    let outer_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let captured_chirho = context_chirho
        .elaboration_state_chirho
        .capture_variable_kinds_chirho(
            &[variable_chirho(&outer_chirho)],
            &SubstChirho::empty_chirho(),
            SpanChirho::DUMMY_CHIRHO,
        )
        .unwrap();
    let private_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let private_rep_chirho = value_representation_chirho(&context_chirho, &private_chirho);
    let mut public_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&outer_chirho), private_chirho);
    let before_chirho = public_chirho.clone();
    let supply_chirho = context_chirho.elaboration_state_chirho.next_variable_chirho;
    assert!(
        context_chirho
            .apply_projected_subst_all_chirho(
                &mut public_chirho,
                captured_chirho,
                &HashSet::new(),
                SpanChirho::DUMMY_CHIRHO
            )
            .is_err()
    );
    assert_eq!(public_chirho, before_chirho);
    assert!(
        context_chirho
            .equality_substitutions_chirho
            .lookup_chirho(&private_rep_chirho)
            .is_none()
    );
    assert_eq!(
        context_chirho.elaboration_state_chirho.next_variable_chirho,
        supply_chirho
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
}

#[test]
fn a_constructor_existential_cannot_escape_only_through_an_outer_kind_chirho() {
    let mut context_chirho = context_chirho();
    let outer_chirho = outer_value_chirho(&mut context_chirho);
    let outer_rep_chirho = value_representation_chirho(&context_chirho, &outer_chirho);
    let before_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
        .quantifier_chirho
        .clone();
    let field_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let field_rep_chirho = value_representation_chirho(&context_chirho, &field_chirho);
    let binders_chirho = [field_rep_chirho, variable_chirho(&field_chirho)];
    for binder_chirho in binders_chirho {
        context_chirho
            .elaboration_state_chirho
            .mark_universal_chirho(binder_chirho, SpanChirho::DUMMY_CHIRHO)
            .unwrap();
    }
    let constructor_chirho = SchemeChirho {
        vars_chirho: QuantifiedVariablesChirho::from_records_chirho(
            binders_chirho
                .into_iter()
                .map(|binder_chirho| {
                    context_chirho
                        .elaboration_state_chirho
                        .variable_kinds_chirho[&binder_chirho]
                        .quantifier_chirho
                        .clone()
                })
                .collect(),
        ),
        preds_chirho: Vec::new(),
        ty_chirho: TyChirho::fun_chirho(field_chirho, TyChirho::bool_chirho()),
    };
    let frame_chirho = context_chirho.begin_pattern_implication_with_roots_chirho(
        std::iter::once(&outer_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    let (fields_chirho, result_chirho, _) =
        context_chirho.instantiate_constructor_pattern_chirho(&constructor_chirho);
    let rigid_kind_chirho = context_chirho
        .skolem_bindings_chirho
        .iter_chirho()
        .find_map(|(handle_chirho, image_chirho)| {
            if image_chirho != &fields_chirho[0] {
                return None;
            }
            let kind_chirho = context_chirho
                .skolem_bindings_chirho
                .lookup_image_chirho(handle_chirho)?
                .kind_chirho
                .as_deref()?;
            let TyChirho::AppChirho(_, representation_chirho) = kind_chirho else {
                return None;
            };
            Some(representation_chirho.as_ref().clone())
        })
        .expect("the actual constructor opener retained its existential field kind");
    assert!(matches!(rigid_kind_chirho, TyChirho::ForallVarChirho(_)));
    let mut step_chirho = SubstChirho::singleton_chirho(outer_rep_chirho, rigid_kind_chirho);
    context_chirho
        .apply_subst_all_chirho(&mut step_chirho)
        .unwrap();
    let solved_chirho = context_chirho.end_pattern_implication_chirho(
        frame_chirho,
        &result_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert!(
        solved_chirho
            .compose_chirho(&SubstChirho::empty_chirho())
            .is_empty_chirho()
    );
    assert!(context_chirho.has_non_deferrable_failure_chirho());
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
            .quantifier_chirho,
        before_chirho
    );
}

#[test]
fn captured_universal_unknown_and_absence_never_gain_meta_permission_chirho() {
    for binding_chirho in [
        Some(ElaborationVariableBindingChirho::UniversalChirho),
        Some(ElaborationVariableBindingChirho::UnknownChirho),
        None,
    ] {
        let mut context_chirho = context_chirho();
        let outer_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
        if let Some(binding_chirho) = binding_chirho {
            context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho
                .get_mut(&variable_chirho(&outer_chirho))
                .unwrap()
                .binding_chirho = binding_chirho;
        } else {
            context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho
                .remove(&variable_chirho(&outer_chirho));
        }
        let captured_chirho = context_chirho
            .elaboration_state_chirho
            .capture_variable_kinds_chirho(
                &[variable_chirho(&outer_chirho)],
                &SubstChirho::empty_chirho(),
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap();
        let mut public_chirho = SubstChirho::empty_chirho();
        context_chirho
            .apply_projected_subst_all_chirho(
                &mut public_chirho,
                captured_chirho,
                &HashSet::new(),
                SpanChirho::DUMMY_CHIRHO,
            )
            .unwrap();
        assert_eq!(
            context_chirho
                .elaboration_state_chirho
                .variable_binding_chirho(variable_chirho(&outer_chirho)),
            binding_chirho.unwrap_or(ElaborationVariableBindingChirho::UnknownChirho)
        );
        assert_eq!(
            context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho
                .contains_key(&variable_chirho(&outer_chirho)),
            binding_chirho.is_some()
        );
    }
}

#[test]
fn exhausted_commit_changes_no_carrier_or_metadata_and_reports_terminal_failure_once_chirho() {
    let mut context_chirho = context_chirho();
    let outer_chirho = outer_value_chirho(&mut context_chirho);
    let element_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let mut step_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&outer_chirho), element_chirho);
    let before_chirho = step_chirho.clone();
    let supply_chirho = context_chirho.elaboration_state_chirho.next_variable_chirho;
    let record_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
        .quantifier_chirho
        .clone();
    exhaust_work_chirho(&mut context_chirho);
    for _attempt_chirho in 0..3 {
        assert!(
            context_chirho
                .apply_subst_all_chirho(&mut step_chirho)
                .is_err()
        );
    }
    assert_eq!(step_chirho, before_chirho);
    assert!(
        context_chirho
            .equality_substitutions_chirho
            .is_empty_chirho()
    );
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
            .quantifier_chirho,
        record_chirho
    );
    let expression_chirho = ExprChirho::ListChirho {
        elements_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (returned_chirho, ty_chirho) = context_chirho.infer_expr_chirho(&expression_chirho);
    assert!(returned_chirho.is_empty_chirho());
    assert_eq!(ty_chirho, InferCtxChirho::terminal_commit_type_chirho());
    assert_eq!(
        context_chirho.elaboration_state_chirho.next_variable_chirho,
        supply_chirho
    );
    let result_chirho = context_chirho.finish_chirho();
    assert_eq!(
        result_chirho
            .non_deferrable_diagnostics_chirho
            .diagnostics_chirho()
            .len(),
        1
    );
    assert!(result_chirho.subst_chirho.is_empty_chirho());
}

#[test]
fn exhausted_inner_commit_uses_prepaid_cleanup_without_fresh_recovery_chirho() {
    let mut context_chirho = context_chirho();
    let outer_chirho = outer_value_chirho(&mut context_chirho);
    let before_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
        .quantifier_chirho
        .clone();
    let frame_chirho = context_chirho.begin_pattern_implication_with_roots_chirho(
        std::iter::once(&outer_chirho),
        SpanChirho::DUMMY_CHIRHO,
    );
    let local_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    context_chirho
        .elaboration_state_chirho
        .mark_universal_chirho(variable_chirho(&local_chirho), SpanChirho::DUMMY_CHIRHO)
        .unwrap();
    let mut private_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&local_chirho), outer_chirho.clone());
    context_chirho
        .apply_subst_all_chirho(&mut private_chirho)
        .unwrap();
    exhaust_work_chirho(&mut context_chirho);
    let supply_chirho = context_chirho.elaboration_state_chirho.next_variable_chirho;
    let mut failed_chirho =
        SubstChirho::singleton_chirho(variable_chirho(&outer_chirho), local_chirho);
    assert!(
        context_chirho
            .apply_subst_all_chirho(&mut failed_chirho)
            .is_err()
    );
    context_chirho.end_pattern_implication_chirho(
        frame_chirho,
        &outer_chirho,
        SpanChirho::DUMMY_CHIRHO,
    );
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho[&variable_chirho(&outer_chirho)]
            .quantifier_chirho,
        before_chirho
    );
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_binding_chirho(variable_chirho(&outer_chirho)),
        ElaborationVariableBindingChirho::MetaChirho
    );
    assert_eq!(
        context_chirho.elaboration_state_chirho.next_variable_chirho,
        supply_chirho
    );
    assert_eq!(
        context_chirho
            .finish_chirho()
            .non_deferrable_diagnostics_chirho
            .diagnostics_chirho()
            .len(),
        1
    );
}

#[test]
fn the_public_module_boundary_rejects_terminal_commit_failure_without_recovery_authority_chirho() {
    let mut context_chirho = context_chirho();
    let left_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let right_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    exhaust_work_chirho(&mut context_chirho);
    assert!(
        context_chirho
            .apply_subst_all_chirho(&mut SubstChirho::singleton_chirho(
                variable_chirho(&left_chirho),
                right_chirho
            ))
            .is_err()
    );
    let result_chirho = infer_elaborated_module_chirho(
        &module_chirho(),
        InferenceImportContextChirho {
            unary_variable_instance_heads_covered_chirho: false,
            local_class_prefix_instances_complete_chirho: false,
            imported_types_chirho: &HashMap::new(),
            selected_imported_value_schemes_chirho: Vec::new(),
            imported_type_synonyms_chirho: &HashMap::new(),
            imported_type_families_chirho: &context_chirho.type_families_chirho,
            imported_class_env_chirho: &crate::class_chirho::ClassEnvChirho::new_chirho(),
            unavailable_values_chirho: &HashSet::new(),
            imported_record_field_names_chirho: &HashMap::new(),
            safe_unqualified_imported_type_names_chirho: &HashSet::new(),
            preferred_qualified_type_names_chirho: &HashMap::new(),
            promotion_scope_chirho: None,
            family_scope_chirho: None,
            type_constructor_scope_chirho: None,
            entry_obligation_chirho: None,
        },
        context_chirho.elaboration_state_chirho,
    );
    assert!(
        result_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
    assert!(result_chirho.subst_chirho.is_empty_chirho());
    assert_eq!(
        result_chirho
            .non_deferrable_diagnostics_chirho
            .diagnostics_chirho()
            .len(),
        1
    );
}
