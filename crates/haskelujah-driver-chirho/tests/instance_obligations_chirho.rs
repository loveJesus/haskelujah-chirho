// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration-time obligations of instance declarations: duplicate instances
//! (GHC-59692). Every rejection names its reason; every control runs and
//! prints what GHC prints, or is a declaration GHC accepts.
//! workflow: language-features-chirho/instance-obligations-chirho

use haskelujah_driver::{compile_source_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;

fn check_chirho(file_name_chirho: &str, source_chirho: &str) -> Result<(), String> {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho)
        .map(|_| ())
        .map_err(|err_chirho| format!("{err_chirho}"))
}

fn run_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    )
    .unwrap_or_else(|err_chirho| panic!("{file_name_chirho} must run: {err_chirho}"));
    machine_chirho.io_output_chirho
}

fn rejected_with_chirho(file_name_chirho: &str, source_chirho: &str, reason_chirho: &str) {
    match check_chirho(file_name_chirho, source_chirho) {
        Err(err_chirho) => assert!(
            err_chirho.contains(reason_chirho),
            "{file_name_chirho}: rejected, but not for `{reason_chirho}`: {err_chirho}"
        ),
        Ok(()) => panic!("{file_name_chirho}: GHC rejects this ({reason_chirho}); we accepted it"),
    }
}

const CLASS_C_CHIRHO: &str = "module Main where\nclass C a where\n  op :: a -> Bool\n";

#[test]
fn two_instances_with_the_same_head_are_rejected_chirho() {
    rejected_with_chirho(
        "DuplicateInstanceChirho.hs",
        &format!(
            "{CLASS_C_CHIRHO}data B = MkB\ninstance C B where\n  op MkB = True\ninstance C B where\n  op MkB = False\nmain :: IO ()\nmain = print (op MkB)\n"
        ),
        "duplicate instance declarations",
    );
}

#[test]
fn heads_equal_up_to_renaming_are_duplicates_whatever_their_contexts_chirho() {
    rejected_with_chirho(
        "DuplicateRenamedChirho.hs",
        &format!(
            "{CLASS_C_CHIRHO}instance Eq a => C [a] where\n  op _ = True\ninstance Show b => C [b] where\n  op _ = False\nmain :: IO ()\nmain = print (op [True])\n"
        ),
        "duplicate instance declarations",
    );
}

#[test]
fn distinct_heads_are_not_duplicates_and_dispatch_separately_chirho() {
    assert_eq!(
        run_chirho(
            "DistinctInstancesChirho.hs",
            &format!(
                "{CLASS_C_CHIRHO}instance C Int where\n  op n = n > 0\ninstance C Bool where\n  op b = not b\nmain :: IO ()\nmain = do\n  print (op (3 :: Int))\n  print (op True)\n"
            ),
        ),
        "True\nFalse\n"
    );
}

#[test]
fn a_repeated_variable_head_is_not_a_duplicate_of_a_two_variable_head_chirho() {
    assert!(
        check_chirho(
            "RepeatedVariableHeadChirho.hs",
            "{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances #-}\nmodule Main where\nclass D a b where\n  pick :: a -> b -> Bool\ninstance D [a] [a] where\n  pick _ _ = True\ninstance D (Maybe a) [b] where\n  pick _ _ = False\nmain :: IO ()\nmain = putStrLn \"declared\"\n",
        )
        .is_ok(),
        "`D [a] [a]` and `D (Maybe a) [b]` are different heads"
    );
}

#[test]
fn a_local_instance_equal_to_a_prelude_instance_is_a_duplicate_chirho() {
    rejected_with_chirho(
        "DuplicateOfPreludeChirho.hs",
        "module Main where\ninstance Eq a => Eq (a, b) where\n  (m, _) == (o, _) = m == o\nmain :: IO ()\nmain = putStrLn \"never\"\n",
        "duplicate instance declarations",
    );
}

