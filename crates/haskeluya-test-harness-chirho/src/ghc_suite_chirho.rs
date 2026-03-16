// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # GHC test suite integration
//!
//! Runner for GHC test cases — both the complete GHC testsuite from GitLab
//! and curated tests in `ghc-tests-chirho/`.
//!
//! ## Modes
//!
//! 1. **Full GHC testsuite**: Clone from `gitlab.haskell.org/ghc/ghc`, parse
//!    `.T` driver files, run matching `.hs` files through Haskeluya, track pass rate.
//!
//! 2. **Curated tests**: Hand-written `.hs` files in `ghc-tests-chirho/` with
//!    metadata comments. Good for regression testing known-working features.
//!
//! ## Test file format (curated)
//!
//! ```haskell
//! -- TEST: compile
//! module T001 where ...
//! ```
//!
//! Or for compile-and-run tests:
//! ```haskell
//! -- TEST: compile_and_run
//! -- EXPECT_OUTPUT: Hello
//! main = putStrLn "Hello"
//! ```
//!
//! Or for expected-failure tests:
//! ```haskell
//! -- TEST: compile_fail
//! module T002 where
//! x = True + 1  -- type error
//! ```
//!
//! ## GHC `.T` file format (full suite)
//!
//! GHC's testsuite uses Python-like `.T` driver files:
//! ```python
//! test('T1234', normal, compile, [''])
//! test('T5678', normal, compile_and_run, [''])
//! test('T9999', normal, compile_fail, [''])
//! ```

use std::fs;
use std::path::{Path, PathBuf};

/// The kind of test: what we expect to happen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhcTestKindChirho {
    /// Source should compile successfully.
    CompileChirho,
    /// Source should compile and produce expected output via STG eval.
    CompileAndRunChirho,
    /// Source should fail to compile (type error, parse error, etc.).
    CompileFailChirho,
}

/// A single GHC test case.
#[derive(Debug, Clone)]
pub struct GhcTestCaseChirho {
    /// Path to the `.hs` file.
    pub path_chirho: PathBuf,
    /// Test name (from file stem).
    pub name_chirho: String,
    /// What kind of test this is.
    pub kind_chirho: GhcTestKindChirho,
    /// Expected output for compile_and_run tests (from `-- EXPECT_OUTPUT:` comment).
    pub expected_output_chirho: Option<String>,
    /// The Haskell source code.
    pub source_chirho: String,
}

/// Result of running a single test.
#[derive(Debug, Clone)]
pub struct GhcTestResultChirho {
    /// Test name.
    pub name_chirho: String,
    /// Whether the test passed.
    pub passed_chirho: bool,
    /// Description of what happened.
    pub message_chirho: String,
}

/// Aggregate results of running a test suite.
#[derive(Debug, Clone)]
pub struct GhcSuiteResultChirho {
    /// Per-test results.
    pub results_chirho: Vec<GhcTestResultChirho>,
    /// Number of tests that passed.
    pub passed_chirho: usize,
    /// Number of tests that failed.
    pub failed_chirho: usize,
    /// Number of tests total.
    pub total_chirho: usize,
}

impl GhcSuiteResultChirho {
    /// Pass rate as a percentage (0.0 to 100.0).
    pub fn pass_rate_chirho(&self) -> f64 {
        if self.total_chirho == 0 {
            100.0
        } else {
            (self.passed_chirho as f64 / self.total_chirho as f64) * 100.0
        }
    }
}

