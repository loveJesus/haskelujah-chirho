// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Which names get an occurrence id per reference (gpt_chirho #28091): the
//! standard set, and the methods of each single-parameter class the compiling
//! module declares, read off the lowered module the driver hands the desugarer.
//! `>>=` and `>>` are minted for their indexed own proof (row 697), and they and `fmap` for a
//! concrete head the module declares (row 699).
//! workflow: language-features-chirho/own-class-method-occurrences-chirho
//! workflow: monadic-dispatch-chirho

use std::collections::HashSet;

use haskelujah_ast_chirho::provenance_chirho::OccurrenceRoleChirho;

use super::occurrence_producers_chirho::lower_chirho;
use crate::evidence_join_chirho::{
    admits_operator_proof_chirho, joins_provenance_role_chirho, method_occurrence_names_chirho,
};

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
    // a seeded standard class's methods outside the standard set stay out, except
    // `fmap`, minted for its concrete-head proof alone (row 699, below).
    let names_chirho = method_occurrence_names_chirho(&lower_chirho(
        "module Main where\nimport NeedsChirho (NeedsChirho (..))\nmain :: IO ()\nmain = print (neededChirho [1 :: Int], fmap negate [1 :: Int], [1 :: Int] <> [2])\n",
    ));
    for name_chirho in ["neededChirho", "<>", "toList", "traverse", "pure"] {
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

#[test]
fn bind_and_then_are_minted_for_their_own_proof_and_the_other_inv_001_names_are_not_chirho() {
    // Row 697: each use of `>>=` and `>>` gets an occurrence id, so the checker's indexed own
    // proof for that use reaches the dictionary pass; the join keeps no other proof for them.
    // `return`, `pure` and `fail` keep their shared id (INV-001).
    let names_chirho = method_occurrence_names_chirho(&lower_chirho(
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int)\n",
    ));
    assert!(
        names_chirho.contains(">>=") && names_chirho.contains(">>"),
        "{names_chirho:?}"
    );
    for name_chirho in ["return", "pure", "fail"] {
        assert!(
            !names_chirho.contains(name_chirho),
            "{name_chirho} must keep its shared id"
        );
    }
}

#[test]
fn bind_and_then_admit_only_an_indexed_own_monad_proof_chirho() {
    // The exact predicate that types the use: its own dictionary parameter.
    for (name_chirho, key_chirho) in [(">>=", "$own:0"), (">>", "$own:12")] {
        assert!(
            admits_operator_proof_chirho(name_chirho, "Monad", key_chirho),
            "{name_chirho} at {key_chirho}"
        );
    }
    // A ground key, a user monad, a bare superclass `$own`, a malformed index or another class
    // proves nothing for them by its key alone, and no key alone proves `fmap`. A concrete head
    // the module declares is judged by the checker's applied proof instead (row 699,
    // `concrete_heads_chirho`); every other use keeps its canonical id and INV-001's path.
    for (name_chirho, class_chirho, key_chirho) in [
        (">>=", "Monad", "IO"),
        (">>", "Monad", "Maybe"),
        (">>=", "Monad", "ReaderChirho"),
        (">>=", "Monad", "$own"),
        (">>", "Monad", "$own:"),
        (">>=", "Monad", "$own:first"),
        (">>=", "MonadChirho", "$own:0"),
        ("fmap", "Functor", "Maybe"),
        ("fmap", "Functor", "ReaderChirho"),
        ("fmap", "Functor", "$own:0"),
    ] {
        assert!(
            !admits_operator_proof_chirho(name_chirho, class_chirho, key_chirho),
            "{name_chirho} {class_chirho} at {key_chirho}"
        );
    }
    // Every other name keeps every proof it had.
    for (name_chirho, class_chirho, key_chirho) in [
        ("show", "Show", "Int"),
        ("==", "Eq", "$own"),
        ("<", "Ord", "IO"),
    ] {
        assert!(admits_operator_proof_chirho(
            name_chirho,
            class_chirho,
            key_chirho
        ));
    }
}

#[test]
fn fmap_is_minted_for_its_concrete_head_proof_alone_chirho() {
    // Row 699, B2: each use of the seeded Functor's `fmap` gets an occurrence id, so the
    // checker's proof at a concrete head the module declares reaches the dictionary pass. Every
    // other proof is refused by the join, and the use keeps its canonical id. A module whose own
    // class declares `fmap` mints it among its own methods, as before.
    for source_chirho in [
        "module Main where\nmain :: IO ()\nmain = print (1 :: Int)\n",
        "module Main where\nimport Prelude hiding (fmap)\nclass MapsChirho f where\n  fmap :: (a -> b) -> f a -> f b\nmain :: IO ()\nmain = print (1 :: Int)\n",
    ] {
        let names_chirho = method_occurrence_names_chirho(&lower_chirho(source_chirho));
        assert!(names_chirho.contains("fmap"), "{names_chirho:?}");
    }
}

#[test]
fn a_do_selection_joins_by_its_own_role_and_a_reference_by_reference_chirho() {
    assert!(joins_provenance_role_chirho(
        ">>=",
        OccurrenceRoleChirho::Bind
    ));
    assert!(joins_provenance_role_chirho(
        ">>",
        OccurrenceRoleChirho::Then
    ));
    assert!(joins_provenance_role_chirho(
        ">>=",
        OccurrenceRoleChirho::Reference
    ));
    assert!(joins_provenance_role_chirho(
        "show",
        OccurrenceRoleChirho::Reference
    ));
    for (name_chirho, role_chirho) in [
        (">>=", OccurrenceRoleChirho::Then),
        (">>", OccurrenceRoleChirho::Bind),
        ("show", OccurrenceRoleChirho::Bind),
        ("fromInteger", OccurrenceRoleChirho::IntegerLiteral),
    ] {
        assert!(
            !joins_provenance_role_chirho(name_chirho, role_chirho),
            "{name_chirho} {role_chirho:?}"
        );
    }
}
