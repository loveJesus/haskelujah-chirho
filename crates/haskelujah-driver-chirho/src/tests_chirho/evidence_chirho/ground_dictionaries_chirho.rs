// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ground dictionaries for conditional instances, on every engine: the STG machine, LLVM and
//! Cranelift each print GHC's answer. The B01, two-context, missing-base and cyclic programs
//! were measured on GHC 9.14.1 (root receipts `equality-and-dictionary-oracle-r1-chirho/<case>/`).
//! The repeated list-context control and the shadowing hold are predictions that root measures
//! before the landing.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use crate::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};

/// A module under `pragmas_chirho`, headed as every source we write is.
fn program_chirho(pragmas_chirho: &str, body_chirho: &str) -> String {
    format!(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{{-# LANGUAGE {pragmas_chirho} #-}}\nmodule Main where\n\nimport Data.Proxy (Proxy (..))\n\n{body_chirho}\n"
    )
}

/// What the STG machine printed, or why the program was rejected.
fn stg_run_chirho(source_chirho: &str) -> Result<String, String> {
    eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "GroundDictionariesChirho.hs",
        None,
    )
    .map(|(_, machine_chirho)| machine_chirho.io_output_chirho)
    .map_err(|diagnostics_chirho| format!("{diagnostics_chirho:?}"))
}

/// Every engine prints `expected_chirho` and exits cleanly.
fn every_engine_prints_chirho(source_chirho: &str, expected_chirho: &str) {
    let printed_chirho = stg_run_chirho(source_chirho)
        .unwrap_or_else(|failure_chirho| panic!("STG rejected it: {failure_chirho}"));
    assert_eq!(printed_chirho, expected_chirho, "STG");
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        let (exit_chirho, stdout_chirho) =
            native_round_trip_chirho(source_chirho, backend_chirho, "").unwrap_or_else(
                |failure_chirho| panic!("{backend_chirho:?} rejected it: {failure_chirho}"),
            );
        assert_eq!(
            (exit_chirho, stdout_chirho.as_str()),
            (0, expected_chirho),
            "{backend_chirho:?}"
        );
    }
}

const KNOWN_NAT_CHIRHO: &str = "data NatChirho = ZeroChirho | SuccChirho NatChirho

class KnownNatChirho (nChirho :: NatChirho) where
  natValChirho :: Proxy nChirho -> Int

instance KnownNatChirho ZeroChirho where
  natValChirho _ = 0

instance KnownNatChirho nChirho => KnownNatChirho (SuccChirho nChirho) where
  natValChirho _ = 1 + natValChirho (Proxy :: Proxy nChirho)
";

const KNOWN_NAT_PRAGMAS_CHIRHO: &str = "DataKinds, KindSignatures, ScopedTypeVariables";

const DESCRIBE_CHIRHO: &str = "class DescribeChirho aChirho where
  describeChirho :: aChirho -> String

instance DescribeChirho Int where
  describeChirho nChirho = \"int\" ++ show nChirho

instance DescribeChirho Bool where
  describeChirho bChirho = if bChirho then \"yes\" else \"no\"
";

#[test]
fn a_recursive_context_instance_at_a_promoted_chain_counts_three_chirho() {
    every_engine_prints_chirho(
        &program_chirho(
            KNOWN_NAT_PRAGMAS_CHIRHO,
            &format!(
                "{KNOWN_NAT_CHIRHO}
main :: IO ()
main = print (natValChirho (Proxy :: Proxy (SuccChirho (SuccChirho (SuccChirho ZeroChirho)))))"
            ),
        ),
        "3\n",
    );
}

#[test]
fn the_context_free_base_instance_alone_keeps_its_route_chirho() {
    every_engine_prints_chirho(
        &program_chirho(
            KNOWN_NAT_PRAGMAS_CHIRHO,
            &format!(
                "{KNOWN_NAT_CHIRHO}
main :: IO ()
main = print (natValChirho (Proxy :: Proxy ZeroChirho))"
            ),
        ),
        "0\n",
    );
}

#[test]
fn a_recursive_context_instance_over_an_ordinary_type_counts_three_chirho() {
    every_engine_prints_chirho(
        &program_chirho(
            "ScopedTypeVariables",
            "class SizeChirho aChirho where
  sizeChirho :: Proxy aChirho -> Int

instance SizeChirho () where
  sizeChirho _ = 0

instance SizeChirho aChirho => SizeChirho (Maybe aChirho) where
  sizeChirho _ = 1 + sizeChirho (Proxy :: Proxy aChirho)

main :: IO ()
main = print (sizeChirho (Proxy :: Proxy (Maybe (Maybe (Maybe ())))))",
        ),
        "3\n",
    );
}

