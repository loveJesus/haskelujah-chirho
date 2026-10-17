// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Declaration-time obligations of instance declarations: duplicate instances
//! (GHC-59692), and methods checked against their type at the instance, with
//! that type's variables rigid and the instance and method contexts as givens.
//! Every rejection names its reason; every control runs and prints what GHC
//! prints, or is a declaration GHC accepts. Each method program was measured on
//! GHC 9.14.1, and main (fd0da809) accepted every one GHC rejects.
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
const UNSATISFIABLE_CONTEXT_SOURCE_CHIRHO: &str = r#"{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances, FlexibleContexts, UndecidableInstances #-}
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
        .replace(
            "renderChirho [1 :: Int, 2]",
            "renderChirho (BoxChirho (1 :: Int))",
        );
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

/// The class every method program below implements.
const METHOD_CLASS_CHIRHO: &str =
    "module Main where\nclass CChirho a where\n  mChirho :: a -> String\n";

fn with_main_chirho(declarations_chirho: &str) -> String {
    format!("{METHOD_CLASS_CHIRHO}{declarations_chirho}main :: IO ()\nmain = putStrLn \"ok\"\n")
}

#[test]
fn a_method_needing_an_instance_that_does_not_exist_is_rejected_chirho() {
    // M01. The same `show x` in a top-level function was already rejected; in
    // a method the leftover wanted was discarded.
    rejected_with_chirho(
        "MethodNoInstanceChirho.hs",
        &with_main_chirho(
            "data TChirho = TChirho\ninstance CChirho TChirho where\n  mChirho x = show x\n",
        ),
        "no instance for `Show TChirho`",
    );
}

#[test]
fn a_method_needing_more_than_its_contexts_give_is_rejected_chirho() {
    // M02: `==` needs `Eq a`; the instance context gives only `Show a`.
    rejected_with_chirho(
        "MethodEqFromShowChirho.hs",
        "module Main where\nclass CChirho a where\n  mChirho :: a -> Bool\ninstance Show a => CChirho [a] where\n  mChirho xs = head xs == head xs\nmain :: IO ()\nmain = putStrLn \"ok\"\n",
        "could not deduce `Eq",
    );
    // M04: `+` needs `Num a`, and there is no context at all.
    rejected_with_chirho(
        "MethodNumNoContextChirho.hs",
        "module Main where\nclass CChirho a where\n  mChirho :: a -> a\ninstance CChirho [a] where\n  mChirho xs = map (+ 1) xs\nmain :: IO ()\nmain = putStrLn \"ok\"\n",
        "could not deduce `Num",
    );
    // M05: `show x` needs `Show a`; the context gives `CChirho a`.
    rejected_with_chirho(
        "MethodShowFromClassChirho.hs",
        &with_main_chirho(
            "instance CChirho a => CChirho (Maybe a) where\n  mChirho (Just x) = mChirho x ++ show x\n  mChirho Nothing = \"\"\n",
        ),
        "could not deduce `Show",
    );
    // V08: `show y` needs `Show b`; the method's own `b` has no context.
    rejected_with_chirho(
        "MethodQuantifierNoShowChirho.hs",
        "module Main where\nclass CChirho a where\n  mChirho :: a -> b -> String\ninstance CChirho Int where\n  mChirho n y = show n ++ show y\nmain :: IO ()\nmain = putStrLn \"ok\"\n",
        "could not deduce `Show",
    );
}

#[test]
fn a_method_may_not_choose_its_rigid_variables_chirho() {
    // V01: the instance's `a` is fixed to `Int`.
    rejected_with_chirho(
        "MethodFixesInstanceVarChirho.hs",
        "module Main where\nclass CChirho a where\n  mChirho :: a -> Int\ninstance CChirho [a] where\n  mChirho xs = sum (xs :: [Int])\nmain :: IO ()\nmain = putStrLn \"ok\"\n",
        "type mismatch",
    );
    // V02: the method's own `b` is fixed to `Bool`.
    rejected_with_chirho(
        "MethodFixesQuantifierChirho.hs",
        "module Main where\nclass CChirho a where\n  mChirho :: a -> b -> b\ninstance CChirho Int where\n  mChirho _ y = not (y :: Bool)\nmain :: IO ()\nmain = putStrLn \"ok\"\n",
        "type mismatch",
    );
}

