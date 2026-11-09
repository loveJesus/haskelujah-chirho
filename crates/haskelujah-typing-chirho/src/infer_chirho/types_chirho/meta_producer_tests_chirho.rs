// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Genuine inference producers and committed, owner-budgeted kind transport.
//! Workflow: type-identity-chirho/committed-meta-kinds-chirho.md.

use super::*;
use crate::kind_chirho::elaboration_chirho::{
    ElaborationStateChirho, ElaborationVariableBindingChirho,
};
use crate::ty_chirho::QuantifiedVariableChirho;
use haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho;
use haskelujah_ast_chirho::expr_chirho::ExprChirho;
use haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho;
use haskelujah_ast_chirho::ty_chirho::inference_variables_chirho::ScopedTypeVariableIdChirho;

fn context_chirho() -> InferCtxChirho {
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.adopt_elaboration_state_chirho(
        ElaborationStateChirho::new_chirho().expect("actual checked owner"),
    );
    context_chirho
}

fn variable_chirho(term_chirho: &TyChirho) -> TyVarChirho {
    let TyChirho::VarChirho(variable_chirho) = term_chirho else {
        panic!("producer returns a genuine variable")
    };
    *variable_chirho
}

fn value_representation_chirho(
    context_chirho: &InferCtxChirho,
    value_chirho: &TyChirho,
) -> TyVarChirho {
    let kind_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(value_chirho)]
        .kind_chirho
        .as_deref()
        .expect("actual value kind");
    let TyChirho::AppChirho(function_chirho, representation_chirho) = kind_chirho else {
        panic!("fresh value producer retains TYPE r")
    };
    assert_eq!(
        function_chirho.as_ref(),
        &TyChirho::ConChirho(
            CompilerRepresentationChirho::RuntimeTypeChirho
                .type_key_chirho()
                .into()
        ),
    );
    variable_chirho(representation_chirho)
}

fn snapshot_chirho(
    context_chirho: &InferCtxChirho,
) -> (
    u32,
    SubstChirho,
    Vec<(
        TyVarChirho,
        QuantifiedVariableChirho,
        ElaborationVariableBindingChirho,
    )>,
) {
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
    (
        context_chirho.elaboration_state_chirho.next_variable_chirho,
        context_chirho
            .elaboration_state_chirho
            .substitution_chirho
            .clone(),
        records_chirho,
    )
}

