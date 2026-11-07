// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 709: ordinary append keeps its selected value identity and lazy list semantics.
//! Expectations are specified predictions until the scheduler runs these sources.
//! Workflow: compiler-pipeline-chirho/native-symbol-identity-chirho.md#polymorphic-list-append-chirho.

use super::{assert_all_engines_chirho, assert_project_chirho};
use haskelujah_span_chirho::SourceMapChirho;

const HEADER_CHIRHO: &str = "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n";

fn source_chirho(body_chirho: &str, expected_chirho: &str) {
    let text_chirho = format!("{HEADER_CHIRHO}{body_chirho}");
    assert_all_engines_chirho((&text_chirho, expected_chirho));
}

fn project_chirho(modules_chirho: &[(&str, &str)], expected_chirho: &str) {
    let owned_chirho = modules_chirho
        .iter()
        .map(|(name_chirho, body_chirho)| (*name_chirho, format!("{HEADER_CHIRHO}{body_chirho}")))
        .collect::<Vec<_>>();
    let borrowed_chirho = owned_chirho
        .iter()
        .map(|(name_chirho, body_chirho)| (*name_chirho, body_chirho.as_str()))
        .collect::<Vec<_>>();
    assert_project_chirho((&borrowed_chirho, expected_chirho));
}

#[test]
fn typed_strings_integers_and_string_lists_append_in_all_four_syntax_forms_chirho() {
    source_chirho(
        r#"module Main where
main :: IO ()
main = do
  putStrLn ("a" ++ "b")
  putStrLn ((++) "a" "b")
  putStrLn (("a" ++) "b")
  putStrLn ((++ "b") "a")
"#,
        "ab\nab\nab\nab\n",
    );
    source_chirho(
        r#"module Main where
reportChirho :: [Int] -> IO ()
reportChirho valuesChirho = print valuesChirho
main :: IO ()
main = do
  reportChirho ([1] ++ [2])
  reportChirho ((++) [1] [2])
  reportChirho (([1] ++) [2])
  reportChirho ((++ [2]) [1])
"#,
        "[1,2]\n[1,2]\n[1,2]\n[1,2]\n",
    );
    source_chirho(
        r#"module Main where
reportChirho :: [String] -> IO ()
reportChirho rowsChirho = mapM_ putStrLn rowsChirho
main :: IO ()
main = do
  reportChirho (["a"] ++ ["b"])
  reportChirho ((++) ["a"] ["b"])
  reportChirho ((["a"] ++) ["b"])
  reportChirho ((++ ["b"]) ["a"])
"#,
        "a\nb\na\nb\na\nb\na\nb\n",
    );
}

#[test]
fn append_does_not_demand_unused_tail_or_elements_chirho() {
    source_chirho(
        r#"module Main where
valuesChirho :: [Int]
valuesChirho = [undefined] ++ error "eager append tail Chirho"
main :: IO ()
main = print (length (take 1 valuesChirho))
"#,
        "1\n",
    );
}

#[test]
fn owned_operators_and_source_append_namesakes_keep_their_bodies_chirho() {
    source_chirho(
        r#"module Main where
import Prelude hiding ((++))
(++) :: Int -> Int -> Int
leftChirho ++ rightChirho = leftChirho - rightChirho
main :: IO ()
main = do
  print (9 ++ 2)
  print ((++) 9 2)
  print ((9 ++) 2)
  print ((++ 2) 9)
  print ((Main.++) 9 2)
"#,
        "7\n7\n7\n7\n7\n",
    );
    source_chirho(
        r#"module Main where
append :: [Int] -> [Int] -> [Int]
append leftChirho rightChirho = [99]
main :: IO ()
main = print ([1 :: Int] ++ [2])
"#,
        "[1,2]\n",
    );
    source_chirho(
        r#"module Main where
main :: IO ()
main = let leftChirho ++ rightChirho = leftChirho
       in putStrLn ("left" ++ "right")
"#,
        "left\n",
    );
}

