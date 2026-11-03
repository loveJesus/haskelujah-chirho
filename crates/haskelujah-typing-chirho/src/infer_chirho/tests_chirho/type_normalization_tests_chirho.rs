// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Normalization: synonyms exposed by reduction, associated families and the primitive state in unification.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::dummy_name_chirho;
use super::*;
use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, DeclChirho};
use haskelujah_ast_chirho::expr_chirho::RhsChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;

#[test]
fn normalize_ty_expands_synonym_exposed_by_family_reduction_chirho() {
    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();
    // Use the production declaration prepass: a raw result spelling alone is
    // not nominal proof, even when this low-level fixture writes the family.
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("SynonymResultFixtureChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::DataDeclChirho {
            name_chirho: dummy_name_chirho("GRep'"),
            type_vars_chirho: vec![dummy_name_chirho("fChirho").into()],
            constructors_chirho: vec![
                haskelujah_ast_chirho::decl_chirho::ConDeclChirho::OrdinaryChirho {
                    name_chirho: dummy_name_chirho("MkGRepChirho"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
            deriving_chirho: vec![],
            kind_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
    ctx_chirho.register_type_synonym_chirho(
        "GRep".to_string(),
        vec!["f".to_string()],
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("GRep'".to_string())),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep1".to_string())),
                Box::new(TyChirho::ForallVarChirho("f".to_string())),
            )),
        ),
    );
    ctx_chirho.register_open_family_chirho("Rep".to_owned(), 1, SpanChirho::DUMMY_CHIRHO);
    ctx_chirho.register_type_family_instance_chirho(
        "Rep".to_string(),
        vec![TyChirho::ForallVarChirho("f".to_string())],
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("GRep".to_string())),
            Box::new(TyChirho::ForallVarChirho("f".to_string())),
        ),
    );

    assert_eq!(
        ctx_chirho.reduce_type_family_chirho("Rep", &[TyChirho::ConChirho("Proxy".to_owned())]),
        Some(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("GRep'".to_owned())),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep1".to_owned())),
                Box::new(TyChirho::ConChirho("Proxy".to_owned())),
            )),
        )),
        "the declared result datatype remains reducible after synonym expansion"
    );

    let normalized_chirho = ctx_chirho.normalize_ty_chirho(&TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Rep".to_string())),
        Box::new(TyChirho::ConChirho("Proxy".to_string())),
    ));

    assert_eq!(
        normalized_chirho,
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("GRep'".to_string())),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep1".to_string())),
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
            )),
        ),
        "normalization should keep expanding synonyms that appear after family reduction"
    );
}

#[test]
fn normalize_ty_expands_ghc_generics_metadata_aliases_chirho() {
    let ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();
    let normalized_d1_chirho = ctx_chirho.normalize_ty_chirho(&TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("D1".to_string())),
                Box::new(TyChirho::ConChirho("MetaData".to_string())),
            )),
            Box::new(TyChirho::ConChirho("Proxy".to_string())),
        )),
        Box::new(TyChirho::ConChirho("Int".to_string())),
    ));
    let expected_d1_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("M1".to_string())),
                    Box::new(TyChirho::ConChirho("D".to_string())),
                )),
                Box::new(TyChirho::ConChirho("MetaData".to_string())),
            )),
            Box::new(TyChirho::ConChirho("Proxy".to_string())),
        )),
        Box::new(TyChirho::ConChirho("Int".to_string())),
    );

    assert_eq!(
        normalized_d1_chirho, expected_d1_chirho,
        "normalization should reduce D1 applications to the corresponding M1 metadata form"
    );
}

