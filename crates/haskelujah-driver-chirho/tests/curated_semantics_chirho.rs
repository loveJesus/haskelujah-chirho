// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Focused executable regressions exposed when the curated output oracles were
//! first recognized. Expected outputs checked independently with GHC 9.14.1.

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

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
