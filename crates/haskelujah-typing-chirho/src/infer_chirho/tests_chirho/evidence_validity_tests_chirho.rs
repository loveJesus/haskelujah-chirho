// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Occurrence and literal evidence, and the predicates that are certainly unsolvable or satisfied.
//! Moved from `infer_chirho.rs` in row 661.

use super::test_support_chirho::{
    dummy_name_chirho, type_family_app2_chirho, type_lit_chirho, unspanned_capture_chirho,
};
use super::*;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::expr_chirho::{GuardedExprChirho, RhsChirho, StmtChirho};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;

#[test]
fn infer_literal_int_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let expr_chirho =
        ExprChirho::LitChirho(LitChirho::IntChirho(42, SpanChirho::DUMMY_CHIRHO, None));
    let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    assert!(matches!(ty_chirho, TyChirho::VarChirho(_)));
    assert_eq!(ctx_chirho.deferred_preds_chirho.len(), 1);
    assert_eq!(
        ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho,
        "Num"
    );
}

#[test]
fn occurrence_record_captures_concrete_eq_char_chirho() {
    // Evidence-threading P2 pin: `(==) 'a' 'b'` must yield a finalized
    // occurrence record (name "==", ordinal 0, class Eq, key Char).
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let app_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("=="))),
            arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
                'a',
                SpanChirho::DUMMY_CHIRHO,
                None,
            ))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::CharChirho(
            'b',
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&app_chirho);
    let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&subst_chirho);
    assert!(
        records_chirho.iter().any(|record_chirho| {
            record_chirho.name_chirho == "=="
                && record_chirho.ordinal_chirho == 0
                && record_chirho.class_name_chirho == "Eq"
                && record_chirho.ty_key_chirho == "Char"
        }),
        "expected a concrete Eq/Char record for `==`, got: {records_chirho:?}"
    );
}

#[test]
fn occurrence_record_defaults_unresolved_standard_class_chirho() {
    // Evidence-threading P2c: local/generalized numeric expressions can
    // leave method occurrence predicates as bare vars even after the module
    // substitution. Use the backed runtime default key rather than letting
    // a free `+`/`==` reach STG.
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let unresolved_ty_chirho = ctx_chirho.fresh_var_chirho();
    ctx_chirho
        .occurrence_captures_chirho
        .push(unspanned_capture_chirho(
            "+",
            0,
            "Num",
            unresolved_ty_chirho,
        ));
    let records_chirho =
        ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
    assert_eq!(
        records_chirho,
        vec![MethodOccurrenceRecordChirho {
            applied_ty_chirho: None,
            name_chirho: "+".to_string(),
            ordinal_chirho: 0,
            class_name_chirho: "Num".to_string(),
            class_identity_chirho:
                haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho::UnknownChirho(
                    haskelujah_ast_chirho::name_chirho::ClassIdentityUnknownChirho::LegacyChirho,
                ),
            ty_key_chirho: "Int".to_string(),
            ground_pred_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_chirho: None,
            resolved_entry_chirho: None,
        }]
    );
}

#[test]
fn occurrence_record_defaults_fractional_literal_method_to_double_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let app_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("negate"))),
        arg_chirho: Box::new(ExprChirho::LitChirho(LitChirho::FloatChirho(
            5.5,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&app_chirho);
    let default_subst_chirho = ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
    if !default_subst_chirho.is_empty_chirho() {
        ctx_chirho.apply_subst_all_chirho(&default_subst_chirho);
    }
    let composed_subst_chirho = default_subst_chirho.compose_chirho(&subst_chirho);
    let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&composed_subst_chirho);
    assert!(
        records_chirho.iter().any(|record_chirho| {
            record_chirho.name_chirho == "negate"
                && record_chirho.ordinal_chirho == 0
                && record_chirho.class_name_chirho == "Num"
                && record_chirho.ty_key_chirho == "Double"
        }),
        "expected Num/Double evidence for `negate 5.5`, got: {records_chirho:?}"
    );
}

