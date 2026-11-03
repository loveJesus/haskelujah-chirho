// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equality implications and constructor-pattern evidence, measured with GHC 9.14.1.
//! Source bytes and commands are retained in equality-and-dictionary-oracle-r1-chirho.
//! The frontend verdicts are checked separately from Core and runtime behavior.
//! Workflow: language-features-chirho/stuck-family-equalities-chirho.md.

use super::{assert_prints_everywhere_chirho, checks_like_ghc_chirho};
use haskelujah_driver::typecheck_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

// GHC source SHA-256: 29a9d688a56b39b017d7e5bf50e4aceafe36a77dbec32fa5fed857111c3ac856
const INFERRED_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho

produceChirho :: ProxyChirho aChirho -> OpaqueChirho aChirho
produceChirho _ = undefined
sameChirho :: valueChirho -> valueChirho -> ()
sameChirho _ _ = ()
needChirho leftChirho rightChirho = sameChirho (produceChirho leftChirho) (produceChirho rightChirho)
main :: IO ()
main = print (needChirho (ProxyChirho :: ProxyChirho Int) (ProxyChirho :: ProxyChirho Int))
"##;

// GHC source SHA-256: bf80818aa2be25bab496188fdba172b68f37b6aeb78faf1a94787ca7910c88ae
const GIVEN_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho

produceChirho :: ProxyChirho aChirho -> OpaqueChirho aChirho
produceChirho _ = undefined
sameChirho :: valueChirho -> valueChirho -> ()
sameChirho _ _ = ()
needChirho :: (OpaqueChirho aChirho ~ OpaqueChirho bChirho) => ProxyChirho aChirho -> ProxyChirho bChirho -> ()
needChirho leftChirho rightChirho = sameChirho (produceChirho leftChirho) (produceChirho rightChirho)
main :: IO ()
main = print (needChirho (ProxyChirho :: ProxyChirho Int) (ProxyChirho :: ProxyChirho Int))
"##;

// GHC source SHA-256: a38d5990e13407ff850f7fbc3724f45b6267eac6bb7a6ec69f6c45604d0c8a85
const MISSING_GIVEN_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho

produceChirho :: ProxyChirho aChirho -> OpaqueChirho aChirho
produceChirho _ = undefined
sameChirho :: valueChirho -> valueChirho -> ()
sameChirho _ _ = ()
needChirho :: forall aChirho bChirho. ProxyChirho aChirho -> ProxyChirho bChirho -> ()
needChirho leftChirho rightChirho = sameChirho (produceChirho leftChirho) (produceChirho rightChirho)
main :: IO ()
main = print (needChirho (ProxyChirho :: ProxyChirho Int) (ProxyChirho :: ProxyChirho Int))
"##;

// GHC source SHA-256: 4d1069cbc222b4f5a96daa2f8278b109ac64e87f4cf6291750c00e7f75379a98
const MISSING_TYPE_ARGUMENT_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ScopedTypeVariables, TypeApplications, AllowAmbiguousTypes, FlexibleContexts #-}
module Main where

type family KnownChirho aChirho where
  KnownChirho Bool = Int
requireChirho :: forall aChirho. (KnownChirho aChirho ~ Int) => Int
requireChirho = 3
badChirho :: Int
badChirho = requireChirho
main :: IO ()
main = print badChirho
"##;

// GHC source SHA-256: 673ac02732ae1b128744e860848f5120d84b4312de3e72a7b3fb2e684493eea0
const VISIBLE_TYPE_ARGUMENT_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, ScopedTypeVariables, TypeApplications, AllowAmbiguousTypes, FlexibleContexts #-}
module Main where

type family KnownChirho aChirho where
  KnownChirho Bool = Int
requireChirho :: forall aChirho. (KnownChirho aChirho ~ Int) => Int
requireChirho = 3
goodChirho :: Int
goodChirho = requireChirho @Bool
main :: IO ()
main = print goodChirho
"##;

