// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 665: range syntax selects the standard Enum methods and carries evidence
//! for its endpoints, while written calls retain their source binding.
//! These controls exclude Enum Double and polymorphic Enum dictionary layout.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{assert_ghc_everywhere_chirho, run_result_chirho};
use haskelujah_driver::{compile_source_chirho, eval_modules_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_project_round_trip_chirho, native_round_trip_chirho,
};
use std::time::Duration;

const OWNED_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (enumFrom, enumFromThen, enumFromTo, enumFromThenTo)
enumFrom :: Int -> [Int]
enumFrom _ = [99]
enumFromThen :: Int -> Int -> [Int]
enumFromThen _ _ = [98]
enumFromTo :: Int -> Int -> [Int]
enumFromTo _ _ = [97]
enumFromThenTo :: Int -> Int -> Int -> [Int]
enumFromThenTo _ _ _ = [96]
main :: IO ()
main = do
  print (take 3 [1 :: Int ..])
  print (take 3 [1,3 :: Int ..])
  print [1 .. 3 :: Int]
  print [1,3 .. 7 :: Int]
  print (enumFrom 1)
  print (enumFromThen 1 3)
  print (enumFromTo 1 3)
  print (enumFromThenTo 1 3 7)
"#,
    "[1,2,3]\n[1,3,5]\n[1,2,3]\n[1,3,5,7]\n[99]\n[98]\n[97]\n[96]\n",
);

const LOCAL_CHIRHO: (&str, &str) = (
    r#"module Main where
main :: IO ()
main = do
  print [1 .. 3 :: Int]
  print (enumFromTo 1 3)
  where
    enumFromTo :: Int -> Int -> [Int]
    enumFromTo firstChirho _ = [firstChirho]
"#,
    "[1,2,3]\n[1]\n",
);

const REBINDABLE_CHIRHO: (&str, &str) = (
    r#"{-# LANGUAGE RebindableSyntax #-}
module Main where
import Prelude hiding (enumFromTo, fromInteger)
import qualified Prelude as P
fromInteger :: Integer -> Int
fromInteger valueChirho = P.negate (P.fromInteger valueChirho)
enumFromTo :: Int -> Int -> [Int]
enumFromTo firstChirho _ = [firstChirho]
main :: IO ()
main = print [3 .. 1] >> print (enumFromTo 3 1)
"#,
    "[-3,-2,-1]\n[-3]\n",
);

const NO_IMPLICIT_CHIRHO: (&str, &str) = (
    r#"{-# LANGUAGE NoImplicitPrelude #-}
module Main where
import Prelude (IO, Int, print, (>>))
enumFromTo :: Int -> Int -> [Int]
enumFromTo firstChirho _ = [firstChirho]
main :: IO ()
main = print [1 .. 3 :: Int] >> print (enumFromTo 1 3)
"#,
    "[1,2,3]\n[1]\n",
);

const USER_METHODS_CHIRHO: (&str, &str) = (
    r#"module Main where
import Prelude hiding (enumFrom, enumFromThen, enumFromTo, enumFromThenTo)
import qualified Prelude as P
data ColorChirho = RedChirho | GreenChirho | BlueChirho deriving Show
instance P.Enum ColorChirho where
  fromEnum RedChirho = 0
  fromEnum GreenChirho = 1
  fromEnum BlueChirho = 2
  toEnum 0 = RedChirho
  toEnum 1 = GreenChirho
  toEnum _ = BlueChirho
  enumFrom firstChirho = [BlueChirho, firstChirho]
  enumFromThen firstChirho nextChirho = [nextChirho, firstChirho]
  enumFromTo firstChirho lastChirho = [lastChirho, GreenChirho, firstChirho]
  enumFromThenTo firstChirho nextChirho lastChirho = [nextChirho, lastChirho, firstChirho]
enumFrom :: ColorChirho -> [ColorChirho]
enumFrom _ = []
enumFromThen :: ColorChirho -> ColorChirho -> [ColorChirho]
enumFromThen _ _ = []
enumFromTo :: ColorChirho -> ColorChirho -> [ColorChirho]
enumFromTo _ _ = []
enumFromThenTo :: ColorChirho -> ColorChirho -> ColorChirho -> [ColorChirho]
enumFromThenTo _ _ _ = []
main :: IO ()
main = do
  print [RedChirho ..]
  print [RedChirho, GreenChirho ..]
  print [RedChirho .. BlueChirho]
  print [RedChirho, GreenChirho .. BlueChirho]
  print (enumFrom RedChirho)
  print (enumFromThen RedChirho GreenChirho)
  print (enumFromTo RedChirho BlueChirho)
  print (enumFromThenTo RedChirho GreenChirho BlueChirho)
"#,
    "[BlueChirho,RedChirho]\n[GreenChirho,RedChirho]\n[BlueChirho,GreenChirho,RedChirho]\n[GreenChirho,BlueChirho,RedChirho]\n[]\n[]\n[]\n[]\n",
);

const ENDPOINTS_CHIRHO: (&str, &str) = (
    r#"module Main where
startChirho :: Int
startChirho = 1
main :: IO ()
main = do
  print [id startChirho .. 3]
  print ['a' .. 'e']
  print ['e','c' .. 'a']
  print ([1 .. 3] :: [Int])
  print (take 3 [5 :: Int ..])
  print [5 .. 1 :: Int]
  print (sum [1 .. 10 :: Int])
  print [1 .. 3]
"#,
    "[1,2,3]\n\"abcde\"\n\"eca\"\n[1,2,3]\n[5,6,7]\n[]\n55\n[1,2,3]\n",
);

