// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared element-kind identities at signatures, visible applications and equality.
//! Workflow: language-features-chirho/declaration-kinds-chirho.
use super::{SourceMapChirho, assert_execution_chirho};
use haskelujah_driver::typecheck_source_chirho;

// Exact sources are independently recorded in producers-chirho/occurrence-reference-chirho.jsonl.
const PREFIX_CHIRHO: &str = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, ExplicitForAll, KindSignatures, PolyKinds, TypeApplications, TypeFamilies, TypeOperators, GADTs #-}
module Main where
import Data.Kind (Type, Constraint)
"###;

fn reference_source_chirho(name_chirho: &str) -> String {
    let body_chirho = match name_chirho {
        "LiteralOccurrenceChirho" => {
            r###"data PChirho (xsChirho :: [Type]) = PChirho Int
idPChirho :: forall xsChirho. PChirho xsChirho -> PChirho xsChirho
idPChirho valueChirho = valueChirho
readPChirho (PChirho valueChirho) = valueChirho
literalChirho :: PChirho '[Int] -> PChirho '[Int]
literalChirho = idPChirho @'[Int]
main = print (readPChirho (literalChirho (PChirho 42)))
"###
        }
        "ConsOccurrenceChirho" => {
            r###"data PChirho (xsChirho :: [Type]) = PChirho Int
idPChirho :: forall xsChirho. PChirho xsChirho -> PChirho xsChirho
idPChirho valueChirho = valueChirho
readPChirho (PChirho valueChirho) = valueChirho
literalChirho :: PChirho (Int ': '[]) -> PChirho (Int ': '[])
literalChirho = idPChirho @(Int ': '[])
main = print (readPChirho (literalChirho (PChirho 42)))
"###
        }
        "EmptyOccurrenceChirho" => {
            r###"data PChirho (xsChirho :: [Type]) = PChirho Int
idPChirho :: forall xsChirho. PChirho xsChirho -> PChirho xsChirho
idPChirho valueChirho = valueChirho
readPChirho (PChirho valueChirho) = valueChirho
literalChirho :: PChirho '[] -> PChirho '[]
literalChirho = idPChirho @'[]
main = print (readPChirho (literalChirho (PChirho 42)))
"###
        }
        "TwoOccurrenceChirho" => {
            r###"data PChirho (xsChirho :: [Type]) = PChirho Int
idPChirho :: forall xsChirho. PChirho xsChirho -> PChirho xsChirho
idPChirho valueChirho = valueChirho
readPChirho (PChirho valueChirho) = valueChirho
literalChirho :: PChirho '[Int, Bool] -> PChirho '[Int, Bool]
literalChirho = idPChirho @'[Int, Bool]
main = print (readPChirho (literalChirho (PChirho 42)))
"###
        }
        "WrongOccurrenceChirho" => {
            r###"data PChirho (xsChirho :: [Type]) = PChirho Int
idPChirho :: forall xsChirho. PChirho xsChirho -> PChirho xsChirho
idPChirho valueChirho = valueChirho
readPChirho (PChirho valueChirho) = valueChirho
literalChirho :: PChirho '[Bool] -> PChirho '[Bool]
literalChirho = idPChirho @'[Int]
main = pure ()
"###
        }
        "DirectNilChirho" => {
            r###"type family DirectNilChirho (xsChirho :: [kindChirho]) :: [kindChirho] where
  DirectNilChirho xsChirho = '[]
main = pure ()
"###
        }
        "ReflexiveChirho" => {
            r###"type family ReflexiveChirho (xsChirho :: [kindChirho]) :: Constraint where
  ReflexiveChirho xsChirho = (xsChirho ~ xsChirho)
main = pure ()
"###
        }
        "EqualityNilChirho" => {
            r###"type family EqualityNilChirho (xsChirho :: [kindChirho]) :: Constraint where
  EqualityNilChirho xsChirho = (xsChirho ~ '[])
main = pure ()
"###
        }
        "WrongEqualityKindChirho" => {
            r###"type family EqualityNilChirho (xsChirho :: [kindChirho]) :: Constraint where
  EqualityNilChirho xsChirho = (xsChirho ~ 'True)
main = pure ()
"###
        }
        _ => panic!("missing independent reference {name_chirho}"),
    };
    format!("{PREFIX_CHIRHO}{body_chirho}")
}

#[test]
fn promoted_list_literal_visible_application_executes_chirho() {
    assert_execution_chirho(&reference_source_chirho("LiteralOccurrenceChirho"), "42\n");
}

#[test]
fn promoted_cons_visible_application_executes_chirho() {
    assert_execution_chirho(&reference_source_chirho("ConsOccurrenceChirho"), "42\n");
}

#[test]
fn empty_promoted_list_visible_application_executes_chirho() {
    assert_execution_chirho(&reference_source_chirho("EmptyOccurrenceChirho"), "42\n");
}

#[test]
fn promoted_list_spine_shares_one_element_kind_chirho() {
    assert_execution_chirho(&reference_source_chirho("TwoOccurrenceChirho"), "42\n");
}

#[test]
fn promoted_list_visible_application_checks_element_identity_chirho() {
    let error_chirho = typecheck_source_chirho(
        &reference_source_chirho("WrongOccurrenceChirho"),
        &mut SourceMapChirho::new_chirho(),
        "WrongOccurrenceChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("Int and Bool list elements remain different types");
    let message_chirho = error_chirho.to_string();
    assert!(
        message_chirho.contains("Int") && message_chirho.contains("Bool"),
        "{message_chirho}"
    );
}

#[test]
fn equality_operands_constrain_the_same_promoted_nil_kind_chirho() {
    // Direct nil and reflexive equality discriminate the two independent paths;
    // the combined equality was the only valid family rejected by351bdbb3.
    for name_chirho in ["DirectNilChirho", "ReflexiveChirho", "EqualityNilChirho"] {
        typecheck_source_chirho(
            &reference_source_chirho(name_chirho),
            &mut SourceMapChirho::new_chirho(),
            &format!("{name_chirho}.hs"),
        )
        .unwrap_or_else(|error_chirho| panic!("{name_chirho}: {error_chirho}"));
    }
}

#[test]
fn homogeneous_equality_rejects_different_operand_kinds_chirho() {
    let error_chirho = typecheck_source_chirho(
        &reference_source_chirho("WrongEqualityKindChirho"),
        &mut SourceMapChirho::new_chirho(),
        "WrongEqualityKindChirho.hs",
    )
    .map(|_result_chirho| ())
    .expect_err("a list and a Bool cannot be operands of homogeneous equality");
    let message_chirho = error_chirho.to_string();
    assert!(message_chirho.contains("kind mismatch"), "{message_chirho}");
}
