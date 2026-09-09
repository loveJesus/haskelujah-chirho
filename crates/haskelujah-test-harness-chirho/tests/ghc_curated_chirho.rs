// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Integration test that runs the curated GHC test suite from
//! `ghc-tests-chirho/` through the Haskelujah pipeline.

use haskelujah_test_harness::ghc_suite_chirho::{
    GhcTestKindChirho, discover_ghc_tests_chirho, run_ghc_suite_chirho,
};
use std::path::Path;

/// Run all curated GHC tests and require every declared behavior.
#[test]
fn ghc_curated_suite_chirho() {
    // Find the ghc-tests-chirho directory relative to the workspace root.
    let suite_dir_chirho = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho");

    assert!(
        suite_dir_chirho.is_dir(),
        "GHC curated fixtures missing: {}",
        suite_dir_chirho.display()
    );

    let tests_chirho =
        discover_ghc_tests_chirho(&suite_dir_chirho).expect("should discover test files");

    assert!(
        !tests_chirho.is_empty(),
        "should find at least one test in {}",
        suite_dir_chirho.display()
    );

    // Independently inventory declared run directives: discovery must not silently
    // turn one into a compile-only test because of its prologue's layout.
    for test_chirho in &tests_chirho {
        if test_chirho
            .source_chirho
            .lines()
            .any(|line_chirho| line_chirho.trim() == "-- TEST: compile_and_run")
        {
            assert_eq!(
                test_chirho.kind_chirho,
                GhcTestKindChirho::CompileAndRunChirho,
                "execution directive was lost: {}",
                test_chirho.name_chirho
            );
            assert!(
                test_chirho.expected_output_chirho.is_some(),
                "missing oracle: {}",
                test_chirho.name_chirho
            );
        }
    }
    let execution_count_chirho = tests_chirho
        .iter()
        .filter(|test_chirho| test_chirho.kind_chirho == GhcTestKindChirho::CompileAndRunChirho)
        .count();
    eprintln!(
        "Running {} curated GHC tests ({} execution oracles)...",
        tests_chirho.len(),
        execution_count_chirho
    );

    let suite_chirho = run_ghc_suite_chirho(&tests_chirho);

    // Print results
    for result_chirho in &suite_chirho.results_chirho {
        let status_chirho = if result_chirho.passed_chirho {
            "PASS"
        } else {
            "FAIL"
        };
        eprintln!(
            "  [{}] {}: {}",
            status_chirho, result_chirho.name_chirho, result_chirho.message_chirho
        );
    }

    eprintln!(
        "\nGHC curated suite: {}/{} passed ({:.1}%)",
        suite_chirho.passed_chirho,
        suite_chirho.total_chirho,
        suite_chirho.pass_rate_chirho(),
    );

    // Assert all tests pass (since we curated them to match our capabilities).
    assert_eq!(
        suite_chirho.failed_chirho,
        0,
        "All curated GHC tests should pass. Failures:\n{}",
        suite_chirho
            .results_chirho
            .iter()
            .filter(|r_chirho| !r_chirho.passed_chirho)
            .map(|r_chirho| format!("  {}: {}", r_chirho.name_chirho, r_chirho.message_chirho))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