#[test]
fn a_method_may_use_what_its_contexts_and_instances_give_chirho() {
    // V03: a local literal still defaults.
    assert_eq!(
        run_chirho(
            "MethodLiteralDefaultChirho.hs",
            "module Main where\nclass CChirho a where\n  mChirho :: a -> Int\ninstance CChirho Int where\n  mChirho _ = length (show 1)\nmain :: IO ()\nmain = print (mChirho (5 :: Int))\n",
        ),
        "1\n"
    );
    // V04: `Ord a` given, and the body needs its superclass `Eq a`.
    assert_eq!(
        run_chirho(
            "MethodSuperclassOfGivenChirho.hs",
            "module Main where\nclass CChirho a where\n  mChirho :: a -> Bool\ninstance Ord a => CChirho [a] where\n  mChirho (x:y:_) = x == y\n  mChirho _ = False\nmain :: IO ()\nmain = print (mChirho [1, 1 :: Int], mChirho [1, 2 :: Int])\n",
        ),
        "(True,False)\n"
    );
    // V07: the method's own context `Show b`.
    assert_eq!(
        run_chirho(
            "MethodOwnContextChirho.hs",
            "module Main where\nclass CChirho a where\n  mChirho :: Show b => a -> b -> String\ninstance CChirho Int where\n  mChirho n y = show n ++ show y\nmain :: IO ()\nmain = putStrLn (mChirho (3 :: Int) True)\n",
        ),
        "3True\n"
    );
    // V05 recursion through the context and V06 recursion at the instance's
    // own type: accepted, as GHC accepts them. They do not RUN yet, on main
    // either ("missing STG binding `mChirho`"), a separate runtime defect.
    for (file_name_chirho, declarations_chirho) in [
        (
            "MethodRecursionThroughContextChirho.hs",
            "instance CChirho Int where\n  mChirho = show\ninstance CChirho a => CChirho (Maybe a) where\n  mChirho (Just x) = mChirho x\n  mChirho Nothing = \"\"\n",
        ),
        (
            "MethodRecursionAtOwnTypeChirho.hs",
            "instance CChirho [a] where\n  mChirho (_:xs) = 'x' : mChirho xs\n  mChirho [] = \"\"\n",
        ),
    ] {
        check_chirho(file_name_chirho, &with_main_chirho(declarations_chirho)).unwrap_or_else(
            |err_chirho| panic!("{file_name_chirho}: GHC accepts this: {err_chirho}"),
        );
    }
}

#[test]
fn instances_the_checker_cannot_fully_judge_stay_accepted_chirho() {
    // agy_chirho's accept traps (#27156), each accepted by GHC 9.14.1 and by
    // main. A03/A04: built-in classes whose seeded method schemes lack their
    // contexts (`Monoid m`, `Applicative f`); their quantifiers stay flexible.
    // A07: an equality given rewrites the rigid variable during subsumption.
    // A09: a superclass reached through a multi-parameter given, which the
    // checker cannot reach yet, so the leftover verdict is withheld.
    for (file_name_chirho, source_chirho) in [
        (
            "A03ValidFoldableFoldmapChirho.hs",
            "module A03ValidFoldableFoldmapChirho where\ndata PairBox a = PairBox a a\ninstance Foldable PairBox where\n  foldMap f (PairBox x y) = mappend (f x) (mappend (f y) mempty)\n  foldr f z (PairBox x y) = f x (f y z)\n",
        ),
        (
            "A04ValidTraversableTraverseChirho.hs",
            "module A04ValidTraversableTraverseChirho where\ndata Box a = Box a\ninstance Functor Box where\n  fmap f (Box x) = Box (f x)\ninstance Foldable Box where\n  foldMap f (Box x) = f x\n  foldr f z (Box x) = f x z\ninstance Traversable Box where\n  traverse f (Box x) = fmap Box (f x)\n",
        ),
        (
            "A07ValidGivenEqualityInExpectedTyChirho.hs",
            "{-# LANGUAGE TypeFamilies, FlexibleInstances, FlexibleContexts #-}\nmodule A07ValidGivenEqualityInExpectedTyChirho where\ndata Box a = Box a\ninstance (a ~ Int) => Show (Box a) where\n  showsPrec _ (Box x) s = show (x :: Int) ++ s\n",
        ),
        (
            "A09ValidMptcSuperclassGivenChirho.hs",
            "{-# LANGUAGE MultiParamTypeClasses, FlexibleInstances #-}\nmodule A09ValidMptcSuperclassGivenChirho where\nclass Show a => Convert a b where\n  convert :: a -> b\ndata Wrap a b = Wrap a b\ninstance Convert a b => Show (Wrap a b) where\n  show (Wrap x _) = show x\n",
        ),
    ] {
        check_chirho(file_name_chirho, source_chirho).unwrap_or_else(|err_chirho| {
            panic!("{file_name_chirho}: GHC accepts this: {err_chirho}")
        });
    }
}

