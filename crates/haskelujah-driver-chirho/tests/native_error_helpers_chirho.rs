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
//!   compile-time refusal naming the primitive. A refusal reaches only
//!   primitives a backend cannot lower, so both lower `compare` on Int, Char,
//!   Double and String (rows 535 and 539); a reachable Prelude `Ord`
//!   dictionary compiles those bodies.
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

/// R1, U04: an unbound name demanded under `main`'s parameter. GHC: `1.5`. (`scanl1`, the
/// first fixture, is bound since row 548; `realToFrac` still needs its dictionaries.)
const UNBOUND_CALL_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (realToFrac (1.5 :: Double) :: Double)\n";

/// R1: the same name in a parameterless constant, which has nothing to alias. GHC: `1.5`.
const UNBOUND_CONSTANT_CHIRHO: &str = "module Main where\nxsChirho :: Double\nxsChirho = realToFrac (1.5 :: Double)\nmain :: IO ()\nmain = print xsChirho\n";

/// R1: an unbound name on a path the program never takes. GHC: `7.5`.
const UNBOUND_UNTAKEN_CHIRHO: &str = "module Main where\npickChirho :: Bool -> Double\npickChirho useLiteralChirho = if useLiteralChirho then 7.5 else realToFrac (1.5 :: Double)\nmain :: IO ()\nmain = print (pickChirho True)\n";

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

/// R5, A01: GHC prints `(6,12)`. Row 564 gave `gcd` and `lcm` their Integral
/// schemes, so the tuple's Show evidence keeps both payload types.
const NO_MATCH_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (gcd 12 (18 :: Int), lcm 4 (6 :: Int))\n";

/// R4, P01: native IORef allocation, mutation, and reading. GHC: `2`.
const IOREF_CHIRHO: &str = "module Main where\nimport Data.IORef\nmain :: IO ()\nmain = do { r <- newIORef (1 :: Int); modifyIORef r (+ 1); v <- readIORef r; print v }\n";

