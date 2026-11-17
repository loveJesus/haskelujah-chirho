// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Real source hierarchy, dictionary dispatch, lazy completion and cumulative output failures.
use super::*;

fn fixture_chirho(group_chirho: &str, name_chirho: &str) -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/frontend_chirho/next_chirho/fixtures_chirho/io_sequencing_chirho")
            .join(group_chirho)
            .join(format!("{name_chirho}.hs")),
    )
    .unwrap()
}
fn output_chirho(group_chirho: &str, name_chirho: &str, expected_chirho: &str) {
    let outcome_chirho = run_main_chirho(
        &fixture_chirho(group_chirho, name_chirho),
        Default::default(),
    )
    .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
    outcome_chirho
        .result_chirho
        .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
    assert_eq!(
        outcome_chirho.machine_chirho.io_output_chirho, expected_chirho,
        "{name_chirho}"
    );
}
#[test]
fn actual_source_hierarchy_sequences_and_preserves_lazy_payloads_chirho() {
    for (name_chirho, expected_chirho) in [
        ("TwoLinesChirho", "a\nb\n"),
        ("BoundNumberChirho", "4\n"),
        ("LazyBindChirho", "ok\n"),
        ("LazyThenChirho", "ok\n"),
        ("RepeatedActionChirho", "x\nx\n"),
        ("StoredSequenceChirho", "ok\n"),
        ("FmapResultChirho", "2\n"),
        ("FmapLazyChirho", "ok\n"),
        ("ApplicativeResultChirho", "3\n"),
    ] {
        output_chirho("positive-chirho", name_chirho, expected_chirho);
    }
}
#[test]
fn do_uses_known_originals_and_preserves_statement_scope_chirho() {
    for (name_chirho, expected_chirho) in [
        ("NoPreludeDoChirho", "a\nb\n"),
        ("HomeBindShadowChirho", "3\n"),
        ("DoLetScopeChirho", "4\n"),
    ] {
        output_chirho("scope-chirho", name_chirho, expected_chirho);
    }
    let outcome_chirho = run_main_chirho(
        "main = do { _ <- return (5 :: Integer); putStrLn \"ok\" }\n",
        Default::default(),
    )
    .unwrap();
    outcome_chirho.result_chirho.unwrap();
    assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "ok\n");
}
#[test]
fn method_local_quantifiers_are_fresh_at_uses_and_instances_chirho() {
    output_chirho("scope-chirho", "MethodPolymorphismChirho", "3\n");
    output_chirho("scope-chirho", "HomeHigherClassChirho", "3\n");
}
#[test]
fn completion_does_not_force_main_or_bound_payload_chirho() {
    output_chirho("scope-chirho", "LazyMainResultChirho", "");
    let outcome_chirho = run_main_chirho("main = do { x_chirho <- return (let loop_chirho = loop_chirho in loop_chirho :: Integer); putStrLn \"ok\" }\n",Default::default()).unwrap();
    outcome_chirho.result_chirho.unwrap();
    assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "ok\n");
}
#[test]
fn sequence_failure_retains_prior_commits_and_discards_pending_output_chirho() {
    for (name_chirho, expected_chirho) in [
        ("BottomActionChirho", ""),
        ("BottomContinuationChirho", "a\n"),
        ("MiddleFailureChirho", "a\n"),
        ("BufferedSecondFailureChirho", "first\n"),
    ] {
        let outcome_chirho = run_main_chirho(
            &fixture_chirho("failure-chirho", name_chirho),
            Default::default(),
        )
        .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
        let error_chirho = outcome_chirho.result_chirho.unwrap_err();
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::ExecutionChirho,
            "{name_chirho}: {error_chirho}"
        );
        assert_eq!(
            outcome_chirho.machine_chirho.io_output_chirho, expected_chirho,
            "{name_chirho}"
        );
    }
}
#[test]
fn cumulative_output_and_step_allowances_do_not_reset_between_actions_chirho() {
    for (limit_chirho, success_chirho) in [(3, false), (4, true)] {
        let outcome_chirho = run_main_chirho(
            &fixture_chirho("positive-chirho", "TwoLinesChirho"),
            RunLimitsChirho {
                output_bytes_chirho: limit_chirho,
                ..Default::default()
            },
        )
        .unwrap();
        if success_chirho {
            outcome_chirho.result_chirho.unwrap();
            assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "a\nb\n");
        } else {
            assert_eq!(
                outcome_chirho.result_chirho.unwrap_err().kind_chirho,
                NextErrorKindChirho::ResourceChirho
            );
            assert_eq!(outcome_chirho.machine_chirho.io_output_chirho, "a\n");
        }
    }
    let outcome_chirho = run_main_chirho(
        &fixture_chirho("positive-chirho", "TwoLinesChirho"),
        RunLimitsChirho {
            steps_chirho: 1,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        outcome_chirho.result_chirho.unwrap_err().kind_chirho,
        NextErrorKindChirho::ResourceChirho
    );
}
#[test]
fn invalid_final_binds_and_kind_heads_are_refused_without_monadfail_stubs_chirho() {
    for (name_chirho, kind_chirho) in [
        ("WrongClassKindChirho", NextErrorKindChirho::TypeChirho),
        ("LastBindChirho", NextErrorKindChirho::InvalidChirho),
        (
            "RefutableBindChirho",
            NextErrorKindChirho::UnsupportedChirho,
        ),
    ] {
        let error_chirho = match run_main_chirho(
            &fixture_chirho(
                if name_chirho == "RefutableBindChirho" {
                    "boundary-chirho"
                } else {
                    "negative-chirho"
                },
                name_chirho,
            ),
            Default::default(),
        ) {
            Err(error_chirho) => error_chirho,
            Ok(_) => panic!("{name_chirho} unexpectedly compiled"),
        };
        assert_eq!(
            error_chirho.kind_chirho, kind_chirho,
            "{name_chirho}: {error_chirho}"
        );
    }
}
#[test]
fn hidden_do_dependency_does_not_expose_prelude_values_chirho() {
    let source_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nimport System.IO (putStrLn)\nmain = do { putStrLn \"a\"; return () }\n";
    let error_chirho = match run_main_chirho(source_chirho, Default::default()) {
        Err(error_chirho) => error_chirho,
        Ok(_) => panic!("hidden Base dependency exposed return"),
    };
    assert_eq!(error_chirho.kind_chirho, NextErrorKindChirho::InvalidChirho);
}

#[test]
fn superclass_kind_groups_default_before_consumers_chirho() {
    for (group_chirho, name_chirho, expected_chirho) in [
        (
            "negative-chirho",
            "ClassGroupH98Chirho",
            NextErrorKindChirho::TypeChirho,
        ),
        (
            "boundary-chirho",
            "ClassGroupPolyChirho",
            NextErrorKindChirho::UnsupportedChirho,
        ),
        (
            "negative-chirho",
            "SignatureKindH98Chirho",
            NextErrorKindChirho::TypeChirho,
        ),
        (
            "boundary-chirho",
            "SignatureKindPolyChirho",
            NextErrorKindChirho::UnsupportedChirho,
        ),
    ] {
        let error_chirho = match run_main_chirho(
            &fixture_chirho(group_chirho, name_chirho),
            Default::default(),
        ) {
            Err(error_chirho) => error_chirho,
            Ok(_) => panic!("{name_chirho} unexpectedly compiled"),
        };
        assert_eq!(
            error_chirho.kind_chirho, expected_chirho,
            "{name_chirho}: {error_chirho}"
        );
    }
}
#[test]
fn only_explicit_binders_in_the_scoped_edition_scope_over_bodies_chirho() {
    for name_chirho in ["ImplicitMethodScopeChirho", "ForallWithoutScopedChirho"] {
        let error_chirho = match run_main_chirho(
            &fixture_chirho("negative-chirho", name_chirho),
            Default::default(),
        ) {
            Err(error_chirho) => error_chirho,
            Ok(_) => panic!("{name_chirho} unexpectedly compiled"),
        };
        assert_eq!(
            error_chirho.kind_chirho,
            NextErrorKindChirho::TypeChirho,
            "{name_chirho}: {error_chirho}"
        );
    }
}

#[test]
fn open_phantom_data_kinds_decline_and_h98_fixed_kinds_refuse_chirho() {
    for (group_chirho, name_chirho, expected_chirho) in [
        (
            "boundary-chirho",
            "PhantomClassPolyChirho",
            NextErrorKindChirho::UnsupportedChirho,
        ),
        (
            "boundary-chirho",
            "PhantomUsePolyChirho",
            NextErrorKindChirho::UnsupportedChirho,
        ),
        (
            "boundary-chirho",
            "RecursiveHeaderH98Chirho",
            NextErrorKindChirho::UnsupportedChirho,
        ),
        (
            "negative-chirho",
            "PhantomClassH98Chirho",
            NextErrorKindChirho::TypeChirho,
        ),
        (
            "negative-chirho",
            "PhantomUseH98Chirho",
            NextErrorKindChirho::TypeChirho,
        ),
    ] {
        let error_chirho = match run_main_chirho(
            &fixture_chirho(group_chirho, name_chirho),
            Default::default(),
        ) {
            Err(error_chirho) => error_chirho,
            Ok(_) => panic!("{name_chirho} unexpectedly compiled"),
        };
        assert_eq!(
            error_chirho.kind_chirho, expected_chirho,
            "{name_chirho}: {error_chirho}"
        );
    }
    output_chirho("positive-chirho", "PinnedDataPositionsChirho", "3\n");
}