#[test]
fn occurrence_record_retains_concrete_structured_show_keys_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.occurrence_captures_chirho.extend([
        unspanned_capture_chirho(
            "print",
            0,
            "Show",
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        ),
        unspanned_capture_chirho(
            "print",
            1,
            "Show",
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(TyChirho::string_chirho()),
            ),
        ),
        unspanned_capture_chirho(
            "print",
            2,
            "Show",
            TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::string_chirho()]),
        ),
    ]);

    let records_chirho =
        ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
    let keys_chirho = records_chirho
        .iter()
        .map(|record_chirho| record_chirho.ty_key_chirho.as_str())
        .collect::<Vec<_>>();

    assert_eq!(keys_chirho, vec!["[Int]", "Maybe String", "(Int,String)"]);
}

#[test]
fn occurrence_record_retains_concrete_structured_ord_keys_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.occurrence_captures_chirho.extend([
        unspanned_capture_chirho(
            "compare",
            0,
            "Ord",
            TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(TyChirho::int_chirho()),
            ),
        ),
        unspanned_capture_chirho(
            "maximum",
            0,
            "Ord",
            TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::string_chirho()]),
        ),
    ]);

    let records_chirho =
        ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
    let keys_chirho = records_chirho
        .iter()
        .map(|record_chirho| record_chirho.ty_key_chirho.as_str())
        .collect::<Vec<_>>();

    assert_eq!(keys_chirho, vec!["Maybe Int", "(Int,String)"]);
}

#[test]
fn print_occurrence_uses_payload_annotation_for_maybe_key_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let annotated_payload_chirho = ExprChirho::AnnChirho {
        expr_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            3,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        ty_chirho: TypeChirho::ConChirho(dummy_name_chirho("Int")),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let maybe_value_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::ConChirho(dummy_name_chirho("Just"))),
        arg_chirho: Box::new(annotated_payload_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let print_app_chirho = ExprChirho::AppChirho {
        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("print"))),
        arg_chirho: Box::new(maybe_value_chirho),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };

    let (subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&print_app_chirho);
    let default_subst_chirho = ctx_chirho.check_deferred_preds_chirho(&subst_chirho);
    let composed_subst_chirho = default_subst_chirho.compose_chirho(&subst_chirho);
    let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&composed_subst_chirho);

    assert!(
        records_chirho.iter().any(|record_chirho| {
            record_chirho.name_chirho == "print"
                && record_chirho.ordinal_chirho == 0
                && record_chirho.class_name_chirho == "Show"
                && record_chirho.ty_key_chirho == "Maybe Int"
        }),
        "payload annotation should resolve print evidence to Maybe Int, got: {records_chirho:?}"
    );
}

#[test]
fn infer_literal_string_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let expr_chirho = ExprChirho::LitChirho(LitChirho::StringChirho(
        "hello".to_string(),
        SpanChirho::DUMMY_CHIRHO,
        None,
    ));
    let (_s_chirho, ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    assert_eq!(ty_chirho, TyChirho::string_chirho());
}

#[test]
fn infer_type_mismatch_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    // if 42 then 1 else 2  (condition is Int, not Bool)
    let expr_chirho = ExprChirho::IfChirho {
        cond_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            42,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        then_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            1,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        else_chirho: Box::new(ExprChirho::LitChirho(LitChirho::IntChirho(
            2,
            SpanChirho::DUMMY_CHIRHO,
            None,
        ))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let _ = ctx_chirho.infer_expr_chirho(&expr_chirho);
    ctx_chirho.check_deferred_preds_chirho(&SubstChirho::empty_chirho());
    let result_chirho = ctx_chirho.finish_chirho();
    assert!(
        result_chirho.diagnostics_chirho.has_errors_chirho(),
        "should report type mismatch for a numeric condition"
    );
}

#[test]
fn defaulted_integer_with_integral_and_bounded_skips_bounded_error_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let constrained_var_chirho = match ctx_chirho.fresh_var_chirho() {
        TyChirho::VarChirho(var_chirho) => var_chirho,
        other_chirho => panic!("expected fresh type variable, got {:?}", other_chirho),
    };
    ctx_chirho.deferred_preds_chirho.push((
        PredChirho::new_chirho("Integral", TyChirho::VarChirho(constrained_var_chirho)),
        SpanChirho::DUMMY_CHIRHO,
    ));
    ctx_chirho.deferred_preds_chirho.push((
        PredChirho::new_chirho("Bounded", TyChirho::VarChirho(constrained_var_chirho)),
        SpanChirho::DUMMY_CHIRHO,
    ));

    let default_subst_chirho = ctx_chirho.check_deferred_preds_chirho(&SubstChirho::empty_chirho());
    assert!(
        matches!(
            default_subst_chirho.lookup_chirho(&constrained_var_chirho),
            Some(TyChirho::ConChirho(name_chirho)) if name_chirho == "Integer"
        ),
        "Integral+Bounded ambiguity should still default to Integer"
    );
    assert!(
        !ctx_chirho.diagnostics_chirho.has_errors_chirho(),
        "defaulted Integral+Bounded Integer should not report a missing Bounded Integer instance: {:?}",
        ctx_chirho.diagnostics_chirho
    );
}

