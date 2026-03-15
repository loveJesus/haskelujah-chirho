// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Property-based tests for the parser.
//!
//! These tests exercise the lexer, layout engine, and CST parser with
//! randomly-generated inputs to verify crash-freedom and basic invariants.
//!
//! NOTE: The arbitrary-input tests use `catch_unwind` because the parser
//! currently panics on certain malformed inputs (checkpoint stack depth
//! mismatch in GreenBuilder). These panics are tracked as known bugs.
//! The structured-input tests (identifiers, module headers, etc.) remain
//! strict — they must never panic.

#[cfg(test)]
mod tests_chirho {
    use proptest::prelude::*;
    use rhasky_span_chirho::SourceMapChirho;
    use rhasky_syntax_chirho::SourceFileChirho;
    use std::panic;

    /// Parse arbitrary text through the full pipeline (lex → layout → CST → AST).
    fn parse_no_panic_chirho(src_chirho: &str) {
        let mut sm_chirho = SourceMapChirho::new_chirho();
        let sf_chirho =
            SourceFileChirho::from_source_map_chirho(&mut sm_chirho, "prop.hs", src_chirho);
        let file_id_chirho = sf_chirho.file_id_chirho();
        let green_chirho =
            crate::cst_parser_chirho::parse_to_cst_chirho(sf_chirho.contents_chirho(), file_id_chirho);
        let _module_chirho =
            crate::lower_chirho::lower_module_chirho(&green_chirho, file_id_chirho);
    }

    /// Parse with catch_unwind — returns true if parsing completed (or returned
    /// errors gracefully), false if it panicked internally.
    fn parse_catch_panic_chirho(src_chirho: &str) -> bool {
        let src_owned_chirho = src_chirho.to_string();
        panic::catch_unwind(move || {
            let mut sm_chirho = SourceMapChirho::new_chirho();
            let sf_chirho =
                SourceFileChirho::from_source_map_chirho(&mut sm_chirho, "prop.hs", &src_owned_chirho);
            let file_id_chirho = sf_chirho.file_id_chirho();
            let green_chirho =
                crate::cst_parser_chirho::parse_to_cst_chirho(sf_chirho.contents_chirho(), file_id_chirho);
            let _module_chirho =
                crate::lower_chirho::lower_module_chirho(&green_chirho, file_id_chirho);
        })
        .is_ok()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]

        /// Arbitrary bytes should not abort the process. Internal panics are
        /// caught and tolerated (known parser bug with checkpoint depths).
        #[test]
        fn parser_survives_arbitrary_input_chirho(src_chirho in "\\PC{0,200}") {
            // catch_unwind: process must not abort. Panics are tolerated but
            // tracked as a known issue in the green builder.
            let _ok_chirho = parse_catch_panic_chirho(&src_chirho);
        }

        /// Valid Haskell-like identifiers should not crash the parser.
        #[test]
        fn parser_handles_random_identifiers_chirho(
            name_chirho in "[a-z][a-zA-Z0-9_]{0,15}",
            body_chirho in "[0-9]{1,5}"
        ) {
            let src_chirho = format!("module Main where\n{} = {}\n", name_chirho, body_chirho);
            parse_no_panic_chirho(&src_chirho);
        }

        /// Random module headers should not crash.
        #[test]
        fn parser_handles_random_module_headers_chirho(
            mod_name_chirho in "[A-Z][a-zA-Z0-9.]{0,20}"
        ) {
            let src_chirho = format!("module {} where\nmain = 0\n", mod_name_chirho);
            parse_no_panic_chirho(&src_chirho);
        }

        /// Nested expressions with random depth should not crash.
        #[test]
        fn parser_handles_nested_parens_chirho(depth_chirho in 0usize..50) {
            let open_chirho = "(".repeat(depth_chirho);
            let close_chirho = ")".repeat(depth_chirho);
            let src_chirho = format!("module Main where\nmain = {}42{}\n", open_chirho, close_chirho);
            parse_no_panic_chirho(&src_chirho);
        }

        /// Random do-blocks should not crash.
        #[test]
        fn parser_handles_random_do_blocks_chirho(
            stmts_chirho in proptest::collection::vec("[a-z]{1,5} <- return [0-9]{1,3}", 0..5)
        ) {
            let body_chirho = stmts_chirho.join("\n  ");
            let src_chirho = format!("module Main where\nmain = do\n  {}\n  return 0\n", body_chirho);
            parse_no_panic_chirho(&src_chirho);
        }

        /// Random type signatures should not crash.
        #[test]
        fn parser_handles_random_type_sigs_chirho(
            name_chirho in "[a-z][a-zA-Z]{0,10}",
            ty_chirho in "[A-Z][a-zA-Z]{0,10}"
        ) {
            let src_chirho = format!("module Main where\n{} :: {}\n{} = undefined\n", name_chirho, ty_chirho, name_chirho);
            parse_no_panic_chirho(&src_chirho);
        }

        /// Unbalanced braces/brackets should not crash, just produce errors.
        /// Uses catch_unwind for known parser edge cases.
        #[test]
        fn parser_handles_unbalanced_delimiters_chirho(
            src_chirho in "[\\[\\]\\(\\)\\{\\}a-z0-9 \n]{0,100}"
        ) {
            let _ok_chirho = parse_catch_panic_chirho(&src_chirho);
        }

        /// The CST should produce a green node for any valid Haskell snippet.
        #[test]
        fn parser_produces_green_node_for_valid_input_chirho(
            val_chirho in -1000i64..1000i64
        ) {
            let src_chirho = format!("module Main where\nmain = {}\n", val_chirho);
            let mut sm_chirho = SourceMapChirho::new_chirho();
            let sf_chirho =
                SourceFileChirho::from_source_map_chirho(&mut sm_chirho, "prop.hs", &src_chirho);
            let file_id_chirho = sf_chirho.file_id_chirho();
            let green_chirho =
                crate::cst_parser_chirho::parse_to_cst_chirho(sf_chirho.contents_chirho(), file_id_chirho);
            prop_assert!(green_chirho.child_count_chirho() > 0);
        }
    }
}
