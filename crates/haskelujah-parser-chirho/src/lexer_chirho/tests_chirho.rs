// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use super::*;

fn lex_chirho(source_chirho: &str) -> Vec<RawTokenChirho> {
    let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
    lexer_chirho.lex_all_chirho()
}

fn non_trivia_kinds_chirho(source_chirho: &str) -> Vec<RawTokenKindChirho> {
    lex_chirho(source_chirho)
        .into_iter()
        .filter(|t_chirho| !t_chirho.kind_chirho.is_trivia_chirho())
        .map(|t_chirho| t_chirho.kind_chirho)
        .collect()
}

#[test]
fn lex_module_header_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("module Main where");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::ModuleChirho,
            RawTokenKindChirho::ConIdChirho,
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_tilde_and_at_prefixed_symbolic_operators_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("(~:) (@?)");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::VarSymChirho,
            RawTokenKindChirho::RightParenChirho,
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::VarSymChirho,
            RawTokenKindChirho::RightParenChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_integer_literals_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("42 0xFF 0o77 0b1010");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_float_literals_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("3.14 1.0e10 2.5E-3");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::FloatLitChirho,
            RawTokenKindChirho::FloatLitChirho,
            RawTokenKindChirho::FloatLitChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_numeric_underscores_chirho() {
    // NumericUnderscores: underscores in decimal, hex, octal, binary, float
    let kinds_chirho = non_trivia_kinds_chirho("1_000_000 0xFF_FF 0o7_7 0b10_10 3.14_15");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::IntLitChirho,
            RawTokenKindChirho::FloatLitChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_string_with_gap_chirho() {
    let tokens_chirho = lex_chirho("\"hello \\    \\world\"");
    let string_tokens_chirho: Vec<_> = tokens_chirho
        .iter()
        .filter(|t_chirho| t_chirho.kind_chirho == RawTokenKindChirho::StringLitChirho)
        .collect();
    assert_eq!(string_tokens_chirho.len(), 1);

    let text_chirho = &"\"hello \\    \\world\""[string_tokens_chirho[0]
        .span_chirho
        .start_chirho()
        .as_usize_chirho()
        ..string_tokens_chirho[0]
            .span_chirho
            .end_chirho()
            .as_usize_chirho()];
    assert_eq!(text_chirho, "\"hello \\    \\world\"");
}

#[test]
fn lex_nested_block_comment_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("{- outer {- inner -} still outer -} x");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_operators_and_special_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho(":: -> <- => .. = | \\ @ ~");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::ColonColonChirho,
            RawTokenKindChirho::RightArrowChirho,
            RawTokenKindChirho::LeftArrowChirho,
            RawTokenKindChirho::FatArrowChirho,
            RawTokenKindChirho::DotDotChirho,
            RawTokenKindChirho::EqualsChirho,
            RawTokenKindChirho::PipeChirho,
            RawTokenKindChirho::BackslashChirho,
            RawTokenKindChirho::AtChirho,
            RawTokenKindChirho::TildeChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_double_backslash_operator_chirho() {
    let source_chirho = "xs \\\\ ys";
    let tokens_chirho = lex_chirho(source_chirho);
    let op_token_chirho = tokens_chirho
        .iter()
        .find(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::VarSymChirho)
        .expect("expected double backslash operator token");
    let text_chirho = &source_chirho[op_token_chirho.span_chirho.start_chirho().as_usize_chirho()
        ..op_token_chirho.span_chirho.end_chirho().as_usize_chirho()];
    assert_eq!(text_chirho, "\\\\");

    let kinds_chirho = non_trivia_kinds_chirho(source_chirho);
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::VarSymChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_lambda_backslash_still_lambda_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("\\x -> x");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::BackslashChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::RightArrowChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_keywords_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("if then else case of let in do where");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::IfChirho,
            RawTokenKindChirho::ThenChirho,
            RawTokenKindChirho::ElseChirho,
            RawTokenKindChirho::CaseChirho,
            RawTokenKindChirho::OfChirho,
            RawTokenKindChirho::LetChirho,
            RawTokenKindChirho::InChirho,
            RawTokenKindChirho::DoChirho,
            RawTokenKindChirho::WhereChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_recursive_do_words_as_plain_identifiers_chirho() {
    assert_eq!(
        non_trivia_kinds_chirho("mdo rec"),
        vec![
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_char_literals_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("'a' '\\n' '\\\\' '0'");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_punctuation_char_literals_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("'(' '[' ',' ')'");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_extended_char_escape_literals_chirho() {
    let tokens_chirho = lex_chirho("'\\026' '\\BS' '\\DEL' '\\x41' '\\o101'");
    let char_tokens_chirho: Vec<_> = tokens_chirho
        .iter()
        .filter(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::CharLitChirho)
        .collect();
    assert_eq!(char_tokens_chirho.len(), 5);

    let source_chirho = "'\\026' '\\BS' '\\DEL' '\\x41' '\\o101'";
    let texts_chirho: Vec<&str> = char_tokens_chirho
        .iter()
        .map(|token_chirho| {
            &source_chirho[token_chirho.span_chirho.start_chirho().as_usize_chirho()
                ..token_chirho.span_chirho.end_chirho().as_usize_chirho()]
        })
        .collect();
    assert_eq!(
        texts_chirho,
        vec!["'\\026'", "'\\BS'", "'\\DEL'", "'\\x41'", "'\\o101'"]
    );
}

#[test]
fn lex_constructor_operator_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho(":+: :*:");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::ConSymChirho,
            RawTokenKindChirho::ConSymChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_punctuation_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("( ) [ ] { } , ;");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::RightParenChirho,
            RawTokenKindChirho::LeftBracketChirho,
            RawTokenKindChirho::RightBracketChirho,
            RawTokenKindChirho::LeftBraceChirho,
            RawTokenKindChirho::RightBraceChirho,
            RawTokenKindChirho::CommaChirho,
            RawTokenKindChirho::SemicolonChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_doc_comment_chirho() {
    let tokens_chirho = lex_chirho("-- | doc comment\n-- regular comment");
    let doc_count_chirho = tokens_chirho
        .iter()
        .filter(|t_chirho| t_chirho.kind_chirho == RawTokenKindChirho::DocCommentChirho)
        .count();
    let line_count_chirho = tokens_chirho
        .iter()
        .filter(|t_chirho| t_chirho.kind_chirho == RawTokenKindChirho::LineCommentChirho)
        .count();
    assert_eq!(doc_count_chirho, 1);
    assert_eq!(line_count_chirho, 1);
}

#[test]
fn lex_hyphen_separator_line_as_comment_chirho() {
    let tokens_chirho = lex_chirho(
        "-----------------------------------------------------------------------------\nmodule Demo where\n",
    );
    assert_eq!(
        tokens_chirho[0].kind_chirho,
        RawTokenKindChirho::LineCommentChirho
    );
    assert!(
        tokens_chirho
            .iter()
            .any(|token_chirho| token_chirho.kind_chirho == RawTokenKindChirho::ModuleChirho)
    );
}

#[test]
fn lex_wildcard_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("_ _x");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::UnderscoreChirho,
            RawTokenKindChirho::VarIdChirho, // _x is an identifier, not wildcard
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_type_signature_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("main :: IO ()");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::ColonColonChirho,
            RawTokenKindChirho::ConIdChirho,
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::RightParenChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

// ── DataKinds lexer tests ──────────────────────────────────────
#[test]
fn lex_promoted_constructor_chirho() {
    // 'True should be Tick + ConId, not a char literal
    let kinds_chirho = non_trivia_kinds_chirho("'True");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::ConIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_char_vs_promoted_chirho() {
    // 'A' is a char literal (uppercase but followed by closing ')
    let kinds_chirho = non_trivia_kinds_chirho("'A'");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_promoted_list_chirho() {
    // '[Int, Bool] should start with Tick + [
    let kinds_chirho = non_trivia_kinds_chirho("'[");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::LeftBracketChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_promoted_cons_symbol_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("':");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::ConSymChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );

    let char_kinds_chirho = non_trivia_kinds_chirho("':'");
    assert_eq!(
        char_kinds_chirho,
        vec![
            RawTokenKindChirho::CharLitChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_promoted_nothing_chirho() {
    // 'Nothing is a promoted constructor
    let kinds_chirho = non_trivia_kinds_chirho("'Nothing");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::ConIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn spans_are_correct_chirho() {
    let source_chirho = "module Main where";
    let tokens_chirho = lex_chirho(source_chirho);
    let module_token_chirho = &tokens_chirho[0];
    assert_eq!(
        module_token_chirho.kind_chirho,
        RawTokenKindChirho::ModuleChirho
    );
    let text_chirho = module_token_chirho
        .span_chirho
        .text_chirho(source_chirho)
        .unwrap();
    assert_eq!(text_chirho, "module");
}

#[test]
fn lex_th_splice_chirho() {
    // $foo should be ThSplice + VarId
    let kinds_chirho = non_trivia_kinds_chirho("$foo");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::ThSpliceChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_th_splice_parens_chirho() {
    // $(expr) should be ThSplice + ( ...
    let kinds_chirho = non_trivia_kinds_chirho("$(foo)");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::ThSpliceChirho,
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::RightParenChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_dollar_operator_chirho() {
    // f $ x — dollar is an operator, not a splice
    let kinds_chirho = non_trivia_kinds_chirho("f $ x");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::VarSymChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_th_typed_splice_chirho() {
    // $$foo should be ThTypedSplice + VarId
    let kinds_chirho = non_trivia_kinds_chirho("$$foo");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::ThTypedSpliceChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_th_expression_quote_chirho() {
    // [| expr |] should produce open/close quote tokens
    let kinds_chirho = non_trivia_kinds_chirho("[| foo |]");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::ThOpenExpQuoteChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::ThCloseQuoteChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_th_dec_quote_chirho() {
    // [d| ... |]
    let kinds_chirho = non_trivia_kinds_chirho("[d| x = 1 |]");
    assert_eq!(kinds_chirho[0], RawTokenKindChirho::ThOpenDecQuoteChirho);
    assert_eq!(
        kinds_chirho[kinds_chirho.len() - 2],
        RawTokenKindChirho::ThCloseQuoteChirho
    );
}

#[test]
fn lex_th_type_quote_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("[t| Int -> Bool |]");
    assert_eq!(kinds_chirho[0], RawTokenKindChirho::ThOpenTypeQuoteChirho);
}

#[test]
fn lex_th_pat_quote_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("[p| (x, y) |]");
    assert_eq!(kinds_chirho[0], RawTokenKindChirho::ThOpenPatQuoteChirho);
}

#[test]
fn lex_normal_list_unaffected_chirho() {
    // [1, 2, 3] should not be affected by TH quote lexing
    let kinds_chirho = non_trivia_kinds_chirho("[1, 2, 3]");
    assert_eq!(kinds_chirho[0], RawTokenKindChirho::LeftBracketChirho);
}

#[test]
fn lex_magic_hash_identifiers_stay_single_tokens_chirho() {
    let source_chirho = "module T where\nfooChirho :: Int# -> Int#\nfooChirho xChirho = newPinnedByteArray# xChirho\n";
    let kinds_chirho = non_trivia_kinds_chirho(source_chirho);
    assert!(kinds_chirho.contains(&RawTokenKindChirho::ConIdChirho));
    assert!(kinds_chirho.contains(&RawTokenKindChirho::VarIdChirho));

    let tokens_chirho = lex_chirho(source_chirho);
    let texts_chirho: Vec<_> = tokens_chirho
        .iter()
        .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
        .map(|token_chirho| {
            &source_chirho[token_chirho.span_chirho.start_chirho().as_usize_chirho()
                ..token_chirho.span_chirho.end_chirho().as_usize_chirho()]
        })
        .collect();
    assert!(texts_chirho.contains(&"Int#"));
    assert!(texts_chirho.contains(&"newPinnedByteArray#"));
    assert!(!texts_chirho.contains(&"#"));
}

#[test]
fn lex_magic_hash_identifier_before_plain_rparen_inside_unboxed_tuple_stays_single_token_chirho() {
    let source_chirho = "module T where\nfChirho = (# MutableByteArray (unsafeCoerce# arr#) #)\n";
    let tokens_chirho = lex_chirho(source_chirho);
    let texts_chirho: Vec<_> = tokens_chirho
        .iter()
        .filter(|token_chirho| !token_chirho.kind_chirho.is_trivia_chirho())
        .map(|token_chirho| {
            &source_chirho[token_chirho.span_chirho.start_chirho().as_usize_chirho()
                ..token_chirho.span_chirho.end_chirho().as_usize_chirho()]
        })
        .collect();
    assert!(
        texts_chirho.contains(&"unsafeCoerce#"),
        "expected unsafeCoerce# to stay intact, got {:?}",
        texts_chirho
    );
    assert!(
        texts_chirho.contains(&"arr#"),
        "expected arr# to stay intact before the inner ), got {:?}",
        texts_chirho
    );
    assert!(
        texts_chirho.contains(&")"),
        "expected a plain ) token for the inner parenthesized application, got {:?}",
        texts_chirho
    );
    assert!(
        texts_chirho.contains(&"#)"),
        "expected the outer unboxed tuple to still close with #), got {:?}",
        texts_chirho
    );
    assert!(
        !texts_chirho.contains(&"arr"),
        "arr# should not be split into arr plus #), got {:?}",
        texts_chirho
    );
}

#[test]
fn lex_preprocessed_primitive_bytearray_unsafe_thaw_arr_hash_stays_single_token_chirho() {
    let source_chirho = crate::test_fixtures_chirho::BYTEARRAY_PREPROCESSED_SOURCE_CHIRHO;
    let declaration_offset_chirho = source_chirho
        .find("unsafeThawByteArray (ByteArray arr#)")
        .expect("expected unsafeThawByteArray declaration");
    let argument_offset_chirho = declaration_offset_chirho
        + source_chirho[declaration_offset_chirho..]
            .find("unsafeCoerce# arr#")
            .expect("expected unsafeCoerce# argument in unsafeThawByteArray")
        + "unsafeCoerce# ".len();
    let file_id_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let mut lexer_chirho = LexerChirho::new_chirho(source_chirho, file_id_chirho);
    let tokens_chirho = lexer_chirho.lex_all_chirho();
    let token_chirho = tokens_chirho
        .iter()
        .find(|token_chirho| {
            let start_chirho = token_chirho.span_chirho.start_chirho().as_usize_chirho();
            let end_chirho = token_chirho.span_chirho.end_chirho().as_usize_chirho();
            start_chirho <= argument_offset_chirho && argument_offset_chirho < end_chirho
        })
        .expect("expected token covering primitive arr# offset");
    let text_chirho = &source_chirho[token_chirho.span_chirho.start_chirho().as_usize_chirho()
        ..token_chirho.span_chirho.end_chirho().as_usize_chirho()];
    assert_eq!(
        text_chirho, "arr#",
        "expected the primitive unsafeThawByteArray occurrence to stay `arr#`, got `{}` with kind {:?}",
        text_chirho, token_chirho.kind_chirho
    );
}

#[test]
fn lex_th_name_quote_value_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("'bimapConst");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::VarIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_th_name_quote_type_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("''Functor");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::ConIdChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}

#[test]
fn lex_th_name_quote_tilde_operator_chirho() {
    let kinds_chirho = non_trivia_kinds_chirho("''(~)");
    assert_eq!(
        kinds_chirho,
        vec![
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::TickChirho,
            RawTokenKindChirho::LeftParenChirho,
            RawTokenKindChirho::TildeChirho,
            RawTokenKindChirho::RightParenChirho,
            RawTokenKindChirho::EofChirho,
        ]
    );
}