/// Compare2: `compare` on Int, Char, Double and String. GHC prints
/// `GT GT LT GT`. Before row 535 Cranelift printed `LT` for the Int and Char
/// lines; before row 539 both backends refused `compareFloat#` and `compareStr#`.
const COMPARE2_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (compare (2 :: Int) 1)\n  print (compare 'b' 'a')\n  print (compare (1.5 :: Double) 2.5)\n  print (compare \"b\" \"a\")\n";

/// Row 539's edges. NaN on either side gives `GT`, and `-0.0` equals `0.0`.
/// Strings order by code point: the empty string, a prefix, a non-ASCII
/// character and a computed `show`.
const COMPARE_EDGES_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (compare (0.0 / 0.0 :: Double) 1.0)\n  print (compare (1.0 :: Double) (0.0 / 0.0))\n  print (compare (0.0 / 0.0 :: Double) (0.0 / 0.0))\n  print (compare (-0.0 :: Double) 0.0)\n  print (compare (2.5 :: Double) 2.5)\n  print (compare (3.0 :: Double) (-1.0))\n  print (compare \"abc\" \"abd\")\n  print (compare \"ab\" \"abc\")\n  print (compare \"abc\" \"abc\")\n  print (compare \"\" \"a\")\n  print (compare \"a\" \"\")\n  print (compare \"\\233\" \"z\")\n  print (compare (show (12 :: Int)) \"9\")\n";

/// GHC 9.14.1's output for [`COMPARE_EDGES_CHIRHO`].
const COMPARE_EDGES_GHC_CHIRHO: &str = "GT\nGT\nGT\nEQ\nEQ\nGT\nLT\nLT\nEQ\nLT\nGT\nGT\nLT\n";

/// R4, P06: nonnegative Int powers keep GHC's wrapping-machine-Int convention.
const POWER_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (2 ^ (0 :: Int) :: Int)\n  print (2 ^ (10 :: Int) :: Int)\n  print ((-3) ^ (3 :: Int) :: Int)\n";

/// Row 574: each form reaches GHC's named negative-exponent boundary before it can return `0`.
const NEGATIVE_POWER_CASES_CHIRHO: [&str; 5] = [
    "module Main where\nmain :: IO ()\nmain = print (2 ^ (-1 :: Int) :: Int)\n",
    "module Main where\nmain :: IO ()\nmain = print ((-2) ^ (-3 :: Int) :: Int)\n",
    "module Main where\nexponentChirho :: Int\nexponentChirho = -1\nmain :: IO ()\nmain = print (3 ^ exponentChirho :: Int)\n",
    "module Main where\nmain :: IO ()\nmain = print (map (2 ^) [-1, 0, 3 :: Int] :: [Int])\n",
    "module Main where\nmain :: IO ()\nmain = print (2 ^ (-1 :: Integer) :: Int)\n",
];

/// R4, P03: GHC and the interpreter print `3.5`.
const FROM_INTEGRAL_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (fromIntegral (7 :: Int) / (2 :: Double))\n";

/// Saved oracle P02: native `map` produces a boxed `[Char]`, while literals
/// and `show` results use native C strings. GHC prints `ABC`.
const CHAR_CASE_MAPPING_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = putStrLn (map toUpper \"abc\")\n";

/// Saved oracle P05: empty/singleton `fromList` needs no key comparison.
const MAP_SINGLETON_SIZE_CHIRHO: &str = "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.size (M.fromList [(1 :: Int, \"a\")]))\n";

/// P04 remains refused: two pairs require ordering and `toList` is not lowered.
const MAP_TO_LIST_BOUNDARY_CHIRHO: &str = "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.toList (M.fromList [(2 :: Int, \"b\"), (1, \"a\")]))\n";

/// Two-pair `fromList` reaches the explicit comparator-evidence boundary.
const MAP_ORDERING_BOUNDARY_CHIRHO: &str = "module Main where\nimport qualified Data.Map as M\nmain :: IO ()\nmain = print (M.size (M.fromList [(2 :: Int, \"b\"), (1, \"a\")]))\n";

fn with_main_chirho(module_chirho: &str, main_chirho: &str) -> String {
    format!("{module_chirho}main :: IO ()\nmain = {main_chirho}\n")
}

fn assert_prints_everywhere_chirho(source_chirho: &str, expected_chirho: &str) {
    assert_interprets_chirho(source_chirho, expected_chirho);
    assert_prints_natively_chirho(source_chirho, expected_chirho);
}

fn assert_interprets_chirho(source_chirho: &str, expected_chirho: &str) {
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
}

fn assert_prints_natively_chirho(source_chirho: &str, expected_chirho: &str) {
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

// ── Row 548: the report's scanl1, foldl1 and until (contract-543 U01-U03, U05) ──

/// GHC 9.14.1: `[1,3,6]`, `[10,7,5]`, `[]`, `9`, `5`, `7`, `128`, `5`. `foldl1 (-)` and
/// `scanl1 (-)` pin left association; `until` covers an immediately true predicate.
const REPORT_FOLDS_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (scanl1 (+) [1, 2, 3 :: Int])\n  print (scanl1 (-) [10, 3, 2 :: Int])\n  print (scanl1 (+) ([] :: [Int]))\n  print (foldl1 max [3, 9, 2 :: Int])\n  print (foldl1 (-) [10, 3, 2 :: Int])\n  print (foldl1 (-) [7 :: Int])\n  print (until (> 100) (* 2) (1 :: Int))\n  print (until (> 0) (+ 1) (5 :: Int))\n";

/// U05: `scanl1` in a parameterless constant. GHC: `[1,3,6]`.
const SCANL1_CONSTANT_CHIRHO: &str = "module Main where\nxsChirho :: [Int]\nxsChirho = scanl1 (+) [1, 2, 3]\nmain :: IO ()\nmain = print xsChirho\n";

/// GHC exits 1, its first stderr line `Prelude.foldl1: empty list`.
const FOLDL1_EMPTY_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (foldl1 max ([] :: [Int]))\n";

#[test]
fn report_folds_answer_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        REPORT_FOLDS_CHIRHO,
        "[1,3,6]\n[10,7,5]\n[]\n9\n5\n7\n128\n5\n",
    );
}

#[test]
fn a_scanl1_constant_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(SCANL1_CONSTANT_CHIRHO, "[1,3,6]\n");
}

#[test]
fn an_empty_foldl1_fails_named_everywhere_chirho() {
    assert_fails_everywhere_chirho(FOLDL1_EMPTY_CHIRHO, "Prelude.foldl1: empty list");
}

// ── Row 558: foldr1, and the report folds' own types ──────────────────────

/// GHC 9.14.1: `9`, `7`, `8`, `[3,5]`, `3`, `5`, `9.0`, `5.0`, `192.0`. `foldr1` had no binding
/// (a named missing binding everywhere). `foldl1`, `foldr1` and `until` had no type scheme, so
/// over Double their step and `show` defaulted to Int: the natives printed a Double's bits as an
/// integer, and the interpreter failed on `SubIntChirho` or printed `192`.
const REPORT_FOLDS_TYPED_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (foldr1 (-) [10, 3, 2 :: Int])\n  print (foldr1 (-) [7 :: Int])\n  print (sum (map (foldr1 max) [[3, 1, 2], [5 :: Int]]))\n  print (map (foldr1 max) [[3, 1, 2], [5 :: Int]])\n  mapM_ (print . foldr1 max) [[3, 1, 2], [5 :: Int]]\n  print (foldr1 (-) ([10, 3, 2] :: [Double]))\n  print (foldl1 (-) ([10, 3, 2] :: [Double]))\n  print (until (> 100) (* 2) (1.5 :: Double))\n";

/// GHC exits 1, its error payload `Prelude.foldr1: empty list`.
const FOLDR1_EMPTY_CHIRHO: &str =
    "module Main where\nmain :: IO ()\nmain = print (foldr1 max ([] :: [Int]))\n";

#[test]
fn report_folds_answer_as_ghc_at_every_type_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        REPORT_FOLDS_TYPED_CHIRHO,
        "9\n7\n8\n[3,5]\n3\n5\n9.0\n5.0\n192.0\n",
    );
}

#[test]
fn an_empty_foldr1_fails_named_everywhere_chirho() {
    assert_fails_everywhere_chirho(FOLDR1_EMPTY_CHIRHO, "Prelude.foldr1: empty list");
}

// ── Row 550: a first-class print keeps its Show proof; GHC's Double spelling ──

