// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dictionary evidence, end to end through the interpreter: a program is not
//! done when it type-checks, it is done when it runs and prints the right
//! thing. Every program here is accepted by GHC with the given output.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn run_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    match eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    ) {
        Ok((_value_chirho, machine_chirho)) => machine_chirho.io_output_chirho,
        Err(err_chirho) => {
            panic!("{file_name_chirho}: GHC runs this program; we failed: {err_chirho}")
        }
    }
}

#[test]
fn literal_under_a_builtin_with_a_string_sibling_chirho() {
    // The pass used to key this literal's `fromInteger` off the string sibling
    // (`Num Char`) and rescue it only through the unsigned binding's scheme.
    assert_eq!(
        run_chirho(
            "TakeStringChirho.hs",
            "module Main where\nmain = putStrLn (take 5 \"hello world\")\n"
        ),
        "hello\n"
    );
}

#[test]
fn literal_in_a_signed_binding_with_a_string_sibling_chirho() {
    // The signed form has no scheme predicates to fall back on: only the
    // checker's evidence for the literal can dispatch it.
    assert_eq!(
        run_chirho(
            "TakeStringSignedChirho.hs",
            "module Main where\nxs :: String\nxs = take 5 \"hello world\"\nmain :: IO ()\nmain = putStrLn xs\n"
        ),
        "hello\n"
    );
}

#[test]
fn literals_under_user_functions_in_a_signed_main_chirho() {
    // The shape of 48 curated programs at dd6d694a (T035_either): a signed
    // `main`, literals under user functions with non-Int siblings.
    assert_eq!(
        run_chirho(
            "SignedMainLiteralsChirho.hs",
            "module Main where\nfromRightChirho :: Int -> Either Int Int -> Int\nfromRightChirho def (Left _) = def\nfromRightChirho _ (Right x) = x\nmain :: IO ()\nmain = do\n  print (fromRightChirho 0 (Right 42))\n  print (fromRightChirho (-1) (Left 99))\n"
        ),
        "42\n-1\n"
    );
}

#[test]
fn method_at_a_rigid_variable_uses_the_bindings_own_dictionary_chirho() {
    // The `+` inside `f` was defaulted to the Int row; `f 2.5` hit the Int primop.
    assert_eq!(
        run_chirho(
            "RigidMethodChirho.hs",
            "module Main where
fChirho :: Num a => a -> a
fChirho x = x + 1
main :: IO ()
main = do
  print (fChirho 2.5)
  print (fChirho (2 :: Int))
"
        ),
        "3.5
3
"
    );
}

#[test]
fn call_sites_pass_the_dictionaries_the_checker_proved_chirho() {
    // `g 5` printed a partial application (no dictionary at the call);
    // `both True 3` took the wrong dictionary for its second predicate;
    // `k = h` is a reference with no arguments to guess from.
    let source_chirho = r#"module Main where
gChirho :: Fractional a => a -> a
gChirho x = x / 2
class DescribeChirho a where
  describeChirho :: a -> String
instance DescribeChirho Bool where
  describeChirho b = if b then "yes" else "no"
instance DescribeChirho Int where
  describeChirho n = show n
bothChirho :: (DescribeChirho a, DescribeChirho b) => a -> b -> String
bothChirho x y = describeChirho x ++ "/" ++ describeChirho y
hChirho :: Num a => a -> a
hChirho x = x + 1
kChirho :: Double -> Double
kChirho = hChirho
main :: IO ()
main = do
  print (gChirho 5)
  putStrLn (bothChirho True (3 :: Int))
  print (kChirho 2.5)
"#;
    assert_eq!(
        run_chirho("CallSiteDictsChirho.hs", source_chirho),
        "2.5\nyes/3\n3.5\n"
    );
}

#[test]
fn integer_literal_at_double_is_dispatched_at_double_chirho() {
    assert_eq!(
        run_chirho(
            "LiteralAtDoubleChirho.hs",
            "module Main where\nhalfChirho :: Double -> Double\nhalfChirho x = x / 2\nmain :: IO ()\nmain = print (halfChirho 5)\n"
        ),
        "2.5\n"
    );
}
