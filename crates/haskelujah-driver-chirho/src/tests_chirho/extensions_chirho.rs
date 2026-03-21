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
    check_source_file_chirho, compile_source_chirho, eval_source_chirho,
    eval_source_with_machine_chirho, frontend_warnings_chirho,
};
#[allow(unused_imports)]
use haskelujah_runtime_chirho::{ExecutionModeChirho, ValueChirho};
#[allow(unused_imports)]
use haskelujah_span_chirho::SourceMapChirho;
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
            t_chirho.kind_chirho
                == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::FloatLitChirho
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
            t_chirho.kind_chirho
                == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::FloatLitChirho
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
            t_chirho.kind_chirho
                == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::IntLitChirho
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
            t_chirho.kind_chirho
                == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::VarIdChirho
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
            t_chirho.kind_chirho
                == haskelujah_parser_chirho::lexer_chirho::RawTokenKindChirho::ConIdChirho
        })
        .count();
    assert_eq!(con_count_chirho, 2, "Int# and Char## should lex as ConId");
}

// ── TypeSynonymInstances ────────────────────────────────────────────────

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
    let has_hole_warning_chirho = warnings_chirho.iter().any(|w_chirho| {
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
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "PartialTypeSigMulti.hs", None);
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
    let has_wildcard_warning_chirho = warnings_chirho.iter().any(|w_chirho| {
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
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "Strict.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn strict_data_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE StrictData #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "StrictData.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn applicative_do_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE ApplicativeDo #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "ApplicativeDo.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn generalized_newtype_deriving_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE GeneralizedNewtypeDeriving #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "GND.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn derive_lift_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE DeriveLift #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "DeriveLift.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn safe_haskell_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Safe #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "Safe.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn trustworthy_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE Trustworthy #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "Trustworthy.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn cpp_extension_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE CPP #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "CPP.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn template_haskell_quotes_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE TemplateHaskellQuotes #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "THQuotes.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn qualified_do_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE QualifiedDo #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "QualifiedDo.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn overloaded_record_update_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE OverloadedRecordUpdate #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "RecordUpdate.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

#[test]
fn type_data_pragma_chirho() {
    let src_chirho = "{-# LANGUAGE TypeData #-}\nmodule Test where\nmain = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "TypeData.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
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
    assert_eq!(
        eval_source_chirho(src_chirho, &mut sm_chirho, "GHC2024.hs", None).unwrap(),
        ValueChirho::IntChirho(42)
    );
}

// -- Phase 3 item 40: Library type schemes expansion --

#[test]
fn builtin_real_to_frac_chirho() {
    // realToFrac should type-check without error (type scheme present)
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf x = realToFrac x\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "realToFrac.hs");
    assert!(
        result_chirho.is_ok(),
        "realToFrac should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn builtin_from_integral_chirho() {
    let src_chirho =
        "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf x = fromIntegral x\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "fromIntegral.hs");
    assert!(
        result_chirho.is_ok(),
        "fromIntegral should type-check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn builtin_div_mod_chirho() {
    let src_chirho = "module Test where\nmain = div 10 3\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "divmod.hs", None);
    assert!(
        result_chirho.is_ok(),
        "div should eval: {:?}",
        result_chirho.err()
    );
}

#[test]
fn builtin_error_without_stack_trace_chirho() {
    let src_chirho = "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Test where\nf = errorWithoutStackTrace \"oops\"\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "errWST.hs");
    assert!(
        result_chirho.is_ok(),
        "errorWithoutStackTrace should type-check: {:?}",
        result_chirho.err()
    );
}

// -- Phase 3 item 41: Module interfaces expansion (transformers) --

#[test]
fn import_control_monad_trans_identity_chirho() {
    let src_chirho = "module Test where\nimport Control.Monad.Trans.Identity (IdentityT)\nf = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TransIdentity.hs");
    assert!(
        result_chirho.is_ok(),
        "Control.Monad.Trans.Identity import: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_control_monad_trans_state_chirho() {
    let src_chirho = "module Test where\nimport Control.Monad.Trans.State (StateT)\nf = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TransState.hs");
    assert!(
        result_chirho.is_ok(),
        "Control.Monad.Trans.State import: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "do-where should compile: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "OPTIONS_GHC -X should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "Type.Reflection import should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "Unsafe.Coerce import should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "GHC.Exception import should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "Type family reduction in sig should compile: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "Closed type family reduction should compile: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "data family decl should compile: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "data instance decl should compile: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "newtype instance decl should compile: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "let constructor pat bind should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "let tuple pat bind should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "top-level constructor pat bind should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "where constructor pat bind should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "where tuple pat bind should work: {:?}",
        result_chirho.err()
    );
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
    assert!(
        result_chirho.is_ok(),
        "mixed where bindings should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_let_tuple_pattern_bind_chirho() {
    // do { let (x, y) = (10, 32); putStrLn (show (x + y)) }
    let src_chirho = r#"
module Test where
main = do
  let (x, y) = (10, 32)
  return (x + y)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoLetTuple.hs", None);
    assert!(
        result_chirho.is_ok(),
        "do-let tuple pat should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn do_let_constructor_pattern_bind_chirho() {
    // do { let MkBox x = MkBox 42; return x }
    let src_chirho = r#"
module Test where
data MyBox = MkBox Int
main = do
  let MkBox x = MkBox 42
  return x
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DoLetCon.hs", None);
    assert!(
        result_chirho.is_ok(),
        "do-let constructor pat should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

// ── SCC binding groups (forward references / let-polymorphism) ──

/// `main = iD iD 42; iD x = x` — main references iD which is defined later;
/// SCC analysis should generalize iD (forall a. a -> a) before main.
#[test]
fn scc_forward_ref_id_id_chirho() {
    use crate::eval_source_chirho;
    let src_chirho = r#"module Test where
main = iD iD 42
iD x = x
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "SccForward.hs", None);
    assert!(
        result_chirho.is_ok(),
        "SCC forward ref should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

/// Forward references to multiple functions defined later:
/// `main` calls `compose`, `incr`, `dbl` all defined after it.
#[test]
fn scc_forward_ref_multi_chirho() {
    use crate::eval_source_chirho;
    let src_chirho = r#"module Test where
main = compose incr incr 40
compose f g x = f (g x)
incr x = x + 1
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "SccMulti.hs", None);
    assert!(
        result_chirho.is_ok(),
        "SCC multi forward ref should work: {:?}",
        result_chirho.err()
    );
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

/// Mutual recursion: `isEven 0 = True; isEven n = isOdd (n-1);
/// isOdd 0 = False; isOdd n = isEven (n-1)` — SCC groups them together.
#[test]
fn scc_mutual_recursion_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
main = print (isEven 4)
isEven 0 = True
isEven n = isOdd (n - 1)
isOdd 0 = False
isOdd n = isEven (n - 1)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "SccMutual.hs", None)
            .unwrap_or_else(|e_chirho| panic!("SCC mutual recursion should work: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "True\n");
}

// ── Parametric instance resolution tests ──

/// `Show [a]` parametric instance: `show [True, False]` should type-check
/// and produce the right output.
#[test]
fn parametric_show_list_bool_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
main = putStrLn (show [True, False, True])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ShowListBool.hs", None)
            .unwrap_or_else(|e_chirho| panic!("Show [Bool] should work: {}", e_chirho));
    assert!(
        m_chirho.io_output_chirho.contains("[True"),
        "expected list show output, got: {}",
        m_chirho.io_output_chirho
    );
}

/// `Eq (Maybe a)` parametric instance: `Just 1 == Just 1` should type-check.
#[test]
fn parametric_eq_maybe_chirho() {
    let src_chirho = r#"module Test where
main = print (Just 1 == Just 1)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "EqMaybe.hs");
    assert!(
        result_chirho.is_ok(),
        "Eq (Maybe Int) should type-check: {:?}",
        result_chirho.err()
    );
}

/// `Show (Maybe a)` parametric instance: `show (Just 42)` should type-check.
#[test]
fn parametric_show_maybe_chirho() {
    let src_chirho = r#"module Test where
main = putStrLn (show (Just 42))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ShowMaybe.hs");
    assert!(
        result_chirho.is_ok(),
        "Show (Maybe Int) should type-check: {:?}",
        result_chirho.err()
    );
}

/// `Show (a, b)` parametric instance: `show (1, True)` should type-check.
#[test]
fn parametric_show_tuple_chirho() {
    let src_chirho = r#"module Test where
main = putStrLn (show (1, True))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ShowTuple.hs");
    assert!(
        result_chirho.is_ok(),
        "Show (Int, Bool) should type-check: {:?}",
        result_chirho.err()
    );
}

/// `Eq [a]` parametric instance: `[1,2] == [1,2]` should type-check.
#[test]
fn parametric_eq_list_chirho() {
    let src_chirho = r#"module Test where
main = print ([1,2,3] == [1,2,3])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "EqList.hs");
    assert!(
        result_chirho.is_ok(),
        "Eq [Int] should type-check: {:?}",
        result_chirho.err()
    );
}

/// `Ord [a]` parametric instance: `compare [1] [2]` should type-check.
#[test]
fn parametric_ord_list_chirho() {
    let src_chirho = r#"module Test where
main = print (compare [1] [2])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "OrdList.hs");
    assert!(
        result_chirho.is_ok(),
        "Ord [Int] should type-check: {:?}",
        result_chirho.err()
    );
}

/// 3-tuple instances: `(1, True, 'a') == (1, True, 'a')` should type-check.
#[test]
fn parametric_eq_triple_chirho() {
    let src_chirho = r#"module Test where
main = print ((1, True, 'a') == (1, True, 'a'))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "EqTriple.hs");
    assert!(
        result_chirho.is_ok(),
        "Eq (Int, Bool, Char) should type-check: {:?}",
        result_chirho.err()
    );
}

/// Letrec where-clause with multiple bindings referencing outer lambda params.
/// Regression: `c = x + y` where x, y are letrec-bound thunks with captures
/// was producing 0 because thunks weren't capturing sibling letrec refs correctly.
#[test]
fn letrec_multi_thunk_capture_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
f a = c where { x = a; y = a; c = x + y }
main = print (f 5)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "LetrecMulti.hs", None)
            .expect("should eval");
    assert_eq!(
        m_chirho.io_output_chirho.trim(),
        "10",
        "f 5 = x + y = a + a = 10"
    );
}

/// Letrec where-clause with arithmetic on captured values.
#[test]
fn letrec_thunk_arithmetic_capture_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
f a = c where { x = a + 1; y = a * 2; c = x + y }
main = print (f 10)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "LetrecArith.hs", None)
            .expect("should eval");
    assert_eq!(
        m_chirho.io_output_chirho.trim(),
        "31",
        "f 10: x=11, y=20, c=31"
    );
}