#[test]
fn overloaded_op_defers_predicate_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let expr_chirho = ExprChirho::LamChirho {
        pats_chirho: vec![
            PatChirho::VarChirho(dummy_name_chirho("x")),
            PatChirho::VarChirho(dummy_name_chirho("y")),
        ],
        body_chirho: Box::new(ExprChirho::AppChirho {
            fun_chirho: Box::new(ExprChirho::AppChirho {
                fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let (_subst_chirho, _ty_chirho) = ctx_chirho.infer_expr_chirho(&expr_chirho);
    assert!(!ctx_chirho.deferred_preds_chirho.is_empty());
    assert_eq!(
        ctx_chirho.deferred_preds_chirho[0].0.class_name_chirho,
        "Num"
    );
}

#[test]
fn plus_on_ints_satisfies_num_chirho() {
    // f x y = x + y  applied to Int arguments should succeed
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("PlusInts"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("addChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("x")),
                    PatChirho::VarChirho(dummy_name_chirho("y")),
                ],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("+"))),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
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
        "addChirho should infer without errors (Num constraint stays polymorphic): {:?}",
        result_chirho.diagnostics_chirho
    );

    // addChirho should have a Num constraint in its scheme
    let scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("addChirho")
        .expect("addChirho should be in environment");
    assert!(
        !scheme_chirho.preds_chirho.is_empty(),
        "addChirho should have Num predicate, got: {scheme_chirho}"
    );
    assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Num");
}

#[test]
fn eq_on_ints_satisfies_constraint_chirho() {
    // eqChirho x y = x == y
    let module_chirho = ModuleChirho {
        name_chirho: dummy_name_chirho("EqInts"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![DeclChirho::FunBindChirho {
            name_chirho: dummy_name_chirho("eqChirho"),
            matches_chirho: vec![MatchArmChirho {
                pats_chirho: vec![
                    PatChirho::VarChirho(dummy_name_chirho("x")),
                    PatChirho::VarChirho(dummy_name_chirho("y")),
                ],
                rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::AppChirho {
                    fun_chirho: Box::new(ExprChirho::AppChirho {
                        fun_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("=="))),
                        arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("x"))),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }),
                    arg_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("y"))),
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
        "eqChirho should infer without errors: {:?}",
        result_chirho.diagnostics_chirho
    );

    let scheme_chirho = result_chirho
        .env_chirho
        .lookup_chirho("eqChirho")
        .expect("eqChirho should be in environment");
    assert!(
        !scheme_chirho.preds_chirho.is_empty(),
        "eqChirho should have Eq predicate, got: {scheme_chirho}"
    );
    assert_eq!(scheme_chirho.preds_chirho[0].class_name_chirho, "Eq");
}

#[test]
fn class_env_is_seeded_chirho() {
    let result_chirho = infer_module_chirho(&ModuleChirho {
        name_chirho: dummy_name_chirho("Empty"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![],
        extensions_chirho: vec![],
        inline_pragmas_chirho: std::collections::HashMap::new(),
        specialize_pragmas_chirho: std::collections::HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    });

    assert!(result_chirho.class_env_chirho.has_class_chirho("Eq"));
    assert!(result_chirho.class_env_chirho.has_class_chirho("Ord"));
    assert!(result_chirho.class_env_chirho.has_class_chirho("Show"));
    assert!(result_chirho.class_env_chirho.has_class_chirho("Num"));
    assert!(result_chirho.class_env_chirho.has_class_chirho("Functor"));
}

#[test]
fn infer_guard_expression_records_method_occurrence_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let match_arm_chirho = MatchArmChirho {
        pats_chirho: vec![
            PatChirho::VarChirho(dummy_name_chirho("aChirho")),
            PatChirho::VarChirho(dummy_name_chirho("bChirho")),
        ],
        rhs_chirho: RhsChirho::GuardedChirho(vec![GuardedExprChirho {
            quals_chirho: vec![StmtChirho::expr_stmt_chirho(ExprChirho::InfixChirho {
                left_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("aChirho"))),
                op_chirho: dummy_name_chirho("=="),
                right_chirho: Box::new(ExprChirho::VarChirho(dummy_name_chirho("bChirho"))),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            })],
            body_chirho: ExprChirho::VarChirho(dummy_name_chirho("aChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }]),
        where_binds_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let expected_ty_chirho = TyChirho::fun_n_chirho(
        vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
        TyChirho::int_chirho(),
    );

    let (subst_chirho, _inferred_ty_chirho) = ctx_chirho.infer_matches_against_expected_chirho(
        &[match_arm_chirho],
        SpanChirho::DUMMY_CHIRHO,
        &expected_ty_chirho,
    );
    let records_chirho = ctx_chirho.finalize_occurrence_records_chirho(&subst_chirho);
    assert!(
        records_chirho.iter().any(|record_chirho| {
            record_chirho.name_chirho == "=="
                && record_chirho.class_name_chirho == "Eq"
                && record_chirho.ty_key_chirho == "Int"
        }),
        "guard method occurrence should be recorded, got: {records_chirho:?}"
    );
}

#[test]
fn typelits_builtin_append_symbol_reduces_literals_and_identities_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let x_chirho = TyChirho::VarChirho(TyVarChirho(7001));

    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
            "AppendSymbol",
            type_lit_chirho("\"\""),
            x_chirho.clone(),
        )),
        x_chirho,
        "AppendSymbol \"\" x should reduce to x"
    );
    assert_eq!(
        ctx_chirho.reduce_type_families_in_ty_chirho(&type_family_app2_chirho(
            "GHC.TypeLits.AppendSymbol",
            type_lit_chirho("\"type\""),
            type_lit_chirho("\"level\""),
        )),
        type_lit_chirho("\"typelevel\""),
        "qualified AppendSymbol should reduce concrete Symbol literals"
    );
}

#[test]
fn certainly_unsolvable_seeded_class_ground_no_instance_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
    assert!(
        ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "ground Eq on a type with no instance and no deriving must be certainly unsolvable"
    );
}

#[test]
fn certainly_unsolvable_suppressed_by_matching_instance_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho
        .class_env_chirho
        .add_instance_chirho(InstDeclChirho::legacy_chirho(
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ConChirho("ZzNoSuchType".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        ));
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "a head-matching instance must suppress the unsolvable verdict"
    );
}

