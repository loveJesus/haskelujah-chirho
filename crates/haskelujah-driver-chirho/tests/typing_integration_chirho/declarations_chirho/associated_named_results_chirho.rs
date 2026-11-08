// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 686: an associated family's named result needs its injectivity annotation.
//!
//! Inside a class, GHC 9.14.1 accepts `type F a = (r :: K) | r -> a` but no form
//! `type F a = (r :: K)` without the `|`: having shifted into the named-result
//! binder, its parser stops with GHC-58481. Each negative below is a program GHC
//! measured, embedded byte for byte (sha256 prefix noted; root receipts in
//! `tmp-chirho/named-result-oracle-chirho`). The checker rejects them too, reading
//! the line as a default whose `g` is not in scope.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use super::rejected_like_ghc_chirho;

/// M01 (sha256 3e8a7583): the pipe-less named result. GHC 9.14.1: GHC-58481.
const PIPELESS_KINDED_RESULT_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- Row 686 control M01 (#31159): The parser test's exact pipe-less class form. GHC shifts into the named-result binder, which needs `|`.
-- Unmeasured. Predicted GHC 9.14.1: rejected (parse error expecting `|`; failing that, `g` not in scope).
{-# LANGUAGE TypeFamilies, MultiParamTypeClasses #-}
module Main where

import Data.Kind (Type)

class FrozenGen f m where
  type MutableGen f m = (g :: Type)
  thawGen :: f -> m (MutableGen f m)

main :: IO ()
main = pure ()
"#;

/// The parser fixture as first written (row 686's `FrozenGen`, before its `| g -> f`
/// was restored). GHC 9.14.1: GHC-58481. A malformed program, never a valid hold.
const ORIGINAL_FROZEN_GEN_FIXTURE_CHIRHO: &str = "module M where\nclass FrozenGen f m where\n  type MutableGen f m = (g :: Type)\n  thawGen :: f -> m (MutableGen f m)\n";

#[test]
fn a_named_result_without_its_injectivity_annotation_is_rejected_like_ghc_chirho() {
    rejected_like_ghc_chirho(
        PIPELESS_KINDED_RESULT_CHIRHO,
        "M01PipelessKindedResultChirho.hs",
        &["type variable not in scope: `g`"],
    );
}

#[test]
fn the_original_pipeless_frozen_gen_fixture_is_rejected_like_ghc_chirho() {
    rejected_like_ghc_chirho(
        ORIGINAL_FROZEN_GEN_FIXTURE_CHIRHO,
        "FrozenGenFixtureChirho.hs",
        &["type variable not in scope: `g`"],
    );
}