/// GHC 9.14.1: each element on its own line, exactly as `show` spells it. Natively, the
/// generic Int-only `print` used to print a Double's bits, a Bool as 1 or 0, a Char as its
/// code point and a String or a derived value as a heap word.
const FIRST_CLASS_PRINT_CHIRHO: &str = "module Main where\ndata ColourChirho = RedChirho | BlueChirho deriving Show\nmain :: IO ()\nmain = do\n  mapM_ print ([7.5, -2.5] :: [Double])\n  mapM_ print [True, False]\n  mapM_ print [\"a\", \"bc\"]\n  mapM_ print ['x', 'y']\n  mapM_ print [1, 2 :: Int]\n  mapM_ print [RedChirho, BlueChirho]\n  sequence_ (map print ([0.5] :: [Double]))\n";

/// GHC 9.14.1's spellings: fixed notation for exponents -1 through 6, scientific outside,
/// signed zero, the subnormal minimum, the maximum, Infinity and NaN.
const DOUBLE_SPELLINGS_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = mapM_ print ([7.0, 0.1, 1.0e-2, 1.0e7, 9999999.0, 12345678.9, 1 / 3, -0.0, 1.0e-3, 5.0e-324, 1.7976931348623157e308, 1 / 0, 0 / 0, -2.5] :: [Double])\n";

#[test]
fn a_first_class_print_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        FIRST_CLASS_PRINT_CHIRHO,
        "7.5\n-2.5\nTrue\nFalse\n\"a\"\n\"bc\"\n'x'\n'y'\n1\n2\nRedChirho\nBlueChirho\n0.5\n",
    );
}

#[test]
fn doubles_are_spelled_as_ghc_spells_them_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        DOUBLE_SPELLINGS_CHIRHO,
        "7.0\n0.1\n1.0e-2\n1.0e7\n9999999.0\n1.23456789e7\n0.3333333333333333\n-0.0\n1.0e-3\n5.0e-324\n1.7976931348623157e308\nInfinity\nNaN\n-2.5\n",
    );
}

#[test]
fn negative_infinity_is_spelled_as_ghc_spells_it_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "module Main where\nmain :: IO ()\nmain = print (negate (1 / 0) :: Double)\n",
        "-Infinity\n",
    );
}

// ── Row 552: sum and product take their element type's own Num rows ──────

/// GHC 9.14.1: `3.75`, `-0.5`, `3.0`, `0.0`, `1.0`, `[1.5,2.25]`, `[3.0,1.0]`, `6`, `24`,
/// `3.75`, `3.0`. The Int-shaped generic bindings added a Double's bits as integers
/// natively (NaN, 0.0) and failed on `AddIntChirho` in the interpreter. Int sums and the
/// explicit folds are the unchanged boundaries.
const AGGREGATES_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = do\n  print (sum ([1.5, 2.25] :: [Double]))\n  print (sum ([0.5, -1.0] :: [Double]))\n  print (product ([1.5, 2.0] :: [Double]))\n  print (sum ([] :: [Double]))\n  print (product ([] :: [Double]))\n  print (map sum ([[1.5], [2.0, 0.25]] :: [[Double]]))\n  print (map product ([[1.5, 2.0], []] :: [[Double]]))\n  print (sum [1, 2, 3 :: Int])\n  print (product [2, 3, 4 :: Int])\n  print (foldr (+) 0 ([1.5, 2.25] :: [Double]))\n  print (foldl (*) 1 ([1.5, 2.0] :: [Double]))\n";

#[test]
fn sums_and_products_answer_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        AGGREGATES_CHIRHO,
        "3.75\n-0.5\n3.0\n0.0\n1.0\n[1.5,2.25]\n[3.0,1.0]\n6\n24\n3.75\n3.0\n",
    );
}

// ── Row 586: sum and product at the binding's own Num key ───────────────

/// Under `Num a`, `sum` and `product` carried the binding's own key (`$own:k`) and still
/// took the Int-shaped generic bindings. At Double the interpreter failed on
/// `AddIntChirho`, and both natives added the Doubles' bits as integers: NaN, 0.0,
/// -1.3671875000000001e-3. They now fold over the binding's own dictionary. The program
/// covers the fold order (`0.6000000000000001`), empty lists, Int instantiations, a
/// point-free `product`, a first-class use under `map`, and two predicates of one class.
const OWN_AGGREGATES_CHIRHO: &str = "module Main where\ntotalChirho :: Num a => [a] -> a\ntotalChirho xs = sum xs\nprodChirho :: Num a => [a] -> a\nprodChirho = product\nbothChirho :: (Num a, Num b) => [a] -> [b] -> (a, b)\nbothChirho xs ys = (sum xs, product ys)\nmain :: IO ()\nmain = do\n  print (totalChirho [0.1, 0.2, 0.3 :: Double], totalChirho [1, 2, 3 :: Int])\n  print (totalChirho ([] :: [Double]), totalChirho [-1.5, 1.5 :: Double])\n  print (prodChirho [1.5, 2, 4 :: Double], prodChirho [2, 3, 7 :: Int])\n  print (prodChirho ([] :: [Double]), map totalChirho [[2.5], [1.5, 0.25 :: Double]])\n  print (bothChirho [1.5, 2.5 :: Double] [2, 3 :: Int], bothChirho [4, 5 :: Int] [0.5, 3 :: Double])\n";

