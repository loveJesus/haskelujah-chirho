// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::{NextErrorKindChirho, check_source_chirho, eval_binding_chirho};
use haskelujah_runtime_chirho::ValueChirho;

fn fixture_chirho(group_chirho: &str, name_chirho: &str) -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/frontend_chirho/next_chirho/fixtures_chirho/list_operators_chirho")
            .join(group_chirho)
            .join(format!("{name_chirho}.hs")),
    )
    .unwrap()
}

fn integer_chirho(source_chirho: &str, entry_chirho: &str, expected_chirho: &str) {
    let (value_chirho, _) = eval_binding_chirho(source_chirho, entry_chirho, 200_000)
        .unwrap_or_else(|error_chirho| panic!("{entry_chirho}: {error_chirho}"));
    assert!(matches!(value_chirho, ValueChirho::IntegerChirho(_)));
    assert_eq!(value_chirho.to_string(), expected_chirho, "{entry_chirho}");
}

#[test]
fn resolved_fixities_preserve_precedence_association_and_parentheses_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "ArithmeticGroupingChirho");
    for (entry_chirho, expected_chirho) in [
        ("leftChirho", "-4"),
        ("leftParenChirho", "-4"),
        ("rightParenChirho", "2"),
        ("precedenceChirho", "7"),
        ("parenthesizedNegativeChirho", "-5"),
    ] {
        integer_chirho(&source_chirho, entry_chirho, expected_chirho);
    }
    for (name_chirho, expected_chirho) in [
        ("ExplicitRightChirho", "2"),
        ("BackquotedDefaultChirho", "-4"),
        ("ResolvedParameterChirho", "-4"),
    ] {
        integer_chirho(
            &fixture_chirho("positive-chirho", name_chirho),
            "resultChirho",
            expected_chirho,
        );
    }
}

#[test]
fn wired_cons_and_pure_cons_patterns_work_without_prelude_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "ConsWithoutPreludeChirho");
    for (entry_chirho, expected_chirho) in [
        ("resultChirho", 'q'),
        ("secondResultChirho", 'b'),
        ("constructorResultChirho", 'c'),
    ] {
        let (value_chirho, _) = eval_binding_chirho(&source_chirho, entry_chirho, 100_000).unwrap();
        assert_eq!(value_chirho, ValueChirho::CharChirho(expected_chirho));
    }
}

#[test]
fn upstream_append_joins_strings_lists_and_an_unevaluated_tail_chirho() {
    let source_chirho = fixture_chirho("positive-chirho", "AppendSourceChirho");
    for (entry_chirho, expected_chirho) in [
        ("resultChirho", 'a'),
        ("secondResultChirho", 'b'),
        ("lazyFirstChirho", 'a'),
        ("lazySecondChirho", 'b'),
        ("emptyLeftChirho", 'q'),
    ] {
        let (value_chirho, _) = eval_binding_chirho(&source_chirho, entry_chirho, 200_000)
            .unwrap_or_else(|error_chirho| panic!("{entry_chirho}: {error_chirho}"));
        assert_eq!(value_chirho, ValueChirho::CharChirho(expected_chirho));
    }
    integer_chirho(&source_chirho, "chainChirho", "4");
    // Evaluating an unsaturated append must not enter its bottom first argument.
    eval_binding_chirho(&source_chirho, "partialChirho", 200_000).unwrap();
}

#[test]
fn demanded_append_inputs_keep_the_source_failure_chirho() {
    let source_chirho = fixture_chirho("failure-chirho", "AppendDemandChirho");
    for entry_chirho in ["leftChirho", "tailChirho"] {
        let error_chirho = eval_binding_chirho(&source_chirho, entry_chirho, 200_000)
            .err()
            .unwrap();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::ExecutionChirho
        );
        assert!(
            error_chirho
                .message_chirho
                .contains("no matching alternative"),
            "{error_chirho}"
        );
    }
}

#[test]
fn only_proved_fixity_conflicts_are_semantic_refusals_chirho() {
    for name_chirho in ["PrecedenceConflictChirho", "MixedAssociativityChirho"] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("negative-chirho", name_chirho)).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::InvalidChirho,
            "{error_chirho}"
        );
        assert!(
            error_chirho.message_chirho.contains("precedence conflict"),
            "{error_chirho}"
        );
    }
}

#[test]
fn scoped_fixity_pattern_chains_and_unowned_pragmas_are_not_lost_chirho() {
    for (name_chirho, message_chirho) in [
        ("LocalFixityChirho", "fixity"),
        ("LetFixityChirho", "fixity"),
        ("MixedPatternChirho", "pattern chain"),
        ("UserNoInlineChirho", "pragma"),
        ("NegatedChainChirho", "negation"),
    ] {
        let error_chirho =
            check_source_chirho(&fixture_chirho("boundary-chirho", name_chirho)).unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::UnsupportedChirho,
            "{name_chirho}: {error_chirho}"
        );
        assert!(
            error_chirho.message_chirho.contains(message_chirho),
            "{name_chirho}: {error_chirho}"
        );
    }
}

