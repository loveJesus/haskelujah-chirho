// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Data, newtype and type synonym kinds, including Constraint return kinds and shadowing.

use super::*;

// -- Module-level kind inference tests --

#[test]
fn data_return_kind_constraint_is_rejected_chirho() {
    // GHC-55233: `data Foo :: Constraint` — unconditionally rejected, no
    // extension licenses it.
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Foo"),
        type_vars_chirho: vec![],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(TypeChirho::ConChirho(
            mk_name_chirho("Constraint"),
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    assert!(
        infer_module_kinds_chirho(&module_chirho)
            .diagnostics_chirho
            .has_errors_chirho(),
        "a data declaration returning Constraint must be rejected"
    );
}

#[test]
fn data_binder_kind_constraint_is_accepted_chirho() {
    // `data Foo (_ :: Constraint)` — GHC ACCEPTS this: the Constraint is the
    // BINDER's kind. Before the data-head binder fix this lowered identically to
    // the case above, which is why the check could not be written.
    let mut binder_chirho: TyVarChirho = mk_name_chirho("_").into();
    binder_chirho.kind_annotation_chirho = Some(AstKindChirho::ConstraintChirho);
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
        name_chirho: mk_name_chirho("Foo"),
        type_vars_chirho: vec![binder_chirho],
        constructors_chirho: vec![],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    assert!(
        !infer_module_kinds_chirho(&module_chirho)
            .diagnostics_chirho
            .has_errors_chirho(),
        "a BINDER of kind Constraint is legal and must stay accepted"
    );
}

#[test]
fn data_return_kind_constraint_stands_down_when_shadowed_chirho() {
    // A module declaring its own `Constraint` shadows the wired-in one.
    let module_chirho = mk_module_chirho(vec![
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Constraint"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("Foo"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(TypeChirho::ConChirho(
                mk_name_chirho("Constraint"),
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ]);
    assert!(
        !infer_module_kinds_chirho(&module_chirho)
            .diagnostics_chirho
            .has_errors_chirho(),
        "a locally declared Constraint shadows the wired-in kind"
    );
}

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
    // f :: * -> *, a :: *, App :: (* -> *) -> * -> *
    let module_chirho = mk_module_chirho(vec![DeclChirho::DataDeclChirho {
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
    let module_chirho = mk_module_chirho(vec![
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
            kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(mk_fun_chirho(
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