/// Discover GHC test cases in a directory.
///
/// Looks for `.hs` files and parses their metadata comments.
pub fn discover_ghc_tests_chirho(
    dir_chirho: impl AsRef<Path>,
) -> std::io::Result<Vec<GhcTestCaseChirho>> {
    let dir_chirho = dir_chirho.as_ref();
    let mut tests_chirho = Vec::new();

    if !dir_chirho.is_dir() {
        return Ok(tests_chirho);
    }

    for entry_chirho in fs::read_dir(dir_chirho)? {
        let entry_chirho = entry_chirho?;
        let path_chirho = entry_chirho.path();

        if path_chirho
            .extension()
            .is_some_and(|ext_chirho| ext_chirho == "hs")
        {
            let source_chirho = fs::read_to_string(&path_chirho)?;
            let name_chirho = path_chirho
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();

            let (kind_chirho, expected_output_chirho) =
                parse_test_metadata_chirho(&source_chirho);

            tests_chirho.push(GhcTestCaseChirho {
                path_chirho,
                name_chirho,
                kind_chirho,
                expected_output_chirho,
                source_chirho,
            });
        }
    }

    tests_chirho.sort_by(|a_chirho, b_chirho| a_chirho.name_chirho.cmp(&b_chirho.name_chirho));
    Ok(tests_chirho)
}

/// Parse test metadata from comment headers.
fn parse_test_metadata_chirho(
    source_chirho: &str,
) -> (GhcTestKindChirho, Option<String>) {
    let mut kind_chirho = GhcTestKindChirho::CompileChirho;
    let mut expected_output_chirho = None;

    for line_chirho in source_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();
        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("-- TEST:") {
            let test_type_chirho = rest_chirho.trim();
            kind_chirho = match test_type_chirho {
                "compile_and_run" => GhcTestKindChirho::CompileAndRunChirho,
                "compile_fail" => GhcTestKindChirho::CompileFailChirho,
                _ => GhcTestKindChirho::CompileChirho,
            };
        } else if let Some(rest_chirho) = trimmed_chirho.strip_prefix("-- EXPECT_OUTPUT:") {
            expected_output_chirho = Some(rest_chirho.trim().to_string());
        } else if !trimmed_chirho.starts_with("--") {
            // Stop parsing at first non-comment line.
            break;
        }
    }

    (kind_chirho, expected_output_chirho)
}

/// Run a single GHC test case through the Haskeluya pipeline.
pub fn run_ghc_test_chirho(
    test_chirho: &GhcTestCaseChirho,
) -> GhcTestResultChirho {
    let mut source_map_chirho = haskeluya_span_chirho::SourceMapChirho::new_chirho();

    match test_chirho.kind_chirho {
        GhcTestKindChirho::CompileChirho => {
            // Should compile without errors.
            match haskeluya_driver_chirho::compile_source_chirho(
                &test_chirho.source_chirho,
                &mut source_map_chirho,
                &test_chirho.name_chirho,
            ) {
                Ok(_) => GhcTestResultChirho {
                    name_chirho: test_chirho.name_chirho.clone(),
                    passed_chirho: true,
                    message_chirho: "compiled successfully".to_string(),
                },
                Err(diag_chirho) => GhcTestResultChirho {
                    name_chirho: test_chirho.name_chirho.clone(),
                    passed_chirho: false,
                    message_chirho: format!(
                        "expected successful compilation, got {} error(s)",
                        diag_chirho.diagnostics_chirho().len()
                    ),
                },
            }
        }
        GhcTestKindChirho::CompileAndRunChirho => {
            // Should compile and produce expected output.
            match haskeluya_driver_chirho::eval_source_with_machine_chirho(
                &test_chirho.source_chirho,
                &mut source_map_chirho,
                &test_chirho.name_chirho,
                None,
            ) {
                Ok((_value_chirho, machine_chirho)) => {
                    let output_chirho = machine_chirho.io_output_chirho.trim().to_string();
                    if let Some(expected_chirho) = &test_chirho.expected_output_chirho {
                        if output_chirho == expected_chirho.trim() {
                            GhcTestResultChirho {
                                name_chirho: test_chirho.name_chirho.clone(),
                                passed_chirho: true,
                                message_chirho: format!("output matched: {}", output_chirho),
                            }
                        } else {
                            GhcTestResultChirho {
                                name_chirho: test_chirho.name_chirho.clone(),
                                passed_chirho: false,
                                message_chirho: format!(
                                    "output mismatch: expected '{}', got '{}'",
                                    expected_chirho, output_chirho
                                ),
                            }
                        }
                    } else {
                        // No expected output specified — just check it runs.
                        GhcTestResultChirho {
                            name_chirho: test_chirho.name_chirho.clone(),
                            passed_chirho: true,
                            message_chirho: format!(
                                "ran successfully, output: {}",
                                if output_chirho.is_empty() {
                                    "(none)"
                                } else {
                                    &output_chirho
                                }
                            ),
                        }
                    }
                }
                Err(error_chirho) => GhcTestResultChirho {
                    name_chirho: test_chirho.name_chirho.clone(),
                    passed_chirho: false,
                    message_chirho: format!("runtime error: {}", error_chirho),
                },
            }
        }
        GhcTestKindChirho::CompileFailChirho => {
            // Should fail to compile.
            match haskeluya_driver_chirho::compile_source_chirho(
                &test_chirho.source_chirho,
                &mut source_map_chirho,
                &test_chirho.name_chirho,
            ) {
                Ok(_) => GhcTestResultChirho {
                    name_chirho: test_chirho.name_chirho.clone(),
                    passed_chirho: false,
                    message_chirho:
                        "expected compilation failure, but compiled successfully"
                            .to_string(),
                },
                Err(_) => GhcTestResultChirho {
                    name_chirho: test_chirho.name_chirho.clone(),
                    passed_chirho: true,
                    message_chirho: "correctly rejected with error".to_string(),
                },
            }
        }
    }
}

