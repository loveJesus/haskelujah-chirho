// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 692, the fundep cohort's companion (gpt #33296, #33306): a restricted binding whose type
//! only a class functional dependency fixes. GHC improves the binding's wanted through the one
//! instance that matches the determining argument before it defaults anything, so the restriction
//! never meets an ambiguous variable.
//!
//! Both programs sit byte for byte beside this file and ran under GHC 9.14.1 (root GO #33303,
//! summary 58744708, fold 44a0b6ba): the positive compiled and printed exactly "43\nxx\n"; the
//! negative was rejected with GHC-39999, no instance for `DescribeChirho Double a0`. The negative
//! is a refusal control that also holds before the improvement exists.
//! workflow: language-features-chirho/fundep-generalization-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

/// Two restricted bindings, each fixed only by the class fundep, with real method output.
const FUNDEP_RESTRICTED_IMPROVEMENT_CHIRHO: &str =
    include_str!("fundep_restricted_improvement_chirho.hs");
/// No instance has a `Double` in the determining position, so nothing fixes the result.
const FUNDEP_RESTRICTED_NO_INSTANCE_CHIRHO: &str =
    include_str!("fundep_restricted_no_instance_chirho.hs");

/// The fundep fixes `answerChirho` at `Int` and `echoChirho` at `String`, so every engine prints
/// GHC's output.
#[test]
fn a_restricted_binding_takes_its_type_from_the_class_fundep_chirho() {
    assert_prints_everywhere_chirho(
        "fundep_restricted_improvement_chirho.hs",
        FUNDEP_RESTRICTED_IMPROVEMENT_CHIRHO,
        "43\nxx\n",
    );
}

/// Without a determining instance the restricted binding stays unsolved, and the module is
/// refused by the class at `Double`, as GHC refuses it.
#[test]
fn a_restricted_binding_without_a_determining_instance_is_refused_chirho() {
    rejected_like_ghc_chirho(
        FUNDEP_RESTRICTED_NO_INSTANCE_CHIRHO,
        "fundep_restricted_no_instance_chirho.hs",
        &["DescribeChirho Double"],
    );
}