#[test]
fn a_local_type_sharing_a_library_name_is_not_a_duplicate_chirho() {
    assert!(
        check_chirho(
            "ShadowingTypeChirho.hs",
            "module Main where\nnewtype MaybeT m a = MaybeT (m (Maybe a))\ninstance Functor m => Functor (MaybeT m) where\n  fmap f (MaybeT x) = MaybeT (fmap (fmap f) x)\nmain :: IO ()\nmain = putStrLn \"declared\"\n",
        )
        .is_ok(),
        "this module's own `MaybeT` is not the library's; its Functor instance is no duplicate"
    );
}

/// Type-check `source_chirho` the way the multi-module frontend does once an
/// import has made a bare type name "safe": every qualified spelling of that
/// name collapses to the bare name inside the checker. Returns the rendered
/// diagnostics.
fn diagnostics_with_collapsed_names_chirho(
    source_chirho: &str,
    collapsed_chirho: &[&str],
) -> String {
    use haskelujah_parser_chirho::cst_parser_chirho::parse_to_cst_chirho;
    use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
    use haskelujah_span_chirho::FileIdChirho;
    use haskelujah_typing_chirho::infer_chirho::infer_module_with_imports_type_synonyms_families_and_class_env_chirho;
    use std::collections::{HashMap, HashSet};

    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let module_chirho = lower_module_chirho(
        &parse_to_cst_chirho(source_chirho, file_chirho),
        file_chirho,
    );
    let mut class_env_chirho = haskelujah_typing_chirho::ClassEnvChirho::new_chirho();
    class_env_chirho.seed_standard_chirho();
    let result_chirho = infer_module_with_imports_type_synonyms_families_and_class_env_chirho(
        &module_chirho,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &class_env_chirho,
        &HashMap::new(),
        &collapsed_chirho
            .iter()
            .map(|name_chirho| name_chirho.to_string())
            .collect::<HashSet<String>>(),
        &HashMap::new(),
    );
    format!("{}", result_chirho.diagnostics_chirho)
}

const TWO_STATE_TRANSFORMERS_CHIRHO: &str = "module Main where\nimport qualified Control.Monad.Trans.State.Strict as Strict\nimport qualified Control.Monad.Trans.State.Lazy as Lazy\nclass Named f where\n  nameOf :: f a -> String\n";

#[test]
fn one_bare_name_under_two_qualifiers_is_two_types_chirho() {
    // GHC 9.14.1 accepts this program. The constraints package declares
    // `Lifting Functor` for both `Strict.StateT` and `Lazy.StateT`; once the
    // frontend collapses both spellings to `StateT` the heads look equal.
    let diagnostics_chirho = diagnostics_with_collapsed_names_chirho(
        &format!(
            "{TWO_STATE_TRANSFORMERS_CHIRHO}instance Named (Strict.StateT s m) where\n  nameOf _ = \"strict\"\ninstance Named (Lazy.StateT s m) where\n  nameOf _ = \"lazy\"\n"
        ),
        &["StateT"],
    );
    assert!(
        !diagnostics_chirho.contains("duplicate instance declarations"),
        "{diagnostics_chirho}"
    );
}

#[test]
fn the_same_spelling_twice_is_still_a_duplicate_chirho() {
    // GHC 9.14.1: GHC-59692 for these two heads.
    let diagnostics_chirho = diagnostics_with_collapsed_names_chirho(
        &format!(
            "{TWO_STATE_TRANSFORMERS_CHIRHO}instance Named (Strict.StateT s m) where\n  nameOf _ = \"strict\"\ninstance Named (Strict.StateT t n) where\n  nameOf _ = \"again\"\n"
        ),
        &["StateT"],
    );
    assert!(
        diagnostics_chirho.contains("duplicate instance declarations"),
        "{diagnostics_chirho}"
    );
}

// An instance whose own context cannot be satisfied does not satisfy anything.
// GHC 9.14.1 on this source: "No instance for `ConvertChirho Int String' arising
// from a use of `renderChirho'" (GHC-39999). Measured 2026-09-19 alongside its
// three controls below.
const UNSATISFIABLE_CONTEXT_SOURCE_CHIRHO: &str =
    r#"{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances, FlexibleContexts, UndecidableInstances #-}