/// Run all discovered tests and produce aggregate results.
pub fn run_ghc_suite_chirho(
    tests_chirho: &[GhcTestCaseChirho],
) -> GhcSuiteResultChirho {
    let mut results_chirho = Vec::new();
    let mut passed_chirho = 0usize;
    let mut failed_chirho = 0usize;

    for test_chirho in tests_chirho {
        let result_chirho = run_ghc_test_chirho(test_chirho);
        if result_chirho.passed_chirho {
            passed_chirho += 1;
        } else {
            failed_chirho += 1;
        }
        results_chirho.push(result_chirho);
    }

    GhcSuiteResultChirho {
        total_chirho: tests_chirho.len(),
        passed_chirho,
        failed_chirho,
        results_chirho,
    }
}

/// Create a test `.hs` file with metadata for the GHC test runner.
pub fn write_ghc_test_chirho(
    dir_chirho: &Path,
    name_chirho: &str,
    kind_chirho: GhcTestKindChirho,
    expected_output_chirho: Option<&str>,
    source_body_chirho: &str,
) -> std::io::Result<PathBuf> {
    let mut content_chirho = String::new();
    match kind_chirho {
        GhcTestKindChirho::CompileChirho => {
            content_chirho.push_str("-- TEST: compile\n");
        }
        GhcTestKindChirho::CompileAndRunChirho => {
            content_chirho.push_str("-- TEST: compile_and_run\n");
        }
        GhcTestKindChirho::CompileFailChirho => {
            content_chirho.push_str("-- TEST: compile_fail\n");
        }
    }
    if let Some(output_chirho) = expected_output_chirho {
        content_chirho.push_str(&format!("-- EXPECT_OUTPUT: {}\n", output_chirho));
    }
    content_chirho.push_str(source_body_chirho);

    let path_chirho = dir_chirho.join(format!("{name_chirho}.hs"));
    fs::write(&path_chirho, &content_chirho)?;
    Ok(path_chirho)
}

// ---------------------------------------------------------------------------
// GHC full testsuite `.T` file parsing
// ---------------------------------------------------------------------------

/// A test entry parsed from a GHC `.T` driver file.
#[derive(Debug, Clone)]
pub struct GhcDriverEntryChirho {
    /// Test name (e.g. "T1234").
    pub name_chirho: String,
    /// Test kind from the `.T` file.
    pub kind_chirho: GhcTestKindChirho,
}