#[test]
fn instance_method_expected_ty_reduces_associated_family_rhs_chirho() {
    let product_head_ast_chirho = TypeChirho::AppChirho {
        fun_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Product"))),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: dummy_name_chirho("Representable"),
        type_vars_chirho: vec![dummy_name_chirho("f").into()],
        methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
            name_chirho: dummy_name_chirho("tabulate"),
            ty_chirho: TypeChirho::FunChirho {
                arg_chirho: Box::new(TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                mult_chirho: None,
                result_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
            name_chirho: dummy_name_chirho("Rep"),
            type_vars_chirho: vec![dummy_name_chirho("f")],
            data_family_chirho: false,
            default_rhs_chirho: None,
            default_params_chirho: vec![],
            default_binders_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let instance_decl_ast_chirho = DeclChirho::InstanceDeclChirho {
        context_chirho: vec![],
        class_chirho: dummy_name_chirho("Representable"),
        types_chirho: vec![product_head_ast_chirho.clone()],
        methods_chirho: vec![],
        assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
            family_name_chirho: dummy_name_chirho("Rep"),
            lhs_types_chirho: vec![product_head_ast_chirho],
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

    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();
    // The real module prepass binds this class's Rep before converting method
    // schemes; GHC.Generics.Rep is a different family with the same bare spelling.
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("RepresentableFixtureChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            class_decl_ast_chirho.clone(),
            instance_decl_ast_chirho.clone(),
        ],
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    };
    ctx_chirho.prepare_family_scope_chirho(&module_chirho, None);
    ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);
    ctx_chirho.process_instance_decl_chirho(&instance_decl_ast_chirho);
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "{}",
        ctx_chirho.diagnostics_chirho
    );

    let class_decl_chirho = ctx_chirho
        .class_env_chirho
        .classes_chirho
        .get("Representable")
        .cloned()
        .expect("class should register");
    let method_scheme_chirho = class_decl_chirho
        .methods_chirho
        .get("tabulate")
        .expect("method scheme should register");
    let normalized_expected_ty_chirho = ctx_chirho.instantiate_instance_method_expected_ty_chirho(
        &class_decl_chirho,
        method_scheme_chirho,
        &[TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Product".to_string())),
                Box::new(TyChirho::ConChirho("Proxy".to_string())),
            )),
            Box::new(TyChirho::ConChirho("U1".to_string())),
        )],
    );

    match normalized_expected_ty_chirho {
        TyChirho::FunChirho(arg_chirho, result_chirho, _) => {
            match *arg_chirho {
                TyChirho::FunChirho(_, inner_result_chirho, _) => {
                    assert!(
                        matches!(*inner_result_chirho, TyChirho::TupleChirho(ref elems_chirho) if elems_chirho.len() == 2),
                        "tabulate argument should return a 2-tuple after associated family reduction; got {inner_result_chirho}; {}",
                        ctx_chirho.diagnostics_chirho
                    );
                }
                other_chirho => panic!("expected function argument, got {:?}", other_chirho),
            }
            assert!(
                matches!(*result_chirho, TyChirho::AppChirho(_, _)),
                "tabulate result should remain applied Product"
            );
        }
        other_chirho => panic!("expected function type, got {:?}", other_chirho),
    }
}

