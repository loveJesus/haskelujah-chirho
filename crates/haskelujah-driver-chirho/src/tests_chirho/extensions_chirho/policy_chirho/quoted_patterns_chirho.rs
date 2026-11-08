// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Fresh GHC 9.14.1 source-stage controls. These test parsing/frontend
//! admission, not the held TH evaluator or Code result-type contract.
use super::ordinary_elaboration_chirho::frontend_chirho;

#[test]
fn quoted_pattern_payloads_are_preserved_under_the_real_quote_license_chirho() {
    frontend_chirho(include_str!("../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/patterns_chirho/PatternSpliceQuotesChirho.hs"))
        .expect("GHC accepts Quotes-only pattern splices inside a quote");
    frontend_chirho(include_str!("../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/patterns_chirho/PatternSpliceUntypedBodyChirho.hs"))
        .expect("an untyped quote does not typecheck its data as executable code");
}

#[test]
fn malformed_or_unlicensed_quoted_patterns_stay_fatal_chirho() {
    for source_chirho in [
        include_str!(
            "../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/patterns_chirho/PatternSpliceDisabledChirho.hs"
        ),
        include_str!(
            "../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/patterns_chirho/PatternSpliceMalformedChirho.hs"
        ),
        include_str!(
            "../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/staging_chirho/EmptyQuotedPatternPayloadChirho.hs"
        ),
    ] {
        let error_chirho = frontend_chirho(source_chirho)
            .err()
            .expect("invalid quoted source must refuse");
        assert!(error_chirho.contains("error[E"), "{error_chirho}");
    }
}

#[test]
fn type_annotation_checking_respects_typed_and_untyped_quote_stages_chirho() {
    frontend_chirho(include_str!("../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/staging_chirho/WrongKindUntypedAnnotationChirho.hs"))
        .expect("GHC accepts an ill-kinded annotation retained in untyped syntax");
    let error_chirho = frontend_chirho(include_str!("../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/staging_chirho/TypedWrongAnnotationChirho.hs"))
        .err().expect("a typed quote checks its annotation");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
    frontend_chirho(include_str!("../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/staging_chirho/TypedCorrectAnnotationChirho.hs"))
        .expect("the correctly kinded typed control stays accepted");
    frontend_chirho(include_str!("../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/staging_chirho/BoundQuotedLambdaChirho.hs"))
        .expect("quoted lambda syntax stays accepted");
}

