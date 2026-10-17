// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Generic instance heads (`dict_chirho/instance_chirho/generic_heads_chirho.rs`).
//! A context-free instance whose head is a constructor applied to distinct
//! variables, with no competing head for that constructor, is keyed by the
//! constructor its use sites carry and filled from its rows as written
//! (claude2_chirho #27335; gpt_chirho #27342 and #27349, contract G1-G5).
//! Every expected output is GHC 9.14.1's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::run_chirho;

const SINGLETON_CHIRHO: &str = "{-# LANGUAGE DataKinds, GADTs, KindSignatures, StandaloneDeriving #-}\nmodule Main where\nimport Data.Kind (Type)\n";

const TAG_CLASS_CHIRHO: &str = "{-# LANGUAGE FlexibleInstances #-}\nmodule Main where\nclass TagChirho a where\n  tagChirho :: a -> String\nuseTagChirho :: TagChirho a => a -> String\nuseTagChirho x = tagChirho x\n";

/// GHC's exact output, or a failure: an ineligible head may stay unsupported,
/// but it must never answer with another instance's method.
fn assert_ghc_or_failure_chirho(file_name_chirho: &str, source_chirho: &str, ghc_chirho: &str) {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    if let Ok((_value_chirho, machine_chirho)) = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    ) {
        assert_eq!(
            machine_chirho.io_output_chirho, ghc_chirho,
            "{file_name_chirho}: neither GHC's output nor a failure"
        );
    }
}