#[test]
fn infer_proxy_index_instance_method_reduces_rep_to_unit_chirho() {
    use haskelujah_ast_chirho::decl_chirho::{AssocTfInstanceChirho, ClassMethodChirho};
    use haskelujah_ast_chirho::expr_chirho::LocalBindChirho;

    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("RepresentableProxyMethod"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![
            DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: dummy_name_chirho("Representable"),
                type_vars_chirho: vec![dummy_name_chirho("f").into()],
                methods_chirho: vec![ClassMethodChirho {
                    name_chirho: dummy_name_chirho("index"),
                    ty_chirho: TypeChirho::FunChirho {
                        arg_chirho: Box::new(TypeChirho::AppChirho {
                            fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        mult_chirho: None,
                        result_chirho: Box::new(TypeChirho::FunChirho {
                            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                            mult_chirho: None,
                            result_chirho: Box::new(TypeChirho::AppChirho {
                                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho(
                                    "Rep",
                                ))),
                                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
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
                associated_tfs_chirho: vec![
                    haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
                        name_chirho: dummy_name_chirho("Rep"),
                        type_vars_chirho: vec![dummy_name_chirho("f")],
                        data_family_chirho: false,
                        default_rhs_chirho: None,
                        default_params_chirho: vec![],
                        default_binders_chirho: None,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            DeclChirho::InstanceDeclChirho {
                context_chirho: vec![],
                class_chirho: dummy_name_chirho("Representable"),
                types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
                methods_chirho: vec![LocalBindChirho::FunBindChirho {
                    name_chirho: dummy_name_chirho("index"),
                    matches_chirho: vec![MatchArmChirho {
                        pats_chirho: vec![
                            PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                            PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                        ],
                        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::TupleChirho {
                            elements_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        }),
                        where_binds_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                assoc_tf_instances_chirho: vec![AssocTfInstanceChirho {
                    family_name_chirho: dummy_name_chirho("Rep"),
                    lhs_types_chirho: vec![TypeChirho::ConChirho(dummy_name_chirho("Proxy"))],
                    rhs_chirho: TypeChirho::TupleChirho {
                        elements_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
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
        "Proxy index instance should typecheck once Rep Proxy reduces to (): {:?}",
        result_chirho.diagnostics_chirho
    );
}

#[test]
fn collect_free_type_vars_walks_parenthesized_assoc_family_lhs_chirho() {
    let vars_chirho = collect_free_type_vars_from_ast_chirho(&[TypeChirho::ParenChirho {
        inner_chirho: Box::new(TypeChirho::AppChirho {
            fun_chirho: Box::new(TypeChirho::AppChirho {
                fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Product"))),
                arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("g"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }]);
    assert_eq!(vars_chirho, vec!["f".to_string(), "g".to_string()]);
}

#[test]
fn infer_composition_with_tuple_family_projection_preserves_applied_rep_chirho() {
    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();
    let arg_ty_chirho = ctx_chirho.fresh_var_chirho();
    let h_ty_chirho = TyChirho::fun_chirho(
        arg_ty_chirho,
        TyChirho::TupleChirho(vec![
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep".to_string())),
                Box::new(TyChirho::ForallVarChirho("f".to_string())),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Rep".to_string())),
                Box::new(TyChirho::ForallVarChirho("g".to_string())),
            ),
        ]),
    );
    ctx_chirho
        .env_chirho
        .bind_chirho("h".to_string(), SchemeChirho::mono_chirho(h_ty_chirho));
    let expr_chirho = ExprChirho::InfixChirho {
        left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("fst"))),
        op_chirho: dummy_name_chirho("."),
        right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("h"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);

    match final_ty_chirho {
        TyChirho::FunChirho(_, result_chirho, _) => {
            assert_eq!(
                *result_chirho,
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("f".to_string()))
                )
            );
        }
        other_chirho => panic!("expected composed function type, got {:?}", other_chirho),
    }
}

#[test]
fn infer_tabulate_application_after_tuple_projection_keeps_component_functor_chirho() {
    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();
    let class_decl_ast_chirho = DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: dummy_name_chirho("Representable"),
        type_vars_chirho: vec![dummy_name_chirho("f").into()],
        methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
            name_chirho: dummy_name_chirho("tabulate"),
            ty_chirho: TypeChirho::FunChirho {
                arg_chirho: Box::new(TypeChirho::FunChirho {
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    mult_chirho: None,
                    result_chirho: Box::new(TypeChirho::AppChirho {
                        fun_chirho: Box::new(TypeChirho::ConChirho(dummy_name_chirho("Rep"))),
                        arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                mult_chirho: None,
                result_chirho: Box::new(TypeChirho::AppChirho {
                    fun_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("f"))),
                    arg_chirho: Box::new(TypeChirho::VarChirho(dummy_name_chirho("a"))),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
            name_chirho: dummy_name_chirho("Rep"),
            type_vars_chirho: vec![dummy_name_chirho("f")],
            data_family_chirho: false,
            default_rhs_chirho: None,
            default_params_chirho: vec![],
            default_binders_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    ctx_chirho.process_class_decl_chirho(&class_decl_ast_chirho);

    let h_arg_ty_chirho = ctx_chirho.fresh_var_chirho();
    ctx_chirho.env_chirho.bind_chirho(
        "h".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            h_arg_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("f".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Rep".to_string())),
                    Box::new(TyChirho::ForallVarChirho("g".to_string())),
                ),
            ]),
        )),
    );

    let projected_fun_chirho = ExprChirho::InfixChirho {
        left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("fst"))),
        op_chirho: dummy_name_chirho("."),
        right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("h"))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let expr_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("tabulate"))),
        arg_chirho: Box::new(projected_fun_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    let final_ty_chirho = ctx_chirho
        .reduce_type_families_in_ty_chirho(&subst_chirho.apply_ty_chirho(&inferred_ty_chirho));

    assert!(
        matches!(final_ty_chirho, TyChirho::AppChirho(_, _)),
        "tabulate (fst . h) should infer an applied component functor, got {:?}",
        final_ty_chirho
    );
}

#[test]
fn unify_normalized_reduces_primstate_before_occurs_check_chirho() {
    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();

    let state_var_ty_chirho = ctx_chirho.fresh_var_chirho();
    let prim_state_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("PrimState".to_string())),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("ST".to_string())),
            Box::new(state_var_ty_chirho.clone()),
        )),
    );

    let subst_chirho = ctx_chirho
        .unify_normalized_chirho(
            &state_var_ty_chirho,
            &prim_state_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        )
        .expect("PrimState (ST t) should reduce before the occurs check fires");

    assert!(
        subst_chirho.is_empty_chirho(),
        "reducing PrimState (ST t) should make the unification trivial"
    );
    assert_eq!(
        ctx_chirho.normalize_ty_chirho(&state_var_ty_chirho),
        ctx_chirho.normalize_ty_chirho(&prim_state_ty_chirho),
        "PrimState (ST t) should normalize to t before raw unification"
    );
}

#[test]
fn app_unification_renormalizes_after_head_substitution_for_primstate_chirho() {
    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();

    let monad_ty_chirho = ctx_chirho.fresh_var_chirho();
    let elem_ty_chirho = ctx_chirho.fresh_var_chirho();
    let expected_ty_chirho = TyChirho::AppChirho(
        Box::new(monad_ty_chirho.clone()),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("PrimState".to_string())),
                    Box::new(monad_ty_chirho.clone()),
                )),
            )),
            Box::new(elem_ty_chirho.clone()),
        )),
    );
    let actual_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("ST".to_string())),
            Box::new(TyChirho::ConChirho("S0".to_string())),
        )),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                Box::new(TyChirho::ConChirho("S0".to_string())),
            )),
            Box::new(TyChirho::ConChirho("Int".to_string())),
        )),
    );

    let subst_chirho = ctx_chirho
        .unify_normalized_chirho(
            &expected_ty_chirho,
            &actual_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        )
        .expect(
            "head substitutions should unlock PrimState reductions inside application arguments",
        );

    let expected_normalized_chirho =
        ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&expected_ty_chirho));
    let actual_normalized_chirho =
        ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&actual_ty_chirho));

    assert_eq!(
        expected_normalized_chirho, actual_normalized_chirho,
        "application unification should re-normalize after substitutions expose PrimState equations"
    );
}