#[test]
fn sums_and_products_at_a_bindings_own_key_answer_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        OWN_AGGREGATES_CHIRHO,
        "(0.6000000000000001,6)\n(0.0,0.0)\n(12.0,42)\n(1.0,[2.5,1.75])\n((4.0,6),(9,1.5))\n",
    );
}

/// Row 586: under `Fractional a` the aggregates fold over the Num inside `$dFractional`.
/// The interpreter prints GHC's `(2.3333333333333335,9.0)`, where it failed on
/// `AddIntChirho`. The natives refused the Fractional Double dictionary's `recip#` until
/// row 583.
const FRACTIONAL_AGGREGATES_CHIRHO: &str = "module Main where\nmeanChirho :: Fractional a => [a] -> a\nmeanChirho xs = sum xs / fromIntegral (length xs)\nscaleSumChirho :: Fractional a => [a] -> a\nscaleSumChirho xs = sum xs * product xs\nmain :: IO ()\nmain = print (meanChirho [1, 2, 4 :: Double], scaleSumChirho [0.5, 4 :: Double])\n";

#[test]
fn a_fractional_bindings_aggregates_take_its_dictionary_chirho() {
    assert_prints_everywhere_chirho(FRACTIONAL_AGGREGATES_CHIRHO, "(2.3333333333333335,9.0)\n");
}

// ── Row 593: a program's own sum and product keep their bodies ──────────

/// A local `sum` or `product` named like the Prelude's. The checker recorded the Prelude
/// function's `Num` evidence on the occurrence, though its canonical id is the program's
/// own `where` binder, and row 552's route replaced the call with the Prelude fold. Every
/// engine printed `(6.5,9,9)`, where GHC prints `(6.5,99,98)`. The Prelude's `sum` and
/// `product` beside them hold, and so do local `maximum` and `minimum`.
const OWN_NAMED_AGGREGATES_CHIRHO: &str = "module Main where\ntotalChirho :: [Double] -> Double\ntotalChirho xs = sum xs + product xs\nmain :: IO ()\nmain = do\n  print (totalChirho [1.5, 2.0], sum [9 :: Int], product [9 :: Int])\n  print (maximum [1, 5 :: Int], minimum [4, 2 :: Int])\n  where\n    sum _ = (99 :: Int)\n    product _ = (98 :: Int)\n    maximum _ = (97 :: Int)\n    minimum _ = (96 :: Int)\n";

#[test]
fn a_programs_own_sum_and_product_keep_their_bodies_everywhere_chirho() {
    assert_prints_everywhere_chirho(OWN_NAMED_AGGREGATES_CHIRHO, "(6.5,99,98)\n(97,96)\n");
}

/// Row 593: inside a `Num a` binding, a local `sum` whose own type carries `Num` took
/// row 586's own-key fold instead of its body. Every engine printed `(4.0,3)`, where GHC
/// prints `(3.0,2)`.
const OWN_KEY_LOCAL_SUM_CHIRHO: &str = "module Main where\ntotalChirho :: Num a => [a] -> a\ntotalChirho xs = sum xs\n  where\n    sum ys = head ys + head ys\nmain :: IO ()\nmain = print (totalChirho [1.5, 2.5 :: Double], totalChirho [1, 2 :: Int])\n";

#[test]
fn a_local_sum_in_a_polymorphic_binding_keeps_its_body_everywhere_chirho() {
    assert_prints_everywhere_chirho(OWN_KEY_LOCAL_SUM_CHIRHO, "(3.0,2)\n");
}

// ── Row 594: a generated Prelude binding never takes a local binder's id ──

/// Local `even`, `head`, `length`, `not`, `reverse` and `maximum` beside the Prelude's.
/// A generated Prelude binding took the first id named like it in the name table's hash
/// order, and that could be the local binder's. The Prelude function the program uses was
/// then left unbound, in about half the runs for each name, on every engine. Each pass
/// hashes its name table afresh, so the interpreter runs it repeatedly.
const LOCAL_PRELUDE_NAMES_CHIRHO: &str = "module Main where\npreludeChirho :: [Int] -> (Bool, Int, Int)\npreludeChirho xs = (even (head xs), head xs, length xs)\npreludeMoreChirho :: [Int] -> (Bool, [Int], Int)\npreludeMoreChirho xs = (not (null xs), reverse xs, maximum xs)\nmain :: IO ()\nmain = do\n  print (preludeChirho [4, 1, 7])\n  print (preludeMoreChirho [4, 1, 7])\n  print (even (3 :: Int), head [9 :: Int], length [9 :: Int])\n  print (not False, reverse [9 :: Int], maximum [9 :: Int])\n  where\n    even _ = True\n    head _ = (0 :: Int)\n    length _ = (99 :: Int)\n    not _ = False\n    reverse _ = ([7] :: [Int])\n    maximum _ = (98 :: Int)\n";

