// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! How native executables fail. Two families of programs live here.
//!
//! - **Shared error helper.** A function and the closures lifted from it share
//!   the module's one `error` helper. The LLVM backend compiled each lifted
//!   closure with a registry of emitted names of its own, so a program that can
//!   fail both in a function and in its `where` closure defined the helper twice
//!   and did not link ("invalid redefinition of function 'haskelujah_error'"),
//!   on every build back to old main (claude2_chirho's D8 and D9, 2026-10-17).
//! - **One failure route (row 535).** Each owned route fails the same way on
//!   both backends: exit 1, no signal, and exactly one stderr line, or a
//!   compile-time refusal naming the primitive. Cranelift's refusal reaches
//!   only primitives it cannot lower, so it lowers `compare` on Int and Char
//!   as LLVM does; a reachable Prelude `Ord` dictionary compiles those bodies.
//!
//! Every expectation was measured on GHC 9.14.1. A definition count alone would
//! prove nothing that runs. Workflow: testing-chirho/native-fail-loud-chirho.md.

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

const BACKENDS_CHIRHO: [NativeBackendChirho; 2] = [
    NativeBackendChirho::LlvmChirho,
    NativeBackendChirho::CraneliftChirho,
];
const LLVM_CHIRHO: NativeBackendChirho = NativeBackendChirho::LlvmChirho;
const CRANELIFT_CHIRHO: NativeBackendChirho = NativeBackendChirho::CraneliftChirho;

/// Fails in the function or in its capturing `where` closure.
const LIMIT_CHIRHO: &str = "module Main where\nlimitChirho :: Int -> Int -> Int\nlimitChirho lim n = if n < 0 then error \"outer-chirho\" else go n\n  where go k = if k > lim then error \"inner-chirho\" else k\n";

/// Two sibling closures and one nested in the second, each able to fail.
const CLASSIFY_CHIRHO: &str = "module Main where\nclassifyChirho :: Int -> Int -> Int\nclassifyChirho lim n\n  | n < 0 = left n\n  | otherwise = right n\n  where\n    left m\n      | m < negate lim = error \"left-chirho\"\n      | otherwise = negate m\n    right m\n      | m > lim = deeper m\n      | otherwise = m\n      where\n        deeper e\n          | e > 2 * lim = error \"deeper-chirho\"\n          | otherwise = e - lim\n";

/// R1, U01: an unbound name demanded under `main`'s parameter. GHC: `[1,3,6]`.
const UNBOUND_CALL_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (scanl1 (+) [1, 2, 3 :: Int])\n";

/// R1, U05: the same name in a parameterless constant, which has nothing to alias.
const UNBOUND_CONSTANT_CHIRHO: &str = "module Main where\nxsChirho :: [Int]\nxsChirho = scanl1 (+) [1, 2, 3]\nmain :: IO ()\nmain = print xsChirho\n";

/// R1: an unbound name on a path the program never takes. GHC: `[7]`.
const UNBOUND_UNTAKEN_CHIRHO: &str = "module Main where\npickChirho :: Bool -> [Int]\npickChirho useLiteralChirho = if useLiteralChirho then [7] else scanl1 (+) [1, 2, 3]\nmain :: IO ()\nmain = print (pickChirho True)\n";

/// R2, C01: GHC prints `Just custom`; the native integer formatter got a pointer.
const BOXED_SHOW_CHIRHO: &str = "module Main where\ndata TChirho = TChirho\ninstance Show TChirho where\n  show _ = \"custom\"\nmain :: IO ()\nmain = print (Just TChirho)\n";