// GHC source SHA-256: fdd9648b680c5628f189952b8a42623c324b3a7e0ada2f8a477088528c2e30e3
const RESULT_EQUALITY_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: forall aChirho. ProxyChirho aChirho -> EqualChirho (OpaqueChirho aChirho) Int -> OpaqueChirho aChirho -> Int
castChirho _ ReflChirho valueChirho = valueChirho
main :: IO ()
main = print (castChirho (ProxyChirho :: ProxyChirho Bool) ReflChirho (3 :: Int))
"##;

// GHC source SHA-256: 5b46612acdde3558f3f964d5723f1ccd293bb492ae8403e8e7c74de03a0b71d3
const INHERITED_LOCAL_SIGNATURE_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: forall aChirho. ProxyChirho aChirho -> EqualChirho (OpaqueChirho aChirho) Int -> OpaqueChirho aChirho -> Int
castChirho _ ReflChirho valueChirho = goChirho valueChirho
  where
    goChirho :: OpaqueChirho aChirho -> Int
    goChirho innerChirho = innerChirho
main :: IO ()
main = print (castChirho (ProxyChirho :: ProxyChirho Bool) ReflChirho (3 :: Int))
"##;

// GHC source SHA-256: 89c07b9d55f6dda4088434da8e45041456b49e5c89576a5f90c8c0ee83a2cdf2
const CASE_RESULT_EQUALITY_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
castChirho :: forall aChirho. ProxyChirho aChirho -> EqualChirho (OpaqueChirho aChirho) Int -> OpaqueChirho aChirho -> Int
castChirho _ equalityChirho valueChirho = case equalityChirho of
  ReflChirho -> valueChirho
main :: IO ()
main = print (castChirho (ProxyChirho :: ProxyChirho Bool) ReflChirho (3 :: Int))
"##;

// GHC source SHA-256: fbb8a3dfdb1fa6a6b40761ccb29fc308d29224ee94d4de489ce575bb8b822329
const PREFIX_CONTEXT_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data BoxChirho aChirho where
  MkBoxChirho :: (OpaqueChirho aChirho ~ Int) => OpaqueChirho aChirho -> BoxChirho aChirho
readChirho :: forall aChirho. BoxChirho aChirho -> Int
readChirho (MkBoxChirho valueChirho) = valueChirho
main :: IO ()
main = print (readChirho (MkBoxChirho (4 :: Int) :: BoxChirho Bool))
"##;

// GHC source SHA-256: 1f9389f5f0a377fc2e81cb33fc6bfdca12e4dfb758b5f8aa9e47b32ba1c1fabb
const RECORD_CONTEXT_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data BoxChirho aChirho where
  MkBoxChirho :: (OpaqueChirho aChirho ~ Int) => { payloadChirho :: OpaqueChirho aChirho } -> BoxChirho aChirho
readChirho :: forall aChirho. BoxChirho aChirho -> Int
readChirho MkBoxChirho { payloadChirho = valueChirho } = valueChirho
main :: IO ()
main = print (readChirho (MkBoxChirho { payloadChirho = (4 :: Int) } :: BoxChirho Bool))
"##;

// GHC source SHA-256: 93d839787cdf8dc7d901cc2807513185f43d6ed7a3b6bc2700d92f95ddd930b5
const SCOPED_BRANCHES_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data BranchChirho aChirho where
  HasChirho :: (OpaqueChirho aChirho ~ Int) => BranchChirho aChirho
  NoChirho :: BranchChirho aChirho
useChirho :: forall aChirho. BranchChirho aChirho -> OpaqueChirho aChirho -> Int
useChirho HasChirho valueChirho = valueChirho
useChirho NoChirho _ = 0
main :: IO ()
main = print (useChirho (HasChirho :: BranchChirho Bool) (7 :: Int))
"##;

// GHC source SHA-256: 667510917dc172eea0415fff67057eef8a1787596c2a4715aa06d14fc78e1774
const LEAKED_BRANCH_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data BranchChirho aChirho where
  HasChirho :: (OpaqueChirho aChirho ~ Int) => BranchChirho aChirho
  NoChirho :: BranchChirho aChirho