#[test]
fn local_binders_leave_the_prelude_functions_bound_everywhere_chirho() {
    let ghc_chirho = "(True,4,3)\n(True,[7,1,4],7)\n(True,0,99)\n(False,[7],98)\n";
    for _ in 0..8 {
        assert_interprets_chirho(LOCAL_PRELUDE_NAMES_CHIRHO, ghc_chirho);
    }
    assert_prints_natively_chirho(LOCAL_PRELUDE_NAMES_CHIRHO, ghc_chirho);
}

/// Row 594: a lone local `foldl` beside the Prelude's `sum`. The generated `foldl` took the
/// local binder's id, the only one named so, and row 552's fold in `main` then called the
/// local function. LLVM printed `(4336781493,6)`, the interpreter failed on `AddIntChirho`
/// and Cranelift printed nothing, where GHC prints `(6,6)`.
const LONE_LOCAL_FOLDL_CHIRHO: &str = "module Main where\nmain :: IO ()\nmain = print (sum [1, 2, 3 :: Int], foldl (5 :: Int))\n  where\n    foldl :: Int -> Int\n    foldl n = n + 1\n";

#[test]
fn a_lone_local_foldl_leaves_the_sum_fold_alone_everywhere_chirho() {
    assert_prints_everywhere_chirho(LONE_LOCAL_FOLDL_CHIRHO, "(6,6)\n");
}

// ── Row 566: a long literal unpacks whole across collections ─────────────

/// A 2000-character literal used as a list. Natively it is unpacked cell by cell, and those
/// allocations cross collections. Before row 566 the built suffix was unrooted, and the
/// collector traced tagged chains only two levels deep: `length` gave 240, and the slice read
/// freed cells. GHC 9.14.1 prints `2000` and `fghij`. The control is native-only. The
/// interpreter is exact on an 8 MiB main thread, but `length` over a list this long overflows
/// a test thread's 2 MiB stack; that is a separate interpreter limit, not this route.
fn long_literal_source_chirho() -> String {
    let literal_chirho = "abcdefghij".repeat(200);
    format!(
        "module Main where\nmain :: IO ()\nmain = do\n  print (length \"{literal_chirho}\")\n  putStrLn (take 5 (drop 1995 \"{literal_chirho}\"))\n"
    )
}

#[test]
fn a_long_literal_unpacks_whole_natively_chirho() {
    assert_prints_natively_chirho(&long_literal_source_chirho(), "2000\nfghij\n");
}

// ── Row 595: a list unpacked for a case stays rooted on Cranelift ────────

/// Walks over a long list built from a literal. Each comes with the size at which it failed
/// on Cranelift, GHC 9.14.1's answer there, and its answer at 50.
/// - Cranelift's case on a literal unpacked it into a fresh list nothing rooted. The next
///   allocation's collection could then free its cells while a thunk still held the tail.
/// - From 1,181 copies, `words` failed, by a message that varied with the run (`<<loop>>`, "no
///   matching alternative", "not valid UTF-8"), or hung.
/// - The interpreter's run budget stops the large walks, so it takes the small ones.
const UNPACKED_LIST_WALKS_CHIRHO: [(&str, usize, &str, &str); 4] = [
    (
        "print (length (words (concat (replicate SIZE \"ab \"))))",
        2000,
        "2000\n",
        "50\n",
    ),
    (
        "print (length (concat (replicate SIZE \"ab \")))",
        10000,
        "30000\n",
        "150\n",
    ),
    (
        "print (length (dropWhile (== 'x') (concat (replicate SIZE \"ab \"))))",
        3000,
        "9000\n",
        "150\n",
    ),
    (
        "print (length (lines (concat (replicate SIZE \"ab\\n\"))))",
        10000,
        "10000\n",
        "50\n",
    ),
];

fn unpacked_list_walk_chirho(walk_chirho: &str, size_chirho: usize) -> String {
    with_main_chirho(
        "module Main where\n",
        &walk_chirho.replace("SIZE", &size_chirho.to_string()),
    )
}

#[test]
fn a_list_unpacked_for_a_case_stays_rooted_natively_chirho() {
    for (walk_chirho, size_chirho, expected_chirho, _) in UNPACKED_LIST_WALKS_CHIRHO {
        assert_prints_natively_chirho(
            &unpacked_list_walk_chirho(walk_chirho, size_chirho),
            expected_chirho,
        );
    }
}

#[test]
fn small_unpacked_list_walks_answer_as_ghc_everywhere_chirho() {
    for (walk_chirho, _, _, small_expected_chirho) in UNPACKED_LIST_WALKS_CHIRHO {
        assert_prints_everywhere_chirho(
            &unpacked_list_walk_chirho(walk_chirho, 50),
            small_expected_chirho,
        );
    }
}

/// `Data.Text`'s `unpack`, applied directly and taken first-class, reaches Cranelift's other two
/// unpack sites: the `unpack` application and the `unpack#` primop. Each runs once a step.
/// - No collection was measured inside their window, but it is the same window, so they are
///   rooted too.
/// - The interpreter and LLVM do not run `Data.Text` here, so this control is Cranelift's.
/// - GHC 9.14.1 prints `9000` for both.
const TEXT_UNPACK_SOURCES_CHIRHO: [&str; 2] = [
    "module Main where\nimport qualified Data.Text as T\nmain :: IO ()\nmain = print (length (concatMap (\\_ -> case T.unpack (T.pack \"ab \") of { (c : rest) -> c : rest; [] -> [] }) [1 .. 3000 :: Int]))\n",
    "module Main where\nimport qualified Data.Text as T\nmain :: IO ()\nmain = do { let { unpackers = [T.unpack] }; print (length (concatMap (\\_ -> case head unpackers (T.pack \"ab \") of { (c : rest) -> c : rest; [] -> [] }) [1 .. 3000 :: Int])) }\n",
];