#[test]
fn unify_normalized_instantiates_forall_before_reducing_primstate_chirho() {
    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();

    let monad_ty_chirho = ctx_chirho.fresh_var_chirho();
    let elem_ty_chirho = TyChirho::ConChirho("Int".to_string());
    let actual_ty_chirho = TyChirho::AppChirho(
        Box::new(monad_ty_chirho.clone()),
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("PrimState".to_string())),
                    Box::new(monad_ty_chirho.clone()),
                )),
            )),
            Box::new(elem_ty_chirho.clone()),
        )),
    );
    let state_var_chirho = TyVarChirho(8000);
    let expected_ty_chirho = TyChirho::ForallChirho {
        vars_chirho: vec![state_var_chirho],
        body_chirho: Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(state_var_chirho)),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("MutableArray".to_string())),
                    Box::new(TyChirho::VarChirho(state_var_chirho)),
                )),
                Box::new(elem_ty_chirho.clone()),
            )),
        )),
    };

    let subst_chirho = ctx_chirho
        .unify_normalized_chirho(
            &actual_ty_chirho,
            &expected_ty_chirho,
            SpanChirho::DUMMY_CHIRHO,
        )
        .expect("normalized unification should instantiate forall results before PrimState occurs checks");

    let actual_normalized_chirho =
        ctx_chirho.normalize_ty_chirho(&subst_chirho.apply_ty_chirho(&actual_ty_chirho));
    match actual_normalized_chirho {
        TyChirho::AppChirho(fun_chirho, result_chirho) => match (*fun_chirho, *result_chirho) {
            (
                TyChirho::AppChirho(st_head_chirho, state_arg_chirho),
                TyChirho::AppChirho(mutable_array_head_chirho, elem_arg_chirho),
            ) => {
                assert_eq!(
                    *st_head_chirho,
                    TyChirho::ConChirho("ST".to_string()),
                    "the monad should specialize to ST after instantiating the forall result"
                );
                match (*mutable_array_head_chirho, *elem_arg_chirho) {
                    (
                        TyChirho::AppChirho(mutable_array_con_chirho, mutable_state_chirho),
                        TyChirho::ConChirho(elem_name_chirho),
                    ) => {
                        assert_eq!(
                            *mutable_array_con_chirho,
                            TyChirho::ConChirho("MutableArray".to_string()),
                            "the result should stay a MutableArray after reduction"
                        );
                        assert_eq!(
                            elem_name_chirho, "Int",
                            "the element type should remain intact through the PrimState reduction"
                        );
                        assert_eq!(
                            *mutable_state_chirho, *state_arg_chirho,
                            "the MutableArray state should match the instantiated ST state variable"
                        );
                    }
                    other_chirho => panic!(
                        "expected ST result to produce a MutableArray state argument, got {:?}",
                        other_chirho
                    ),
                }
            }
            other_chirho => panic!(
                "expected normalized actual type to specialize to ST s (MutableArray s Int), got {:?}",
                other_chirho
            ),
        },
        other_chirho => panic!(
            "expected normalized actual type to remain an application, got {:?}",
            other_chirho
        ),
    }
}

