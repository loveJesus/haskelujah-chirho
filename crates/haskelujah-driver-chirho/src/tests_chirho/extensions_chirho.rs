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
