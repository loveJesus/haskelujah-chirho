// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

#[test]
fn data_no_params_has_kind_star_chirho() {
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Color"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: mk_name_chirho("Red"),
            fields_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Color"),
        Some(&KindChirho::StarChirho)
    );
}

#[test]
fn data_one_param_has_kind_star_to_star_chirho() {
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Box"),
        type_vars_chirho: vec![mk_name_chirho("a").into()],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: mk_name_chirho("MkBox"),
            fields_chirho: vec![(
                StrictnessChirho::LazyChirho,
                TypeChirho::VarChirho(mk_name_chirho("a")),
            )],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Box"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho
        ))
    );
}

#[test]
fn data_two_params_chirho() {
    // data Pair a b = MkPair a b
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Pair"),
        type_vars_chirho: vec![mk_name_chirho("a").into(), mk_name_chirho("b").into()],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: mk_name_chirho("MkPair"),
            fields_chirho: vec![
                (
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(mk_name_chirho("a")),
                ),
                (
                    StrictnessChirho::LazyChirho,
                    TypeChirho::VarChirho(mk_name_chirho("b")),
                ),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Pair"),
        Some(&KindChirho::arrow_n_chirho(
            vec![KindChirho::StarChirho, KindChirho::StarChirho],
            KindChirho::StarChirho
        ))
    );
}

#[test]
fn higher_kinded_type_param_chirho() {
    // data App f a = MkApp (f a)
    // This legacy-edition control checks defaulting, not GHC2021's
    // more general App :: forall k. (k -> Type) -> k -> Type.
    let mut module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("App"),
        type_vars_chirho: vec![mk_name_chirho("f").into(), mk_name_chirho("a").into()],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: mk_name_chirho("MkApp"),
            fields_chirho: vec![(
                StrictnessChirho::LazyChirho,
                TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("f"))),
                    arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            )],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    module_chirho.extensions_chirho = vec!["Haskell2010".into()];
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    // App :: (* -> *) -> * -> *
    let expected_chirho = KindChirho::arrow_chirho(
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho),
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("App"),
        Some(&expected_chirho)
    );
}

