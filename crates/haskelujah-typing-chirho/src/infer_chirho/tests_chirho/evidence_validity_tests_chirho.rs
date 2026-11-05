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
            ty_key_chirho: "Int".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            origin_chirho: None,
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
        .add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::ConChirho("ZzNoSuchType".to_string()),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });
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
        .add_instance_chirho(InstDeclChirho {
            class_name_chirho: "Eq".to_string(),
            head_ty_chirho: TyChirho::VarChirho(TyVarChirho(0)),
            extra_head_tys_chirho: vec![],
            context_chirho: vec![],
        });
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