useChirho :: forall aChirho. BranchChirho aChirho -> OpaqueChirho aChirho -> Int
useChirho HasChirho valueChirho = valueChirho
useChirho NoChirho valueChirho = valueChirho
main :: IO ()
main = print (useChirho (HasChirho :: BranchChirho Bool) (7 :: Int))
"##;

// GHC source SHA-256: 563aae35fec4b6c358881a0ea29f8c10b387985d15170e699aaf71910a9e97a7
const CASE_LEAKED_BRANCH_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, ScopedTypeVariables, FlexibleContexts #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
type family OpaqueChirho aChirho
type instance OpaqueChirho Bool = Int

data BranchChirho aChirho where
  HasChirho :: (OpaqueChirho aChirho ~ Int) => BranchChirho aChirho
  NoChirho :: BranchChirho aChirho
useChirho :: forall aChirho. BranchChirho aChirho -> OpaqueChirho aChirho -> Int
useChirho branchChirho valueChirho = case branchChirho of
  HasChirho -> valueChirho
  NoChirho -> valueChirho
main :: IO ()
main = print (useChirho (HasChirho :: BranchChirho Bool) (7 :: Int))
"##;

// GHC source SHA-256: 1bcdb35fd6a5d65ef4004cf9ec48661067bd8571d6d2a5131c1edec29464591b
const DISCARD_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ScopedTypeVariables #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
data SomeChirho where
  SomeChirho :: forall aChirho. ProxyChirho aChirho -> SomeChirho
discardChirho :: SomeChirho -> ()
discardChirho (SomeChirho _) = ()
main :: IO ()
main = print (discardChirho (SomeChirho (ProxyChirho :: ProxyChirho Bool)))
"##;

// GHC source SHA-256: 1187c7817e0f8dbdeb974c351db3f8477e35f33f673da9254f17faa1fade446b
const FIXED_RESULT_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ScopedTypeVariables #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
data SomeChirho where
  SomeChirho :: forall aChirho. ProxyChirho aChirho -> SomeChirho
badChirho :: SomeChirho -> ProxyChirho Int
badChirho (SomeChirho valueChirho) = valueChirho
main :: IO ()
main = print ()
"##;

// GHC source SHA-256: cfad652fe78a267236e42585a6d0f0c524ff4d5f170473867fdd920974fd8dbb
const ESCAPE_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, ScopedTypeVariables #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
data SomeChirho where
  SomeChirho :: forall aChirho. ProxyChirho aChirho -> SomeChirho
escapeChirho (SomeChirho valueChirho) = valueChirho
main :: IO ()
main = print ()
"##;

// GHC source SHA-256: b5ea273e1ed0d41e86d8abdb6ebb56295e1848ebe927b355ac293ef1e4c6e423
const UNIVERSAL_RETURN_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs #-}
module Main where

data ProxyChirho aChirho = ProxyChirho
data IndexedChirho aChirho where
  IndexedChirho :: ProxyChirho aChirho -> IndexedChirho aChirho
getChirho :: IndexedChirho aChirho -> ProxyChirho aChirho
getChirho (IndexedChirho valueChirho) = valueChirho
main :: IO ()
main = print (case getChirho (IndexedChirho (ProxyChirho :: ProxyChirho Bool)) of ProxyChirho -> ())
"##;

