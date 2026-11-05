// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! User classes and instances, and the schemes of their methods.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::dummy_name_chirho;
use super::*;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;

/// Test: user-defined class declaration registers in class env.
/// Haskell: `class MyEq a where myEq :: a -> a -> Bool`
#[test]
fn infer_user_class_decl_chirho() {
    use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("UserClass"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("MyEqChirho"),
            type_vars_chirho: vec![dummy_name_chirho("a").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: dummy_name_chirho("myEqChirho"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Bool"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "User class decl should not error: {:?}",
        result_chirho.diagnostics_chirho
    );
    // Class should be registered
    assert!(
        result_chirho
            .class_env_chirho
            .has_class_chirho("MyEqChirho"),
        "MyEqChirho should be in class env"
    );
    // Method should be in the type environment
    assert!(
        result_chirho
            .env_chirho
            .lookup_chirho("myEqChirho")
            .is_some(),
        "myEqChirho method should be in type env"
    );
}

/// Test: user-defined class with superclass context.
/// Haskell: `class Eq a => Ord a where compare :: a -> a -> Ordering`
#[test]
fn infer_user_class_with_superclass_chirho() {
    use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;
    use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("SuperClass"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![AstConstraintChirho::ClassChirho {
                kind_args_chirho: Vec::new(),
                class_chirho: dummy_name_chirho("Eq"),
                args_chirho: vec![TypeChirho::VarChirho(dummy_name_chirho("a"))],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            name_chirho: dummy_name_chirho("MyOrdChirho"),
            type_vars_chirho: vec![dummy_name_chirho("a").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: dummy_name_chirho("myCompareChirho"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                            "Ordering",
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "Class with superclass should not error: {:?}",
        result_chirho.diagnostics_chirho
    );
    assert!(
        result_chirho
            .class_env_chirho
            .has_class_chirho("MyOrdChirho")
    );
    let supers_chirho = result_chirho
        .class_env_chirho
        .superclasses_chirho("MyOrdChirho");
    assert_eq!(supers_chirho, vec!["Eq".to_string()]);
}

#[test]
fn infer_user_class_method_scheme_keeps_outer_forall_and_free_vars_chirho() {
    use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

    let pa_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let pab_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(pa_chirho),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let tp_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("t"))),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let tpa_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(tp_chirho),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let tpab_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(tpa_chirho),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("HigherRankClassScheme"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("BifunctorMonadChirho"),
            type_vars_chirho: vec![dummy_name_chirho("t").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: dummy_name_chirho("bireturnChirho"),
                ty_chirho: TypeChirho::ForallChirho {
                    vars_chirho: vec![dummy_name_chirho("a").into(), dummy_name_chirho("b").into()],
                    body_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(pab_chirho),
                        mult_chirho: None,
                        result_chirho: Box::new(tpab_chirho),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "higher-rank class method scheme should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );

    let class_decl_chirho = result_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("BifunctorMonadChirho")
        .expect("class should be registered");
    let scheme_chirho = class_decl_chirho
        .methods_chirho
        .get("bireturnChirho")
        .expect("method scheme should be registered");

    assert_eq!(
        scheme_chirho.vars_chirho.len(),
        4,
        "scheme should quantify a, b, p, and t: {scheme_chirho}"
    );
    assert!(
        scheme_chirho
            .preds_chirho
            .iter()
            .any(|pred_chirho| pred_chirho.class_name_chirho == "BifunctorMonadChirho"),
        "scheme should retain the enclosing class predicate: {scheme_chirho}"
    );
}

#[test]
fn infer_user_class_method_scheme_through_type_alias_keeps_free_vars_chirho() {
    use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

    let p_to_t_p_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(":->"))),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("t"))),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("HigherRankAliasClassScheme"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::TypeAliasDeclChirho {
                name_chirho: dummy_name_chirho(":->"),
                type_vars_chirho: vec![
                    dummy_name_chirho("p").into(),
                    dummy_name_chirho("q").into(),
                ],
                rhs_chirho: TypeChirho::ForallChirho {
                    vars_chirho: vec![dummy_name_chirho("a").into(), dummy_name_chirho("b").into()],
                    body_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("q"))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::ClassDeclChirho {
                standalone_kind_chirho: None,
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("BifunctorMonadChirho"),
                type_vars_chirho: vec![dummy_name_chirho("t").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("bireturnChirho"),
                    ty_chirho: p_to_t_p_chirho,
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "alias-based class method scheme should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );

    let class_decl_chirho = result_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("BifunctorMonadChirho")
        .expect("class should be registered");
    let scheme_chirho = class_decl_chirho
        .methods_chirho
        .get("bireturnChirho")
        .expect("method scheme should be registered");

    assert_eq!(
        scheme_chirho.vars_chirho.len(),
        4,
        "alias-based scheme should quantify a, b, p, and t: {scheme_chirho}"
    );
}

#[test]
fn ast_scheme_expands_constraint_type_alias_predicates_chirho() {
    use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;

    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.register_type_synonym_chirho(
        "ShowEq".to_string(),
        vec!["a".to_string()],
        TyChirho::TupleChirho(vec![
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Show".to_string())),
                Box::new(TyChirho::ForallVarChirho("a".to_string())),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Eq".to_string())),
                Box::new(TyChirho::ForallVarChirho("a".to_string())),
            ),
        ]),
    );

    let a_ty_chirho = TypeChirho::VarChirho(dummy_name_chirho("a"));
    let sig_chirho = TypeChirho::QualChirho {
        context_chirho: vec![AstConstraintChirho::ClassChirho {
            kind_args_chirho: Vec::new(),
            class_chirho: dummy_name_chirho("ShowEq"),
            args_chirho: vec![a_ty_chirho.clone()],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        body_chirho: Box::new(TypeChirho::FunChirho {
            arg_chirho: Box::new(a_ty_chirho),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("String"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let scheme_chirho = ctx_chirho.ast_type_to_scheme_chirho(&sig_chirho);
    let pred_classes_chirho: Vec<String> = scheme_chirho
        .preds_chirho
        .iter()
        .map(|pred_chirho| pred_chirho.class_name_chirho.clone())
        .collect();

    assert_eq!(
        pred_classes_chirho,
        vec!["Show".to_string(), "Eq".to_string()],
        "constraint aliases should expand to real class predicates: {scheme_chirho}"
    );
}

#[test]
fn infer_user_class_method_scheme_keeps_applied_class_parameter_chirho() {
    use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("BiapplicativeScheme"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("BiapplicativeChirho"),
            type_vars_chirho: vec![dummy_name_chirho("p").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: dummy_name_chirho("bipureChirho"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "biapplicative class scheme should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );

    let class_decl_chirho = result_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("BiapplicativeChirho")
        .expect("class should be registered");
    let scheme_chirho = class_decl_chirho
        .methods_chirho
        .get("bipureChirho")
        .expect("method scheme should be registered");

    let result_slot_chirho = match &scheme_chirho.ty_chirho {
        TyChirho::FunChirho(_, result_chirho, _) => match result_chirho.as_ref() {
            TyChirho::FunChirho(_, result2_chirho, _) => result2_chirho.as_ref().clone(),
            other_chirho => panic!("expected second function arrow, got: {other_chirho}"),
        },
        other_chirho => panic!("expected function type, got: {other_chirho}"),
    };

    assert!(
        matches!(result_slot_chirho, TyChirho::AppChirho(_, _)),
        "method result should keep p a b application rather than collapsing to bare p: {scheme_chirho}"
    );
}

#[test]
fn instantiate_instance_method_expected_ty_keeps_tuple_application_chirho() {
    use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("BiapplicativeExpected"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::ClassDeclChirho {
            standalone_kind_chirho: None,
            context_chirho: vec![],
            name_chirho: dummy_name_chirho("BiapplicativeChirho"),
            type_vars_chirho: vec![dummy_name_chirho("p").into()],
            methods_chirho: vec![ClassMethodChirho {
                name_chirho: dummy_name_chirho("bipureChirho"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("p"))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("b"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                default_chirho: None,
                default_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            associated_tfs_chirho: vec![],
            fundeps_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let _ = ctx_chirho.infer_module_chirho(&module_chirho);
    let class_decl_chirho = ctx_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("BiapplicativeChirho")
        .cloned()
        .expect("class should be registered");
    let method_scheme_chirho = class_decl_chirho
        .methods_chirho
        .get("bipureChirho")
        .expect("method scheme should be registered");
    let expected_ty_chirho = ctx_chirho.instantiate_instance_method_expected_ty_chirho(
        &class_decl_chirho,
        method_scheme_chirho,
        &[TyChirho::ConChirho("(,)".to_string())],
    );

    let result_slot_chirho = match expected_ty_chirho {
        TyChirho::FunChirho(_, result_chirho, _) => match *result_chirho {
            TyChirho::FunChirho(_, result2_chirho, _) => *result2_chirho,
            other_chirho => panic!("expected second function arrow, got: {other_chirho}"),
        },
        other_chirho => panic!("expected function type, got: {other_chirho}"),
    };

    assert!(
        matches!(result_slot_chirho, TyChirho::AppChirho(_, _)),
        "instantiated method result should remain tuple constructor application"
    );
}

/// Test: user-defined instance declaration registers in class env.
/// Haskell: `instance Eq Int where ...`
#[test]
fn infer_user_instance_decl_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("UserInst"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::InstanceDeclChirho {
            kind_args_chirho: Vec::new(),
            context_chirho: vec![],
            class_chirho: dummy_name_chirho("Eq"),
            types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("MyType"))],
            methods_chirho: vec![],
            assoc_tf_instances_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "Instance decl should not error: {:?}",
        result_chirho.diagnostics_chirho
    );
    // The instance should now be resolvable
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("MyType".to_string()));
    assert!(
        result_chirho.class_env_chirho.entails_chirho(&pred_chirho),
        "Eq MyType should be entailed after instance registration"
    );
}

/// The AST of MethodShapeChirho.hs, written before the R1 repair. This invokes
/// the production class-method converter without introducing a parser dependency.
fn duplicate_written_method_class_chirho() -> DeclChirho {
    use haskelujah_ast_chirho::decl_chirho::ClassMethodChirho;
    use haskelujah_ast_chirho::ty_chirho::ConstraintChirho;
    let method_chirho = |name_chirho: &str, classes_chirho: &[&str]| {
        let body_chirho = TypeChirho::FunChirho {
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("aChirho"))),
            mult_chirho: None,
            result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Bool"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };
        ClassMethodChirho {
            name_chirho: dummy_name_chirho(name_chirho),
            ty_chirho: if classes_chirho.is_empty() {
                body_chirho
            } else {
                TypeChirho::QualChirho {
                    context_chirho: classes_chirho
                        .iter()
                        .map(|class_chirho| ConstraintChirho::ClassChirho {
                            class_chirho: dummy_name_chirho(class_chirho),
                            kind_args_chirho: vec![],
                            args_chirho: vec![TypeChirho::VarChirho(dummy_name_chirho("aChirho"))],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        })
                        .collect(),
                    body_chirho: Box::new(body_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }
            },
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    };
    DeclChirho::ClassDeclChirho {
        standalone_kind_chirho: None,
        context_chirho: vec![],
        name_chirho: dummy_name_chirho("OwnChirho"),
        type_vars_chirho: vec![dummy_name_chirho("aChirho").into()],
        methods_chirho: vec![
            method_chirho("duplicateWrittenChirho", &["Eq", "Eq"]),
            method_chirho("presentOwnChirho", &["OwnChirho", "OwnChirho", "Eq"]),
            method_chirho("absentOwnChirho", &["Eq", "Show", "Eq"]),
            method_chirho("simpleOwnChirho", &[]),
        ],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn class_method_conversion_retains_written_duplicates_and_only_appends_missing_own_class_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.process_class_decl_chirho(&duplicate_written_method_class_chirho());
    let class_chirho = ctx_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("OwnChirho")
        .expect("class converted");
    for (method_chirho, expected_chirho) in [
        ("duplicateWrittenChirho", vec!["Eq", "Eq", "OwnChirho"]),
        ("presentOwnChirho", vec!["OwnChirho", "OwnChirho", "Eq"]),
        ("absentOwnChirho", vec!["Eq", "Show", "Eq", "OwnChirho"]),
        ("simpleOwnChirho", vec!["OwnChirho"]),
    ] {
        let actual_chirho: Vec<_> = class_chirho.methods_chirho[method_chirho]
            .preds_chirho
            .iter()
            .map(|predicate_chirho| predicate_chirho.class_name_chirho.as_str())
            .collect();
        assert_eq!(actual_chirho, expected_chirho, "{method_chirho}");
    }
    assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(!ctx_chirho.has_non_deferrable_failure_chirho());
}

#[test]
fn method_conversion_on_soft_exhaustion_keeps_written_shape_and_only_refuses_own_survivor_chirho() {
    use haskelujah_ast_chirho::name_chirho::{
        ClassIdentityUnknownChirho, PredicateClassProofChirho,
    };
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    assert!(
        ctx_chirho
            .class_env_chirho
            .charge_predicate_metadata_chirho(
                ctx_chirho.class_env_chirho.registration_limit_chirho()
            )
    );
    ctx_chirho.process_class_decl_chirho(&duplicate_written_method_class_chirho());
    let class_chirho = ctx_chirho
        .class_env_chirho
        .classes_chirho()
        .get_chirho("OwnChirho")
        .expect("class converted");
    let written_chirho = &class_chirho.methods_chirho["presentOwnChirho"].preds_chirho;
    assert_eq!(
        written_chirho
            .iter()
            .map(|predicate_chirho| predicate_chirho.class_name_chirho.as_str())
            .collect::<Vec<_>>(),
        ["OwnChirho", "OwnChirho", "Eq"]
    );
    assert!(matches!(
        written_chirho[0].class_identity_chirho(),
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(_))
    ));
    assert_eq!(
        written_chirho[1].class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho)
    );
    assert_eq!(
        written_chirho[2].class_identity_chirho(),
        &PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::LegacyChirho)
    );
    let absent_chirho = &class_chirho.methods_chirho["absentOwnChirho"].preds_chirho;
    assert_eq!(
        absent_chirho
            .iter()
            .map(|predicate_chirho| predicate_chirho.class_name_chirho.as_str())
            .collect::<Vec<_>>(),
        ["Eq", "Show", "Eq", "OwnChirho"]
    );
    assert!(matches!(
        absent_chirho[3].class_identity_chirho(),
        PredicateClassProofChirho::UnknownChirho(ClassIdentityUnknownChirho::ResourceChirho(_))
    ));
    assert!(!ctx_chirho.diagnostics_chirho.has_errors_chirho());
    assert!(!ctx_chirho.has_non_deferrable_failure_chirho());
}