const IMPORTED_MODULE_CHIRHO: &str = r#"module RangeSupplierChirho (enumFromTo) where
import Prelude hiding (enumFromTo)
enumFromTo :: Int -> Int -> [Int]
enumFromTo firstChirho _ = [firstChirho]
"#;

const IMPORTED_MAIN_CHIRHO: &str = r#"module Main where
import Prelude hiding (enumFromTo)
import RangeSupplierChirho (enumFromTo)
main :: IO ()
main = do
  print [1 .. 3 :: Int]
  print (enumFromTo 1 3)
"#;

#[test]
fn an_imported_namesake_never_supplies_the_range_method_chirho() {
    let sources_chirho = [
        ("RangeSupplierChirho.hs", IMPORTED_MODULE_CHIRHO),
        ("Main.hs", IMPORTED_MAIN_CHIRHO),
    ];
    let (_, machine_chirho) =
        eval_modules_with_machine_chirho(&sources_chirho, &mut SourceMapChirho::new_chirho(), None)
            .expect("the imported namesake program compiles");
    assert_eq!(machine_chirho.io_output_chirho, "[1,2,3]\n[1]\n");
    let modules_chirho = [
        ("RangeSupplierChirho", IMPORTED_MODULE_CHIRHO),
        ("Main", IMPORTED_MAIN_CHIRHO),
    ];
    for backend_chirho in [
        NativeBackendChirho::LlvmChirho,
        NativeBackendChirho::CraneliftChirho,
    ] {
        assert_eq!(
            native_project_round_trip_chirho(
                &modules_chirho,
                backend_chirho,
                "",
                Duration::from_secs(30),
            )
            .unwrap_or_else(|error_chirho| panic!("{backend_chirho:?}: {error_chirho}")),
            (0, "[1,2,3]\n[1]\n".to_string()),
        );
    }
}

#[test]
fn ranges_keep_enum_identity_beside_owned_names_everywhere_chirho() {
    for (name_chirho, (source_chirho, output_chirho)) in [
        ("RangeOwnedChirho.hs", OWNED_CHIRHO),
        ("RangeLocalChirho.hs", LOCAL_CHIRHO),
        ("RangeRebindableChirho.hs", REBINDABLE_CHIRHO),
        ("RangeNoImplicitChirho.hs", NO_IMPLICIT_CHIRHO),
    ] {
        assert_ghc_everywhere_chirho(name_chirho, source_chirho, output_chirho);
    }
}

#[test]
fn ranges_call_the_instances_written_methods_everywhere_chirho() {
    assert_ghc_everywhere_chirho(
        "RangeUserMethodsChirho.hs",
        USER_METHODS_CHIRHO.0,
        USER_METHODS_CHIRHO.1,
    );
}

#[test]
fn range_endpoints_determine_the_list_and_show_instance_everywhere_chirho() {
    assert_ghc_everywhere_chirho(
        "RangeEndpointsChirho.hs",
        ENDPOINTS_CHIRHO.0,
        ENDPOINTS_CHIRHO.1,
    );
}

const NO_ENUM_CHIRHO: &str = r#"module Main where
data NoEnumChirho = NoEnumChirho deriving Show
main :: IO ()
main = print [NoEnumChirho .. NoEnumChirho]
"#;

const MISMATCH_CHIRHO: &str = r#"module Main where
main :: IO ()
main = print ['a', True .. 'z']
"#;

/// Every path must reject at the frontend; no generated program is run.
#[test]
fn range_endpoints_require_one_enum_type_chirho() {
    for (name_chirho, source_chirho, expected_types_chirho) in [
        (
            "RangeNoEnumChirho.hs",
            NO_ENUM_CHIRHO,
            ["Enum", "NoEnumChirho"],
        ),
        ("RangeMismatchChirho.hs", MISMATCH_CHIRHO, ["Char", "Bool"]),
        (
            "tcfail009.hs",
            include_str!(
                "../../../../../ghc-tests-chirho/typecheck-chirho/should_fail/tcfail009.hs"
            ),
            ["Int", "Integer"],
        ),
    ] {
        let mut map_chirho = SourceMapChirho::new_chirho();
        let diagnostics_chirho =
            match compile_source_chirho(source_chirho, &mut map_chirho, name_chirho) {
                Err(diagnostics_chirho) => format!("{diagnostics_chirho:?}"),
                Ok(_) => panic!("{name_chirho}: the frontend accepted GHC-invalid endpoints"),
            };
        for expected_chirho in expected_types_chirho {
            assert!(
                diagnostics_chirho.contains(expected_chirho),
                "{name_chirho}: wrong refusal: {diagnostics_chirho}"
            );
        }
        assert!(
            run_result_chirho(name_chirho, source_chirho).is_err(),
            "{name_chirho}: the interpreter accepted GHC-invalid endpoints"
        );
        for backend_chirho in [
            NativeBackendChirho::LlvmChirho,
            NativeBackendChirho::CraneliftChirho,
        ] {
            assert!(
                native_round_trip_chirho(source_chirho, backend_chirho, "").is_err(),
                "{name_chirho} {backend_chirho:?}: compiled GHC-invalid endpoints"
            );
        }
    }
}