#[test]
fn certainly_unsolvable_suppressed_by_var_headed_instance_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho
        .class_env_chirho
        .add_instance_chirho(InstDeclChirho::legacy_chirho(
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        ));
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "a variable-headed instance can match anything and must suppress the verdict"
    );
}

#[test]
fn certainly_unsolvable_suppressed_by_deriving_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho
        .derived_instance_heads_chirho
        .insert(("Eq".to_string(), "ZzNoSuchType".to_string()));
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "a deriving mention is a potential instance and must suppress the verdict"
    );
}

#[test]
fn certainly_unsolvable_suppressed_by_given_chirho() {
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::ConChirho("ZzNoSuchType".to_string()));
    ctx_chirho.given_preds_chirho.push(pred_chirho.clone());
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "a signature given entailing the predicate must suppress the verdict"
    );
}

#[test]
fn certainly_unsolvable_skips_undeclared_class_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let pred_chirho = PredChirho::new_chirho(
        "ZzClassFromSomeImport",
        TyChirho::ConChirho("Bool".to_string()),
    );
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "a class that is neither seeded nor module-declared may have unseen instances"
    );
}

#[test]
fn certainly_unsolvable_skips_unboxed_primitive_head_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let pred_chirho = PredChirho::new_chirho("Num", TyChirho::ConChirho("Int#".to_string()));
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "unboxed literals are still typed through the Num path, so `#` heads get no verdict"
    );
}

