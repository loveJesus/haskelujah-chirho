// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Row 694 role-annotation surfaces (#31860, #31863) against root's fresh GHC 9.14.1
//! references (`role-surfaces-and-direct-gnd-r1-chirho`, #31865): the extension gate and its
//! editions, the targets a role annotation may name, a class parameter's role and a repeat.
//! Each source is byte for byte the measured one, with the sha256 its plan pins; each differs
//! from RA4 (`role_annotation_checks_chirho.rs`) only in its pragma or role lines.
//! workflow: compiler-pipeline-chirho/declaration-validity-chirho

use super::gnd_coercion_chirho::{assert_refused_chirho, interpreted_chirho};

/// RX1: GHC 9.14.1 refuses it, GHC-17779 (RA4 without RoleAnnotations). sha256 18aa28c3cdc380a4a783c64104592b464bd40516781024e4522e3cb8af95af85.
const ROLE_ANNOTATIONS_OFF_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
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
fn a_role_annotation_without_its_extension_is_refused_chirho() {
    assert_refused_chirho(
        "RoleAnnotationsOffHoldChirho.hs",
        ROLE_ANNOTATIONS_OFF_HOLD_CHIRHO,
        &["E0209", "Illegal role annotation for HoldChirho"],
    );
}

/// RX2: GHC 9.14.1 prints 8 (GHC2024 implies RoleAnnotations). sha256 4f5e72d04923ee0a51f4fc82ab67d0cb9be54f1c35ee975862cd988ad9ebe8ff.
const ROLE_ANNOTATIONS_EDITION_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2024 #-}
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
fn ghc2024_enables_role_annotations_chirho() {
    assert_eq!(
        interpreted_chirho(
            "RoleAnnotationsEditionHoldChirho.hs",
            ROLE_ANNOTATIONS_EDITION_HOLD_CHIRHO
        ),
        "8\n"
    );
}

/// RX3: GHC 9.14.1 refuses it, GHC-17779 (the later GHC2021 replaces GHC2024). sha256 411e957761769255d9f1473060651034d5cf25d404adb001b27aba3d49f26937.
const ROLE_ANNOTATIONS_LAST_EDITION_HOLD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2024 #-}
{-# LANGUAGE GHC2021, DerivingStrategies #-}
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
fn the_last_language_edition_decides_role_annotations_chirho() {
    assert_refused_chirho(
        "RoleAnnotationsLastEditionHoldChirho.hs",
        ROLE_ANNOTATIONS_LAST_EDITION_HOLD_CHIRHO,
        &["E0209", "Illegal role annotation for HoldChirho"],
    );
}

/// RX4: GHC 9.14.1 refuses it, GHC-77192 (a type synonym takes no roles). sha256 e162b9337ac6af54eb05e8c4c2b4cdf019f491de1198c711ad5353afc7a97801.
const SYNONYM_ROLE_TARGET_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HeldChirho representational
data HoldChirho aChirho = HoldChirho aChirho
type HeldChirho aChirho = HoldChirho aChirho

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
fn a_role_annotation_on_a_synonym_is_refused_chirho() {
    assert_refused_chirho(
        "SynonymRoleTargetChirho.hs",
        SYNONYM_ROLE_TARGET_CHIRHO,
        &[
            "E0209",
            "Illegal role annotation for HeldChirho; they are allowed only for datatypes and classes.",
        ],
    );
}

/// RX5: GHC 9.14.1 prints 8 (a nominal class parameter). sha256 74ef01398ef6aa73919000934a044b504d3d6243f499252f575bb64708a8a14c.
const CLASS_NOMINAL_ROLE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role WrapChirho nominal
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
fn a_nominal_class_role_holds_chirho() {
    assert_eq!(
        interpreted_chirho("ClassNominalRoleChirho.hs", CLASS_NOMINAL_ROLE_CHIRHO),
        "8\n"
    );
}

/// RX6: GHC 9.14.1 refuses it, GHC-18273 (a representational class parameter without IncoherentInstances). sha256 e8413bea7b765c0be5c586e14fd51eba9bdc0870165b63aefe6913b9bb488255.
const CLASS_REPRESENTATIONAL_ROLE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role WrapChirho representational
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
fn a_representational_class_role_is_refused_chirho() {
    assert_refused_chirho(
        "ClassRepresentationalRoleChirho.hs",
        CLASS_REPRESENTATIONAL_ROLE_CHIRHO,
        &[
            "E0209",
            "Roles other than ‘nominal’ for class parameters can lead to incoherence.",
        ],
    );
}

/// RX7: GHC 9.14.1 refuses it, GHC-44432 (a target declared nowhere). sha256 ea9a954f4a127512f69f17971e39591e0f06e8f938c9d8294d1f501d13b4e758.
const UNDECLARED_ROLE_TARGET_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role MissingChirho representational
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
fn a_role_annotation_for_an_undeclared_type_is_refused_chirho() {
    assert_refused_chirho(
        "UndeclaredRoleTargetChirho.hs",
        UNDECLARED_ROLE_TARGET_CHIRHO,
        &[
            "E0209",
            "The role annotation for ‘MissingChirho’ lacks an accompanying binding",
        ],
    );
}

/// RX8: GHC 9.14.1 refuses it, GHC-44432 (an imported target). sha256 f3ec99e698a3842a4d3416b7399bf04393322e2269d26f3dc1cff3d039930d1d.
const IMPORTED_ROLE_TARGET_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role Maybe representational
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
fn a_role_annotation_for_an_imported_type_is_refused_chirho() {
    assert_refused_chirho(
        "ImportedRoleTargetChirho.hs",
        IMPORTED_ROLE_TARGET_CHIRHO,
        &[
            "E0209",
            "The role annotation for ‘Maybe’ lacks an accompanying binding",
        ],
    );
}

/// RX9: GHC 9.14.1 refuses it, GHC-97170 (RA4's annotation written twice). sha256 05b9dee16d0670a40308bd28abc1ae550ea1968a7c96667c7b6a25b584fe472c.
const DUPLICATE_ROLE_ANNOTATION_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving, RoleAnnotations #-}
type role HoldChirho representational
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
fn a_repeated_role_annotation_is_refused_chirho() {
    assert_refused_chirho(
        "DuplicateRoleAnnotationChirho.hs",
        DUPLICATE_ROLE_ANNOTATION_CHIRHO,
        &["E0209", "Duplicate role annotations for ‘HoldChirho’"],
    );
}