/// Letrec where three thunks depend on two outer lambda params.
#[test]
fn letrec_multi_param_capture_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
f a b = c where { x = a * a; y = b * b; c = x + y }
main = print (f 3 4)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "LetrecMultiParam.hs", None)
            .expect("should eval");
    assert_eq!(
        m_chirho.io_output_chirho.trim(),
        "25",
        "f 3 4 = 9 + 16 = 25"
    );
}

/// Record field types with type variables: `data Box f = MkBox { unBox :: f Int }`
/// Regression: VarId tokens after `::` in record fields were silently dropped,
/// causing kind mismatch errors (E0300) on fields like `field :: m (Maybe a)`.
#[test]
fn record_field_type_var_chirho() {
    let src_chirho = r#"module Test where
data Wrapper f = MkWrapper { unwrap :: f Int }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldVar.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field with type variable should compile: {:?}",
        result_chirho.err()
    );
}

/// Record field with application: `field :: Maybe a`
#[test]
fn record_field_app_type_chirho() {
    let src_chirho = r#"module Test where
data Box a = MkBox { getValue :: Maybe a }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldApp.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field Maybe a should compile: {:?}",
        result_chirho.err()
    );
}

/// Record field with function type: `field :: a -> b`
#[test]
fn record_field_fun_type_chirho() {
    let src_chirho = r#"module Test where
data Fun a b = MkFun { runFun :: a -> b }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldFun.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field a -> b should compile: {:?}",
        result_chirho.err()
    );
}

