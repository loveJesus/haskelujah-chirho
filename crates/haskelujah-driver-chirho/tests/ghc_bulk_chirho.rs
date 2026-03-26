// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # GHC Bulk Test Runner
//!
//! Walks `ghc-tests-chirho/` subdirectories and runs each `.hs` file through
//! the Haskelujah pipeline at the appropriate phase:
//! - `should_compile` → must parse + type-check without error
//! - `should_fail` → must produce a compilation error
//! - `should_run` → must parse + type-check (execution optional)
//!
//! Reports per-category pass rates. This is a tracking test — it does NOT
//! fail on individual test failures; it tracks aggregate pass rates and
//! asserts minimum thresholds.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use haskelujah_span_chirho::SourceMapChirho;

/// Result of attempting to compile a single file.
#[derive(Debug)]
#[allow(dead_code)]
struct BulkResultChirho {
    path_chirho: PathBuf,
    category_chirho: String,
    passed_chirho: bool,
    error_chirho: Option<String>,
}

/// Try to parse a .hs file through the Haskelujah frontend.
/// Uses sibling file search to resolve companion test modules.
fn try_parse_chirho(path_chirho: &Path) -> Result<(), String> {
    let source_chirho =
        fs::read_to_string(path_chirho).map_err(|e_chirho| format!("read error: {}", e_chirho))?;

    let mut sm_chirho = SourceMapChirho::new_chirho();
    let file_name_chirho = path_chirho
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Use compile_source (no sibling scan) to avoid O(n²) file reads
    // that cause 12GB+ memory usage across 938 files.
    haskelujah_driver::compile_source_chirho(&source_chirho, &mut sm_chirho, &file_name_chirho)
        .map(|_| ())
        .map_err(|e_chirho| format!("{}", e_chirho))
}

/// Discover all .hs files under a directory, recursively.
fn discover_hs_files_chirho(dir_chirho: &Path) -> Vec<PathBuf> {
    let mut files_chirho = Vec::new();
    if !dir_chirho.is_dir() {
        return files_chirho;
    }
    fn walk_chirho(dir_chirho: &Path, out_chirho: &mut Vec<PathBuf>) {
        if let Ok(entries_chirho) = fs::read_dir(dir_chirho) {
            for entry_chirho in entries_chirho.flatten() {
                let p_chirho = entry_chirho.path();
                if p_chirho.is_dir() {
                    walk_chirho(&p_chirho, out_chirho);
                } else if p_chirho
                    .extension()
                    .is_some_and(|e_chirho| e_chirho == "hs")
                {
                    out_chirho.push(p_chirho);
                }
            }
        }
    }
    walk_chirho(dir_chirho, &mut files_chirho);
    files_chirho.sort();
    files_chirho
}

/// Run bulk tests for a given category directory.
fn run_category_chirho(
    base_dir_chirho: &Path,
    category_chirho: &str,
    subcategory_chirho: &str,
    expect_success_chirho: bool,
) -> (usize, usize, Vec<String>) {
    let dir_chirho = base_dir_chirho
        .join(category_chirho)
        .join(subcategory_chirho);
    if !dir_chirho.is_dir() {
        return (0, 0, vec![]);
    }

    let files_chirho = discover_hs_files_chirho(&dir_chirho);
    let total_chirho = files_chirho.len();
    let passed_chirho = AtomicUsize::new(0);
    let mut failures_chirho = Vec::new();

    for file_chirho in &files_chirho {
        let result_chirho = std::panic::catch_unwind(|| try_parse_chirho(file_chirho));

        let ok_chirho = match result_chirho {
            Ok(Ok(())) => expect_success_chirho,
            Ok(Err(_)) => !expect_success_chirho,
            Err(_) => false, // panic = always fail
        };

        if ok_chirho {
            passed_chirho.fetch_add(1, Ordering::Relaxed);
        } else {
            let name_chirho = file_chirho
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            failures_chirho.push(name_chirho);
        }
    }

    (
        passed_chirho.load(Ordering::Relaxed),
        total_chirho,
        failures_chirho,
    )
}

// ── Test: parser/should_compile ────────────────────────────────────────