/// R3, E01.
const ERROR_CALL_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = putStrLn (error \"boom\")\n";

/// R3, E03.
const UNDEFINED_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (undefined :: Int)\n";

/// R3, E05: GHC writes the payload with no call stack.
const ERROR_WITHOUT_STACK_TRACE_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = putStrLn (errorWithoutStackTrace \"plain\")\n";

/// R3, M01: the dictionary pass's missing-method stub. GHC: `Just [2,3]`.
const MISSING_METHOD_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (mapM (\\x -> Just (x + 1)) [1, 2 :: Int])\n";

/// R5, A01: no compiled alternative matches. GHC: `(6,12)`.
const NO_MATCH_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (gcd 12 (18 :: Int), lcm 4 (6 :: Int))\n";

/// R4, P01: IORef has no native implementation. GHC: `2`.
const IOREF_CHIRHO: &str = "module Main where\nimport Data.IORef\nmain :: IO ()\nmain = do { r <- newIORef (1 :: Int); modifyIORef r (+ 1); v <- readIORef r; print v }\n";

/// `compare` on Int and Char. GHC prints `GT` twice; before row 535, Cranelift
/// printed `LT` twice (the first two lines of Compare2's measured 15e1b77b run).
const INT_CHAR_COMPARE_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (compare (2 :: Int) 1)\n  print (compare 'b' 'a')\n";

/// R4, P06: GHC prints `1024`; both native backends now share the RTS's `^#`.
const POWER_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (2 ^ (10 :: Int) :: Int)\n";

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

/// The runtime's program route: exit 1 (the harness reports a signal another
/// way) and exactly one stderr line, after nothing on stdout.
fn assert_fails_named_chirho(
    backend_chirho: NativeBackendChirho,
    source_chirho: &str,
    message_chirho: &str,
) {
    assert_eq!(
        native_round_trip_chirho(source_chirho, backend_chirho, "")
            .expect_err("GHC fails here, or the route names a missing native operation"),
        format!(
            "native program emitted stderr (exit 1): haskelujah: error: {message_chirho}\n\nstdout:\n"
        ),
        "{backend_chirho:?}"
    );
}

/// A compile-time refusal naming each unsupported primitive, as `name/arity`.
fn assert_refused_chirho(
    backend_chirho: NativeBackendChirho,
    source_chirho: &str,
    primitives_chirho: &str,
) {
    let backend_name_chirho = match backend_chirho {
        NativeBackendChirho::LlvmChirho => "LLVM",
        NativeBackendChirho::CraneliftChirho => "Cranelift",
    };
    assert_eq!(
        native_round_trip_chirho(source_chirho, backend_chirho, "")
            .expect_err("an executable that needs the primitive is refused"),
        format!("{backend_name_chirho} does not support primitive(s): {primitives_chirho}"),
        "{backend_chirho:?}"
    );
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
        assert_fails_named_chirho(backend_chirho, source_chirho, message_chirho);
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

// ── R1: an unbound reference fails named, and only when demanded ──────────

#[test]
fn an_unbound_call_fails_named_on_llvm_chirho() {
    assert_fails_named_chirho(
        LLVM_CHIRHO,
        UNBOUND_CALL_CHIRHO,
        "missing native binding `scanl1`",
    );
}

#[test]
fn an_unbound_call_fails_named_on_cranelift_chirho() {
    assert_fails_named_chirho(
        CRANELIFT_CHIRHO,
        UNBOUND_CALL_CHIRHO,
        "missing native binding `scanl1`",
    );
}

#[test]
fn an_unbound_constant_fails_named_on_llvm_chirho() {
    assert_fails_named_chirho(
        LLVM_CHIRHO,
        UNBOUND_CONSTANT_CHIRHO,
        "missing native binding `scanl1`",
    );
}

#[test]
fn an_unbound_constant_fails_named_on_cranelift_chirho() {
    assert_fails_named_chirho(
        CRANELIFT_CHIRHO,
        UNBOUND_CONSTANT_CHIRHO,
        "missing native binding `scanl1`",
    );
}

#[test]
fn an_unbound_name_on_an_untaken_path_runs_everywhere_chirho() {
    assert_prints_everywhere_chirho(UNBOUND_UNTAKEN_CHIRHO, "[7]\n");
}

// ── R2: no integer formatter prints a heap address ─────────────────────────

#[test]
fn a_boxed_value_is_refused_by_integer_formatting_on_llvm_chirho() {
    assert_fails_named_chirho(
        LLVM_CHIRHO,
        BOXED_SHOW_CHIRHO,
        "show: no native Show instance for a boxed value",
    );
}

#[test]
fn a_boxed_value_is_refused_by_integer_formatting_on_cranelift_chirho() {
    assert_fails_named_chirho(
        CRANELIFT_CHIRHO,
        BOXED_SHOW_CHIRHO,
        "show: no native Show instance for a boxed value",
    );
}

// ── R3: one error route ───────────────────────────────────────────────────

#[test]
fn error_reports_its_payload_on_llvm_chirho() {
    assert_fails_named_chirho(LLVM_CHIRHO, ERROR_CALL_CHIRHO, "boom");
}

#[test]
fn error_reports_its_payload_on_cranelift_chirho() {
    assert_fails_named_chirho(CRANELIFT_CHIRHO, ERROR_CALL_CHIRHO, "boom");
}

#[test]
fn undefined_fails_named_on_llvm_chirho() {
    assert_fails_named_chirho(LLVM_CHIRHO, UNDEFINED_CHIRHO, "Prelude.undefined");
}

#[test]
fn undefined_fails_named_on_cranelift_chirho() {
    assert_fails_named_chirho(CRANELIFT_CHIRHO, UNDEFINED_CHIRHO, "Prelude.undefined");
}

#[test]
fn error_without_stack_trace_reports_its_payload_on_llvm_chirho() {
    assert_fails_named_chirho(LLVM_CHIRHO, ERROR_WITHOUT_STACK_TRACE_CHIRHO, "plain");
}

#[test]
fn error_without_stack_trace_reports_its_payload_on_cranelift_chirho() {
    assert_fails_named_chirho(CRANELIFT_CHIRHO, ERROR_WITHOUT_STACK_TRACE_CHIRHO, "plain");
}

#[test]
fn a_missing_method_fails_named_on_llvm_chirho() {
    assert_fails_named_chirho(
        LLVM_CHIRHO,
        MISSING_METHOD_CHIRHO,
        "missing method Traversable.mapM for []",
    );
}

#[test]
fn a_missing_method_fails_named_on_cranelift_chirho() {
    assert_fails_named_chirho(
        CRANELIFT_CHIRHO,
        MISSING_METHOD_CHIRHO,
        "missing method Traversable.mapM for []",
    );
}

// ── R4: an unknown primitive is refused at compile time ────────────────────

#[test]
fn an_unknown_primitive_is_refused_on_llvm_chirho() {
    assert_refused_chirho(
        LLVM_CHIRHO,
        IOREF_CHIRHO,
        "newIORef#/1, readIORef#/1, writeIORef#/2",
    );
}

#[test]
fn an_unknown_primitive_is_refused_on_cranelift_chirho() {
    assert_refused_chirho(
        CRANELIFT_CHIRHO,
        IOREF_CHIRHO,
        "newIORef#/1, readIORef#/1, writeIORef#/2",
    );
}

#[test]
fn integer_power_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(POWER_CHIRHO, "1024\n");
}

#[test]
fn int_and_char_compare_answer_as_ghc_on_cranelift_chirho() {
    assert_eq!(
        native_round_trip_chirho(INT_CHAR_COMPARE_CHIRHO, CRANELIFT_CHIRHO, "")
            .unwrap_or_else(|error_chirho| panic!("Cranelift: {error_chirho}")),
        (0, "GT\nGT\n".to_string())
    );
}

// ── R5: no matching alternative fails named ───────────────────────────────

#[test]
fn no_matching_alternative_fails_named_on_llvm_chirho() {
    assert_fails_named_chirho(LLVM_CHIRHO, NO_MATCH_CHIRHO, "no matching alternative");
}

#[test]
fn no_matching_alternative_fails_named_on_cranelift_chirho() {
    assert_fails_named_chirho(CRANELIFT_CHIRHO, NO_MATCH_CHIRHO, "no matching alternative");
}