#[test]
fn certainly_unsolvable_skips_magic_class_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let pred_chirho = PredChirho::new_chirho("Coercible", TyChirho::ConChirho("Bool".to_string()));
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "compiler-solved classes have no declared instances by design"
    );
}

#[test]
fn certainly_unsolvable_skips_nonground_pred_chirho() {
    let ctx_chirho = InferCtxChirho::new_chirho();
    let pred_chirho = PredChirho::new_chirho("Eq", TyChirho::VarChirho(TyVarChirho(7)));
    assert!(
        !ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho),
        "a predicate with unification variables has no final verdict"
    );
}

/// A pure checked kind telescope plus the existing legacy ordinary class arena.
/// This fixture supplies no source-unit or runtime dictionary identity.
fn kind_prefix_context_chirho() -> (InferCtxChirho, PredChirho) {
    use crate::ty_chirho::declarations_chirho::{
        DeclarationAssociationChirho, DeclarationBinderChirho, DeclarationSchemeChirho,
        TelescopeVisibilityChirho,
    };
    use haskelujah_ast_chirho::name_chirho::{
        CompilerTypeConstructorChirho, TypeConstructorDeclarationChirho, TypeConstructorRoleChirho,
        TypeConstructorScopeChirho,
    };
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.local_class_prefix_instances_complete_chirho = true;
    ctx_chirho.class_env_chirho = ClassEnvChirho::with_registration_limit_chirho(4096);
    let type_chirho = TyChirho::ConChirho(
        TypeConstructorDeclarationChirho::compiler_data_chirho(
            CompilerTypeConstructorChirho::TypeUniverseChirho,
        )
        .type_key_chirho(),
    );
    let schema_chirho = |proof_chirho: TypeConstructorDeclarationChirho,
                         binders_chirho: Vec<DeclarationBinderChirho>,
                         result_chirho: TyChirho| DeclarationSchemeChirho {
        identity_chirho: std::sync::Arc::from(proof_chirho.type_key_chirho()),
        owner_chirho: None,
        role_chirho: proof_chirho.role_chirho,
        association_chirho: if proof_chirho.role_chirho == TypeConstructorRoleChirho::ClassChirho {
            DeclarationAssociationChirho::ClassChirho {
                families_chirho: Vec::new(),
            }
        } else {
            DeclarationAssociationChirho::NoneChirho
        },
        declaration_chirho: Some(proof_chirho),
        injectivity_chirho: None,
        visible_arity_chirho: binders_chirho
            .iter()
            .filter(|binder_chirho| {
                binder_chirho.visibility_chirho == TelescopeVisibilityChirho::RequiredChirho
            })
            .count(),
        reduction_arity_chirho: binders_chirho.len(),
        binders_chirho,
        source_variables_chirho: Vec::new(),
        result_kind_chirho: result_chirho,
        synonym_rhs_chirho: None,
        synonym_syntax_chirho: None,
        local_binders_chirho: std::sync::Arc::from([]),
        promoted_chirho: None,
        held_chirho: false,
    };
    for constructor_chirho in [
        CompilerTypeConstructorChirho::BoolChirho,
        CompilerTypeConstructorChirho::IntChirho,
    ] {
        ctx_chirho
            .type_families_chirho
            .declaration_schemes_mut_chirho()
            .insert_chirho(schema_chirho(
                TypeConstructorDeclarationChirho::compiler_data_chirho(constructor_chirho),
                Vec::new(),
                type_chirho.clone(),
            ))
            .unwrap();
    }
    let proof_chirho = TypeConstructorDeclarationChirho::source_chirho(
        "KindsChirho",
        "HiddenChirho",
        TypeConstructorRoleChirho::ClassChirho,
    );
    let mut scope_chirho = TypeConstructorScopeChirho::default();
    scope_chirho.bind_chirho("HiddenChirho".to_owned(), Some(proof_chirho.clone()));
    ctx_chirho.type_constructor_scope_chirho = Some(std::sync::Arc::new(scope_chirho));
    ctx_chirho
        .type_families_chirho
        .declaration_schemes_mut_chirho()
        .insert_chirho(schema_chirho(
            proof_chirho,
            vec![
                DeclarationBinderChirho {
                    variable_chirho: TyVarChirho(0),
                    kind_chirho: type_chirho,
                    visibility_chirho: TelescopeVisibilityChirho::InferredChirho,
                    term_specificity_chirho: None,
                },
                DeclarationBinderChirho {
                    variable_chirho: TyVarChirho(1),
                    kind_chirho: TyChirho::VarChirho(TyVarChirho(0)),
                    visibility_chirho: TelescopeVisibilityChirho::RequiredChirho,
                    term_specificity_chirho: None,
                },
            ],
            TyChirho::ConChirho(
                TypeConstructorDeclarationChirho::compiler_data_chirho(
                    CompilerTypeConstructorChirho::ConstraintUniverseChirho,
                )
                .type_key_chirho(),
            ),
        ))
        .unwrap();
    ctx_chirho
        .class_env_chirho
        .add_class_chirho(ClassDeclChirho::legacy_chirho(
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: vec![TyVarChirho(0)],
                name_chirho: "HiddenChirho".to_owned(),
                supers_chirho: Vec::new(),
                var_chirho: TyVarChirho(1),
                extra_vars_chirho: Vec::new(),
                fundeps_chirho: Vec::new(),
                methods_chirho: HashMap::new(),
                defaults_chirho: HashMap::new(),
            },
        ));
    ctx_chirho
        .fully_known_class_names_chirho
        .insert("HiddenChirho".to_owned());
    ctx_chirho
        .class_env_chirho
        .add_instance_chirho(InstDeclChirho::legacy_chirho(
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: vec![TyChirho::bool_chirho()],
                class_name_chirho: "HiddenChirho".to_owned(),
                head_ty_chirho: TyChirho::VarChirho(TyVarChirho(2)),
                extra_head_tys_chirho: Vec::new(),
                context_chirho: Vec::new(),
            },
        ));
    let pred_chirho = PredChirho::from_proof_chirho(
        haskelujah_ast_chirho::name_chirho::PredicateClassProofChirho::UnknownChirho(
            haskelujah_ast_chirho::name_chirho::ClassIdentityUnknownChirho::LegacyChirho,
        ),
        crate::class_chirho::PredPartsChirho {
            kind_tys_chirho: vec![TyChirho::int_chirho()],
            class_name_chirho: "HiddenChirho".to_owned(),
            ty_chirho: TyChirho::ConChirho("Opaque#".to_owned()),
            extra_tys_chirho: Vec::new(),
        },
    );
    (ctx_chirho, pred_chirho)
}

