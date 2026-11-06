// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Proof, ownership and terminal-failure controls for the live checked projection.

use super::*;
use haskelujah_ast_chirho::name_chirho::{
    BuiltinClassScopeChirho, NameChirho, RawNameChirho, TypeConstructorScopeChirho,
};
use haskelujah_ast_chirho::promotion_chirho::PromotionScopeChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

fn with_elaborator_chirho(action_chirho: impl FnOnce(&mut TypeElaboratorChirho<'_>)) {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let module_chirho = ModuleChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "MChirho",
            span_chirho,
        )),
        exports_chirho: None,
        imports_chirho: Vec::new(),
        decls_chirho: Vec::new(),
        extensions_chirho: Vec::new(),
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
        deriving_via_chirho: Vec::new(),
        span_chirho,
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
        },
    )
    .unwrap();
    action_chirho(&mut elaborator_chirho);
}

#[test]
fn checked_empty_list_reelaborates_without_a_duplicate_hidden_argument_chirho() {
    with_elaborator_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        elaborator_chirho
            .seed_compiler_header_chirho(CompilerTypeConstructorChirho::ListChirho, span_chirho)
            .unwrap();
        let mut syntax_chirho = TypeChirho::PromotedListChirho {
            elements_chirho: Vec::new(),
            span_chirho,
        };
        let first_chirho = elaborator_chirho
            .infer_type_chirho(&mut syntax_chirho)
            .unwrap();
        assert!(
            matches!(&syntax_chirho, TypeChirho::KindApplicationChirho { written_chirho: false, fun_chirho, .. }
            if matches!(&**fun_chirho, TypeChirho::PromotedConChirho { name_chirho, .. }
                if name_chirho.promotion_chirho().is_some_and(|promotion_chirho| promotion_chirho.key_chirho() == &PromotedKeyChirho::builtin_nil_chirho())))
        );
        let second_chirho = elaborator_chirho
            .infer_type_chirho(&mut syntax_chirho)
            .unwrap();
        assert_eq!(
            elaborator_chirho
                .state_chirho
                .resolve_chirho(&first_chirho.term_chirho, span_chirho)
                .unwrap(),
            elaborator_chirho
                .state_chirho
                .resolve_chirho(&second_chirho.term_chirho, span_chirho)
                .unwrap()
        );
        assert_eq!(
            elaborator_chirho
                .state_chirho
                .resolve_chirho(&first_chirho.kind_chirho, span_chirho)
                .unwrap(),
            elaborator_chirho
                .state_chirho
                .resolve_chirho(&second_chirho.kind_chirho, span_chirho)
                .unwrap()
        );
    });
}

#[test]
fn opaque_type_spelling_never_borrows_registered_representation_proof_chirho() {
    with_elaborator_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        elaborator_chirho.seed_compiler_header_chirho(CompilerTypeConstructorChirho::RepresentationChirho(haskelujah_ast_chirho::name_chirho::CompilerRepresentationChirho::RuntimeTypeChirho), span_chirho).unwrap();
        let syntax_chirho = elaborator_chirho
            .surface_chirho(&TyChirho::ConChirho("TYPE".to_owned()), span_chirho)
            .unwrap();
        let TypeChirho::ConChirho(name_chirho) = syntax_chirho else {
            panic!("opaque spelling remains ordinary");
        };
        assert_eq!(name_chirho.text_chirho(), "TYPE");
        assert!(name_chirho.type_constructor_chirho().is_none());
        assert!(name_chirho.promotion_chirho().is_none());
    });
}

#[test]
fn projection_depth_failure_latches_before_a_later_tiny_projection_chirho() {
    with_elaborator_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        elaborator_chirho.type_depth_chirho = 128;
        assert!(
            elaborator_chirho
                .surface_chirho(&ElaborationStateChirho::type_universe_chirho(), span_chirho)
                .is_err()
        );
        assert!(elaborator_chirho.state_chirho.failure_chirho().is_some());
        elaborator_chirho.type_depth_chirho = 0;
        assert!(
            elaborator_chirho
                .surface_chirho(&ElaborationStateChirho::type_universe_chirho(), span_chirho)
                .is_err()
        );
        assert!(elaborator_chirho.state_chirho.failure_chirho().is_some());
    });
}

#[test]
fn long_promoted_list_fails_before_building_a_deep_cons_spine_chirho() {
    with_elaborator_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        let mut syntax_chirho = TypeChirho::PromotedListChirho {
            elements_chirho: (0..128)
                .map(|_| TypeChirho::WildcardChirho { span_chirho })
                .collect(),
            span_chirho,
        };
        assert!(
            elaborator_chirho
                .infer_type_chirho(&mut syntax_chirho)
                .is_err()
        );
        assert!(elaborator_chirho.state_chirho.failure_chirho().is_some());
        assert!(
            elaborator_chirho
                .state_chirho
                .fresh_chirho(None, span_chirho)
                .is_err()
        );
        assert!(
            matches!(syntax_chirho, TypeChirho::PromotedListChirho { elements_chirho, .. } if elements_chirho.len() == 128)
        );
    });
}

