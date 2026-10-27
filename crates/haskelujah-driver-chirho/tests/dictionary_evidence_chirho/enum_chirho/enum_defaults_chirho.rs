// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 669: Enum's defaults for the methods an instance does not write, as GHC's are, and a
//! derived instance's bounded `enumFrom` and `enumFromThen`. GHC 9.14.1 was run on every program
//! below, and each is checked byte for byte on the interpreter, LLVM and true Cranelift.
//!
//! Every default goes through the instance's own `toEnum` and `fromEnum` with Int's enumeration
//! in between, and calls no other method the instance may write. A derived instance writes the
//! bounded pair, built in Core from its key's rows and `>=#`, so a program-owned `enumFromTo`,
//! `enumFromThenTo`, `fromEnum` or `(>=)` never reaches them. Polymorphic enumeration calls
//! are row 671's, and a written enumeration call beside a program-owned namesake of its own name
//! is a named residual measured separately, so these programs reach the rows through spellings
//! the program does not own.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::BTreeSet;

use haskelujah_core_chirho::simplify_chirho::free_vars_chirho;
use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::assert_ghc_everywhere_chirho;

/// A newtype over Int whose `fromEnum` is ten times its value: each default visits every Int between its endpoints' `fromEnum`.
const FORMULA_CHIRHO: (&str, &str) = (
    r#"module Main where

newtype StepChirho = StepChirho Int deriving Show

instance Enum StepChirho where
  fromEnum (StepChirho valueChirho) = valueChirho * 10
  toEnum valueChirho = StepChirho valueChirho

main :: IO ()
main = do
  print (take 3 (enumFrom (StepChirho 1)))
  print (take 3 (enumFromThen (StepChirho 1) (StepChirho 2)))
  print (enumFromTo (StepChirho 1) (StepChirho 2))
  print (enumFromThenTo (StepChirho 1) (StepChirho 2) (StepChirho 4))
  print (succ (StepChirho 1))
  print (pred (StepChirho 1))
"#,
    r#"[StepChirho 10,StepChirho 11,StepChirho 12]
[StepChirho 10,StepChirho 20,StepChirho 30]
[StepChirho 10,StepChirho 11,StepChirho 12,StepChirho 13,StepChirho 14,StepChirho 15,StepChirho 16,StepChirho 17,StepChirho 18,StepChirho 19,StepChirho 20]
[StepChirho 10,StepChirho 20,StepChirho 30,StepChirho 40]
StepChirho 11
StepChirho 9
"#,
);

/// An instance writing `enumFromTo` and `succ`: no default calls either.
const INDEPENDENCE_CHIRHO: (&str, &str) = (
    r#"module Main where

newtype MixChirho = MixChirho Int deriving Show

instance Enum MixChirho where
  fromEnum (MixChirho valueChirho) = valueChirho
  toEnum valueChirho = MixChirho valueChirho
  enumFromTo _ _ = []
  succ _ = MixChirho 0

main :: IO ()
main = do
  print (enumFromThenTo (MixChirho 1) (MixChirho 3) (MixChirho 7))
  print (take 3 (enumFrom (MixChirho 5)))
  print (take 3 (enumFromThen (MixChirho 5) (MixChirho 7)))
  print (pred (MixChirho 5))
  print (enumFromTo (MixChirho 1) (MixChirho 3))
  print (succ (MixChirho 5))
"#,
    r#"[MixChirho 1,MixChirho 3,MixChirho 5,MixChirho 7]
[MixChirho 5,MixChirho 6,MixChirho 7]
[MixChirho 5,MixChirho 7,MixChirho 9]
MixChirho 4
[]
MixChirho 0
"#,
);