#[test]
fn two_contexts_of_one_class_keep_their_own_dictionaries_chirho() {
    every_engine_prints_chirho(
        &program_chirho(
            "ScopedTypeVariables",
            &format!(
                "{DESCRIBE_CHIRHO}
data PairChirho aChirho bChirho = PairChirho aChirho bChirho

instance (DescribeChirho aChirho, DescribeChirho bChirho) => DescribeChirho (PairChirho aChirho bChirho) where
  describeChirho (PairChirho xChirho yChirho) = describeChirho xChirho ++ \"&\" ++ describeChirho yChirho

main :: IO ()
main = putStrLn (describeChirho (PairChirho (1 :: Int) (PairChirho True (2 :: Int))))"
            ),
        ),
        "int1&yes&int2\n",
    );
}

#[test]
fn two_list_contexts_of_one_class_are_told_apart_by_their_predicates_chirho() {
    every_engine_prints_chirho(
        &program_chirho(
            "FlexibleContexts, ScopedTypeVariables",
            &format!(
                "{DESCRIBE_CHIRHO}
instance DescribeChirho aChirho => DescribeChirho [aChirho] where
  describeChirho xsChirho = concatMap describeChirho xsChirho

data ListsChirho aChirho bChirho = ListsChirho [aChirho] [bChirho]

instance (DescribeChirho [aChirho], DescribeChirho [bChirho]) => DescribeChirho (ListsChirho aChirho bChirho) where
  describeChirho (ListsChirho xsChirho ysChirho) = describeChirho xsChirho ++ \"|\" ++ describeChirho ysChirho

main :: IO ()
main = putStrLn (describeChirho (ListsChirho [1 :: Int, 2] [True, False]))"
            ),
        ),
        "int1int2|yesno\n",
    );
}

#[test]
fn a_recursive_dictionary_ties_its_knot_and_prints_seven_chirho() {
    every_engine_prints_chirho(
        &program_chirho(
            "FlexibleInstances, FlexibleContexts, UndecidableInstances",
            "class LoopChirho aChirho where
  loopChirho :: Proxy aChirho -> Int

instance LoopChirho [aChirho] => LoopChirho [aChirho] where
  loopChirho _ = 7

main :: IO ()
main = print (loopChirho (Proxy :: Proxy [Int]))",
        ),
        "7\n",
    );
}

#[test]
fn a_shadowing_binder_keeps_its_lexical_binding_chirho() {
    every_engine_prints_chirho(
        &program_chirho(
            "ScopedTypeVariables",
            "class SizeChirho aChirho where
  sizeChirho :: Proxy aChirho -> Int

instance SizeChirho () where
  sizeChirho _ = 0

instance SizeChirho aChirho => SizeChirho (Maybe aChirho) where
  sizeChirho _ = 1 + sizeChirho (Proxy :: Proxy aChirho)

main :: IO ()
main = do
  let sizeChirho :: Proxy (Maybe ()) -> Int
      sizeChirho _ = 42
  print (sizeChirho (Proxy :: Proxy (Maybe ())))",
        ),
        "42\n",
    );
}

#[test]
fn a_chain_with_no_base_instance_is_rejected_by_name_chirho() {
    let source_chirho = program_chirho(
        "ScopedTypeVariables",
        "class NoBaseChirho aChirho where
  noBaseChirho :: Proxy aChirho -> Int

instance NoBaseChirho aChirho => NoBaseChirho (Maybe aChirho) where
  noBaseChirho _ = 1 + noBaseChirho (Proxy :: Proxy aChirho)

main :: IO ()
main = print (noBaseChirho (Proxy :: Proxy (Maybe (Maybe Int))))",
    );
    let failure_chirho = match stg_run_chirho(&source_chirho) {
        Ok(printed_chirho) => panic!("GHC rejects this; it printed {printed_chirho:?}"),
        Err(failure_chirho) => failure_chirho,
    };
    assert!(
        failure_chirho.contains("no instance for `NoBaseChirho Int`"),
        "the missing base is named: {failure_chirho}"
    );
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert!(
            native_round_trip_chirho(&source_chirho, backend_chirho, "").is_err(),
            "{backend_chirho:?} must reject it too"
        );
    }
}
