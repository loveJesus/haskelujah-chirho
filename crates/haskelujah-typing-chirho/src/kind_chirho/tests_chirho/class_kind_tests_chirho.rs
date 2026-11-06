// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Class kinds, associated families shadowing builtins, and instance-head arity.

use super::*;

#[test]
fn class_single_param_chirho() {
    // class Eq a where eq :: a -> a -> Bool
    let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho: vec![],
        name_chirho: mk_name_chirho("Eq"),
        type_vars_chirho: vec![mk_name_chirho("a").into()],
        methods_chirho: vec![ClassMethodChirho {
            name_chirho: mk_name_chirho("eq"),
            ty_chirho: mk_fun_chirho(
                TypeChirho::VarChirho(mk_name_chirho("a")),
                mk_fun_chirho(
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                    TypeChirho::ConChirho(mk_name_chirho("Bool")),
                ),
            ),
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Eq"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::ConstraintChirho
        ))
    );
}

#[test]
fn class_higher_kinded_param_chirho() {
    // class Functor f where fmap :: (a -> b) -> f a -> f b
    // f :: * -> *, Functor :: (* -> *) -> Constraint
    let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho: vec![],
        name_chirho: mk_name_chirho("Functor"),
        type_vars_chirho: vec![mk_name_chirho("f").into()],
        methods_chirho: vec![ClassMethodChirho {
            name_chirho: mk_name_chirho("fmap"),
            ty_chirho: mk_fun_chirho(
                // (a -> b)
                mk_fun_chirho(
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                    TypeChirho::VarChirho(mk_name_chirho("b")),
                ),
                // f a -> f b
                mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("f")),
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                    ),
                    mk_app_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("f")),
                        TypeChirho::VarChirho(mk_name_chirho("b")),
                    ),
                ),
            ),
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    // Functor :: (* -> *) -> Constraint
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Functor"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::ConstraintChirho
        ))
    );
}

#[test]
fn local_tagged_decl_shadows_builtin_tagged_kind_chirho() {
    let module_chirho = mk_module_chirho(vec![
        DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: mk_name_chirho("SumSize"),
            type_vars_chirho: vec![mk_name_chirho("f").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: mk_name_chirho("sumSize"),
                ty_chirho: mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Tagged")),
                    TypeChirho::VarChirho(mk_name_chirho("f")),
                ),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::NewtypeDeclChirho {
            name_chirho: mk_name_chirho("Tagged"),
            type_vars_chirho: vec![TyVarChirho::annotated_chirho(
                mk_name_chirho("s"),
                AstKindChirho::ArrowChirho(
                    Box::new(AstKindChirho::StarChirho),
                    Box::new(AstKindChirho::StarChirho),
                ),
            )],
            constructor_chirho: ConDeclChirho::RecordChirho {
                name_chirho: mk_name_chirho("Tagged"),
                fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                    multiplicity_chirho: None,
                    names_chirho: vec![mk_name_chirho("unTagged")],
                    strictness_chirho: StrictnessChirho::LazyChirho,
                    ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                prefix_chirho: haskelujah_ast_chirho::decl_chirho::ConPrefixChirho::default(),
            },
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "local Tagged should shadow the builtin Tagged kind: {:?}",
        result_chirho.diagnostics_chirho
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("SumSize"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::ConstraintChirho
        ))
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Tagged"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::StarChirho
        ))
    );
}

#[test]
fn class_associated_type_family_shadows_builtin_rep_chirho() {
    let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho: vec![ConstraintChirho::ClassChirho {
            kind_args_chirho: Vec::new(),
            class_chirho: mk_name_chirho("Contravariant"),
            args_chirho: vec![TypeChirho::VarChirho(mk_name_chirho("f"))],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        name_chirho: mk_name_chirho("Representable"),
        type_vars_chirho: vec![mk_name_chirho("f").into()],
        methods_chirho: vec![
            ClassMethodChirho {
                name_chirho: mk_name_chirho("tabulate"),
                ty_chirho: mk_fun_chirho(
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                        mk_app_chirho(
                            TypeChirho::ConChirho(mk_name_chirho("Rep")),
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                        ),
                    ),
                    mk_app_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("f")),
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                    ),
                ),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            ClassMethodChirho {
                name_chirho: mk_name_chirho("index"),
                ty_chirho: mk_fun_chirho(
                    mk_app_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("f")),
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                    ),
                    mk_fun_chirho(
                        TypeChirho::VarChirho(mk_name_chirho("a")),
                        mk_app_chirho(
                            TypeChirho::ConChirho(mk_name_chirho("Rep")),
                            TypeChirho::VarChirho(mk_name_chirho("f")),
                        ),
                    ),
                ),
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        associated_tfs_chirho: vec![AssocTypeFamilyChirho {
            injectivity_chirho: None,
            default_arguments_chirho: None,
            name_chirho: mk_name_chirho("Rep"),
            result_kind_chirho: None,
            type_vars_chirho: vec![mk_name_chirho("f").into()],
            data_family_chirho: false,
            default_rhs_chirho: None,
            default_params_chirho: vec![],
            default_binders_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "associated Rep family should kind-check inside class methods: {:?}",
        result_chirho.diagnostics_chirho
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Rep"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
            KindChirho::StarChirho
        ))
    );
}

/// `class C a b` plus `instance C <n args>`.
fn mk_class_and_instance_module_chirho(
    class_param_names_chirho: &[&str],
    instance_head_types_chirho: Vec<TypeChirho>,
) -> ModuleChirho {
    mk_module_chirho(vec![
        DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
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
            kind_args_chirho: Vec::new(),
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
        kind_args_chirho: Vec::new(),
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