/// Parse a GHC `.T` driver file to extract test entries.
///
/// GHC `.T` files contain lines like:
/// ```text
/// test('T1234', normal, compile, [''])
/// test('T5678', [extra_files(['helper.hs'])], compile_and_run, [''])
/// test('T9999', normal, compile_fail, [''])
/// ```
///
/// We extract the test name and the test kind (compile, compile_and_run, compile_fail).
pub fn parse_dot_t_file_chirho(
    content_chirho: &str,
) -> Vec<GhcDriverEntryChirho> {
    let mut entries_chirho = Vec::new();

    for line_chirho in content_chirho.lines() {
        let trimmed_chirho = line_chirho.trim();

        // Match lines starting with "test("
        if !trimmed_chirho.starts_with("test(") {
            continue;
        }

        // Extract test name from first quoted string.
        let name_chirho = match extract_quoted_string_chirho(trimmed_chirho) {
            Some(name_chirho) => name_chirho,
            None => continue,
        };

        // Extract test kind from the third argument.
        let kind_chirho = if trimmed_chirho.contains("compile_and_run") {
            GhcTestKindChirho::CompileAndRunChirho
        } else if trimmed_chirho.contains("compile_fail") {
            GhcTestKindChirho::CompileFailChirho
        } else if trimmed_chirho.contains("compile") {
            GhcTestKindChirho::CompileChirho
        } else {
            // Skip test kinds we don't handle (multimod, ghci_script, etc.)
            continue;
        };

        entries_chirho.push(GhcDriverEntryChirho {
            name_chirho,
            kind_chirho,
        });
    }

    entries_chirho
}

/// Extract the first single-quoted string from a line.
fn extract_quoted_string_chirho(line_chirho: &str) -> Option<String> {
    let start_chirho = line_chirho.find('\'')?;
    let rest_chirho = &line_chirho[start_chirho + 1..];
    let end_chirho = rest_chirho.find('\'')?;
    Some(rest_chirho[..end_chirho].to_string())
}

/// Discover GHC testsuite tests from a directory containing `.T` files and
/// their corresponding `.hs` source files.
///
/// Walks a GHC testsuite directory tree (e.g. `testsuite/tests/`),
/// finds all `.T` files, parses them, and locates the matching `.hs` files.
pub fn discover_ghc_full_suite_chirho(
    testsuite_dir_chirho: impl AsRef<Path>,
) -> std::io::Result<Vec<GhcTestCaseChirho>> {
    let dir_chirho = testsuite_dir_chirho.as_ref();
    let mut tests_chirho = Vec::new();

    discover_t_files_recursive_chirho(dir_chirho, &mut tests_chirho)?;

    tests_chirho.sort_by(|a_chirho, b_chirho| a_chirho.name_chirho.cmp(&b_chirho.name_chirho));
    Ok(tests_chirho)
}

/// Recursively find `.T` files and build test cases.
fn discover_t_files_recursive_chirho(
    dir_chirho: &Path,
    tests_chirho: &mut Vec<GhcTestCaseChirho>,
) -> std::io::Result<()> {
    if !dir_chirho.is_dir() {
        return Ok(());
    }

    let mut t_file_entries_chirho: Vec<(PathBuf, GhcDriverEntryChirho)> = Vec::new();

    for entry_chirho in fs::read_dir(dir_chirho)? {
        let entry_chirho = entry_chirho?;
        let path_chirho = entry_chirho.path();

        if path_chirho.is_dir() {
            discover_t_files_recursive_chirho(&path_chirho, tests_chirho)?;
        } else if path_chirho
            .extension()
            .is_some_and(|ext_chirho| ext_chirho == "T")
        {
            // Parse the .T file.
            if let Ok(content_chirho) = fs::read_to_string(&path_chirho) {
                let entries_chirho = parse_dot_t_file_chirho(&content_chirho);
                for entry_chirho in entries_chirho {
                    t_file_entries_chirho
                        .push((path_chirho.clone(), entry_chirho));
                }
            }
        }
    }

    // For each entry, find the corresponding .hs file.
    for (t_path_chirho, entry_chirho) in &t_file_entries_chirho {
        let test_dir_chirho = t_path_chirho.parent().unwrap_or(dir_chirho);
        let hs_path_chirho = test_dir_chirho.join(format!("{}.hs", entry_chirho.name_chirho));

        if hs_path_chirho.exists() {
            if let Ok(source_chirho) = fs::read_to_string(&hs_path_chirho) {
                tests_chirho.push(GhcTestCaseChirho {
                    path_chirho: hs_path_chirho,
                    name_chirho: entry_chirho.name_chirho.clone(),
                    kind_chirho: entry_chirho.kind_chirho.clone(),
                    expected_output_chirho: None, // GHC uses .stdout files
                    source_chirho,
                });
            }
        }
    }

    Ok(())
}

