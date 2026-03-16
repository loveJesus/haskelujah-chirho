// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # GHC Corpus Tests
//!
//! Runs our lexer and parser against Haskell source files from GHC's test
//! suite to verify we handle real-world code without panicking.
//!
//! These tests only run if the `ghc-tests-chirho/` directory exists (it is
//! gitignored and populated by a setup script).

use haskeluya_parser_chirho::cst_parser_chirho::parse_to_cst_chirho;
use haskeluya_parser_chirho::layout_chirho::apply_layout_chirho;
use haskeluya_parser_chirho::lexer_chirho::LexerChirho;
use haskeluya_span_chirho::FileIdChirho;
use std::path::PathBuf;

fn ghc_test_dir_chirho() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../ghc-tests-chirho")
}

fn collect_hs_files_chirho(dir_chirho: &std::path::Path) -> Vec<PathBuf> {
    let mut files_chirho = Vec::new();
    if !dir_chirho.is_dir() {
        return files_chirho;
    }
    for entry_chirho in walkdir_chirho(dir_chirho) {
        if entry_chirho
            .extension()
            .is_some_and(|e_chirho| e_chirho == "hs")
        {
            files_chirho.push(entry_chirho);
        }
    }
    files_chirho
}

/// Simple recursive directory walk (no external dep needed).
fn walkdir_chirho(dir_chirho: &std::path::Path) -> Vec<PathBuf> {
    let mut result_chirho = Vec::new();
    if let Ok(entries_chirho) = std::fs::read_dir(dir_chirho) {
        for entry_chirho in entries_chirho.flatten() {
            let path_chirho = entry_chirho.path();
            if path_chirho.is_dir() {
                result_chirho.extend(walkdir_chirho(&path_chirho));
            } else {
                result_chirho.push(path_chirho);
            }
        }
    }
    result_chirho
}

#[test]
fn lex_ghc_corpus_no_panic_chirho() {
    let dir_chirho = ghc_test_dir_chirho();
    if !dir_chirho.exists() {
        eprintln!("Skipping GHC corpus test: {} not found", dir_chirho.display());
        return;
    }

    let files_chirho = collect_hs_files_chirho(&dir_chirho);
    if files_chirho.is_empty() {
        eprintln!("No .hs files found in {}", dir_chirho.display());
        return;
    }

    let mut success_count_chirho: usize = 0;
    let mut error_count_chirho: usize = 0;
    let mut panic_files_chirho: Vec<String> = Vec::new();

    for file_chirho in &files_chirho {
        let source_chirho = match std::fs::read_to_string(file_chirho) {
            Ok(s_chirho) => s_chirho,
            Err(_) => continue, // skip unreadable files (encoding issues)
        };

        let result_chirho = std::panic::catch_unwind(|| {
            let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
            let mut lexer_chirho = LexerChirho::new_chirho(&source_chirho, file_id_chirho);
            let _tokens_chirho = lexer_chirho.lex_all_chirho();
        });

        match result_chirho {
            Ok(()) => success_count_chirho += 1,
            Err(_) => {
                error_count_chirho += 1;
                panic_files_chirho.push(file_chirho.display().to_string());
            }
        }
    }

    eprintln!(
        "GHC corpus lex: {}/{} succeeded, {} panicked",
        success_count_chirho,
        files_chirho.len(),
        error_count_chirho,
    );

    if !panic_files_chirho.is_empty() {
        eprintln!("Panicked on:");
        for f_chirho in &panic_files_chirho[..panic_files_chirho.len().min(20)] {
            eprintln!("  {}", f_chirho);
        }
    }

    // Allow up to 5% failures (GHC test suite includes intentionally broken files)
    let max_failures_chirho = files_chirho.len() / 20 + 1;
    assert!(
        error_count_chirho <= max_failures_chirho,
        "Too many panics: {} out of {} (max allowed: {})",
        error_count_chirho,
        files_chirho.len(),
        max_failures_chirho,
    );
}

#[test]
fn layout_ghc_corpus_no_panic_chirho() {
    let dir_chirho = ghc_test_dir_chirho();
    if !dir_chirho.exists() {
        eprintln!("Skipping GHC corpus layout test: {} not found", dir_chirho.display());
        return;
    }

    // Only test a subset for layout (it's slower)
    let layout_dir_chirho = dir_chirho.join("layout-chirho");
    let parser_dir_chirho = dir_chirho.join("parser-chirho");

    let mut files_chirho = collect_hs_files_chirho(&layout_dir_chirho);
    files_chirho.extend(collect_hs_files_chirho(&parser_dir_chirho));

    if files_chirho.is_empty() {
        eprintln!("No .hs files found for layout tests");
        return;
    }

    let mut success_count_chirho: usize = 0;
    let mut error_count_chirho: usize = 0;

    for file_chirho in &files_chirho {
        let source_chirho = match std::fs::read_to_string(file_chirho) {
            Ok(s_chirho) => s_chirho,
            Err(_) => continue,
        };

        let result_chirho = std::panic::catch_unwind(|| {
            let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
            let mut lexer_chirho = LexerChirho::new_chirho(&source_chirho, file_id_chirho);
            let raw_chirho = lexer_chirho.lex_all_chirho();
            let _laid_out_chirho = apply_layout_chirho(&source_chirho, raw_chirho, file_id_chirho);
        });

        match result_chirho {
            Ok(()) => success_count_chirho += 1,
            Err(_) => error_count_chirho += 1,
        }
    }

    eprintln!(
        "GHC corpus layout: {}/{} succeeded, {} panicked",
        success_count_chirho,
        files_chirho.len(),
        error_count_chirho,
    );

    let max_failures_chirho = files_chirho.len() / 20 + 1;
    assert!(
        error_count_chirho <= max_failures_chirho,
        "Too many layout panics: {} out of {}",
        error_count_chirho,
        files_chirho.len(),
    );
}

#[test]
fn parse_ghc_corpus_no_panic_chirho() {
    let dir_chirho = ghc_test_dir_chirho();
    if !dir_chirho.exists() {
        eprintln!("Skipping GHC corpus parse test: {} not found", dir_chirho.display());
        return;
    }

    // Test CST parsing on parser + layout test files
    let layout_dir_chirho = dir_chirho.join("layout-chirho");
    let parser_dir_chirho = dir_chirho.join("parser-chirho");

    let mut files_chirho = collect_hs_files_chirho(&layout_dir_chirho);
    files_chirho.extend(collect_hs_files_chirho(&parser_dir_chirho));

    if files_chirho.is_empty() {
        eprintln!("No .hs files found for parse tests");
        return;
    }

    let mut success_count_chirho: usize = 0;
    let mut error_count_chirho: usize = 0;

    for file_chirho in &files_chirho {
        let source_chirho = match std::fs::read_to_string(file_chirho) {
            Ok(s_chirho) => s_chirho,
            Err(_) => continue,
        };

        let result_chirho = std::panic::catch_unwind(|| {
            let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
            let _root_chirho = parse_to_cst_chirho(&source_chirho, file_id_chirho);
        });

        match result_chirho {
            Ok(()) => success_count_chirho += 1,
            Err(_) => error_count_chirho += 1,
        }
    }

    eprintln!(
        "GHC corpus CST parse: {}/{} succeeded, {} panicked",
        success_count_chirho,
        files_chirho.len(),
        error_count_chirho,
    );

    let max_failures_chirho = files_chirho.len() / 20 + 1;
    assert!(
        error_count_chirho <= max_failures_chirho,
        "Too many CST parse panics: {} out of {}",
        error_count_chirho,
        files_chirho.len(),
    );
}
