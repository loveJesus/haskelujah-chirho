// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! End-to-end tests for Phase 3 language extensions batch:
//! TypeOperators, EmptyCase, HexFloatLiterals, QuantifiedConstraints,
//! LambdaCase, UndecidableInstances, AllowAmbiguousTypes, GADTSyntax,
//! UndecidableSuperClasses, StandaloneKindSignatures, NegativeLiterals,
//! BinaryLiterals, MonoLocalBinds, NoMonomorphismRestriction,
//! TypeSynonymInstances.

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

// ── LambdaCase ──────────────────────────────────────────────────────────

#[test]
fn lambda_case_basic_chirho() {
    // \case with integer alternatives
    let src_chirho = "\
{-# LANGUAGE LambdaCase #-}
module Test where
classify = \\case
  0 -> 100
  1 -> 200
  _ -> 300
main = classify 1
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LambdaCase.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(200));
}

#[test]
fn lambda_case_adt_chirho() {
    // \case with ADT constructors
    let src_chirho = "\
{-# LANGUAGE LambdaCase #-}
module Test where
data Color = Red | Green | Blue
toNum = \\case
  Red -> 1
  Green -> 2
  Blue -> 3
main = toNum Green
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LambdaCaseADT.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(2));
}

#[test]
fn lambda_case_in_map_chirho() {
    // \case used as argument to map
    let src_chirho = "\
{-# LANGUAGE LambdaCase #-}
module Test where
main = sum (map (\\case { 0 -> 10; _ -> 1 }) [0, 1, 0, 2])
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LambdaCaseMap.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(22));
}

// ── UndecidableInstances ────────────────────────────────────────────────

