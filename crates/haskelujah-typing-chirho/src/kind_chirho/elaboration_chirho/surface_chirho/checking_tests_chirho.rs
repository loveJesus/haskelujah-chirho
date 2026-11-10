// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho;

fn with_elaborator_chirho(action_chirho: impl FnOnce(&mut TypeElaboratorChirho<'_>)) {
    let mut module_chirho = ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "CheckingBoundaryChirho",
            SpanChirho::DUMMY_CHIRHO,
        )),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![],
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    let registry_chirho = TypeFamilyEnvChirho::default();
    let type_scope_chirho = TypeConstructorScopeChirho::default();
    let family_scope_chirho = TypeFamilyScopeChirho::default();
    let promotion_scope_chirho = PromotionScopeChirho::default();
    let builtin_classes_chirho = BuiltinClassScopeChirho::default();
    let mut elaborator_chirho = TypeElaboratorChirho::new_chirho(
        &module_chirho,
        ElaborationImportsChirho {
            registry_chirho: &registry_chirho,
            type_scope_chirho: &type_scope_chirho,
            family_scope_chirho: &family_scope_chirho,
            promotion_scope_chirho: &promotion_scope_chirho,
            builtin_classes_chirho: &builtin_classes_chirho,
            source_unit_chirho: None,
        },
    )
    .unwrap();
    elaborator_chirho
        .predeclare_chirho(&mut module_chirho)
        .expect("the module's real header producer runs before its bodies");
    action_chirho(&mut elaborator_chirho);
}

#[test]
fn final_kind_comparison_preserves_a_closed_universal_scope_chirho() {
    for universal_chirho in [false, true] {
        with_elaborator_chirho(|elaborator_chirho| {
            let span_chirho = SpanChirho::DUMMY_CHIRHO;
            let representation_chirho = elaborator_chirho
                .state_chirho
                .fresh_chirho(
                    Some(TyChirho::ConChirho(
                        CompilerRepresentationChirho::RuntimeRepChirho
                            .type_key_chirho()
                            .into(),
                    )),
                    span_chirho,
                )
                .unwrap();
            if universal_chirho {
                elaborator_chirho
                    .state_chirho
                    .mark_universal_chirho(representation_chirho, span_chirho)
                    .unwrap();
                let scope_chirho = elaborator_chirho
                    .state_chirho
                    .open_rigid_scope_chirho(&[representation_chirho], span_chirho)
                    .unwrap();
                elaborator_chirho
                    .state_chirho
                    .close_rigid_scope_chirho(scope_chirho);
            }
            let result_chirho = ElaboratedTypeChirho {
                term_chirho: TyChirho::ConChirho("CheckedProducerChirho".into()),
                kind_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho(
                        CompilerRepresentationChirho::RuntimeTypeChirho
                            .type_key_chirho()
                            .into(),
                    )),
                    Box::new(TyChirho::VarChirho(representation_chirho)),
                ),
            };
            let checked_chirho = elaborator_chirho.check_inferred_kind_chirho(
                &mut TypeChirho::WildcardChirho { span_chirho },
                result_chirho,
                &ElaborationStateChirho::type_universe_chirho(),
            );
            assert_eq!(checked_chirho.is_err(), universal_chirho);
            // Both success and refusal close only their own temporary scope.
            // Existing evidence may subsequently determine an owned universal.
            elaborator_chirho
                .state_chirho
                .unify_chirho(
                    &TyChirho::VarChirho(representation_chirho),
                    crate::ty_chirho::representation_chirho::boxed_representation_chirho(true),
                    span_chirho,
                )
                .expect("final comparison restores the original rigid scope");
        });
    }
}

#[test]
fn kind_level_and_original_resource_failure_survive_a_failed_check_chirho() {
    with_elaborator_chirho(|elaborator_chirho| {
        let original_chirho = elaborator_chirho
            .state_chirho
            .charge_chirho(usize::MAX, 0, SpanChirho::DUMMY_CHIRHO)
            .unwrap_err();
        let error_chirho = elaborator_chirho
            .infer_kind_syntax_chirho(&mut TypeChirho::WildcardChirho {
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })
            .unwrap_err();
        assert_eq!(error_chirho, original_chirho);
        assert!(!elaborator_chirho.checking_kind_chirho);
        assert_eq!(
            elaborator_chirho.state_chirho.failure_chirho(),
            Some(&error_chirho)
        );
        assert_eq!(
            elaborator_chirho
                .infer_type_chirho(&mut TypeChirho::WildcardChirho {
                    span_chirho: SpanChirho::DUMMY_CHIRHO
                },)
                .unwrap_err(),
            error_chirho
        );
    });
}

