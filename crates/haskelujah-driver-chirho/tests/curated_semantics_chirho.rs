// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Focused executable regressions exposed when the curated output oracles were
//! first recognized. Expected outputs checked independently with GHC 9.14.1.
//! Row 555's prefix-minus controls also run on both native backends.

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

fn assert_output_chirho(source_chirho: &str, expected_chirho: &str) {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        "CuratedRegressionChirho.hs",
        None,
    )
    .unwrap();
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho);
}

/// The interpreter, LLVM and Cranelift each print `expected_chirho`.
fn assert_everywhere_chirho(source_chirho: &str, expected_chirho: &str) {
    assert_output_chirho(source_chirho, expected_chirho);
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let (status_chirho, output_chirho) =
            native_round_trip_chirho(source_chirho, backend_chirho, "").unwrap();
        assert_eq!(
            (status_chirho, output_chirho.as_str()),
            (0, expected_chirho),
            "{backend_chirho:?}"
        );
    }
}

#[test]
fn backtick_arithmetic_uses_the_declared_fixity_chirho() {
    assert_output_chirho(
        r#"module Main where
main = do
  print (100 * 101 `div` 2)
  print (10 `rem` 3 * 4)
  print (10 * 3 `quot` 4)
  print (10 * 3 `mod` 4)
"#,
        "5050\n4\n7\n2\n",
    );
}

/// Row 555: prefix minus is `infixl 6`, so a negation takes in each following operator
/// that binds tighter. All three engines printed `(negate a) op b` for the Int lines
/// (`-3 ^ 2` was 9 and ``-7 `mod` 3`` was 2), and `- [10, 20, 30] !! 1` did not
/// type-check. `max` has no fixity declaration, so it is `infixl 9` and taken in too.
/// `-xChirho * 2` negated a top-level Double that has no type key: the interpreter
/// stopped at `NegIntChirho` and both natives printed -6.0. The negation's operand is
/// now the product, which keys as Double.
#[test]
fn prefix_minus_takes_in_the_operators_that_bind_tighter_chirho() {
    assert_everywhere_chirho(
        r#"module Main where
xChirho :: Double
xChirho = 1.5
main :: IO ()
main = do
  print (-3 ^ 2 :: Int)
  print (- 2 ^ 2 :: Int)
  print (-7 `mod` 3 :: Int)
  print (-7 `div` 2 :: Int)
  print (-2 ^ 2 + 1 :: Int)
  print ((-4 :: Int) == - 2 ^ 2)
  print ((\x -> -x ^ 2) (3 :: Int))
  print (-3 `max` 2 :: Int)
  print (- [10, 20, 30] !! 1 :: Int)
  print (-xChirho * 2)
"#,
        "-9\n-4\n-1\n-3\n-3\nTrue\n-9\n-3\n-20\n-3.0\n",
    );
}

/// Row 555's boundaries: `*`, `/`, `rem` and `quot` regroup under the negation but print
/// the same values at Int and Double. A parenthesized negation, `negate`, a negative
/// argument or section operand, and an operator at or below precedence 6 keep their
/// grouping.
#[test]
fn negation_boundaries_and_holds_keep_their_values_chirho() {
    assert_everywhere_chirho(
        r#"module Main where
main :: IO ()
main = do
  print (-3 * 2 + 1 :: Int)
  print (-7 `rem` 2 :: Int)
  print (-7 `quot` 2 :: Int)
  print (-3 / 2 :: Double)
  print (-2.5 * 2 :: Double)
  print ((-3) ^ 2 :: Int)
  print (negate 3 ^ 2 :: Int)
  print (abs (-3) ^ 2 :: Int)
  print (-3 + 2 :: Int)
  print (-3 < (2 :: Int))
  print (map (+ (-3)) [1, 2 :: Int])
"#,
        "-5\n-1\n-3\n-1.5\n-5.0\n9\n9\n9\n-1\nTrue\n[-2,-1]\n",
    );
}

/// Row 555's named residual, which is numeric evidence, not precedence. `-2 ** 2` is now
/// `negate (2 ** 2)`, and the dictionary pass has no key for a `**` result, so the
/// negation stays `negate#`. The interpreter stops at `NegIntChirho`, where it used to
/// print a wrong 4.0 (GHC: -4.0). Both natives refuse `**#`, with or without the negation.
#[test]
fn prefix_minus_over_a_float_power_stays_the_named_residual_chirho() {
    let error_chirho = eval_source_with_machine_chirho(
        "module Main where\nmain :: IO ()\nmain = print (-2 ** 2 :: Double)\n",
        &mut SourceMapChirho::new_chirho(),
        "CuratedRegressionChirho.hs",
        None,
    )
    .map(|_| ())
    .expect_err("row 555 leaves `negate#` over a `**` result unkeyed");
    assert!(
        error_chirho.to_string().contains("NegIntChirho"),
        "{error_chirho}"
    );
}

#[test]
fn euler_sum_square_difference_chirho() {
    assert_output_chirho(
        include_str!("../../../ghc-tests-chirho/T149_euler6.hs"),
        "25164150\n",
    );
}

#[test]
fn recursive_equality_and_show_use_element_evidence_chirho() {
    assert_output_chirho(
        include_str!("../../../ghc-tests-chirho/T278_intersperse.hs"),
        "1-2-3\n",
    );
    assert_output_chirho(include_str!("../../../ghc-tests-chirho/T423_nub.hs"), "5\n");
    assert_output_chirho(
        include_str!("../../../ghc-tests-chirho/T424_intercalate.hs"),
        "1-2-3-4-5\n",
    );
    assert_output_chirho(
        include_str!("../../../ghc-tests-chirho/T426_palindrome.hs"),
        "1\n0\n",
    );
}