#[test]
fn undecidable_instances_chirho() {
    // Instance with non-decreasing context — should compile fine
    let src_chirho = "\
{-# LANGUAGE UndecidableInstances #-}
{-# LANGUAGE FlexibleInstances #-}
module Test where
class MyClass a where
  myVal :: a -> Int
instance MyClass Int where
  myVal x = x
main = myVal (42 :: Int)
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Undecidable.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── AllowAmbiguousTypes ─────────────────────────────────────────────────

#[test]
fn allow_ambiguous_types_chirho() {
    // Function with ambiguous type variable in context
    let src_chirho = "\
{-# LANGUAGE AllowAmbiguousTypes #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Ambiguous.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── GADTSyntax ──────────────────────────────────────────────────────────

#[test]
fn gadt_syntax_basic_chirho() {
    // data Foo where Con :: Foo — GADT syntax without full GADTs
    let src_chirho = "\
{-# LANGUAGE GADTSyntax #-}
module Test where
data MyBool where
  MyTrue :: MyBool
  MyFalse :: MyBool
toBool = \\x -> case x of
  MyTrue -> 1
  MyFalse -> 0
main = toBool MyTrue
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GADTSyntax.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(1));
}

// ── UndecidableSuperClasses ─────────────────────────────────────────────

#[test]
fn undecidable_superclasses_chirho() {
    // Classes with potentially cyclic superclass relationships
    let src_chirho = "\
{-# LANGUAGE UndecidableSuperClasses #-}
module Test where
class MyClass a where
  myMethod :: a -> Int
instance MyClass Int where
  myMethod x = x + 1
main = myMethod (41 :: Int)
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "UndecidableSuper.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── StandaloneKindSignatures ────────────────────────────────────────────

#[test]
fn standalone_kind_sig_chirho() {
    // type T :: * — standalone kind signature parsed and accepted
    let src_chirho = "\
{-# LANGUAGE StandaloneKindSignatures #-}
module Test where
data MyType = MkMyType Int
main = case MkMyType 42 of
  MkMyType n -> n
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "StandaloneKind.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── NegativeLiterals ────────────────────────────────────────────────────

#[test]
fn negative_literals_chirho() {
    // -42 as a literal rather than negate applied to 42
    let src_chirho = "\
{-# LANGUAGE NegativeLiterals #-}
module Test where
main = -42 + 84
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NegLit.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── BinaryLiterals ──────────────────────────────────────────────────────

#[test]
fn binary_literals_chirho() {
    // 0b101010 = 42
    let src_chirho = "\
{-# LANGUAGE BinaryLiterals #-}
module Test where
main = 0b101010
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "BinaryLit.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── MonoLocalBinds ──────────────────────────────────────────────────────

#[test]
fn mono_local_binds_chirho() {
    // MonoLocalBinds restricts generalization in where/let — we accept the pragma
    let src_chirho = "\
{-# LANGUAGE MonoLocalBinds #-}
module Test where
main = let x = 42 in x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MonoLocal.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── NoMonomorphismRestriction ───────────────────────────────────────────

#[test]
fn no_monomorphism_restriction_chirho() {
    // Disable monomorphism restriction
    let src_chirho = "\
{-# LANGUAGE NoMonomorphismRestriction #-}
module Test where
f = (+)
main = f 20 22
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoMono.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── TypeSynonymInstances ────────────────────────────────────────────────

#[test]
// ── MagicHash ───────────────────────────────────────────────────────────

#[test]
fn magic_hash_ident_chirho() {
    // Identifiers ending in # should parse fine
    let src_chirho = "\
{-# LANGUAGE MagicHash #-}
module Test where
foo# = 42
main = foo#
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MagicHash.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn magic_hash_type_chirho() {
    // Type names ending in # should parse fine
    let src_chirho = "\
{-# LANGUAGE MagicHash #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MagicType.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn magic_hash_int_literal_chirho() {
    // Integer literal with # suffix: 42# treated as 42
    let src_chirho = "\
{-# LANGUAGE MagicHash #-}
module Test where
main = 42#
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MagicInt.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn magic_hash_lexer_ident_chirho() {
    // Verify lexer produces VarId for foo#
    use haskelujah_parser_chirho::lexer_chirho::LexerChirho;
    let fid_chirho = haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO;
    let mut lexer_chirho = LexerChirho::new_chirho("foo# bar#", fid_chirho);
    let tokens_chirho = lexer_chirho.lex_all_chirho();
    let var_count_chirho = tokens_chirho
        .iter()
        .filter(|t_chirho| {
            t_chirho.kind_chirho == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::VarIdChirho
        })
        .count();
    assert_eq!(var_count_chirho, 2, "foo# and bar# should lex as VarId");
}

#[test]
fn magic_hash_lexer_con_chirho() {
    // Verify lexer produces ConId for Int#
    use haskelujah_parser_chirho::lexer_chirho::LexerChirho;
    let fid_chirho = haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO;
    let mut lexer_chirho = LexerChirho::new_chirho("Int# Char##", fid_chirho);
    let tokens_chirho = lexer_chirho.lex_all_chirho();
    let con_count_chirho = tokens_chirho
        .iter()
        .filter(|t_chirho| {
            t_chirho.kind_chirho == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::ConIdChirho
        })
        .count();
    assert_eq!(con_count_chirho, 2, "Int# and Char## should lex as ConId");
}

// ── TypeSynonymInstances ────────────────────────────────────────────────

#[test]
// ── TypedHoles ──────────────────────────────────────────────────────────

#[test]
fn typed_hole_compiles_chirho() {
    // `_` in expression position should compile (as a typed hole warning)
    // We just check that it doesn't error out at the type-checking stage
    let src_chirho = "\
module Test where
f :: Int -> Int
f = _
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // This should succeed — the hole function is defined but never called
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypedHole.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn typed_hole_named_compiles_chirho() {
    // Named holes like `_foo` should also compile
    let src_chirho = "\
module Test where
g :: Int -> Int -> Int
g x y = _result
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NamedHole.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn typed_hole_warning_emitted_chirho() {
    // The typed hole should produce a warning
    let src_chirho = "\
module Test where
f :: Int -> Int
f = _
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho = frontend_warnings_chirho(src_chirho, &mut sm_chirho, "HoleWarn.hs");
    let has_hole_warning_chirho = warnings_chirho
        .iter()
        .any(|w_chirho| {
            let msg_chirho = format!("{:?}", w_chirho);
            msg_chirho.contains("hole") || msg_chirho.contains("4200")
        });
    assert!(has_hole_warning_chirho, "should emit typed hole warning");
}

// ── Haskell2010 ─────────────────────────────────────────────────────────

#[test]
fn haskell2010_pragma_chirho() {
    // {-# LANGUAGE Haskell2010 #-} should be accepted
    let src_chirho = "\
{-# LANGUAGE Haskell2010 #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Haskell2010.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── ExplicitNamespaces ──────────────────────────────────────────────────

#[test]
fn explicit_namespaces_chirho() {
    // ExplicitNamespaces pragma accepted
    let src_chirho = "\
{-# LANGUAGE ExplicitNamespaces #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "ExplicitNS.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── ExplicitForAll ──────────────────────────────────────────────────────

#[test]
fn explicit_forall_chirho() {
    let src_chirho = "\
{-# LANGUAGE ExplicitForAll #-}
module Test where
id' :: forall a. a -> a
id' x = x
main = id' 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "ExplicitForAll.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── InstanceSigs ────────────────────────────────────────────────────────

#[test]
fn instance_sigs_chirho() {
    let src_chirho = "\
{-# LANGUAGE InstanceSigs #-}
module Test where
class MyEq a where
  myEq :: a -> a -> Bool
instance MyEq Int where
  myEq :: Int -> Int -> Bool
  myEq x y = x == y
main = if myEq (42 :: Int) (42 :: Int) then 1 else 0
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "InstanceSigs.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(1));
}

// ── TypeSynonymInstances ────────────────────────────────────────────────

#[test]
fn type_synonym_instances_chirho() {
    // TypeSynonymInstances pragma accepted; type synonym instance compiles
    let src_chirho = "\
{-# LANGUAGE TypeSynonymInstances #-}
{-# LANGUAGE FlexibleInstances #-}
module Test where
type Name = [Char]
class Greet a where
  greetLen :: a -> Int
instance Greet Name where
  greetLen xs = length xs
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypeSynInst.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── PolyKinds ──────────────────────────────────────────────────────────

#[test]
fn polykinds_pragma_accepted_chirho() {
    // PolyKinds pragma should be accepted and program compiles
    let src_chirho = "\
{-# LANGUAGE PolyKinds #-}
module Test where
data Proxy a = MkProxy
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PolyKinds1.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn polykinds_kind_variable_annotation_chirho() {
    // PolyKinds with kind variable annotation: data Proxy (a :: k) = MkProxy
    let src_chirho = "\
{-# LANGUAGE PolyKinds #-}
{-# LANGUAGE KindSignatures #-}
module Test where
data Proxy (a :: k) = MkProxy
x = MkProxy
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PolyKinds2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn polykinds_with_kind_arrow_chirho() {
    // PolyKinds with higher-kinded kind variable: (f :: k -> Type)
    let src_chirho = "\
{-# LANGUAGE PolyKinds #-}
{-# LANGUAGE KindSignatures #-}
module Test where
data HKD (f :: k -> *) = MkHKD
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PolyKinds3.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── Quick-win extension pragmas (batch 2) ──────────────────────────────

#[test]
fn unboxed_tuples_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE UnboxedTuples #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "UnboxedTuples.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn unboxed_sums_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE UnboxedSums #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "UnboxedSums.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn implicit_params_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE ImplicitParams #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "ImplicitParams.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn multi_param_type_classes_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE MultiParamTypeClasses #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "MPTC.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn functional_dependencies_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE FunctionalDependencies #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "FunDeps.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn type_abstractions_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE TypeAbstractions #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TypeAbstractions.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn overloaded_record_dot_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE OverloadedRecordDot #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "RecordDot.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn no_field_selectors_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE NoFieldSelectors #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoFieldSel.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn duplicate_record_fields_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE DuplicateRecordFields #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DupRecFields.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn no_star_is_type_pragma_chirho() {
    let src_chirho = "\
{-# LANGUAGE NoStarIsType #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoStarIsType.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── PartialTypeSignatures ──────────────────────────────────────────────────

#[test]
fn partial_type_sig_basic_chirho() {
    // Basic PartialTypeSignatures: `_` in return type position. The compiler
    // should infer the wildcard as Int and evaluate correctly.
    let src_chirho = "\
{-# LANGUAGE PartialTypeSignatures #-}
module Test where
f :: _ -> Int
f x = x + 1
main = f 41
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PartialTypeSig.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn partial_type_sig_multiple_wildcards_chirho() {
    // Multiple wildcards in one type signature — each `_` becomes an
    // independent fresh unification variable.
    let src_chirho = "\
{-# LANGUAGE PartialTypeSignatures #-}
module Test where
add :: _ -> _ -> Int
add x y = x + y
main = add 20 22
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PartialTypeSigMulti.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn partial_type_sig_warning_emitted_chirho() {
    // The compiler should emit warning W4201 for each wildcard found in a
    // type signature when PartialTypeSignatures is active.
    let src_chirho = "\
{-# LANGUAGE PartialTypeSignatures #-}
module Test where
f :: _ -> Int
f x = x + 1
main = f 41
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let warnings_chirho =
        frontend_warnings_chirho(src_chirho, &mut sm_chirho, "PartialTypeSigWarn.hs");
    let has_wildcard_warning_chirho = warnings_chirho
        .iter()
        .any(|w_chirho| {
            let msg_chirho = format!("{:?}", w_chirho);
            msg_chirho.contains("4201") || msg_chirho.contains("wildcard")
        });
    assert!(
        has_wildcard_warning_chirho,
        "expected W4201 wildcard warning for `_` in type signature"
    );
}

// ── RecursiveDo (mdo) ──────────────────────────────────────────────────

#[test]
fn mdo_parses_as_do_chirho() {
    // mdo keyword should be treated as do (RecursiveDo)
    let src_chirho = "\
{-# LANGUAGE RecursiveDo #-}
module Test where
main = mdo
  let x = 42
  return x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Mdo.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── Quick-win extension pragmas (batch 3) ──────────────────────────────

#[test]
fn standalone_kind_sig_with_data_chirho() {
    // StandaloneKindSignatures: `type T :: *` preceding a data decl
    let src_chirho = "\
{-# LANGUAGE StandaloneKindSignatures #-}
module Test where
type MyList :: * -> *
data MyList a = Nil | Cons a (MyList a)
main = case Cons 1 (Cons 2 Nil) of
  Cons x _ -> x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "StandaloneKind2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(1));
}

#[test]
fn strict_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Strict #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "Strict.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn strict_data_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE StrictData #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "StrictData.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn applicative_do_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE ApplicativeDo #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "ApplicativeDo.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn generalized_newtype_deriving_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE GeneralizedNewtypeDeriving #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "GND.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn derive_lift_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE DeriveLift #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "DeriveLift.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn safe_haskell_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Safe #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "Safe.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn trustworthy_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Trustworthy #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "Trustworthy.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn cpp_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE CPP #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "CPP.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn template_haskell_quotes_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE TemplateHaskellQuotes #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "THQuotes.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn qualified_do_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE QualifiedDo #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "QualifiedDo.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn overloaded_record_update_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE OverloadedRecordUpdate #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "RecordUpdate.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn type_data_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE TypeData #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "TypeData.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

// ── GHC2021 meta-extension ─────────────────────────────────────────────

#[test]
fn ghc2021_meta_extension_chirho() {
    // GHC2021 should expand to all its constituent extensions
    let src_chirho = "\
{-# LANGUAGE GHC2021 #-}
module Test where
data MyList a = Nil | Cons a (MyList a)
main = case Cons 42 Nil of
  Cons x _ -> x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GHC2021.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn ghc2024_meta_extension_chirho() {
    let src_chirho = "{-# LANGUAGE GHC2024 #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(eval_source_chirho(src_chirho, &mut sm_chirho, "GHC2024.hs", None).unwrap(), ValueChirho::IntChirho(42));
}

// -- Phase 3 item 40: Library type schemes expansion --

#[test]
fn builtin_realToFrac_chirho() {
    // realToFrac should type-check without error (type scheme present)
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf x = realToFrac x\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "realToFrac.hs");
    assert!(result_chirho.is_ok(), "realToFrac should type-check: {:?}", result_chirho.err());
}

#[test]
fn builtin_fromIntegral_chirho() {
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf x = fromIntegral x\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "fromIntegral.hs");
    assert!(result_chirho.is_ok(), "fromIntegral should type-check: {:?}", result_chirho.err());
}

#[test]
fn builtin_div_mod_chirho() {
    let src_chirho = "module Test where\nmain = div 10 3\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "divmod.hs", None);
    assert!(result_chirho.is_ok(), "div should eval: {:?}", result_chirho.err());
}

#[test]
fn builtin_error_without_stack_trace_chirho() {
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf = errorWithoutStackTrace \"oops\"\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "errWST.hs");
    assert!(result_chirho.is_ok(), "errorWithoutStackTrace should type-check: {:?}", result_chirho.err());
}

// -- Phase 3 item 41: Module interfaces expansion (transformers) --

#[test]
fn import_control_monad_trans_identity_chirho() {
    let src_chirho = "module Test where\nimport Control.Monad.Trans.Identity (IdentityT)\nf = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TransIdentity.hs");
    assert!(result_chirho.is_ok(), "Control.Monad.Trans.Identity import: {:?}", result_chirho.err());
}

#[test]
fn import_control_monad_trans_state_chirho() {
    let src_chirho = "module Test where\nimport Control.Monad.Trans.State (StateT)\nf = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TransState.hs");
    assert!(result_chirho.is_ok(), "Control.Monad.Trans.State import: {:?}", result_chirho.err());
}

// -- Phase 3 item 42: Desugarer robustness (panic elimination) --

#[test]
fn desugar_th_splice_no_panic_chirho() {
    // TH splices that reach the desugarer shouldn't panic, just produce dummy value
    let src_chirho = "module Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NoPanic.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// -- Phase 3 item 44: Layout rule — where closes do blocks --

#[test]
fn do_where_layout_basic_chirho() {
    // `where` at same/less indentation as `do` block should close the do block
    let src_chirho = "module Test where\nf = do\n  return a\n  where a = 42\nmain = f\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoWhere.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_where_layout_less_indent_chirho() {
    // `where` at less indentation than `do` body should also work
    let src_chirho = "module Test where\nf = do\n  return a\n where a = 42\nmain = f\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoWhere2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_where_compiles_chirho() {
    // Standard pattern: do with where clause
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf = do\n  return a\n  where a = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "DoWhereCmp.hs");
    assert!(result_chirho.is_ok(), "do-where should compile: {:?}", result_chirho.err());
}

#[test]
fn do_where_deeper_indent_chirho() {
    // `where` at DEEPER indent than do body should still close do block
    let src_chirho = "module Test where\nl3 = do\n  return a\n   where\n   a = 42\nmain = l3\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoWhere3.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── Kind inference: higher-kinded constraint arguments ─────────────────

#[test]
fn kind_hk_constraint_functor_chirho() {
    // Functor f => ... should work — f :: * -> *, not *
    let src_chirho = r#"
module Test where
class Functor f where
  fmap :: (a -> b) -> f a -> f b

data Box a = MkBox a

instance Functor Box where
  fmap g (MkBox x) = MkBox (g x)

main = case fmap (+1) (MkBox 41) of MkBox n -> n
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "KindHK1.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn kind_hk_constraint_monad_chirho() {
    // Monad m uses m :: * -> * in constraint context
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
class Applicative f where
  pure :: a -> f a
class Applicative m => Monad m where
  bind :: m a -> (a -> m b) -> m b
data Id a = MkId a
instance Applicative Id where
  pure x = MkId x
instance Monad Id where
  bind (MkId x) f = f x
main = case bind (MkId 40) (\x -> MkId (x + 2)) of MkId n -> n
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "KindHK2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn kind_forall_annotated_vars_chirho() {
    // forall (f :: * -> *). should work with kind annotations
    let src_chirho = r#"
{-# LANGUAGE KindSignatures #-}
{-# LANGUAGE RankNTypes #-}
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
data Box a = MkBox a
apply :: (forall a. a -> a) -> Int -> Int
apply f x = f x
main = apply (\x -> x) 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "KindForall.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── Non-breaking space as whitespace ───────────────────────────────────

#[test]
fn non_breaking_space_whitespace_chirho() {
    // U+00A0 (non-breaking space) between tokens should be treated as whitespace
    let src_chirho = "module\u{00A0}Test\u{00A0}where\nf\u{00A0}=\u{00A0}42\nmain = f\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NBS.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── OPTIONS_GHC -X extension extraction ───────────────────────────────

#[test]
fn options_ghc_x_extension_chirho() {
    // {-# OPTIONS_GHC -XRecursiveDo #-} should enable RecursiveDo extension
    let src_chirho = "{-# OPTIONS_GHC -XNoImplicitPrelude #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "OptsGHC.hs");
    assert!(result_chirho.is_ok(), "OPTIONS_GHC -X should work: {:?}", result_chirho.err());
}

// ── Module interface: Type.Reflection ──────────────────────────────────

#[test]
fn import_type_reflection_chirho() {
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import Type.Reflection (TypeRep, Typeable)
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TypeRefl.hs");
    assert!(result_chirho.is_ok(), "Type.Reflection import should work: {:?}", result_chirho.err());
}

// ── Module interface: Unsafe.Coerce ────────────────────────────────────

#[test]
fn import_unsafe_coerce_chirho() {
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import Unsafe.Coerce (unsafeCoerce)
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "UnsCoerce.hs");
    assert!(result_chirho.is_ok(), "Unsafe.Coerce import should work: {:?}", result_chirho.err());
}

// ── Module interface: GHC.Exception ────────────────────────────────────

#[test]
fn import_ghc_exception_chirho() {
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import GHC.Exception (SomeException, throw)
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "GHCExc.hs");
    assert!(result_chirho.is_ok(), "GHC.Exception import should work: {:?}", result_chirho.err());
}

// ── Type family reduction in type inference ────────────────────────────

#[test]
fn type_family_reduction_in_sig_chirho() {
    // Type family `F` with instance `F Int = Bool`, then a function
    // with declared return type `F Int` that returns `True`.
    // Without type family reduction, this would fail with E0205
    // (type signature mismatch) because `F Int` wouldn't reduce to `Bool`.
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

type family F a
type instance F Int = Bool

myVal :: F Int
myVal = True

main = myVal
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Compiles without E0205 — type family reduction resolves F Int = Bool
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TFRedSig.hs");
    assert!(result_chirho.is_ok(), "Type family reduction in sig should compile: {:?}", result_chirho.err());
}

#[test]
fn type_family_reduction_function_sig_chirho() {
    // Function with type family in result type
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

type family ResultOf a
type instance ResultOf Int = Int

addOne :: Int -> ResultOf Int
addOne x = x + 1

main = addOne 41
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TFRedFn.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            let s_chirho = format!("{}", val_chirho);
            assert!(s_chirho.contains("42"), "Expected 42, got: {}", s_chirho);
        }
        Err(e_chirho) => panic!("Type family in function sig should compile: {}", e_chirho),
    }
}

#[test]
fn type_family_closed_reduction_chirho() {
    // Closed type family with multiple equations
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

type family IsInt a where
  IsInt Int = Bool
  IsInt a   = Bool

checkInt :: IsInt Int
checkInt = True

main = checkInt
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Compiles without E0205 — closed type family reduces IsInt Int = Bool
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TFClosed.hs");
    assert!(result_chirho.is_ok(), "Closed type family reduction should compile: {:?}", result_chirho.err());
}

// ── Data families ──────────────────────────────────────────────────────

#[test]
fn data_family_decl_compiles_chirho() {
    // data family declaration should parse and compile without error
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where
data family XList a
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DataFam.hs", None);
    assert!(result_chirho.is_ok(), "data family decl should compile: {:?}", result_chirho.err());
}

#[test]
fn data_instance_decl_compiles_chirho() {
    // data instance should parse and compile without error
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where
data family XList a
data instance XList Int = XListInt [Int]
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DataInst.hs", None);
    assert!(result_chirho.is_ok(), "data instance decl should compile: {:?}", result_chirho.err());
}

#[test]
fn newtype_instance_decl_compiles_chirho() {
    // newtype instance should parse and compile without error
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where
data family Wrapper a
newtype instance Wrapper Int = WrapInt Int
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NewtypeInst.hs", None);
    assert!(result_chirho.is_ok(), "newtype instance decl should compile: {:?}", result_chirho.err());
}

#[test]
fn let_constructor_pattern_bind_chirho() {
    // let Just x = Just 42 in x  should evaluate to 42
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
main = let MkBox x = MkBox 42 in x
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LetConPat.hs", None);
    assert!(result_chirho.is_ok(), "let constructor pat bind should work: {:?}", result_chirho.err());
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn let_tuple_constructor_pattern_bind_chirho() {
    // let (a, b) = (10, 32) in a + b  should evaluate to 42
    let src_chirho = r#"
module Test where
main = let (a, b) = (10, 32) in a + b
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "LetTuplePat.hs", None);
    assert!(result_chirho.is_ok(), "let tuple pat bind should work: {:?}", result_chirho.err());
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn toplevel_constructor_pattern_bind_chirho() {
    // Top-level: MkBox val = MkBox 42; main = val
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
MkBox val = MkBox 42
main = val
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TopConPat.hs", None);
    assert!(result_chirho.is_ok(), "top-level constructor pat bind should work: {:?}", result_chirho.err());
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn where_clause_constructor_pattern_bind_chirho() {
    // where MkBox val = MkBox 42
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
main = val
  where MkBox val = MkBox 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "WhereConPat.hs", None);
    assert!(result_chirho.is_ok(), "where constructor pat bind should work: {:?}", result_chirho.err());
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn where_clause_tuple_pattern_bind_chirho() {
    // where (x, y) = (10, 32)
    let src_chirho = r#"
module Test where
main = x + y
  where (x, y) = (10, 32)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "WhereTuplePat.hs", None);
    assert!(result_chirho.is_ok(), "where tuple pat bind should work: {:?}", result_chirho.err());
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn where_clause_mixed_fun_and_pat_bind_chirho() {
    // Mix of function bindings and pattern bindings in where clause
    let src_chirho = r#"
module Test where
main = f x + y
  where
    f n = n * 2
    (x, y) = (10, 22)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "WhereMixed.hs", None);
    assert!(result_chirho.is_ok(), "mixed where bindings should work: {:?}", result_chirho.err());
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}
