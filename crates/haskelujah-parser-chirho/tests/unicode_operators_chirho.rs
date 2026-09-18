// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Operator identity is a lexical contract, including its full UTF-8 spelling.
use haskelujah_parser::lexer_chirho::{LexerChirho, RawTokenKindChirho};
use haskelujah_span_chirho::FileIdChirho;

#[test]
fn graphic_brackets_and_quotes_are_not_operators_chirho() {
    for source_chirho in ["⟨", "⟩", "（", "）", "「", "」", "‘", "’"] {
        let token_chirho = LexerChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
            .next_token_chirho();
        assert_eq!(
            token_chirho.kind_chirho,
            RawTokenKindChirho::ErrorChirho,
            "{source_chirho}"
        );
    }
}

#[test]
fn incomplete_unicode_quotes_never_panic_chirho() {
    // Minimized by the full parser property test during the scanner repair.
    for source_chirho in ["'🌀", "'⊗", "'×", "'λ"] {
        let tree_chirho = haskelujah_parser::cst_parser_chirho::parse_to_cst_chirho(
            source_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
        );
        let _module_chirho = haskelujah_parser::lower_chirho::lower_module_chirho(
            &tree_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
        );
    }
}

#[test]
fn mixed_unicode_operators_keep_one_identity_chirho() {
    let mut failures_chirho = Vec::new();
    for (source_chirho, expected_chirho) in [
        (":⊗:", RawTokenKindChirho::ConSymChirho),
        (":×:", RawTokenKindChirho::ConSymChirho),
        ("⊗+", RawTokenKindChirho::VarSymChirho),
        ("+⊗", RawTokenKindChirho::VarSymChirho),
        ("⊗⊕", RawTokenKindChirho::VarSymChirho),
        ("MChirho.:⊗:", RawTokenKindChirho::QualifiedIdChirho),
        ("MChirho.⊗+", RawTokenKindChirho::QualifiedIdChirho),
        ("->⊗", RawTokenKindChirho::VarSymChirho),
        ("=⊗", RawTokenKindChirho::VarSymChirho),
        ("--⊗", RawTokenKindChirho::VarSymChirho),
        ("→⊗", RawTokenKindChirho::VarSymChirho),
        ("⊗→", RawTokenKindChirho::VarSymChirho),
        ("∀→", RawTokenKindChirho::VarSymChirho),
        ("MChirho.⊗.+", RawTokenKindChirho::QualifiedIdChirho),
        ("MChirho..⊗", RawTokenKindChirho::QualifiedIdChirho),
        (":^:", RawTokenKindChirho::ConSymChirho),
    ] {
        let tokens_chirho =
            LexerChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).lex_all_chirho();
        let actual_chirho: Vec<_> = tokens_chirho
            .iter()
            .filter(|token_chirho| token_chirho.kind_chirho != RawTokenKindChirho::EofChirho)
            .map(|token_chirho| {
                let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
                let end_chirho = token_chirho.span_chirho.end_chirho().as_usize_chirho();
                (
                    token_chirho.kind_chirho,
                    &source_chirho[start_chirho..end_chirho],
                )
            })
            .collect();
        if actual_chirho != [(expected_chirho, source_chirho)] {
            failures_chirho.push(format!("{source_chirho:?}: {actual_chirho:?}"));
        }
    }
    assert!(failures_chirho.is_empty(), "{}", failures_chirho.join("\n"));
}