#[test]
fn executable_pattern_splice_refusal_remains_coded_and_non_deferrable_chirho() {
    let error_chirho = frontend_chirho("{-# LANGUAGE TemplateHaskell #-}\n{-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule PatternUnsupportedChirho where\nimport Language.Haskell.TH\nfChirho $(wildP) = ()\n")
        .err().expect("an unexpanded executable splice is explicitly unsupported");
    assert!(
        error_chirho.contains("unsupported unexpanded executable pattern splice"),
        "{error_chirho}"
    );
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn original_t13984_source_is_admitted_without_rewriting_the_oracle_chirho() {
    frontend_chirho(include_str!(
        "../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/patterns_chirho/T13984.hs"
    ))
    .expect("the exact retained GHC-positive source reaches ordinary inference");
}

#[test]
fn lowering_retains_pattern_payload_span_and_quote_specificity_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_ast_chirho::expr_chirho::{ExprChirho, RhsChirho};
    use haskelujah_ast_chirho::pat_chirho::PatChirho;
    let source_chirho = "module PreserveChirho where\nquotedChirho = [p| $(wildP) |]\n";
    let parser_chirho = crate::ParserChirho::new_chirho(
        source_chirho,
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    );
    let module_chirho = crate::lower_module_chirho(
        &parser_chirho.parse_chirho(),
        haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
    );
    let DeclChirho::FunBindChirho { matches_chirho, .. } = &module_chirho.decls_chirho[0] else {
        panic!("quoted binding")
    };
    let RhsChirho::UnguardedChirho(ExprChirho::QuotePatChirho { pat_chirho, .. }) =
        &matches_chirho[0].rhs_chirho
    else {
        panic!("pattern quote")
    };
    let PatChirho::SpliceChirho {
        expr_chirho: Some(payload_chirho),
        span_chirho,
    } = pat_chirho
    else {
        panic!("complete pattern-splice payload")
    };
    assert!(
        matches!(payload_chirho.as_ref(), ExprChirho::VarChirho(name_chirho) if name_chirho.text_chirho() == "wildP")
    );
    let range_chirho =
        span_chirho.start_chirho().as_usize_chirho()..span_chirho.end_chirho().as_usize_chirho();
    assert_eq!(&source_chirho[range_chirho], "$(wildP)");
    for (quotation_chirho, expected_chirho) in [("[| () |]", false), ("[|| () ||]", true)] {
        let source_chirho =
            format!("module PreserveChirho where\nquotedChirho = {quotation_chirho}\n");
        let parser_chirho = crate::ParserChirho::new_chirho(
            &source_chirho,
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
        );
        let module_chirho = crate::lower_module_chirho(
            &parser_chirho.parse_chirho(),
            haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO,
        );
        let DeclChirho::FunBindChirho { matches_chirho, .. } = &module_chirho.decls_chirho[0]
        else {
            panic!("quoted binding")
        };
        let RhsChirho::UnguardedChirho(ExprChirho::QuoteExprChirho { typed_chirho, .. }) =
            &matches_chirho[0].rhs_chirho
        else {
            panic!("expression quote")
        };
        assert_eq!(*typed_chirho, expected_chirho);
    }
}

#[test]
fn quoted_headers_remain_bounded_syntax_without_outer_signature_proofs_chirho() {
    use haskelujah_ast_chirho::decl_chirho::DeclChirho;
    use haskelujah_ast_chirho::expr_chirho::{ExprChirho, LocalBindChirho, RhsChirho};
    use haskelujah_ast_chirho::ty_chirho::TypeChirho;
    let result_chirho = frontend_chirho(include_str!("../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/headers_chirho/LocalSignatureQuotationBracesChirho.hs"))
        .expect("the separately GHC-accepted braced signature quote is syntax data");
    let declaration_chirho = result_chirho.module_chirho.decls_chirho.iter().find(|declaration_chirho| matches!(declaration_chirho, DeclChirho::FunBindChirho { name_chirho, .. } if name_chirho.text_chirho() == "quotationChirho")).expect("actual quotation binding");
    let DeclChirho::FunBindChirho { matches_chirho, .. } = declaration_chirho else {
        unreachable!()
    };
    let RhsChirho::UnguardedChirho(ExprChirho::QuoteExprChirho {
        expr_chirho,
        typed_chirho: false,
        ..
    }) = &matches_chirho[0].rhs_chirho
    else {
        panic!("untyped quote")
    };
    let ExprChirho::LetChirho { binds_chirho, .. } = expr_chirho.as_ref() else {
        panic!("retained local declarations")
    };
    let LocalBindChirho::TypeSigChirho {
        ty_chirho: TypeChirho::ForallChirho { vars_chirho, .. },
        ..
    } = &binds_chirho[0]
    else {
        panic!("retained explicit signature")
    };
    assert!(
        vars_chirho[0].name_chirho.type_variable_chirho().is_none(),
        "quotation syntax cannot acquire a checked outer owner"
    );
    for source_chirho in [
        include_str!(
            "../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/headers_chirho/DeclarationHeaderQuotationBracesChirho.hs"
        ),
        include_str!(
            "../kind_repair_fixtures_chirho/syntax_chirho/quotations_chirho/headers_chirho/TypeSyntaxQuotationChirho.hs"
        ),
    ] {
        frontend_chirho(source_chirho).expect("fresh GHC accepts these raw quotation headers");
    }
}
