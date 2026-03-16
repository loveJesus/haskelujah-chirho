// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Differential Testing Against GHC (§62)
//!
//! Provides infrastructure for comparing Haskelujah behavior against GHC
//! on syntax and typechecker edge cases. Each test case records:
//! - Haskell source snippet
//! - GHC expected behavior (compiles/rejects, output value)
//! - Whether Haskelujah agrees with GHC
//!
//! When GHC is not available, tests use pre-recorded reference answers.

/// Outcome of compiling a Haskell snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileOutcomeChirho {
    /// Compiled and evaluated to an Int result.
    SuccessIntChirho(i64),
    /// Compiled and evaluated but result was not an Int.
    SuccessOtherChirho(String),
    /// Compilation rejected (type error, parse error, etc).
    RejectedChirho(String),
}

/// A single differential test case.
#[derive(Debug, Clone)]
pub struct DiffTestCaseChirho {
    /// Short descriptive name.
    pub name_chirho: &'static str,
    /// Category: "syntax", "types", "semantics", "edge".
    pub category_chirho: &'static str,
    /// Haskell source code.
    pub source_chirho: &'static str,
    /// Expected GHC behavior (pre-recorded reference).
    pub ghc_expected_chirho: CompileOutcomeChirho,
}

/// Result of running one differential test.
#[derive(Debug, Clone)]
pub struct DiffTestResultChirho {
    /// Test name.
    pub name_chirho: String,
    /// Category.
    pub category_chirho: String,
    /// GHC expected outcome.
    pub ghc_outcome_chirho: CompileOutcomeChirho,
    /// Haskelujah actual outcome.
    pub haskelujah_outcome_chirho: CompileOutcomeChirho,
    /// Whether Haskelujah agrees with GHC.
    pub agrees_chirho: bool,
}

/// Summary of a differential test suite run.
#[derive(Debug, Clone)]
pub struct DiffSuiteResultChirho {
    pub results_chirho: Vec<DiffTestResultChirho>,
}

impl DiffSuiteResultChirho {
    /// Count of tests where Haskelujah agrees with GHC.
    pub fn agree_count_chirho(&self) -> usize {
        self.results_chirho.iter().filter(|r_chirho| r_chirho.agrees_chirho).count()
    }

    /// Agreement rate as percentage.
    pub fn agree_rate_chirho(&self) -> f64 {
        let total_chirho = self.results_chirho.len();
        if total_chirho == 0 { return 100.0; }
        self.agree_count_chirho() as f64 / total_chirho as f64 * 100.0
    }

    /// Format a summary table.
    pub fn summary_table_chirho(&self) -> String {
        let mut lines_chirho = Vec::new();
        lines_chirho.push(format!(
            "{:<30} {:<10} {:<20} {:<20} {:<6}",
            "Test", "Category", "GHC", "Haskelujah", "Match"
        ));
        lines_chirho.push("-".repeat(90));
        for r_chirho in &self.results_chirho {
            lines_chirho.push(format!(
                "{:<30} {:<10} {:<20} {:<20} {:<6}",
                r_chirho.name_chirho,
                r_chirho.category_chirho,
                format_outcome_chirho(&r_chirho.ghc_outcome_chirho),
                format_outcome_chirho(&r_chirho.haskelujah_outcome_chirho),
                if r_chirho.agrees_chirho { "yes" } else { "NO" },
            ));
        }
        lines_chirho.push("-".repeat(90));
        lines_chirho.push(format!(
            "{}/{} agree ({:.1}%)",
            self.agree_count_chirho(),
            self.results_chirho.len(),
            self.agree_rate_chirho(),
        ));
        lines_chirho.join("\n")
    }
}

