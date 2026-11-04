// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Seeded builtins, function arities, expected-type composition and binding-group order.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::dummy_name_chirho;
use super::*;
use haskelujah_ast_chirho::expr_chirho::RhsChirho;

#[test]
fn seed_builtins_include_ghc_cstring_and_foreign_names_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    for name_chirho in [
        "GHC.CString.unpackCString#",
        "GHC.Base.unpackCStringUtf8#",
        "LittleEndian",
        "BigEndian",
        "GHC.ByteOrder.targetByteOrder",
        "byteSwap32",
        "GHC.Word.byteSwap64",
        "GHC.Foreign.withCStringLen",
        "GHC.Foreign.peekCStringLen",
        "Foreign.C.String.withCStringLen",
        "Data.Text.Array.run",
        "Data.ByteString.packCStringLen",
        "Data.ByteString.useAsCStringLen",
        "assert",
    ] {
        assert!(
            ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_some(),
            "seeded environment should include {name_chirho}"
        );
    }
}

#[test]
fn seed_builtins_include_indexed_traversable_core_methods_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    for name_chirho in [
        "imap",
        "WithIndex.imap",
        "Data.Functor.WithIndex.imap",
        "ifoldMap",
        "WithIndex.ifoldMap",
        "Data.Foldable.WithIndex.ifoldMap",
        "ifoldr",
        "WithIndex.ifoldr",
        "Data.Foldable.WithIndex.ifoldr",
        "ifoldl",
        "WithIndex.ifoldl",
        "Data.Foldable.WithIndex.ifoldl",
        "itraverse",
        "WithIndex.itraverse",
        "Data.Traversable.WithIndex.itraverse",
    ] {
        assert!(
            ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_some(),
            "seeded environment should include {name_chirho}"
        );
    }
}

#[test]
fn seed_builtins_include_data_sequence_snoc_operator_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    for name_chirho in ["|>", "Data.Sequence.|>"] {
        assert!(
            ctx_chirho.env_chirho.lookup_chirho(name_chirho).is_some(),
            "seeded environment should include {name_chirho}"
        );
    }
}

#[test]
fn seed_builtins_include_polymorphic_monadfix_method_chirho() {
    let schemes_chirho = builtin_value_schemes_chirho();
    let mfix_scheme_chirho = schemes_chirho
        .get("Control.Monad.Fix.mfix")
        .expect("qualified mfix builtin should exist");
    let mfix_m_chirho = TyVarChirho(1720);
    let mfix_a_chirho = TyVarChirho(1721);
    let mfix_result_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::VarChirho(mfix_m_chirho)),
        Box::new(TyChirho::VarChirho(mfix_a_chirho)),
    );

    assert_eq!(
        mfix_scheme_chirho,
        &SchemeChirho {
            vars_chirho: vec![mfix_m_chirho, mfix_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "MonadFix".to_string(),
                ty_chirho: TyChirho::VarChirho(mfix_m_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(mfix_a_chirho),
                    mfix_result_chirho.clone(),
                ),
                mfix_result_chirho,
            ),
        },
        "mfix should preserve both the MonadFix constructor and result element variables"
    );
}

#[test]
fn builtin_splitmix_helpers_use_word_sized_signatures_chirho() {
    let schemes_chirho = builtin_value_schemes_chirho();

    let next_word32_scheme_chirho = schemes_chirho
        .get("nextWord32")
        .expect("nextWord32 builtin should exist");
    assert_eq!(
        next_word32_scheme_chirho.ty_chirho,
        TyChirho::fun_chirho(
            TyChirho::ConChirho("SMGen".to_string()),
            TyChirho::TupleChirho(vec![
                TyChirho::ConChirho("Word32".to_string()),
                TyChirho::ConChirho("SMGen".to_string()),
            ]),
        ),
        "nextWord32 should return a Word32 and the updated generator"
    );

    let bitmask_scheme_chirho = schemes_chirho
        .get("bitmaskWithRejection64'")
        .expect("bitmaskWithRejection64' builtin should exist");
    assert_eq!(
        bitmask_scheme_chirho.ty_chirho,
        TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Word64".to_string()),
                TyChirho::ConChirho("SMGen".to_string()),
            ],
            TyChirho::TupleChirho(vec![
                TyChirho::ConChirho("Word64".to_string()),
                TyChirho::ConChirho("SMGen".to_string()),
            ]),
        ),
        "bitmaskWithRejection64' should accept a Word64 upper bound"
    );
}

#[test]
fn builtin_enum_from_to_is_enum_polymorphic_chirho() {
    let schemes_chirho = builtin_value_schemes_chirho();
    let enum_from_to_scheme_chirho = schemes_chirho
        .get("enumFromTo")
        .expect("enumFromTo builtin should exist");
    let ef_a_chirho = TyVarChirho(3268);

    assert_eq!(
        enum_from_to_scheme_chirho,
        &SchemeChirho {
            vars_chirho: vec![ef_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(ef_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(ef_a_chirho),
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(ef_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ef_a_chirho))),
                ),
            ),
        },
        "enumFromTo should work over any Enum, not just Int"
    );
}

