// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-policy contracts through the same public file/SourceMap/Batch entry as CLI check.
//! GHC 9.14.1 checked the fixture bytes; the adjacent oracle record keeps those receipts.

fn fixture_path_chirho(relative_chirho: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/tests_chirho/extensions_chirho/policy_chirho/source_policy_fixtures_chirho")
        .join(relative_chirho)
}

fn check_fixture_chirho(
    relative_chirho: &str,
) -> (
    Result<crate::CheckSummaryChirho, crate::DiagnosticBundleChirho>,
    crate::SourceMapChirho,
) {
    let mut source_map_chirho = crate::SourceMapChirho::new_chirho();
    let result_chirho = crate::check_source_path_with_source_map_chirho(
        fixture_path_chirho(relative_chirho),
        crate::ExecutionModeChirho::BatchChirho,
        &mut source_map_chirho,
    );
    (result_chirho, source_map_chirho)
}

#[track_caller]
fn accepts_chirho(relative_chirho: &str) {
    let (result_chirho, source_map_chirho) = check_fixture_chirho(relative_chirho);
    if let Err(diagnostics_chirho) = result_chirho {
        panic!(
            "GHC-accepted source {relative_chirho} was refused: {}",
            crate::render_diagnostics_chirho(&diagnostics_chirho, &source_map_chirho, false)
        );
    }
}

#[track_caller]
fn refuses_for_chirho(relative_chirho: &str, required_chirho: &[&str]) {
    let (result_chirho, source_map_chirho) = check_fixture_chirho(relative_chirho);
    let Err(diagnostics_chirho) = result_chirho else {
        panic!("GHC-refused source {relative_chirho} was accepted");
    };
    // Match diagnostic metadata, never the rendered source excerpt: a pragma itself
    // names the extension even when an unrelated error is the actual refusal.
    let messages_chirho = diagnostics_chirho
        .diagnostics_chirho()
        .iter()
        .filter(|diagnostic_chirho| {
            diagnostic_chirho.severity_chirho
                == haskelujah_diagnostics_chirho::SeverityChirho::ErrorChirho
        })
        .map(|diagnostic_chirho| {
            std::iter::once(diagnostic_chirho.message_chirho.as_str())
                .chain(diagnostic_chirho.notes_chirho.iter().map(String::as_str))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>();
    let rendered_chirho =
        crate::render_diagnostics_chirho(&diagnostics_chirho, &source_map_chirho, false);
    assert!(
        messages_chirho.iter().any(|message_chirho| {
            let message_chirho = message_chirho.to_ascii_lowercase();
            required_chirho
                .iter()
                .all(|token_chirho| message_chirho.contains(&token_chirho.to_ascii_lowercase()))
        }),
        "{relative_chirho} needs its permission/escape reason {required_chirho:?}: {rendered_chirho}"
    );
    for message_chirho in &messages_chirho {
        let message_chirho = message_chirho.to_ascii_lowercase();
        assert!(
            !["resource", "budget", "bound exhausted", "limit exceeded"]
                .iter()
                .any(|token_chirho| message_chirho.contains(token_chirho)),
            "a resource refusal proves no source-policy result: {rendered_chirho}"
        );
    }
}

#[test]
fn explicit_forall_alone_licenses_a_top_level_signature_chirho() {
    accepts_chirho("positive_chirho/explicit_forall_top_chirho/ExplicitForallTopChirho.hs");
}

#[test]
fn rank_n_types_licenses_a_polymorphic_newtype_field_chirho() {
    accepts_chirho("positive_chirho/rank_n_field_chirho/RankNFieldChirho.hs");
}

#[test]
fn haskell2010_datatype_context_keeps_its_monomorphic_field_chirho() {
    accepts_chirho("positive_chirho/datatype_head_context_chirho/DatatypeHeadContextChirho.hs");
}

#[test]
fn a_universal_pattern_variable_may_occur_in_the_result_chirho() {
    accepts_chirho(
        "positive_chirho/universal_pattern_result_chirho/UniversalPatternResultChirho.hs",
    );
}

#[test]
fn a_pattern_existential_may_occur_only_in_an_argument_chirho() {
    accepts_chirho("positive_chirho/existential_argument_chirho/ExistentialArgumentChirho.hs");
}

#[test]
fn scoped_type_variables_does_not_license_a_polymorphic_record_field_chirho() {
    refuses_for_chirho(
        "negative_chirho/scoped_record_no_rank_chirho/ScopedRecordNoRankChirho.hs",
        &["RankNTypes", "forall"],
    );
}

#[test]
fn an_explicit_no_rank_n_types_overrides_the_modern_default_chirho() {
    refuses_for_chirho(
        "negative_chirho/modern_no_rank_chirho/ModernNoRankChirho.hs",
        &["RankNTypes", "forall"],
    );
}

#[test]
fn an_explicit_no_explicit_forall_rejects_a_written_top_level_forall_chirho() {
    refuses_for_chirho(
        "negative_chirho/disabled_forall_top_chirho/DisabledForallTopChirho.hs",
        &["ExplicitForAll", "forall"],
    );
}

#[test]
fn a_pattern_existential_never_escapes_in_its_result_chirho() {
    refuses_for_chirho(
        "frozen_chirho/pattern_existential_chirho/PatSynExistential.hs",
        &["existential", "result"],
    );
}

#[test]
fn haskell2010_does_not_license_explicit_forall_syntax_chirho() {
    refuses_for_chirho(
        "frozen_chirho/legacy_forall_syntax_chirho/tcfail183.hs",
        &["ExplicitForAll", "forall"],
    );
}

#[test]
fn scoped_type_variables_does_not_license_the_frozen_rank_n_field_chirho() {
    refuses_for_chirho(
        "frozen_chirho/legacy_rank_n_field_chirho/tcfail184.hs",
        &["RankNTypes", "forall"],
    );
}