#[test]
fn a_method_judged_after_subsumption_and_its_givens_improvement_chirho() {
    // agy_chirho A05: `showsPrec`'s third argument goes past `ShowS`, so `x`'s
    // type meets the rigid `a` only in subsumption; the wanted `Show a` must
    // see that (GHC-39999).
    rejected_with_chirho(
        "A05ShowsPrecArity3WantedChirho.hs",
        "module A05ShowsPrecArity3WantedChirho where\ndata Box a = Box a\ninstance Show (Box a) where\n  showsPrec _ (Box x) s = show x ++ s\n",
        "Show",
    );
    // agy_chirho A06: the given `Extract c NoShow` improves the wanted's `e`
    // to `NoShow` through the dependency `c -> e`, and `Show NoShow` has no
    // instance (GHC-39999).
    rejected_with_chirho(
        "A06FundepGivenImprovementWantedChirho.hs",
        "{-# LANGUAGE MultiParamTypeClasses, FunctionalDependencies, FlexibleInstances, FlexibleContexts, UndecidableInstances #-}\nmodule A06FundepGivenImprovementWantedChirho where\nclass Extract c e | c -> e where\n  extract :: c -> e\ndata NoShow = NoShow\nclass Render a where\n  render :: a -> String\ninstance Extract c NoShow => Render [c] where\n  render (x:_) = show (extract x)\n  render []    = \"\"\n",
        "Show NoShow",
    );
}

/// A private instance context beside an unrelated binding: the binding's own
/// wanted must never be solved by it.
const PRIVATE_CONTEXT_CHIRHO: &str = "{-# LANGUAGE FlexibleContexts, UndecidableInstances #-}\nmodule Main where\ndata TChirho = TChirho\nclass CChirho aChirho where\n  mChirho :: aChirho -> String\ninstance Show TChirho => CChirho TChirho where\n";

#[test]
fn a_methods_private_context_solves_nothing_else_in_the_module_chirho() {
    // gpt_chirho's isolation canaries (#27163/#27164), rejected by GHC 9.14.1
    // and by main: the instance context `Show TChirho` is evidence for its own
    // methods only, not for a top-level binding beside it.
    rejected_with_chirho(
        "IsolationValueChirho.hs",
        &format!(
            "{PRIVATE_CONTEXT_CHIRHO}  mChirho _ = \"ok\"\nbadChirho = show TChirho\nmain :: IO ()\nmain = putStrLn badChirho\n"
        ),
        "no instance for `Show TChirho`",
    );
    rejected_with_chirho(
        "IsolationFunctionChirho.hs",
        &format!(
            "{PRIVATE_CONTEXT_CHIRHO}  mChirho _ = \"ok\"\nbadChirho :: TChirho -> String\nbadChirho valueChirho = show valueChirho\nmain :: IO ()\nmain = putStrLn \"ok\"\n"
        ),
        "no instance for `Show TChirho`",
    );
    // The reciprocal positive: inside its own method, the context is
    // evidence, and GHC accepts the instance.
    check_chirho(
        "IsolationOwnMethodChirho.hs",
        &format!(
            "{PRIVATE_CONTEXT_CHIRHO}  mChirho x = show x\nmain :: IO ()\nmain = putStrLn \"ok\"\n"
        ),
    )
    .unwrap_or_else(|err_chirho| {
        panic!("GHC accepts a method using its own context: {err_chirho}")
    });
}
