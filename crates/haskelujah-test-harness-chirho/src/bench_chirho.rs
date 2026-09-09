// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Benchmark suite (nofib-style)
//!
//! Provides infrastructure for timing Haskelujah compiler pipeline stages
//! and STG evaluator execution. Modeled after GHC's nofib benchmark suite.
//!
//! Each benchmark is a Haskell source snippet that exercises a specific
//! workload (arithmetic, recursion, list operations, pattern matching, etc).
//! The harness measures wall-clock time for parsing, type checking, desugaring,
//! and evaluation, then reports results in a structured format.

use std::time::{Duration, Instant};

/// A single benchmark definition.
#[derive(Debug, Clone)]
pub struct BenchDefChirho {
    /// Human-readable benchmark name.
    pub name_chirho: String,
    /// Category (e.g. "imaginary", "spectral", "real").
    pub category_chirho: String,
    /// Haskell source code.
    pub source_chirho: String,
    /// Expected result as Int (None to skip result validation).
    pub expected_chirho: Option<i64>,
}

/// Timing results for one benchmark run.
#[derive(Debug, Clone)]
pub struct BenchResultChirho {
    /// Benchmark name.
    pub name_chirho: String,
    /// Category.
    pub category_chirho: String,
    /// Total wall-clock time.
    pub total_time_chirho: Duration,
    /// Parse + type-check time.
    pub frontend_time_chirho: Duration,
    /// Desugaring + Core simplification time.
    pub core_time_chirho: Duration,
    /// STG evaluation time.
    pub eval_time_chirho: Duration,
    /// Whether the result matched the expected value.
    pub correct_chirho: bool,
    /// The actual Int result (if available).
    pub result_chirho: Option<i64>,
    /// Evaluation failure, retained so a red benchmark reports its actual cause.
    pub error_chirho: Option<String>,
}

/// Summary of a benchmark suite run.
#[derive(Debug, Clone)]
pub struct BenchSuiteResultChirho {
    /// Individual benchmark results.
    pub results_chirho: Vec<BenchResultChirho>,
    /// Total wall-clock time for all benchmarks.
    pub total_time_chirho: Duration,
}

impl BenchSuiteResultChirho {
    /// Number of benchmarks that produced correct results.
    pub fn correct_count_chirho(&self) -> usize {
        self.results_chirho
            .iter()
            .filter(|r_chirho| r_chirho.correct_chirho)
            .count()
    }

    /// Format a human-readable summary table.
    pub fn summary_table_chirho(&self) -> String {
        let mut lines_chirho = Vec::new();
        lines_chirho.push(format!(
            "{:<25} {:>10} {:>10} {:>10} {:>10} {:>7}",
            "Benchmark", "Frontend", "Core", "Eval", "Total", "OK?"
        ));
        lines_chirho.push("-".repeat(80));
        for r_chirho in &self.results_chirho {
            lines_chirho.push(format!(
                "{:<25} {:>10} {:>10} {:>10} {:>10} {:>7}",
                r_chirho.name_chirho,
                format_duration_chirho(r_chirho.frontend_time_chirho),
                format_duration_chirho(r_chirho.core_time_chirho),
                format_duration_chirho(r_chirho.eval_time_chirho),
                format_duration_chirho(r_chirho.total_time_chirho),
                if r_chirho.correct_chirho { "yes" } else { "NO" },
            ));
        }
        lines_chirho.push("-".repeat(80));
        lines_chirho.push(format!(
            "{}/{} correct, total: {}",
            self.correct_count_chirho(),
            self.results_chirho.len(),
            format_duration_chirho(self.total_time_chirho),
        ));
        lines_chirho.join("\n")
    }
}

/// Format a Duration as human-readable (e.g. "1.23ms", "456us").
fn format_duration_chirho(d_chirho: Duration) -> String {
    let us_chirho = d_chirho.as_micros();
    if us_chirho < 1000 {
        format!("{}us", us_chirho)
    } else if us_chirho < 1_000_000 {
        format!("{:.2}ms", us_chirho as f64 / 1000.0)
    } else {
        format!("{:.2}s", d_chirho.as_secs_f64())
    }
}

