// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! An associated default is scoped by its own equation (row 686, GHC-76037).
//! workflow: compiler-pipeline-chirho/type-scope-resolution-chirho

use super::*;
use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, TyVarChirho, TyVarVisibilityChirho,
};
use haskelujah_ast_chirho::name_chirho::RawNameChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn binder_chirho(text_chirho: &str) -> TyVarChirho {
    TyVarChirho::plain_chirho(name_chirho(text_chirho))
}

fn kind_binder_chirho(text_chirho: &str) -> TyVarChirho {
    let mut binder_chirho = binder_chirho(text_chirho);
    binder_chirho.visibility_chirho = TyVarVisibilityChirho::InvisibleChirho;
    binder_chirho
}

fn variable_chirho(text_chirho: &str) -> TypeChirho {
    TypeChirho::VarChirho(name_chirho(text_chirho))
}

/// A class over `class_params_chirho` whose one associated family has a
/// written default with the given right side and recorded scope.
fn class_with_default_chirho(
    class_params_chirho: &[&str],
    default_rhs_chirho: TypeChirho,
    default_binders_chirho: Option<Vec<TyVarChirho>>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("DefaultScopeChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: name_chirho("ClassChirho"),
            type_vars_chirho: class_params_chirho
                .iter()
                .map(|param_chirho| binder_chirho(param_chirho))
                .collect(),
            methods_chirho: vec![],
            associated_tfs_chirho: vec![AssocTypeFamilyChirho {
                injectivity_chirho: None,
                default_arguments_chirho: None,
                name_chirho: name_chirho("FamilyChirho"),
                result_kind_chirho: None,
                type_vars_chirho: vec![name_chirho("aChirho").into()],
                data_family_chirho: false,
                default_rhs_chirho: Some(default_rhs_chirho),
                default_params_chirho: vec![],
                default_binders_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn check_chirho(module_chirho: &ModuleChirho) -> DiagnosticBundleChirho {
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    check_module_type_scope_chirho(
        module_chirho,
        &NameEnvChirho::new_chirho(),
        &[],
        &mut diagnostics_chirho,
    );
    diagnostics_chirho
}

#[test]
fn a_written_default_does_not_see_other_class_parameters_chirho() {
    // T5481 and D01: GHC 9.14.1 reports `b` not in scope in `type X a = b`.
    let diagnostics_chirho = check_chirho(&class_with_default_chirho(
        &["aChirho", "bChirho"],
        variable_chirho("bChirho"),
        Some(vec![binder_chirho("aChirho")]),
    ));
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("type variable not in scope: `bChirho`"));
}

#[test]
fn a_written_default_sees_the_kind_variables_its_binders_bind_chirho() {
    // T18585: `type Dom (f :: i -> j) = SelfDom f (DefaultCat i)` uses `i`.
    let diagnostics_chirho = check_chirho(&class_with_default_chirho(
        &["fChirho"],
        TypeChirho::AppChirho {
            fun_chirho: Box::new(variable_chirho("fChirho")),
            arg_chirho: Box::new(variable_chirho("iChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        Some(vec![
            binder_chirho("fChirho"),
            kind_binder_chirho("iChirho"),
        ]),
    ));
    assert!(diagnostics_chirho.is_empty_chirho());
}

#[test]
fn a_default_without_recorded_scope_keeps_the_class_binders_chirho() {
    // A directly built AST records no default scope; it keeps the legacy walk.
    let diagnostics_chirho = check_chirho(&class_with_default_chirho(
        &["aChirho", "bChirho"],
        variable_chirho("bChirho"),
        None,
    ));
    assert!(diagnostics_chirho.is_empty_chirho());
}

fn universe_chirho() -> TypeChirho {
    let mut name_chirho = name_chirho("Type");
    name_chirho.set_type_constructor_chirho(Some(
        haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho::compiler_data_chirho(
            CompilerTypeConstructorChirho::TypeUniverseChirho,
        ),
    ));
    TypeChirho::ConChirho(name_chirho)
}

fn type_namespace_chirho(names_chirho: &[&str]) -> NameEnvChirho {
    let mut env_chirho = NameEnvChirho::new_chirho();
    for name_chirho in names_chirho {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            NamespaceChirho::TypeChirho,
            SpanChirho::DUMMY_CHIRHO,
        );
    }
    env_chirho
}

fn check_type_chirho(ty_chirho: TypeChirho, env_chirho: &NameEnvChirho) -> DiagnosticBundleChirho {
    let mut module_chirho = class_with_default_chirho(&[], variable_chirho("aChirho"), None);
    module_chirho.decls_chirho = vec![DeclChirho::TypeSigChirho {
        name_chirho: name_chirho("valueChirho"),
        ty_chirho,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }];
    let mut diagnostics_chirho = DiagnosticBundleChirho::empty_chirho();
    check_module_type_scope_chirho(&module_chirho, env_chirho, &[], &mut diagnostics_chirho);
    diagnostics_chirho
}

#[test]
fn full_kind_variable_capture_respects_nested_forall_scope_chirho() {
    let kind_chirho = TypeChirho::FunChirho {
        arg_chirho: Box::new(TypeChirho::ForallChirho {
            vars_chirho: vec![binder_chirho("kChirho")],
            body_chirho: Box::new(TypeChirho::FunChirho {
                arg_chirho: Box::new(variable_chirho("kChirho")),
                mult_chirho: None,
                result_chirho: Box::new(universe_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        mult_chirho: None,
        result_chirho: Box::new(variable_chirho("jChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut names_chirho = Vec::new();
    collect_kind_variable_names_chirho(&kind_chirho, &mut names_chirho);
    assert_eq!(names_chirho, ["jChirho"]);
    let mut module_chirho = class_with_default_chirho(&[], variable_chirho("aChirho"), None);
    for (rhs_chirho, expected_errors_chirho) in [("jChirho", 0), ("kChirho", 1)] {
        module_chirho.decls_chirho = vec![DeclChirho::TypeAliasDeclChirho {
            name_chirho: name_chirho("AliasChirho"),
            type_vars_chirho: vec![TyVarChirho::annotated_chirho(
                name_chirho("fChirho"),
                kind_chirho.clone(),
            )],
            rhs_chirho: variable_chirho(rhs_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }];
        assert_eq!(
            check_chirho(&module_chirho).error_count_chirho(),
            expected_errors_chirho
        );
    }
}

#[test]
fn full_kind_annotations_and_hidden_constraint_arguments_resolve_names_chirho() {
    let missing_chirho = || TypeChirho::ConChirho(name_chirho("MissingKindChirho"));
    let predicate_chirho = || ConstraintChirho::ClassChirho {
        class_chirho: Box::new(name_chirho("CChirho")),
        kind_args_chirho: vec![],
        args_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let mut prefix_chirho = predicate_chirho();
    let ConstraintChirho::ClassChirho {
        kind_args_chirho, ..
    } = &mut prefix_chirho
    else {
        unreachable!()
    };
    kind_args_chirho.push(missing_chirho());
    let mut types_chirho = vec![
        TypeChirho::KindApplicationChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(name_chirho("FChirho"))),
            arg_chirho: Box::new(missing_chirho()),
            written_chirho: true,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        TypeChirho::KindAnnotationChirho {
            ty_chirho: Box::new(variable_chirho("aChirho")),
            kind_chirho: Box::new(missing_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ];
    for constraint_chirho in [
        prefix_chirho,
        ConstraintChirho::KindAnnotationChirho {
            constraint_chirho: Box::new(predicate_chirho()),
            kind_chirho: Box::new(missing_chirho()),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ] {
        types_chirho.push(TypeChirho::QualChirho {
            context_chirho: vec![constraint_chirho],
            body_chirho: Box::new(variable_chirho("aChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        });
    }
    let env_chirho = type_namespace_chirho(&["CChirho", "FChirho"]);
    for ty_chirho in types_chirho {
        let diagnostics_chirho = check_type_chirho(ty_chirho, &env_chirho);
        assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
        assert!(format!("{diagnostics_chirho}").contains("MissingKindChirho"));
    }
}

#[test]
fn a_signature_annotation_does_not_hide_its_outer_forall_scope_chirho() {
    let ty_chirho = TypeChirho::KindAnnotationChirho {
        ty_chirho: Box::new(TypeChirho::ForallChirho {
            vars_chirho: vec![binder_chirho("aChirho")],
            body_chirho: Box::new(variable_chirho("bChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        kind_chirho: Box::new(universe_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_type_chirho(ty_chirho, &NameEnvChirho::new_chirho());
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("bChirho"));
}

#[test]
fn compiler_universe_identity_is_not_inferred_from_a_source_homonym_chirho() {
    let kinded_chirho = |kind_chirho| TypeChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho::annotated_chirho(
            name_chirho("aChirho"),
            kind_chirho,
        )],
        body_chirho: Box::new(variable_chirho("aChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    assert!(
        check_type_chirho(
            kinded_chirho(universe_chirho()),
            &NameEnvChirho::new_chirho()
        )
        .is_empty_chirho()
    );
    let source_chirho = TypeChirho::ConChirho(name_chirho("Type"));
    assert_eq!(
        check_type_chirho(
            kinded_chirho(source_chirho.clone()),
            &NameEnvChirho::new_chirho()
        )
        .error_count_chirho(),
        1
    );
    // A source binding is still a legitimate namespace binding; this says
    // nothing about its kind or universe role.
    assert!(
        check_type_chirho(
            kinded_chirho(source_chirho),
            &type_namespace_chirho(&["Type"])
        )
        .is_empty_chirho()
    );
}

#[test]
fn written_default_kind_scope_uses_its_recorded_names_only_chirho() {
    let annotation_chirho = TypeChirho::ListChirho {
        element_chirho: Box::new(variable_chirho("jChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    for recorded_chirho in [false, true] {
        let mut binders_chirho = vec![TyVarChirho::annotated_chirho(
            name_chirho("fChirho"),
            annotation_chirho.clone(),
        )];
        if recorded_chirho {
            binders_chirho.push(kind_binder_chirho("jChirho"));
        }
        let diagnostics_chirho = check_chirho(&class_with_default_chirho(
            &["jChirho"],
            variable_chirho("jChirho"),
            Some(binders_chirho),
        ));
        assert_eq!(
            diagnostics_chirho.error_count_chirho(),
            usize::from(!recorded_chirho)
        );
    }
}

#[test]
fn the_shared_preflight_rejects_a_deep_kind_before_scope_collection_chirho() {
    let mut kind_chirho = universe_chirho();
    for _ in 0..haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitsChirho::default()
        .depth_chirho
        + 8
    {
        kind_chirho = TypeChirho::ListChirho {
            element_chirho: Box::new(kind_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
    }
    let ty_chirho = TypeChirho::ForallChirho {
        vars_chirho: vec![TyVarChirho::annotated_chirho(
            name_chirho("aChirho"),
            kind_chirho,
        )],
        body_chirho: Box::new(variable_chirho("aChirho")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let diagnostics_chirho = check_type_chirho(ty_chirho, &NameEnvChirho::new_chirho());
    assert_eq!(diagnostics_chirho.error_count_chirho(), 1);
    assert!(format!("{diagnostics_chirho}").contains("projection exceeded"));
}