module Main where
class ConvertChirho a b where
  convertChirho :: a -> b
instance ConvertChirho Int Bool where
  convertChirho n = n > 0
class RenderChirho a where
  renderChirho :: a -> String
instance ConvertChirho a String => RenderChirho [a] where
  renderChirho xs = concatMap (\x -> convertChirho x ++ ";") xs
main :: IO ()
main = putStrLn (renderChirho [1 :: Int, 2])
"#;

#[test]
fn an_instance_context_that_cannot_hold_does_not_satisfy_its_head_chirho() {
    rejected_with_chirho(
        "UnsatisfiableContextChirho.hs",
        UNSATISFIABLE_CONTEXT_SOURCE_CHIRHO,
        "no instance for `ConvertChirho Int [Char]`",
    );
}

#[test]
fn the_same_shape_at_a_data_head_is_rejected_the_same_way_chirho() {
    let source_chirho = UNSATISFIABLE_CONTEXT_SOURCE_CHIRHO
        .replace("RenderChirho [a]", "RenderChirho (BoxChirho a)")
        .replace(
            "renderChirho xs = concatMap (\\x -> convertChirho x ++ \";\") xs",
            "renderChirho (BoxChirho x) = convertChirho x",
        )
        .replace(
            "class RenderChirho a where",
            "data BoxChirho a = BoxChirho a\nclass RenderChirho a where",
        )
        .replace("renderChirho [1 :: Int, 2]", "renderChirho (BoxChirho (1 :: Int))");
    rejected_with_chirho(
        "UnsatisfiableContextBoxChirho.hs",
        &source_chirho,
        "no instance for `ConvertChirho Int [Char]`",
    );
}

#[test]
fn the_same_program_with_the_instance_present_is_accepted_chirho() {
    let source_chirho = UNSATISFIABLE_CONTEXT_SOURCE_CHIRHO.replace(
        "instance ConvertChirho Int Bool where\n  convertChirho n = n > 0",
        "instance ConvertChirho Int String where\n  convertChirho n = show n",
    );
    check_chirho("SatisfiableContextChirho.hs", &source_chirho)
        .expect("GHC accepts this program, and so must we");
}

#[test]
fn a_sub_goal_over_a_variable_gives_no_verdict_chirho() {
    // The same instance used under a GIVEN that discharges its context: the
    // sub-goal is `ConvertChirho a String` over a rigid `a`, never ground, so the
    // rule must stay silent rather than invent a rejection. GHC 9.14.1 accepts
    // this program and prints "ok".
    let source_chirho = UNSATISFIABLE_CONTEXT_SOURCE_CHIRHO.replace(
        "main :: IO ()\nmain = putStrLn (renderChirho [1 :: Int, 2])",
        "allOfChirho :: ConvertChirho a String => [a] -> String\nallOfChirho xs = renderChirho xs\nmain :: IO ()\nmain = putStrLn \"ok\"",
    );
    check_chirho("VariableSubGoalChirho.hs", &source_chirho)
        .expect("a sub-goal over a rigid variable is unproved, not unsolvable");
}

#[test]
fn instantiating_that_given_at_a_missing_instance_is_rejected_chirho() {
    // The same program, used at `[Int]`: GHC rejects it with "No instance for
    // `ConvertChirho Int String' arising from a use of `allOfChirho'", so the
    // silence above is about the variable, not about the shape.
    let source_chirho = UNSATISFIABLE_CONTEXT_SOURCE_CHIRHO.replace(
        "main :: IO ()\nmain = putStrLn (renderChirho [1 :: Int, 2])",
        "allOfChirho :: ConvertChirho a String => [a] -> String\nallOfChirho xs = renderChirho xs\nmain :: IO ()\nmain = putStrLn (allOfChirho ([] :: [Int]))",
    );
    rejected_with_chirho(
        "VariableSubGoalUsedChirho.hs",
        &source_chirho,
        "no instance for `ConvertChirho Int [Char]`",
    );
}