/// The curated nofib-style benchmark set.
///
/// Categories follow GHC nofib:
/// - "imaginary": small synthetic workloads
/// - "spectral": algorithmic benchmarks
pub fn nofib_benchmarks_chirho() -> Vec<BenchDefChirho> {
    vec![
        // ── imaginary ────────────────────────────────────────────────────
        BenchDefChirho {
            name_chirho: "nfib".to_string(),
            category_chirho: "imaginary".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "nfib n = if n <= 1 then 1 else nfib (n-1) + nfib (n-2) + 1\n",
                "main = nfib 15\n",
            )
            .to_string(),
            expected_chirho: Some(1973),
        },
        BenchDefChirho {
            name_chirho: "tak".to_string(),
            category_chirho: "imaginary".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "tak x y z = if y >= x then z\n",
                "            else tak (tak (x-1) y z) (tak (y-1) z x) (tak (z-1) x y)\n",
                "main = tak 12 8 4\n",
            )
            .to_string(),
            expected_chirho: Some(5),
        },
        BenchDefChirho {
            name_chirho: "fib".to_string(),
            category_chirho: "imaginary".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "fib n = if n == 0 then 0 else if n == 1 then 1 else fib (n-1) + fib (n-2)\n",
                "main = fib 10\n",
            )
            .to_string(),
            expected_chirho: Some(55),
        },
        BenchDefChirho {
            name_chirho: "ack".to_string(),
            category_chirho: "imaginary".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "ack m n = if m == 0 then n + 1\n",
                "          else if n == 0 then ack (m-1) 1\n",
                "          else ack (m-1) (ack m (n-1))\n",
                "main = ack 2 5\n",
            )
            .to_string(),
            expected_chirho: Some(13),
        },
        BenchDefChirho {
            name_chirho: "factorial".to_string(),
            category_chirho: "imaginary".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "fact 0 = 1\n",
                "fact n = n * fact (n-1)\n",
                "main = fact 12\n",
            )
            .to_string(),
            expected_chirho: Some(479001600),
        },
        // ── spectral ─────────────────────────────────────────────────────
        BenchDefChirho {
            name_chirho: "sumList".to_string(),
            category_chirho: "spectral".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "sumTo 0 = 0\n",
                "sumTo n = n + sumTo (n-1)\n",
                "main = sumTo 100\n",
            )
            .to_string(),
            expected_chirho: Some(5050),
        },
        BenchDefChirho {
            name_chirho: "gcd".to_string(),
            category_chirho: "spectral".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "myGcd a 0 = a\n",
                "myGcd a b = myGcd b (a `mod` b)\n",
                "main = myGcd 123456789 987654321\n",
            )
            .to_string(),
            expected_chirho: Some(9),
        },
        BenchDefChirho {
            name_chirho: "collatz".to_string(),
            category_chirho: "spectral".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "collatzLen 1 = 0\n",
                "collatzLen n = if n `mod` 2 == 0\n",
                "               then 1 + collatzLen (n `div` 2)\n",
                "               else 1 + collatzLen (3 * n + 1)\n",
                "main = collatzLen 27\n",
            )
            .to_string(),
            expected_chirho: Some(111),
        },
        BenchDefChirho {
            name_chirho: "power".to_string(),
            category_chirho: "spectral".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "pow b 0 = 1\n",
                "pow b e = if e `mod` 2 == 0\n",
                "          then pow (b*b) (e `div` 2)\n",
                "          else b * pow b (e-1)\n",
                "main = pow 2 30\n",
            )
            .to_string(),
            expected_chirho: Some(1073741824),
        },
        BenchDefChirho {
            name_chirho: "isPrime".to_string(),
            category_chirho: "spectral".to_string(),
            source_chirho: concat!(
                "module Bench where\n",
                "isqrt n = isqrtHelper 0 n n\n",
                "isqrtHelper lo hi n = if lo >= hi then lo\n",
                "                      else let mid = (lo + hi + 1) `div` 2\n",
                "                           in if mid * mid > n\n",
                "                              then isqrtHelper lo (mid - 1) n\n",
                "                              else isqrtHelper mid hi n\n",
                "checkPrime n 1 = 1\n",
                "checkPrime n d = if n `mod` d == 0 then 0 else checkPrime n (d-1)\n",
                "isPrime n = if n < 2 then 0 else checkPrime n (isqrt n)\n",
                "main = isPrime 104729\n",
            )
            .to_string(),
            expected_chirho: Some(1),
        },
    ]
}

/// Run a single benchmark, measuring frontend/core/eval times separately.
///
/// `compile_and_eval_fn_chirho` should accept source code and return
/// `(frontend_time, core_time, eval_time, result_value)`.
pub fn run_benchmark_chirho<F>(
    bench_chirho: &BenchDefChirho,
    compile_and_eval_fn_chirho: F,
) -> BenchResultChirho
where
    F: FnOnce(&str) -> Result<(Duration, Duration, Duration, i64), String>,
{
    let start_chirho = Instant::now();
    match compile_and_eval_fn_chirho(&bench_chirho.source_chirho) {
        Ok((frontend_chirho, core_chirho, eval_chirho, result_chirho)) => {
            let total_chirho = start_chirho.elapsed();
            let correct_chirho = bench_chirho
                .expected_chirho
                .map_or(true, |e_chirho| e_chirho == result_chirho);
            BenchResultChirho {
                name_chirho: bench_chirho.name_chirho.clone(),
                category_chirho: bench_chirho.category_chirho.clone(),
                total_time_chirho: total_chirho,
                frontend_time_chirho: frontend_chirho,
                core_time_chirho: core_chirho,
                eval_time_chirho: eval_chirho,
                correct_chirho,
                result_chirho: Some(result_chirho),
                error_chirho: None,
            }
        }
        Err(error_chirho) => {
            let total_chirho = start_chirho.elapsed();
            BenchResultChirho {
                name_chirho: bench_chirho.name_chirho.clone(),
                category_chirho: bench_chirho.category_chirho.clone(),
                total_time_chirho: total_chirho,
                frontend_time_chirho: Duration::ZERO,
                core_time_chirho: Duration::ZERO,
                eval_time_chirho: Duration::ZERO,
                correct_chirho: false,
                result_chirho: None,
                error_chirho: Some(error_chirho),
            }
        }
    }
}

