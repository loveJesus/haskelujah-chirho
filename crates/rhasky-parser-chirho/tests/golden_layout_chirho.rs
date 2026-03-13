// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Golden tests for the layout rule pass.
//!
//! Each `.hs` file under `tests-chirho/golden-chirho/lex-chirho/` is lexed,
//! passed through the layout rule, and the resulting token stream (non-trivia
//! only) is compared against a sibling `.layout.expected` file.
//! Run with `BLESS_CHIRHO=1` to regenerate expected files.

use rhasky_parser_chirho::layout_chirho::apply_layout_chirho;
use rhasky_parser_chirho::lexer_chirho::LexerChirho;
use rhasky_span_chirho::FileIdChirho;
use rhasky_test_harness_chirho::{
    assert_golden_chirho, bless_golden_chirho, discover_golden_tests_chirho,
};
use std::path::PathBuf;

fn golden_dir_chirho() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests-chirho/golden-chirho/lex-chirho")
}

fn render_layout_tokens_chirho(source_chirho: &str) -> String {
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, file_id_chirho);
    let raw_chirho = lexer_chirho.lex_all_chirho();
    let laid_out_chirho = apply_layout_chirho(source_chirho, raw_chirho, file_id_chirho);

    let mut output_chirho = String::new();
    for tok_chirho in &laid_out_chirho {
        // Skip trivia for cleaner output
        if tok_chirho.kind_chirho.is_trivia_chirho() {
            continue;
        }

        let start_chirho = tok_chirho.span_chirho.start_chirho().as_usize_chirho();
        let end_chirho = tok_chirho.span_chirho.end_chirho().as_usize_chirho();
        let text_chirho = if start_chirho < end_chirho && end_chirho <= source_chirho.len() {
            let raw_chirho = &source_chirho[start_chirho..end_chirho];
            raw_chirho
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
        } else {
            String::new() // virtual tokens have zero-width spans
        };

        output_chirho.push_str(&format!(
            "{:>4}..{:<4} {:?} {:?}\n",
            start_chirho, end_chirho, tok_chirho.kind_chirho, text_chirho,
        ));
    }
    output_chirho
}

#[test]
fn golden_layout_all_chirho() {
    let bless_chirho = std::env::var("BLESS_CHIRHO").is_ok();
    let dir_chirho = golden_dir_chirho();
    let cases_chirho =
        discover_golden_tests_chirho(&dir_chirho).expect("should discover golden lex tests");

    assert!(
        !cases_chirho.is_empty(),
        "No golden layout tests found in {}",
        dir_chirho.display()
    );

    let mut failures_chirho: Vec<String> = Vec::new();

    for case_chirho in &cases_chirho {
        let source_chirho =
            std::fs::read_to_string(&case_chirho.input_path_chirho).expect("read .hs file");
        let actual_chirho = render_layout_tokens_chirho(&source_chirho);

        // Use .layout.expected instead of .expected to avoid conflicting
        // with the raw lex golden tests.
        let layout_expected_path_chirho = case_chirho
            .input_path_chirho
            .with_extension("layout.expected");

        if bless_chirho {
            bless_golden_chirho(&actual_chirho, &layout_expected_path_chirho)
                .expect("bless golden");
        } else if let Err(mismatch_chirho) =
            assert_golden_chirho(&actual_chirho, &layout_expected_path_chirho)
        {
            failures_chirho.push(format!(
                "--- {} ---\n{}",
                case_chirho.name_chirho, mismatch_chirho
            ));
        }
    }

    if !failures_chirho.is_empty() {
        panic!(
            "{} golden layout test(s) failed:\n\n{}",
            failures_chirho.len(),
            failures_chirho.join("\n\n")
        );
    }
}
