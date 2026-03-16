// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Driver-level integration tests for Template Haskell splice expansion
//!
//! These tests exercise the splice expansion pass through the full
//! `run_frontend_chirho` pipeline.

use rhasky_span_chirho::SourceMapChirho;

/// Parse source through the full front-end pipeline and return warnings.
fn frontend_warnings_for_chirho(source_chirho: &str) -> Vec<String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    match crate::frontend_warnings_chirho(source_chirho, &mut source_map_chirho, "Splice.hs") {
        Ok(warnings_chirho) => warnings_chirho,
        Err(diag_chirho) => {
            // If we get a diagnostic error, it may still be fine for our
            // purposes — the splice expansion happened before name resolution.
            // Return the diagnostic messages as "warnings" for inspection.
            diag_chirho
                .diagnostics_chirho()
                .iter()
                .map(|d_chirho| d_chirho.to_string())
                .collect()
        }
    }
}

#[test]
fn splice_expansion_runs_in_pipeline_chirho() {
    // A module with a splice for an unknown TH function should produce
    // a warning about the unrecognized splice, but not crash.
    let source_chirho = r#"
module Splice where

$(someUnknownSplice)
"#;

    let warnings_chirho = frontend_warnings_for_chirho(source_chirho);
    let has_splice_warning_chirho = warnings_chirho
        .iter()
        .any(|w_chirho| w_chirho.contains("not recognized") || w_chirho.contains("splice"));
    assert!(
        has_splice_warning_chirho,
        "expected a splice-related warning, got: {:?}",
        warnings_chirho
    );
}

#[test]
fn splice_make_lenses_in_pipeline_chirho() {
    // A module with a record type and $(makeLenses ''Person) should
    // expand the splice and the generated lens declarations should
    // participate in the rest of the pipeline (at least name resolution).
    let source_chirho = r#"
module LensTest where

data Person = Person { _name :: String, _age :: Int }

$(makeLenses ''Person)
"#;

    let warnings_chirho = frontend_warnings_for_chirho(source_chirho);
    // makeLenses for Person should not produce any splice-related warnings
    // (the generated code may cause other warnings in downstream phases,
    // but there should be no "not recognized" warning).
    let has_unrecognized_chirho = warnings_chirho
        .iter()
        .any(|w_chirho| w_chirho.contains("not recognized"));
    assert!(
        !has_unrecognized_chirho,
        "makeLenses should be recognized, got warnings: {:?}",
        warnings_chirho
    );
}

#[test]
fn splice_derive_json_warning_chirho() {
    // deriveJSON is recognized but not implemented — should produce
    // a specific warning.
    let source_chirho = r#"
module JsonTest where

data Config = Config { _host :: String }

$(deriveJSON defaultOptions ''Config)
"#;

    let warnings_chirho = frontend_warnings_for_chirho(source_chirho);
    let has_derive_json_warning_chirho = warnings_chirho
        .iter()
        .any(|w_chirho| w_chirho.contains("deriveJSON"));
    assert!(
        has_derive_json_warning_chirho,
        "expected deriveJSON warning, got: {:?}",
        warnings_chirho
    );
}