#[test]
fn imported_source_operator_and_real_forwarded_builtin_stay_distinct_chirho() {
    project_chirho(
        &[
            (
                "OwnAppendChirho",
                r#"module OwnAppendChirho ((++)) where
import Prelude hiding ((++))
(++) :: Int -> Int -> Int
leftChirho ++ rightChirho = leftChirho - rightChirho
"#,
            ),
            (
                "Main",
                r#"module Main where
import qualified OwnAppendChirho as P
main :: IO ()
main = do
  print (9 P.++ 2)
  print ((P.++) 9 2)
  print ((9 P.++) 2)
  print ((P.++ 2) 9)
"#,
            ),
        ],
        "7\n7\n7\n7\n",
    );
    project_chirho(
        &[
            (
                "ForwardAppendChirho",
                r#"module ForwardAppendChirho ((++)) where
import Prelude ((++))
"#,
            ),
            (
                "Main",
                r#"module Main where
import qualified ForwardAppendChirho as F
main :: IO ()
main = mapM_ putStrLn (["a"] F.++ ["b"])
"#,
            ),
        ],
        "a\nb\n",
    );
}

#[test]
fn declared_data_list_append_agrees_with_implicit_prelude_and_qualified_forms_chirho() {
    source_chirho(
        r#"module Main where
import Data.List ((++))
import qualified Data.List as ListChirho
reportChirho :: [String] -> IO ()
reportChirho rowsChirho = mapM_ putStrLn rowsChirho
main :: IO ()
main = do
  print ([1 :: Int] ++ [2])
  putStrLn ((ListChirho.++) "a" "b")
  reportChirho (["a"] ListChirho.++ ["b"])
  reportChirho ((ListChirho.++) ["a"] ["b"])
  reportChirho ((["a"] ListChirho.++) ["b"])
  reportChirho ((ListChirho.++ ["b"]) ["a"])
"#,
        "[1,2]\nab\na\nb\na\nb\na\nb\na\nb\n",
    );
}

#[test]
fn selected_source_data_list_append_keeps_its_own_operator_chirho() {
    project_chirho(
        &[
            (
                "Data.List",
                r#"module Data.List ((++)) where
import Prelude hiding ((++))
(++) :: Int -> Int -> Int
leftChirho ++ rightChirho = leftChirho - rightChirho
"#,
            ),
            (
                "Main",
                r#"module Main where
import qualified Data.List as ListChirho
main :: IO ()
main = do
  print ([1 :: Int] ++ [2])
  print (9 ListChirho.++ 2)
  print ((ListChirho.++) 9 2)
  print ((9 ListChirho.++) 2)
  print ((ListChirho.++ 2) 9)
"#,
            ),
        ],
        "[1,2]\n7\n7\n7\n7\n",
    );
}

#[test]
fn selected_source_prelude_replaces_fallback_but_explicit_base_keeps_its_exposure_chirho() {
    project_chirho(
        &[
            (
                "Prelude",
                r#"{-# LANGUAGE NoImplicitPrelude #-}
module Prelude ((++)) where
leftChirho ++ rightChirho = rightChirho
"#,
            ),
            (
                "Main",
                r#"{-# LANGUAGE NoImplicitPrelude, PackageImports #-}
module Main where
import qualified "base" Prelude as B
import qualified Prelude as P
main :: B.IO ()
main = B.print ((P.++) [1 :: B.Int] [2], (B.++) [1 :: B.Int] [2])
"#,
            ),
        ],
        "([2],[1,2])\n",
    );
}

#[test]
fn hidden_or_unimported_append_never_acquires_a_builtin_body_chirho() {
    for body_chirho in [
        r#"{-# LANGUAGE NoImplicitPrelude #-}
module Main where
import Prelude (IO, putStrLn)
main :: IO ()
main = putStrLn ("a" ++ "b")
"#,
        r#"module Main where
import qualified Prelude as P hiding ((++))
main :: IO ()
main = putStrLn ("a" P.++ "b")
"#,
        r#"module Main where
main :: IO ()
main = putStrLn ("a" UnknownAppendChirho.++ "b")
"#,
        r#"module Main where
import qualified Data.List as ListChirho hiding ((++))
main :: IO ()
main = putStrLn ("a" ListChirho.++ "b")
"#,
        r#"module Main where
import qualified Data.List as ListChirho
rowsChirho :: [Int]
rowsChirho = [1] ListChirho.++ ["wrong element Chirho"]
main :: IO ()
main = print rowsChirho
"#,
    ] {
        let text_chirho = format!("{HEADER_CHIRHO}{body_chirho}");
        let result_chirho = haskelujah_driver::compile_source_chirho(
            &text_chirho,
            &mut SourceMapChirho::new_chirho(),
            "Main.hs",
        );
        assert!(
            result_chirho.is_err(),
            "an out-of-scope append compiled: {body_chirho}"
        );
    }
}