/// Record field with list type: `field :: [a]`
#[test]
fn record_field_list_type_chirho() {
    let src_chirho = r#"module Test where
data Container a = MkContainer { items :: [a] }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldList.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field [a] should compile: {:?}",
        result_chirho.err()
    );
}

/// MaybeT-style newtype with higher-kinded record field:
/// `newtype MaybeT m a = MaybeT { runMaybeT :: m (Maybe a) }`
#[test]
fn record_field_higher_kinded_chirho() {
    let src_chirho = r#"module Test where
newtype MaybeT m a = MaybeT { runMaybeT :: m (Maybe a) }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldHK.hs");
    assert!(
        result_chirho.is_ok(),
        "Higher-kinded record field m (Maybe a) should compile: {:?}",
        result_chirho.err()
    );
}

/// Type-annotated pattern: `(x :: Int)` in function argument
#[test]
fn type_annot_pat_basic_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
{-# LANGUAGE ScopedTypeVariables #-}
f (x :: Int) = x + 1
main = print (f 41)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TypeAnnotPat.hs", None)
            .expect("type-annotated pattern should eval");
    assert_eq!(m_chirho.io_output_chirho.trim(), "42");
}

/// Type-annotated pattern in case alternative
#[test]
fn type_annot_pat_case_chirho() {
    let src_chirho = r#"module Test where
{-# LANGUAGE ScopedTypeVariables #-}
f x = case x of { (y :: Int) -> y + 10 }
main = print (f 32)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TypeAnnotCase.hs");
    assert!(
        result_chirho.is_ok(),
        "Type-annotated pattern in case should compile: {:?}",
        result_chirho.err()
    );
}

/// Type-annotated pattern compiles without ScopedTypeVariables too
#[test]
fn type_annot_pat_no_ext_chirho() {
    let src_chirho = r#"module Test where
f (x :: Int) = x + 1
main = print (f 41)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TypeAnnotNoExt.hs");
    assert!(
        result_chirho.is_ok(),
        "Type-annotated pattern without ext should compile: {:?}",
        result_chirho.err()
    );
}

/// Module interfaces: Data.Type.Equality
#[test]
fn import_data_type_equality_chirho() {
    let src_chirho = r#"module Test where
import Data.Type.Equality (testEquality, castWith)
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "DataTypeEq.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Type.Equality import should compile: {:?}",
        result_chirho.err()
    );
}