#[test]
fn a_checked_kind_prefix_refutes_without_classifying_an_opaque_visible_head_chirho() {
    let (mut ctx_chirho, pred_chirho) = kind_prefix_context_chirho();
    assert!(ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho));
    ctx_chirho.given_preds_chirho.push(pred_chirho.clone());
    assert!(!ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho));
}

#[test]
fn incomplete_or_derived_kind_prefix_metadata_never_certifies_a_refutation_chirho() {
    for case_chirho in 0..5 {
        let (mut ctx_chirho, pred_chirho) = kind_prefix_context_chirho();
        match case_chirho {
            0 => {
                ctx_chirho.type_constructor_scope_chirho = None;
            }
            1 => {
                ctx_chirho
                    .derived_instance_heads_chirho
                    .insert(("HiddenChirho".to_owned(), "*".to_owned()));
            }
            4 => {
                ctx_chirho.local_class_prefix_instances_complete_chirho = false;
            }
            _ => {
                let prefix_chirho = if case_chirho == 2 {
                    Vec::new()
                } else {
                    vec![TyChirho::ConChirho("UnprovedKindChirho".to_owned())]
                };
                ctx_chirho
                    .class_env_chirho
                    .add_instance_chirho(InstDeclChirho::legacy_chirho(
                        crate::class_chirho::InstDeclPartsChirho {
                            kind_tys_chirho: prefix_chirho,
                            class_name_chirho: "HiddenChirho".to_owned(),
                            head_ty_chirho: TyChirho::VarChirho(TyVarChirho(3)),
                            extra_head_tys_chirho: Vec::new(),
                            context_chirho: Vec::new(),
                        },
                    ));
            }
        }
        assert!(!ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho));
    }
}

