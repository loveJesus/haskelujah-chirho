// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Direct late-solving and premise-authority controls on the real owned state.

use super::*;
use crate::class_chirho::{PredChirho, PredPartsChirho};
use crate::infer_chirho::{TypeFamilyEnvChirho, TypeFamilyScopeChirho};
use crate::kind_chirho::elaboration_chirho::{ElaborationImportsChirho, elaborate_module_chirho};
use crate::ty_chirho::{QuantifiedVariablesChirho, TyVarChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassScopeChirho, CompilerRepresentationChirho, NameChirho, RawNameChirho,
    TypeConstructorScopeChirho,
};
use haskelujah_ast_chirho::promotion_chirho::PromotionScopeChirho;
use std::collections::HashMap;
use std::sync::Arc;

fn context_chirho() -> InferCtxChirho {
    let mut module_chirho = ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "ParameterEntryChirho",
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
    };
    let type_scope_chirho = TypeConstructorScopeChirho::default();
    let family_scope_chirho = TypeFamilyScopeChirho::default();
    let promotion_scope_chirho = PromotionScopeChirho::default();
    let checked_chirho = elaborate_module_chirho(
        &mut module_chirho,
        ElaborationImportsChirho {
            registry_chirho: &TypeFamilyEnvChirho::default(),
            type_scope_chirho: &type_scope_chirho,
            family_scope_chirho: &family_scope_chirho,
            promotion_scope_chirho: &promotion_scope_chirho,
            builtin_classes_chirho: &BuiltinClassScopeChirho::default(),
        },
    )
    .unwrap();
    let mut context_chirho = InferCtxChirho::new_chirho();
    context_chirho.elaboration_state_chirho = checked_chirho.state_chirho;
    context_chirho.type_families_chirho = checked_chirho.registry_chirho;
    context_chirho.type_constructor_scope_chirho = Some(Arc::new(type_scope_chirho));
    context_chirho.family_scope_chirho = Some(Arc::new(family_scope_chirho));
    context_chirho.promotion_scope_chirho = Some(Arc::new(promotion_scope_chirho));
    context_chirho
}

fn signature_chirho(
    context_chirho: &mut InferCtxChirho,
) -> (SchemeChirho, TyVarChirho, TyVarChirho) {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let state_chirho = &mut context_chirho.elaboration_state_chirho;
    let runtime_rep_chirho = TyChirho::ConChirho(
        CompilerRepresentationChirho::RuntimeRepChirho
            .type_key_chirho()
            .into(),
    );
    let left_chirho = state_chirho
        .fresh_chirho(Some(runtime_rep_chirho.clone()), span_chirho)
        .unwrap();
    let right_chirho = state_chirho
        .fresh_chirho(Some(runtime_rep_chirho), span_chirho)
        .unwrap();
    let value_chirho = state_chirho
        .fresh_chirho(
            Some(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(
                    CompilerRepresentationChirho::RuntimeTypeChirho
                        .type_key_chirho()
                        .into(),
                )),
                Box::new(TyChirho::VarChirho(left_chirho)),
            )),
            span_chirho,
        )
        .unwrap();
    for variable_chirho in [left_chirho, right_chirho, value_chirho] {
        state_chirho
            .mark_universal_chirho(variable_chirho, span_chirho)
            .unwrap();
    }
    let vars_chirho = QuantifiedVariablesChirho::from_records_chirho(
        [left_chirho, right_chirho, value_chirho]
            .into_iter()
            .map(|variable_chirho| {
                state_chirho.variable_kinds_chirho[&variable_chirho]
                    .quantifier_chirho
                    .clone()
            })
            .collect(),
    );
    (
        SchemeChirho {
            vars_chirho,
            preds_chirho: Vec::new(),
            ty_chirho: TyChirho::VarChirho(value_chirho),
        },
        left_chirho,
        right_chirho,
    )
}

fn equality_chirho(left_chirho: TyChirho, right_chirho: TyChirho) -> PredChirho {
    PredChirho::legacy_chirho(PredPartsChirho {
        class_name_chirho: "~".into(),
        kind_tys_chirho: Vec::new(),
        ty_chirho: left_chirho,
        extra_tys_chirho: vec![right_chirho],
    })
}

fn fixed_chirho() -> TyChirho {
    crate::ty_chirho::representation_chirho::boxed_representation_chirho(false).clone()
}

