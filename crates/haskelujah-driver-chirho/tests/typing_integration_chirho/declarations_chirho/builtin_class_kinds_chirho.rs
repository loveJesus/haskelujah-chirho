// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! K01 C: builtin class kinds through the driver. Each program here was measured by GHC
//! 9.14.1 in the oracle slot of #31202 (receipts `oracle-classes-receipts-r1-chirho`) and
//! is embedded byte for byte, with its sha256 prefix. Holds must type-check; negatives
//! must be rejected for the kind their builtin class needs. T5120 is the
//! parenthesized-element regression: GHC accepts it, and its signature must keep its whole
//! context. CH02 and CH07 are GHC-valid, but baseline ce999556 already rejects their
//! `sum` and `concatMap` calls (E0200, Foldable method dispatch), so this unit leaves them
//! unclaimed; their declaration-only twins CH02D and CH07D carry the kind check.
//! workflow: compiler-pipeline-chirho/builtin-class-identity-chirho

use haskelujah_span_chirho::SourceMapChirho;

use super::rejected_like_ghc_chirho;

/// GHC accepts the program, so the checker must too.
fn checks_like_ghc_chirho(file_name_chirho: &str, source_chirho: &str) {
    if let Err(error_chirho) = haskelujah_driver::typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
    ) {
        panic!("{file_name_chirho}: GHC accepts this program; we rejected it: {error_chirho}");
    }
}