#[test]
fn builtin_swap_supports_binary_constructor_polymorphism_chirho() {
    let schemes_chirho = builtin_value_schemes_chirho();
    let swap_scheme_chirho = schemes_chirho
        .get("swap")
        .expect("swap builtin should exist");
    let sw_p_chirho = TyVarChirho(1035);
    let sw_a_chirho = TyVarChirho(1036);
    let sw_b_chirho = TyVarChirho(1037);

    assert_eq!(
        swap_scheme_chirho,
        &SchemeChirho {
            vars_chirho: vec![sw_p_chirho, sw_a_chirho, sw_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(sw_p_chirho)),
                        Box::new(TyChirho::VarChirho(sw_a_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(sw_b_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(sw_p_chirho)),
                        Box::new(TyChirho::VarChirho(sw_b_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(sw_a_chirho)),
                ),
            ),
        },
        "swap should work over any binary constructor, not only tuples"
    );
}

#[test]
fn builtin_test_tasty_helpers_are_module_qualified_only_chirho() {
    let schemes_chirho = builtin_value_schemes_chirho();

    for bare_name_chirho in [
        "safeRead",
        "Option",
        "lookupOption",
        "defaultValue",
        "parseValue",
        "optionName",
        "optionHelp",
        "defaultMain",
        "defaultMainWithIngredients",
        "testGroup",
        "includingOptions",
    ] {
        assert!(
            !schemes_chirho.contains_key(bare_name_chirho),
            "builtin scheme seeding should not leak {bare_name_chirho} into unrelated modules"
        );
    }

    for qualified_name_chirho in [
        "Test.Tasty.Options.safeRead",
        "Test.Tasty.Options.Option",
        "Test.Tasty.Options.lookupOption",
        "Test.Tasty.Options.defaultValue",
        "Test.Tasty.Options.parseValue",
        "Test.Tasty.Options.optionName",
        "Test.Tasty.Options.optionHelp",
        "Test.Tasty.defaultMain",
        "Test.Tasty.defaultMainWithIngredients",
        "Test.Tasty.testGroup",
        "Test.Tasty.includingOptions",
    ] {
        assert!(
            schemes_chirho.contains_key(qualified_name_chirho),
            "builtin scheme seeding should keep {qualified_name_chirho} available for imported tasty modules"
        );
    }
}

#[test]
fn fun_arity_treats_expanded_shows_alias_as_terminal_result_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let expanded_show_s_chirho =
        ctx_chirho.expand_type_synonyms_chirho(&TyChirho::ConChirho("ShowS".to_string()));
    let fun_ty_chirho = TyChirho::fun_n_chirho(
        vec![
            TyChirho::int_chirho(),
            TyChirho::string_chirho(),
            TyChirho::int_chirho(),
            ctx_chirho.fresh_var_chirho(),
        ],
        expanded_show_s_chirho,
    );

    assert_eq!(
        ctx_chirho.fun_arity_chirho(&fun_ty_chirho),
        4,
        "expanded ShowS should count as a terminal result when matching function equations"
    );
}

#[test]
fn fun_arity_treats_expanded_reads_alias_as_terminal_result_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let read_result_var_chirho = ctx_chirho.fresh_var_chirho();
    let expanded_read_s_chirho = ctx_chirho.expand_type_synonyms_chirho(&TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("ReadS".to_string())),
        Box::new(read_result_var_chirho.clone()),
    ));
    let fun_ty_chirho = TyChirho::fun_n_chirho(
        vec![
            TyChirho::fun_chirho(TyChirho::string_chirho(), expanded_read_s_chirho.clone()),
            TyChirho::int_chirho(),
        ],
        expanded_read_s_chirho,
    );

    assert_eq!(
        ctx_chirho.fun_arity_chirho(&fun_ty_chirho),
        2,
        "expanded ReadS a should count as a terminal result when matching function equations"
    );
}