#[test]
fn an_empty_list_has_its_intrinsic_element_kind_but_a_hole_is_not_upgraded_chirho() {
    let mut context_chirho = context_chirho();
    let hole_chirho = context_chirho.fresh_var_chirho();
    let (_, list_chirho) = context_chirho.infer_expr_chirho(&ExprChirho::ListChirho {
        elements_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
    let TyChirho::ListChirho(element_chirho) = list_chirho else {
        panic!("intrinsic list")
    };
    let record_chirho = &context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(&element_chirho)];
    assert_eq!(
        record_chirho.binding_chirho,
        ElaborationVariableBindingChirho::MetaChirho
    );
    assert_eq!(
        record_chirho.kind_chirho.as_deref(),
        Some(&ElaborationStateChirho::type_universe_chirho())
    );
    assert!(
        !context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho
            .contains_key(&variable_chirho(&hole_chirho))
    );
}

#[test]
fn a_value_result_solves_its_representation_on_the_existing_commit_path_chirho() {
    let mut context_chirho = context_chirho();
    let value_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let representation_chirho = value_representation_chirho(&context_chirho, &value_chirho);
    let element_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let owner_chirho = context_chirho
        .elaboration_state_chirho
        .owner_chirho
        .expect("actual owner");
    let source_chirho = ScopedTypeVariableIdChirho::InferenceChirho {
        owner_chirho,
        variable_chirho: variable_chirho(&value_chirho),
    };
    let value_record_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .get_mut(&variable_chirho(&value_chirho))
        .expect("fresh record");
    value_record_chirho.source_chirho = Some(source_chirho.clone());
    value_record_chirho.specificity_chirho = Some(TyVarSpecificityChirho::InferredChirho);
    context_chirho
        .apply_subst_all_chirho(&mut SubstChirho::singleton_chirho(
            variable_chirho(&value_chirho),
            element_chirho.clone(),
        ))
        .expect("checked substitution commit");
    assert_eq!(
        context_chirho
            .equality_substitutions_chirho
            .lookup_chirho(&representation_chirho),
        Some(crate::ty_chirho::representation_chirho::boxed_representation_chirho(true)),
    );
    for term_chirho in [&value_chirho, &element_chirho] {
        let record_chirho = &context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho[&variable_chirho(term_chirho)];
        assert_eq!(
            record_chirho.binding_chirho,
            ElaborationVariableBindingChirho::MetaChirho
        );
        assert_eq!(
            record_chirho.kind_chirho.as_deref(),
            Some(&ElaborationStateChirho::type_universe_chirho())
        );
    }
    let records_chirho = &context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho;
    assert_eq!(
        records_chirho[&variable_chirho(&value_chirho)].source_chirho,
        Some(source_chirho)
    );
    assert_eq!(
        records_chirho[&variable_chirho(&value_chirho)].specificity_chirho,
        Some(TyVarSpecificityChirho::InferredChirho)
    );
    assert_eq!(
        records_chirho[&variable_chirho(&element_chirho)].source_chirho,
        None
    );
    assert_eq!(
        records_chirho[&variable_chirho(&element_chirho)].specificity_chirho,
        None
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn a_committed_hidden_kind_solution_reaches_dependent_records_chirho() {
    for prior_checked_chirho in [false, true] {
        let mut context_chirho = context_chirho();
        let kind_chirho = context_chirho
            .elaboration_state_chirho
            .fresh_chirho(
                Some(ElaborationStateChirho::type_universe_chirho()),
                SpanChirho::DUMMY_CHIRHO,
            )
            .expect("checked kind meta");
        let value_chirho = context_chirho
            .elaboration_state_chirho
            .fresh_chirho(
                Some(TyChirho::VarChirho(kind_chirho)),
                SpanChirho::DUMMY_CHIRHO,
            )
            .expect("dependent kind meta");
        let mut committed_chirho = if prior_checked_chirho {
            context_chirho
                .elaboration_state_chirho
                .unify_chirho(
                    &TyChirho::VarChirho(kind_chirho),
                    &ElaborationStateChirho::type_universe_chirho(),
                    SpanChirho::DUMMY_CHIRHO,
                )
                .expect("actual earlier checked kind equation");
            let left_chirho =
                context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
            let right_chirho =
                context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
            SubstChirho::singleton_chirho(variable_chirho(&left_chirho), right_chirho)
        } else {
            SubstChirho::singleton_chirho(
                kind_chirho,
                ElaborationStateChirho::type_universe_chirho(),
            )
        };
        let old_links_chirho = context_chirho
            .elaboration_state_chirho
            .substitution_chirho
            .clone();
        context_chirho
            .apply_subst_all_chirho(&mut committed_chirho)
            .expect("checked substitution commit");
        let record_chirho = &context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho[&value_chirho];
        assert_eq!(
            record_chirho.kind_chirho.as_deref(),
            Some(&ElaborationStateChirho::type_universe_chirho())
        );
        assert_eq!(
            record_chirho.binding_chirho,
            ElaborationVariableBindingChirho::MetaChirho
        );
        assert_eq!(
            context_chirho.elaboration_state_chirho.substitution_chirho,
            old_links_chirho
        );
        assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
    }
}

#[test]
fn an_alias_keeps_universal_dominance_and_never_promotes_absence_to_meta_chirho() {
    for barrier_case_chirho in 0..3 {
        let universal_chirho = barrier_case_chirho == 2;
        for reverse_chirho in [false, true] {
            let mut context_chirho = context_chirho();
            let barrier_chirho = if universal_chirho {
                let barrier_chirho =
                    context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
                context_chirho
                    .elaboration_state_chirho
                    .mark_universal_chirho(
                        variable_chirho(&barrier_chirho),
                        SpanChirho::DUMMY_CHIRHO,
                    )
                    .expect("actual universal binder");
                barrier_chirho
            } else if barrier_case_chirho == 1 {
                let barrier_chirho =
                    context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
                let foreign_owner_chirho = ElaborationStateChirho::new_chirho()
                    .expect("actual other owner")
                    .owner_chirho
                    .expect("other checked owner");
                let record_chirho = context_chirho
                    .elaboration_state_chirho
                    .variable_kinds_chirho
                    .get_mut(&variable_chirho(&barrier_chirho))
                    .expect("kind metadata does not prove a binding role");
                record_chirho.binding_chirho = ElaborationVariableBindingChirho::UnknownChirho;
                record_chirho.source_chirho = Some(ScopedTypeVariableIdChirho::InferenceChirho {
                    owner_chirho: foreign_owner_chirho,
                    variable_chirho: variable_chirho(&barrier_chirho),
                });
                barrier_chirho
            } else {
                context_chirho.fresh_var_chirho()
            };
            let source_before_chirho = context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho
                .get(&variable_chirho(&barrier_chirho))
                .and_then(|record_chirho| record_chirho.source_chirho.clone());
            let meta_chirho =
                context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
            let (source_chirho, target_chirho) = if reverse_chirho {
                (&meta_chirho, &barrier_chirho)
            } else {
                (&barrier_chirho, &meta_chirho)
            };
            context_chirho
                .apply_subst_all_chirho(&mut SubstChirho::singleton_chirho(
                    variable_chirho(source_chirho),
                    target_chirho.clone(),
                ))
                .expect("checked substitution commit");
            assert_eq!(
                context_chirho
                    .elaboration_state_chirho
                    .variable_binding_chirho(variable_chirho(target_chirho)),
                if universal_chirho {
                    ElaborationVariableBindingChirho::UniversalChirho
                } else {
                    ElaborationVariableBindingChirho::UnknownChirho
                }
            );
            assert_eq!(
                context_chirho
                    .elaboration_state_chirho
                    .variable_kinds_chirho
                    .get(&variable_chirho(&barrier_chirho))
                    .and_then(|record_chirho| record_chirho.source_chirho.clone()),
                source_before_chirho
            );
            if barrier_case_chirho == 0 && reverse_chirho {
                assert!(
                    !context_chirho
                        .elaboration_state_chirho
                        .variable_kinds_chirho
                        .contains_key(&variable_chirho(&barrier_chirho))
                );
            }
        }
    }
}

#[test]
fn an_unprojected_compound_does_not_grant_kinds_to_its_free_variables_chirho() {
    let mut context_chirho = context_chirho();
    let result_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let unknown_chirho = context_chirho.fresh_var_chirho();
    let argument_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    context_chirho
        .apply_subst_all_chirho(&mut SubstChirho::singleton_chirho(
            variable_chirho(&result_chirho),
            TyChirho::AppChirho(
                Box::new(unknown_chirho.clone()),
                Box::new(argument_chirho.clone()),
            ),
        ))
        .expect("checked substitution commit");
    assert!(
        !context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho
            .contains_key(&variable_chirho(&unknown_chirho))
    );
    let result_record_chirho = &context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(&result_chirho)];
    assert_eq!(
        result_record_chirho.binding_chirho,
        ElaborationVariableBindingChirho::UnknownChirho
    );
    assert!(result_record_chirho.kind_chirho.is_none());
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .variable_binding_chirho(variable_chirho(&argument_chirho)),
        ElaborationVariableBindingChirho::MetaChirho
    );
    assert!(!context_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn transport_exhaustion_publishes_no_partial_records_or_fresh_supply_chirho() {
    let mut context_chirho = context_chirho();
    let left_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let right_chirho = context_chirho.fresh_list_element_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    loop {
        let allowance_chirho = context_chirho
            .elaboration_state_chirho
            .lend_equality_budget_chirho(SpanChirho::DUMMY_CHIRHO)
            .expect("original allowance")
            .remaining_chirho()
            .0;
        if allowance_chirho <= 1 {
            break;
        }
        context_chirho
            .elaboration_state_chirho
            .charge_chirho(allowance_chirho - 1, 0, SpanChirho::DUMMY_CHIRHO)
            .expect("consume the original owner's work");
    }
    let before_chirho = snapshot_chirho(&context_chirho);
    let result_chirho = context_chirho
        .elaboration_state_chirho
        .follow_committed_variable_kinds_chirho(
            &SubstChirho::singleton_chirho(variable_chirho(&left_chirho), right_chirho),
            &context_chirho.type_families_chirho,
            SpanChirho::DUMMY_CHIRHO,
        );
    assert!(result_chirho.is_err());
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
    assert!(
        context_chirho
            .elaboration_state_chirho
            .failure_chirho()
            .is_some()
    );
}

#[test]
fn strict_representation_equations_use_actual_meta_authority_in_either_orientation_chirho() {
    for binding_chirho in [
        ElaborationVariableBindingChirho::MetaChirho,
        ElaborationVariableBindingChirho::UniversalChirho,
        ElaborationVariableBindingChirho::UnknownChirho,
    ] {
        for reverse_chirho in [false, true] {
            let mut context_chirho = context_chirho();
            let value_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
            let representation_chirho = value_representation_chirho(&context_chirho, &value_chirho);
            context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho
                .get_mut(&representation_chirho)
                .expect("actual representation record")
                .binding_chirho = binding_chirho;
            let kind_chirho = context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho[&variable_chirho(&value_chirho)]
                .kind_chirho
                .as_deref()
                .expect("checked TYPE r");
            let lifted_kind_chirho = ElaborationStateChirho::type_universe_chirho();
            let (left_chirho, right_chirho) = if reverse_chirho {
                (&lifted_kind_chirho, kind_chirho)
            } else {
                (kind_chirho, &lifted_kind_chirho)
            };
            let before_chirho = snapshot_chirho(&context_chirho);
            let mut work_chirho = 10_000usize;
            let mut output_chirho = 10_000usize;
            let result_chirho = crate::unify_chirho::unify_monomorphic_bounded_chirho(
                left_chirho,
                right_chirho,
                &|variable_chirho| {
                    context_chirho
                        .elaboration_state_chirho
                        .variable_binding_chirho(variable_chirho)
                        == ElaborationVariableBindingChirho::MetaChirho
                },
                &mut |amount_chirho, allocation_chirho| {
                    work_chirho = work_chirho.checked_sub(amount_chirho)?;
                    output_chirho = output_chirho.checked_sub(allocation_chirho)?;
                    Some(())
                },
            )
            .expect("bounded pure relation");
            assert_eq!(
                result_chirho.is_some(),
                binding_chirho == ElaborationVariableBindingChirho::MetaChirho
            );
            if let Some(solution_chirho) = result_chirho {
                assert_eq!(
                    solution_chirho.lookup_chirho(&representation_chirho),
                    Some(
                        crate::ty_chirho::representation_chirho::boxed_representation_chirho(true)
                    )
                );
            }
            assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
        }
    }
}

#[test]
fn representation_equation_exhaustion_does_not_publish_a_solution_chirho() {
    let mut context_chirho = context_chirho();
    let value_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
    let kind_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho[&variable_chirho(&value_chirho)]
        .kind_chirho
        .as_deref()
        .expect("actual TYPE r");
    let before_chirho = snapshot_chirho(&context_chirho);
    let mut remaining_chirho = 1usize;
    let result_chirho = crate::unify_chirho::unify_monomorphic_bounded_chirho(
        kind_chirho,
        &ElaborationStateChirho::type_universe_chirho(),
        &|variable_chirho| {
            context_chirho
                .elaboration_state_chirho
                .variable_binding_chirho(variable_chirho)
                == ElaborationVariableBindingChirho::MetaChirho
        },
        &mut |work_chirho, output_chirho| {
            remaining_chirho = remaining_chirho.checked_sub(work_chirho.max(output_chirho))?;
            Some(())
        },
    );
    assert!(result_chirho.is_err());
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
}

fn literal_module_chirho(
    extensions_chirho: &[&str],
) -> haskelujah_ast_chirho::module_chirho::ModuleChirho {
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
    haskelujah_ast_chirho::module_chirho::ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "LiteralProducerChirho",
            SpanChirho::DUMMY_CHIRHO,
        )),
        exports_chirho: None,
        imports_chirho: Vec::new(),
        decls_chirho: Vec::new(),
        extensions_chirho: extensions_chirho
            .iter()
            .map(|extension_chirho| (*extension_chirho).to_owned())
            .collect(),
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: Vec::new(),
        deriving_via_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn checked_literal_context_chirho(extensions_chirho: &[&str]) -> InferCtxChirho {
    use crate::kind_chirho::elaboration_chirho::{
        ElaborationImportsChirho, elaborate_module_chirho,
    };
    use haskelujah_ast_chirho::name_chirho::{BuiltinClassScopeChirho, TypeConstructorScopeChirho};
    let mut module_chirho = literal_module_chirho(extensions_chirho);
    let type_scope_chirho = TypeConstructorScopeChirho::default();
    let family_scope_chirho = crate::infer_chirho::TypeFamilyScopeChirho::default();
    let promotion_scope_chirho =
        haskelujah_ast_chirho::promotion_chirho::PromotionScopeChirho::default();
    let checked_chirho = elaborate_module_chirho(
        &mut module_chirho,
        ElaborationImportsChirho {
            registry_chirho: &crate::infer_chirho::TypeFamilyEnvChirho::default(),
            type_scope_chirho: &type_scope_chirho,
            family_scope_chirho: &family_scope_chirho,
            promotion_scope_chirho: &promotion_scope_chirho,
            builtin_classes_chirho: &BuiltinClassScopeChirho::default(),
            source_unit_chirho: None,
        },
    )
    .expect("actual checked owner, import scopes and literal policy");
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.adopt_elaboration_state_chirho(checked_chirho.state_chirho);
    context_chirho.type_families_chirho = checked_chirho.registry_chirho;
    context_chirho.type_constructor_scope_chirho = Some(std::sync::Arc::new(type_scope_chirho));
    context_chirho.family_scope_chirho = Some(std::sync::Arc::new(family_scope_chirho));
    context_chirho.promotion_scope_chirho = Some(std::sync::Arc::new(promotion_scope_chirho));
    context_chirho.infer_module_chirho(&module_chirho);
    assert!(!context_chirho.has_non_deferrable_failure_chirho());
    context_chirho
}

#[test]
fn declared_compiler_value_parameters_instantiate_with_their_actual_type_kind_chirho() {
    for name_chirho in ["+", "-", "*", "print"] {
        let mut context_chirho = checked_literal_context_chirho(&[]);
        let scheme_chirho = context_chirho
            .env_chirho
            .lookup_chirho(name_chirho)
            .unwrap()
            .clone();
        assert_eq!(scheme_chirho.vars_chirho.len(), 1);
        let record_chirho = &scheme_chirho.vars_chirho.records_chirho()[0];
        assert_eq!(
            record_chirho.kind_chirho.as_deref(),
            Some(&ElaborationStateChirho::type_universe_chirho())
        );
        assert!(record_chirho.source_chirho.is_none());
        assert!(record_chirho.specificity_chirho.is_none());
        let (opened_chirho, _predicates_chirho) =
            context_chirho.instantiate_scheme_parts_chirho(&scheme_chirho);
        let TyChirho::FunChirho(parameter_chirho, _, _) = opened_chirho else {
            panic!("actual compiler value signature")
        };
        let parameter_variable_chirho = variable_chirho(&parameter_chirho);
        let instantiated_chirho = &context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho[&parameter_variable_chirho];
        assert_eq!(
            instantiated_chirho.binding_chirho,
            ElaborationVariableBindingChirho::MetaChirho
        );
        assert_eq!(
            instantiated_chirho.kind_chirho.as_deref(),
            Some(&ElaborationStateChirho::type_universe_chirho())
        );
        let result_chirho = context_chirho.fresh_value_meta_chirho(SpanChirho::DUMMY_CHIRHO);
        let mut alias_chirho =
            SubstChirho::singleton_chirho(parameter_variable_chirho, result_chirho.clone());
        context_chirho
            .apply_subst_all_chirho(&mut alias_chirho)
            .unwrap();
        assert_eq!(
            context_chirho
                .elaboration_state_chirho
                .variable_binding_chirho(variable_chirho(&result_chirho)),
            ElaborationVariableBindingChirho::MetaChirho
        );
    }
}

#[test]
fn unknown_value_shadows_and_higher_kind_parameters_are_not_given_type_by_name_chirho() {
    let mut context_chirho = checked_literal_context_chirho(&[]);
    let higher_chirho = context_chirho
        .env_chirho
        .lookup_chirho("return")
        .unwrap()
        .clone();
    let (opened_chirho, _predicates_chirho) =
        context_chirho.instantiate_scheme_parts_chirho(&higher_chirho);
    let TyChirho::FunChirho(_, result_chirho, _) = opened_chirho else {
        panic!("actual return signature")
    };
    let TyChirho::AppChirho(constructor_chirho, _) = result_chirho.as_ref() else {
        panic!("actual higher-kind result")
    };
    let constructor_kind_chirho = context_chirho
        .elaboration_state_chirho
        .variable_kinds_chirho
        .get(&variable_chirho(constructor_chirho))
        .and_then(|record_chirho| record_chirho.kind_chirho.as_deref());
    assert_ne!(
        constructor_kind_chirho,
        Some(&ElaborationStateChirho::type_universe_chirho())
    );
    let unknown_chirho = TyVarChirho(91_000);
    context_chirho.env_chirho.bind_chirho(
        "print".into(),
        crate::ty_chirho::SchemeChirho {
            vars_chirho: vec![unknown_chirho].into(),
            preds_chirho: Vec::new(),
            ty_chirho: TyChirho::VarChirho(unknown_chirho),
        },
    );
    let name_chirho = haskelujah_ast_chirho::name_chirho::NameChirho::RawChirho(
        haskelujah_ast_chirho::name_chirho::RawNameChirho::unqualified_chirho(
            "print",
            SpanChirho::DUMMY_CHIRHO,
        ),
    );
    let (_, value_chirho) = context_chirho.infer_expr_chirho(&ExprChirho::VarChirho(name_chirho));
    assert!(
        !context_chirho
            .elaboration_state_chirho
            .variable_kinds_chirho
            .contains_key(&variable_chirho(&value_chirho))
    );
}

#[test]
fn owned_numeric_literals_use_the_sealed_class_premise_independently_of_origin_and_shadow_chirho() {
    use haskelujah_ast_chirho::lit_chirho::LitChirho;
    use haskelujah_ast_chirho::name_chirho::{
        CompilerClassChirho, PredicateClassIdentityChirho, PredicateClassProofChirho,
    };
    let mut origins_chirho =
        haskelujah_ast_chirho::provenance_chirho::OriginSupplyChirho::new_chirho();
    for origin_chirho in [None, Some(origins_chirho.fresh_chirho())] {
        for (literal_chirho, identity_chirho) in [
            (
                LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, origin_chirho),
                CompilerClassChirho::NumChirho,
            ),
            (
                LitChirho::FloatChirho(1.5, SpanChirho::DUMMY_CHIRHO, origin_chirho),
                CompilerClassChirho::FractionalChirho,
            ),
        ] {
            let mut context_chirho = checked_literal_context_chirho(&[]);
            for name_chirho in ["fromInteger", "fromRational"] {
                context_chirho.env_chirho.bind_chirho(
                    name_chirho.into(),
                    crate::ty_chirho::SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
                );
            }
            let (_, value_chirho) =
                context_chirho.infer_expr_chirho(&ExprChirho::LitChirho(literal_chirho));
            let record_chirho = &context_chirho
                .elaboration_state_chirho
                .variable_kinds_chirho[&variable_chirho(&value_chirho)];
            assert_eq!(
                record_chirho.binding_chirho,
                ElaborationVariableBindingChirho::MetaChirho
            );
            assert_eq!(
                record_chirho.kind_chirho.as_deref(),
                Some(&ElaborationStateChirho::type_universe_chirho())
            );
            assert_eq!(
                context_chirho
                    .deferred_preds_chirho
                    .last()
                    .unwrap()
                    .0
                    .class_identity_chirho(),
                &PredicateClassProofChirho::ProvenChirho(
                    PredicateClassIdentityChirho::CompilerChirho(identity_chirho)
                )
            );
        }
    }
}

#[test]
fn raw_unproven_rebindable_and_hash_literal_routes_retain_unknown_metavariables_chirho() {
    use haskelujah_ast_chirho::lit_chirho::LitChirho;
    let mut contexts_chirho = vec![
        InferCtxChirho::new_chirho(),
        context_chirho(),
        checked_literal_context_chirho(&["RebindableSyntax"]),
        checked_literal_context_chirho(&["MagicHash"]),
    ];
    // Real module preparation can set the translation mode, but an unowned or
    // merely new state still has no checked module-policy premise.
    contexts_chirho[0].infer_module_chirho(&literal_module_chirho(&[]));
    contexts_chirho[1].infer_module_chirho(&literal_module_chirho(&[]));
    for mut context_chirho in contexts_chirho {
        for literal_chirho in [
            LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
            LitChirho::FloatChirho(1.5, SpanChirho::DUMMY_CHIRHO, None),
        ] {
            let (_, value_chirho) =
                context_chirho.infer_expr_chirho(&ExprChirho::LitChirho(literal_chirho));
            assert!(
                !context_chirho
                    .elaboration_state_chirho
                    .variable_kinds_chirho
                    .contains_key(&variable_chirho(&value_chirho))
            );
        }
    }
}

#[test]
fn numeric_literal_exhaustion_allocates_no_recovery_variable_or_evidence_chirho() {
    use haskelujah_ast_chirho::lit_chirho::LitChirho;
    let mut context_chirho = checked_literal_context_chirho(&[]);
    let before_chirho = snapshot_chirho(&context_chirho);
    let deferred_chirho = context_chirho.deferred_preds_chirho.len();
    // Each loan is capped, so drain successive actual loans until the owner has
    // no work left before exercising terminal numeric-literal allocation.
    loop {
        let remaining_chirho = context_chirho
            .elaboration_state_chirho
            .lend_equality_budget_chirho(SpanChirho::DUMMY_CHIRHO)
            .unwrap()
            .remaining_chirho()
            .0;
        if remaining_chirho == 0 {
            break;
        }
        context_chirho
            .elaboration_state_chirho
            .charge_chirho(remaining_chirho, 0, SpanChirho::DUMMY_CHIRHO)
            .unwrap();
    }
    assert_eq!(
        context_chirho
            .elaboration_state_chirho
            .lend_equality_budget_chirho(SpanChirho::DUMMY_CHIRHO)
            .unwrap()
            .remaining_chirho()
            .0,
        0
    );
    let (returned_chirho, value_chirho) = context_chirho.infer_expr_chirho(&ExprChirho::LitChirho(
        LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
    ));
    assert_eq!(snapshot_chirho(&context_chirho), before_chirho);
    assert!(returned_chirho.is_empty_chirho());
    assert_eq!(value_chirho, InferCtxChirho::terminal_commit_type_chirho());
    assert_eq!(context_chirho.deferred_preds_chirho.len(), deferred_chirho);
    assert_eq!(
        context_chirho
            .non_deferrable_diagnostics_chirho
            .diagnostics_chirho()
            .len(),
        1
    );
}