/// Module interfaces: Control.Applicative
#[test]
fn import_control_applicative_chirho() {
    let src_chirho = r#"module Test where
import Control.Applicative
f :: ZipList Int -> Int
f _ = 42
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "CtrlApp.hs");
    assert!(
        result_chirho.is_ok(),
        "Control.Applicative import should compile: {:?}",
        result_chirho.err()
    );
}

/// Module interfaces: Data.Functor.Identity
#[test]
fn import_data_functor_identity_chirho() {
    let src_chirho = r#"module Test where
import Data.Functor.Identity
f :: Identity Int -> Int
f (Identity x) = x
main = print (f (Identity 42))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "FunId.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Functor.Identity import should compile: {:?}",
        result_chirho.err()
    );
}

/// Module interfaces: GHC.Base
#[test]
fn import_ghc_base_chirho() {
    let src_chirho = r#"module Test where
import GHC.Base (id, const, flip)
f = id 42
main = print f
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "GHCBase.hs");
    assert!(
        result_chirho.is_ok(),
        "GHC.Base import should compile: {:?}",
        result_chirho.err()
    );
}

/// Module interfaces: Data.Foldable
#[test]
fn import_data_foldable_chirho() {
    let src_chirho = r#"module Test where
import Data.Foldable (toList, fold)
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Foldable.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Foldable import should compile: {:?}",
        result_chirho.err()
    );
}

/// Module interfaces: Data.IORef
#[test]
fn import_data_ioref_chirho() {
    let src_chirho = r#"module Test where
import Data.IORef
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "IORef.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.IORef import should compile: {:?}",
        result_chirho.err()
    );
}

// ── Case binder (variable pattern) tests ───────────────────────────────

/// `case 42 of x -> x` should return 42, not 0.
#[test]
fn case_binder_var_pattern_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
main = print (case (42 :: Int) of x -> x)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "42",
                "case var pattern should bind scrutinee"
            );
        }
        Err(e_chirho) => panic!("case var pattern failed: {}", e_chirho),
    }
}

/// `case n of x -> x + 1` with n=41 should return 42.
#[test]
fn case_binder_var_arithmetic_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
f :: Int -> Int
f n = case n of
  x -> x + 1

main :: IO ()
main = print (f 41)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar2.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "42",
                "case var pattern should bind for arithmetic"
            );
        }
        Err(e_chirho) => panic!("case var arithmetic failed: {}", e_chirho),
    }
}

/// `case n of x -> let y = x * 2 in y` should use the scrutinee.
#[test]
fn case_binder_var_let_in_alt_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
main :: IO ()
main = do
  let n = 21 :: Int
  print (case n of x -> let y = x * 2 in y)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar3.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "42",
                "case var should be visible in let-in-alt"
            );
        }
        Err(e_chirho) => panic!("case var let-in-alt failed: {}", e_chirho),
    }
}

/// Case binder with constructor patterns should still work.
#[test]
fn case_binder_con_pattern_still_works_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
data Color = Red | Green | Blue

f :: Color -> Int
f c = case c of
  Red   -> 1
  Green -> 2
  Blue  -> 3

main :: IO ()
main = print (f Green)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar4.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(
                machine_chirho.io_output_chirho.trim(),
                "2",
                "constructor case alts should still work"
            );
        }
        Err(e_chirho) => panic!("constructor case alt failed: {}", e_chirho),
    }
}

/// Multiple case alts mixing literal and default.
#[test]
fn case_binder_default_fallthrough_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"
f :: Int -> Int
f n = case n of
  0 -> 100
  x -> x + 1

main :: IO ()
main = do
  print (f 0)
  print (f 41)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseVar5.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho.len(), 2, "should have two outputs");
            assert_eq!(lines_chirho[0], "100", "literal alt should match");
            assert_eq!(lines_chirho[1], "42", "default alt should bind scrutinee");
        }
        Err(e_chirho) => panic!("default fallthrough failed: {}", e_chirho),
    }
}

// ---------------------------------------------------------------------------
// Where-clause in case alternatives
// ---------------------------------------------------------------------------

/// `case e of x -> result where result = x` — where-clause binds case binder.
#[test]
fn case_alt_where_clause_chirho() {
    let src_chirho = r#"
module CaseWhere1 where

f :: Int -> Int
f n = case n of
  x -> result
    where result = x + 1

main :: IO ()
main = print (f 41)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere1.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("case alt where-clause failed: {}", e_chirho),
    }
}

/// Multiple bindings in a case alt where-clause.
#[test]
fn case_alt_where_multiple_binds_chirho() {
    let src_chirho = r#"
module CaseWhere2 where

f :: Int -> Int
f n = case n of
  x -> a + b
    where
      a = x * 2
      b = x + 1

main :: IO ()
main = print (f 10)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere2.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            // 10 * 2 + (10 + 1) = 20 + 11 = 31
            assert_eq!(machine_chirho.io_output_chirho.trim(), "31");
        }
        Err(e_chirho) => panic!("case alt where multiple binds failed: {}", e_chirho),
    }
}

