// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Universal annotation controls independently measured with GHC9.14.1.

use super::ordinary_elaboration_chirho::frontend_chirho;

#[test]
fn qualified_constructor_arguments_accept_original_tc092_chirho() {
    haskelujah_driver_original_tc092_chirho()
        .unwrap_or_else(|error_chirho| panic!("GHC accepts original tc092: {error_chirho}"));
}

fn haskelujah_driver_original_tc092_chirho() -> Result<(), String> {
    crate::typecheck_source_chirho(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../ghc-tests-chirho/typecheck-chirho/should_compile/tc092.hs"
        )),
        &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(),
        "tc092.hs",
    )
    .map(|_| ())
    .map_err(|error_chirho| error_chirho.to_string())
}

#[test]
fn qualified_constructor_arguments_keep_literals_annotations_and_named_values_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/QualifiedConstructorArgumentChirho.hs"
    ))
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn qualified_constructor_arguments_keep_explicit_type_abstractions_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/ExplicitTypeSigmaArgumentChirho.hs"
    ))
    .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn qualified_constructor_arguments_reject_specialization_missing_evidence_and_escape_chirho() {
    for source_chirho in [
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/SpecializedSigmaArgumentChirho.hs"
        ),
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/MissingSigmaEvidenceChirho.hs"
        ),
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/EscapingSigmaArgumentChirho.hs"
        ),
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/ExplicitEscapeSigmaArgumentChirho.hs"
        ),
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/ExplicitSpecializedSigmaArgumentChirho.hs"
        ),
    ] {
        let error_chirho = frontend_chirho(source_chirho)
            .err()
            .expect("the argument must serve every rigid type with only its own givens");
        assert!(error_chirho.contains("error[E020"), "{error_chirho}");
    }
}

#[test]
fn qualified_constructor_arguments_open_the_sigma_before_checking_branches_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/sigma-arguments-chirho/BranchTypeSigmaArgumentChirho.hs"
    ))
    .err()
    .expect("GHC rejects a type abstraction inside an already opened branch");
    assert!(
        error_chirho.contains("invisible type abstraction requires an expected forall"),
        "{error_chirho}"
    );
}

#[test]
fn annotation_universal_annotation_positive_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/UniversalAnnotationPositiveChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_captured_annotation_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/CapturedAnnotationChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC universal negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_specialized_annotation_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/SpecializedAnnotationChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC universal negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_scoped_annotation_shadow_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/ScopedAnnotationShadowChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_annotation_given_generalized_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotationGivenGeneralizedChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_annotation_given_negative_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotationGivenNegativeChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("fresh GHC universal negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_annotation_visible_type_application_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotationVisibleTypeApplicationChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_annotated_monomorphic_vta_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotatedMonomorphicVtaChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("actual GHC visible type application negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_annotated_excess_vta_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotatedExcessVtaChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("actual GHC visible type application negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_annotated_inferred_selection_positive_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotatedInferredSelectionPositiveChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_annotated_inferred_selection_wrong_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/annotations-chirho/AnnotatedInferredSelectionWrongChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("actual GHC visible type application negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_method_class_positive_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/pattern-contracts-chirho/MethodClassPositiveChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_method_class_wrong_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/pattern-contracts-chirho/MethodClassWrongChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("actual GHC source-class VTA negative rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn annotation_constructor_do_shadow_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/pattern-contracts-chirho/ConstructorDoShadowChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_constructor_list_scope_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/pattern-contracts-chirho/ConstructorListScopeChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_constructor_guard_scope_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/pattern-contracts-chirho/ConstructorGuardScopeChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_structured_pattern_positive_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/pattern-contracts-chirho/StructuredPatternPositiveChirho.hs"
    ));
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn annotation_structured_pattern_wrong_chirho() {
    let result_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/pattern-contracts-chirho/StructuredPatternWrongChirho.hs"
    ));
    let error_chirho = result_chirho
        .err()
        .expect("actual GHC wrong constructor typepattern rejects");
    assert!(error_chirho.contains("error[E020"), "{error_chirho}");
}

#[test]
fn malformed_layout_is_a_non_deferrable_syntax_error_chirho() {
    let source_chirho = include_str!(
        "kind_repair_fixtures_chirho/impredicative-application-chirho/syntax-chirho/ConstructorStructuredShadowChirho.hs"
    );
    for header_chirho in [
        "",
        "{-# OPTIONS_GHC -fdefer-type-errors -fdefer-typed-holes -fdefer-out-of-scope-variables #-}\n",
    ] {
        let error_chirho = frontend_chirho(&format!("{header_chirho}{source_chirho}"))
            .err()
            .expect("syntax cannot be deferred or lowered into a placeholder value");
        assert!(error_chirho.contains("error[E0002]"), "{error_chirho}");
    }
}

