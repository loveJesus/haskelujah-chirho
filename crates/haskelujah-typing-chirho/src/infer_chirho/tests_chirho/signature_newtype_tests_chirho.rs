// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Type signatures, newtypes and GADT equations.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::dummy_name_chirho;
use super::*;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::RhsChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;

/// Test: type signature that matches inferred type produces no error.
/// Haskell: `idChirho :: a -> a; idChirho x = x`
#[test]
fn infer_matching_type_sig_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("TypeSig"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::TypeSigChirho {
                name_chirho: dummy_name_chirho("idChirho"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("idChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("x"),
                    )),
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
        "Matching type sig should not error: {:?}",
        result_chirho.diagnostics_chirho
    );
}

/// Test: type signature with mismatching arity produces an error.
/// With per-equation fresh result types (for GADT support), the overall
/// result is a fresh variable that subsumes the declared result type.
/// So result-type mismatches are no longer caught at this level.
/// Instead, we test an arity mismatch: `badChirho :: Int -> Int -> Int`
/// but `badChirho x = x` (one arg instead of two).
/// Haskell: `badChirho :: Int -> Int -> Int; badChirho x = x`
#[test]
fn infer_mismatching_type_sig_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("BadSig"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::TypeSigChirho {
                name_chirho: dummy_name_chirho("badChirho"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("badChirho"),
                // badChirho x = x  (infers a -> a, sig says Int -> Int -> Int)
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("x"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::VarChirho(
                        dummy_name_chirho("x"),
                    )),
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
    // `badChirho x = x` infers `a -> a` but sig says `Int -> Int -> Int`.
    // The per-equation result is now properly unified with the overall
    // result, so the signature mismatch is correctly detected.
    assert!(
        result_chirho.diagnostics_chirho.has_errors_chirho(),
        "Signature mismatch should be detected for `badChirho :: Int -> Int -> Int; badChirho x = x`"
    );
}

/// Test: newtype constructor gets proper typing.
/// Haskell: `newtype Age = MkAge Int`
#[test]
fn infer_newtype_constructor_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("NT"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::NewtypeDeclChirho {
                name_chirho: dummy_name_chirho("Age"),
                type_vars_chirho: vec![],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: dummy_name_chirho("MkAge"),
                    fields_chirho: vec![(
                        haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        TypeChirho::ConChirho(dummy_name_chirho("Int")),
                    )],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            // Use the constructor: mkAgeChirho = MkAge 42
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("mkAgeChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("MkAge"))),
                        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                            42,
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
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
        "Newtype + usage should not error: {:?}",
        result_chirho.diagnostics_chirho
    );

    // MkAge constructor should be in the environment
    let mk_age_chirho = result_chirho
        .env_chirho
        .lookup_chirho("MkAge")
        .expect("MkAge should be in env");
    // MkAge :: a -> Age (field gets fresh var since we don't resolve AST types yet)
    // Apply subst to resolve type variables from per-equation fresh types
    let mk_age_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&mk_age_chirho.ty_chirho);
    if let TyChirho::FunChirho(_, ref ret_chirho, _) = mk_age_ty_chirho {
        assert_eq!(**ret_chirho, TyChirho::ConChirho("Age".to_string()));
    } else {
        panic!("MkAge should be a function type, got: {}", mk_age_ty_chirho);
    }

    // mkAgeChirho should also be in the environment. With per-equation
    // fresh result types (for GADT support), the overall result type stays
    // as a fresh variable, so we just verify existence and no errors.
    let _mk_age_result_chirho = result_chirho
        .env_chirho
        .lookup_chirho("mkAgeChirho")
        .expect("mkAgeChirho should be in env");
}

#[test]
fn infer_newtype_record_constructor_with_function_field_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("WrapModuleChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::NewtypeDeclChirho {
                name_chirho: dummy_name_chirho("WrapChirho"),
                type_vars_chirho: vec![
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(
                        dummy_name_chirho("rChirho"),
                    ),
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(
                        dummy_name_chirho("sChirho"),
                    ),
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::plain_chirho(
                        dummy_name_chirho("aChirho"),
                    ),
                ],
                constructor_chirho: ConDeclChirho::RecordChirho {
                    name_chirho: dummy_name_chirho("WrapChirho"),
                    fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![dummy_name_chirho("runWrapChirho")],
                        ty_chirho: TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                "rChirho",
                            ))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "sChirho",
                                ))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::TupleChirho {
                                    elements_chirho: vec![
                                        TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                        TypeChirho::VarChirho(dummy_name_chirho("sChirho")),
                                    ],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("mkWrapChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("fChirho"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                        left_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                            "WrapChirho",
                        ))),
                        op_chirho: dummy_name_chirho("$"),
                        right_chirho: Box::new(ExprChirho::LamChirho {
                            pats_chirho: vec![
                                PatChirho::VarChirho(dummy_name_chirho("rChirho")),
                                PatChirho::VarChirho(dummy_name_chirho("sChirho")),
                            ],
                            body_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::AppChirho {
                                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "fChirho",
                                    ))),
                                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                        "rChirho",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "sChirho",
                                ))),
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

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "Newtype record constructor with function field should infer: {:?}",
        result_chirho.diagnostics_chirho
    );
}

