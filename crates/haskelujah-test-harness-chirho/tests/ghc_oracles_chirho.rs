// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Prove that the actual curated header spellings constrain observed output.

use haskelujah_test_harness::ghc_suite_chirho::{discover_ghc_tests_chirho, run_ghc_test_chirho};

fn header_result_chirho(header_chirho: &str, body_chirho: &str) -> bool {
    let directory_chirho = tempfile::tempdir().unwrap();
    std::fs::write(
        directory_chirho.path().join("OracleChirho.hs"),
        format!("-- TEST: compile_and_run\n{header_chirho}\nmodule Main where\n{body_chirho}\n"),
    )
    .unwrap();
    let tests_chirho = discover_ghc_tests_chirho(directory_chirho.path()).unwrap();
    assert_eq!(tests_chirho.len(), 1);
    run_ghc_test_chirho(&tests_chirho[0]).passed_chirho
}

#[test]
fn both_curated_oracle_spellings_reject_wrong_answers_chirho() {
    for spelling_chirho in ["EXPECTED", "EXPECT_OUTPUT"] {
        assert!(
            !header_result_chirho(&format!("-- {spelling_chirho}: 999"), "main = print 42"),
            "{spelling_chirho} was ignored: 42 is not 999"
        );
        assert!(header_result_chirho(
            &format!("-- {spelling_chirho}: 42"),
            "main = print 42"
        ));
    }
}

#[test]
fn absent_output_is_not_an_execution_oracle_chirho() {
    assert!(!header_result_chirho("-- no oracle", "main = print 42"));
}

#[test]
fn escaped_multiline_and_empty_output_have_real_oracles_chirho() {
    assert!(header_result_chirho(
        "-- EXPECTED: 3\\n2\\n1",
        "main = do { print 3; print 2; print 1 }"
    ));
    assert!(header_result_chirho(
        "-- EXPECT_OUTPUT:",
        "main = return ()"
    ));
    assert!(!header_result_chirho(
        "-- EXPECT_OUTPUT:",
        "main = print 42"
    ));
}

#[test]
fn significant_spaces_and_literal_escapes_are_not_erased_chirho() {
    assert!(!header_result_chirho(
        "-- EXPECTED: 42",
        "main = putStrLn \" 42\""
    ));
    assert!(!header_result_chirho(
        "-- EXPECTED: 42",
        "main = putStrLn \"42 \""
    ));
    assert!(header_result_chirho(
        r"-- EXPECTED: \\n",
        r#"main = putStrLn "\\n""#
    ));
}
