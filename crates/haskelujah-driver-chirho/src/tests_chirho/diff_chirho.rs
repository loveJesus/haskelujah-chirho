// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Differential Testing Against GHC (§62)
//!
//! Runs the diff test suite through the full Haskelujah pipeline and
//! compares results against pre-recorded GHC reference answers.

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::diff_chirho::{
    diff_test_cases_chirho, run_diff_suite_chirho, CompileOutcomeChirho,
};

use crate::eval_source_with_machine_chirho;

/// Evaluate a Haskell snippet through the full Haskelujah pipeline,
/// returning a CompileOutcomeChirho.
fn haskelujah_eval_chirho(source_chirho: &str) -> CompileOutcomeChirho {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    match eval_source_with_machine_chirho(source_chirho, &mut sm_chirho, "Diff.hs", None) {
        Ok((val_chirho, _machine_chirho)) => match val_chirho {
            haskelujah_runtime_chirho::ValueChirho::IntChirho(n_chirho) => {
                CompileOutcomeChirho::SuccessIntChirho(n_chirho)
            }
            other_chirho => {
                CompileOutcomeChirho::SuccessOtherChirho(format!("{:?}", other_chirho))
            }
        },
        Err(e_chirho) => CompileOutcomeChirho::RejectedChirho(format!("{}", e_chirho)),
    }
}

// ── Individual category tests ──────────────────────────────────────────

#[test]
fn diff_syntax_cases_chirho() {
    let cases_chirho = diff_test_cases_chirho();
    let syntax_chirho: Vec<_> = cases_chirho
        .iter()
        .filter(|c_chirho| c_chirho.category_chirho == "syntax")
        .collect();
    assert!(!syntax_chirho.is_empty());

    for case_chirho in &syntax_chirho {
        let result_chirho = haskelujah_eval_chirho(case_chirho.source_chirho);
        if result_chirho != case_chirho.ghc_expected_chirho {
            eprintln!(
                "DIFF MISMATCH [syntax/{}]: GHC={:?}, Haskelujah={:?}",
                case_chirho.name_chirho, case_chirho.ghc_expected_chirho, result_chirho
            );
        }
    }
}

#[test]
fn diff_types_cases_chirho() {
    let cases_chirho = diff_test_cases_chirho();
    let types_chirho: Vec<_> = cases_chirho
        .iter()
        .filter(|c_chirho| c_chirho.category_chirho == "types")
        .collect();
    assert!(!types_chirho.is_empty());

    for case_chirho in &types_chirho {
        let result_chirho = haskelujah_eval_chirho(case_chirho.source_chirho);
        if result_chirho != case_chirho.ghc_expected_chirho {
            eprintln!(
                "DIFF MISMATCH [types/{}]: GHC={:?}, Haskelujah={:?}",
                case_chirho.name_chirho, case_chirho.ghc_expected_chirho, result_chirho
            );
        }
    }
}

#[test]
fn diff_semantics_cases_chirho() {
    let cases_chirho = diff_test_cases_chirho();
    let semantics_chirho: Vec<_> = cases_chirho
        .iter()
        .filter(|c_chirho| c_chirho.category_chirho == "semantics")
        .collect();
    assert!(!semantics_chirho.is_empty());

    for case_chirho in &semantics_chirho {
        let result_chirho = haskelujah_eval_chirho(case_chirho.source_chirho);
        if result_chirho != case_chirho.ghc_expected_chirho {
            eprintln!(
                "DIFF MISMATCH [semantics/{}]: GHC={:?}, Haskelujah={:?}",
                case_chirho.name_chirho, case_chirho.ghc_expected_chirho, result_chirho
            );
        }
    }
}

#[test]
fn diff_edge_cases_chirho() {
    let cases_chirho = diff_test_cases_chirho();
    let edge_chirho: Vec<_> = cases_chirho
        .iter()
        .filter(|c_chirho| c_chirho.category_chirho == "edge")
        .collect();
    assert!(!edge_chirho.is_empty());

    for case_chirho in &edge_chirho {
        let result_chirho = haskelujah_eval_chirho(case_chirho.source_chirho);
        if result_chirho != case_chirho.ghc_expected_chirho {
            eprintln!(
                "DIFF MISMATCH [edge/{}]: GHC={:?}, Haskelujah={:?}",
                case_chirho.name_chirho, case_chirho.ghc_expected_chirho, result_chirho
            );
        }
    }
}

// ── Full suite ─────────────────────────────────────────────────────────

#[test]
fn diff_full_suite_chirho() {
    let suite_chirho = run_diff_suite_chirho(haskelujah_eval_chirho);
    let total_chirho = suite_chirho.results_chirho.len();
    let agree_chirho = suite_chirho.agree_count_chirho();
    let rate_chirho = suite_chirho.agree_rate_chirho();

    eprintln!("\n{}\n", suite_chirho.summary_table_chirho());

    // Track agreement rate — should be high
    assert!(
        rate_chirho >= 60.0,
        "GHC agreement rate too low: {}/{} ({:.1}%)",
        agree_chirho, total_chirho, rate_chirho
    );
}

#[test]
fn diff_suite_agreement_count_chirho() {
    let suite_chirho = run_diff_suite_chirho(haskelujah_eval_chirho);
    let agree_chirho = suite_chirho.agree_count_chirho();
    // We should agree with GHC on at least 15 of 25 cases
    assert!(
        agree_chirho >= 15,
        "should agree on at least 15 cases, got {}",
        agree_chirho
    );
}