#[test]
fn defining_fixity_crosses_reexports_but_a_facade_cannot_invent_one_chirho() {
    for (names_chirho, entry_chirho, expected_chirho) in [
        (
            vec!["ALibChirho", "BFacadeChirho", "CUseChirho"],
            "CUseChirho",
            "2",
        ),
        (
            vec!["ADefaultChirho", "CDefaultUseChirho"],
            "CDefaultUseChirho",
            "-4",
        ),
    ] {
        let sources_chirho: Vec<_> = names_chirho
            .into_iter()
            .map(|name_chirho| fixture_chirho("imports-chirho", name_chirho))
            .collect();
        let sources_chirho: Vec<_> = sources_chirho.iter().map(String::as_str).collect();
        let (value_chirho, _) = super::eval_sources_binding_chirho(
            &sources_chirho,
            entry_chirho,
            "resultChirho",
            200_000,
        )
        .unwrap();
        assert!(matches!(value_chirho, ValueChirho::IntegerChirho(_)));
        assert_eq!(value_chirho.to_string(), expected_chirho);
    }
    let sources_chirho = [
        fixture_chirho("imports-chirho", "ASliceFacadeChirho"),
        fixture_chirho("imports-chirho", "CSliceUseChirho"),
    ];
    let sources_chirho: Vec<_> = sources_chirho.iter().map(String::as_str).collect();
    let error_chirho = super::check_sources_chirho(&sources_chirho, "CSliceUseChirho").unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho,
        "{error_chirho}"
    );
    assert!(
        error_chirho.message_chirho.contains("fixity"),
        "{error_chirho}"
    );
}

#[test]
fn builtin_syntax_does_not_mint_list_constructor_export_scope_chirho() {
    let error_chirho =
        check_source_chirho(&fixture_chirho("imports-chirho", "AConsChirho")).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
    let sources_chirho = [
        fixture_chirho("imports-chirho", "AConsChirho"),
        fixture_chirho("imports-chirho", "CConsUseChirho"),
    ];
    let sources_chirho: Vec<_> = sources_chirho.iter().map(String::as_str).collect();
    assert_eq!(
        super::check_sources_chirho(&sources_chirho, "CConsUseChirho")
            .unwrap_err()
            .kind_chirho,
        NextErrorKindChirho::UnsupportedChirho
    );
}

#[test]
fn bare_infix_function_head_is_refused_before_recovery_chirho() {
    let error_chirho =
        check_source_chirho(&fixture_chirho("boundary-chirho", "BareHeadChirho")).unwrap_err();
    // The reused CST already reports this GHC precedence error. If later parsed,
    // HIR's bare-head gate still declines until checkPrecMatch is implemented.
    assert!(matches!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::InvalidChirho | NextErrorKindChirho::UnsupportedChirho
    ));
}

#[test]
fn large_resolved_chain_stops_at_the_owned_depth_ceiling_chirho() {
    let chain_chirho = vec!["'x'"; 256].join(" <+> ");
    let source_chirho = format!(
        "{{-# LANGUAGE NoImplicitPrelude #-}}\nmodule LongChainChirho where\naChirho <+> _ = aChirho\nresultChirho = {chain_chirho}\n"
    );
    let error_chirho = check_source_chirho(&source_chirho).unwrap_err();
    assert_eq!(
        error_chirho.kind_chirho,
        NextErrorKindChirho::ResourceChirho,
        "{error_chirho}"
    );
}

#[test]
fn malformed_fixity_selection_is_not_a_verdict_on_a_user_program_chirho() {
    use haskelujah_hir_chirho::{CompletenessChirho, LoweringPolicyChirho, UnitIdentityChirho};
    use haskelujah_span_chirho::FileIdChirho;
    for declarations_chirho in [
        "infixl 10 <+>\naChirho <+> _ = aChirho\n",
        "infixr 5 missingChirho\n",
        "infixl 5 <+>\ninfixl 5 <+>\naChirho <+> _ = aChirho\n",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE NoImplicitPrelude #-}}\nmodule FixityPolicyChirho where\n{declarations_chirho}"
        );
        for (unit_chirho, expected_chirho) in [
            (
                UnitIdentityChirho::StandaloneHomeChirho,
                NextErrorKindChirho::InvalidChirho,
            ),
            (
                UnitIdentityChirho::CompilerChirho,
                NextErrorKindChirho::UnsupportedChirho,
            ),
        ] {
            let parsed_chirho =
                super::parse_chirho::parse_chirho(&source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
                    .expect("syntactically represented fixity");
            let error_chirho = super::check_parsed_with_policy_chirho(
                parsed_chirho,
                unit_chirho,
                &Default::default(),
                &LoweringPolicyChirho {
                    completeness_chirho: CompletenessChirho::SliceChirho,
                    ..Default::default()
                },
            )
            .unwrap_err();
            assert_eq!(error_chirho.kind_chirho, expected_chirho, "{error_chirho}");
        }
    }
}