/// Constructors with only `toEnum` and `fromEnum` written.
const CONSTRUCTORS_CHIRHO: (&str, &str) = (
    r#"module Main where

data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving Show

instance Enum ColorChirho where
  fromEnum RedChirho = 0
  fromEnum GreenChirho = 1
  fromEnum BlueChirho = 2
  toEnum 0 = RedChirho
  toEnum 1 = GreenChirho
  toEnum _ = BlueChirho

main :: IO ()
main = do
  print (enumFromTo RedChirho BlueChirho)
  print (take 3 (enumFrom RedChirho))
  print (take 2 (enumFromThen RedChirho BlueChirho))
  print (enumFromThenTo RedChirho GreenChirho BlueChirho)
  print (succ RedChirho)
  print (pred BlueChirho)
"#,
    r#"[RedChirho,GreenChirho,BlueChirho]
[RedChirho,GreenChirho,BlueChirho]
[RedChirho,BlueChirho]
[RedChirho,GreenChirho,BlueChirho]
GreenChirho
GreenChirho
"#,
);

/// `succ` and `pred` through a polymorphic function, at the program's type and at Int.
const POLYMORPHIC_STEPS_CHIRHO: (&str, &str) = (
    r#"module Main where

data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving Show

instance Enum ColorChirho where
  fromEnum RedChirho = 0
  fromEnum GreenChirho = 1
  fromEnum BlueChirho = 2
  toEnum 0 = RedChirho
  toEnum 1 = GreenChirho
  toEnum _ = BlueChirho

nextChirho :: Enum a => a -> a
nextChirho = succ

previousChirho :: Enum a => a -> a
previousChirho = pred

main :: IO ()
main = do
  print (nextChirho RedChirho)
  print (previousChirho BlueChirho)
  print (nextChirho (41 :: Int))
"#,
    r#"GreenChirho
GreenChirho
42
"#,
);

/// The six written calls at Int, whose own rows hold.
const INT_HOLD_CHIRHO: (&str, &str) = (
    r#"module Main where

main :: IO ()
main = do
  print (enumFromThenTo 1 3 (7 :: Int))
  print (take 3 (enumFromThen 5 (7 :: Int)))
  print (enumFromTo 1 (3 :: Int))
  print (take 3 (enumFrom (5 :: Int)))
  print (succ (5 :: Int))
  print (pred (5 :: Int))
"#,
    r#"[1,3,5,7]
[5,7,9]
[1,2,3]
[5,6,7]
6
4
"#,
);

/// A derived instance: bounded `enumFrom` and `enumFromThen`, defaulted `enumFromTo` and `enumFromThenTo`.
const DERIVED_CHIRHO: (&str, &str) = (
    r#"module Main where

data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving (Show, Enum)

main :: IO ()
main = do
  print (succ RedChirho, pred BlueChirho)
  print (enumFromTo RedChirho BlueChirho)
  print (enumFromThenTo RedChirho GreenChirho BlueChirho)
  print (enumFrom GreenChirho)
  print (enumFromThen RedChirho BlueChirho)
"#,
    r#"(GreenChirho,GreenChirho)
[RedChirho,GreenChirho,BlueChirho]
[RedChirho,GreenChirho,BlueChirho]
[GreenChirho,BlueChirho]
[RedChirho,BlueChirho]
"#,
);