/// Run all benchmarks in the nofib suite.
pub fn run_nofib_suite_chirho<F>(compile_and_eval_fn_chirho: F) -> BenchSuiteResultChirho
where
    F: Fn(&str) -> Result<(Duration, Duration, Duration, i64), String>,
{
    let start_chirho = Instant::now();
    let benchmarks_chirho = nofib_benchmarks_chirho();
    let results_chirho: Vec<BenchResultChirho> = benchmarks_chirho
        .iter()
        .map(|b_chirho| {
            run_benchmark_chirho(b_chirho, |s_chirho| compile_and_eval_fn_chirho(s_chirho))
        })
        .collect();
    let total_chirho = start_chirho.elapsed();
    BenchSuiteResultChirho {
        results_chirho,
        total_time_chirho: total_chirho,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn nofib_benchmarks_not_empty_chirho() {
        let benches_chirho = nofib_benchmarks_chirho();
        assert!(
            benches_chirho.len() >= 10,
            "should have at least 10 benchmarks, got {}",
            benches_chirho.len()
        );
    }

    #[test]
    fn bench_result_summary_table_chirho() {
        let suite_chirho = BenchSuiteResultChirho {
            results_chirho: vec![BenchResultChirho {
                name_chirho: "test".to_string(),
                category_chirho: "imaginary".to_string(),
                total_time_chirho: Duration::from_millis(42),
                frontend_time_chirho: Duration::from_millis(10),
                core_time_chirho: Duration::from_millis(5),
                eval_time_chirho: Duration::from_millis(27),
                correct_chirho: true,
                result_chirho: Some(42),
                error_chirho: None,
            }],
            total_time_chirho: Duration::from_millis(42),
        };
        let table_chirho = suite_chirho.summary_table_chirho();
        assert!(table_chirho.contains("test"));
        assert!(table_chirho.contains("1/1 correct"));
    }

    #[test]
    fn format_duration_microseconds_chirho() {
        assert_eq!(format_duration_chirho(Duration::from_micros(500)), "500us");
    }

    #[test]
    fn format_duration_milliseconds_chirho() {
        let result_chirho = format_duration_chirho(Duration::from_micros(1500));
        assert!(result_chirho.contains("ms"));
    }

    #[test]
    fn format_duration_seconds_chirho() {
        let result_chirho = format_duration_chirho(Duration::from_secs(2));
        assert!(result_chirho.contains("s"));
    }

    #[test]
    fn run_benchmark_failure_path_chirho() {
        let bench_chirho = BenchDefChirho {
            name_chirho: "fail".to_string(),
            category_chirho: "test".to_string(),
            source_chirho: "".to_string(),
            expected_chirho: Some(42),
        };
        let result_chirho =
            run_benchmark_chirho(&bench_chirho, |_| Err("compilation failed".to_string()));
        assert!(!result_chirho.correct_chirho);
        assert!(result_chirho.result_chirho.is_none());
        assert_eq!(
            result_chirho.error_chirho.as_deref(),
            Some("compilation failed")
        );
    }

    #[test]
    fn run_benchmark_success_path_chirho() {
        let bench_chirho = BenchDefChirho {
            name_chirho: "ok".to_string(),
            category_chirho: "test".to_string(),
            source_chirho: "".to_string(),
            expected_chirho: Some(42),
        };
        let result_chirho = run_benchmark_chirho(&bench_chirho, |_| {
            Ok((
                Duration::from_millis(1),
                Duration::from_millis(1),
                Duration::from_millis(1),
                42,
            ))
        });
        assert!(result_chirho.correct_chirho);
        assert_eq!(result_chirho.result_chirho, Some(42));
    }

    #[test]
    fn correct_count_chirho() {
        let suite_chirho = BenchSuiteResultChirho {
            results_chirho: vec![
                BenchResultChirho {
                    name_chirho: "a".to_string(),
                    category_chirho: "t".to_string(),
                    total_time_chirho: Duration::ZERO,
                    frontend_time_chirho: Duration::ZERO,
                    core_time_chirho: Duration::ZERO,
                    eval_time_chirho: Duration::ZERO,
                    correct_chirho: true,
                    result_chirho: Some(1),
                    error_chirho: None,
                },
                BenchResultChirho {
                    name_chirho: "b".to_string(),
                    category_chirho: "t".to_string(),
                    total_time_chirho: Duration::ZERO,
                    frontend_time_chirho: Duration::ZERO,
                    core_time_chirho: Duration::ZERO,
                    eval_time_chirho: Duration::ZERO,
                    correct_chirho: false,
                    result_chirho: None,
                    error_chirho: Some("evaluation failed".to_string()),
                },
            ],
            total_time_chirho: Duration::ZERO,
        };
        assert_eq!(suite_chirho.correct_count_chirho(), 1);
    }
}