#[test]
fn text_unpack_lists_answer_as_ghc_on_cranelift_chirho() {
    for source_chirho in TEXT_UNPACK_SOURCES_CHIRHO {
        assert_eq!(
            native_round_trip_chirho(source_chirho, CRANELIFT_CHIRHO, "")
                .unwrap_or_else(|error_chirho| panic!("{error_chirho}")),
            (0, "9000\n".to_string())
        );
    }
}

// ── Row 560 (minimal): native Strings carry code points; NUL fails named ──

/// Code points above ASCII through unpack, `reverse`, `toEnum`, `succ` and comparison, printed
/// with `putStrLn`. Before row 560, unpack made one cell per UTF-8 byte (`length` 5 and
/// `[195,169,211,146]`), and pack wrote '?' for each. GHC 9.14.1's output is the expected text.
const CODE_POINTS_CHIRHO: &str = r#"module Main where
main :: IO ()
main = do
  print (length "\233\1234x")
  print (map fromEnum "\233\1234")
  putStrLn (reverse "h\233llo")
  putStrLn [toEnum 233, toEnum 1234]
  putStrLn (map succ "\233z")
  print (length (filter (> 'z') "a\233b\1234"))
"#;

/// A literal holding NUL. GHC 9.14.1 prints `3`; natively its NUL-terminated global read `1`.
const NUL_LITERAL_CHIRHO: &str = r#"module Main where
main :: IO ()
main = print (length "a\0b")
"#;

/// A String built at run time holding NUL. GHC 9.14.1 writes `a`, NUL, `b`; natively pack
/// ended the buffer at the NUL.
const NUL_PACKED_CHIRHO: &str = r#"module Main where
main :: IO ()
main = putStrLn ['a', toEnum 0, 'b']
"#;

/// The same String through `putStr`, which reads it as `putStrLn` does since row 579.
const NUL_PUT_STR_CHIRHO: &str = r#"module Main where
main :: IO ()
main = putStr ['a', toEnum 0, 'b']
"#;

const NUL_STRING_CHIRHO: &str = "string: a NUL character has no native representation";

#[test]
fn native_strings_carry_code_points_everywhere_chirho() {
    assert_prints_everywhere_chirho(CODE_POINTS_CHIRHO, "3\n[233,1234]\nolléh\néӒ\nê{\n2\n");
}

/// The interpreter keeps the NUL; natively it fails by one named line instead of truncating.
#[test]
fn a_nul_character_fails_named_natively_chirho() {
    for (source_chirho, expected_chirho) in [
        (NUL_LITERAL_CHIRHO, "3\n"),
        (NUL_PACKED_CHIRHO, "a\0b\n"),
        (NUL_PUT_STR_CHIRHO, "a\0b"),
    ] {
        let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "NulStringChirho.hs",
            None,
        )
        .expect("the interpreter runs it");
        assert_eq!(
            machine_chirho.io_output_chirho, expected_chirho,
            "interpreter"
        );
        for backend_chirho in BACKENDS_CHIRHO {
            assert_fails_named_chirho(backend_chirho, source_chirho, NUL_STRING_CHIRHO);
        }
    }
}

// ── Row 561: literal escapes decode as GHC lexes them ────────────────────

/// Escaped quotes, `\&`, a decimal escape before digits, the longest ASCII name, caret controls
/// and a gap, in character and string literals. Decoding happens before any engine runs, and
/// `putStrLn` shows the bytes. Before row 561: `'\''` decoded to U+FFFD, `\&` stayed, `\012` was NUL
/// and "12", and `\SO1` and `\^A` stayed as written. GHC 9.14.1's output is the expected text.
const LITERAL_ESCAPES_CHIRHO: &str = r#"module Main where
main :: IO ()
main = do
  putStrLn ['\'', '"', '\\', '\^A', '\^[', '\x41', '\o101', '\65']
  putStrLn "a\&b\&|\1234\&5|\SO\&H"
  putStrLn "\012x\01\65|\SO1\SOH2|\DELx"
  putStrLn "\^A\^Z\^_\^\\^]x"
  putStrLn "it\'s \"q\" ab\   \cd"
"#;

#[test]
fn literal_escapes_decode_as_ghc_lexes_them_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        LITERAL_ESCAPES_CHIRHO,
        "'\"\\\u{1}\u{1b}AAA\nab|\u{4d2}5|\u{e}H\n\u{c}x\u{1}A|\u{e}1\u{1}2|\u{7f}x\n\u{1}\u{1a}\u{1f}\u{1c}\u{1d}x\nit's \"q\" abcd\n",
    );
}

// ── Row 557: Char and String show as GHC's showLitChar spells them ──────