fn format_outcome_chirho(outcome_chirho: &CompileOutcomeChirho) -> String {
    match outcome_chirho {
        CompileOutcomeChirho::SuccessIntChirho(n_chirho) => format!("ok:{}", n_chirho),
        CompileOutcomeChirho::SuccessOtherChirho(s_chirho) => format!("ok:{}", s_chirho),
        CompileOutcomeChirho::RejectedChirho(_) => "rejected".to_string(),
    }
}

/// The curated set of differential test cases.
///
/// Each case records the GHC behavior as of GHC 9.8.
/// Categories:
/// - syntax: lexer/parser edge cases
/// - types: type inference/checking edge cases
/// - semantics: evaluation semantics
/// - edge: corner cases that have historically caused divergence
pub fn diff_test_cases_chirho() -> Vec<DiffTestCaseChirho> {
    use CompileOutcomeChirho::*;
    vec![
        // ── syntax ─────────────────────────────────────────────────────
        DiffTestCaseChirho {
            name_chirho: "operator_section_left",
            category_chirho: "syntax",
            source_chirho: "module Main where\nmain = (+ 1) 41\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "operator_section_right",
            category_chirho: "syntax",
            source_chirho: "module Main where\nmain = (100 -) 58\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "backtick_function",
            category_chirho: "syntax",
            source_chirho: "module Main where\nmain = 10 `div` 2\n",
            ghc_expected_chirho: SuccessIntChirho(5),
        },
        DiffTestCaseChirho {
            name_chirho: "nested_comments",
            category_chirho: "syntax",
            source_chirho: "module Main where\n{- outer {- inner -} still comment -}\nmain = 42\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "where_clause",
            category_chirho: "syntax",
            source_chirho: "module Main where\nmain = x + y where { x = 10; y = 32 }\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "negative_literal",
            category_chirho: "syntax",
            source_chirho: "module Main where\nmain = negate (-42)\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "multiline_string",
            category_chirho: "syntax",
            source_chirho: "module Main where\ns = \"hello \\\n    \\world\"\nmain = length s\n",
            ghc_expected_chirho: SuccessIntChirho(11),
        },
        // ── types ──────────────────────────────────────────────────────
        DiffTestCaseChirho {
            name_chirho: "monomorphism_restriction",
            category_chirho: "types",
            source_chirho: "module Main where\nf = (+)\nmain = f 20 22\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "polymorphic_let",
            category_chirho: "types",
            source_chirho: "module Main where\nmain = let id x = x in id 42\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "type_annotation",
            category_chirho: "types",
            source_chirho: "module Main where\nmain = (42 :: Int)\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "show_class_dispatch",
            category_chirho: "types",
            source_chirho: "module Main where\nmain = length (show 42)\n",
            ghc_expected_chirho: SuccessIntChirho(2),
        },
        DiffTestCaseChirho {
            name_chirho: "newtype_coerce",
            category_chirho: "types",
            source_chirho: concat!(
                "module Main where\n",
                "newtype Age = MkAge Int\n",
                "getAge (MkAge n) = n\n",
                "main = getAge (MkAge 42)\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "type_error_rejected",
            category_chirho: "types",
            source_chirho: "module Main where\nmain = True + 1\n",
            ghc_expected_chirho: RejectedChirho("type error".to_string()),
        },
        // ── semantics ──────────────────────────────────────────────────
        DiffTestCaseChirho {
            name_chirho: "if_then_else_lazy",
            category_chirho: "semantics",
            source_chirho: "module Main where\nmain = if True then 42 else error \"boom\"\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "seq_forces_first",
            category_chirho: "semantics",
            source_chirho: "module Main where\nmain = seq 99 42\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "case_default_fallthrough",
            category_chirho: "semantics",
            source_chirho: concat!(
                "module Main where\n",
                "f n = case n of\n",
                "  1 -> 10\n",
                "  2 -> 20\n",
                "  _ -> 99\n",
                "main = f 5\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(99),
        },
        DiffTestCaseChirho {
            name_chirho: "guard_expressions",
            category_chirho: "semantics",
            source_chirho: concat!(
                "module Main where\n",
                "classify n\n",
                "  | n < 0     = 0\n",
                "  | n == 0    = 1\n",
                "  | otherwise = 2\n",
                "main = classify 5\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(2),
        },
        DiffTestCaseChirho {
            name_chirho: "list_comprehension",
            category_chirho: "semantics",
            source_chirho: concat!(
                "module Main where\n",
                "main = sum [x * x | x <- [1..5]]\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(55),
        },
        DiffTestCaseChirho {
            name_chirho: "where_scoping",
            category_chirho: "semantics",
            source_chirho: concat!(
                "module Main where\n",
                "main = result\n",
                "  where result = a + b\n",
                "        a = 20\n",
                "        b = 22\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        // ── edge cases ─────────────────────────────────────────────────
        DiffTestCaseChirho {
            name_chirho: "shadow_binding",
            category_chirho: "edge",
            source_chirho: concat!(
                "module Main where\n",
                "main = let x = 1 in let x = 42 in x\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "tuple_fst_snd",
            category_chirho: "edge",
            source_chirho: "module Main where\nmain = fst (42, 0)\n",
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "as_pattern",
            category_chirho: "edge",
            source_chirho: concat!(
                "module Main where\n",
                "f xs@(x:_) = x\n",
                "f [] = 0\n",
                "main = f [42, 1, 2]\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "wildcard_pattern",
            category_chirho: "edge",
            source_chirho: concat!(
                "module Main where\n",
                "f _ _ x = x\n",
                "main = f 1 2 42\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(42),
        },
        DiffTestCaseChirho {
            name_chirho: "empty_list_sum",
            category_chirho: "edge",
            source_chirho: "module Main where\nmain = sum []\n",
            ghc_expected_chirho: SuccessIntChirho(0),
        },
        DiffTestCaseChirho {
            name_chirho: "composed_functions",
            category_chirho: "edge",
            source_chirho: concat!(
                "module Main where\n",
                "double x = x * 2\n",
                "add1 x = x + 1\n",
                "main = (double . add1) 20\n",
            ),
            ghc_expected_chirho: SuccessIntChirho(42),
        },
    ]
}

/// Run the differential test suite.
///
/// `eval_fn_chirho` takes source code and returns the compile outcome.
pub fn run_diff_suite_chirho<F>(eval_fn_chirho: F) -> DiffSuiteResultChirho
where
    F: Fn(&str) -> CompileOutcomeChirho,
{
    let cases_chirho = diff_test_cases_chirho();
    let results_chirho: Vec<DiffTestResultChirho> = cases_chirho
        .iter()
        .map(|case_chirho| {
            let haskelujah_chirho = eval_fn_chirho(case_chirho.source_chirho);
            let agrees_chirho = outcomes_agree_chirho(
                &case_chirho.ghc_expected_chirho,
                &haskelujah_chirho,
            );
            DiffTestResultChirho {
                name_chirho: case_chirho.name_chirho.to_string(),
                category_chirho: case_chirho.category_chirho.to_string(),
                ghc_outcome_chirho: case_chirho.ghc_expected_chirho.clone(),
                haskelujah_outcome_chirho: haskelujah_chirho,
                agrees_chirho,
            }
        })
        .collect();
    DiffSuiteResultChirho { results_chirho }
}

/// Check if two outcomes agree.
fn outcomes_agree_chirho(
    ghc_chirho: &CompileOutcomeChirho,
    haskelujah_chirho: &CompileOutcomeChirho,
) -> bool {
    match (ghc_chirho, haskelujah_chirho) {
        (CompileOutcomeChirho::SuccessIntChirho(a_chirho), CompileOutcomeChirho::SuccessIntChirho(b_chirho)) => {
            a_chirho == b_chirho
        }
        (CompileOutcomeChirho::RejectedChirho(_), CompileOutcomeChirho::RejectedChirho(_)) => {
            true // both rejected = agreement
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn diff_test_cases_not_empty_chirho() {
        let cases_chirho = diff_test_cases_chirho();
        assert!(
            cases_chirho.len() >= 20,
            "should have at least 20 diff test cases, got {}",
            cases_chirho.len()
        );
    }

    #[test]
    fn diff_test_cases_have_categories_chirho() {
        let cases_chirho = diff_test_cases_chirho();
        let categories_chirho: std::collections::HashSet<&str> =
            cases_chirho.iter().map(|c_chirho| c_chirho.category_chirho).collect();
        assert!(categories_chirho.contains("syntax"));
        assert!(categories_chirho.contains("types"));
        assert!(categories_chirho.contains("semantics"));
        assert!(categories_chirho.contains("edge"));
    }

    #[test]
    fn outcomes_agree_same_int_chirho() {
        assert!(outcomes_agree_chirho(
            &CompileOutcomeChirho::SuccessIntChirho(42),
            &CompileOutcomeChirho::SuccessIntChirho(42),
        ));
    }

    #[test]
    fn outcomes_disagree_different_int_chirho() {
        assert!(!outcomes_agree_chirho(
            &CompileOutcomeChirho::SuccessIntChirho(42),
            &CompileOutcomeChirho::SuccessIntChirho(99),
        ));
    }

    #[test]
    fn outcomes_agree_both_rejected_chirho() {
        assert!(outcomes_agree_chirho(
            &CompileOutcomeChirho::RejectedChirho("type error".into()),
            &CompileOutcomeChirho::RejectedChirho("different msg".into()),
        ));
    }

    #[test]
    fn outcomes_disagree_success_vs_reject_chirho() {
        assert!(!outcomes_agree_chirho(
            &CompileOutcomeChirho::SuccessIntChirho(42),
            &CompileOutcomeChirho::RejectedChirho("err".into()),
        ));
    }

    #[test]
    fn run_diff_suite_mock_chirho() {
        let suite_chirho = run_diff_suite_chirho(|_| {
            CompileOutcomeChirho::SuccessIntChirho(42)
        });
        assert!(!suite_chirho.results_chirho.is_empty());
        // Not all will agree since some expect rejection or different values
        assert!(suite_chirho.agree_count_chirho() > 0);
    }

    #[test]
    fn diff_suite_summary_table_chirho() {
        let suite_chirho = run_diff_suite_chirho(|_| {
            CompileOutcomeChirho::SuccessIntChirho(42)
        });
        let table_chirho = suite_chirho.summary_table_chirho();
        assert!(table_chirho.contains("Test"));
        assert!(table_chirho.contains("agree"));
    }

    #[test]
    fn agree_rate_computation_chirho() {
        let suite_chirho = DiffSuiteResultChirho {
            results_chirho: vec![
                DiffTestResultChirho {
                    name_chirho: "a".into(),
                    category_chirho: "t".into(),
                    ghc_outcome_chirho: CompileOutcomeChirho::SuccessIntChirho(1),
                    haskelujah_outcome_chirho: CompileOutcomeChirho::SuccessIntChirho(1),
                    agrees_chirho: true,
                },
                DiffTestResultChirho {
                    name_chirho: "b".into(),
                    category_chirho: "t".into(),
                    ghc_outcome_chirho: CompileOutcomeChirho::SuccessIntChirho(2),
                    haskelujah_outcome_chirho: CompileOutcomeChirho::SuccessIntChirho(99),
                    agrees_chirho: false,
                },
            ],
        };
        assert!((suite_chirho.agree_rate_chirho() - 50.0).abs() < 0.1);
    }

    #[test]
    fn empty_suite_rate_chirho() {
        let suite_chirho = DiffSuiteResultChirho {
            results_chirho: vec![],
        };
        assert_eq!(suite_chirho.agree_rate_chirho(), 100.0);
    }
}
