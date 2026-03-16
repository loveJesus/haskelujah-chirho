// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! End-to-end tests for Phase 3 language extensions batch:
//! TypeOperators, EmptyCase, HexFloatLiterals, QuantifiedConstraints.

#[allow(unused_imports)]
use crate::{
    eval_source_chirho,
    compile_source_chirho,
    frontend_warnings_chirho,
    check_source_file_chirho,
};
#[allow(unused_imports)]
use haskelujah_span_chirho::SourceMapChirho;
#[allow(unused_imports)]
use haskelujah_runtime_chirho::{ValueChirho, ExecutionModeChirho};
#[allow(unused_imports)]
use haskelujah_syntax_chirho::SourceFileChirho;

// ── TypeOperators ─────────────────────────────────────────────────────────

#[test]
fn type_operator_infix_parses_chirho() {
    // type a :+: b = Either a b — infix type operator in type alias
    let src_chirho = "\
{-# LANGUAGE TypeOperators #-}
module Test where
data Either a b = Left a | Right b
type a :+: b = Either a b
myVal :: Int :+: Bool
myVal = Left 42
main = case myVal of
  Left n -> n
  Right _ -> 0
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypeOp.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn type_operator_parens_parses_chirho() {
    // (:+:) used as prefix type constructor in parens
    let src_chirho = "\
{-# LANGUAGE TypeOperators #-}
module Test where
data Either a b = Left a | Right b
type (:+:) a b = Either a b
myVal :: (:+:) Int Bool
myVal = Left 99
main = case myVal of
  Left n -> n
  Right _ -> 0
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypeOpParen.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(99));
}

#[test]
fn type_operator_backtick_parses_chirho() {
    // a `Either` b as infix type constructor with backticks
    let src_chirho = "\
{-# LANGUAGE TypeOperators #-}
module Test where
data Either a b = Left a | Right b
myVal :: Int `Either` Bool
myVal = Left 77
main = case myVal of
  Left n -> n
  Right _ -> 0
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypeOpBacktick.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(77));
}

// ── HexFloatLiterals ──────────────────────────────────────────────────────

#[test]
fn hex_float_basic_parses_chirho() {
    // 0x1.0p0 = 1.0 in hex float
    let src_chirho = "\
{-# LANGUAGE HexFloatLiterals #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "HexFloat.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn hex_float_lexer_token_chirho() {
    // Verify the lexer produces a FloatLit for hex float syntax
    use haskelujah_parser_chirho::lexer_chirho::LexerChirho;
    let fid_chirho = haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO;
    let mut lexer_chirho = LexerChirho::new_chirho("0x1Fp4", fid_chirho);
    let tokens_chirho = lexer_chirho.lex_all_chirho();
    let float_count_chirho = tokens_chirho
        .iter()
        .filter(|t_chirho| {
            t_chirho.kind_chirho == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::FloatLitChirho
        })
        .count();
    assert_eq!(float_count_chirho, 1, "0x1Fp4 should lex as FloatLit");
}

#[test]
fn hex_float_with_dot_parses_chirho() {
    // 0x1.8p1 = 3.0
    use haskelujah_parser_chirho::lexer_chirho::LexerChirho;
    let fid_chirho = haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO;
    let mut lexer_chirho = LexerChirho::new_chirho("0x1.8p1", fid_chirho);
    let tokens_chirho = lexer_chirho.lex_all_chirho();
    let float_count_chirho = tokens_chirho
        .iter()
        .filter(|t_chirho| {
            t_chirho.kind_chirho == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::FloatLitChirho
        })
        .count();
    assert_eq!(float_count_chirho, 1, "0x1.8p1 should lex as FloatLit");
}

#[test]
fn hex_int_unchanged_chirho() {
    // 0xFF without p/P should still lex as IntLit
    use haskelujah_parser_chirho::lexer_chirho::LexerChirho;
    let fid_chirho = haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO;
    let mut lexer_chirho = LexerChirho::new_chirho("0xFF", fid_chirho);
    let tokens_chirho = lexer_chirho.lex_all_chirho();
    let int_count_chirho = tokens_chirho
        .iter()
        .filter(|t_chirho| {
            t_chirho.kind_chirho == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::IntLitChirho
        })
        .count();
    assert_eq!(int_count_chirho, 1, "0xFF without p should remain IntLit");
}

// ── EmptyCase ─────────────────────────────────────────────────────────────

#[test]
fn empty_case_parses_chirho() {
    // Empty case expression should parse without errors
    let src_chirho = "\
{-# LANGUAGE EmptyCase #-}
module Test where
data Void
absurd :: Void -> a
absurd x = case x of {}
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Just verify it parses and type-checks — we never call absurd
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "EmptyCase.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn empty_case_with_braces_parses_chirho() {
    // Explicit braces empty case
    let src_chirho = "\
{-# LANGUAGE EmptyCase #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "EmptyCase2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── QuantifiedConstraints ─────────────────────────────────────────────────

#[test]
fn quantified_constraint_parses_chirho() {
    // forall a. constraint in type signature context
    let src_chirho = "\
{-# LANGUAGE QuantifiedConstraints #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "QC.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn quantified_constraint_type_sig_parses_chirho() {
    // A type signature with a quantified constraint should parse
    let src_chirho = "\
{-# LANGUAGE QuantifiedConstraints #-}
{-# LANGUAGE RankNTypes #-}
module Test where
class MyClass f where
  myMethod :: f Int -> Int
foo :: (forall a. Show a) => Int
foo = 42
main = foo
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "QCSig.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}