#[test]
fn synonym_expansion_and_associated_substitution_preserve_arrow_multiplicity_chirho() {
    let mut ctx_chirho = super::test_support_chirho::family_fixture_context_chirho();
    for mult_chirho in [
        crate::ty_chirho::MultChirho::OneChirho,
        crate::ty_chirho::MultChirho::ManyChirho,
    ] {
        let variable_chirho = TyChirho::ForallVarChirho("aChirho".to_owned());
        let rhs_chirho = TyChirho::FunChirho(
            Box::new(variable_chirho.clone()),
            Box::new(variable_chirho),
            mult_chirho,
        );
        ctx_chirho.register_type_synonym_chirho(
            "IdentityArrowChirho".to_owned(),
            vec!["aChirho".to_owned()],
            rhs_chirho.clone(),
        );
        let expected_chirho = TyChirho::FunChirho(
            Box::new(TyChirho::int_chirho()),
            Box::new(TyChirho::int_chirho()),
            mult_chirho,
        );
        assert_eq!(
            ctx_chirho.normalize_ty_chirho(&TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("IdentityArrowChirho".to_owned())),
                Box::new(TyChirho::int_chirho())
            )),
            expected_chirho
        );
        assert_eq!(
            super::substitute_type_vars_chirho(
                &rhs_chirho,
                &HashMap::from([("aChirho".to_owned(), TyChirho::int_chirho())])
            ),
            expected_chirho
        );
    }
}
