// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Module-level kinds: defaults, standalone signatures, families, promoted and GADT forms,
//! and generics representations.

use super::*;

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
fn empty_module_no_errors_chirho() {
    let module_chirho = mk_module_chirho(vec![]);
    let result_chirho = infer_module_kinds_chirho(&module_chirho);
    assert!(!result_chirho.diagnostics_chirho.has_errors_chirho());
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
        kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(TypeChirho::ForallChirho {
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

#[test]
fn type_family_decl_result_kind_guides_later_applications_chirho() {
    let t_var_chirho = TyVarChirho::plain_chirho(mk_name_chirho("t"));
    let mut module_chirho = mk_module_chirho(vec![
        DeclChirho::TypeFamilyDeclChirho {
            injectivity_chirho: None,
            name_chirho: mk_name_chirho("TrivialFamily"),
            type_vars_chirho: vec![t_var_chirho.clone()],
            result_kind_chirho: Some(TypeChirho::ConChirho(mk_name_chirho("Type"))),
            data_family_chirho: false,
            standalone_kind_chirho: None,
            closed_chirho: false,
            equations_chirho: vec![],
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
fn polykinds_unannotated_type_family_result_stays_flexible_chirho() {
    let mut module_chirho = mk_module_chirho(vec![
        DeclChirho::TypeFamilyDeclChirho {
            injectivity_chirho: None,
            name_chirho: mk_name_chirho("FamilyChirho"),
            type_vars_chirho: vec![TyVarChirho::plain_chirho(mk_name_chirho("a"))],
            result_kind_chirho: None,
            data_family_chirho: false,
            standalone_kind_chirho: None,
            closed_chirho: false,
            equations_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        DeclChirho::DataDeclChirho {
            name_chirho: mk_name_chirho("IndexedChirho"),
            type_vars_chirho: vec![],
            constructors_chirho: vec![],
            deriving_chirho: vec![],
            kind_sig_chirho: Some(DataKindSigChirho::ResultChirho(mk_fun_chirho(
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
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "unannotated PolyKinds type-family result should remain flexible at use sites: {:?}",
        result_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .map(|diagnostic_chirho| diagnostic_chirho.to_string())
            .collect::<Vec<_>>()
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
            signature_chirho: Box::new(
                haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho::from_type_chirho(
                    mk_fun_chirho(
                        mk_app_chirho(t_chirho, l_chirho),
                        mk_fun_chirho(stack_ls_t_chirho, stack_cons_t_chirho),
                    ),
                ),
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