#[test]
fn structural_tuple_syntax_carries_its_checked_compiler_telescope_chirho() {
    use haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho;

    for arity_chirho in [0, 2, 3] {
        with_elaborator_chirho(|elaborator_chirho| {
            let span_chirho = SpanChirho::DUMMY_CHIRHO;
            let mut syntax_chirho = TypeChirho::TupleChirho {
                elements_chirho: (0..arity_chirho)
                    .map(|_| TypeChirho::WildcardChirho { span_chirho })
                    .collect(),
                span_chirho,
            };
            let checked_chirho = elaborator_chirho
                .infer_type_chirho(&mut syntax_chirho)
                .expect("valid structural tuple syntax elaborates");
            let mut equations_chirho = Vec::new();
            let kind_chirho =
                crate::kind_chirho::elaboration_chirho::check_monomorphic_kind_chirho(
                    &elaborator_chirho.registry_chirho,
                    elaborator_chirho.state_chirho.owner_chirho.unwrap(),
                    &checked_chirho.term_chirho,
                    &SubstChirho::empty_chirho(),
                    &|variable_chirho| {
                        elaborator_chirho
                            .state_chirho
                            .variable_kinds_chirho
                            .get(&variable_chirho)
                            .and_then(|record_chirho| record_chirho.kind_chirho.as_deref())
                    },
                    &mut equations_chirho,
                    &mut |_, _| Some(()),
                )
                .unwrap();
            assert_eq!(
                kind_chirho,
                Some(ElaborationStateChirho::type_universe_chirho())
            );
            let identity_chirho = if arity_chirho == 0 {
                CompilerTypeConstructorChirho::UnitChirho
            } else {
                CompilerTypeConstructorChirho::TupleChirho(arity_chirho)
            };
            let declaration_chirho = elaborator_chirho
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(identity_chirho.name_chirho())
                .expect("the syntax's checked producer publishes its own declaration");
            assert_eq!(
                declaration_chirho.declaration_chirho,
                Some(TypeConstructorDeclarationChirho::compiler_data_chirho(
                    identity_chirho
                ))
            );
        });
    }
}

#[test]
fn dependent_telescope_arguments_keep_their_checked_prefix_images_chirho() {
    with_elaborator_chirho(|elaborator_chirho| {
        for (kind_argument_chirho, expected_chirho) in [
            (ElaborationStateChirho::type_universe_chirho(), true),
            (TyChirho::bool_chirho(), false),
            (ElaborationStateChirho::constraint_universe_chirho(), false),
        ] {
            let term_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Proxy".into())),
                    Box::new(kind_argument_chirho),
                )),
                Box::new(TyChirho::bool_chirho()),
            );
            let mut equations_chirho = Vec::new();
            let result_chirho =
                crate::kind_chirho::elaboration_chirho::check_monomorphic_kind_chirho(
                    &elaborator_chirho.registry_chirho,
                    elaborator_chirho.state_chirho.owner_chirho.unwrap(),
                    &term_chirho,
                    &SubstChirho::empty_chirho(),
                    &|_| None,
                    &mut equations_chirho,
                    &mut |_, _| Some(()),
                )
                .unwrap();
            assert_eq!(
                result_chirho,
                Some(ElaborationStateChirho::type_universe_chirho())
            );
            let consistent_chirho = equations_chirho.iter().all(|(left_chirho, right_chirho)| {
                crate::unify_chirho::unify_monomorphic_bounded_chirho(
                    left_chirho,
                    right_chirho,
                    &|_| false,
                    &mut |_, _| Some(()),
                )
                .unwrap()
                .is_some()
            });
            assert_eq!(
                consistent_chirho, expected_chirho,
                "the hidden kind controls the visible argument; no producer-local variable becomes bindable"
            );
        }
    });
}
