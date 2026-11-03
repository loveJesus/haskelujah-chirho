// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Explicit lazy field annotations (`~T`, K01 follow-up #31236/#31240).
//!
//! A leading `~` marks a field explicitly lazy. The parser keeps it apart from
//! the unannotated default, every strictness consumer treats it as lazy, and
//! declaration validity judges where GHC allows it (rules 5 and 6). This is the
//! explicit override only: the module-wide `StrictData` default (unannotated
//! fields strict) is not implemented. The holds therefore print GHC's output
//! whichever way the default goes; what they prove is that a `~` field holding
//! `undefined`, or omitted, is never forced.
//!
//! The L programs are embedded byte for byte from the oracle files under
//! `tmp-chirho/r686c-followup-chirho/oracle-lazy-*-chirho/`; each header comment
//! keeps the prediction written before GHC ran. L01-L08 were measured on GHC
//! 9.14.1 by root (receipt `tmp-chirho/lazy-gadt-followup-oracle-chirho/receipts-chirho.json`)
//! and every prediction matched. L07, a `~` GADT argument under StrictData, is
//! GHC-valid but unclaimed: GADT constructor arguments carry no strictness.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use haskelujah_span_chirho::SourceMapChirho;

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

/// L01 (sha256 7fc8e296): GHC 9.14.1: accepted, prints 3.
const L01_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy field oracle L01 (#31236): a positional `~` field under StrictData stays lazy
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints 3.
{-# LANGUAGE StrictData #-}
module Main where

data PairChirho = PairChirho ~Int Int

main :: IO ()
main = case PairChirho undefined 3 of
  PairChirho _ countChirho -> print countChirho
"##;

/// L02 (sha256 f4bcb86a): GHC 9.14.1: accepted, prints 4.
const L02_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy field oracle L02 (#31236): a record `~` field under StrictData stays lazy (LazyFieldsDisabled with StrictData)
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints 4.
{-# LANGUAGE StrictData #-}
module Main where

data BoxChirho = BoxChirho { lazyChirho :: ~Int, strictChirho :: Int }

main :: IO ()
main = case BoxChirho { lazyChirho = undefined, strictChirho = 4 } of
  BoxChirho _ countChirho -> print countChirho
"##;

/// L03 (sha256 ddeff89d): GHC 9.14.1: accepted, prints 5.
const L03_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy field oracle L03 (#31236): a positional `~` field under Strict (which implies StrictData)
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints 5.
{-# LANGUAGE Strict #-}
module Main where

data PairChirho = PairChirho ~Int Int

main :: IO ()
main = case PairChirho undefined 5 of
  PairChirho _ countChirho -> print countChirho
"##;

/// L04 (sha256 bf9546f6): GHC 9.14.1: rejected, GHC-04049.
const L04_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy field oracle L04 (#31236): a `~` on a newtype field under StrictData
-- Unmeasured. Predicted GHC 9.14.1: rejected (a newtype constructor takes no strictness annotation).
{-# LANGUAGE StrictData #-}
module Main where

newtype WrapChirho = WrapChirho ~Int

main :: IO ()
main = pure ()
"##;

/// L05 (sha256 3fc58695): GHC 9.14.1: rejected, GHC-81601.
const L05_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy field oracle L05 (#31236): a positional `~` field without StrictData
-- Unmeasured. Predicted GHC 9.14.1: rejected, GHC-81601.
module Main where

data PairChirho = PairChirho ~Int Int

main :: IO ()
main = pure ()
"##;

/// L06 (sha256 1a1eddb8): GHC 9.14.1: rejected, GHC-81601.
const L06_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy field oracle L06 (#31236): a record `~` field without StrictData (LazyFieldsDisabled's shape)
-- Unmeasured. Predicted GHC 9.14.1: rejected, GHC-81601.
module Main where

data BoxChirho = BoxChirho { lazyChirho :: ~Int }

main :: IO ()
main = pure ()
"##;

/// L08 (sha256 99ce2cc7): GHC 9.14.1: rejected, GHC-04049.
const L08_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy field oracle L08 (#31236): a `~` on a newtype field without StrictData
-- Unmeasured. Predicted GHC 9.14.1: rejected (GHC-81601 or the newtype annotation error).
module Main where

newtype WrapChirho = WrapChirho ~Int

main :: IO ()
main = pure ()
"##;

/// L09 (sha256 0098cf74): predicted accepted, prints 7; GHC 9.14.1 measures it before the gate.
const L09_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy consumer oracle L09 (#31240): a record construction may omit an explicitly lazy field (only strict fields are required, GHC-95909)
-- Unmeasured. Predicted GHC 9.14.1: accepted (missing-fields warning only), prints 7.
{-# LANGUAGE StrictData #-}
module Main where

data BoxChirho = BoxChirho { lazyChirho :: ~Int, countChirho :: !Int }

main :: IO ()
main = print (countChirho (BoxChirho { countChirho = 7 }))
"##;

/// L10 (sha256 a9684b5e): predicted accepted, prints built; GHC 9.14.1 measures it before the gate.
const L10_CHIRHO: &str = r##"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- K01 explicit-lazy consumer oracle L10 (#31240): `C {}` on a positional constructor whose arguments are all explicitly lazy (tcfail112 needs a strict one)
-- Unmeasured. Predicted GHC 9.14.1: accepted, prints built.
{-# LANGUAGE StrictData #-}
module Main where

data PairChirho = PairChirho ~Int ~Int

main :: IO ()
main = case PairChirho {} of
  PairChirho _ _ -> putStrLn "built"
"##;

#[test]
fn explicit_lazy_fields_are_never_forced_chirho() {
    // L01-L03 hold `undefined` in a `~` field; L09 omits a `~` record field
    // (GHC-95909 requires strict fields only); L10 builds `C {}` whose arguments
    // are all `~` (tcfail112 requires a strict one).
    for (file_chirho, source_chirho, ghc_chirho) in [
        ("L01PositionalStrictDataChirho.hs", L01_CHIRHO, "3\n"),
        ("L02RecordStrictDataChirho.hs", L02_CHIRHO, "4\n"),
        ("L03PositionalStrictChirho.hs", L03_CHIRHO, "5\n"),
        ("L09OmittedLazyRecordFieldChirho.hs", L09_CHIRHO, "7\n"),
        (
            "L10EmptyBracesLazyPositionalChirho.hs",
            L10_CHIRHO,
            "built\n",
        ),
    ] {
        assert_prints_everywhere_chirho(file_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn lazy_annotations_without_strict_data_are_rejected_like_ghc_chirho() {
    for (file_chirho, source_chirho) in [
        ("L05PositionalDisabledChirho.hs", L05_CHIRHO),
        ("L06RecordDisabledChirho.hs", L06_CHIRHO),
        (
            "LazyFieldsDisabled.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/LazyFieldsDisabled.hs"
            ),
        ),
    ] {
        rejected_like_ghc_chirho(
            source_chirho,
            file_chirho,
            &["E0209", "Lazy field annotations (~) are disabled"],
        );
    }
}

#[test]
fn newtype_fields_take_no_strictness_annotation_chirho() {
    // `~` with and without StrictData (L04, L08) and `!` (readFail008).
    for (file_chirho, source_chirho) in [
        ("L04NewtypeStrictDataChirho.hs", L04_CHIRHO),
        ("L08NewtypeDisabledChirho.hs", L08_CHIRHO),
        (
            "readFail008.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/parser-chirho/should_fail/readFail008.hs"
            ),
        ),
    ] {
        rejected_like_ghc_chirho(
            source_chirho,
            file_chirho,
            &["E0209", "must not have a strictness annotation"],
        );
    }
}

#[test]
fn strict_data_corpus_lazy_fields_keep_checking_chirho() {
    // Before the CST read `~`, these `~` fields were dropped; now they reach naming
    // and the kind pass (an unlifted `Array# Int`, and `U :: UnliftedType`).
    for (file_chirho, source_chirho) in [
        (
            "T21951a.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T21951a.hs"
            ),
        ),
        (
            "T21951b.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_compile/T21951b.hs"
            ),
        ),
    ] {
        if let Err(error_chirho) = haskelujah_driver::typecheck_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            file_chirho,
        ) {
            panic!("{file_chirho}: GHC accepts it; we rejected it: {error_chirho}");
        }
    }
}