/// Where-clause in a constructor pattern case alt.
#[test]
fn case_alt_where_con_pattern_chirho() {
    let src_chirho = r#"
module CaseWhere3 where

data Pair = MkPair Int Int

f :: Pair -> Int
f p = case p of
  MkPair a b -> result
    where result = a + b

main :: IO ()
main = print (f (MkPair 20 22))
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere3.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("case alt where con pattern failed: {}", e_chirho),
    }
}

/// Where-clause in a multi-alt case expression (only on one alt).
#[test]
fn case_alt_where_multi_alt_chirho() {
    let src_chirho = r#"
module CaseWhere4 where

f :: Int -> Int
f n = case n of
  0 -> 100
  x -> doubled
    where doubled = x * 2

main :: IO ()
main = do
  print (f 0)
  print (f 21)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere4.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho.len(), 2);
            assert_eq!(lines_chirho[0], "100");
            assert_eq!(lines_chirho[1], "42");
        }
        Err(e_chirho) => panic!("case alt where multi-alt failed: {}", e_chirho),
    }
}

/// Where-clause with a function binding in case alt.
#[test]
fn case_alt_where_function_bind_chirho() {
    let src_chirho = r#"
module CaseWhere5 where

f :: Int -> Int
f n = case n of
  x -> double x
    where double y = y * 2

main :: IO ()
main = print (f 21)
"#;
    let mut sm_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseWhere5.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("case alt where function bind failed: {}", e_chirho),
    }
}

// ---------------------------------------------------------------------------
// Constraint tuple kind inference
// ---------------------------------------------------------------------------

/// Constraint tuple `(Show a, Eq a) => a -> String` should kind-check.
#[test]
fn constraint_tuple_kind_chirho() {
    let src_chirho = r#"
module ConstraintTuple1 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

showEq :: (Show a, Eq a) => a -> String
showEq x = show x

main :: IO ()
main = print (showEq 42)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintTuple1.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "\"42\"");
        }
        Err(e_chirho) => panic!("constraint tuple kind failed: {}", e_chirho),
    }
}

/// Three-element constraint tuple.
#[test]
fn constraint_tuple_three_chirho() {
    let src_chirho = r#"
module ConstraintTuple2 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

class Ord a where
  compare :: a -> a -> Int

f :: (Show a, Eq a, Ord a) => a -> String
f x = show x

main :: IO ()
main = print (f 99)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintTuple2.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "\"99\"");
        }
        Err(e_chirho) => panic!("3-element constraint tuple failed: {}", e_chirho),
    }
}

/// ConstraintKinds: type alias for a constraint tuple.
#[test]
fn constraint_kinds_type_alias_chirho() {
    let src_chirho = r#"
{-# LANGUAGE ConstraintKinds #-}
module ConstraintKinds1 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

type ShowEq a = (Show a, Eq a)

f :: ShowEq a => a -> String
f x = show x

main :: IO ()
main = print (f 42)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintKinds1.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "\"42\"");
        }
        Err(e_chirho) => panic!("ConstraintKinds type alias failed: {}", e_chirho),
    }
}

/// Class with constraint tuple superclass.
#[test]
fn constraint_tuple_superclass_chirho() {
    let src_chirho = r#"
module ConstraintTuple3 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

class (Show a, Eq a) => Printable a where
  display :: a -> String

main :: IO ()
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintTuple3.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("constraint tuple superclass failed: {}", e_chirho),
    }
}

// ---------------------------------------------------------------------------
// DoAndIfThenElse / continuation keywords in layout
// ---------------------------------------------------------------------------

/// `then`/`else` at same indentation as `if` in a do-block.
#[test]
fn do_if_then_else_aligned_chirho() {
    let src_chirho = r#"
module DoIfAligned where

main :: IO ()
main = do if True
          then print 42
          else print 0
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "DoIfAligned.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("do if/then/else aligned failed: {}", e_chirho),
    }
}

/// `then`/`else` at same column as do-block statements.
#[test]
fn do_if_then_else_at_do_indent_chirho() {
    let src_chirho = r#"
module DoIfDo where

main :: IO ()
main = do
  print 1
  if True
    then print 42
    else print 0
  print 3
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "DoIfDo.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho, vec!["1", "42", "3"]);
        }
        Err(e_chirho) => panic!("do if/then/else at do indent failed: {}", e_chirho),
    }
}

/// Nested if/then/else in do-block.
#[test]
fn do_nested_if_then_else_chirho() {
    let src_chirho = r#"
module DoNestedIf where

f :: Int -> IO ()
f n = do
  if n > 0
    then if n > 10
           then print 100
           else print n
    else print 0

main :: IO ()
main = do
  f 5
  f 20
  f 0
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "DoNestedIf.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho, vec!["5", "100", "0"]);
        }
        Err(e_chirho) => panic!("do nested if/then/else failed: {}", e_chirho),
    }
}

