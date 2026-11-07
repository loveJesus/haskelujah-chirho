// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A fixed semantic live set must not retain a self-tail loop's discarded objects.
//! Proposed expected RED until scoped native roots land; never ignored or waived.
//! Workflow: testing-chirho/native-runtime-measurement-chirho.md.

use std::time::Duration;

use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, NativeRuntimeMeasurementChirho, measure_native_runtime_chirho,
};

const SMALL_ITERATIONS_CHIRHO: u64 = 4096;
const LARGE_ITERATIONS_CHIRHO: u64 = 4 * SMALL_ITERATIONS_CHIRHO;

const SOURCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
data ParcelChirho = ParcelChirho !Int !Int
{-# NOINLINE makeParcelChirho #-}
makeParcelChirho :: Int -> ParcelChirho
makeParcelChirho valueChirho = ParcelChirho valueChirho (valueChirho + 1)
{-# NOINLINE loopChirho #-}
loopChirho :: Int -> ParcelChirho -> Int
loopChirho remainingChirho sentinelChirho =
  case sentinelChirho of
    ParcelChirho markerChirho witnessChirho ->
      if remainingChirho == 0
      then markerChirho + witnessChirho
      else case makeParcelChirho remainingChirho of
        ParcelChirho copiedChirho neighborChirho ->
          if copiedChirho == remainingChirho && neighborChirho == remainingChirho + 1 && markerChirho == 17 && witnessChirho == 18
          then loopChirho (remainingChirho - 1) sentinelChirho
          else -1
main :: IO ()
main = print (loopChirho ITERATIONS_CHIRHO (makeParcelChirho 17))
"#;

#[test]
fn native_self_tail_loop_retires_dead_objects_with_a_live_sentinel_chirho() {
    // Complete both scales on both true backends before judging any failure.
    let mut observations_chirho = Vec::new();
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        for iterations_chirho in [SMALL_ITERATIONS_CHIRHO, LARGE_ITERATIONS_CHIRHO] {
            let source_chirho =
                SOURCE_CHIRHO.replace("ITERATIONS_CHIRHO", &iterations_chirho.to_string());
            let result_chirho = measure_native_runtime_chirho(
                &source_chirho,
                backend_chirho,
                Duration::from_secs(30),
            );
            eprintln!("{backend_chirho:?}, {iterations_chirho} iterations: {result_chirho:#?}");
            observations_chirho.push((backend_chirho, iterations_chirho, result_chirho));
        }
    }
    let mut failures_chirho = Vec::new();
    for (backend_chirho, iterations_chirho, result_chirho) in &observations_chirho {
        match result_chirho {
            Err(error_chirho) => failures_chirho.push(format!(
                "{backend_chirho:?}/{iterations_chirho}: {error_chirho}"
            )),
            Ok(measurement_chirho) => {
                let counters_chirho = &measurement_chirho.counters_chirho;
                if measurement_chirho.exit_code_chirho != 0
                    || measurement_chirho.stdout_chirho != "35\n"
                {
                    failures_chirho.push(format!("{backend_chirho:?}/{iterations_chirho}: sentinel result differs: {measurement_chirho:#?}"));
                }
                // Observe real work and collections; an unexercised envelope cannot pass.
                if counters_chirho.allocated_objects_chirho < *iterations_chirho
                    || counters_chirho.allocated_bytes_chirho < 24 * *iterations_chirho
                    || counters_chirho.collection_count_chirho < 3
                {
                    failures_chirho.push(format!("{backend_chirho:?}/{iterations_chirho}: insufficient allocation/collection exercise: {counters_chirho:#?}"));
                }
            }
        }
    }
    for pair_chirho in observations_chirho.chunks_exact(2) {
        if let (Ok(small_chirho), Ok(large_chirho)) = (&pair_chirho[0].2, &pair_chirho[1].2) {
            check_envelope_chirho(small_chirho, large_chirho, &mut failures_chirho);
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}

fn check_envelope_chirho(
    small_chirho: &NativeRuntimeMeasurementChirho,
    large_chirho: &NativeRuntimeMeasurementChirho,
    failures_chirho: &mut Vec<String>,
) {
    let small_counters_chirho = &small_chirho.counters_chirho;
    let large_counters_chirho = &large_chirho.counters_chirho;
    // Coarse fixed slack permits backend bookkeeping, not N-dependent retention.
    // These are semantic growth envelopes, not exact incidental root counts.
    for (name_chirho, small_value_chirho, large_value_chirho, slack_chirho) in [
        (
            "peak root depth",
            small_counters_chirho.root_depth_high_water_chirho,
            large_counters_chirho.root_depth_high_water_chirho,
            256,
        ),
        (
            "peak post-GC live objects",
            small_counters_chirho.live_objects_after_gc_high_water_chirho,
            large_counters_chirho.live_objects_after_gc_high_water_chirho,
            256,
        ),
        (
            "peak post-GC live bytes",
            small_counters_chirho.live_bytes_after_gc_high_water_chirho,
            large_counters_chirho.live_bytes_after_gc_high_water_chirho,
            16 * 1024,
        ),
    ] {
        if large_value_chirho > small_value_chirho.saturating_add(slack_chirho) {
            failures_chirho.push(format!("{name_chirho} grows with discarded iterations: {small_value_chirho} -> {large_value_chirho}; small evidence {}, large evidence {}", small_chirho.artifact_directory_chirho.display(), large_chirho.artifact_directory_chirho.display()));
        }
    }
}
