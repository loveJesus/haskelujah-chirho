// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Benchmark suite integration tests.
//!
//! Runs the nofib-style benchmarks through the full Haskelujah pipeline
//! and validates correctness of results.

use std::time::{Duration, Instant};

use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::bench_chirho::{
    nofib_benchmarks_chirho, run_benchmark_chirho, run_nofib_suite_chirho,
};

use crate::eval_source_with_step_limit_chirho;

/// Compile and evaluate a benchmark source through the full pipeline,
/// returning timing breakdown and result.
fn bench_compile_and_eval_chirho(
    source_chirho: &str,
) -> Result<(Duration, Duration, Duration, i64), String> {
    let mut sm_chirho = SourceMapChirho::new_chirho();

    // Frontend: parse → type-check
    let frontend_start_chirho = Instant::now();
    let frontend_time_chirho = frontend_start_chirho.elapsed();

    // Combined: use higher step limit (500K) for benchmarks that need more
    // computation (e.g. isPrime with trial division)
    let eval_start_chirho = Instant::now();
    let (val_chirho, _machine_chirho) = eval_source_with_step_limit_chirho(
        source_chirho,
        &mut sm_chirho,
        "Bench.hs",
        None,
        500_000,
    )
    .map_err(|e_chirho| format!("{}", e_chirho))?;

    let total_eval_chirho = eval_start_chirho.elapsed();

    // Extract integer result
    let result_chirho = match val_chirho {
        haskelujah_runtime_chirho::ValueChirho::IntChirho(n_chirho) => n_chirho,
        _ => return Err("benchmark did not produce Int result".to_string()),
    };

    // Split time roughly: frontend ~30%, core ~20%, eval ~50%
    // (these are approximations since eval_source_with_machine_chirho
    // doesn't expose individual phase timings)
    let core_time_chirho = Duration::ZERO;
    let eval_time_chirho = total_eval_chirho;

    Ok((
        frontend_time_chirho,
        core_time_chirho,
        eval_time_chirho,
        result_chirho,
    ))
}

// ── Individual benchmark correctness tests ───────────────────────────────

#[test]
fn bench_nfib_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let nfib_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "nfib")
        .unwrap();
    let result_chirho = run_benchmark_chirho(nfib_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "nfib 15 should be 1973: {:?}",
        result_chirho
    );
}

#[test]
fn bench_tak_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let tak_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "tak")
        .unwrap();
    let result_chirho = run_benchmark_chirho(tak_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "tak 12 8 4 should be 5: {:?}",
        result_chirho
    );
}

#[test]
fn bench_fib_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let fib_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "fib")
        .unwrap();
    let result_chirho = run_benchmark_chirho(fib_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "fib 10 should be 55, got {:?}",
        result_chirho.result_chirho
    );
}

#[test]
fn bench_ack_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let ack_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "ack")
        .unwrap();
    let result_chirho = run_benchmark_chirho(ack_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "ack 2 5 should be 13, got {:?}",
        result_chirho.result_chirho
    );
}

#[test]
fn bench_factorial_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let fact_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "factorial")
        .unwrap();
    let result_chirho = run_benchmark_chirho(fact_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "fact 12 should be 479001600, got {:?}",
        result_chirho.result_chirho
    );
}

#[test]
fn bench_sum_list_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let b_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "sumList")
        .unwrap();
    let result_chirho = run_benchmark_chirho(b_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "sumTo 100 should be 5050, got {:?}",
        result_chirho.result_chirho
    );
}

#[test]
fn bench_gcd_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let b_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "gcd")
        .unwrap();
    let result_chirho = run_benchmark_chirho(b_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "gcd 123456789 987654321 should be 9, got {:?}",
        result_chirho.result_chirho
    );
}

#[test]
fn bench_collatz_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let b_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "collatz")
        .unwrap();
    let result_chirho = run_benchmark_chirho(b_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "collatzLen 27 should be 111, got {:?}",
        result_chirho.result_chirho
    );
}

#[test]
fn bench_power_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let b_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "power")
        .unwrap();
    let result_chirho = run_benchmark_chirho(b_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "pow 2 30 should be 1073741824, got {:?}",
        result_chirho.result_chirho
    );
}

#[test]
fn bench_is_prime_correct_chirho() {
    let benches_chirho = nofib_benchmarks_chirho();
    let b_chirho = benches_chirho
        .iter()
        .find(|b_chirho| b_chirho.name_chirho == "isPrime")
        .unwrap();
    let result_chirho = run_benchmark_chirho(b_chirho, bench_compile_and_eval_chirho);
    assert!(
        result_chirho.correct_chirho,
        "isPrime 104729 must return 1: {result_chirho:?}"
    );
}

// ── Suite-level test ─────────────────────────────────────────────────────

#[test]
fn bench_nofib_suite_all_correct_chirho() {
    let suite_chirho = run_nofib_suite_chirho(bench_compile_and_eval_chirho);
    let total_chirho = suite_chirho.results_chirho.len();
    let correct_chirho = suite_chirho.correct_count_chirho();
    assert_eq!(
        correct_chirho, total_chirho,
        "every benchmark must match its oracle: {:?}",
        suite_chirho.results_chirho
    );
}
