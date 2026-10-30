// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equality at the type level has GHC's precedence four, below numeric addition.
//! Both the positive proof and false-equality controls are measured with GHC 9.14.1.

use super::{SourceMapChirho, assert_prints_everywhere_chirho, compile_source_chirho};

const TYPE_EQUALITY_CHIRHO: &str = r#"{-# LANGUAGE DataKinds, GADTs, TypeOperators #-}
module Main where
import Data.Type.Equality
import GHC.TypeNats
leftProofChirho :: (1 + 2 ~ 3) :~: ((1 + 2) ~ 3)
leftProofChirho = Refl
rightProofChirho :: (3 ~ 1 + 2) :~: (3 ~ (1 + 2))
rightProofChirho = Refl
main = case leftProofChirho of
  Refl -> case rightProofChirho of
    Refl -> putStrLn "equality"
"#;

const PARENTHESIZED_CHIRHO: &str = r#"{-# LANGUAGE DataKinds, GADTs, TypeOperators #-}
module Main where
import Data.Type.Equality
import GHC.TypeNats
proofChirho :: ((1 + 2) ~ 3) :~: ((1 + 2) ~ 3)
proofChirho = Refl
main = case proofChirho of Refl -> putStrLn "parenthesized"
"#;

const FALSE_EQUALITY_CHIRHO: &str = r#"{-# LANGUAGE DataKinds, TypeOperators #-}
module Main where
import Data.Type.Equality
import GHC.TypeNats
proofChirho :: (1 + 2 ~ 4) :~: ((1 + 2) ~ 3)
proofChirho = Refl
main = return ()
"#;

const CHAINED_EQUALITY_CHIRHO: &str = r#"{-# LANGUAGE TypeOperators #-}
module Main where
import Data.Type.Equality
proofChirho :: (aChirho ~ bChirho ~ cChirho) :~: ()
proofChirho = Refl
main = return ()
"#;

#[test]
fn type_equality_binds_below_addition_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "TypeEqualityFixityChirho.hs",
        TYPE_EQUALITY_CHIRHO,
        "equality\n",
    );
}

#[test]
fn explicit_type_equality_parentheses_keep_their_meaning_chirho() {
    assert_prints_everywhere_chirho(
        "ParenthesizedTypeEqualityChirho.hs",
        PARENTHESIZED_CHIRHO,
        "parenthesized\n",
    );
}

#[test]
fn false_and_unparenthesized_chained_equalities_stay_rejected_chirho() {
    for (name_chirho, source_chirho) in [
        ("FalseTypeEqualityChirho.hs", FALSE_EQUALITY_CHIRHO),
        ("ChainedTypeEqualityChirho.hs", CHAINED_EQUALITY_CHIRHO),
    ] {
        assert!(
            compile_source_chirho(
                source_chirho,
                &mut SourceMapChirho::new_chirho(),
                name_chirho
            )
            .is_err(),
            "GHC rejects {name_chirho}; we accepted it"
        );
    }
}
