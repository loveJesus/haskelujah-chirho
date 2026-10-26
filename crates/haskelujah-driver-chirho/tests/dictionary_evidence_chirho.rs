// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Dictionary evidence, end to end through the interpreter: a program is not
//! done when it type-checks, it is done when it runs and prints the right
//! thing. Every program here is accepted by GHC with the given output.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

// A test crate root's `mod` resolves beside the root file, so the submodule
// in this target's own directory is named by path.
#[path = "dictionary_evidence_chirho/ordering_chirho/compare_route_chirho.rs"]
mod compare_route_chirho;
#[path = "dictionary_evidence_chirho/derived_show_chirho.rs"]
mod derived_show_chirho;
#[path = "dictionary_evidence_chirho/ordering_chirho/double_primops_chirho.rs"]
mod double_primops_chirho;
#[path = "dictionary_evidence_chirho/enum_occurrences_chirho.rs"]
mod enum_occurrences_chirho;
#[path = "dictionary_evidence_chirho/equality_sections_chirho.rs"]
mod equality_sections_chirho;
#[path = "dictionary_evidence_chirho/ordering_chirho/extrema_chirho.rs"]
mod extrema_chirho;
#[path = "dictionary_evidence_chirho/fractional_literals_chirho.rs"]
mod fractional_literals_chirho;
#[path = "dictionary_evidence_chirho/ground_keys_chirho.rs"]
mod ground_keys_chirho;
#[path = "dictionary_evidence_chirho/higher_order_occurrences_chirho.rs"]
mod higher_order_occurrences_chirho;
#[path = "dictionary_evidence_chirho/list_orders_chirho.rs"]
mod list_orders_chirho;
#[path = "dictionary_evidence_chirho/ordering_chirho/ordering_literals_chirho.rs"]
mod ordering_literals_chirho;
#[path = "dictionary_evidence_chirho/proven_occurrences_chirho.rs"]
mod proven_occurrences_chirho;
#[path = "dictionary_evidence_chirho/string_lists_chirho.rs"]
mod string_lists_chirho;

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

/// The program exits 0 with GHC's output on both native backends.
fn assert_native_chirho(file_name_chirho: &str, program_chirho: &str, ghc_chirho: &str) {
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_round_trip_chirho(program_chirho, backend_chirho, "").unwrap_or_else(
                |error_chirho| panic!("{file_name_chirho} {backend_chirho:?}: {error_chirho}")
            ),
            (0, ghc_chirho.to_string()),
            "{file_name_chirho} {backend_chirho:?}"
        );
    }
}

/// Every engine stops normally and names the same bounded runtime obligation.
fn assert_named_failure_everywhere_chirho(
    file_name_chirho: &str,
    program_chirho: &str,
    needle_chirho: &str,
) {
    let interpreter_chirho = run_result_chirho(file_name_chirho, program_chirho)
        .expect_err("the interpreter must fail by name");
    assert!(
        interpreter_chirho.contains(needle_chirho),
        "{file_name_chirho} interpreter: {interpreter_chirho}"
    );

    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let native_chirho = native_round_trip_chirho(program_chirho, backend_chirho, "")
            .expect_err("the native program must fail by name");
        assert!(
            native_chirho.contains(needle_chirho),
            "{file_name_chirho} {backend_chirho:?}: {native_chirho}"
        );
    }
}

/// The run's output, or the error that stopped it.
fn run_result_chirho(file_name_chirho: &str, source_chirho: &str) -> Result<String, String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    )
    .map(|(_value_chirho, machine_chirho)| machine_chirho.io_output_chirho)
    .map_err(|err_chirho| err_chirho.to_string())
}