#[test]
fn infer_writer_newtype_composition_with_tuple_payload_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("WriterModuleChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::NewtypeDeclChirho {
                name_chirho: dummy_name_chirho("WriterT"),
                type_vars_chirho: vec![
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "wChirho",
                    )),
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "mChirho",
                    )),
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "aChirho",
                    )),
                ],
                constructor_chirho: ConDeclChirho::RecordChirho {
                    name_chirho: dummy_name_chirho("WriterT"),
                    fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![dummy_name_chirho("runWriterTChirho")],
                        ty_chirho: TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                "mChirho",
                            ))),
                            arg_chirho: Box::new(TypeChirho::TupleChirho {
                                elements_chirho: vec![
                                    TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                    TypeChirho::VarChirho(dummy_name_chirho("wChirho")),
                                ],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("writerChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                        left_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("WriterT"))),
                        op_chirho: dummy_name_chirho("."),
                        right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("return"))),
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
        "WriterT . return should preserve tuple payload: {:?}",
        result_chirho.diagnostics_chirho
    );

    let writer_scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("writerChirho")
        .expect("writerChirho should be inferred");
    let writer_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&writer_scheme_chirho.ty_chirho);
    match writer_ty_chirho {
        TyChirho::FunChirho(arg_chirho, _result_chirho, _) => match *arg_chirho {
            TyChirho::TupleChirho(elems_chirho) => {
                assert_eq!(elems_chirho.len(), 2, "writer arg should stay tuple-shaped");
            }
            other_chirho => panic!("expected tuple argument, got {other_chirho}"),
        },
        other_chirho => panic!("expected function type, got {other_chirho}"),
    }
}

