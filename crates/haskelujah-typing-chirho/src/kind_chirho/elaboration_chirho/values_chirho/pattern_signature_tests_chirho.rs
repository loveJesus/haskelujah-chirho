// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::name_chirho::RawNameChirho;

fn parameter_chirho() -> NameChirho {
    let mut name_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        "aChirho",
        SpanChirho::DUMMY_CHIRHO,
    ));
    name_chirho.set_type_variable_chirho(Some(ScopedTypeVariableIdChirho::QuantifiedChirho {
        declaration_chirho: Arc::from("alias-proof-chirho"),
        index_chirho: 0,
    }));
    name_chirho
}

fn scalar_chirho() -> TypeChirho {
    TypeChirho::ConChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        "Int",
        SpanChirho::DUMMY_CHIRHO,
    )))
}

#[test]
fn substituted_argument_keeps_its_caller_environment_chirho() {
    let catalogue_chirho = DeclarationSchemesChirho::default();
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let caller_chirho = scalar_chirho();
    let parameter_chirho = parameter_chirho();
    let parameter_type_chirho = TypeChirho::VarChirho(parameter_chirho.clone());
    let mut checker_chirho = CheckerChirho {
        catalogue_chirho: &catalogue_chirho,
        state_chirho: &mut state_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let source_chirho = parameter_chirho.type_variable_chirho().unwrap().clone();
    let outer_chirho = Some(Arc::new(FrameChirho {
        parent_chirho: None,
        depth_chirho: 1,
        bindings_chirho: vec![(
            source_chirho.clone(),
            Some(ViewChirho::source_chirho(&caller_chirho, None)),
        )],
    }));
    let argument_chirho = ViewChirho::source_chirho(&parameter_type_chirho, outer_chirho.clone());
    let inner_chirho = Some(Arc::new(FrameChirho {
        parent_chirho: outer_chirho,
        depth_chirho: 2,
        bindings_chirho: vec![(source_chirho, Some(argument_chirho))],
    }));
    let resolved_chirho = checker_chirho
        .resolve_chirho(ViewChirho::source_chirho(
            &parameter_type_chirho,
            inner_chirho,
        ))
        .unwrap();
    assert!(matches!(
        resolved_chirho.node_chirho,
        NodeChirho::SourceChirho(TypeChirho::ConChirho(_))
    ));
    assert!(resolved_chirho.environment_chirho.is_none());
}

#[test]
fn lexical_mask_distinguishes_two_uses_of_one_published_binder_chirho() {
    let catalogue_chirho = DeclarationSchemesChirho::default();
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let name_chirho = parameter_chirho();
    let binder_chirho = AstTyVarChirho::plain_chirho(name_chirho.clone());
    let mut checker_chirho = CheckerChirho {
        catalogue_chirho: &catalogue_chirho,
        state_chirho: &mut state_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let outer_chirho = checker_chirho
        .mask_chirho(std::slice::from_ref(&binder_chirho), None)
        .unwrap();
    let inner_chirho = checker_chirho
        .mask_chirho(std::slice::from_ref(&binder_chirho), outer_chirho.clone())
        .unwrap();
    assert!(
        checker_chirho
            .variable_chirho(&name_chirho, &outer_chirho)
            .unwrap()
            != checker_chirho
                .variable_chirho(&name_chirho, &inner_chirho)
                .unwrap()
    );
}

#[test]
fn constraint_heads_and_multiplicities_scan_the_checked_image_chirho() {
    let catalogue_chirho = DeclarationSchemesChirho::default();
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let name_chirho = parameter_chirho();
    let scalar_chirho = scalar_chirho();
    let environment_chirho = Some(Arc::new(FrameChirho {
        parent_chirho: None,
        depth_chirho: 1,
        bindings_chirho: vec![(
            name_chirho.type_variable_chirho().unwrap().clone(),
            Some(ViewChirho::source_chirho(&scalar_chirho, None)),
        )],
    }));
    let syntax_chirho = TypeChirho::QualChirho {
        context_chirho: vec![ConstraintChirho::ClassChirho {
            class_chirho: name_chirho.clone(),
            args_chirho: vec![scalar_chirho.clone()],
            kind_args_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(TypeChirho::FunChirho {
            arg_chirho: Box::new(scalar_chirho.clone()),
            mult_chirho: Some(MultiplicityChirho::MultVarChirho(name_chirho)),
            result_chirho: Box::new(scalar_chirho.clone()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut checker_chirho = CheckerChirho {
        catalogue_chirho: &catalogue_chirho,
        state_chirho: &mut state_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    // This checks scope/substitution only; the synthetic image grants no class,
    // multiplicity or instance authority in the actual type checker.
    checker_chirho
        .scan_chirho(
            ViewChirho::source_chirho(&syntax_chirho, environment_chirho),
            &HashSet::new(),
        )
        .unwrap();
}

#[test]
fn lexical_depth_exhaustion_latches_on_the_original_state_chirho() {
    let catalogue_chirho = DeclarationSchemesChirho::default();
    let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
    let mut checker_chirho = CheckerChirho {
        catalogue_chirho: &catalogue_chirho,
        state_chirho: &mut state_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut environment_chirho = None;
    for _ in 0..128 {
        environment_chirho = checker_chirho.mask_chirho(&[], environment_chirho).unwrap();
    }
    let error_chirho = checker_chirho
        .mask_chirho(&[], environment_chirho)
        .err()
        .expect("next lexical frame exceeds the existing depth cap");
    assert!(checker_chirho.state_chirho.failure_chirho().is_some());
    let tiny_chirho = scalar_chirho();
    let later_chirho = checker_chirho
        .exposed_chirho(ViewChirho::source_chirho(&tiny_chirho, None))
        .err()
        .expect("later tiny input cannot clear a terminal failure");
    assert_eq!(later_chirho.message_chirho, error_chirho.message_chirho);
}

#[test]
fn only_a_nonopaque_checked_function_head_consumes_a_pattern_argument_chirho() {
    use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;

    let mut checked_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        "->",
        SpanChirho::DUMMY_CHIRHO,
    ));
    checked_chirho.set_type_constructor_chirho(Some(
        TypeConstructorDeclarationChirho::compiler_data_chirho(
            CompilerTypeConstructorChirho::FunctionChirho,
        ),
    ));
    let mut opaque_chirho = checked_chirho.clone();
    // A stale copied proof cannot override a projection's explicit opacity.
    if let NameChirho::RawChirho(raw_chirho) = &mut opaque_chirho {
        raw_chirho.type_constructor_opaque_chirho = true;
    }
    let raw_chirho = NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        "->",
        SpanChirho::DUMMY_CHIRHO,
    ));
    for (head_chirho, accepted_chirho) in [
        (checked_chirho, true),
        (opaque_chirho, false),
        (raw_chirho, false),
    ] {
        let parameter_chirho = parameter_chirho();
        let syntax_chirho = TypeChirho::QualChirho {
            context_chirho: vec![],
            body_chirho: Box::new(TypeChirho::ForallChirho {
                vars_chirho: vec![AstTyVarChirho::plain_chirho(parameter_chirho.clone())],
                body_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(head_chirho)),
                        arg_chirho: Box::new(TypeChirho::VarChirho(parameter_chirho)),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    arg_chirho: Box::new(scalar_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        let catalogue_chirho = DeclarationSchemesChirho::default();
        let mut state_chirho = ElaborationStateChirho::new_chirho().unwrap();
        let mut checker_chirho = CheckerChirho {
            catalogue_chirho: &catalogue_chirho,
            state_chirho: &mut state_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        assert_eq!(
            checker_chirho.check_chirho(&syntax_chirho, 1).is_ok(),
            accepted_chirho
        );
    }
}
