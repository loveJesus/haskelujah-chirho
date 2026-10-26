// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Integer literals divided at Double (row 543). An integer literal desugars to
//! `fromInteger n`, and the dictionary pass keyed that application `Int` by name. So a
//! division whose first argument was such a literal found no `Fractional Int` row and
//! left `/` unbound: `print (7 / 2)` failed with `missing STG binding /` in the
//! interpreter and `missing native binding /` on both native backends. The key is now
//! the type the checker solved for that literal occurrence, and `Int` only without one.
//! Each program's GHC 9.14.1 output is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_native_chirho, run_chirho};

#[test]
fn integer_literals_divide_at_double_everywhere_chirho() {
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "D02LitHalfChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (1 / 2 :: Double)\n",
            "0.5\n",
        ),
        (
            "D05TopLevelChirho.hs",
            "module Main where\nxChirho :: Double\nxChirho = 1 / 2\nmain :: IO ()\nmain = print xChirho\n",
            "0.5\n",
        ),
        (
            "D06DefaultedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (7 / 2)\n",
            "3.5\n",
        ),
        // With row 539's native compareFloat# and row 542's interpreter NaN ordering.
        (
            "D04CompareLitChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare (0 / 0 :: Double) 1)\n",
            "GT\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho} interpreter"
        );
        assert_native_chirho(file_name_chirho, program_chirho, ghc_chirho);
    }
}

#[test]
fn integer_literal_non_finite_values_render_in_the_interpreter_chirho() {
    for (file_name_chirho, expression_chirho, ghc_chirho) in [
        ("D01NaNChirho.hs", "0 / 0", "NaN\n"),
        ("D01InfinityChirho.hs", "1 / 0", "Infinity\n"),
        (
            "D01NegativeInfinityChirho.hs",
            "negate (1 / 0 :: Double)",
            "-Infinity\n",
        ),
    ] {
        let program_chirho = format!(
            "module Main where\nmain :: IO ()\nmain = print ({expression_chirho} :: Double)\n"
        );
        assert_eq!(
            run_chirho(file_name_chirho, &program_chirho),
            ghc_chirho,
            "{file_name_chirho} interpreter"
        );
        if expression_chirho == "0 / 0" {
            assert_native_chirho(file_name_chirho, &program_chirho, ghc_chirho);
        }
    }
}

/// GHC 9.14.1 changes from fixed to scientific notation below exponent -1 and above
/// exponent 6. Native whole-Double formatting remains a separate residual.
#[test]
fn finite_double_show_uses_ghcs_exponent_boundaries_in_the_interpreter_chirho() {
    let program_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (9999999.0 :: Double)\n  print (10000000.0 :: Double)\n  print (0.1 :: Double)\n  print (0.01 :: Double)\n  print (1.0e20 :: Double)\n  print (1.0e-20 :: Double)\n";
    assert_eq!(
        run_chirho("FiniteDoubleShowChirho.hs", program_chirho),
        "9999999.0\n1.0e7\n0.1\n1.0e-2\n1.0e20\n1.0e-20\n"
    );
}

/// A literal the checker solved at Int still dispatches at Int.
#[test]
fn integer_literals_at_int_still_dispatch_at_int_chirho() {
    let program_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (7 `div` 2 :: Int)\n  print (2 ^ (10 :: Int) :: Int)\n";
    assert_eq!(
        run_chirho("IntLiteralsChirho.hs", program_chirho),
        "3\n1024\n"
    );
    assert_native_chirho("IntLiteralsChirho.hs", program_chirho, "3\n1024\n");
}
