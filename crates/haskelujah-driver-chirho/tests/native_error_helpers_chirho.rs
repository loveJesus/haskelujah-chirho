// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A function and the closures lifted from it share the module's one `error`
//! helper. The LLVM backend compiled each lifted closure with a registry of
//! emitted names of its own, so a program that can fail both in a function and
//! in its `where` closure defined the helper twice and did not link ("invalid
//! redefinition of function 'haskelujah_error'"), on every build back to old
//! main (claude2_chirho's D8 and D9, 2026-10-17). Every expectation was measured
//! on GHC 9.14.1, and each program must give it in the interpreter and on both
//! native backends: a definition count alone would prove nothing that runs.

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

const BACKENDS_CHIRHO: [NativeBackendChirho; 2] = [
    NativeBackendChirho::LlvmChirho,
    NativeBackendChirho::CraneliftChirho,
];

/// Fails in the function or in its capturing `where` closure.
const LIMIT_CHIRHO: &str = "module Main where\nlimitChirho :: Int -> Int -> Int\nlimitChirho lim n = if n < 0 then error \"outer-chirho\" else go n\n  where go k = if k > lim then error \"inner-chirho\" else k\n";

/// Two sibling closures and one nested in the second, each able to fail.
const CLASSIFY_CHIRHO: &str = "module Main where\nclassifyChirho :: Int -> Int -> Int\nclassifyChirho lim n\n  | n < 0 = left n\n  | otherwise = right n\n  where\n    left m\n      | m < negate lim = error \"left-chirho\"\n      | otherwise = negate m\n    right m\n      | m > lim = deeper m\n      | otherwise = m\n      where\n        deeper e\n          | e > 2 * lim = error \"deeper-chirho\"\n          | otherwise = e - lim\n";

fn with_main_chirho(module_chirho: &str, main_chirho: &str) -> String {
    format!("{module_chirho}main :: IO ()\nmain = {main_chirho}\n")
}

fn assert_prints_everywhere_chirho(source_chirho: &str, expected_chirho: &str) {
    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ErrorHelperChirho.hs",
        None,
    )
    .expect("the interpreter runs it");
    assert_eq!(
        machine_chirho.io_output_chirho, expected_chirho,
        "interpreter"
    );
    for backend_chirho in BACKENDS_CHIRHO {
        assert_eq!(
            native_round_trip_chirho(source_chirho, backend_chirho, "")
                .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, expected_chirho.to_string()),
            "{backend_chirho:?}"
        );
    }
}

fn assert_fails_everywhere_chirho(source_chirho: &str, message_chirho: &str) {
    let error_chirho = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ErrorHelperChirho.hs",
        None,
    )
    .expect_err("GHC fails here");
    assert!(
        error_chirho.contains(message_chirho),
        "interpreter: {error_chirho}"
    );
    for backend_chirho in BACKENDS_CHIRHO {
        let error_chirho = native_round_trip_chirho(source_chirho, backend_chirho, "")
            .expect_err("GHC fails here");
        // The harness reports any stderr as an error, even after exit 0, so
        // a program must also have FAILED: killed by a signal, or a nonzero
        // exit.
        let failed_chirho = error_chirho.starts_with("native program terminated by signal: ")
            || (error_chirho.starts_with("native program emitted stderr (exit ")
                && !error_chirho.starts_with("native program emitted stderr (exit 0)"));
        assert!(
            failed_chirho && error_chirho.contains(message_chirho),
            "{backend_chirho:?}: {error_chirho}"
        );
    }
}

#[test]
fn a_closure_and_its_function_share_one_error_helper_chirho() {
    // D9: an explicit `error` in the function and in its closure.
    assert_prints_everywhere_chirho(
        "module Main where\nfChirho :: Int -> Int\nfChirho n = if n < 0 then error \"neg\" else go n\n  where go k = if k > 100 then error \"big\" else k\nmain :: IO ()\nmain = print (fChirho 5)\n",
        "5\n",
    );
    // D8: guards that can fall off the end, in the function and its closure.
    assert_prints_everywhere_chirho(
        "module Main where\nisPosChirho :: Int -> Bool\nisPosChirho n\n  | n > 0 = True\n  | n <= 0 = go n\n  where\n    go d\n      | d < 0 = False\n      | d == 0 = False\nmain :: IO ()\nmain = putStrLn (if isPosChirho 5 then \"pos\" else \"nonpos\")\n",
        "pos\n",
    );
    // E1: the closure captures the function's parameter.
    assert_prints_everywhere_chirho(
        &with_main_chirho(LIMIT_CHIRHO, "print (limitChirho 10 5)"),
        "5\n",
    );
}

#[test]
fn sibling_and_nested_closures_share_one_error_helper_chirho() {
    // E4: `left` and `right` are siblings, `deeper` is nested in `right`.
    assert_prints_everywhere_chirho(
        &with_main_chirho(
            CLASSIFY_CHIRHO,
            "print (map (classifyChirho 10) [-5, 5, 15])",
        ),
        "[5,5,5]\n",
    );
}

#[test]
fn each_error_path_fails_with_its_own_message_chirho() {
    // E2 the function's own error, E3 the capturing closure's, E5 a sibling
    // closure's, E6 a nested closure's.
    assert_fails_everywhere_chirho(
        &with_main_chirho(LIMIT_CHIRHO, "print (limitChirho 10 (-1))"),
        "outer-chirho",
    );
    assert_fails_everywhere_chirho(
        &with_main_chirho(LIMIT_CHIRHO, "print (limitChirho 10 50)"),
        "inner-chirho",
    );
    assert_fails_everywhere_chirho(
        &with_main_chirho(CLASSIFY_CHIRHO, "print (classifyChirho 10 (-50))"),
        "left-chirho",
    );
    assert_fails_everywhere_chirho(
        &with_main_chirho(CLASSIFY_CHIRHO, "print (classifyChirho 10 50)"),
        "deeper-chirho",
    );
}