#[test]
fn a_truncated_class_arena_never_turns_retained_prefixes_into_a_closed_universe_chirho() {
    let (mut ctx_chirho, pred_chirho) = kind_prefix_context_chirho();
    assert!(ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho));
    for index_chirho in 0..4096 {
        ctx_chirho
            .class_env_chirho
            .add_instance_chirho(InstDeclChirho::legacy_chirho(
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: vec![TyChirho::bool_chirho()],
                    class_name_chirho: "HiddenChirho".to_owned(),
                    head_ty_chirho: TyChirho::VarChirho(TyVarChirho(index_chirho + 10)),
                    extra_head_tys_chirho: Vec::new(),
                    context_chirho: Vec::new(),
                },
            ));
        if ctx_chirho
            .class_env_chirho
            .registry_error_chirho()
            .is_some()
        {
            break;
        }
    }
    assert!(
        ctx_chirho
            .class_env_chirho
            .registry_error_chirho()
            .is_some()
    );
    assert!(
        !ctx_chirho
            .class_env_chirho
            .instances_chirho()
            .get_chirho("HiddenChirho")
            .unwrap()
            .is_empty_chirho()
    );
    assert!(!ctx_chirho.is_certainly_unsolvable_pred_chirho(&pred_chirho));
}

/// `head` applied to `args`, as unification leaves a constructor variable's instance.
fn applied_constructor_chirho(head_chirho: &str, args_chirho: Vec<TyChirho>) -> TyChirho {
    args_chirho.into_iter().fold(
        TyChirho::ConChirho(head_chirho.to_string()),
        |fun_chirho, arg_chirho| TyChirho::AppChirho(Box::new(fun_chirho), Box::new(arg_chirho)),
    )
}

/// `m Int` at `m := []` resolves to the applied list constructor `[] Int`, and a head
/// variable at `(,)` to the applied tuple constructor. Unification equates each with its
/// sugar, so each keys exactly as its sugar does. Show rows are named by those keys, and
/// natively `print` of the applied form had found none (row 697).
#[test]
fn applied_list_and_saturated_tuple_constructors_key_as_their_sugar_chirho() {
    let int_list_chirho = applied_constructor_chirho("[]", vec![TyChirho::int_chirho()]);
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.occurrence_captures_chirho.extend([
        unspanned_capture_chirho("print", 0, "Show", int_list_chirho.clone()),
        unspanned_capture_chirho(
            "print",
            1,
            "Show",
            applied_constructor_chirho("[]", vec![TyChirho::char_chirho()]),
        ),
        unspanned_capture_chirho(
            "print",
            2,
            "Show",
            applied_constructor_chirho("Maybe", vec![int_list_chirho.clone()]),
        ),
        unspanned_capture_chirho(
            "print",
            3,
            "Show",
            applied_constructor_chirho(
                "(,)",
                vec![TyChirho::int_chirho(), int_list_chirho.clone()],
            ),
        ),
        unspanned_capture_chirho(
            "==",
            4,
            "Eq",
            applied_constructor_chirho("[]", vec![int_list_chirho.clone()]),
        ),
        unspanned_capture_chirho(
            "compare",
            5,
            "Ord",
            applied_constructor_chirho(
                "(,,)",
                vec![
                    TyChirho::int_chirho(),
                    TyChirho::bool_chirho(),
                    TyChirho::char_chirho(),
                ],
            ),
        ),
    ]);

    let records_chirho =
        ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
    let keys_chirho = records_chirho
        .iter()
        .map(|record_chirho| record_chirho.ty_key_chirho.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        keys_chirho,
        vec![
            "[Int]",
            "[Char]",
            "Maybe [Int]",
            "(Int,[Int])",
            "[[Int]]",
            "(Int,Bool,Char)"
        ]
    );
    assert_eq!(
        evidence_chirho::ground_list_key_chirho(&int_list_chirho).as_deref(),
        Some("[Int]"),
        "reference, literal and local keys take the same whole list key"
    );
}

