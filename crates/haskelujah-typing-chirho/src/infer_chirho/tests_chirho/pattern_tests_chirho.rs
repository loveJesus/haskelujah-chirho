// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Patterns: tuples, lists, records, constructors, negations and magic-hash binders.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::dummy_name_chirho;
use super::*;
use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::{
    AltChirho, GuardedExprChirho, LocalBindChirho, RhsChirho, StmtChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::pat_chirho::PatFieldChirho;

#[test]
fn infer_case_unboxed_tuple_hash_primed_binders_stay_in_scope_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.env_chirho.bind_chirho(
        "foo#".to_string(),
        SchemeChirho::mono_chirho(TyChirho::TupleChirho(vec![
            TyChirho::ConChirho("State#".to_string()),
            TyChirho::ConChirho("ByteArray#".to_string()),
        ])),
    );
    ctx_chirho.env_chirho.bind_chirho(
        "ByteArray".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("ByteArray#".to_string()),
            TyChirho::ConChirho("ByteArray".to_string()),
        )),
    );

    let expr_chirho = ExprChirho::CaseChirho {
        scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("foo#"))),
        alts_chirho: vec![AltChirho {
            pat_chirho: PatChirho::TupleChirho {
                elements_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("s'#")),
                    PatChirho::VarChirho(dummy_name_chirho("arr'#")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::TupleChirho {
                elements_chirho: vec![
                    ExprChirho::VarChirho(dummy_name_chirho("s'#")),
                    ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("ByteArray"))),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("arr'#"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (_subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "hash-primed unboxed-tuple binders should stay in scope inside the case rhs: {:?}",
        ctx_chirho.diagnostics_chirho
    );
    assert_eq!(
        inferred_ty_chirho,
        TyChirho::TupleChirho(vec![
            TyChirho::ConChirho("State#".to_string()),
            TyChirho::ConChirho("ByteArray".to_string()),
        ]),
        "the rhs should see both s'# and arr'# as locally bound names"
    );
}

#[test]
fn infer_top_level_magic_hash_pattern_binder_survives_unsafe_coerce_rhs_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.env_chirho.bind_chirho(
        "primitive".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::VarChirho(TyVarChirho(9100)),
            TyChirho::VarChirho(TyVarChirho(9101)),
        )),
    );

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("PrimitiveMiniChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("ByteArray"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: dummy_name_chirho("ByteArray"),
                    fields_chirho: vec![(
                        haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        TypeChirho::ConChirho(dummy_name_chirho("ByteArray#")),
                    )],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("MutableByteArray"),
                type_vars_chirho: vec![dummy_name_chirho("s").into()],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: dummy_name_chirho("MutableByteArray"),
                    fields_chirho: vec![(
                        haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        TypeChirho::ConChirho(dummy_name_chirho("ByteArray#")),
                    )],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("unsafeThawByteArray"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::ConChirho {
                        con_chirho: dummy_name_chirho("ByteArray"),
                        args_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("arr#"))],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("primitive"))),
                        arg_chirho: Box::new(ExprChirho::ParenChirho {
                            inner_chirho: Box::new(ExprChirho::LamChirho {
                                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("s#"))],
                                body_chirho: Box::new(ExprChirho::TupleChirho {
                                    elements_chirho: vec![
                                        ExprChirho::VarChirho(dummy_name_chirho("s#")),
                                        ExprChirho::AppChirho {
                                            fun_chirho: Box::new(ExprChirho::ConChirho(
                                                dummy_name_chirho("MutableByteArray"),
                                            )),
                                            arg_chirho: Box::new(ExprChirho::ParenChirho {
                                                inner_chirho: Box::new(ExprChirho::AppChirho {
                                                    fun_chirho: Box::new(ExprChirho::VarChirho(
                                                        dummy_name_chirho("unsafeCoerce#"),
                                                    )),
                                                    arg_chirho: Box::new(ExprChirho::VarChirho(
                                                        dummy_name_chirho("arr#"),
                                                    )),
                                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                                }),
                                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                                            }),
                                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                                        },
                                    ],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
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

    let _subst_chirho = ctx_chirho.infer_module_chirho(&module_chirho);
    assert!(
        !ctx_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .iter()
            .any(|diag_chirho| diag_chirho
                .message_chirho
                .contains("unbound variable: `arr`")),
        "top-level hash binder should not degrade to bare arr in the rhs: {:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn infer_view_pattern_expression_keeps_num_predicate_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("ViewPattern"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("doubleChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("fChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::ViewChirho {
                        expr_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                            "doubleChirho",
                        ))),
                        pat_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("n"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("n"),
                    )),
                    where_binds_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        extensions_chirho: vec!["ViewPatterns".to_string()],
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
        "view pattern should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );
    let scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("fChirho")
        .expect("fChirho should be in environment");
    assert!(
        scheme_chirho
            .preds_chirho
            .iter()
            .any(|pred_chirho| pred_chirho.class_name_chirho == "Num"),
        "view pattern expression should keep Num predicate, got: {scheme_chirho}"
    );
}

/// Test: case expression with infix cons pattern `(x:xs)` binds both vars.
/// Haskell: `headChirho xs = case xs of { (y:ys) -> y }`
#[test]
fn infer_case_infix_con_pattern_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("InfixConPat"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("headChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("xs"))],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xs"))),
                    alts_chirho: vec![AltChirho {
                        pat_chirho: PatChirho::InfixConChirho {
                            left_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("y"))),
                            op_chirho: dummy_name_chirho(":"),
                            right_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho("ys"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("y"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
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
        "headChirho with infix con pattern should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );
    let scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("headChirho")
        .expect("headChirho should be in environment");
    // headChirho :: a -> b  (since we don't yet unify cons pattern args with list)
    // The key check: no errors — `y` was bound by bind_pat_chirho
    assert!(!format!("{scheme_chirho}").is_empty());
}

#[test]
fn infer_custom_infix_constructor_pattern_uses_constructor_scheme_chirho() {
    let module_chirho =
        ModuleChirho {
            name_chirho: dummy_name_chirho("StrictPairPattern"),
            exports_chirho: None,
            imports_chirho: vec![],
            decls_chirho: vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: dummy_name_chirho("StrictPair"),
                    type_vars_chirho: vec![
                        dummy_name_chirho("a").into(),
                        dummy_name_chirho("b").into(),
                    ],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: dummy_name_chirho(":*:"),
                        fields_chirho: vec![
                            (
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                                TypeChirho::VarChirho(dummy_name_chirho("a")),
                            ),
                            (
                                haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                                TypeChirho::VarChirho(dummy_name_chirho("b")),
                            ),
                        ],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    kind_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("firstBoolChirho"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("pairChirho"))],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                            scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "pairChirho",
                            ))),
                            alts_chirho: vec![AltChirho {
                                pat_chirho: PatChirho::InfixConChirho {
                                    left_chirho: Box::new(PatChirho::VarChirho(dummy_name_chirho(
                                        "flagChirho",
                                    ))),
                                    op_chirho: dummy_name_chirho(":*:"),
                                    right_chirho: Box::new(PatChirho::WildcardChirho(
                                        SpanChirho::DUMMY_CHIRHO,
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                },
                                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::IfChirho {
                                    cond_chirho: Box::new(ExprChirho::VarChirho(
                                        dummy_name_chirho("flagChirho"),
                                    )),
                                    then_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    else_chirho: Box::new(ExprChirho::LitChirho(
                                        LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                                    )),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                where_binds_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
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
        "custom infix constructor patterns should use the constructor scheme: {:?}",
        result_chirho.diagnostics_chirho
    );
}

/// Test: case expression with list pattern `[a, b]` binds elements.
/// Haskell: `sumTwoChirho xs = case xs of { [a, b] -> a }`
#[test]
fn infer_case_list_pattern_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("ListPat"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("sumTwoChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("xs"))],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xs"))),
                    alts_chirho: vec![AltChirho {
                        pat_chirho: PatChirho::ListChirho {
                            elements_chirho: vec![
                                PatChirho::VarChirho(dummy_name_chirho("a")),
                                PatChirho::VarChirho(dummy_name_chirho("b")),
                            ],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("a"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
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
        "sumTwoChirho with list pattern should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );
    result_chirho
        .env_chirho
        .lookup_chirho("sumTwoChirho")
        .expect("sumTwoChirho should be in environment");
}

/// Test: case expression with record pattern `Foo { bar = x }` binds x.
/// Haskell: `getBarChirho v = case v of { Foo { bar = x } -> x }`
#[test]
fn infer_case_record_pattern_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("RecPat"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("getBarChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("v"))],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("v"))),
                    alts_chirho: vec![AltChirho {
                        pat_chirho: PatChirho::RecordChirho {
                            has_wildcard_chirho: false,
                            con_chirho: dummy_name_chirho("Foo"),
                            fields_chirho: vec![PatFieldChirho {
                                name_chirho: dummy_name_chirho("bar"),
                                pattern_chirho: PatChirho::VarChirho(dummy_name_chirho("x")),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                            dummy_name_chirho("x"),
                        )),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
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
        "getBarChirho with record pattern should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );
    result_chirho
        .env_chirho
        .lookup_chirho("getBarChirho")
        .expect("getBarChirho should be in environment");
}

/// Test: negated literal pattern `(-1)` doesn't introduce new bindings.
/// Haskell: `isNegOneChirho x = case x of { (-1) -> 1; _ -> 0 }`
#[test]
fn infer_case_neg_pattern_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("NegPat"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("isNegOneChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                    alts_chirho: vec![
                        AltChirho {
                            pat_chirho: PatChirho::NegChirho {
                                lit_chirho: LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            },
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                                LitChirho::IntChirho(1, SpanChirho::DUMMY_CHIRHO, None),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        AltChirho {
                            pat_chirho: PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LitChirho(
                                LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None),
                            )),
                            where_binds_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
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
        "isNegOneChirho with neg pattern should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );
    let scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("isNegOneChirho")
        .expect("isNegOneChirho should be in environment");
    // isNegOneChirho should have a function type. With per-equation fresh
    // result types (for GADT support), the overall result stays as a fresh
    // variable, so we verify the function shape rather than the exact
    // return type.
    let resolved_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&scheme_chirho.ty_chirho);
    assert!(
        matches!(&resolved_ty_chirho, TyChirho::FunChirho(..)),
        "expected function type, got: {}",
        resolved_ty_chirho
    );
}

#[test]
fn infer_zero_arity_match_against_tuple_constructor_expected_ty_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let a_var_chirho = ctx_chirho.fresh_var_chirho();
    let b_var_chirho = ctx_chirho.fresh_var_chirho();
    let expected_ty_chirho = TyChirho::fun_n_chirho(
        vec![a_var_chirho.clone(), b_var_chirho.clone()],
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("(,)".to_string())),
                Box::new(a_var_chirho),
            )),
            Box::new(b_var_chirho),
        ),
    );
    let match_arm_chirho = MatchArmChirho {
        pats_chirho: vec![],
        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::LamChirho {
            pats_chirho: vec![
                PatChirho::VarChirho(dummy_name_chirho("xChirho")),
                PatChirho::VarChirho(dummy_name_chirho("yChirho")),
            ],
            body_chirho: Box::new(ExprChirho::TupleChirho {
                elements_chirho: vec![
                    ExprChirho::VarChirho(dummy_name_chirho("xChirho")),
                    ExprChirho::VarChirho(dummy_name_chirho("yChirho")),
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        where_binds_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (_subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_matches_against_expected_chirho(
        &[match_arm_chirho],
        SpanChirho::DUMMY_CHIRHO,
        &expected_ty_chirho,
    );

    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "zero-arity tuple-constructor matches should infer against the expected function type: {:?}",
        ctx_chirho.diagnostics_chirho
    );
    assert!(
        matches!(inferred_ty_chirho, TyChirho::FunChirho(_, _, _)),
        "expected function result, got: {inferred_ty_chirho}"
    );
}

#[test]
fn infer_guarded_match_with_where_keeps_lhs_params_in_scope_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let match_arm_chirho = MatchArmChirho {
        pats_chirho: vec![
            PatChirho::VarChirho(dummy_name_chirho("aChirho")),
            PatChirho::VarChirho(dummy_name_chirho("bChirho")),
        ],
        rhs_chirho: RhsChirho::GuardedChirho(vec![GuardedExprChirho {
            quals_chirho: vec![StmtChirho::expr_stmt_chirho(ExprChirho::VarChirho(
                dummy_name_chirho("otherwise"),
            ))],
            body_chirho: ExprChirho::InfixChirho {
                left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("aChirho"))),
                op_chirho: dummy_name_chirho("+"),
                right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("rChirho"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]),
        where_binds_chirho: vec![LocalBindChirho::PatBindChirho {
            pat_chirho: PatChirho::VarChirho(dummy_name_chirho("rChirho")),
            rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho(
                "bChirho",
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let expected_ty_chirho = TyChirho::fun_n_chirho(
        vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
        TyChirho::int_chirho(),
    );

    let (_subst_chirho, _inferred_ty_chirho) = ctx_chirho.infer_matches_against_expected_chirho(
        &[match_arm_chirho],
        SpanChirho::DUMMY_CHIRHO,
        &expected_ty_chirho,
    );

    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "guarded matches with trailing where bindings should keep lhs params in scope: {:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn infer_tuple_pattern_function_arg_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("TuplePatternArg"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("writerLikeChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![PatChirho::TupleChirho {
                    elements_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("aChirho")),
                        PatChirho::VarChirho(dummy_name_chirho("wChirho")),
                    ],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(dummy_name_chirho(
                    "aChirho",
                ))),
                where_binds_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
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
        "Tuple pattern arg should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );

    let writer_like_scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("writerLikeChirho")
        .expect("writerLikeChirho should be inferred");
    let writer_like_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&writer_like_scheme_chirho.ty_chirho);
    match writer_like_ty_chirho {
        TyChirho::FunChirho(arg_chirho, _result_chirho, _) => match *arg_chirho {
            TyChirho::TupleChirho(elems_chirho) => {
                assert_eq!(elems_chirho.len(), 2, "tuple arg should keep arity 2");
            }
            other_chirho => panic!("expected tuple argument, got {other_chirho}"),
        },
        other_chirho => panic!("expected function type, got {other_chirho}"),
    }
}

#[test]
fn assoc_type_family_instance_uses_lhs_free_vars_as_pattern_vars_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let product_lhs_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Product"))),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let assoc_inst_decl_chirho = DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: dummy_name_chirho("Representable"),
        types_chirho: vec![product_lhs_chirho.clone()],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
            family_name_chirho: dummy_name_chirho("Rep"),
            lhs_types_chirho: vec![product_lhs_chirho],
            rhs_chirho: TypeChirho::TupleChirho {
                elements_chirho: vec![
                    TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    ctx_chirho.process_instance_decl_chirho(&assoc_inst_decl_chirho);

    let reduced_chirho = ctx_chirho.reduce_type_family_chirho(
        "Rep",
        &[TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Product".to_string())),
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
            )),
            Box::new(TyChirho::ConChirho("U1".to_string())),
        )],
    );

    assert_eq!(
        reduced_chirho,
        Some(TyChirho::TupleChirho(vec![
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep".to_string())),
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep".to_string())),
                Box::new(TyChirho::ConChirho("U1".to_string())),
            ),
        ])),
        "associated type family instance should treat lhs free vars as pattern variables"
    );
}