#[test]
fn a_derived_singleton_eq_reaches_its_dictionary_and_its_row_chirho() {
    // claude2_chirho #27335: `deriving instance Eq (SBoolChirho b)`. The row
    // is written `$prim_Eq_==_SBoolChirho b`, the head is typed
    // `(SBoolChirho t0)`, and the use site carries `SBoolChirho`. Both halves
    // gate. Without the constructor key, the use site finds no dictionary and
    // runs the raw `==` ("missing STG binding"). Without the written row, the
    // slot is a stub ("missing method"). All five fail on 29be1b7f and on
    // 3ece4a33. On main 3ca4fc54 the two with a binder header fail too. The
    // phantom-indexed field makes the method's answer observable: GHC
    // (True,False).
    for (file_name_chirho, program_chirho, expected_chirho) in [
        (
            "ZeroBinderHeaderChirho.hs",
            "data SBoolChirho :: Bool -> Type where\n  STrueChirho :: SBoolChirho 'True\n  SFalseChirho :: SBoolChirho 'False\nderiving instance Eq (SBoolChirho b)\nmain :: IO ()\nmain = print (STrueChirho == STrueChirho)\n",
            "True\n",
        ),
        (
            "BinderHeaderChirho.hs",
            "data SBoolChirho (b :: Bool) where\n  STrueChirho :: SBoolChirho 'True\n  SFalseChirho :: SBoolChirho 'False\nderiving instance Eq (SBoolChirho b)\nmain :: IO ()\nmain = print (STrueChirho == STrueChirho)\n",
            "True\n",
        ),
        (
            "KindedWrittenHeadChirho.hs",
            "data SBoolChirho :: Bool -> Type where\n  STrueChirho :: SBoolChirho 'True\n  SFalseChirho :: SBoolChirho 'False\nderiving instance Eq (SBoolChirho (b :: Bool))\nmain :: IO ()\nmain = print (STrueChirho == STrueChirho)\n",
            "True\n",
        ),
        (
            "PolymorphicSiteChirho.hs",
            "data SBoolChirho :: Bool -> Type where\n  STrueChirho :: SBoolChirho 'True\n  SFalseChirho :: SBoolChirho 'False\nderiving instance Eq (SBoolChirho b)\nsameChirho :: SBoolChirho b -> SBoolChirho b -> Bool\nsameChirho = (==)\nmain :: IO ()\nmain = print (sameChirho STrueChirho STrueChirho)\n",
            "True\n",
        ),
        (
            "PhantomIndexedFieldChirho.hs",
            "data TaggedChirho (b :: Bool) = TaggedChirho Int\nderiving instance Eq (TaggedChirho b)\nmain :: IO ()\nmain = print (TaggedChirho 1 == (TaggedChirho 1 :: TaggedChirho 'True), TaggedChirho 1 == (TaggedChirho 2 :: TaggedChirho 'False))\n",
            "(True,False)\n",
        ),
    ] {
        assert_eq!(
            run_chirho(
                file_name_chirho,
                &format!("{SINGLETON_CHIRHO}{program_chirho}")
            ),
            expected_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn a_written_generic_head_serves_each_constructor_chirho() {
    // A hand-written context-free head over two distinct variables, by a
    // direct use and through a constrained reference, and two generic heads
    // that must each keep their own constructor. All fail on main 3ca4fc54,
    // 29be1b7f and 3ece4a33 with "missing STG binding `tagChirho`".
    for (file_name_chirho, program_chirho, expected_chirho) in [
        (
            "TwoVariableHeadChirho.hs",
            "data PairBoxChirho a b = PairBoxChirho\ninstance TagChirho (PairBoxChirho a b) where\n  tagChirho _ = \"generic\"\nmain :: IO ()\nmain = do\n  putStrLn (tagChirho (PairBoxChirho :: PairBoxChirho Int Bool))\n  putStrLn (useTagChirho (PairBoxChirho :: PairBoxChirho Char ()))\n",
            "generic\ngeneric\n",
        ),
        (
            "TwoConstructorsChirho.hs",
            "data OneChirho a = OneChirho\ndata TwoChirho a = TwoChirho\ninstance TagChirho (OneChirho a) where\n  tagChirho _ = \"one\"\ninstance TagChirho (TwoChirho a) where\n  tagChirho _ = \"two\"\nmain :: IO ()\nmain = do\n  putStrLn (tagChirho (OneChirho :: OneChirho Int))\n  putStrLn (useTagChirho (TwoChirho :: TwoChirho Bool))\n",
            "one\ntwo\n",
        ),
    ] {
        assert_eq!(
            run_chirho(
                file_name_chirho,
                &format!("{TAG_CLASS_CHIRHO}{program_chirho}")
            ),
            expected_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn an_ineligible_head_never_answers_for_another_chirho() {
    // A repeated variable, a competing specialized or contextual head, and a
    // head that needs a context are not generic: each may stay unsupported,
    // but none may answer with the wrong instance's method.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "RepeatedVariableChirho.hs",
            "data PairBoxChirho a b = PairBoxChirho\ninstance TagChirho (PairBoxChirho a a) where\n  tagChirho _ = \"repeated\"\nmain :: IO ()\nmain = putStrLn (tagChirho (PairBoxChirho :: PairBoxChirho Int Int))\n",
            "repeated\n",
        ),
        (
            "SpecializedCompetitorAtItsTypeChirho.hs",
            "data BoxChirho a = BoxChirho\ninstance {-# OVERLAPPING #-} TagChirho (BoxChirho Int) where\n  tagChirho _ = \"specialized\"\ninstance TagChirho (BoxChirho a) where\n  tagChirho _ = \"generic\"\nmain :: IO ()\nmain = putStrLn (tagChirho (BoxChirho :: BoxChirho Int))\n",
            "specialized\n",
        ),
        (
            "SpecializedCompetitorElsewhereChirho.hs",
            "data BoxChirho a = BoxChirho\ninstance {-# OVERLAPPING #-} TagChirho (BoxChirho Int) where\n  tagChirho _ = \"specialized\"\ninstance TagChirho (BoxChirho a) where\n  tagChirho _ = \"generic\"\nmain :: IO ()\nmain = putStrLn (tagChirho (BoxChirho :: BoxChirho Bool))\n",
            "generic\n",
        ),
        (
            "ContextualCompetitorChirho.hs",
            "data BoxChirho a = BoxChirho\ninstance {-# OVERLAPPING #-} Show a => TagChirho (BoxChirho [a]) where\n  tagChirho _ = \"contextual\"\ninstance TagChirho (BoxChirho a) where\n  tagChirho _ = \"generic\"\nmain :: IO ()\nmain = putStrLn (tagChirho (BoxChirho :: BoxChirho [Int]))\n",
            "contextual\n",
        ),
        (
            "ContextRequiringRowChirho.hs",
            "data CellChirho a = CellChirho a\ninstance Show a => TagChirho (CellChirho a) where\n  tagChirho (CellChirho x) = show x\nmain :: IO ()\nmain = putStrLn (tagChirho (CellChirho (2.5 :: Double)))\n",
            "2.5\n",
        ),
    ] {
        assert_ghc_or_failure_chirho(
            file_name_chirho,
            &format!("{TAG_CLASS_CHIRHO}{program_chirho}"),
            ghc_chirho,
        );
    }
}

#[test]
fn a_repeated_variable_head_never_serves_a_mismatched_use_chirho() {
    // GHC rejects this use (GHC-39999: no instance for
    // `TagChirho (PairBoxChirho Int Bool)`). Our checker accepts it, so it is
    // the dictionary pass that must not answer: a repeated variable is not a
    // bijection, so the head is never keyed by its constructor.
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let outcome_chirho = eval_source_with_machine_chirho(
        &format!(
            "{TAG_CLASS_CHIRHO}data PairBoxChirho a b = PairBoxChirho\ninstance TagChirho (PairBoxChirho a a) where\n  tagChirho _ = \"repeated\"\nmain :: IO ()\nmain = putStrLn (tagChirho (PairBoxChirho :: PairBoxChirho Int Bool))\n"
        ),
        &mut source_map_chirho,
        "RepeatedVariableMismatchChirho.hs",
        None,
    );
    if let Ok((_value_chirho, machine_chirho)) = outcome_chirho {
        panic!(
            "GHC rejects this program; we answered {:?}",
            machine_chirho.io_output_chirho
        );
    }
}