/// CH01EitherTwinFunctorChirho (sha256 35d60f93): a legal twin of `Functor (Either e)` under FlexibleInstances. GHC 9.14.1: accepted, prints 2.
const CH01_EITHER_TWIN_FUNCTOR_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CH01 (#31176): legal twin of `Functor (Either e)` under FlexibleInstances,
-- so gate (iii) judges a resolved `* -> *` argument
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints 2. Predicted under S+C: accept.
{-# LANGUAGE FlexibleInstances #-}
module Main where

data EitherTwinChirho e a = LeftTwinChirho e | RightTwinChirho a

instance Functor (EitherTwinChirho e) where
  fmap _ (LeftTwinChirho errorChirho) = LeftTwinChirho errorChirho
  fmap functionChirho (RightTwinChirho valueChirho) = RightTwinChirho (functionChirho valueChirho)

main :: IO ()
main = case fmap (+ 1) (RightTwinChirho 1 :: EitherTwinChirho String Int) of
  RightTwinChirho valueChirho -> print valueChirho
  LeftTwinChirho _ -> putStrLn "left"
"##;

/// CH03ReaderTwinFunctorChirho (sha256 39f6bba4): a legal twin of `Functor ((->) r)` under FlexibleInstances. GHC 9.14.1: accepted, prints 6.
const CH03_READER_TWIN_FUNCTOR_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CH03 (#31176): legal twin of `Functor ((->) r)` under FlexibleInstances
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints 6. Predicted under S+C: accept.
{-# LANGUAGE FlexibleInstances #-}
module Main where

newtype ReaderTwinChirho r a = ReaderTwinChirho (r -> a)

instance Functor (ReaderTwinChirho r) where
  fmap functionChirho (ReaderTwinChirho readChirho) = ReaderTwinChirho (functionChirho . readChirho)

main :: IO ()
main = case fmap (* 2) (ReaderTwinChirho length) of
  ReaderTwinChirho readChirho -> print (readChirho "abc")
"##;

/// CH04QualifiedFunctorOwnTypeChirho (sha256 0c6912a8): a qualified builtin class at an own type. GHC 9.14.1: accepted, prints 42.
const CH04_QUALIFIED_FUNCTOR_OWN_TYPE_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CH04 (#31176): a qualified builtin class at a fresh own type, no extensions
-- (the representable P3 path)
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints 42. Predicted under S+C: accept.
module Main where

import Prelude
import qualified Prelude as P

newtype BoxChirho a = BoxChirho a

instance P.Functor BoxChirho where
  fmap functionChirho (BoxChirho valueChirho) = BoxChirho (functionChirho valueChirho)

main :: IO ()
main = case P.fmap (+ 1) (BoxChirho (41 :: Int)) of
  BoxChirho valueChirho -> print valueChirho
"##;

/// CH05LocalFunctorHomonymChirho (sha256 18b0ce46): a local class named Functor of kind `* -> Constraint`. GHC 9.14.1: accepted, prints local functor.
const CH05_LOCAL_FUNCTOR_HOMONYM_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CH05 (#31176): a local class named Functor of kind `* -> Constraint`;
-- the bare spelling must never be the builtin class
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints local functor. Predicted under S+C: accept.
module Main where

import Prelude hiding (Functor)

class Functor a where
  describeChirho :: a -> String

instance Functor Bool where
  describeChirho _ = "local functor"

main :: IO ()
main = putStrLn (describeChirho True)
"##;

/// CH06VariableHeadShowChirho (sha256 aa8e8ae0): a variable-headed builtin instance, left unjudged. GHC 9.14.1: accepted, prints applied.
const CH06_VARIABLE_HEAD_SHOW_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CH06 (#31176): a variable-headed builtin instance under FlexibleInstances;
-- its argument kind never resolves, so it stays unjudged
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints applied. Predicted under S+C: accept.
{-# LANGUAGE FlexibleInstances #-}
module Main where

newtype BoxChirho a = BoxChirho a

instance Show (f a) where
  show _ = "applied"

main :: IO ()
main = putStrLn (show (BoxChirho 'x'))
"##;

/// CH02DPairTwinFoldableDeclChirho (sha256 d1b30b0c): CH02's declarations with `main = pure ()`, so no Foldable method is dispatched at a use. GHC 9.14.1: accepted.
const CH02D_PAIR_TWIN_FOLDABLE_DECL_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CH02D (#31229): CH02's declarations unchanged (the instance and its
-- foldr body), with `main = pure ()` so no Foldable method is dispatched at a use. CH02 itself
-- stays an unclaimed boundary: baseline ce999556 rejects its `sum` call (E0200).
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints nothing. Predicted under S+C: accept.
{-# LANGUAGE PolyKinds #-}
module Main where

data PairTwinChirho a b = PairTwinChirho a b

instance Foldable (PairTwinChirho a) where
  foldr functionChirho seedChirho (PairTwinChirho _ valueChirho) = functionChirho valueChirho seedChirho

main :: IO ()
main = pure ()
"##;

/// CH07DContextSignatureDeclChirho (sha256 a63a27d1): CH07's signature context with a body that calls no Foldable method. GHC 9.14.1: accepted.
const CH07D_CONTEXT_SIGNATURE_DECL_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CH07D (#31229): CH07's signature context unchanged, with a body that
-- calls no Foldable method and `main = pure ()`. CH07 itself stays an unclaimed boundary:
-- baseline ce999556 rejects its `concatMap` call (E0200).
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints nothing. Predicted under S+C: accept.
module Main where

describeAllChirho :: (Show a, Functor f, Foldable f) => f a -> String
describeAllChirho _ = "held"

main :: IO ()
main = pure ()
"##;

/// T5120 (sha256 9e90d1e0): typecheck should_compile T5120, as GHC measured it. GHC 9.14.1: accepted.
const T5120_CHIRHO: &str = r##"{-# LANGUAGE ImplicitParams #-}
{-# LANGUAGE TypeFamilies #-}
{-# LANGUAGE NoMonomorphismRestriction #-}

module Test where

class C t where
    type TF t
    ttt :: TF t -> t

b :: (C t, ?x :: TF t) => t
b = ttt ?x
"##;

/// CN01FunctorOnNullaryChirho (sha256 8f414108): T1633's shape at a fresh type, under FlexibleInstances. GHC 9.14.1: GHC-83865.
const CN01_FUNCTOR_ON_NULLARY_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CN01 (#31176): T1633's shape at a fresh type, under FlexibleInstances
-- Unmeasured. Predicted GHC 9.14.1: rejected, GHC-83865. Predicted under S+C: reject (gate iii).
{-# LANGUAGE FlexibleInstances #-}
module Main where

data FlagChirho = FlagChirho

instance Functor FlagChirho

main :: IO ()
main = pure ()
"##;

/// CN02EqOnUnaryChirho (sha256 d6f3486e): T11464's shape at a fresh type, under PolyKinds. GHC 9.14.1: GHC-83865.
const CN02_EQ_ON_UNARY_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CN02 (#31176): T11464's shape at a fresh type, under PolyKinds
-- Unmeasured. Predicted GHC 9.14.1: rejected, GHC-83865. Predicted under S+C: reject (gate iii).
{-# LANGUAGE PolyKinds #-}
module Main where

data EitherTwinChirho e a = LeftTwinChirho e | RightTwinChirho a

instance Eq (EitherTwinChirho e)

main :: IO ()
main = pure ()
"##;

/// CN03QualifiedFunctorOnNullaryChirho (sha256 33ea5b25): a qualified builtin class head. GHC 9.14.1: GHC-83865.
const CN03_QUALIFIED_FUNCTOR_ON_NULLARY_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CN03 (#31176): a qualified builtin class head, no extensions
-- Unmeasured. Predicted GHC 9.14.1: rejected, GHC-83865. Predicted under S+C: reject (P3 by full spelling).
module Main where

import Prelude
import qualified Prelude as P

data FlagChirho = FlagChirho

instance P.Functor FlagChirho

main :: IO ()
main = pure ()
"##;

/// CN04FoldableOnNullaryChirho (sha256 d6f7a103): Foldable at a type of kind `*`. GHC 9.14.1: GHC-83865.
const CN04_FOLDABLE_ON_NULLARY_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CN04 (#31176): Foldable at a type of kind `*`, no extensions
-- Unmeasured. Predicted GHC 9.14.1: rejected, GHC-83865. Predicted under S+C: reject (P3).
module Main where

newtype CountChirho = CountChirho Int

instance Foldable CountChirho

main :: IO ()
main = pure ()
"##;

/// CN05QualifiedShowFieldChirho (sha256 b9b990fe): a qualified builtin class as a newtype field. GHC 9.14.1: GHC-83865.
const CN05_QUALIFIED_SHOW_FIELD_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 C head oracle CN05 (#31176): a qualified builtin class as a newtype field
-- (UnliftedNewtypesFail through a qualifier)
-- Unmeasured. Predicted GHC 9.14.1: rejected, GHC-83865. Predicted under S+C: reject (S gate with the C scope).
module Main where

import Prelude
import qualified Prelude as P

newtype BazChirho = BazChirho (P.Show Int)

main :: IO ()
main = pure ()
"##;

#[test]
fn builtin_class_holds_check_like_ghc_chirho() {
    for (file_name_chirho, source_chirho) in [
        (
            "CH01EitherTwinFunctorChirho.hs",
            CH01_EITHER_TWIN_FUNCTOR_CHIRHO,
        ),
        (
            "CH02DPairTwinFoldableDeclChirho.hs",
            CH02D_PAIR_TWIN_FOLDABLE_DECL_CHIRHO,
        ),
        (
            "CH03ReaderTwinFunctorChirho.hs",
            CH03_READER_TWIN_FUNCTOR_CHIRHO,
        ),
        (
            "CH04QualifiedFunctorOwnTypeChirho.hs",
            CH04_QUALIFIED_FUNCTOR_OWN_TYPE_CHIRHO,
        ),
        (
            "CH05LocalFunctorHomonymChirho.hs",
            CH05_LOCAL_FUNCTOR_HOMONYM_CHIRHO,
        ),
        (
            "CH06VariableHeadShowChirho.hs",
            CH06_VARIABLE_HEAD_SHOW_CHIRHO,
        ),
        (
            "CH07DContextSignatureDeclChirho.hs",
            CH07D_CONTEXT_SIGNATURE_DECL_CHIRHO,
        ),
        ("T5120.hs", T5120_CHIRHO),
    ] {
        checks_like_ghc_chirho(file_name_chirho, source_chirho);
    }
}

#[test]
fn builtin_class_heads_of_another_kind_are_rejected_like_ghc_chirho() {
    for (file_name_chirho, source_chirho) in [
        (
            "CN01FunctorOnNullaryChirho.hs",
            CN01_FUNCTOR_ON_NULLARY_CHIRHO,
        ),
        ("CN02EqOnUnaryChirho.hs", CN02_EQ_ON_UNARY_CHIRHO),
        (
            "CN03QualifiedFunctorOnNullaryChirho.hs",
            CN03_QUALIFIED_FUNCTOR_ON_NULLARY_CHIRHO,
        ),
        (
            "CN04FoldableOnNullaryChirho.hs",
            CN04_FOLDABLE_ON_NULLARY_CHIRHO,
        ),
    ] {
        rejected_like_ghc_chirho(
            source_chirho,
            file_name_chirho,
            &["kind mismatch in the instance head"],
        );
    }
}

#[test]
fn a_qualified_builtin_class_field_is_rejected_like_ghc_chirho() {
    rejected_like_ghc_chirho(
        CN05_QUALIFIED_SHOW_FIELD_CHIRHO,
        "CN05QualifiedShowFieldChirho.hs",
        &["expected a type", "is a constraint"],
    );
}
