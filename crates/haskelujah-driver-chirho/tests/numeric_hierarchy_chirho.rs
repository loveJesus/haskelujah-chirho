// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC's numeric class hierarchy, not the Haskell 98 report's: `Num` has no
//! superclasses (a Num dictionary has no superclass slots, and `Num a` does not
//! entail `Eq a` or `Show a`), and `Integral` inherits `Eq`/`Ord` through
//! `Real`. Every expected output is GHC 9.14.1's own (runghc).
//!
//! Scope: the interpreter. On untouched main both native backends already
//! print wrong values for these very programs (custom-Num literals, `half 5`
//! as 0, `double 1.5` as 3; LLVM lacks `fromIntegral#`/`recip#`), so a native
//! assertion here could not tell this correction from those defects; native
//! layout consistency is covered by the existing round-trip suites.
//! workflow: language-features-chirho/instance-obligations-chirho

use haskelujah_driver::{compile_source_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;

fn output_chirho(name_chirho: &str, source_chirho: &str) -> String {
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        name_chirho,
        None,
    )
    .unwrap_or_else(|err_chirho| panic!("{name_chirho} must run: {err_chirho}"));
    machine_chirho.io_output_chirho
}

#[test]
fn a_num_instance_needs_no_eq_or_show_instance_chirho() {
    // Arithmetic goes through helpers: an inner use of the class's own method
    // at another type inside an instance body is a separate, filed defect.
    let source_chirho = r#"module Main where
data V = V Int
plusInt :: Int -> Int -> Int
plusInt a b = a + b
timesInt :: Int -> Int -> Int
timesInt a b = a * b
absInt :: Int -> Int
absInt a = if a < 0 then 0 - a else a
instance Num V where
  V a + V b = V (plusInt a b)
  V a * V b = V (timesInt a b)
  abs (V a) = V (absInt a)
  signum v = v
  fromInteger n = V (fromInteger n)
  negate (V a) = V (0 - a)
unV :: V -> Int
unV (V n) = n
main :: IO ()
main = do
  print (unV (V 2 * 3 + 1))
  print (unV (negate (V 5) + abs (V (-4))))
"#;
    assert_eq!(
        output_chirho("CustomNumChirho.hs", source_chirho),
        "7\n-1\n"
    );
}

#[test]
fn arithmetic_runs_through_a_passed_num_dictionary_chirho() {
    let source_chirho = r#"module Main where
double :: Num a => a -> a
double x = x + x
square :: Num a => a -> a
square x = x * x
main :: IO ()
main = do
  print (double (21 :: Int))
  print (double (1.5 :: Double))
  print (square (double (3 :: Integer)))
"#;
    assert_eq!(
        output_chirho("PassedDictionaryChirho.hs", source_chirho),
        "42\n3.0\n36\n"
    );
}

#[test]
fn integral_inherits_equality_through_real_and_ord_chirho() {
    let source_chirho = r#"module Main where
isZero :: Integral a => a -> Bool
isZero n = n == 0
isEven :: Integral a => a -> Bool
isEven n = n `mod` 2 == 0
half :: Fractional a => a -> a
half x = x / 2
main :: IO ()
main = do
  print (isZero (0 :: Int))
  print (isZero (5 :: Integer))
  print (isEven (10 :: Int))
  print (half (5 :: Double))
"#;
    assert_eq!(
        output_chirho("InheritedChirho.hs", source_chirho),
        "True\nFalse\nTrue\n2.5\n"
    );
}

fn rejected_for_chirho(name_chirho: &str, source_chirho: &str, reason_chirho: &str) {
    match compile_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        name_chirho,
    ) {
        Err(err_chirho) => {
            let text_chirho = format!("{err_chirho}");
            assert!(
                text_chirho.contains(reason_chirho),
                "{name_chirho}: rejected, but not for `{reason_chirho}`: {text_chirho}"
            );
        }
        Ok(_) => {
            panic!("{name_chirho}: GHC rejects this (GHC-39999 could not deduce); we accepted it")
        }
    }
}

#[test]
fn num_does_not_entail_eq_chirho() {
    rejected_for_chirho(
        "NumIsNotEqChirho.hs",
        "module Main where\nsame :: Num a => a -> a -> Bool\nsame x y = x == y\nmain :: IO ()\nmain = print (same (1 :: Int) 1)\n",
        "Eq",
    );
}

#[test]
fn num_does_not_entail_show_chirho() {
    rejected_for_chirho(
        "NumIsNotShowChirho.hs",
        "module Main where\nrender :: Num a => a -> String\nrender x = show x\nmain :: IO ()\nmain = putStrLn (render (1 :: Int))\n",
        "Show",
    );
}
