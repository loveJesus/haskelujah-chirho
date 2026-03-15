// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # rhasky-test-harness-chirho
//!
//! Testing infrastructure for the Rhasky compiler.
//! Provides golden test runners, test corpus discovery, and snapshot
//! comparison utilities.

pub mod ghc_suite_chirho;

use std::fs;
use std::path::{Path, PathBuf};

/// A single golden test case: a `.hs` input file paired with a `.expected`
/// output file.
#[derive(Debug, Clone)]
pub struct GoldenTestCaseChirho {
    /// Path to the `.hs` input file.
    pub input_path_chirho: PathBuf,
    /// Path to the `.expected` output file.
    pub expected_path_chirho: PathBuf,
    /// The test name, derived from the file stem.
    pub name_chirho: String,
}

/// Discover all golden test cases in a directory.
///
/// Looks for files matching `*.hs` and checks for a sibling `*.expected`.
/// Test cases without a `.expected` file are included with `expected_path_chirho`
/// pointing to where the file would be (useful for `--bless` workflows).
pub fn discover_golden_tests_chirho(
    dir_chirho: impl AsRef<Path>,
) -> std::io::Result<Vec<GoldenTestCaseChirho>> {
    let mut tests_chirho = Vec::new();
    let dir_chirho = dir_chirho.as_ref();

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
            let name_chirho = path_chirho
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            let expected_path_chirho = path_chirho.with_extension("expected");
            tests_chirho.push(GoldenTestCaseChirho {
                input_path_chirho: path_chirho,
                expected_path_chirho,
                name_chirho,
            });
        }
    }

    tests_chirho.sort_by(|a_chirho, b_chirho| a_chirho.name_chirho.cmp(&b_chirho.name_chirho));
    Ok(tests_chirho)
}

/// Compare actual output against expected output.
/// Returns `Ok(())` if they match, or an `Err` with a diff description.
pub fn assert_golden_chirho(
    actual_chirho: &str,
    expected_path_chirho: &Path,
) -> Result<(), GoldenMismatchChirho> {
    if !expected_path_chirho.exists() {
        return Err(GoldenMismatchChirho {
            expected_chirho: String::new(),
            actual_chirho: actual_chirho.to_owned(),
            path_chirho: expected_path_chirho.to_path_buf(),
            missing_expected_chirho: true,
            read_error_chirho: None,
        });
    }

    let expected_chirho =
        fs::read_to_string(expected_path_chirho).map_err(|read_error_chirho| {
            GoldenMismatchChirho {
                expected_chirho: String::new(),
                actual_chirho: actual_chirho.to_owned(),
                path_chirho: expected_path_chirho.to_path_buf(),
                missing_expected_chirho: false,
                read_error_chirho: Some(read_error_chirho.to_string()),
            }
        })?;

    let expected_normalized_chirho = normalize_line_endings_chirho(&expected_chirho);
    let actual_normalized_chirho = normalize_line_endings_chirho(actual_chirho);

    if expected_normalized_chirho == actual_normalized_chirho {
        Ok(())
    } else {
        Err(GoldenMismatchChirho {
            expected_chirho,
            actual_chirho: actual_chirho.to_owned(),
            path_chirho: expected_path_chirho.to_path_buf(),
            missing_expected_chirho: false,
            read_error_chirho: None,
        })
    }
}

/// Write actual output as the new expected file (the `--bless` workflow).
pub fn bless_golden_chirho(
    actual_chirho: &str,
    expected_path_chirho: &Path,
) -> std::io::Result<()> {
    if let Some(parent_chirho) = expected_path_chirho.parent() {
        fs::create_dir_all(parent_chirho)?;
    }
    fs::write(expected_path_chirho, actual_chirho)
}

/// A mismatch between actual and expected golden output.
#[derive(Debug)]
pub struct GoldenMismatchChirho {
    pub expected_chirho: String,
    pub actual_chirho: String,
    pub path_chirho: PathBuf,
    pub missing_expected_chirho: bool,
    pub read_error_chirho: Option<String>,
}

impl std::fmt::Display for GoldenMismatchChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.missing_expected_chirho {
            write!(
                f_chirho,
                "Golden file missing: {}\nActual output:\n{}",
                self.path_chirho.display(),
                self.actual_chirho
            )
        } else if let Some(read_error_chirho) = &self.read_error_chirho {
            write!(
                f_chirho,
                "Golden file unreadable: {}\nRead error: {}\nActual output:\n{}",
                self.path_chirho.display(),
                read_error_chirho,
                self.actual_chirho
            )
        } else {
            write!(
                f_chirho,
                "Golden mismatch in: {}\n--- expected ---\n{}\n--- actual ---\n{}",
                self.path_chirho.display(),
                self.expected_chirho,
                self.actual_chirho
            )
        }
    }
}

impl std::error::Error for GoldenMismatchChirho {}

fn normalize_line_endings_chirho(text_chirho: &str) -> String {
    text_chirho.replace("\r\n", "\n")
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use std::io::Write;

    #[test]
    fn golden_match_succeeds_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let expected_file_chirho = dir_chirho.path().join("test.expected");
        let mut f_chirho = fs::File::create(&expected_file_chirho).unwrap();
        write!(f_chirho, "hello world\n").unwrap();

        assert!(assert_golden_chirho("hello world\n", &expected_file_chirho).is_ok());
    }

    #[test]
    fn golden_mismatch_fails_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let expected_file_chirho = dir_chirho.path().join("test.expected");
        fs::write(&expected_file_chirho, "expected\n").unwrap();

        let result_chirho = assert_golden_chirho("actual\n", &expected_file_chirho);
        assert!(result_chirho.is_err());
    }

    #[test]
    fn golden_missing_expected_chirho() {
        let path_chirho = PathBuf::from("/nonexistent/test.expected");
        let result_chirho = assert_golden_chirho("output\n", &path_chirho);
        assert!(result_chirho.is_err());
        assert!(result_chirho.unwrap_err().missing_expected_chirho);
    }

    #[test]
    fn golden_read_error_is_distinct_from_missing_chirho() {
        let dir_chirho = tempfile::tempdir().unwrap();
        let result_chirho = assert_golden_chirho("output\n", dir_chirho.path());
        let error_chirho = result_chirho.unwrap_err();
        assert!(!error_chirho.missing_expected_chirho);
        assert!(error_chirho.read_error_chirho.is_some());
    }
}