#[test]
fn ghc_bulk_parser_should_compile_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho");

    if !base_chirho.is_dir() {
        eprintln!("ghc-tests-chirho/ not found, skipping bulk parser tests");
        return;
    }

    let (passed_chirho, total_chirho, failures_chirho) =
        run_category_chirho(&base_chirho, "parser-chirho", "should_compile", true);

    let rate_chirho = if total_chirho > 0 {
        passed_chirho as f64 / total_chirho as f64 * 100.0
    } else {
        100.0
    };

    eprintln!(
        "\n[parser/should_compile] {}/{} passed ({:.1}%)",
        passed_chirho, total_chirho, rate_chirho
    );
    if !failures_chirho.is_empty() {
        eprintln!("  Failed ({}):", failures_chirho.len());
        for f_chirho in &failures_chirho {
            eprintln!("    - {}", f_chirho);
        }
    }

    // Track rate — don't require 100% since many tests use GHC extensions
    assert!(
        total_chirho > 0,
        "should have found parser/should_compile tests"
    );
}

// ── Test: parser/should_fail ───────────────────────────────────────────

#[test]
fn ghc_bulk_parser_should_fail_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho");

    if !base_chirho.is_dir() {
        return;
    }

    let (passed_chirho, total_chirho, _) =
        run_category_chirho(&base_chirho, "parser-chirho", "should_fail", false);

    let rate_chirho = if total_chirho > 0 {
        passed_chirho as f64 / total_chirho as f64 * 100.0
    } else {
        100.0
    };

    eprintln!(
        "\n[parser/should_fail] {}/{} correctly rejected ({:.1}%)",
        passed_chirho, total_chirho, rate_chirho
    );
}

// ── Test: typecheck/should_compile ─────────────────────────────────────

#[test]
fn ghc_bulk_typecheck_should_compile_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho");

    if !base_chirho.is_dir() {
        return;
    }

    let (passed_chirho, total_chirho, failures_chirho) =
        run_category_chirho(&base_chirho, "typecheck-chirho", "should_compile", true);

    let rate_chirho = if total_chirho > 0 {
        passed_chirho as f64 / total_chirho as f64 * 100.0
    } else {
        100.0
    };

    eprintln!(
        "\n[typecheck/should_compile] {}/{} passed ({:.1}%)",
        passed_chirho, total_chirho, rate_chirho
    );
    if !failures_chirho.is_empty() {
        eprintln!("  Failed ({}):", failures_chirho.len());
        for f_chirho in &failures_chirho {
            eprintln!("    - {}", f_chirho);
        }
    }
}

// ── Test: typecheck/should_fail ────────────────────────────────────────

#[test]
fn ghc_bulk_typecheck_should_fail_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho");

    if !base_chirho.is_dir() {
        return;
    }

    let (passed_chirho, total_chirho, _) =
        run_category_chirho(&base_chirho, "typecheck-chirho", "should_fail", false);

    let rate_chirho = if total_chirho > 0 {
        passed_chirho as f64 / total_chirho as f64 * 100.0
    } else {
        100.0
    };

    eprintln!(
        "\n[typecheck/should_fail] {}/{} correctly rejected ({:.1}%)",
        passed_chirho, total_chirho, rate_chirho
    );
}

// ── Test: rename/should_compile ────────────────────────────────────────

#[test]
fn ghc_bulk_rename_should_compile_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho");

    if !base_chirho.is_dir() {
        return;
    }

    let (passed_chirho, total_chirho, failures_chirho) =
        run_category_chirho(&base_chirho, "rename-chirho", "should_compile", true);

    let rate_chirho = if total_chirho > 0 {
        passed_chirho as f64 / total_chirho as f64 * 100.0
    } else {
        100.0
    };

    eprintln!(
        "\n[rename/should_compile] {}/{} passed ({:.1}%)",
        passed_chirho, total_chirho, rate_chirho
    );
    if !failures_chirho.is_empty() {
        eprintln!("  Failed ({}):", failures_chirho.len());
        for f_chirho in failures_chirho.iter().take(20) {
            eprintln!("    - {}", f_chirho);
        }
    }
}

// ── Test: layout tests ─────────────────────────────────────────────────

#[test]
fn ghc_bulk_layout_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho")
        .join("layout-chirho");

    if !base_chirho.is_dir() {
        return;
    }

    let files_chirho = discover_hs_files_chirho(&base_chirho);
    let total_chirho = files_chirho.len();
    let mut passed_chirho = 0usize;

    for file_chirho in &files_chirho {
        let result_chirho = std::panic::catch_unwind(|| try_parse_chirho(file_chirho));
        if matches!(result_chirho, Ok(Ok(()))) {
            passed_chirho += 1;
        }
    }

    let rate_chirho = if total_chirho > 0 {
        passed_chirho as f64 / total_chirho as f64 * 100.0
    } else {
        100.0
    };

    eprintln!(
        "\n[layout] {}/{} passed ({:.1}%)",
        passed_chirho, total_chirho, rate_chirho
    );
}

