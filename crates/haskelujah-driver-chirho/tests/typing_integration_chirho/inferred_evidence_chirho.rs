// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Inferred constrained bindings retain their own predicate indices (row 663).
//! These sources are also the oracle inputs; execution and classification are recorded in
//! the row's receipts. Neither a literal nor another parameter determines a method's key.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::assert_prints_everywhere_chirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_project_round_trip_chirho,
};

const INFERRED_NUM_CHIRHO: [(&str, &str, &str); 6] = [
    (
        "OwnedMixedPlus664Chirho.hs",
        r#"module Main where
import Prelude hiding ((+))
aChirho + bChirho = fromIntegral aChirho * bChirho
main :: IO ()
main = print ((3 :: Int) + (4.5 :: Double))
"#,
        "13.5\n",
    ),
    (
        "InferredIncrementChirho.hs",
        r#"module Main where
incrementChirho valueChirho = valueChirho + 1
main :: IO ()
main = print (incrementChirho 2.5)
"#,
        "3.5\n",
    ),
    (
        "InferredIncrementMixedChirho.hs",
        r#"module Main where
incrementChirho valueChirho = valueChirho + 1
main :: IO ()
main = do
  print (incrementChirho (2 :: Int))
  print (incrementChirho 2.5)
"#,
        "3\n3.5\n",
    ),
    (
        "InferredSumOneChirho.hs",
        r#"module Main where
sumOneChirho leftChirho rightChirho = leftChirho + rightChirho + 1
main :: IO ()
main = do
  print (sumOneChirho 2 3 :: Int)
  print (sumOneChirho 2.5 1)
"#,
        "6\n4.5\n",
    ),
    (
        "InferredWrittenConversionChirho.hs",
        r#"module Main where
incrementChirho valueChirho = valueChirho + fromInteger (1 :: Integer)
main :: IO ()
main = print (incrementChirho (2.5 :: Double))
"#,
        "3.5\n",
    ),
    (
        "InferredOperatorOnlyChirho.hs",
        r#"module Main where
addChirho leftChirho rightChirho = leftChirho + rightChirho
main :: IO ()
main = do
  print (addChirho (2 :: Int) 3)
  print (addChirho (2.5 :: Double) 1)
"#,
        "5\n3.5\n",
    ),
];

const SIGNED_AND_LOCAL_HOLDS_CHIRHO: [(&str, &str, &str); 9] = [
    (
        "OwnedShowDirectChirho.hs",
        r#"module Main where
import Prelude hiding (show)
import qualified Prelude as P
show valueChirho = "<" ++ P.show valueChirho ++ ">"
main :: IO ()
main = putStrLn (show (42 :: Int))
"#,
        "<42>\n",
    ),
    (
        "OwnedAndPreludeAbsChirho.hs",
        r#"module Main where
import Prelude hiding (abs)
import qualified Prelude as P
abs valueChirho = (1, valueChirho P.+ 1)
main :: IO ()
main = do
  print (fst (abs (2.5 :: Double)) :: Int)
  print (P.abs (-3 :: Int))
"#,
        "1\n3\n",
    ),
    (
        "ResultOnlyNumPairChirho.hs",
        r#"module Main where
pairChirho valueChirho = (1, valueChirho + 1)
main :: IO ()
main = print (fst (pairChirho (2.5 :: Double)) :: Int)
"#,
        "1\n",
    ),
    (
        "OwnedResultOnlyNumPairChirho.hs",
        r#"module Main where
import Prelude hiding (abs)
import qualified Prelude as P
abs valueChirho = (1, valueChirho P.+ 1)
main :: IO ()
main = print (fst (abs (2.5 :: Double)) :: Int)
"#,
        "1\n",
    ),
    (
        "SignedIncrementMixedChirho.hs",
        r#"module Main where
incrementChirho :: Num aChirho => aChirho -> aChirho
incrementChirho valueChirho = valueChirho + 1
main :: IO ()
main = do
  print (incrementChirho (2 :: Int))
  print (incrementChirho 2.5)
"#,
        "3\n3.5\n",
    ),
    (
        "SignedSumOneChirho.hs",
        r#"module Main where
sumOneChirho :: Num aChirho => aChirho -> aChirho -> aChirho
sumOneChirho leftChirho rightChirho = leftChirho + rightChirho + 1
main :: IO ()
main = do
  print (sumOneChirho 2 3 :: Int)
  print (sumOneChirho 2.5 1)
"#,
        "6\n4.5\n",
    ),
    (
        "SignedOuterLocalIncrementChirho.hs",
        r#"module Main where
outerChirho :: Num aChirho => aChirho -> aChirho
outerChirho valueChirho = let incrementChirho innerChirho = innerChirho + 1 in incrementChirho valueChirho
main :: IO ()
main = print (outerChirho (2.5 :: Double))
"#,
        "3.5\n",
    ),
    (
        "LocalIntSpecializationChirho.hs",
        r#"module Main where
outerChirho valueChirho = let incrementChirho innerChirho = innerChirho + 1 in incrementChirho (valueChirho :: Int)
main :: IO ()
main = print (outerChirho 2)
"#,
        "3\n",
    ),
    (
        "RecursiveIntGroupChirho.hs",
        r#"module Main where
evenChirho valueChirho = if valueChirho == (0 :: Int) then True else oddChirho (valueChirho - 1)
oddChirho valueChirho = if valueChirho == (0 :: Int) then False else evenChirho (valueChirho - 1)
main :: IO ()
main = do
  print (evenChirho 8)
  print (oddChirho 9)
"#,
        "True\nTrue\n",
    ),
];