/// Derived bounds: from the first, a middle and the last constructor, ascending, descending and zero steps, endpoints, an empty range and a singleton type.
const DERIVED_BOUNDS_CHIRHO: (&str, &str) = (
    r#"module Main where

data StageChirho = AlphaChirho | BetaChirho | GammaChirho | DeltaChirho deriving (Show, Enum)

data OnlyChirho = OnlyChirho deriving (Show, Enum)

main :: IO ()
main = do
  print (enumFrom AlphaChirho)
  print (enumFrom GammaChirho)
  print (enumFrom DeltaChirho)
  print (enumFromThen AlphaChirho GammaChirho)
  print (enumFromThen DeltaChirho BetaChirho)
  print (enumFromThen BetaChirho AlphaChirho)
  print (take 3 (enumFromThen GammaChirho GammaChirho))
  print (enumFromTo BetaChirho DeltaChirho)
  print (enumFromTo DeltaChirho AlphaChirho)
  print (enumFromThenTo AlphaChirho BetaChirho DeltaChirho)
  print (enumFromThenTo DeltaChirho GammaChirho AlphaChirho)
  print (enumFrom OnlyChirho)
  print (take 2 (enumFromThen OnlyChirho OnlyChirho))
  print (succ AlphaChirho, pred DeltaChirho)
"#,
    r#"[AlphaChirho,BetaChirho,GammaChirho,DeltaChirho]
[GammaChirho,DeltaChirho]
[DeltaChirho]
[AlphaChirho,GammaChirho]
[DeltaChirho,BetaChirho]
[BetaChirho,AlphaChirho]
[GammaChirho,GammaChirho,GammaChirho]
[BetaChirho,GammaChirho,DeltaChirho]
[]
[AlphaChirho,BetaChirho,GammaChirho,DeltaChirho]
[DeltaChirho,GammaChirho,BetaChirho,AlphaChirho]
[OnlyChirho]
[OnlyChirho,OnlyChirho]
(BetaChirho,GammaChirho)
"#,
);

/// A derived instance in a program that owns `enumFromTo`, `enumFromThenTo`, `fromEnum` and `(>=)`.
const DERIVED_CAPTURE_CHIRHO: (&str, &str) = (
    r#"module Main where

import Prelude hiding (enumFromThenTo, enumFromTo, fromEnum, (>=))
import qualified Prelude as P

data StageChirho = AlphaChirho | BetaChirho | GammaChirho | DeltaChirho deriving (Show, Enum)

enumFromTo :: Int -> Int -> [Int]
enumFromTo _ _ = []

enumFromThenTo :: Int -> Int -> Int -> [Int]
enumFromThenTo _ _ _ = []

fromEnum :: StageChirho -> Int
fromEnum _ = 99

(>=) :: Int -> Int -> Bool
_ >= _ = False

main :: IO ()
main = do
  print (P.enumFrom BetaChirho)
  print (P.enumFromThen AlphaChirho GammaChirho)
  print (P.enumFromThen DeltaChirho BetaChirho)
  print (P.succ AlphaChirho, P.pred DeltaChirho)
  print (enumFromTo 1 3, enumFromThenTo 1 2 3, fromEnum AlphaChirho, (1 :: Int) >= 0)
"#,
    r#"[BetaChirho,GammaChirho,DeltaChirho]
[AlphaChirho,GammaChirho]
[DeltaChirho,BetaChirho]
(BetaChirho,GammaChirho)
([],[],99,False)
"#,
);

/// A written instance with only `toEnum` and `fromEnum`, in a program that owns the same four.
const WRITTEN_CAPTURE_CHIRHO: (&str, &str) = (
    r#"module Main where

import Prelude hiding (enumFromThenTo, enumFromTo, fromEnum, (>=))
import qualified Prelude as P

data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving Show

instance P.Enum ColorChirho where
  fromEnum RedChirho = 0
  fromEnum GreenChirho = 1
  fromEnum BlueChirho = 2
  toEnum 0 = RedChirho
  toEnum 1 = GreenChirho
  toEnum _ = BlueChirho

enumFromTo :: Int -> Int -> [Int]
enumFromTo _ _ = []

enumFromThenTo :: Int -> Int -> Int -> [Int]
enumFromThenTo _ _ _ = []

fromEnum :: ColorChirho -> Int
fromEnum _ = 99

(>=) :: Int -> Int -> Bool
_ >= _ = False

main :: IO ()
main = do
  print (take 3 (P.enumFrom RedChirho))
  print (take 2 (P.enumFromThen RedChirho BlueChirho))
  print (P.succ RedChirho, P.pred BlueChirho)
  print (enumFromTo 1 3, fromEnum RedChirho, (1 :: Int) >= 0)
"#,
    r#"[RedChirho,GreenChirho,BlueChirho]
[RedChirho,BlueChirho]
(GreenChirho,GreenChirho)
([],99,False)
"#,
);