#[test]
fn shared_class_schema_materializes_full_hidden_and_higher_order_kinds_chirho() {
    use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;
    with_elaborator_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        for class_chirho in [
            CompilerClassChirho::KnownNatChirho,
            CompilerClassChirho::TypeableChirho,
            CompilerClassChirho::MonadTransChirho,
        ] {
            elaborator_chirho
                .seed_class_header_chirho(class_chirho, span_chirho)
                .unwrap();
        }
        let schema_chirho = |class_chirho| {
            let key_chirho = TypeConstructorDeclarationChirho::compiler_data_chirho(
                CompilerTypeConstructorChirho::ClassChirho(class_chirho),
            )
            .type_key_chirho();
            elaborator_chirho
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(&key_chirho)
                .unwrap()
                .clone()
        };
        let nat_chirho = schema_chirho(CompilerClassChirho::KnownNatChirho);
        assert_eq!(
            nat_chirho.binders_chirho[0].kind_chirho,
            TyChirho::ConChirho(
                TypeConstructorDeclarationChirho::compiler_data_chirho(
                    CompilerTypeConstructorChirho::NatChirho
                )
                .type_key_chirho()
            )
        );
        let typeable_chirho = schema_chirho(CompilerClassChirho::TypeableChirho);
        assert_eq!(
            (
                typeable_chirho.visible_arity_chirho,
                typeable_chirho.reduction_arity_chirho
            ),
            (1, 2)
        );
        assert_eq!(
            typeable_chirho.binders_chirho[0].visibility_chirho,
            TelescopeVisibilityChirho::InferredChirho
        );
        assert_eq!(
            typeable_chirho.binders_chirho[1].kind_chirho,
            TyChirho::VarChirho(typeable_chirho.binders_chirho[0].variable_chirho)
        );
        let transformer_chirho = schema_chirho(CompilerClassChirho::MonadTransChirho);
        let type_chirho = ElaborationStateChirho::type_universe_chirho();
        let constructor_chirho =
            ElaborationStateChirho::arrow_chirho(type_chirho.clone(), type_chirho);
        assert_eq!(
            transformer_chirho.binders_chirho[0].kind_chirho,
            ElaborationStateChirho::arrow_chirho(constructor_chirho.clone(), constructor_chirho)
        );
        assert_eq!(
            (
                transformer_chirho.visible_arity_chirho,
                transformer_chirho.reduction_arity_chirho
            ),
            (1, 1)
        );
    });
}

#[test]
fn malformed_class_slot_never_becomes_a_fresh_or_type_only_binder_chirho() {
    use haskelujah_ast_chirho::name_chirho::{
        CompilerClassKindAtomChirho, CompilerClassKindChirho,
    };
    with_elaborator_chirho(|elaborator_chirho| {
        let kind_chirho =
            CompilerClassKindChirho::AtomChirho(CompilerClassKindAtomChirho::BoundSlotChirho(0));
        assert!(
            elaborator_chirho
                .compiler_class_kind_chirho(&kind_chirho, 0, 0, SpanChirho::DUMMY_CHIRHO)
                .is_err()
        );
    });
}

#[test]
fn compiler_class_kind_depth_failure_remains_terminal_for_a_later_tiny_kind_chirho() {
    use haskelujah_ast_chirho::name_chirho::{
        CompilerClassGroundKindChirho, CompilerClassKindAtomChirho, CompilerClassKindChirho,
    };
    static TYPE_AST_CHIRHO: CompilerClassKindChirho = CompilerClassKindChirho::AtomChirho(
        CompilerClassKindAtomChirho::GroundChirho(CompilerClassGroundKindChirho::TYPE_CHIRHO),
    );
    static CYCLE_CHIRHO: CompilerClassKindChirho =
        CompilerClassKindChirho::ArrowChirho(&CYCLE_CHIRHO, &TYPE_AST_CHIRHO);
    with_elaborator_chirho(|elaborator_chirho| {
        let span_chirho = SpanChirho::DUMMY_CHIRHO;
        assert!(
            elaborator_chirho
                .compiler_class_kind_chirho(&CYCLE_CHIRHO, 0, 0, span_chirho)
                .is_err()
        );
        assert!(elaborator_chirho.state_chirho.failure_chirho().is_some());
        assert!(
            elaborator_chirho
                .compiler_class_kind_chirho(&TYPE_AST_CHIRHO, 0, 0, span_chirho)
                .is_err()
        );
        assert!(elaborator_chirho.state_chirho.failure_chirho().is_some());
    });
}