/// GHC's exact output, or a failure that names the obligation: never another
/// dictionary's answer.
fn assert_ghc_or_named_chirho(
    file_name_chirho: &str,
    source_chirho: &str,
    ghc_chirho: &str,
    obligation_chirho: &str,
) {
    match run_result_chirho(file_name_chirho, source_chirho) {
        Ok(output_chirho) => assert_eq!(
            output_chirho, ghc_chirho,
            "{file_name_chirho}: neither GHC's output nor a named failure"
        ),
        Err(err_chirho) => assert!(
            err_chirho.contains(&format!("no instance dictionary for `{obligation_chirho}`")),
            "{file_name_chirho}: the failure must name `{obligation_chirho}`: {err_chirho}"
        ),
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
fn local_bindings_are_served_by_the_type_they_are_instantiated_at_chirho() {
    // A `where` binding generalized over `Num` gets neither a dictionary
    // parameter nor a specialization from the pass; the checker's evidence
    // says every instantiation is at Int, transitively through a local
    // binding instantiated only by another local binding (`isPrime` →
    // `checkDiv`). The shape of 14 curated programs at dd6d694a.
    let source_chirho = r#"module Main where
fibChirho :: Int -> Int
fibChirho n = go n 0 1
  where go 0 a _ = a
        go k a b = go (k - 1) b (a + b)
countPrimesChirho :: Int -> Int
countPrimesChirho limit = sieve 2 0
  where
    sieve n count
      | n >= limit = count
      | isPrime n = sieve (n + 1) (count + 1)
      | otherwise = sieve (n + 1) count
    isPrime n = checkDiv n 2
    checkDiv n d
      | d * d > n = True
      | n `mod` d == 0 = False
      | otherwise = checkDiv n (d + 1)
main :: IO ()
main = do
  print (fibChirho 30)
  print (countPrimesChirho 100)
"#;
    assert_eq!(
        run_chirho("LocalBindingsChirho.hs", source_chirho),
        "832040\n25\n"
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

#[test]
fn result_only_applicative_and_monad_parameters_are_passed_and_used_chirho() {
    assert_eq!(
        run_chirho(
            "ApplicativeEvidenceChirho.hs",
            include_str!("dictionary_evidence_chirho/applicative_chirho.hs"),
        ),
        "Just 4\n[5]\n6\nJust 7\n[8]\n(Just 1,Right 2)\nRight [10]\n",
    );
}

#[test]
fn recursive_calls_preserve_the_predicates_element_type_chirho() {
    let source_chirho = r#"module Main where
renderChirho _ [] = ""
renderChirho _ [x] = show x
renderChirho sep (x:xs) = show x ++ sep ++ renderChirho sep xs
memberChirho _ [] = False
memberChirho x (y:ys) = x == y || memberChirho x ys
dedupChirho [] = []
dedupChirho (x:xs) = if memberChirho x xs then dedupChirho xs else x : dedupChirho xs
main = do
  putStrLn (renderChirho "-" [1,2,3])
  putStrLn (renderChirho "/" [True,False,True])
  putStrLn (renderChirho "/" [1.5,2.5,3.5])
  print (dedupChirho [1,2,3,2,1,4,3,5,1])
"#;
    assert_eq!(
        run_chirho("RecursiveEvidenceChirho.hs", source_chirho),
        "1-2-3\nTrue/False/True\n1.5/2.5/3.5\n[2,4,3,5,1]\n"
    );
}

#[test]
fn tuple_and_result_variables_keep_their_numeric_dictionary_chirho() {
    assert_eq!(
        run_chirho(
            "TupleEvidenceChirho.hs",
            r#"module Main where
addPairChirho (x,y) = x + y
main = do
  print (addPairChirho (1.25, 2.5))
  print (addPairChirho (1 :: Int, 2))
"#
        ),
        "3.75\n3\n"
    );
}

// A test crate root's `mod` resolves beside the root file, so the submodule
// in this target's own directory is named by path.
#[path = "dictionary_evidence_chirho/data_map_tolist_chirho.rs"]
mod data_map_tolist_chirho;
#[path = "dictionary_evidence_chirho/generic_heads_chirho.rs"]
mod generic_heads_chirho;
#[path = "dictionary_evidence_chirho/non_empty_routes_chirho.rs"]
mod non_empty_routes_chirho;
