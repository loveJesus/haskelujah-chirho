// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Recovered CST syntax remains lossless but cannot be compiled as a value.

use super::*;

#[test]
fn checked_bindings_require_a_rhs_separator_chirho() {
    for source_chirho in [
        "module MChirho where\nJust xChirho -> xChirho\n",
        "module MChirho where\nfChirho xChirho -> xChirho\n",
    ] {
        let lossless_chirho = parse_to_cst_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
        assert_eq!(lossless_chirho.text_len_chirho(), source_chirho.len());
        let error_chirho = ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
            .parse_checked_chirho()
            .expect_err("a missing binding RHS is a syntax error");
        assert!(
            error_chirho
                .diagnostics_chirho()
                .iter()
                .any(|diagnostic_chirho| diagnostic_chirho.to_string().contains("error[E0002]"))
        );
    }
}

#[test]
fn checked_parsing_preserves_signatures_guards_and_empty_cases_chirho() {
    for source_chirho in [
        "module MChirho where\nfChirho :: Int\nfChirho = 1\n",
        "module MChirho where\nJust xChirho | True = Just 1\n",
        "{-# LANGUAGE EmptyCase #-}\nmodule MChirho where\nfChirho xChirho = case xChirho of {}\n",
    ] {
        ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
            .parse_checked_chirho()
            .expect("ordinary valid syntax still parses");
    }
}

#[test]
fn checked_parse_refuses_an_existing_expression_error_node_chirho() {
    let source_chirho = "module MChirho where\nfChirho = )\n";
    assert!(
        ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO)
            .parse_checked_chirho()
            .is_err()
    );
}