#[test]
fn written_instances_take_ghcs_defaults_everywhere_chirho() {
    for (file_name_chirho, (source_chirho, ghc_chirho)) in [
        ("FORMULAChirho.hs", FORMULA_CHIRHO),
        ("CONSTRUCTORSChirho.hs", CONSTRUCTORS_CHIRHO),
    ] {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn no_default_calls_a_method_the_instance_writes_chirho() {
    let (source_chirho, ghc_chirho) = INDEPENDENCE_CHIRHO;
    assert_ghc_everywhere_chirho("INDEPENDENCEChirho.hs", source_chirho, ghc_chirho);
}

#[test]
fn polymorphic_succ_and_pred_take_the_defaults_chirho() {
    let (source_chirho, ghc_chirho) = POLYMORPHIC_STEPS_CHIRHO;
    assert_ghc_everywhere_chirho("POLYMORPHIC_STEPSChirho.hs", source_chirho, ghc_chirho);
}

#[test]
fn int_keeps_its_own_enumeration_rows_chirho() {
    let (source_chirho, ghc_chirho) = INT_HOLD_CHIRHO;
    assert_ghc_everywhere_chirho("INT_HOLDChirho.hs", source_chirho, ghc_chirho);
}

#[test]
fn derived_enumerations_are_bounded_as_ghcs_are_chirho() {
    for (file_name_chirho, (source_chirho, ghc_chirho)) in [
        ("DERIVEDChirho.hs", DERIVED_CHIRHO),
        ("DERIVED_BOUNDSChirho.hs", DERIVED_BOUNDS_CHIRHO),
    ] {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn program_owned_namesakes_never_reach_the_generated_rows_chirho() {
    for (file_name_chirho, (source_chirho, ghc_chirho)) in [
        ("DERIVED_CAPTUREChirho.hs", DERIVED_CAPTURE_CHIRHO),
        ("WRITTEN_CAPTUREChirho.hs", WRITTEN_CAPTURE_CHIRHO),
    ] {
        assert_ghc_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// The derived pair names only its key's own rows and the `>=#` primop, never a source
/// spelling, in a program that owns `enumFromTo`, `enumFromThenTo`, `fromEnum` and `(>=)`.
#[test]
fn derived_rows_reference_their_key_rows_and_a_primop_chirho() {
    let compiled_chirho = compile_source_chirho(
        DERIVED_CAPTURE_CHIRHO.0,
        &mut SourceMapChirho::new_chirho(),
        "DerivedCaptureChirho.hs",
    )
    .expect("the program compiles");
    let core_chirho = &compiled_chirho.core_chirho;
    let row_chirho = |name_chirho: &str| {
        core_chirho
            .bindings_chirho
            .iter()
            .find(|binding_chirho| binding_chirho.binder_chirho.name_chirho == name_chirho)
            .unwrap_or_else(|| panic!("{name_chirho} is generated"))
    };
    let referenced_chirho = |name_chirho: &str| -> BTreeSet<String> {
        free_vars_chirho(&row_chirho(name_chirho).rhs_chirho)
            .into_iter()
            .map(|id_chirho| core_chirho.names_chirho[&id_chirho].clone())
            .collect()
    };
    assert_eq!(
        referenced_chirho("$prim_Enum_enumFrom_StageChirho"),
        BTreeSet::from(["$prim_Enum_enumFromTo_StageChirho".to_string()])
    );
    assert_eq!(
        referenced_chirho("$prim_Enum_enumFromThen_StageChirho"),
        BTreeSet::from([
            "$prim_Enum_enumFromThenTo_StageChirho".to_string(),
            "$prim_Enum_fromEnum_StageChirho".to_string(),
        ])
    );
    let from_then_chirho = format!(
        "{:?}",
        row_chirho("$prim_Enum_enumFromThen_StageChirho").rhs_chirho
    );
    assert!(
        from_then_chirho.contains(r#"name_chirho: ">=#""#),
        "{from_then_chirho}"
    );
}