#[test]
fn qualified_type_constructor_kind_does_not_collide_chirho() {
    let env_chirho = KindEnvChirho::with_builtins_chirho();
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
    ctx_chirho.env_chirho.bind_chirho(
        "Operator".to_string(),
        KindChirho::arrow_n_chirho(
            vec![
                KindChirho::StarChirho,
                KindChirho::StarChirho,
                KindChirho::StarChirho,
            ],
            KindChirho::StarChirho,
        ),
    );

    let ty_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::ConChirho(NameChirho::RawChirho(
                        haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                            "N",
                            "Operator",
                            SpanChirho::DUMMY_CHIRHO,
                        ),
                    ))),
                    arg_chirho: Box::new(TypeChirho::ListChirho {
                        element_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("tok"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("st"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::ConChirho(NameChirho::RawChirho(
                haskelujah_ast_chirho::name_chirho::RawNameChirho::qualified_chirho(
                    "Control.Monad",
                    "Identity",
                    SpanChirho::DUMMY_CHIRHO,
                ),
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(TypeChirho::VarChirho(mk_name_chirho("a"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let _kind_chirho = ctx_chirho.infer_type_kind_chirho(&ty_chirho);
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "qualified imported type constructors should not collide with local unqualified ones: {:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn rep_kind_accepts_higher_kinded_argument_chirho() {
    let env_chirho = KindEnvChirho::with_builtins_chirho();
    let mut ctx_chirho = KindInferCtxChirho::new_chirho(env_chirho);
    ctx_chirho.env_chirho.bind_chirho(
        "QChirho".to_string(),
        KindChirho::arrow_n_chirho(
            vec![
                KindChirho::StarChirho,
                KindChirho::StarChirho,
                KindChirho::StarChirho,
            ],
            KindChirho::StarChirho,
        ),
    );

    let ty_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Rep"))),
        arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("QChirho"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let kind_chirho = ctx_chirho.infer_type_kind_chirho(&ty_chirho);
    let normalized_kind_chirho = ctx_chirho.subst_chirho.apply_chirho(&kind_chirho);
    assert_eq!(
        normalized_kind_chirho,
        KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
    );
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "Rep should accept higher-kinded arguments without diagnostics: {:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn class_single_param_chirho() {
    // class Eq a where eq :: a -> a -> Bool
    let module_chirho = mk_module_chirho(vec![DeclChirho::ClassDeclChirho {
        context_written_chirho: false,
        minimal_chirho: None,
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
        context_written_chirho: false,
        minimal_chirho: None,
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
            context_written_chirho: false,
            minimal_chirho: None,
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
                    names_chirho: vec![mk_name_chirho("unTagged")],
                    strictness_chirho: StrictnessChirho::LazyChirho,
                    ty_chirho: TypeChirho::ConChirho(mk_name_chirho("Int")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
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
        context_written_chirho: false,
        minimal_chirho: None,
        context_chirho: vec![ConstraintChirho::ClassChirho {
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
            name_chirho: mk_name_chirho("Rep"),
            type_vars_chirho: vec![mk_name_chirho("f")]
                .into_iter()
                .map(Into::into)
                .collect(),
            result_chirho: Default::default(),
            data_chirho: false,
            head_declared_chirho: true,
            defaults_chirho: vec![],
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

#[test]
fn type_alias_kind_chirho() {
    // type StringPair = (String, String)
    let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
        name_chirho: mk_name_chirho("StringPair"),
        type_vars_chirho: vec![],
        rhs_chirho: TypeChirho::TupleChirho {
            elements_chirho: vec![
                TypeChirho::ConChirho(mk_name_chirho("String")),
                TypeChirho::ConChirho(mk_name_chirho("String")),
            ],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("StringPair"),
        Some(&KindChirho::StarChirho)
    );
}

#[test]
fn standalone_kind_signature_expands_local_kind_synonym_application_chirho() {
    let mut module_chirho = mk_module_chirho(vec![
        DeclChirho::TypeAliasDeclChirho {
            name_chirho: mk_name_chirho("Cat"),
            type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("k"))],
            rhs_chirho: mk_fun_chirho(
                TypeChirho::VarChirho(mk_name_chirho("k")),
                mk_fun_chirho(
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                    TypeChirho::ConChirho(mk_name_chirho("Type")),
                ),
            ),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("FreeCat"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(DeclKindSigChirho::ResultChirho(mk_fun_chirho(
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Cat")),
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                ),
                mk_app_chirho(
                    TypeChirho::ConChirho(mk_name_chirho("Cat")),
                    TypeChirho::VarChirho(mk_name_chirho("k")),
                ),
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);

    // The asserted monomorphic kind is the explicit NoPolyKinds contract.
    // Default-edition generality is exercised by the source integration control.
    module_chirho.extensions_chirho = vec!["NoPolyKinds".into()];
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "local kind synonym applications should expand in standalone kind signatures: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("FreeCat"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
            ),
            KindChirho::arrow_chirho(
                KindChirho::StarChirho,
                KindChirho::arrow_chirho(KindChirho::StarChirho, KindChirho::StarChirho)
            )
        ))
    );
}

#[test]
fn type_alias_visible_kind_binder_proxy_function_rhs_chirho() {
    let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
        name_chirho: mk_name_chirho("S"),
        type_vars_chirho: vec![
            TyVarChirho::annotated_chirho(mk_name_chirho("k"), AstKindChirho::StarChirho),
            TyVarChirho::annotated_chirho(
                mk_name_chirho("a"),
                AstKindChirho::VarChirho("k".to_string()),
            ),
        ],
        rhs_chirho: mk_fun_chirho(
            mk_app_chirho(
                TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                TypeChirho::VarChirho(mk_name_chirho("a")),
            ),
            mk_app_chirho(
                TypeChirho::ConChirho(mk_name_chirho("Proxy")),
                TypeChirho::VarChirho(mk_name_chirho("k")),
            ),
        ),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "proxy function alias should kind-check: {:?}",
        result_chirho.diagnostics_chirho
    );
}

#[test]
fn newtype_kind_chirho() {
    // newtype Wrapper a = Wrap a
    let module_chirho = mk_module_chirho(vec![DeclChirho::NewtypeDeclChirho {
        name_chirho: mk_name_chirho("Wrapper"),
        type_vars_chirho: vec![mk_name_chirho("a").into()],
        constructor_chirho: ConDeclChirho::OrdinaryChirho {
            name_chirho: mk_name_chirho("Wrap"),
            fields_chirho: vec![(
                StrictnessChirho::LazyChirho,
                TypeChirho::VarChirho(mk_name_chirho("a")),
            )],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Wrapper"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho
        ))
    );
}

#[test]
fn empty_module_no_errors_chirho() {
    let module_chirho = mk_module_chirho(vec![]);
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
}

#[test]
fn default_unconstrained_vars_chirho() {
    let k_chirho = KindChirho::VarChirho(KindVarChirho(99));
    assert_eq!(default_kind_vars_chirho(&k_chirho), KindChirho::StarChirho);
}

#[test]
fn constraint_kind_display_chirho() {
    assert_eq!(KindChirho::ConstraintChirho.to_string(), "Constraint");
}

#[test]
fn builtin_typeable_accepts_higher_kinded_argument_chirho() {
    let module_chirho = mk_module_chirho(vec![DeclChirho::TypeAliasDeclChirho {
        name_chirho: mk_name_chirho("Typeable1Chirho"),
        type_vars_chirho: vec![],
        rhs_chirho: TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Typeable"))),
            arg_chirho: Box::new(TypeChirho::ConChirho(mk_name_chirho("Maybe"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "Typeable should accept higher-kinded arguments: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("Typeable1Chirho"),
        Some(&KindChirho::ConstraintChirho)
    );
}

#[test]
fn standalone_forall_kind_signature_preserves_arrow_kind_chirho() {
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("AppChirho"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: Some(DeclKindSigChirho::ResultChirho(TypeChirho::ForallChirho {
            vars_chirho: vec![TyVarChirho::annotated_chirho(
                mk_name_chirho("fChirho"),
                AstKindChirho::ArrowChirho(
                    Box::new(AstKindChirho::StarChirho),
                    Box::new(AstKindChirho::StarChirho),
                ),
            )],
            body_chirho: Box::new(mk_fun_chirho(
                TypeChirho::ConChirho(mk_name_chirho("Type")),
                TypeChirho::ConChirho(mk_name_chirho("Type")),
            )),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        })),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);

    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "forall standalone kind signature should kind-check: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result_chirho.env_chirho.lookup_chirho("AppChirho"),
        Some(&KindChirho::arrow_chirho(
            KindChirho::StarChirho,
            KindChirho::StarChirho
        ))
    );
}
