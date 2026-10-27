// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which names get an occurrence id per reference (gpt_chirho #28091): the
//! standard set, and the methods of each single-parameter class the compiling
//! module declares, read off the lowered module the driver hands the desugarer.
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use std::collections::HashSet;

use super::occurrence_producers_chirho::lower_chirho;
use crate::evidence_join_chirho::method_occurrence_names_chirho;

/// What a module opts in beyond what a module that declares no class does.
fn added_names_chirho(source_chirho: &str) -> HashSet<String> {
    let standard_chirho = method_occurrence_names_chirho(&lower_chirho(
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int)\n",
    ));
    method_occurrence_names_chirho(&lower_chirho(source_chirho))
        .difference(&standard_chirho)
        .cloned()
        .collect()
}

#[test]
fn a_declared_single_parameter_class_adds_exactly_its_methods_chirho() {
    assert_eq!(
        added_names_chirho(
            "module Main where\nclass NeedsChirho a where\n  neededChirho :: a -> Int\n  spareChirho :: a -> a\nmain :: IO ()\nmain = print (1 :: Int)\n",
        ),
        HashSet::from(["neededChirho".to_string(), "spareChirho".to_string()])
    );
}

#[test]
fn a_multi_parameter_class_and_the_inv_001_names_are_never_added_chirho() {
    // A multi-parameter class keeps its shared id. A single-parameter class
    // that declares `pure` adds its other methods, never `pure` (INV-001).
    assert_eq!(
        added_names_chirho(
            "module Main where\nclass ConvertChirho a b where\n  convertChirho :: a -> b\nclass PointedChirho m where\n  pure :: a -> m a\n  pointChirho :: a -> m a\nmain :: IO ()\nmain = print (1 :: Int)\n",
        ),
        HashSet::from(["pointChirho".to_string()])
    );
}

#[test]
fn imported_and_seeded_class_methods_keep_their_shared_id_chirho() {
    // A class imported from another module is not this module's to opt in, and
    // a seeded standard class's methods outside the standard set stay out.
    let names_chirho = method_occurrence_names_chirho(&lower_chirho(
        "module Main where\nimport NeedsChirho (NeedsChirho (..))\nmain :: IO ()\nmain = print (neededChirho [1 :: Int], fmap negate [1 :: Int], [1 :: Int] <> [2])\n",
    ));
    for name_chirho in [
        "neededChirho",
        "fmap",
        "<>",
        "toList",
        "traverse",
        "pure",
        ">>=",
    ] {
        assert!(
            !names_chirho.contains(name_chirho),
            "{name_chirho} must keep its shared id"
        );
    }
    assert!(
        ["compare", "show", "succ", "pred"]
            .into_iter()
            .all(|name_chirho| names_chirho.contains(name_chirho)),
        "the standard set stays and includes the locally specialized Enum steps: {names_chirho:?}"
    );
}
