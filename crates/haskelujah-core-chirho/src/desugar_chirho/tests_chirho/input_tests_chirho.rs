// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unchecked public AST input must not erase an executable pattern splice.
use super::*;
use haskelujah_ast_chirho::decl_chirho::{GadtProjectionLimitChirho, GadtProjectionLimitsChirho};
use haskelujah_ast_chirho::pat_chirho::PatFieldChirho;

fn unit_chirho() -> ExprChirho {
    ExprChirho::TupleChirho {
        elements_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(declaration_chirho: DeclChirho) -> ModuleChirho {
    ModuleChirho {
        name_chirho: dummy_name_chirho("InputChirho"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho: vec![declaration_chirho],
        extensions_chirho: vec![],
        inline_pragmas_chirho: Default::default(),
        specialize_pragmas_chirho: Default::default(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}

fn binding_chirho(pattern_chirho: PatChirho) -> DeclChirho {
    DeclChirho::PatBindChirho {
        pat_chirho: pattern_chirho,
        rhs_chirho: RhsChirho::UnguardedChirho(unit_chirho()),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn splice_chirho(present_chirho: bool) -> PatChirho {
    PatChirho::SpliceChirho {
        expr_chirho: present_chirho
            .then(|| Box::new(ExprChirho::VarChirho(dummy_name_chirho("wildP")))),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn assert_refusal_chirho(module_chirho: &ModuleChirho) {
    let error_chirho =
        desugar_module_chirho(module_chirho).expect_err("no partial Core is published");
    assert!(
        error_chirho.is_unexpanded_pattern_chirho(),
        "{error_chirho}"
    );
}

#[test]
fn unexpanded_payload_and_missing_payload_both_refuse_before_core_chirho() {
    for present_chirho in [true, false] {
        assert_refusal_chirho(&module_chirho(binding_chirho(splice_chirho(
            present_chirho,
        ))));
    }
}

#[test]
fn lazy_record_and_view_patterns_cannot_erase_an_unexpanded_splice_chirho() {
    for pattern_chirho in [
        PatChirho::LazyChirho {
            inner_chirho: Box::new(splice_chirho(true)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        PatChirho::RecordChirho {
            con_chirho: dummy_name_chirho("BoxChirho"),
            fields_chirho: vec![PatFieldChirho {
                name_chirho: dummy_name_chirho("fieldChirho"),
                pattern_chirho: splice_chirho(true),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            has_wildcard_chirho: false,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
        PatChirho::ViewChirho {
            expr_chirho: Box::new(ExprChirho::LamChirho {
                pats_chirho: vec![splice_chirho(true)],
                body_chirho: Box::new(unit_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
            pat_chirho: Box::new(PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO)),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    ] {
        assert_refusal_chirho(&module_chirho(binding_chirho(pattern_chirho)));
    }
}

#[test]
fn explicit_synonym_builders_are_checked_before_collection_chirho() {
    assert_refusal_chirho(&module_chirho(DeclChirho::PatSynDeclChirho {
        name_chirho: dummy_name_chirho("PatternChirho"),
        record_chirho: false,
        args_chirho: vec![],
        pat_chirho: PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
        dir_chirho: PatSynDirChirho::ExplBidirChirho {
            builder_binds_chirho: vec![LocalBindChirho::PatBindChirho {
                pat_chirho: splice_chirho(true),
                rhs_chirho: RhsChirho::UnguardedChirho(unit_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        },
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }));
}

#[test]
fn quotation_data_is_not_executable_pattern_input_chirho() {
    let module_chirho = module_chirho(DeclChirho::PatBindChirho {
        pat_chirho: PatChirho::VarChirho(dummy_name_chirho("quotationChirho")),
        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::QuotePatChirho {
            pat_chirho: splice_chirho(true),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
    desugar_module_chirho(&module_chirho)
        .expect("the Pat-only gate retains syntax data; this does not claim TH execution");
}

#[test]
fn borrowed_pattern_scan_uses_the_same_sticky_allowance_without_output_chirho() {
    let module_chirho = module_chirho(binding_chirho(PatChirho::WildcardChirho(
        SpanChirho::DUMMY_CHIRHO,
    )));
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 100,
            output_chirho: 0,
            depth_chirho: 128,
        });
    input_chirho::check_executable_patterns_chirho(
        &module_chirho,
        &mut budget_chirho,
        Some(&DesugarCtxChirho::new_chirho()),
    )
    .expect("borrowed scan copies no output");
    assert_eq!(budget_chirho.remaining_limits_chirho().output_chirho, 0);
    assert!(budget_chirho.remaining_limits_chirho().work_chirho < 100);
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 1,
            output_chirho: 0,
            depth_chirho: 128,
        });
    let error_chirho = input_chirho::check_executable_patterns_chirho(
        &module_chirho,
        &mut budget_chirho,
        Some(&DesugarCtxChirho::new_chirho()),
    )
    .expect_err("the cursor shares the finite allowance");
    let Some(error_chirho) = error_chirho.projection_error_chirho() else {
        panic!("resource error must remain typed")
    };
    assert_eq!(
        error_chirho.limit_chirho,
        GadtProjectionLimitChirho::WorkChirho
    );
    assert_eq!(
        budget_chirho.visit_borrowed_syntax_chirho(SpanChirho::DUMMY_CHIRHO, 1),
        Err(error_chirho)
    );
}

fn transform_module_chirho(present_chirho: bool) -> ModuleChirho {
    use haskelujah_ast_chirho::expr_chirho::{
        ComprehensionQualifierChirho, ComprehensionTransformChirho,
    };
    module_chirho(DeclChirho::PatBindChirho {
        pat_chirho: PatChirho::VarChirho(dummy_name_chirho("resultChirho")),
        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::ListCompChirho {
            body_chirho: Box::new(unit_chirho()),
            quals_chirho: vec![
                haskelujah_ast_chirho::expr_chirho::StmtChirho::expr_stmt_chirho(
                    ExprChirho::ConChirho(dummy_name_chirho("True")),
                )
                .into(),
                ComprehensionQualifierChirho::TransformChirho(ComprehensionTransformChirho {
                    group_chirho: true,
                    using_chirho: present_chirho.then(|| {
                        Box::new(ExprChirho::VarChirho(dummy_name_chirho(
                            "transformerChirho",
                        )))
                    }),
                    by_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }),
            ],
            parallel_quals_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    })
}

#[test]
fn unread_transform_refuses_before_any_core_publication_chirho() {
    assert!(
        desugar_module_chirho(&transform_module_chirho(false))
            .expect_err("unread transformer must refuse before Core Chirho")
            .is_unread_transform_chirho()
    );
}

#[test]
fn transform_output_reserves_the_original_sticky_module_allowance_chirho() {
    let context_chirho = DesugarCtxChirho::new_chirho();
    let module_chirho = transform_module_chirho(true);
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 10000,
            output_chirho: 127,
            depth_chirho: 128,
        });
    let error_chirho = input_chirho::check_executable_patterns_chirho(
        &module_chirho,
        &mut budget_chirho,
        Some(&context_chirho),
    )
    .expect_err("even a zero-column transform emits a real row/loop");
    let Some(error_chirho) = error_chirho.projection_error_chirho() else {
        panic!("typed original resource failure")
    };
    assert_eq!(
        error_chirho.limit_chirho,
        GadtProjectionLimitChirho::OutputChirho
    );
    assert_eq!(
        budget_chirho.reserve_transport_chirho(1, 1, SpanChirho::DUMMY_CHIRHO),
        Err(error_chirho)
    );
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 10000,
            output_chirho: 128,
            depth_chirho: 128,
        });
    input_chirho::check_executable_patterns_chirho(
        &module_chirho,
        &mut budget_chirho,
        Some(&context_chirho),
    )
    .expect("the exact checked reserve fits");
    assert_eq!(budget_chirho.remaining_limits_chirho().output_chirho, 0);
}

#[test]
fn blocked_named_record_fields_refuse_with_a_typed_input_cause_chirho() {
    let pattern_chirho = PatChirho::RecordChirho {
        con_chirho: dummy_name_chirho("BoxChirho"),
        fields_chirho: vec![PatFieldChirho {
            name_chirho: dummy_name_chirho("fieldChirho"),
            pattern_chirho: PatChirho::VarChirho(dummy_name_chirho("valueChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        has_wildcard_chirho: false,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let construction_chirho = module_chirho(DeclChirho::PatBindChirho {
        pat_chirho: PatChirho::VarChirho(dummy_name_chirho("valueChirho")),
        rhs_chirho: RhsChirho::UnguardedChirho(ExprChirho::RecordConChirho {
            con_chirho: dummy_name_chirho("BoxChirho"),
            fields_chirho: vec![haskelujah_ast_chirho::expr_chirho::FieldAssignChirho {
                name_chirho: dummy_name_chirho("fieldChirho"),
                value_chirho: unit_chirho(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            has_wildcard_chirho: false,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    });
    for module_chirho in [
        module_chirho(binding_chirho(pattern_chirho)),
        construction_chirho,
    ] {
        let mut context_chirho = DesugarCtxChirho::new_chirho();
        context_chirho.con_field_names_chirho.insert(
            "BoxChirho".to_owned(),
            haskelujah_typing_chirho::infer_chirho::ConstructorFieldScopeChirho::blocked_chirho(),
        );
        let error_chirho = context_chirho
            .desugar_module_chirho(&module_chirho)
            .expect_err("blocked positioning evidence cannot guess a named field slot Chirho");
        assert!(error_chirho.is_field_account_chirho());
        assert!(error_chirho.constructor_error_chirho().is_none());
        assert!(error_chirho.projection_error_chirho().is_none());
        assert!(error_chirho.ghc_code_chirho().is_none());
        assert_eq!(error_chirho.span_chirho(), SpanChirho::DUMMY_CHIRHO);
    }
}

#[test]
fn blocked_unused_record_wildcard_keeps_the_constructor_arity_chirho() {
    let module_chirho = module_chirho(binding_chirho(PatChirho::RecordChirho {
        con_chirho: dummy_name_chirho("BoxChirho"),
        fields_chirho: vec![],
        has_wildcard_chirho: true,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }));
    let mut context_chirho = DesugarCtxChirho::new_chirho();
    context_chirho
        .con_arities_chirho
        .insert("BoxChirho".to_owned(), 1);
    context_chirho.con_field_names_chirho.insert(
        "BoxChirho".to_owned(),
        haskelujah_typing_chirho::infer_chirho::ConstructorFieldScopeChirho::blocked_chirho(),
    );
    context_chirho
        .desugar_module_chirho(&module_chirho)
        .expect("no named field demands a position or invents a binder Chirho");
}