/// Load expected output from a GHC `.stdout` file (if it exists).
pub fn load_expected_stdout_chirho(
    test_path_chirho: &Path,
) -> Option<String> {
    let stdout_path_chirho = test_path_chirho.with_extension("stdout");
    fs::read_to_string(&stdout_path_chirho).ok()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn parse_metadata_compile_chirho() {
        let (kind_chirho, output_chirho) =
            parse_test_metadata_chirho("-- TEST: compile\nmodule Foo where\n");
        assert_eq!(kind_chirho, GhcTestKindChirho::CompileChirho);
        assert!(output_chirho.is_none());
    }

    #[test]
    fn parse_metadata_compile_and_run_chirho() {
        let (kind_chirho, output_chirho) =
            parse_test_metadata_chirho("-- TEST: compile_and_run\n-- EXPECT_OUTPUT: 42\nmain = print 42\n");
        assert_eq!(kind_chirho, GhcTestKindChirho::CompileAndRunChirho);
        assert_eq!(output_chirho.unwrap(), "42");
    }

    #[test]
    fn parse_metadata_compile_fail_chirho() {
        let (kind_chirho, output_chirho) =
            parse_test_metadata_chirho("-- TEST: compile_fail\nx = True + 1\n");
        assert_eq!(kind_chirho, GhcTestKindChirho::CompileFailChirho);
        assert!(output_chirho.is_none());
    }

    #[test]
    fn parse_metadata_default_chirho() {
        let (kind_chirho, _) = parse_test_metadata_chirho("module Foo where\nfoo = 1\n");
        assert_eq!(kind_chirho, GhcTestKindChirho::CompileChirho);
    }

    #[test]
    fn discover_ghc_tests_chirho_test() {
        let tmp_chirho = tempfile::tempdir().unwrap();
        write_ghc_test_chirho(
            tmp_chirho.path(),
            "T001",
            GhcTestKindChirho::CompileChirho,
            None,
            "module T001 where\nfoo = 1\n",
        )
        .unwrap();
        write_ghc_test_chirho(
            tmp_chirho.path(),
            "T002",
            GhcTestKindChirho::CompileAndRunChirho,
            Some("42"),
            "main = putStrLn \"42\"\n",
        )
        .unwrap();

        let tests_chirho = discover_ghc_tests_chirho(tmp_chirho.path()).unwrap();
        assert_eq!(tests_chirho.len(), 2);
        assert_eq!(tests_chirho[0].name_chirho, "T001");
        assert_eq!(tests_chirho[1].name_chirho, "T002");
    }

    #[test]
    fn run_compile_test_pass_chirho() {
        let test_chirho = GhcTestCaseChirho {
            path_chirho: PathBuf::from("inline.hs"),
            name_chirho: "inline".to_string(),
            kind_chirho: GhcTestKindChirho::CompileChirho,
            expected_output_chirho: None,
            source_chirho: "module Inline where\nfoo x = x + 1\n".to_string(),
        };

        let result_chirho = run_ghc_test_chirho(&test_chirho);
        assert!(result_chirho.passed_chirho, "msg: {}", result_chirho.message_chirho);
    }

    #[test]
    fn run_compile_fail_test_pass_chirho() {
        let test_chirho = GhcTestCaseChirho {
            path_chirho: PathBuf::from("typeerror.hs"),
            name_chirho: "typeerror".to_string(),
            kind_chirho: GhcTestKindChirho::CompileFailChirho,
            expected_output_chirho: None,
            source_chirho: "module TypeErr where\nx :: Int\nx = True\n".to_string(),
        };

        let result_chirho = run_ghc_test_chirho(&test_chirho);
        assert!(result_chirho.passed_chirho, "msg: {}", result_chirho.message_chirho);
    }

    #[test]
    fn run_compile_and_run_test_pass_chirho() {
        let test_chirho = GhcTestCaseChirho {
            path_chirho: PathBuf::from("hello.hs"),
            name_chirho: "hello".to_string(),
            kind_chirho: GhcTestKindChirho::CompileAndRunChirho,
            expected_output_chirho: Some("42".to_string()),
            source_chirho: "main = putStrLn \"42\"\n".to_string(),
        };

        let result_chirho = run_ghc_test_chirho(&test_chirho);
        assert!(result_chirho.passed_chirho, "msg: {}", result_chirho.message_chirho);
    }

    #[test]
    fn run_suite_chirho_test() {
        let tests_chirho = vec![
            GhcTestCaseChirho {
                path_chirho: PathBuf::from("t1.hs"),
                name_chirho: "t1".to_string(),
                kind_chirho: GhcTestKindChirho::CompileChirho,
                expected_output_chirho: None,
                source_chirho: "module T1 where\nfoo = 1\n".to_string(),
            },
            GhcTestCaseChirho {
                path_chirho: PathBuf::from("t2.hs"),
                name_chirho: "t2".to_string(),
                kind_chirho: GhcTestKindChirho::CompileAndRunChirho,
                expected_output_chirho: Some("hello".to_string()),
                source_chirho: "main = putStrLn \"hello\"\n".to_string(),
            },
            GhcTestCaseChirho {
                path_chirho: PathBuf::from("t3.hs"),
                name_chirho: "t3".to_string(),
                kind_chirho: GhcTestKindChirho::CompileFailChirho,
                expected_output_chirho: None,
                source_chirho: "module T3 where\nx :: Int\nx = True\n".to_string(),
            },
        ];

        let suite_chirho = run_ghc_suite_chirho(&tests_chirho);
        assert_eq!(suite_chirho.total_chirho, 3);
        assert_eq!(suite_chirho.passed_chirho, 3);
        assert_eq!(suite_chirho.failed_chirho, 0);
        assert!((suite_chirho.pass_rate_chirho() - 100.0).abs() < 0.01);
    }

    #[test]
    fn suite_empty_is_100_percent_chirho() {
        let suite_chirho = run_ghc_suite_chirho(&[]);
        assert_eq!(suite_chirho.total_chirho, 0);
        assert!((suite_chirho.pass_rate_chirho() - 100.0).abs() < 0.01);
    }

    #[test]
    fn write_and_discover_and_run_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();

        write_ghc_test_chirho(
            tmp_chirho.path(),
            "basic_let",
            GhcTestKindChirho::CompileAndRunChirho,
            Some("7"),
            "main = print (let x = 3 in x + 4)\n",
        )
        .unwrap();

        write_ghc_test_chirho(
            tmp_chirho.path(),
            "basic_where",
            GhcTestKindChirho::CompileChirho,
            None,
            "module BasicWhere where\nf x = y + x where y = 10\n",
        )
        .unwrap();

        let tests_chirho = discover_ghc_tests_chirho(tmp_chirho.path()).unwrap();
        assert_eq!(tests_chirho.len(), 2);

        let suite_chirho = run_ghc_suite_chirho(&tests_chirho);
        assert_eq!(suite_chirho.total_chirho, 2);
        // Both should pass
        assert_eq!(
            suite_chirho.passed_chirho, 2,
            "failures: {:?}",
            suite_chirho
                .results_chirho
                .iter()
                .filter(|r_chirho| !r_chirho.passed_chirho)
                .map(|r_chirho| format!("{}: {}", r_chirho.name_chirho, r_chirho.message_chirho))
                .collect::<Vec<_>>()
        );
    }

    // -----------------------------------------------------------------------
    // `.T` file parser tests
    // -----------------------------------------------------------------------

    #[test]
    fn parse_dot_t_compile_chirho() {
        let content_chirho = "test('T001', normal, compile, [''])\n";
        let entries_chirho = parse_dot_t_file_chirho(content_chirho);
        assert_eq!(entries_chirho.len(), 1);
        assert_eq!(entries_chirho[0].name_chirho, "T001");
        assert_eq!(entries_chirho[0].kind_chirho, GhcTestKindChirho::CompileChirho);
    }

    #[test]
    fn parse_dot_t_compile_and_run_chirho() {
        let content_chirho = "test('T002', normal, compile_and_run, [''])\n";
        let entries_chirho = parse_dot_t_file_chirho(content_chirho);
        assert_eq!(entries_chirho.len(), 1);
        assert_eq!(entries_chirho[0].name_chirho, "T002");
        assert_eq!(entries_chirho[0].kind_chirho, GhcTestKindChirho::CompileAndRunChirho);
    }

    #[test]
    fn parse_dot_t_compile_fail_chirho() {
        let content_chirho = "test('T003', normal, compile_fail, [''])\n";
        let entries_chirho = parse_dot_t_file_chirho(content_chirho);
        assert_eq!(entries_chirho.len(), 1);
        assert_eq!(entries_chirho[0].name_chirho, "T003");
        assert_eq!(entries_chirho[0].kind_chirho, GhcTestKindChirho::CompileFailChirho);
    }

    #[test]
    fn parse_dot_t_multiple_chirho() {
        let content_chirho = "\
test('T001', normal, compile, [''])
test('T002', [extra_files(['helper.hs'])], compile_and_run, [''])
test('T003', normal, compile_fail, [''])
# comment line
test('T004', normal, compile, ['-O'])
";
        let entries_chirho = parse_dot_t_file_chirho(content_chirho);
        assert_eq!(entries_chirho.len(), 4);
        assert_eq!(entries_chirho[0].name_chirho, "T001");
        assert_eq!(entries_chirho[1].name_chirho, "T002");
        assert_eq!(entries_chirho[2].name_chirho, "T003");
        assert_eq!(entries_chirho[3].name_chirho, "T004");
    }

    #[test]
    fn parse_dot_t_skips_unknown_kinds_chirho() {
        let content_chirho = "test('T001', normal, ghci_script, [''])\n";
        let entries_chirho = parse_dot_t_file_chirho(content_chirho);
        assert_eq!(entries_chirho.len(), 0);
    }

    #[test]
    fn discover_full_suite_from_temp_dir_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();

        // Create a .T file
        fs::write(
            tmp_chirho.path().join("all.T"),
            "test('Simple', normal, compile, [''])\ntest('Run', normal, compile_and_run, [''])\n",
        )
        .unwrap();

        // Create matching .hs files
        fs::write(
            tmp_chirho.path().join("Simple.hs"),
            "module Simple where\nfoo = 1\n",
        )
        .unwrap();
        fs::write(
            tmp_chirho.path().join("Run.hs"),
            "main = putStrLn \"ok\"\n",
        )
        .unwrap();

        let tests_chirho = discover_ghc_full_suite_chirho(tmp_chirho.path()).unwrap();
        assert_eq!(tests_chirho.len(), 2);

        // Run them
        let suite_chirho = run_ghc_suite_chirho(&tests_chirho);
        assert_eq!(suite_chirho.total_chirho, 2);
        assert_eq!(suite_chirho.passed_chirho, 2);
    }

    #[test]
    fn discover_full_suite_skips_missing_hs_chirho() {
        let tmp_chirho = tempfile::tempdir().unwrap();

        // .T file references a test but no .hs file exists
        fs::write(
            tmp_chirho.path().join("all.T"),
            "test('Missing', normal, compile, [''])\n",
        )
        .unwrap();

        let tests_chirho = discover_ghc_full_suite_chirho(tmp_chirho.path()).unwrap();
        assert_eq!(tests_chirho.len(), 0); // No matching .hs file → no test
    }

    #[test]
    fn extract_quoted_string_chirho_test() {
        assert_eq!(
            extract_quoted_string_chirho("test('T001', normal, compile, [''])"),
            Some("T001".to_string())
        );
        assert_eq!(
            extract_quoted_string_chirho("no quotes here"),
            None
        );
    }
}