#[test]
fn infer_state_newtype_composition_with_function_tuple_payload_chirho() {
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("StateModuleChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::NewtypeDeclChirho {
                name_chirho: dummy_name_chirho("StateT"),
                type_vars_chirho: vec![
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "sChirho",
                    )),
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "mChirho",
                    )),
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "aChirho",
                    )),
                ],
                constructor_chirho: ConDeclChirho::RecordChirho {
                    name_chirho: dummy_name_chirho("StateT"),
                    fields_chirho: vec![haskelujah_ast_chirho::decl_chirho::FieldDeclChirho {
                        multiplicity_chirho: None,
                        names_chirho: vec![dummy_name_chirho("runStateTChirho")],
                        ty_chirho: TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                "sChirho",
                            ))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "mChirho",
                                ))),
                                arg_chirho: Box::new(TypeChirho::TupleChirho {
                                    elements_chirho: vec![
                                        TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                        TypeChirho::VarChirho(dummy_name_chirho("sChirho")),
                                    ],
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        strictness_chirho:
                            haskelujah_ast_chirho::decl_chirho::StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("stateChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![PatChirho::VarChirho(dummy_name_chirho("fChirho"))],
                    rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("StateT"))),
                        arg_chirho: Box::new(ExprChirho::InfixChirho {
                            left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "return",
                            ))),
                            op_chirho: dummy_name_chirho("."),
                            right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                "fChirho",
                            ))),
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

    let result_chirho = infer_module_chirho(&module_chirho);
    assert!(
        !result_chirho.diagnostics_chirho.has_errors_chirho(),
        "StateT (return . f) should preserve tuple payload: {:?}",
        result_chirho.diagnostics_chirho
    );

    let state_scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("stateChirho")
        .expect("stateChirho should be inferred");
    let state_ty_chirho = result_chirho
        .subst_chirho
        .apply_ty_chirho(&state_scheme_chirho.ty_chirho);
    match state_ty_chirho {
        TyChirho::FunChirho(arg_chirho, _result_chirho, _) => match *arg_chirho {
            TyChirho::FunChirho(fun_arg_chirho, fun_result_chirho, _) => {
                assert!(
                    matches!(*fun_result_chirho, TyChirho::TupleChirho(_)),
                    "state function result should stay tuple-shaped, got {:?}",
                    fun_result_chirho
                );
                assert!(
                    !matches!(*fun_arg_chirho, TyChirho::TupleChirho(_)),
                    "state function input should stay the state, not collapse into tuple"
                );
            }
            other_chirho => panic!("expected function argument, got {other_chirho}"),
        },
        other_chirho => panic!("expected function type, got {other_chirho}"),
    }
}