const TWO_NUM_PARAMETERS_CHIRHO: (&str, &str, &str) = (
    "TwoInferredNumParametersChirho.hs",
    r#"module Main where
bothChirho leftChirho rightChirho = (leftChirho + 1, rightChirho + 2)
main :: IO ()
main = do
  print (fst (bothChirho (2 :: Int) (3.5 :: Double)))
  print (snd (bothChirho (2 :: Int) (3.5 :: Double)))
  print (fst (bothChirho (2.5 :: Double) (3 :: Int)))
  print (snd (bothChirho (2.5 :: Double) (3 :: Int)))
"#,
    "3\n5.5\n3.5\n5\n",
);

const TWO_SHOW_PARAMETERS_CHIRHO: (&str, &str, &str) = (
    "TwoInferredShowParametersChirho.hs",
    r#"module Main where
leftChirho |+| rightChirho = show leftChirho ++ show rightChirho
main :: IO ()
main = putStrLn ((1 :: Int) |+| True)
"#,
    "1True\n",
);

#[test]
fn inferred_numeric_methods_and_literals_use_the_binding_dictionary_chirho() {
    for (name_chirho, source_chirho, expected_chirho) in INFERRED_NUM_CHIRHO {
        assert_prints_everywhere_chirho(name_chirho, source_chirho, expected_chirho);
    }
}

#[test]
fn inferred_dictionary_indices_keep_two_num_parameters_apart_chirho() {
    let (name_chirho, source_chirho, expected_chirho) = TWO_NUM_PARAMETERS_CHIRHO;
    assert_prints_everywhere_chirho(name_chirho, source_chirho, expected_chirho);
}

#[test]
fn signed_local_and_recursive_dictionary_routes_keep_their_answers_chirho() {
    for (name_chirho, source_chirho, expected_chirho) in SIGNED_AND_LOCAL_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(name_chirho, source_chirho, expected_chirho);
    }
}

#[test]
fn inferred_dictionary_indices_keep_two_show_parameters_apart_chirho() {
    let (name_chirho, source_chirho, expected_chirho) = TWO_SHOW_PARAMETERS_CHIRHO;
    assert_prints_everywhere_chirho(name_chirho, source_chirho, expected_chirho);
}

#[test]
fn imported_owned_names_keep_reference_evidence_under_each_qualifier_chirho() {
    let modules_chirho = [
        (
            "OwnedChirho",
            r#"module OwnedChirho (abs) where
import Prelude hiding (abs)
import qualified Prelude as P
abs valueChirho = valueChirho P.+ 1
"#,
        ),
        (
            "Main",
            r#"module Main where
import Prelude hiding (abs)
import OwnedChirho (abs)
import qualified OwnedChirho
import qualified OwnedChirho as AliasChirho
main :: IO ()
main = do
  print (abs (2.5 :: Double))
  print (OwnedChirho.abs (3.5 :: Double))
  print (AliasChirho.abs (4.5 :: Double))
"#,
        ),
    ];
    let sources_chirho: Vec<(String, &str)> = modules_chirho
        .iter()
        .map(|(name_chirho, source_chirho)| (format!("{name_chirho}.hs"), *source_chirho))
        .collect();
    let borrowed_chirho: Vec<(&str, &str)> = sources_chirho
        .iter()
        .map(|(name_chirho, source_chirho)| (name_chirho.as_str(), *source_chirho))
        .collect();
    let (_, machine_chirho) = haskelujah_driver::eval_modules_with_machine_chirho(
        &borrowed_chirho,
        &mut SourceMapChirho::new_chirho(),
        None,
    )
    .expect("imported owned names compile and run on the interpreter");
    let expected_chirho = "3.5\n4.5\n5.5\n";
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho);
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_project_round_trip_chirho(
                &modules_chirho,
                backend_chirho,
                "",
                std::time::Duration::from_secs(30),
            )
            .expect("an imported namesake uses the same predicate evidence"),
            (0, expected_chirho.to_string()),
            "{backend_chirho:?}",
        );
    }
}