fn with_pattern_chirho(
    context_chirho: &mut InferCtxChirho,
    predicate_chirho: PredChirho,
    body_chirho: impl FnOnce(&mut InferCtxChirho),
) {
    let owner_chirho = context_chirho.begin_equality_scope_chirho(
        crate::infer_chirho::equality_scopes_chirho::EqualityScopeKindChirho::PatternChirho,
    );
    let start_chirho = context_chirho.given_rewrites_chirho.len();
    context_chirho.install_given_equalities_chirho(&[predicate_chirho]);
    context_chirho.record_equality_rewrites_chirho(
        context_chirho.given_rewrites_chirho[start_chirho..].to_vec(),
    );
    body_chirho(context_chirho);
    context_chirho.given_rewrites_chirho.truncate(start_chirho);
    context_chirho.end_equality_scope_chirho(owner_chirho);
}

fn solve_parameter_chirho(
    context_chirho: &mut InferCtxChirho,
    parameter_chirho: &TyChirho,
    expected_chirho: &TyChirho,
) {
    let TyChirho::VarChirho(variable_chirho) = parameter_chirho else {
        panic!("actual fresh parameter meta")
    };
    let mut solution_chirho = SubstChirho::empty_chirho();
    solution_chirho.insert_chirho(*variable_chirho, expected_chirho.clone());
    context_chirho.apply_subst_all_chirho(&solution_chirho);
}

#[test]
fn late_parameter_solution_distinguishes_own_and_enclosing_pattern_evidence_chirho() {
    for enclosing_chirho in [false, true] {
        let mut context_chirho = context_chirho();
        let (signature_chirho, left_chirho, _) = signature_chirho(&mut context_chirho);
        context_chirho.with_rigid_signature_chirho(
            &signature_chirho,
            SpanChirho::DUMMY_CHIRHO,
            |context_chirho, expected_chirho| {
                let parameter_chirho =
                    context_chirho.fresh_value_parameter_chirho(SpanChirho::DUMMY_CHIRHO);
                let entry_chirho = context_chirho.value_parameter_scope_chirho().unwrap();
                let left_chirho = context_chirho
                    .skolem_bindings_chirho
                    .lookup_chirho(&left_chirho)
                    .unwrap()
                    .clone();
                with_pattern_chirho(
                    context_chirho,
                    equality_chirho(left_chirho, fixed_chirho()),
                    |context_chirho| {
                        let inner_chirho = context_chirho.value_parameter_scope_chirho().unwrap();
                        solve_parameter_chirho(context_chirho, &parameter_chirho, expected_chirho);
                        context_chirho.check_solved_value_parameters_chirho(
                            Some(if enclosing_chirho {
                                &inner_chirho
                            } else {
                                &entry_chirho
                            }),
                            &[parameter_chirho],
                            SpanChirho::DUMMY_CHIRHO,
                        );
                    },
                );
            },
        );
        assert_eq!(
            context_chirho
                .non_deferrable_diagnostics_chirho
                .has_errors_chirho(),
            !enclosing_chirho
        );
        assert!(
            context_chirho
                .elaboration_state_chirho
                .failure_chirho()
                .is_none()
        );
    }
}