/// Quotes, backslashes, control names, decimal escapes and both `\&` separators, in
/// literals and in strings built at run time, which natively were shown as `""`. Raw
/// strings keep the Haskell escapes readable. GHC 9.14.1's output is
/// [`SHOW_TEXT_GHC_CHIRHO`].
const SHOW_TEXT_CHIRHO: &str = r#"module Main where
main :: IO ()
main = do
  print "a\"b\\c\td"
  print ("\1234" ++ "5", "\SO" ++ "H")
  print "\DEL\1\31\233x"
  print (show "a\nb")
  print (head "'")
  print '\n'
  print '\\'
  print '"'
  print '\1234'
  print (reverse "c\nba")
  print ["ab", reverse "dc"]
  print (Just (reverse "\"x"))
  print (concat ["\SO", "H"])
  print (filter (== 'z') "abc")
"#;

/// GHC 9.14.1's output for [`SHOW_TEXT_CHIRHO`].
const SHOW_TEXT_GHC_CHIRHO: &str = r#""a\"b\\c\td"
("\1234\&5","\SO\&H")
"\DEL\SOH\US\233x"
"\"a\\nb\""
'\''
'\n'
'\\'
'"'
'\1234'
"ab\nc"
["ab","cd"]
Just "x\""
"\SO\&H"
""
"#;

/// A runtime-built String above ASCII. GHC 9.14.1 prints `"x\233"`. Row 560's unpack gives
/// code points, so every engine reads it exactly (before: a cell per UTF-8 byte).
const RUNTIME_NON_ASCII_CHIRHO: &str = r#"module Main where
main :: IO ()
main = print (reverse "\233x")
"#;

/// GHC 9.14.1 prints `"Hi"`. With row 520's Char `toEnum`, the String the renderer reads is
/// all characters on every engine (before row 520 the interpreter's cells held Ints).
const TO_ENUM_STRING_CHIRHO: &str = r#"module Main where
main :: IO ()
main = print (map toEnum [72, 105] :: String)
"#;

#[test]
fn chars_and_strings_are_shown_as_ghc_shows_them_everywhere_chirho() {
    assert_prints_everywhere_chirho(SHOW_TEXT_CHIRHO, SHOW_TEXT_GHC_CHIRHO);
}

#[test]
fn a_runtime_string_above_ascii_shows_as_ghc_shows_it_everywhere_chirho() {
    assert_prints_everywhere_chirho(RUNTIME_NON_ASCII_CHIRHO, "\"x\\233\"\n");
}

#[test]
fn a_string_of_converted_characters_shows_as_ghc_shows_it_everywhere_chirho() {
    assert_prints_everywhere_chirho(TO_ENUM_STRING_CHIRHO, "\"Hi\"\n");
}

// ── Row 576: native ++ and literal tails read whole ──────────────────────

/// Runtime lists through `++`, and a cons onto a literal, shown and printed. Natively `++#`
/// read a list as "", so `"<" ++ reverse "abc" ++ ">"` printed `<>`; the pack and show walkers
/// stopped at a literal tail, so `'a' : "bc"` printed `a` or was refused; and the interpreter's
/// `++#` dropped that tail (`ad`). GHC 9.14.1's output is the expected text.
const STRING_APPEND_TAILS_CHIRHO: &str = r#"module Main where
main :: IO ()
main = do
  putStrLn ("<" ++ reverse "abc" ++ ">")
  putStrLn ('a' : "bc")
  putStrLn (('a' : "bc") ++ "d")
  print ('a' : "bc")
  putStrLn ("x" ++ map succ "ab")
  print (length ("ab" ++ reverse "cd"))
  putStrLn (reverse "ba" ++ reverse "dc")
  print ("<" ++ reverse "ab")
  putStrLn (replicate 2 'x' ++ "y")
"#;

#[test]
fn appends_and_literal_tails_read_whole_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        STRING_APPEND_TAILS_CHIRHO,
        "<cba>\nabc\nabcd\n\"abc\"\nxbc\n4\nabcd\n\"<ba\"\nxxy\n",
    );
}

// ── Row 579: lazy Core text functions, and putStr reading a list ─────────

/// `words`, `lines`, `unwords`, `unlines` and `intercalate` over Strings built at run time,
/// empty and terminal cases, GHC's `isSpace` set (U+0085 and U+2028 are no spaces), and
/// infinite input. They were strict primops that read only a literal: a run-time String read
/// as "", and an infinite one never ended. GHC 9.14.1's output is the expected text.
const REPORT_TEXT_CHIRHO: &str = r#"module Main where
import Data.Char (isSpace)
import Data.List (intercalate)
main :: IO ()
main = do
  print (words (reverse "b a"))
  print (lines (reverse "\n\nb\n\na"))
  putStrLn (unwords ["x", map succ "ab"])
  print (unlines [reverse "ba", "cd"])
  putStrLn (intercalate ", " [reverse "ba", "cd"])
  print (words "", lines "")
  print (unwords [], unlines [])
  print (lines "a\n", lines "\n\n")
  print (words "a\tb\nc\x85\&d\xa0\&e\x3000\&f")
  print (map isSpace "\t\x85\xa0\x2028\x3000\&a")
  print (take 1 (lines (cycle "a\n")))
  print (take 2 (words (cycle "a ")))
  putStrLn (take 3 (head (lines (repeat 'a'))))
  putStrLn (take 5 (unwords (repeat "ab")))
  putStrLn (take 7 (intercalate ", " (repeat "ab")))