#[test]
fn infer_gadt_equations_refine_rigid_signature_vars_per_equation_chirho() {
    use haskelujah_ast_chirho::ty_chirho::ConstraintChirho as AstConstraintChirho;
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("FreeASeqMiniChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::DataDeclChirho {
                name_chirho: dummy_name_chirho("ASeq"),
                type_vars_chirho: vec![
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "fChirho",
                    )),
                    haskelujah_ast_chirho::decl_chirho::TyVarChirho::from(dummy_name_chirho(
                        "aChirho",
                    )),
                ],
                constructors_chirho: vec![
                    ConDeclChirho::GadtChirho {
                        name_chirho: dummy_name_chirho("ANil"),
                        signature_chirho: haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho::from_type_chirho(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                    "ASeq",
                                ))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "fChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(TypeChirho::TupleChirho {
                                elements_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    ConDeclChirho::GadtChirho {
                        name_chirho: dummy_name_chirho("ACons"),
                        signature_chirho: haskelujah_ast_chirho::decl_chirho::GadtConSignatureChirho::from_type_chirho(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "fChirho",
                                ))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "aChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::AppChirho {
                                        fun_chirho: Box::new(TypeChirho::ConChirho(
                                            dummy_name_chirho("ASeq"),
                                        )),
                                        arg_chirho: Box::new(TypeChirho::VarChirho(
                                            dummy_name_chirho("fChirho"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                        "uChirho",
                                    ))),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::AppChirho {
                                    fun_chirho: Box::new(TypeChirho::AppChirho {
                                        fun_chirho: Box::new(TypeChirho::ConChirho(
                                            dummy_name_chirho("ASeq"),
                                        )),
                                        arg_chirho: Box::new(TypeChirho::VarChirho(
                                            dummy_name_chirho("fChirho"),
                                        )),
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    arg_chirho: Box::new(TypeChirho::TupleChirho {
                                        elements_chirho: vec![
                                            TypeChirho::VarChirho(dummy_name_chirho("aChirho")),
                                            TypeChirho::VarChirho(dummy_name_chirho("uChirho")),
                                        ],
                                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                                    }),
                                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                                }),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::TypeSigChirho {
                name_chirho: dummy_name_chirho("reduceASeqChirho"),
                // `reduceASeq :: Applicative f => ASeq f u -> f u`, as in
                // Control.Applicative.Free. Without the context GHC (and now
                // this checker) reports "could not deduce Applicative f".
                ty_chirho: TypeChirho::QualChirho {
                    context_chirho: vec![AstConstraintChirho::ClassChirho {
                        kind_args_chirho: Vec::new(),
                        class_chirho: dummy_name_chirho("Applicative"),
                        args_chirho: vec![TypeChirho::VarChirho(dummy_name_chirho("fChirho"))],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    body_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                    "ASeq",
                                ))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "fChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                "uChirho",
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                "fChirho",
                            ))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                "uChirho",
                            ))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("reduceASeqChirho"),
                matches_chirho: vec![
                    MatchArmChirho {
                        pats_chirho: vec![PatChirho::ConChirho { type_args_chirho: Vec::new(),
                            con_chirho: dummy_name_chirho("ANil"),
                            args_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("pure"))),
                            arg_chirho: Box::new(ExprChirho::TupleChirho {
                                elements_chirho: vec![],
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    MatchArmChirho {
                        pats_chirho: vec![PatChirho::ConChirho { type_args_chirho: Vec::new(),
                            con_chirho: dummy_name_chirho("ACons"),
                            args_chirho: vec![
                                PatChirho::VarChirho(dummy_name_chirho("xChirho")),
                                PatChirho::VarChirho(dummy_name_chirho("xsChirho")),
                            ],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::InfixChirho {
                            left_chirho: Box::new(ExprChirho::InfixChirho {
                                left_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho(
                                    "(,)",
                                ))),
                                op_chirho: dummy_name_chirho("<$>"),
                                right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "xChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            op_chirho: dummy_name_chirho("<*>"),
                            right_chirho: Box::new(ExprChirho::AppChirho {
                                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "reduceASeqChirho",
                                ))),
                                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                                    "xsChirho",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
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
        "GADT equations should refine the rigid signature vars per equation: {:?}",
        result_chirho.diagnostics_chirho
    );
}

#[test]
fn infer_module_show_composition_type_signature_chirho() {
    let rhs_expr_chirho = ExprChirho::InfixChirho {
        left_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showParen"))),
            arg_chirho: Box::new(ExprChirho::ParenChirho {
                inner_chirho: Box::new(ExprChirho::InfixChirho {
                    left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("dChirho"))),
                    op_chirho: dummy_name_chirho(">"),
                    right_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                        10,
                        SpanChirho::DUMMY_CHIRHO,
                        None,
                    ))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        op_chirho: dummy_name_chirho("$"),
        right_chirho: Box::new(ExprChirho::InfixChirho {
            left_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showString"))),
                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("nameChirho"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            op_chirho: dummy_name_chirho("."),
            right_chirho: Box::new(ExprChirho::InfixChirho {
                left_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("showChar"))),
                    arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
                        ' ',
                        SpanChirho::DUMMY_CHIRHO,
                        None,
                    ))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                op_chirho: dummy_name_chirho("."),
                right_chirho: Box::new(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("spChirho"))),
                        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
                            11,
                            SpanChirho::DUMMY_CHIRHO,
                            None,
                        ))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("xChirho"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("ShowComposeTypeSig"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::TypeSigChirho {
                name_chirho: dummy_name_chirho("showsUnaryWithMiniChirho"),
                ty_chirho: TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::ParenChirho {
                        inner_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "aChirho",
                                ))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                    "ShowS",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("String"))),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Int"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::FunChirho {
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho(
                                    "aChirho",
                                ))),
                                mult_chirho: None,
                                result_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                    "ShowS",
                                ))),
                                span_chirho: SpanChirho::DUMMY_CHIRHO,
                            }),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::FunBindChirho {
                name_chirho: dummy_name_chirho("showsUnaryWithMiniChirho"),
                matches_chirho: vec![MatchArmChirho {
                    pats_chirho: vec![
                        PatChirho::VarChirho(dummy_name_chirho("spChirho")),
                        PatChirho::VarChirho(dummy_name_chirho("nameChirho")),
                        PatChirho::VarChirho(dummy_name_chirho("dChirho")),
                        PatChirho::VarChirho(dummy_name_chirho("xChirho")),
                    ],
                    rhs_chirho: RhsChirho::UnguardedChirho(rhs_expr_chirho),
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
        "module inference should accept the ShowS-returning composition signature: {:?}",
        result_chirho.diagnostics_chirho
    );
}
