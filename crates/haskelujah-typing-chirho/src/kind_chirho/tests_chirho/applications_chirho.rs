// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn type_family_decl_result_kind_guides_later_applications_chirho() {
    let t_var_chirho = TyVarChirho::plain_chirho(mk_name_chirho("t"));
    let mut module_chirho = mk_module_chirho(vec![
        DeclChirho::TypeFamilyDeclChirho {
            name_chirho: mk_name_chirho("TrivialFamily"),
            data_chirho: false,
            type_vars_chirho: vec![t_var_chirho.clone()],
            result_chirho: haskelujah_ast_chirho::decl_chirho::TypeFamilyResultChirho {
                kind_sig_chirho: Some(DeclKindSigChirho::ResultChirho(TypeChirho::ConChirho(
                    mk_name_chirho("Type"),
                ))),
                ..Default::default()
            },
            body_chirho: haskelujah_ast_chirho::decl_chirho::TypeFamilyBodyChirho::OpenChirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("ProblemTypeChirho"),
            type_vars_chirho: vec![t_var_chirho],
            rhs_chirho: mk_app_chirho(
                TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("TrivialFamily")),
                    TypeChirho::VarChirho(mk_name_chirho("t")),
                ),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);
    module_chirho
        .extensions_chirho
        .push("PolyKinds".to_string());

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "type family result kind should make later applications kind-check: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn family_result_kind_is_not_an_arbitrary_kind_at_each_use_chirho() {
    let mut module_chirho = mk_module_chirho(vec![
        DeclChirho::TypeFamilyDeclChirho {
            name_chirho: mk_name_chirho("FamilyChirho"),
            data_chirho: false,
            type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
            result_chirho: Default::default(),
            body_chirho: haskelujah_ast_chirho::decl_chirho::TypeFamilyBodyChirho::OpenChirho,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("IndexedChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(DeclKindSigChirho::ResultChirho(mk_fun_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("FamilyChirho")),
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                ),
                TypeChirho::ConChirho(mk_name_chirho("Type")),
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::TypeSigChirho {
            name_chirho: mk_name_chirho("useIndexedChirho"),
            ty_chirho: mk_fun_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("IndexedChirho")),
                    TypeChirho::ConChirho(mk_name_chirho("Maybe")),
                ),
                mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("IndexedChirho")),
                        TypeChirho::ConChirho(mk_name_chirho("Int")),
                    ),
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                ),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);
    module_chirho
        .extensions_chirho
        .push("PolyKinds".to_string());

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    // GHC 9.14.1 rejects both applications: the term Family k is not
    // interchangeable with the kind OF that term. The former assertion
    // expected this invalid program to pass by freshening the entire term.
    let errors_chirho = result_chirho.diagnostics_chirho.diagnostics_chirho();
    assert_eq!(errors_chirho.len(), 2, "{errors_chirho:?}");
    assert!(
        errors_chirho.iter().all(|error_chirho| {
            error_chirho.code_chirho == Some(ErrorCodeChirho::error_chirho(300))
                && error_chirho.message_chirho.contains("FamilyChirho")
                && error_chirho.message_chirho.contains("type application")
        }),
        "{errors_chirho:?}"
    );
}

