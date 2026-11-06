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
        matches!(
            error_chirho,
            DesugarInputErrorChirho::UnexpandedPatternChirho { .. }
        ),
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
    input_chirho::check_executable_patterns_chirho(&module_chirho, &mut budget_chirho)
        .expect("borrowed scan copies no output");
    assert_eq!(budget_chirho.remaining_limits_chirho().output_chirho, 0);
    assert!(budget_chirho.remaining_limits_chirho().work_chirho < 100);
    let mut budget_chirho =
        GadtProjectionBudgetChirho::with_limits_chirho(GadtProjectionLimitsChirho {
            work_chirho: 1,
            output_chirho: 0,
            depth_chirho: 128,
        });
    let error_chirho =
        input_chirho::check_executable_patterns_chirho(&module_chirho, &mut budget_chirho)
            .expect_err("the cursor shares the finite allowance");
    let DesugarInputErrorChirho::ProjectionChirho(error_chirho) = error_chirho else {
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
