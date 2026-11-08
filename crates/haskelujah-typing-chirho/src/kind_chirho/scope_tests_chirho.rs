// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn var_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

fn fun_chirho(argument_chirho: TypeChirho, result_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::FunChirho {
        arg_chirho: Box::new(argument_chirho),
        mult_chirho: None,
        result_chirho: Box::new(result_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn app_chirho(function_chirho: TypeChirho, argument_chirho: TypeChirho) -> TypeChirho {
    TypeChirho::AppChirho {
        fun_chirho: Box::new(function_chirho),
        arg_chirho: Box::new(argument_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn forall_chirho(
    required_chirho: bool,
    binder_chirho: &str,
    body_chirho: TypeChirho,
) -> TypeChirho {
    let vars_chirho = vec![TyVarChirho::plain_chirho(name_chirho(binder_chirho))];
    let body_chirho = Box::new(body_chirho);
    if required_chirho {
        TypeChirho::RequiredForallChirho {
            vars_chirho,
            body_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    } else {
        TypeChirho::ForallChirho {
            vars_chirho,
            body_chirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }
}

#[test]
fn higher_kinded_outer_variable_survives_both_forall_forms_chirho() {
    for required_chirho in [false, true] {
        for outer_first_chirho in [false, true] {
            let mut ctx_chirho =
                KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
            let outer_use_chirho = app_chirho(
                var_chirho("fChirho"),
                TypeChirho::ConChirho(name_chirho("Int")),
            );
            let inner_chirho = forall_chirho(
                required_chirho,
                "fChirho",
                fun_chirho(var_chirho("fChirho"), var_chirho("fChirho")),
            );
            let ty_chirho = if outer_first_chirho {
                fun_chirho(
                    outer_use_chirho.clone(),
                    fun_chirho(inner_chirho, outer_use_chirho),
                )
            } else {
                fun_chirho(
                    inner_chirho,
                    fun_chirho(outer_use_chirho.clone(), outer_use_chirho),
                )
            };
            assert_eq!(
                ctx_chirho.infer_type_kind_chirho(&ty_chirho),
                KindChirho::StarChirho
            );
            assert!(
                !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
                "{:?}",
                ctx_chirho.diagnostics_chirho
            );
            assert_eq!(
                ctx_chirho
                    .subst_chirho
                    .apply_chirho(ctx_chirho.env_chirho.lookup_chirho("fChirho").unwrap()),
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
            );
        }
    }
}

#[test]
fn free_type_variable_first_found_inside_forall_stays_shared_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    let inner_chirho = forall_chirho(
        false,
        "localChirho",
        fun_chirho(var_chirho("freeChirho"), var_chirho("localChirho")),
    );
    ctx_chirho.infer_type_kind_chirho(&inner_chirho);
    assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(ctx_chirho.env_chirho.lookup_chirho("localChirho").is_none());
    let bad_chirho = app_chirho(
        var_chirho("freeChirho"),
        TypeChirho::ConChirho(name_chirho("Int")),
    );
    ctx_chirho.infer_type_kind_chirho(&bad_chirho);
    assert!(
        ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "a free variable fixed to Type inside the forall cannot later become a type constructor"
    );
}

#[test]
fn kind_interpretation_shadows_the_identity_cache_not_just_the_type_environment_chirho() {
    for required_chirho in [false, true] {
        let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
        let outer_chirho = ctx_chirho.type_to_kind_chirho(&var_chirho("kChirho"));
        let shadow_chirho = forall_chirho(required_chirho, "kChirho", var_chirho("kChirho"));
        let inner_chirho = ctx_chirho.type_to_kind_chirho(&shadow_chirho);
        assert_ne!(
            inner_chirho, outer_chirho,
            "the inner kind binder is not the outer kind variable"
        );
        assert_eq!(
            ctx_chirho.type_to_kind_chirho(&var_chirho("kChirho")),
            outer_chirho
        );
        assert!(ctx_chirho.env_chirho.lookup_chirho("kChirho").is_none());
    }
}

#[test]
fn kind_interpretation_preserves_new_free_names_but_discards_local_binders_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    let ty_chirho = forall_chirho(
        false,
        "boundChirho",
        fun_chirho(var_chirho("freeChirho"), var_chirho("boundChirho")),
    );
    let kind_chirho = ctx_chirho.type_to_kind_chirho(&ty_chirho);
    let KindChirho::ArrowChirho(free_chirho, bound_chirho) = kind_chirho else {
        panic!("expected an arrow kind")
    };
    assert_eq!(
        ctx_chirho.type_to_kind_chirho(&var_chirho("freeChirho")),
        *free_chirho
    );
    assert!(!ctx_chirho.kind_var_cache_chirho.contains_key("boundChirho"));
    assert_ne!(
        ctx_chirho.type_to_kind_chirho(&var_chirho("boundChirho")),
        *bound_chirho
    );
}

#[test]
fn quantified_constraint_binders_do_not_capture_the_qualified_body_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    let constraint_chirho = ConstraintChirho::QuantifiedChirho {
        vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("fChirho"))],
        context_chirho: vec![],
        body_chirho: Box::new(ConstraintChirho::ClassChirho {
            kind_args_chirho: Vec::new(),
            class_chirho: Box::new(name_chirho("Eq")),
            args_chirho: vec![app_chirho(
                var_chirho("fChirho"),
                TypeChirho::ConChirho(name_chirho("Int")),
            )],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let ty_chirho = TypeChirho::QualChirho {
        context_chirho: vec![constraint_chirho],
        body_chirho: Box::new(fun_chirho(var_chirho("fChirho"), var_chirho("fChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    ctx_chirho.infer_type_kind_chirho(&ty_chirho);
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn genuine_bad_application_inside_a_shadow_is_still_a_kind_error_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::with_builtins_chirho());
    let ty_chirho = forall_chirho(
        false,
        "fChirho",
        fun_chirho(
            var_chirho("fChirho"),
            app_chirho(
                var_chirho("fChirho"),
                TypeChirho::ConChirho(name_chirho("Int")),
            ),
        ),
    );
    ctx_chirho.infer_type_kind_chirho(&ty_chirho);
    assert!(ctx_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn annotations_use_the_nearest_kind_binder_and_reuse_it_within_the_group_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    let annotation_chirho = var_chirho("kChirho");
    let outer_chirho = ctx_chirho.ast_kind_to_kind_ctx_chirho(&annotation_chirho);
    let binders_chirho = vec![
        TyVarChirho::plain_chirho(name_chirho("kChirho")),
        TyVarChirho::annotated_chirho(name_chirho("aChirho"), annotation_chirho.clone()),
        TyVarChirho::annotated_chirho(name_chirho("bChirho"), annotation_chirho.clone()),
    ];
    ctx_chirho.with_kind_binders_chirho(&binders_chirho, |inner_chirho| {
        let kind_chirho = inner_chirho.ast_kind_to_kind_ctx_chirho(&annotation_chirho);
        assert_ne!(kind_chirho, outer_chirho);
        assert_eq!(
            inner_chirho.env_chirho.lookup_chirho("aChirho"),
            Some(&kind_chirho)
        );
        assert_eq!(
            inner_chirho.env_chirho.lookup_chirho("bChirho"),
            Some(&kind_chirho)
        );
        // A deeper shadow must not capture either annotation on return.
        let nested_chirho = forall_chirho(false, "kChirho", var_chirho("kChirho"));
        assert_ne!(
            inner_chirho.type_to_kind_chirho(&nested_chirho),
            kind_chirho
        );
        assert_eq!(
            inner_chirho.ast_kind_to_kind_ctx_chirho(&annotation_chirho),
            kind_chirho
        );
    });
    assert_eq!(
        ctx_chirho.ast_kind_to_kind_ctx_chirho(&annotation_chirho),
        outer_chirho
    );
    assert!(ctx_chirho.env_chirho.lookup_chirho("aChirho").is_none());
    assert!(ctx_chirho.env_chirho.lookup_chirho("bChirho").is_none());
}

#[test]
fn synonym_substitution_retains_kind_annotations_applications_and_predicate_prefixes_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    let predicate_chirho = ConstraintChirho::KindAnnotationChirho {
        constraint_chirho: Box::new(ConstraintChirho::ClassChirho {
            class_chirho: Box::new(name_chirho("CChirho")),
            kind_args_chirho: vec![var_chirho("kChirho")],
            args_chirho: vec![var_chirho("aChirho")],
            span_chirho,
        }),
        kind_chirho: Box::new(var_chirho("kChirho")),
        span_chirho,
    };
    let source_chirho = TypeChirho::KindAnnotationChirho {
        ty_chirho: Box::new(TypeChirho::QualChirho {
            context_chirho: vec![predicate_chirho],
            body_chirho: Box::new(TypeChirho::KindApplicationChirho {
                fun_chirho: Box::new(var_chirho("fChirho")),
                arg_chirho: Box::new(var_chirho("kChirho")),
                written_chirho: false,
                span_chirho,
            }),
            span_chirho,
        }),
        kind_chirho: Box::new(var_chirho("kChirho")),
        span_chirho,
    };
    let replacement_chirho = TypeChirho::ConChirho(name_chirho("Int"));
    let result_chirho = KindInferCtxChirho::substitute_type_kind_synonym_param_chirho(
        &source_chirho,
        "kChirho",
        &replacement_chirho,
    );
    let TypeChirho::KindAnnotationChirho {
        ty_chirho,
        kind_chirho,
        ..
    } = result_chirho
    else {
        panic!("outer annotation");
    };
    assert_eq!(*kind_chirho, replacement_chirho);
    let TypeChirho::QualChirho {
        context_chirho,
        body_chirho,
        ..
    } = *ty_chirho
    else {
        panic!("context");
    };
    let TypeChirho::KindApplicationChirho {
        arg_chirho,
        written_chirho,
        ..
    } = *body_chirho
    else {
        panic!("kind application");
    };
    assert!(!written_chirho);
    assert_eq!(*arg_chirho, replacement_chirho);
    let ConstraintChirho::KindAnnotationChirho {
        constraint_chirho,
        kind_chirho,
        ..
    } = &context_chirho[0]
    else {
        panic!("constraint annotation");
    };
    assert_eq!(kind_chirho.as_ref(), &replacement_chirho);
    let ConstraintChirho::ClassChirho {
        kind_args_chirho,
        args_chirho,
        ..
    } = constraint_chirho.as_ref()
    else {
        panic!("class");
    };
    assert_eq!(kind_args_chirho, &[replacement_chirho]);
    assert_eq!(args_chirho, &[var_chirho("aChirho")]);
}

#[test]
fn a_rebound_synonym_parameter_stops_after_its_binder_kind_chirho() {
    let span_chirho = SpanChirho::DUMMY_CHIRHO;
    for required_chirho in [false, true] {
        let vars_chirho = vec![
            TyVarChirho::annotated_chirho(name_chirho("aChirho"), var_chirho("kChirho")),
            TyVarChirho::plain_chirho(name_chirho("kChirho")),
            TyVarChirho::annotated_chirho(name_chirho("bChirho"), var_chirho("kChirho")),
        ];
        let source_chirho = if required_chirho {
            TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho: Box::new(var_chirho("kChirho")),
                span_chirho,
            }
        } else {
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho: Box::new(var_chirho("kChirho")),
                span_chirho,
            }
        };
        let replacement_chirho = TypeChirho::ConChirho(name_chirho("Int"));
        let result_chirho = KindInferCtxChirho::substitute_type_kind_synonym_param_chirho(
            &source_chirho,
            "kChirho",
            &replacement_chirho,
        );
        let (vars_chirho, body_chirho) = match result_chirho {
            TypeChirho::ForallChirho {
                vars_chirho,
                body_chirho,
                ..
            }
            | TypeChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
                ..
            } => (vars_chirho, body_chirho),
            _ => panic!("forall retained"),
        };
        assert_eq!(
            vars_chirho[0].kind_annotation_chirho.as_deref(),
            Some(&replacement_chirho)
        );
        assert_eq!(
            vars_chirho[2].kind_annotation_chirho.as_deref(),
            Some(&var_chirho("kChirho"))
        );
        assert_eq!(*body_chirho, var_chirho("kChirho"));
    }
}

#[test]
fn outer_annotations_survive_legacy_synonym_expansion_chirho() {
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    ctx_chirho.type_kind_synonyms_chirho.insert(
        "AliasChirho".to_string(),
        KindTypeSynonymChirho {
            params_chirho: vec!["aChirho".to_string()],
            rhs_chirho: var_chirho("aChirho"),
        },
    );
    let source_chirho = TypeChirho::KindAnnotationChirho {
        ty_chirho: Box::new(app_chirho(
            TypeChirho::ConChirho(name_chirho("AliasChirho")),
            TypeChirho::ConChirho(name_chirho("Int")),
        )),
        kind_chirho: Box::new(TypeChirho::ConChirho(name_chirho("Type"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (_, result_chirho) = ctx_chirho
        .expand_type_kind_synonym_once_chirho(&source_chirho)
        .expect("ordinary synonym expands");
    let TypeChirho::KindAnnotationChirho {
        ty_chirho,
        kind_chirho,
        ..
    } = result_chirho
    else {
        panic!("annotation retained");
    };
    assert_eq!(*ty_chirho, TypeChirho::ConChirho(name_chirho("Int")));
    assert_eq!(*kind_chirho, TypeChirho::ConChirho(name_chirho("Type")));
}

#[test]
fn hidden_declaration_binders_do_not_add_visible_kind_arrows_chirho() {
    let mut hidden_chirho = TyVarChirho::plain_chirho(name_chirho("kChirho"));
    hidden_chirho.visibility_chirho =
        haskelujah_ast_chirho::decl_chirho::TyVarVisibilityChirho::InvisibleChirho;
    let binders_chirho = vec![
        hidden_chirho,
        TyVarChirho::annotated_chirho(
            name_chirho("aChirho"),
            TypeChirho::ConChirho(name_chirho("*")),
        ),
    ];
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(KindEnvChirho::new_chirho());
    ctx_chirho.infer_class_kind_chirho("CChirho", &binders_chirho, SpanChirho::DUMMY_CHIRHO);
    assert_eq!(
        ctx_chirho.env_chirho.lookup_chirho("CChirho"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::ConstraintChirho
        ))
    );
    ctx_chirho.infer_type_family_decl_kind_chirho(
        "FChirho",
        &binders_chirho,
        None,
        None,
        SpanChirho::DUMMY_CHIRHO,
        false,
    );
    assert_eq!(
        ctx_chirho.env_chirho.lookup_chirho("FChirho"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho
        ))
    );
}
