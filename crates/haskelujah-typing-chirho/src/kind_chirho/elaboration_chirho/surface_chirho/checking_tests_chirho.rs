// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho;

fn with_elaborator_chirho(action_chirho: impl FnOnce(&mut TypeElaboratorChirho<'_>)) {
    let module_chirho = ModuleChirho {
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
