// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Boxed application must preserve currying and live captures across real GC.
//! These controls supplement the byte-unchanged original row 704 survival cases.
//! Workflow: testing-chirho/native-root-lifetimes-chirho.md.

use std::time::Duration;

use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, measure_native_runtime_chirho,
};

const PRELUDE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
data BoxChirho = BoxChirho !Int
{-# NOINLINE makeBoxChirho #-}
makeBoxChirho :: Int -> BoxChirho
makeBoxChirho valueChirho = BoxChirho valueChirho
{-# NOINLINE burnChirho #-}
burnChirho :: Int -> Int
burnChirho remainingChirho =
  if remainingChirho == 0 then 0
  else case makeBoxChirho remainingChirho of
    BoxChirho copiedChirho -> burnChirho (copiedChirho - 1)
{-# NOINLINE makeBinaryClosureChirho #-}
makeBinaryClosureChirho :: BoxChirho -> Int -> Int -> Int
makeBinaryClosureChirho boxChirho = case boxChirho of
  BoxChirho savedChirho -> if savedChirho > 0 then (\firstChirho secondChirho -> savedChirho + firstChirho + secondChirho) else (\firstChirho secondChirho -> savedChirho - firstChirho - secondChirho)
{-# NOINLINE addThreeChirho #-}
addThreeChirho :: Int -> Int -> Int -> Int
addThreeChirho firstChirho secondChirho thirdChirho = firstChirho + secondChirho + thirdChirho
{-# NOINLINE nestedPartialChirho #-}
nestedPartialChirho :: BoxChirho -> Int -> Int -> Int
nestedPartialChirho boxChirho = case boxChirho of
  BoxChirho savedChirho -> addThreeChirho savedChirho
{-# NOINLINE makeCapturedPartialChirho #-}
makeCapturedPartialChirho :: BoxChirho -> Int -> Int
makeCapturedPartialChirho boxChirho = case boxChirho of
  BoxChirho savedChirho ->
    let combineChirho firstChirho secondChirho = savedChirho + firstChirho + secondChirho
    in combineChirho 3
"#;

const GROUPED_CLOSURE_MAIN_CHIRHO: &str = r#"
main :: IO ()
main = let closureChirho = makeBinaryClosureChirho (makeBoxChirho 17)
       in closureChirho `seq` case burnChirho 4096 of
         0 -> print (closureChirho 3 4)
         _ -> print (-1 :: Int)
"#;

const STAGED_CLOSURE_MAIN_CHIRHO: &str = r#"
main :: IO ()
main = let closureChirho = makeBinaryClosureChirho (makeBoxChirho 17)
           nextChirho = closureChirho 3
       in closureChirho `seq` nextChirho `seq` case burnChirho 4096 of
         0 -> print (nextChirho 4)
         _ -> print (-1 :: Int)
"#;

const GROUPED_PAP_MAIN_CHIRHO: &str = r#"
main :: IO ()
main = let partialChirho = nestedPartialChirho (makeBoxChirho 17)
       in partialChirho `seq` case burnChirho 4096 of
         0 -> print (partialChirho 3 4)
         _ -> print (-1 :: Int)
"#;

const CAPTURED_PAP_MAIN_CHIRHO: &str = r#"
main :: IO ()
main = let partialChirho = makeCapturedPartialChirho (makeBoxChirho 17)
       in partialChirho `seq` case burnChirho 4096 of
         0 -> print (partialChirho 4)
         _ -> print (-1 :: Int)
"#;

#[test]
fn grouped_and_staged_boxed_applications_preserve_captured_values_across_gc_chirho() {
    let mut results_chirho = Vec::new();
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        for (case_chirho, main_chirho) in [
            (
                "grouped-captured-closure-chirho",
                GROUPED_CLOSURE_MAIN_CHIRHO,
            ),
            ("staged-captured-closure-chirho", STAGED_CLOSURE_MAIN_CHIRHO),
            ("grouped-pap-chirho", GROUPED_PAP_MAIN_CHIRHO),
            ("captured-worker-pap-chirho", CAPTURED_PAP_MAIN_CHIRHO),
        ] {
            let source_chirho = format!("{PRELUDE_CHIRHO}{main_chirho}");
            let result_chirho = measure_native_runtime_chirho(
                &source_chirho,
                backend_chirho,
                Duration::from_secs(30),
            );
            eprintln!("{backend_chirho:?}/{case_chirho}: {result_chirho:#?}");
            results_chirho.push((backend_chirho, case_chirho, result_chirho));
        }
    }
    let mut failures_chirho = Vec::new();
    for (backend_chirho, case_chirho, result_chirho) in results_chirho {
        match result_chirho {
            Err(error_chirho) => {
                failures_chirho.push(format!("{backend_chirho:?}/{case_chirho}: {error_chirho}"));
            }
            Ok(measurement_chirho) => {
                if measurement_chirho.exit_code_chirho != 0
                    || measurement_chirho.stdout_chirho != "24\n"
                {
                    failures_chirho.push(format!("{backend_chirho:?}/{case_chirho}: application result differs: {measurement_chirho:#?}"));
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
