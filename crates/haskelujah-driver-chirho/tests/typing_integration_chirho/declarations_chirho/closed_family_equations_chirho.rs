// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 686 closed type-family equation controls (686a carrier).
//!
//! Each program is the row 686 oracle file GHC 9.14.1 measured, embedded byte for
//! byte (its sha256 prefix is noted). The negatives run the whole frontend and must
//! fail for GHC's reason; the positives must check, and run where GHC printed.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use haskelujah_span_chirho::SourceMapChirho;

use super::{assert_prints_everywhere_chirho, checks_like_ghc_chirho, rejected_like_ghc_chirho};

/// `R01UnboundHeadChirho.hs`, sha256 eceb7f2d3f488110.
const R01_UNBOUND_HEAD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R01 (686a): the equation head names an unbound type. GHC message UNKNOWN: not in scope, or mismatched type name.
{-# LANGUAGE TypeFamilies #-}
module R01UnboundHeadChirho where

type family FChirho aChirho where
  GChirho aChirho = Int
"#;

/// `R02ArityManyChirho.hs`, sha256 05dca4e0f5bc23ad.
const R02_ARITY_MANY_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R02 (686a): one argument too many. Predicted GHC: "Number of parameters must match family declaration; expected 1".
{-# LANGUAGE TypeFamilies #-}
module R02ArityManyChirho where

type family FChirho aChirho where
  FChirho Int Bool = Char
"#;

/// `R03ArityFewChirho.hs`, sha256 b5f1fddade91bbc9.
const R03_ARITY_FEW_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R03 (686a): one argument too few, with a kind-correct right side. Predicted GHC: "Number of parameters must match family declaration; expected 2".
{-# LANGUAGE TypeFamilies #-}
module R03ArityFewChirho where

type family FChirho aChirho bChirho where
  FChirho Int = Bool
"#;

/// `R04VariableHeadChirho.hs`, sha256 4e417f0724c251e2.
const R04_VARIABLE_HEAD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R04 (686a): a type variable heads the left side. GHC message UNKNOWN (malformed head or parse error).
{-# LANGUAGE TypeFamilies #-}
module R04VariableHeadChirho where

type family FChirho aChirho where
  aChirho Int = Bool
"#;

/// `R05LiteralHeadChirho.hs`, sha256 ae9d843983a36d49.
const R05_LITERAL_HEAD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R05 (686a): a type-level literal is the whole left side. GHC message UNKNOWN.
{-# LANGUAGE TypeFamilies, DataKinds #-}
module R05LiteralHeadChirho where

type family FChirho aChirho where
  3 = Int
"#;

/// `R06MissingEqualsChirho.hs`, sha256 7648302003738df0.
const R06_MISSING_EQUALS_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R06 (686a): an equation with no `=`. Predicted GHC: parse error.
{-# LANGUAGE TypeFamilies #-}
module R06MissingEqualsChirho where

type family FChirho aChirho where
  FChirho Int
"#;

/// `R07ContextChirho.hs`, sha256 6ea917ec305dc053.
const R07_CONTEXT_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R07 (686a): an equation with a context. Predicted GHC: parse error.
{-# LANGUAGE TypeFamilies #-}
module R07ContextChirho where

type family FChirho aChirho where
  Show aChirho => FChirho aChirho = Int
"#;

/// `R08QualifiedHeadChirho.hs`, sha256 5a893c2c841c515f.
const R08_QUALIFIED_HEAD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R08 (686a): the head is qualified with its own module. GHC verdict UNKNOWN: if accepted, the head check must compare unqualified names.
{-# LANGUAGE TypeFamilies #-}
module R08QualifiedHeadChirho where

type family FChirho aChirho where
  R08QualifiedHeadChirho.FChirho Int = Bool
"#;

/// `R10UsedForallChirho.hs`, sha256 56f124c085c47e62.
const R10_USED_FORALL_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle R10 (#30844): R09 with an actual use, so peeling the forall cannot newly
-- make this invalid program compile or run. Predicted GHC 9.14.1: rejected, GHC-30337.
{-# LANGUAGE TypeFamilies, ExplicitForAll #-}
module Main where

type family GChirho where
  forall bChirho. GChirho = ()

valueChirho :: GChirho
valueChirho = ()

main :: IO ()
main = print valueChirho
"#;

/// `A06ShapesChirho.hs`, sha256 cab1fc01a6c3c90c.
const A06_SHAPES_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle A06 (686a): a left side continued on the next line (the CST value-decl
-- guard stops a btype at a new-line VarId followed by `=`), a kind-annotated pattern, and an
-- overlapped equation that is unreachable but legal.
-- Predicted GHC 9.14.1 output: (1,'c'), True, False, 3 (OverChirho Bool may warn).
{-# LANGUAGE DataKinds, TypeFamilies, KindSignatures, ScopedTypeVariables #-}
module Main where

import Data.Kind (Type)
import Data.Proxy (Proxy (..))

class KnownBoolChirho (bChirho :: Bool) where
  boolValChirho :: Proxy bChirho -> Bool

instance KnownBoolChirho True where
  boolValChirho _ = True

instance KnownBoolChirho False where
  boolValChirho _ = False

type family PairChirho aChirho bChirho where
  PairChirho aChirho
    bChirho = (aChirho, bChirho)

type family IsBoolChirho (aChirho :: Type) :: Bool where
  IsBoolChirho (Bool :: Type) = True
  IsBoolChirho aChirho = False

type family OverChirho aChirho where
  OverChirho aChirho = Int
  OverChirho Bool = Char

main :: IO ()
main = do
  print ((1, 'c') :: PairChirho Int Char)
  print (boolValChirho (Proxy :: Proxy (IsBoolChirho Bool)))
  print (boolValChirho (Proxy :: Proxy (IsBoolChirho Int)))
  print (3 :: OverChirho Bool)
"#;

/// `A07ParenHeadChirho.hs`, sha256 76dffc2ac037bddf.
const A07_PAREN_HEAD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 oracle A07 (686a): a parenthesized partial head. GHC's verdict is UNKNOWN; this
-- case decides whether head decomposition sees through parentheses or reports a malformed head.
-- If GHC accepts: predicted output (1,'c').
{-# LANGUAGE TypeFamilies #-}
module Main where

type family TwoChirho aChirho bChirho where
  (TwoChirho aChirho) bChirho = (aChirho, bChirho)

main :: IO ()
main = print ((1, 'c') :: TwoChirho Int Char)
"#;

/// The frontend's error text for a program it must reject.
fn rejection_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
    match haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
    ) {
        Ok(_) => panic!("{file_name_chirho}: GHC rejects this program; we accepted it"),
        Err(error_chirho) => error_chirho.to_string(),
    }
}

#[test]
fn an_equation_head_must_name_a_type_in_scope_first_chirho() {
    // GHC-76037: the renamer reports the unbound head before any validity rule.
    let message_chirho = rejection_chirho("R01UnboundHeadChirho.hs", R01_UNBOUND_HEAD_CHIRHO);
    assert!(
        message_chirho.contains("GChirho"),
        "naming names the head: {message_chirho}"
    );
    assert!(
        !message_chirho.contains("E0209"),
        "naming errors come first: {message_chirho}"
    );
}

#[test]
fn an_equation_naming_another_type_is_mismatched_chirho() {
    for (file_chirho, source_chirho) in [
        (
            "T11623.hs",
            include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T11623.hs"),
        ),
        (
            "T20260.hs",
            include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T20260.hs"),
        ),
    ] {
        rejected_like_ghc_chirho(source_chirho, file_chirho, &["E0209", "GHC-88221"]);
    }
}

#[test]
fn closed_equations_keep_the_declared_argument_count_chirho() {
    rejected_like_ghc_chirho(
        R02_ARITY_MANY_CHIRHO,
        "R02ArityManyChirho.hs",
        &["E0209", "GHC-12985", "expected 1"],
    );
    rejected_like_ghc_chirho(
        R03_ARITY_FEW_CHIRHO,
        "R03ArityFewChirho.hs",
        &["E0209", "GHC-12985", "expected 2"],
    );
}

#[test]
fn an_equation_without_a_type_constructor_head_is_malformed_chirho() {
    rejected_like_ghc_chirho(
        R04_VARIABLE_HEAD_CHIRHO,
        "R04VariableHeadChirho.hs",
        &["E0209", "GHC-47568"],
    );
    rejected_like_ghc_chirho(
        R05_LITERAL_HEAD_CHIRHO,
        "R05LiteralHeadChirho.hs",
        &["E0209", "GHC-47568"],
    );
}

#[test]
fn an_incomplete_equation_is_a_parse_error_chirho() {
    rejected_like_ghc_chirho(
        R06_MISSING_EQUALS_CHIRHO,
        "R06MissingEqualsChirho.hs",
        &["E0209", "GHC-58481"],
    );
    rejected_like_ghc_chirho(
        R07_CONTEXT_CHIRHO,
        "R07ContextChirho.hs",
        &["E0209", "GHC-58481"],
    );
}

#[test]
fn an_unused_forall_binder_does_not_newly_compile_chirho() {
    // GHC-30337 is not a row 686 rule, but the equation is held: it never
    // reduces, so `GChirho` stays stuck and the use is still refused.
    let message_chirho = rejection_chirho("R10UsedForallChirho.hs", R10_USED_FORALL_CHIRHO);
    assert!(
        message_chirho.contains("GChirho"),
        "the held family stays stuck: {message_chirho}"
    );
}

#[test]
fn self_qualified_and_parenthesized_heads_stay_legal_chirho() {
    checks_like_ghc_chirho("R08QualifiedHeadChirho.hs", R08_QUALIFIED_HEAD_CHIRHO);
    assert_prints_everywhere_chirho("A07ParenHeadChirho.hs", A07_PAREN_HEAD_CHIRHO, "(1,'c')\n");
}

#[test]
fn a_continued_left_side_and_an_overlapped_equation_keep_running_chirho() {
    assert_prints_everywhere_chirho(
        "A06ShapesChirho.hs",
        A06_SHAPES_CHIRHO,
        "(1,'c')\nTrue\nFalse\n3\n",
    );
}

#[test]
fn split_wd_checks_once_its_infix_equations_have_their_heads_chirho() {
    checks_like_ghc_chirho(
        "SplitWD.hs",
        include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/SplitWD.hs"),
    );
}

/// `K02KindWithoutKindChirho.hs`, sha256 1770b46f2287ebb6: a right-side `::` with no kind, which
/// GHC rejects as a parse error (prediction; the K set is queued for GHC 9.14.1).
const K02_KIND_WITHOUT_KIND_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 follow-up control K02 (#31070): the malformed boundary of a right-side kind
-- annotation, a `::` with no kind. Predicted GHC 9.14.1: rejected (parse error).
{-# LANGUAGE TypeFamilies #-}
module Main where

type family FooChirho a where
  FooChirho a = Int ::

main :: IO ()
main = pure ()
"#;

#[test]
fn a_right_side_kind_annotation_checks_like_ghc_chirho() {
    // KindSigs, which GHC accepts: `Foo a = Int :: Type` is one equation, where the
    // 686a carrier read its annotation as a second, incomplete one.
    checks_like_ghc_chirho(
        "KindSigs.hs",
        include_str!("../../../../../ghc-tests-chirho/parser-chirho/should_compile/KindSigs.hs"),
    );
}

#[test]
fn a_kind_annotation_without_a_kind_stays_incomplete_chirho() {
    rejected_like_ghc_chirho(
        K02_KIND_WITHOUT_KIND_CHIRHO,
        "K02KindWithoutKindChirho.hs",
        &["E0209", "GHC-58481"],
    );
}