#[test]
fn written_context_extension_cannot_smuggle_a_live_pattern_premise_chirho() {
    for written_fixed_chirho in [false, true] {
        let mut context_chirho = context_chirho();
        let (signature_chirho, left_chirho, right_chirho) = signature_chirho(&mut context_chirho);
        context_chirho.with_rigid_signature_chirho(
            &signature_chirho,
            SpanChirho::DUMMY_CHIRHO,
            |context_chirho, expected_chirho| {
                let parameter_chirho =
                    context_chirho.fresh_value_parameter_chirho(SpanChirho::DUMMY_CHIRHO);
                let entry_chirho = context_chirho.value_parameter_scope_chirho().unwrap();
                let left_chirho = context_chirho
                    .skolem_bindings_chirho
                    .lookup_chirho(&left_chirho)
                    .unwrap()
                    .clone();
                let right_chirho = context_chirho
                    .skolem_bindings_chirho
                    .lookup_chirho(&right_chirho)
                    .unwrap()
                    .clone();
                with_pattern_chirho(
                    context_chirho,
                    equality_chirho(right_chirho.clone(), fixed_chirho()),
                    |context_chirho| {
                        let predicates_chirho = vec![equality_chirho(
                            left_chirho.clone(),
                            if written_fixed_chirho {
                                fixed_chirho()
                            } else {
                                right_chirho
                            },
                        )];
                        let extended_chirho = context_chirho
                            .extend_value_parameter_scope_chirho(
                                &entry_chirho,
                                &predicates_chirho,
                                SpanChirho::DUMMY_CHIRHO,
                            )
                            .unwrap();
                        context_chirho.with_signature_givens_chirho(
                            predicates_chirho,
                            expected_chirho,
                            |context_chirho| {
                                assert_eq!(
                                    context_chirho.normalize_ty_chirho(&left_chirho),
                                    fixed_chirho()
                                );
                                solve_parameter_chirho(
                                    context_chirho,
                                    &parameter_chirho,
                                    expected_chirho,
                                );
                                context_chirho.check_solved_value_parameters_chirho(
                                    Some(&extended_chirho),
                                    &[parameter_chirho],
                                    SpanChirho::DUMMY_CHIRHO,
                                );
                            },
                        );
                    },
                );
            },
        );
        assert_eq!(
            context_chirho
                .non_deferrable_diagnostics_chirho
                .has_errors_chirho(),
            !written_fixed_chirho
        );
    }
}

#[test]
fn consecutive_written_contexts_share_only_admitted_entry_ancestry_chirho() {
    let mut context_chirho = context_chirho();
    let (signature_chirho, left_chirho, right_chirho) = signature_chirho(&mut context_chirho);
    context_chirho.with_rigid_signature_chirho(
        &signature_chirho,
        SpanChirho::DUMMY_CHIRHO,
        |context_chirho, expected_chirho| {
            let parameter_chirho =
                context_chirho.fresh_value_parameter_chirho(SpanChirho::DUMMY_CHIRHO);
            let entry_chirho = context_chirho.value_parameter_scope_chirho().unwrap();
            let left_chirho = context_chirho
                .skolem_bindings_chirho
                .lookup_chirho(&left_chirho)
                .unwrap()
                .clone();
            let right_chirho = context_chirho
                .skolem_bindings_chirho
                .lookup_chirho(&right_chirho)
                .unwrap()
                .clone();
            let first_chirho = context_chirho
                .extend_value_parameter_scope_chirho(
                    &entry_chirho,
                    &[equality_chirho(left_chirho, right_chirho.clone())],
                    SpanChirho::DUMMY_CHIRHO,
                )
                .unwrap();
            let second_chirho = context_chirho
                .extend_value_parameter_scope_chirho(
                    &first_chirho,
                    &[equality_chirho(right_chirho, fixed_chirho())],
                    SpanChirho::DUMMY_CHIRHO,
                )
                .unwrap();
            solve_parameter_chirho(context_chirho, &parameter_chirho, expected_chirho);
            context_chirho.check_solved_value_parameters_chirho(
                Some(&second_chirho),
                &[parameter_chirho],
                SpanChirho::DUMMY_CHIRHO,
            );
        },
    );
    assert!(
        !context_chirho
            .non_deferrable_diagnostics_chirho
            .has_errors_chirho()
    );
}

#[test]
fn context_extension_preserves_terminal_resource_failure_and_scope_validity_chirho() {
    for resource_chirho in [false, true] {
        let mut context_chirho = context_chirho();
        context_chirho.env_chirho.push_scope_chirho();
        let entry_chirho = context_chirho.value_parameter_scope_chirho().unwrap();
        let failure_chirho = if resource_chirho {
            Some(
                context_chirho
                    .elaboration_state_chirho
                    .charge_chirho(usize::MAX, 0, SpanChirho::DUMMY_CHIRHO)
                    .unwrap_err(),
            )
        } else {
            context_chirho.env_chirho.pop_scope_chirho();
            None
        };
        assert!(
            context_chirho
                .extend_value_parameter_scope_chirho(&entry_chirho, &[], SpanChirho::DUMMY_CHIRHO)
                .is_none()
        );
        assert!(
            context_chirho
                .non_deferrable_diagnostics_chirho
                .has_errors_chirho()
        );
        assert_eq!(
            context_chirho.elaboration_state_chirho.failure_chirho(),
            failure_chirho.as_ref()
        );
    }
}