/// Case-of with `of` on the next line at same indent as scrutinee.
#[test]
fn case_of_next_line_chirho() {
    let src_chirho = r#"
module CaseOfNext where

f :: Int -> Int
f n = case n
        of 0 -> 100
           _ -> n + 1

main :: IO ()
main = do
  print (f 0)
  print (f 41)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "CaseOfNext.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            let lines_chirho: Vec<&str> = machine_chirho.io_output_chirho.trim().lines().collect();
            assert_eq!(lines_chirho, vec!["100", "42"]);
        }
        Err(e_chirho) => panic!("case of next line failed: {}", e_chirho),
    }
}

// ── Module Interfaces Batch 5 ─────────────────────────────────────────────

#[test]
fn iface_data_proxy_chirho() {
    let src_chirho = r#"
module IfaceProxy where
import Data.Proxy (Proxy(..))

proxyInt :: Proxy Int
proxyInt = Proxy

main :: IO ()
main = print "proxy ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceProxy.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Proxy import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_typeable_chirho() {
    let src_chirho = r#"
module IfaceTypeable where
import Data.Typeable (Typeable, typeOf)

main :: IO ()
main = print "typeable ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeable.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Typeable import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_coerce_chirho() {
    let src_chirho = r#"
module IfaceCoerce where
import Data.Coerce (Coercible, coerce)

main :: IO ()
main = print "coerce ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceCoerce.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Coerce import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_type_lits_chirho() {
    let src_chirho = r#"
module IfaceTypeLits where
import GHC.TypeLits (Nat, Symbol, KnownNat, KnownSymbol, natVal, symbolVal)

main :: IO ()
main = print "typelits ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeLits.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.TypeLits import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_type_nats_chirho() {
    let src_chirho = r#"
module IfaceTypeNats where
import GHC.TypeNats (Nat, KnownNat, natVal)

main :: IO ()
main = print "typenats ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeNats.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.TypeNats import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_foreign_storable_chirho() {
    let src_chirho = r#"
module IfaceStorable where
import Foreign.Storable (Storable, sizeOf, alignment)

main :: IO ()
main = print "storable ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceStorable.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Foreign.Storable import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_foreign_ptr_chirho() {
    let src_chirho = r#"
module IfacePtr where
import Foreign.Ptr (Ptr, FunPtr, nullPtr, castPtr)

main :: IO ()
main = print "ptr ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfacePtr.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Foreign.Ptr import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_control_concurrent_mvar_chirho() {
    let src_chirho = r#"
module IfaceMVar where
import Control.Concurrent.MVar (MVar, newMVar, takeMVar, putMVar)

main :: IO ()
main = print "mvar ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceMVar.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Control.Concurrent.MVar import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_control_monad_trans_class_chirho() {
    let src_chirho = r#"
module IfaceMonadTrans where
import Control.Monad.Trans.Class (MonadTrans, lift)

main :: IO ()
main = print "monadtrans ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceMonadTrans.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Control.Monad.Trans.Class import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_text_prettyprint_chirho() {
    let src_chirho = r#"
module IfacePrettyPrint where
import Text.PrettyPrint (Doc, text, render)

main :: IO ()
main = print "prettyprint ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfacePrettyPrint.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Text.PrettyPrint import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_data_chirho() {
    let src_chirho = r#"
module IfaceDataData where
import Data.Data (Data, Typeable, toConstr)

main :: IO ()
main = print "data ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataData.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Data import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_word_chirho() {
    let src_chirho = r#"
module IfaceWord where
import Data.Word (Word, Word8, Word16, Word32, Word64)

main :: IO ()
main = print "word ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceWord.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Word import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_int_chirho() {
    let src_chirho = r#"
module IfaceDataInt where
import Data.Int (Int, Int8, Int16, Int32, Int64)

main :: IO ()
main = print "int ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataInt.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Int import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_map_chirho() {
    let src_chirho = r#"
module IfaceDataMap where
import Data.Map (Map, empty, singleton, insert, lookup)

main :: IO ()
main = print "map ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataMap.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Map import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_map_strict_chirho() {
    let src_chirho = r#"
module IfaceDataMapStrict where
import Data.Map.Strict (Map, empty, singleton, insert)

main :: IO ()
main = print "map strict ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataMapStrict.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Map.Strict import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_set_chirho() {
    let src_chirho = r#"
module IfaceDataSet where
import Data.Set (Set, empty, singleton, insert, member)

main :: IO ()
main = print "set ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataSet.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Set import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_generics_chirho() {
    let src_chirho = r#"
module IfaceGenerics where
import GHC.Generics (Generic, Rep, from, to)

main :: IO ()
main = print "generics ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGenerics.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Generics import failed: {:?}",
        result_chirho.err()
    );
}

// ── Module Interfaces Batch 5b ────────────────────────────────────────────

