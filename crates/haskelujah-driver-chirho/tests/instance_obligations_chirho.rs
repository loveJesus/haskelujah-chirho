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