#[test]
fn promoted_constructor_does_not_reuse_same_name_type_constructor_kind_chirho() {
    let module_chirho = mk_module_chirho(vec![
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("R"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("PromotedRAppChirho"),
            type_vars_chirho: vec![],
            rhs_chirho: mk_app_chirho(
                TypeChirho::PromotedConChirho {
                    name_chirho: mk_name_chirho("R"),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                TypeChirho::ConChirho(mk_name_chirho("Int")),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "promoted constructor should not reuse its same-name type constructor's `*` kind: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("R"),
        Some(&KindChirho::StarChirho)
    );
}

#[test]
fn promoted_list_cons_accepts_polykinded_head_chirho() {
    let mut module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
        name_chirho: mk_name_chirho("ConsMaybeChirho"),
        type_vars_chirho: vec![],
        rhs_chirho: mk_app_chirho(
            mk_app_chirho(
                TypeChirho::ConChirho(mk_name_chirho(":")),
                TypeChirho::ConChirho(mk_name_chirho("Maybe")),
            ),
            TypeChirho::PromotedListChirho {
                elements_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    module_chirho
        .extensions_chirho
        .push("PolyKinds".to_string());

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "promoted list cons should not force its head to kind `*`: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn gadt_return_type_accepts_promoted_list_cons_chirho() {
    let l_chirho = TypeChirho::VarChirho(mk_name_chirho("l"));
    let ls_chirho = TypeChirho::VarChirho(mk_name_chirho("ls"));
    let t_chirho = TypeChirho::VarChirho(mk_name_chirho("t"));
    let promoted_cons_chirho = mk_app_chirho(
        mk_app_chirho(TypeChirho::ConChirho(mk_name_chirho(":")), l_chirho.clone()),
        ls_chirho.clone(),
    );
    let stack_ls_t_chirho = mk_app_chirho(
        mk_app_chirho(TypeChirho::ConChirho(mk_name_chirho("Stack")), ls_chirho),
        t_chirho.clone(),
    );
    let stack_cons_t_chirho = mk_app_chirho(
        mk_app_chirho(
            TypeChirho::ConChirho(mk_name_chirho("Stack")),
            promoted_cons_chirho,
        ),
        t_chirho.clone(),
    );
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Stack"),
        type_vars_chirho: vec![
            TyVarChirho::plain_chirho(mk_name_chirho("lrs")),
            TyVarChirho::annotated_chirho(
                mk_name_chirho("t"),
                AstKindChirho::ArrowChirho(
                    Box::new(AstKindChirho::StarChirho),
                    Box::new(AstKindChirho::StarChirho),
                ),
            ),
        ],
        constructors_chirho: vec![ConDeclChirho::GadtChirho {
            name_chirho: mk_name_chirho("SLayer"),
            ty_chirho: mk_fun_chirho(
                mk_app_chirho(t_chirho, l_chirho),
                mk_fun_chirho(stack_ls_t_chirho, stack_cons_t_chirho),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "GADT return type should accept `Stack (l ': ls) t`: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
}

#[test]
fn builtin_ghc_generics_representations_accept_partial_apps_chirho() {
    let unit_ty_chirho = TypeChirho::TupleChirho {
        elements_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = mk_module_chirho(vec![
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("GK1Chirho"),
            type_vars_chirho: vec![],
            rhs_chirho: mk_app_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("K1")),
                    unit_ty_chirho.clone(),
                ),
                TypeChirho::ConChirho(mk_name_chirho("Int")),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("GM1Chirho"),
            type_vars_chirho: vec![],
            rhs_chirho: mk_app_chirho(
                mk_app_chirho(
                    mk_app_chirho(
                        TypeChirho::ConChirho(mk_name_chirho("M1")),
                        unit_ty_chirho.clone(),
                    ),
                    unit_ty_chirho.clone(),
                ),
                TypeChirho::ConChirho(mk_name_chirho("U1")),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("GSumChirho"),
            type_vars_chirho: vec![],
            rhs_chirho: mk_app_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho(":+:")),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                TypeChirho::ConChirho(mk_name_chirho("U1")),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("GProdChirho"),
            type_vars_chirho: vec![],
            rhs_chirho: mk_app_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho(":*:")),
                    TypeChirho::ConChirho(mk_name_chirho("U1")),
                ),
                TypeChirho::ConChirho(mk_name_chirho("U1")),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "GHC.Generics representation constructors should kind-check when partially applied: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
    let expected_rep_functor_kind_chirho =
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho);
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("GK1Chirho"),
        Some(&expected_rep_functor_kind_chirho)
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("GM1Chirho"),
        Some(&expected_rep_functor_kind_chirho)
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("GSumChirho"),
        Some(&expected_rep_functor_kind_chirho)
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("GProdChirho"),
        Some(&expected_rep_functor_kind_chirho)
    );
}

#[test]
fn ast_kind_constraint_converts_chirho() {
    let ast_chirho = AstKindChirho::ConstraintChirho;
    assert_eq!(
        ast_kind_to_kind_chirho(&ast_chirho),
        KindChirho::ConstraintChirho
    );
}

#[test]
fn ast_kind_constraint_arrow_converts_chirho() {
    let ast_chirho = AstKindChirho::ArrowChirho(
        Box::new(AstKindChirho::StarChirho),
        Box::new(AstKindChirho::ConstraintChirho),
    );
    let expected_chirho =
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::ConstraintChirho);
    assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), expected_chirho);
}

#[test]
fn ast_kind_var_defaults_to_star_chirho() {
    // PolyKinds: standalone ast_kind_to_kind_chirho defaults kind vars to *
    let ast_chirho = AstKindChirho::VarChirho("k".to_string());
    assert_eq!(ast_kind_to_kind_chirho(&ast_chirho), KindChirho::StarChirho);
}

/// `class C a b` plus `instance C <n args>`.
fn mk_class_and_instance_module_chirho(
    class_param_names_chirho: &[&str],
    instance_head_types_chirho: Vec<TypeChirho>,
) -> ModuleChirho {
    mk_module_chirho(vec![
        DeclChirho::ClassDeclChirho {
            context_written_chirho: false,
            minimal_chirho: None,
            context_chirho: vec![],
            name_chirho: mk_name_chirho("CChirho"),
            type_vars_chirho: class_param_names_chirho
                .iter()
                .map(|p_chirho| mk_name_chirho(p_chirho).into())
                .collect(),
            methods_chirho: vec![],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::InstanceDeclChirho {
            context_chirho: vec![],
            class_chirho: mk_name_chirho("CChirho"),
            types_chirho: instance_head_types_chirho,
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ])
}

#[test]
fn instance_head_under_applied_class_is_rejected_chirho() {
    let module_chirho = mk_class_and_instance_module_chirho(
        &["a", "b"],
        vec![TypeChirho::ConChirho(mk_name_chirho("Int"))],
    );
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        result_chirho.diagnostics_chirho.has_errors_chirho(),
        "a two-parameter class applied to one argument must be rejected"
    );
}

#[test]
fn instance_head_over_applied_class_is_rejected_chirho() {
    let module_chirho = mk_class_and_instance_module_chirho(
        &["a"],
        vec![
            TypeChirho::ConChirho(mk_name_chirho("Int")),
            TypeChirho::ConChirho(mk_name_chirho("Bool")),
        ],
    );
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        result_chirho.diagnostics_chirho.has_errors_chirho(),
        "a one-parameter class applied to two arguments must be rejected"
    );
}

#[test]
fn instance_head_matching_arity_is_accepted_chirho() {
    let module_chirho = mk_class_and_instance_module_chirho(
        &["a", "b"],
        vec![
            TypeChirho::ConChirho(mk_name_chirho("Int")),
            TypeChirho::ConChirho(mk_name_chirho("Bool")),
        ],
    );
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "a correctly-saturated instance head must be accepted: {:?}",
        result_chirho.diagnostics_chirho
    );
}

#[test]
fn instance_head_of_undeclared_class_is_not_judged_chirho() {
    // The class is not declared here, so its arity is unknown to us and no
    // verdict may be reached — an imported class could have any arity.
    let module_chirho = mk_module_chirho(vec![DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: mk_name_chirho("SomeImportedClassChirho"),
        types_chirho: vec![TypeChirho::ConChirho(mk_name_chirho("Int"))],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "an instance of an undeclared class must not be judged: {:?}",
        result_chirho.diagnostics_chirho
    );
}

#[test]
fn kind_arity_is_final_only_when_tail_is_not_a_variable_chirho() {
    // `k -> *` has arity 1 even though the ARGUMENT kind is a variable —
    // only a variable TAIL leaves the arity open.
    let open_tail_chirho = KindChirho::arrow_chirho(
        KindChirho::StarChirho,
        KindChirho::VarChirho(KindVarChirho(0)),
    );
    assert_eq!(
        KindInferCtxChirho::kind_arity_chirho(&open_tail_chirho),
        (1, false)
    );
    let var_arg_chirho = KindChirho::arrow_chirho(
        KindChirho::VarChirho(KindVarChirho(0)),
        KindChirho::ConstraintChirho,
    );
    assert_eq!(
        KindInferCtxChirho::kind_arity_chirho(&var_arg_chirho),
        (1, true)
    );
}

#[test]
fn kind_var_cache_reuses_same_var_chirho() {
    // PolyKinds: context-aware conversion maps same name to same kind var
    let env_chirho = KindEnvChirho::new_chirho();
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
    let k1_chirho =
        ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
    let k2_chirho =
        ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("k".to_string()));
    assert_eq!(k1_chirho, k2_chirho);
    // Different name gets different var
    let j_chirho =
        ctx_chirho.ast_kind_to_kind_ctx_chirho(&AstKindChirho::VarChirho("j".to_string()));
    assert_ne!(k1_chirho, j_chirho);
}