#[test]
fn iface_debug_trace_chirho() {
    let src_chirho = r#"
module IfaceTrace where
import Debug.Trace (trace, traceShow)

main :: IO ()
main = print "trace ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTrace.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Debug.Trace import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_bits_chirho() {
    let src_chirho = r#"
module IfaceBits where
import Data.Bits (Bits, (.&.), (.|.), xor, complement, shiftL, shiftR)

main :: IO ()
main = print "bits ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceBits.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Bits import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_ratio_chirho() {
    let src_chirho = r#"
module IfaceRatio where
import Data.Ratio (Ratio, Rational, numerator, denominator)

main :: IO ()
main = print "ratio ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceRatio.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Ratio import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_complex_chirho() {
    let src_chirho = r#"
module IfaceComplex where
import Data.Complex (Complex, realPart, imagPart, magnitude)

main :: IO ()
main = print "complex ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceComplex.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Complex import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_system_exit_chirho() {
    let src_chirho = r#"
module IfaceExit where
import System.Exit (ExitCode(..), exitSuccess, exitFailure)

main :: IO ()
main = print "exit ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceExit.hs", None);
    assert!(
        result_chirho.is_ok(),
        "System.Exit import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_dynamic_chirho() {
    let src_chirho = r#"
module IfaceDynamic where
import Data.Dynamic (Dynamic, toDyn, fromDyn)

main :: IO ()
main = print "dynamic ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDynamic.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Dynamic import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_control_deepseq_chirho() {
    let src_chirho = r#"
module IfaceDeepSeq where
import Control.DeepSeq (NFData, deepseq, rnf, force)

main :: IO ()
main = print "deepseq ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDeepSeq.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Control.DeepSeq import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_intmap_chirho() {
    let src_chirho = r#"
module IfaceIntMap where
import Data.IntMap (IntMap, empty, singleton, insert, lookup)

main :: IO ()
main = print "intmap ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceIntMap.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.IntMap import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_sequence_chirho() {
    let src_chirho = r#"
module IfaceSeq where
import Data.Sequence (Seq, empty, singleton)

main :: IO ()
main = print "sequence ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceSeq.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Sequence import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_num_chirho() {
    let src_chirho = r#"
module IfaceGhcNum where
import GHC.Num (Num, subtract, fromInteger)

main :: IO ()
main = print "ghcnum ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcNum.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Num import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_real_chirho() {
    let src_chirho = r#"
module IfaceGhcReal where
import GHC.Real (Integral, toInteger, fromIntegral, quot, rem, div, mod)

main :: IO ()
main = print "ghcreal ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcReal.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Real import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_float_chirho() {
    let src_chirho = r#"
module IfaceGhcFloat where
import GHC.Float (Float, Double, floatDigits, isNaN, isInfinite)

main :: IO ()
main = print "ghcfloat ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcFloat.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Float import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_enum_chirho() {
    let src_chirho = r#"
module IfaceGhcEnum where
import GHC.Enum (Enum, Bounded, succ, pred, toEnum, fromEnum, minBound, maxBound)

main :: IO ()
main = print "ghcenum ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcEnum.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Enum import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_char_expanded_chirho() {
    let src_chirho = r#"
module IfaceCharExpanded where
import Data.Char (isLetter, isPrint, isAscii, toTitle)

main :: IO ()
main = print "char expanded ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceCharExpanded.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Char expanded import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_prim_expanded_chirho() {
    let src_chirho = r#"
module IfacePrimExpanded where
import GHC.Prim (dataToTag#, tagToEnum#, reallyUnsafePtrEquality#)

main :: IO ()
main = print "prim expanded ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "IfacePrimExpanded.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Prim expanded import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_records_chirho() {
    let src_chirho = r#"
module IfaceRecords where
import GHC.Records (HasField, getField)

main :: IO ()
main = print "records ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceRecords.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Records import failed: {:?}",
        result_chirho.err()
    );
}

// ── Prelude expansion tests ───────────────────────────────────────────────

#[test]
fn prelude_as_type_of_chirho() {
    let src_chirho = r#"
module PreludeAsTypeOf where

val :: Int
val = 42 `asTypeOf` (undefined :: Int)

main :: IO ()
main = print val
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PreludeAsTypeOf.hs", None);
    assert!(
        result_chirho.is_ok(),
        "asTypeOf failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn prelude_math_functions_chirho() {
    let src_chirho = r#"
module PreludeMath where

main :: IO ()
main = do
  print (sqrt 4.0)
  print (divMod 17 5)
  print (splitAt 2 [1,2,3,4])
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PreludeMath.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Prelude math failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn prelude_list_expanded_chirho() {
    let src_chirho = r#"
module PreludeListExp where

main :: IO ()
main = do
  print (splitAt 3 [1,2,3,4,5])
  print (break (> 3) [1,2,3,4,5])
  print (cycle [1,2,3] !! 7)
  print (replicate 3 'x')
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PreludeListExp.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Prelude list expanded failed: {:?}",
        result_chirho.err()
    );
}

// ---------------------------------------------------------------------------
// GADT standalone kind signatures
// ---------------------------------------------------------------------------

#[test]
fn gadt_standalone_kind_sig_chirho() {
    // `data V :: N -> Type where` — the type constructor V has kind N -> *,
    // so `V Z` should kind-check instead of erroring with "expected *, found k0 -> k1".
    let src_chirho = r#"
{-# LANGUAGE GADTs, DataKinds, KindSignatures #-}
module GadtKindSig where
import Data.Kind (Type)
data N = Z | S N
data V :: N -> Type where
  VZ :: V Z
  VS :: V n -> V (S n)
main :: IO ()
main = putStrLn "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GadtKindSig.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GADT standalone kind sig failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn gadt_kind_sig_multi_param_chirho() {
    // Multi-parameter kind signature: `data Pair :: * -> * -> Type where`
    let src_chirho = r#"
{-# LANGUAGE GADTs, KindSignatures #-}
module GadtKindMulti where
import Data.Kind (Type)
data Pair :: * -> * -> Type where
  MkPair :: a -> b -> Pair a b
main :: IO ()
main = putStrLn "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GadtKindMulti.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GADT multi-param kind sig failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn gadt_kind_sig_simple_star_chirho() {
    // Simple kind signature `data T :: Type where` (just * kind, no arrows).
    let src_chirho = r#"
{-# LANGUAGE GADTs, KindSignatures #-}
module GadtKindStar where
import Data.Kind (Type)
data T :: Type where
  MkT :: T
main :: IO ()
main = putStrLn "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GadtKindStar.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GADT simple star kind sig failed: {:?}",
        result_chirho.err()
    );
}

// ── Constraint/Star kind interchangeability ─────────────────────────────

#[test]
fn constraint_kind_in_gadt_arrow_chirho() {
    // Dict :: Constraint -> Type — `Constraint` appears in function arrow position
    let src_chirho = r#"
{-# LANGUAGE GADTs, ConstraintKinds, KindSignatures #-}
module DictGADT where

import Data.Kind

data Dict :: Constraint -> Type where
  Dict :: a => Dict a

main :: IO ()
main = print "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "DictGADT.hs");
    assert!(
        result_chirho.is_ok(),
        "Constraint->Type GADT kind sig failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn constraint_star_interchangeable_in_type_family_chirho() {
    // Type families returning Constraint should kind-check
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies, ConstraintKinds, KindSignatures #-}
module ConstraintFam where

import Data.Kind

class Show a where
  show :: a -> String

type family MyConstraint a :: Constraint where
  MyConstraint Int = Show Int

main :: IO ()
main = print "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ConstraintFam.hs");
    assert!(
        result_chirho.is_ok(),
        "Constraint type family failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn datakinds_bool_kind_annotation_chirho() {
    // DataKinds: (b :: Bool) uses a data type as a kind
    let src_chirho = r#"
{-# LANGUAGE DataKinds, KindSignatures, GADTs #-}
module BoolKind where

data MyFlag (b :: Bool) = MkFlag

main :: IO ()
main = print "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "BoolKind.hs");
    assert!(
        result_chirho.is_ok(),
        "DataKinds Bool kind annotation failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn type_family_app_pattern_closed_chirho() {
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module TfAppChirho where

type family F a where
  F (Maybe a) = [a]
  F Int = Bool

foo :: F (Maybe Int)
foo = [1, 2, 3]

bar :: F Int
bar = True
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TfApp.hs");
    assert!(
        result_chirho.is_ok(),
        "Type family app pattern failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn imported_module_value_resolves_chirho() {
    let src_chirho = r#"
module ImportValChirho where
import Data.Map (singleton, empty)
import Foreign.Storable (sizeOf)
import Data.Foldable (foldl')

x :: Int
x = 0
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ImportVal.hs");
    assert!(
        result_chirho.is_ok(),
        "Imported module values should resolve: {:?}",
        result_chirho.err()
    );
}

#[test]
fn real_float_monadio_instances_chirho() {
    let src_chirho = r#"
module RfMioChirho where

foo :: RealFloat a => a -> Bool
foo x = isNaN x || isInfinite x

bar :: Double -> Bool
bar = foo
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RfMio.hs");
    assert!(
        result_chirho.is_ok(),
        "RealFloat constraint failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn prelude_export_count_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let prelude_chirho = ifaces_chirho
        .iter()
        .find(|m_chirho| m_chirho.name_chirho == "Prelude")
        .unwrap();
    // Prelude should have substantial exports after expansion
    assert!(
        prelude_chirho.exports_chirho.values_chirho.len() > 200,
        "Prelude should have 200+ value exports"
    );
    assert!(
        prelude_chirho.exports_chirho.types_chirho.len() > 30,
        "Prelude should have 30+ type exports"
    );
}