#[test]
fn aligned_and_explicit_case_blocks_keep_valid_syntax_chirho() {
    for source_chirho in [
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/syntax-chirho/ConstructorIndentedShadowChirho.hs"
        ),
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/syntax-chirho/ExplicitEmptyCasePositiveChirho.hs"
        ),
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/syntax-chirho/SiblingCaseLetAlignedChirho.hs"
        ),
    ] {
        frontend_chirho(source_chirho).unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
    }
}

#[test]
fn inferred_results_accept_original_qualified_elements_and_ordinary_case_branches_chirho() {
    for (name_chirho, source_chirho) in [
        (
            "T2846.hs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T2846.hs"
            )),
        ),
        (
            "T18412.hs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T18412.hs"
            )),
        ),
    ] {
        crate::typecheck_source_chirho(
            source_chirho,
            &mut haskelujah_span_chirho::SourceMapChirho::new_chirho(),
            name_chirho,
        )
        .unwrap_or_else(|error_chirho| {
            panic!("GHC accepts original {name_chirho}: {error_chirho}")
        });
    }
}

#[test]
fn inferred_results_preserve_a_higher_rank_branch_result_chirho() {
    frontend_chirho(include_str!("kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/InferredResultRuntimeChirho.hs"))
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn inferred_results_do_not_turn_parameters_recursion_or_implications_into_polytypes_chirho() {
    for (name_chirho, source_chirho) in [
        (
            "ordinary parameter",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/OrdinaryParameterChirho.hs"
            ),
        ),
        (
            "recursive result",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/RecursiveResultChirho.hs"
            ),
        ),
        (
            "existential after ordinary",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/ExistentialAfterOrdinaryChirho.hs"
            ),
        ),
        (
            "existential before ordinary",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/ExistentialBeforeOrdinaryChirho.hs"
            ),
        ),
        (
            "already equal GADT index",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/GadtEqualIndexChirho.hs"
            ),
        ),
        (
            "case eta",
            include_str!(
                "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/CaseEtaChirho.hs"
            ),
        ),
    ] {
        let error_chirho = frontend_chirho(source_chirho)
            .err()
            .unwrap_or_else(|| panic!("{name_chirho} must retain GHC's monotype restriction"));
        assert!(
            error_chirho.contains("error[E020"),
            "{name_chirho}: {error_chirho}"
        );
    }
}

#[test]
fn inferred_results_do_not_publish_a_later_gadt_alternatives_refinement_chirho() {
    for source_chirho in [
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/RefinementLeakChirho.hs"
        ),
        include_str!(
            "kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/RefinementRhsLeakChirho.hs"
        ),
    ] {
        let error_chirho = frontend_chirho(source_chirho)
            .err()
            .expect("a branch-local equality cannot change the public result");
        assert!(error_chirho.contains("error[E020"), "{error_chirho}");
    }
}

#[test]
fn inferred_results_preserve_an_outer_result_under_existential_refinement_chirho() {
    frontend_chirho(include_str!("kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/RefinementEscapeChirho.hs"))
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn inferred_results_keep_user_ordinary_constructor_patterns_polymorphic_chirho() {
    frontend_chirho(include_str!("kind_repair_fixtures_chirho/impredicative-application-chirho/inferred-results-chirho/OrdinaryConstructorResultChirho.hs"))
        .unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn inferred_results_require_rank_n_permission_at_the_importing_binding_chirho() {
    let path_chirho = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../ghc-tests-chirho/typecheck-chirho/should_fail/T17213.hs");
    let error_chirho =
        crate::check_source_path_chirho(path_chirho, crate::ExecutionModeChirho::BatchChirho)
            .err()
            .expect("the importing Haskell2010 module cannot infer a rank-N binding")
            .to_string();
    assert!(error_chirho.contains("RankNTypes"), "{error_chirho}");

    frontend_chirho(
        "{-# LANGUAGE RankNTypes #-}\nmodule InferredRankChirho where\nrankChirho :: (forall aChirho. aChirho -> aChirho) -> Int\nrankChirho _ = 3\ninferredChirho = rankChirho\n",
    )
    .unwrap_or_else(|error_chirho| panic!("RankNTypes licenses the inferred type: {error_chirho}"));
}