"#;

/// `putStr` of a literal, a String built at run time, a cons onto a literal, and `unlines`.
/// Natively `putStr` read a list as a buffer and printed nothing. GHC 9.14.1's output.
const PUT_STR_LISTS_CHIRHO: &str = r#"module Main where
main :: IO ()
main = do
  putStr "ab\n"
  putStr (reverse "\ndc")
  putStr ('e' : "f\n")
  putStr (unlines [reverse "hg", "ij"])
"#;

#[test]
fn report_text_functions_are_lazy_and_exact_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        REPORT_TEXT_CHIRHO,
        "[\"a\",\"b\"]\n[\"a\",\"\",\"b\",\"\"]\nx bc\n\"ab\\ncd\\n\"\nab, cd\n([],[])\n(\"\",\"\")\n([\"a\"],[\"\",\"\"])\n[\"a\",\"b\",\"c\\133d\",\"e\",\"f\"]\n[True,False,True,False,True,False]\n[\"a\"]\n[\"a\",\"a\"]\naaa\nab ab\nab, ab,\n",
    );
}

#[test]
fn put_str_reads_every_string_representation_everywhere_chirho() {
    assert_prints_everywhere_chirho(PUT_STR_LISTS_CHIRHO, "ab\ncd\nef\ngh\nij\n");
}

// ── R1: an unbound reference fails named, and only when demanded ──────────

#[test]
fn an_unbound_call_fails_named_on_llvm_chirho() {
    assert_fails_named_chirho(
        LLVM_CHIRHO,
        UNBOUND_CALL_CHIRHO,
        "missing native binding `realToFrac`",
    );
}

#[test]
fn an_unbound_call_fails_named_on_cranelift_chirho() {
    assert_fails_named_chirho(
        CRANELIFT_CHIRHO,
        UNBOUND_CALL_CHIRHO,
        "missing native binding `realToFrac`",
    );
}

#[test]
fn an_unbound_constant_fails_named_on_llvm_chirho() {
    assert_fails_named_chirho(
        LLVM_CHIRHO,
        UNBOUND_CONSTANT_CHIRHO,
        "missing native binding `realToFrac`",
    );
}

#[test]
fn an_unbound_constant_fails_named_on_cranelift_chirho() {
    assert_fails_named_chirho(
        CRANELIFT_CHIRHO,
        UNBOUND_CONSTANT_CHIRHO,
        "missing native binding `realToFrac`",
    );
}

#[test]
fn an_unbound_name_on_an_untaken_path_runs_everywhere_chirho() {
    assert_prints_everywhere_chirho(UNBOUND_UNTAKEN_CHIRHO, "7.5\n");
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

// ── R4: formerly unknown primitives lower only with measured semantics ─────

#[test]
fn io_ref_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(IOREF_CHIRHO, "2\n");
}

#[test]
fn integer_power_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(POWER_CHIRHO, "1\n1024\n-27\n");
}

#[test]
fn a_negative_integer_exponent_fails_named_everywhere_chirho() {
    for source_chirho in NEGATIVE_POWER_CASES_CHIRHO {
        assert_fails_everywhere_chirho(source_chirho, "Negative exponent");
    }
}

#[test]
fn from_integral_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(FROM_INTEGRAL_CHIRHO, "3.5\n");
}

#[test]
fn compare_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(COMPARE2_CHIRHO, "GT\nGT\nLT\nGT\n");
}

/// Row 539 covers the native backends; row 542 closes the interpreter's
/// unordered-Double residual, so the shared edge contract now holds everywhere.
#[test]
fn compare_edges_answer_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(COMPARE_EDGES_CHIRHO, COMPARE_EDGES_GHC_CHIRHO);
}

#[test]
fn char_case_mapping_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(CHAR_CASE_MAPPING_CHIRHO, "ABC\n");
}

#[test]
fn singleton_map_size_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(MAP_SINGLETON_SIZE_CHIRHO, "1\n");
}

#[test]
fn ordered_map_to_list_stays_refused_natively_chirho() {
    for backend_chirho in BACKENDS_CHIRHO {
        let error_chirho =
            native_round_trip_chirho(MAP_TO_LIST_BOUNDARY_CHIRHO, backend_chirho, "")
                .expect_err("native ordering is unavailable without comparator evidence");
        assert!(
            error_chirho.contains("does not support primitive(s): mapToList#/1"),
            "{backend_chirho:?}: {error_chirho}"
        );
    }
}

#[test]
fn multi_pair_map_size_fails_named_natively_chirho() {
    for backend_chirho in BACKENDS_CHIRHO {
        assert_eq!(
            native_round_trip_chirho(MAP_ORDERING_BOUNDARY_CHIRHO, backend_chirho, "")
                .expect_err("native ordering is unavailable without comparator evidence"),
            "native program emitted stderr (exit 1): haskelujah: internal error: mapFromList# native maps currently require at most one pair\n\nstdout:\n",
            "{backend_chirho:?}"
        );
    }
}

// ── R5: Integral results keep their tuple evidence ────────────────────────

#[test]
fn gcd_and_lcm_pair_answers_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(NO_MATCH_CHIRHO, "(6,12)\n");
}
