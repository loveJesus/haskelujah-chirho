// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Edition defaults and explicit extension choices must agree before layout and after lowering.

use haskelujah_parser::cst_parser_chirho::parse_to_cst_chirho;
use haskelujah_parser::lexer_chirho::LexerChirho;
use haskelujah_parser::lower_chirho::lower_module_chirho;
use haskelujah_parser::pragma_chirho::{
    collect_raw_pragma_extensions_chirho, extension_enabled_chirho,
};
use haskelujah_span_chirho::FileIdChirho;

fn assert_constraint_kinds_chirho(pragmas_chirho: &str, expected_chirho: bool) {
    let source_chirho = format!("{pragmas_chirho}\nmodule EditionChirho where\n");
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let tokens_chirho = LexerChirho::new_chirho(&source_chirho, file_chirho).lex_all_chirho();
    let raw_extensions_chirho =
        collect_raw_pragma_extensions_chirho(&source_chirho, &tokens_chirho);
    let module_chirho = lower_module_chirho(
        &parse_to_cst_chirho(&source_chirho, file_chirho),
        file_chirho,
    );
    for extensions_chirho in [&raw_extensions_chirho, &module_chirho.extensions_chirho] {
        assert_eq!(
            extension_enabled_chirho(extensions_chirho, "ConstraintKinds"),
            expected_chirho,
            "{pragmas_chirho}: {extensions_chirho:?}"
        );
    }
}

#[test]
fn explicit_extensions_override_edition_defaults_across_pragmas_chirho() {
    for pragmas_chirho in [
        "{-# LANGUAGE NoConstraintKinds, GHC2021 #-}",
        "{-# LANGUAGE NoConstraintKinds #-}\n{-# LANGUAGE GHC2021 #-}",
        "{-# LANGUAGE GHC2021, ConstraintKinds, NoConstraintKinds #-}",
    ] {
        assert_constraint_kinds_chirho(pragmas_chirho, false);
    }
    for pragmas_chirho in [
        "{-# LANGUAGE ConstraintKinds, Haskell2010 #-}",
        "{-# LANGUAGE ConstraintKinds #-}\n{-# LANGUAGE Haskell2010 #-}",
        "{-# LANGUAGE NoConstraintKinds, GHC2021, ConstraintKinds #-}",
    ] {
        assert_constraint_kinds_chirho(pragmas_chirho, true);
    }
}

#[test]
fn only_the_selected_edition_supplies_defaults_chirho() {
    assert_constraint_kinds_chirho("{-# LANGUAGE GHC2021, Haskell2010 #-}", false);
    assert_constraint_kinds_chirho("{-# LANGUAGE Haskell2010, GHC2021 #-}", true);
}

#[test]
fn options_editions_and_language_editions_use_the_same_rules_chirho() {
    assert_constraint_kinds_chirho("{-# OPTIONS_GHC -XGHC2021 #-}", true);
    assert_constraint_kinds_chirho(
        "{-# LANGUAGE NoConstraintKinds #-}\n{-# OPTIONS_GHC -XGHC2021 #-}",
        false,
    );
    assert_constraint_kinds_chirho(
        "{-# OPTIONS_GHC -XConstraintKinds #-}\n{-# LANGUAGE Haskell2010 #-}",
        true,
    );
}