#[test]
fn infer_expected_show_composition_rhs_with_seeded_params_chirho() {
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

    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let shared_arg_ty_chirho = ctx_chirho.fresh_var_chirho();
    let expanded_show_s_chirho =
        ctx_chirho.expand_type_synonyms_chirho(&TyChirho::ConChirho("ShowS".to_string()));
    ctx_chirho.env_chirho.bind_chirho(
        "spChirho".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), shared_arg_ty_chirho.clone()],
            expanded_show_s_chirho.clone(),
        )),
    );
    ctx_chirho.env_chirho.bind_chirho(
        "nameChirho".to_string(),
        SchemeChirho::mono_chirho(TyChirho::string_chirho()),
    );
    ctx_chirho.env_chirho.bind_chirho(
        "dChirho".to_string(),
        SchemeChirho::mono_chirho(TyChirho::int_chirho()),
    );
    ctx_chirho.env_chirho.bind_chirho(
        "xChirho".to_string(),
        SchemeChirho::mono_chirho(shared_arg_ty_chirho),
    );

    let (_ordinary_subst_chirho, ordinary_ty_chirho) =
        ctx_chirho.infer_expr_chirho(&rhs_expr_chirho);
    assert_eq!(
        ctx_chirho.normalize_ty_chirho(&ordinary_ty_chirho),
        ctx_chirho.normalize_ty_chirho(&expanded_show_s_chirho),
        "ordinary inference with the seeded parameter types should return ShowS"
    );

    let (expected_subst_chirho, expected_ty_chirho) = ctx_chirho
        .infer_expr_against_expected_chirho(
            &rhs_expr_chirho,
            &expanded_show_s_chirho,
            SpanChirho::DUMMY_CHIRHO,
        );
    let final_expected_ty_chirho = expected_subst_chirho.apply_ty_chirho(&expected_ty_chirho);
    assert_eq!(
        ctx_chirho.normalize_ty_chirho(&final_expected_ty_chirho),
        ctx_chirho.normalize_ty_chirho(&expanded_show_s_chirho),
        "expected-type inference with the seeded parameter types should return ShowS"
    );
    assert!(
        ctx_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .is_empty(),
        "seeded composition inference should not emit diagnostics: {:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn infer_matches_against_expected_for_show_composition_signature_chirho() {
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
    let match_arm_chirho = MatchArmChirho {
        pats_chirho: vec![
            PatChirho::VarChirho(dummy_name_chirho("spChirho")),
            PatChirho::VarChirho(dummy_name_chirho("nameChirho")),
            PatChirho::VarChirho(dummy_name_chirho("dChirho")),
            PatChirho::VarChirho(dummy_name_chirho("xChirho")),
        ],
        rhs_chirho: RhsChirho::UnguardedChirho(rhs_expr_chirho),
        where_binds_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let shared_arg_ty_chirho = ctx_chirho.fresh_var_chirho();
    let expanded_show_s_chirho =
        ctx_chirho.expand_type_synonyms_chirho(&TyChirho::ConChirho("ShowS".to_string()));
    let expected_ty_chirho = TyChirho::fun_n_chirho(
        vec![
            TyChirho::fun_n_chirho(
                vec![TyChirho::int_chirho(), shared_arg_ty_chirho.clone()],
                expanded_show_s_chirho.clone(),
            ),
            TyChirho::string_chirho(),
            TyChirho::int_chirho(),
            shared_arg_ty_chirho,
        ],
        expanded_show_s_chirho.clone(),
    );

    let (subst_chirho, inferred_ty_chirho) = ctx_chirho.infer_matches_against_expected_chirho(
        &[match_arm_chirho],
        SpanChirho::DUMMY_CHIRHO,
        &expected_ty_chirho,
    );
    let final_ty_chirho = subst_chirho.apply_ty_chirho(&inferred_ty_chirho);

    assert_eq!(
        ctx_chirho.normalize_ty_chirho(&final_ty_chirho),
        ctx_chirho.normalize_ty_chirho(&expected_ty_chirho),
        "match inference should keep the ShowS-returning signature intact"
    );
    assert!(
        ctx_chirho
            .diagnostics_chirho
            .diagnostics_chirho()
            .is_empty(),
        "match inference should not emit diagnostics: {:?}",
        ctx_chirho.diagnostics_chirho
    );
}

/// Binding groups come out dependencies first, one group per strongly connected
/// component: a mutual pair, a binding after the one it calls, and two bindings that
/// call nothing. Row 661 pins this order across Tarjan's state.
#[test]
fn binding_groups_keep_dependency_order_chirho() {
    fn calls_chirho(callees_chirho: &[&str]) -> Vec<MatchArmChirho> {
        let body_chirho = callees_chirho.iter().fold(
            ExprChirho::LitChirho(LitChirho::IntChirho(0, SpanChirho::DUMMY_CHIRHO, None)),
            |argument_chirho, callee_chirho| ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho(callee_chirho))),
                arg_chirho: Box::new(argument_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        vec![MatchArmChirho {
            pats_chirho: vec![],
            rhs_chirho: RhsChirho::UnguardedChirho(body_chirho),
            where_binds_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]
    }
    // `alpha` and `beta` call each other, `gamma` calls `delta`, and `epsilon` and
    // `zeta` call nothing.
    let names_chirho: Vec<String> = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta"]
        .iter()
        .map(|name_chirho| name_chirho.to_string())
        .collect();
    let arms_chirho = [
        calls_chirho(&["beta"]),
        calls_chirho(&["alpha"]),
        calls_chirho(&["delta"]),
        calls_chirho(&[]),
        calls_chirho(&[]),
        calls_chirho(&[]),
    ];
    let decls_chirho: Vec<(usize, &[MatchArmChirho])> = arms_chirho
        .iter()
        .enumerate()
        .map(|(index_chirho, arm_list_chirho)| (index_chirho, arm_list_chirho.as_slice()))
        .collect();
    assert_eq!(
        binding_groups_chirho(&names_chirho, &decls_chirho, &HashSet::new()),
        vec![vec![1, 0], vec![3], vec![2], vec![4], vec![5]]
    );
}