/// Only a fully saturated constructor over ground arguments earns its aggregate key: a
/// partly applied tuple constructor keeps its applied spelling, and an applied list over
/// a variable keeps the head `[]` (row 697).
#[test]
fn a_partial_tuple_or_open_list_constructor_earns_no_aggregate_key_chirho() {
    let open_list_chirho =
        applied_constructor_chirho("[]", vec![TyChirho::VarChirho(TyVarChirho(41))]);
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    ctx_chirho.occurrence_captures_chirho.extend([
        unspanned_capture_chirho(
            "show",
            0,
            "Show",
            applied_constructor_chirho("(,)", vec![TyChirho::int_chirho()]),
        ),
        unspanned_capture_chirho("show", 1, "Show", open_list_chirho.clone()),
    ]);

    let records_chirho =
        ctx_chirho.finalize_occurrence_records_chirho(&SubstChirho::empty_chirho());
    let keys_chirho = records_chirho
        .iter()
        .map(|record_chirho| record_chirho.ty_key_chirho.as_str())
        .collect::<Vec<_>>();

    assert_eq!(keys_chirho, vec!["(,) Int", "[]"]);
    assert_eq!(
        evidence_chirho::ground_list_key_chirho(&open_list_chirho),
        None
    );
}

#[test]
fn a_qualified_capture_keeps_the_entry_its_lookup_resolved_chirho() {
    // Row 698: a qualified reference's capture keeps the qualified entry its scheme came
    // from, its own spelling (`P.print`) or the one entry it is a suffix of (`Q.go` in
    // `Data.Q.go`), never a spelling guessed from the reference. An unqualified reference
    // keeps none, so the join judges it by its bare name as before.
    use haskelujah_ast_chirho::name_chirho::RawNameChirho;
    let mut ctx_chirho = InferCtxChirho::new_chirho();
    let var_chirho = TyVarChirho(0);
    let show_scheme_chirho = SchemeChirho {
        vars_chirho: vec![var_chirho].into(),
        preds_chirho: vec![SchemePredChirho::legacy_chirho(
            crate::ty_chirho::SchemePredPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(var_chirho),
                extra_tys_chirho: vec![],
            },
        )],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        ),
    };
    for entry_chirho in ["print", "P.print", "Data.Q.go"] {
        ctx_chirho
            .env_chirho
            .bind_chirho(entry_chirho.to_string(), show_scheme_chirho.clone());
    }
    let qualified_chirho = |qualifier_chirho: &str, text_chirho: &str| {
        NameChirho::RawChirho(RawNameChirho::qualified_chirho(
            qualifier_chirho,
            text_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    };
    let pred_chirho = PredChirho::new_chirho("Show", ctx_chirho.fresh_var_chirho());
    for name_chirho in [
        qualified_chirho("P", "print"),
        qualified_chirho("Q", "go"),
        dummy_name_chirho("print"),
    ] {
        ctx_chirho
            .capture_occurrence_predicates_chirho(&name_chirho, std::slice::from_ref(&pred_chirho));
    }
    let entries_chirho: Vec<Option<&str>> = ctx_chirho
        .occurrence_captures_chirho
        .iter()
        .map(|capture_chirho| capture_chirho.resolved_entry_chirho.as_deref())
        .collect();
    assert_eq!(
        entries_chirho,
        vec![Some("P.print"), Some("Data.Q.go"), None]
    );
}