macro_rules! equality_positive_cases_chirho {
    ($(($test_chirho:ident, $file_chirho:literal, $source_chirho:ident, $stdout_chirho:literal)),* $(,)?) => {
        const POSITIVES_CHIRHO: &[(&str, &str, &str)] = &[$(($file_chirho, $source_chirho, $stdout_chirho)),*];
        $(#[test] fn $test_chirho() { assert_prints_everywhere_chirho($file_chirho, $source_chirho, $stdout_chirho); })*
    };
}

equality_positive_cases_chirho! {
    (inferred_runs_on_every_engine_chirho, "InferredChirho.hs", INFERRED_CHIRHO, "()\n"),
    (given_runs_on_every_engine_chirho, "GivenChirho.hs", GIVEN_CHIRHO, "()\n"),
    (visible_type_argument_runs_on_every_engine_chirho, "VisibleTypeArgumentChirho.hs", VISIBLE_TYPE_ARGUMENT_CHIRHO, "3\n"),
    (result_equality_runs_on_every_engine_chirho, "ResultEqualityChirho.hs", RESULT_EQUALITY_CHIRHO, "3\n"),
    (inherited_local_signature_runs_on_every_engine_chirho, "InheritedLocalSignatureChirho.hs", INHERITED_LOCAL_SIGNATURE_CHIRHO, "3\n"),
    (case_result_equality_runs_on_every_engine_chirho, "CaseResultEqualityChirho.hs", CASE_RESULT_EQUALITY_CHIRHO, "3\n"),
    (prefix_context_runs_on_every_engine_chirho, "PrefixContextChirho.hs", PREFIX_CONTEXT_CHIRHO, "4\n"),
    (record_context_runs_on_every_engine_chirho, "RecordContextChirho.hs", RECORD_CONTEXT_CHIRHO, "4\n"),
    (scoped_branches_runs_on_every_engine_chirho, "ScopedBranchesChirho.hs", SCOPED_BRANCHES_CHIRHO, "7\n"),
    (discard_runs_on_every_engine_chirho, "DiscardChirho.hs", DISCARD_CHIRHO, "()\n"),
    (universal_return_runs_on_every_engine_chirho, "UniversalReturnChirho.hs", UNIVERSAL_RETURN_CHIRHO, "()\n"),
}

macro_rules! equality_negative_cases_chirho {
    ($(($test_chirho:ident, $file_chirho:literal, $source_chirho:ident)),* $(,)?) => {
        $(#[test] fn $test_chirho() {
            let result_chirho = typecheck_source_chirho($source_chirho, &mut SourceMapChirho::new_chirho(), $file_chirho);
            assert!(result_chirho.is_err(), "{}: GHC rejects this; the frontend accepted it", $file_chirho);
        })*
    };
}

equality_negative_cases_chirho! {
    (missing_given_is_rejected_by_the_frontend_chirho, "MissingGivenChirho.hs", MISSING_GIVEN_CHIRHO),
    (missing_type_argument_is_rejected_by_the_frontend_chirho, "MissingTypeArgumentChirho.hs", MISSING_TYPE_ARGUMENT_CHIRHO),
    (leaked_branch_is_rejected_by_the_frontend_chirho, "LeakedBranchChirho.hs", LEAKED_BRANCH_CHIRHO),
    (case_leaked_branch_is_rejected_by_the_frontend_chirho, "CaseLeakedBranchChirho.hs", CASE_LEAKED_BRANCH_CHIRHO),
    (fixed_result_is_rejected_by_the_frontend_chirho, "FixedResultChirho.hs", FIXED_RESULT_CHIRHO),
    (escape_is_rejected_by_the_frontend_chirho, "EscapeChirho.hs", ESCAPE_CHIRHO),
}

#[test]
fn supplied_equality_and_pattern_evidence_checks_like_ghc_chirho() {
    for (file_chirho, source_chirho, _) in POSITIVES_CHIRHO {
        checks_like_ghc_chirho(file_chirho, source_chirho);
    }
}

#[test]
fn the_three_measured_corpus_equality_negatives_stay_rejected_chirho() {
    for (file_chirho, source_chirho) in [
        (
            "T16512a.hs",
            include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T16512a.hs"),
        ),
        (
            "T23162b.hs",
            include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T23162b.hs"),
        ),
        (
            "T23162d.hs",
            include_str!("../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T23162d.hs"),
        ),
    ] {
        let result_chirho = typecheck_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            file_chirho,
        );
        assert!(
            result_chirho.is_err(),
            "{file_chirho}: fresh GHC rejects this; the frontend accepted it"
        );
    }
}