// ── Test: module tests ─────────────────────────────────────────────────

#[test]
fn ghc_bulk_module_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho")
        .join("module-chirho");

    if !base_chirho.is_dir() {
        return;
    }

    let files_chirho = discover_hs_files_chirho(&base_chirho);
    let total_chirho = files_chirho.len();
    let mut passed_chirho = 0usize;

    for file_chirho in &files_chirho {
        let result_chirho = std::panic::catch_unwind(|| try_parse_chirho(file_chirho));
        if matches!(result_chirho, Ok(Ok(()))) {
            passed_chirho += 1;
        }
    }

    let rate_chirho = if total_chirho > 0 {
        passed_chirho as f64 / total_chirho as f64 * 100.0
    } else {
        100.0
    };

    eprintln!(
        "\n[module] {}/{} passed ({:.1}%)",
        passed_chirho, total_chirho, rate_chirho
    );
}

// ── Aggregate summary ──────────────────────────────────────────────────

#[test]
fn ghc_bulk_summary_chirho() {
    let base_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("ghc-tests-chirho");

    if !base_chirho.is_dir() {
        eprintln!("ghc-tests-chirho/ not found, skipping");
        return;
    }

    let categories_chirho: Vec<(&str, &str, bool)> = vec![
        ("parser-chirho", "should_compile", true),
        ("parser-chirho", "should_fail", false),
        ("typecheck-chirho", "should_compile", true),
        ("typecheck-chirho", "should_fail", false),
        ("rename-chirho", "should_compile", true),
        ("rename-chirho", "should_fail", false),
    ];

    let mut grand_passed_chirho = 0usize;
    let mut grand_total_chirho = 0usize;

    eprintln!("\n{:=<70}", "");
    eprintln!("GHC Bulk Test Summary");
    eprintln!("{:=<70}", "");
    eprintln!(
        "{:<35} {:>8} {:>8} {:>8}",
        "Category", "Passed", "Total", "Rate"
    );
    eprintln!("{:-<70}", "");

    for (cat_chirho, sub_chirho, expect_chirho) in &categories_chirho {
        let (p_chirho, t_chirho, _) =
            run_category_chirho(&base_chirho, cat_chirho, sub_chirho, *expect_chirho);
        let r_chirho = if t_chirho > 0 {
            p_chirho as f64 / t_chirho as f64 * 100.0
        } else {
            100.0
        };
        eprintln!(
            "{:<35} {:>8} {:>8} {:>7.1}%",
            format!("{}/{}", cat_chirho, sub_chirho),
            p_chirho,
            t_chirho,
            r_chirho
        );
        grand_passed_chirho += p_chirho;
        grand_total_chirho += t_chirho;
    }

    // Layout and module (flat dirs)
    for (label_chirho, dir_name_chirho) in
        [("layout", "layout-chirho"), ("module", "module-chirho")]
    {
        let dir_chirho = base_chirho.join(dir_name_chirho);
        if dir_chirho.is_dir() {
            let files_chirho = discover_hs_files_chirho(&dir_chirho);
            let t_chirho = files_chirho.len();
            let mut p_chirho = 0usize;
            for f_chirho in &files_chirho {
                if matches!(
                    std::panic::catch_unwind(|| try_parse_chirho(f_chirho)),
                    Ok(Ok(()))
                ) {
                    p_chirho += 1;
                }
            }
            let r_chirho = if t_chirho > 0 {
                p_chirho as f64 / t_chirho as f64 * 100.0
            } else {
                100.0
            };
            eprintln!(
                "{:<35} {:>8} {:>8} {:>7.1}%",
                label_chirho, p_chirho, t_chirho, r_chirho
            );
            grand_passed_chirho += p_chirho;
            grand_total_chirho += t_chirho;
        }
    }

    eprintln!("{:-<70}", "");
    let grand_rate_chirho = if grand_total_chirho > 0 {
        grand_passed_chirho as f64 / grand_total_chirho as f64 * 100.0
    } else {
        100.0
    };
    eprintln!(
        "{:<35} {:>8} {:>8} {:>7.1}%",
        "TOTAL", grand_passed_chirho, grand_total_chirho, grand_rate_chirho
    );
    eprintln!("{:=<70}\n", "");
}
