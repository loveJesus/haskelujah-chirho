// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Returned values must survive another allocating call after their callee frame retires.
//! Expectations are predictions until the scheduler runs these unchanged sources.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use std::time::Duration;

use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, measure_native_runtime_chirho,
};

const PRELUDE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
data BoxChirho = BoxChirho !Int
data PairChirho = PairChirho BoxChirho BoxChirho
{-# NOINLINE makeBoxChirho #-}
makeBoxChirho :: Int -> BoxChirho
makeBoxChirho valueChirho = BoxChirho valueChirho
{-# NOINLINE burnChirho #-}
burnChirho :: Int -> Int
burnChirho remainingChirho =
  if remainingChirho == 0 then 0
  else case makeBoxChirho remainingChirho of
    BoxChirho copiedChirho -> burnChirho (copiedChirho - 1)
{-# NOINLINE makePairChirho #-}
makePairChirho :: Int -> PairChirho
makePairChirho valueChirho = PairChirho (makeBoxChirho valueChirho) (makeBoxChirho (valueChirho + 1))
{-# NOINLINE makeClosureChirho #-}
makeClosureChirho :: BoxChirho -> Int -> Int
makeClosureChirho boxChirho = case boxChirho of
  BoxChirho savedChirho -> if savedChirho > 0 then (\valueChirho -> savedChirho + valueChirho) else (\valueChirho -> valueChirho)
{-# NOINLINE addThreeChirho #-}
addThreeChirho :: Int -> Int -> Int -> Int
addThreeChirho firstChirho secondChirho thirdChirho = firstChirho + secondChirho + thirdChirho
{-# NOINLINE nestedPartialChirho #-}
nestedPartialChirho :: BoxChirho -> Int -> Int -> Int
nestedPartialChirho boxChirho = case boxChirho of
  BoxChirho savedChirho -> addThreeChirho savedChirho
"#;

const CONSTRUCTOR_MAIN_CHIRHO: &str = r#"
main :: IO ()
main = case makePairChirho 17 of
  PairChirho leftChirho rightChirho -> case burnChirho 4096 of
    0 -> case leftChirho of
      BoxChirho firstChirho -> case rightChirho of
        BoxChirho secondChirho -> print (firstChirho + secondChirho)
    _ -> print (-1 :: Int)
"#;

const CLOSURE_MAIN_CHIRHO: &str = r#"
main :: IO ()
main = let closureChirho = makeClosureChirho (makeBoxChirho 17)
       in closureChirho `seq` case burnChirho 4096 of
         0 -> print (closureChirho 5)
         _ -> print (-1 :: Int)
"#;

const PAP_MAIN_CHIRHO: &str = r#"
main :: IO ()
main = let partialChirho = nestedPartialChirho (makeBoxChirho 17)
           nextChirho = partialChirho 3
       in partialChirho `seq` nextChirho `seq` case burnChirho 4096 of
         0 -> print (nextChirho 4)
         _ -> print (-1 :: Int)
"#;

#[test]
fn returned_constructor_closure_and_nested_pap_survive_collection_chirho() {
    let mut results_chirho = Vec::new();
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        for (case_chirho, main_chirho, expected_chirho) in [
            ("constructor-chirho", CONSTRUCTOR_MAIN_CHIRHO, "35\n"),
            ("closure-chirho", CLOSURE_MAIN_CHIRHO, "22\n"),
            ("nested-pap-chirho", PAP_MAIN_CHIRHO, "24\n"),
        ] {
            let source_chirho = format!("{PRELUDE_CHIRHO}{main_chirho}");
            let result_chirho = measure_native_runtime_chirho(
                &source_chirho,
                backend_chirho,
                Duration::from_secs(30),
            );
            eprintln!("{backend_chirho:?}/{case_chirho}: {result_chirho:#?}");
            results_chirho.push((backend_chirho, case_chirho, expected_chirho, result_chirho));
        }
    }
    let mut failures_chirho = Vec::new();
    for (backend_chirho, case_chirho, expected_chirho, result_chirho) in results_chirho {
        match result_chirho {
            Err(error_chirho) => {
                failures_chirho.push(format!("{backend_chirho:?}/{case_chirho}: {error_chirho}"))
            }
            Ok(measurement_chirho) => {
                if measurement_chirho.exit_code_chirho != 0
                    || measurement_chirho.stdout_chirho != expected_chirho
                {
                    failures_chirho.push(format!("{backend_chirho:?}/{case_chirho}: returned value differs: {measurement_chirho:#?}"));
                }
                if measurement_chirho.counters_chirho.collection_count_chirho < 3
                    || measurement_chirho.counters_chirho.allocated_objects_chirho < 4096
                {
                    failures_chirho.push(format!("{backend_chirho:?}/{case_chirho}: insufficient collection exercise: {measurement_chirho:#?}"));
                }
            }
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}
