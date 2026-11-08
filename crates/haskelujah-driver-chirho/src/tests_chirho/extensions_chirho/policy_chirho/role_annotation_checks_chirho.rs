// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Row 694 role annotations judged at declaration checking (#31844), against root's fresh
//! GHC 9.14.1 references (`role-annotation-r1-chirho`). Each source is byte for byte the
//! reference's, with the sha256 its plan pins. RA2 and RA3 use a method that never mentions
//! the parameter, so no GND refusal can hide the invalid declaration; RA1 and RA4 differ only
//! in the written role.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use super::gnd_coercion_chirho::{assert_refused_chirho, interpreted_chirho};

/// RA1: GHC 9.14.1 refuses it, GHC-29178. sha256 ac4f16490c2345aa87b6dc3aa8b8e431a48bc1bb1d9fb6595e2e058e15d321a3.
const WRITTEN_PHANTOM_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HoldChirho phantom
data HoldChirho aChirho = HoldChirho aChirho

class WrapChirho aChirho where
  wrapChirho :: aChirho -> HoldChirho aChirho

instance WrapChirho Int where
  wrapChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (WrapChirho)

main :: IO ()
main = case wrapChirho (AgeChirho 7) of
  HoldChirho (AgeChirho yearsChirho) -> print yearsChirho
"#;

#[test]
fn a_written_role_below_the_inferred_one_is_refused_chirho() {
    assert_refused_chirho(
        "WrittenPhantomHoldChirho.hs",
        WRITTEN_PHANTOM_HOLD_CHIRHO,
        &["E0209", "Role mismatch on variable aChirho"],
    );
}

/// RA2: GHC 9.14.1 refuses it, GHC-54298. sha256 8adcb78f7a2d0a14a48fe518231fc42488802fe5e199a96a5cc07c8a3dbfbbb3.
const ROLE_COUNT_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HoldChirho nominal nominal
data HoldChirho aChirho = HoldChirho aChirho

class TallyChirho aChirho where
  tallyChirho :: aChirho -> HoldChirho Int

instance TallyChirho Int where
  tallyChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (TallyChirho)

main :: IO ()
main = case tallyChirho (AgeChirho 7) of
  HoldChirho countChirho -> print countChirho
"#;

#[test]
fn a_wrong_number_of_roles_is_refused_chirho() {
    assert_refused_chirho(
        "RoleCountHoldChirho.hs",
        ROLE_COUNT_HOLD_CHIRHO,
        &["E0209", "Wrong number of roles"],
    );
}

/// RA3: GHC 9.14.1 refuses it, GHC-09009. sha256 13a1a76aedf8e2f792b3f04d683ba94dcef573f07b609db12b2e2684b0e2c914.
const UNKNOWN_ROLE_WORD_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HoldChirho representation
data HoldChirho aChirho = HoldChirho aChirho

class TallyChirho aChirho where
  tallyChirho :: aChirho -> HoldChirho Int

instance TallyChirho Int where
  tallyChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (TallyChirho)

main :: IO ()
main = case tallyChirho (AgeChirho 7) of
  HoldChirho countChirho -> print countChirho
"#;

#[test]
fn an_unknown_role_word_is_refused_chirho() {
    assert_refused_chirho(
        "UnknownRoleWordHoldChirho.hs",
        UNKNOWN_ROLE_WORD_HOLD_CHIRHO,
        &["E0209", "Illegal role name"],
    );
}

/// RA4: GHC 9.14.1 prints 8. sha256 1f138585a3b741c9955da2242e20d85b2ac6ce1015ce59c80f5b0907c9fa0352.
const WRITTEN_REPRESENTATIONAL_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HoldChirho representational
data HoldChirho aChirho = HoldChirho aChirho

class WrapChirho aChirho where
  wrapChirho :: aChirho -> HoldChirho aChirho

instance WrapChirho Int where
  wrapChirho valueChirho = HoldChirho (valueChirho + 1)

newtype AgeChirho = AgeChirho Int deriving newtype (WrapChirho)

main :: IO ()
main = case wrapChirho (AgeChirho 7) of
  HoldChirho (AgeChirho yearsChirho) -> print yearsChirho
"#;

#[test]
fn a_written_role_at_the_inferred_one_holds_chirho() {
    assert_eq!(
        interpreted_chirho(
            "WrittenRepresentationalHoldChirho.hs",
            WRITTEN_REPRESENTATIONAL_HOLD_CHIRHO
        ),
        "8\n"
    );
}
