// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Multi-module programs: cross-module calls, data types and re-exports (row 700).

use haskelujah_span_chirho::SourceMapChirho;

// -----------------------------------------------------------------------
// Multi-module evaluation tests
// -----------------------------------------------------------------------

// -----------------------------------------------------------------------
// Multi-module evaluation tests
// -----------------------------------------------------------------------

#[test]
fn eval_multi_module_function_call_chirho() {
    use crate::eval_modules_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Lib defines add1, Main imports Lib and calls add1
    let lib_chirho = "\
module Lib where
add1 x = x + 1
";
    let main_chirho = "\
module Main where
import Lib
main = add1 42
";
    let result_chirho = eval_modules_chirho(
        &[("Lib.hs", lib_chirho), ("Main.hs", main_chirho)],
        &mut source_map_chirho,
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(43),
            );
        }
        Err(e_chirho) => panic!("multi-module function call should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_multi_module_data_type_chirho() {
    use crate::eval_modules_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // Lib defines Color data type and colorToInt, Main uses them
    let lib_chirho = "\
module Lib where
data Color = Red | Green | Blue
colorToInt c = case c of
  Red -> 1
  Green -> 2
  Blue -> 3
";
    let main_chirho = "\
module Main where
import Lib
main = colorToInt Green
";
    let result_chirho = eval_modules_chirho(
        &[("Lib.hs", lib_chirho), ("Main.hs", main_chirho)],
        &mut source_map_chirho,
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(2),
            );
        }
        Err(e_chirho) => panic!("multi-module data type should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_multi_module_three_modules_chirho() {
    use crate::eval_modules_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    // A defines bump, B imports A and defines bumpTwo, Main imports B
    let mod_a_chirho = "\
module A where
bump x = x + 1
";
    let mod_b_chirho = "\
module B where
import A
bumpTwo x = bump (bump x)
";
    let mod_main_chirho = "\
module Main where
import B
main = bumpTwo 10
";
    let result_chirho = eval_modules_chirho(
        &[
            ("A.hs", mod_a_chirho),
            ("B.hs", mod_b_chirho),
            ("Main.hs", mod_main_chirho),
        ],
        &mut source_map_chirho,
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(12),
            );
        }
        Err(e_chirho) => panic!("3-module chain should evaluate: {}", e_chirho),
    }
}

#[test]
fn eval_module_re_export_chirho() {
    // Inner defines add1; Reexporter re-exports Inner; Main imports Reexporter.
    use crate::eval_modules_chirho;
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let inner_chirho = "\
module Inner where
add1 x = x + 1
";
    let reexporter_chirho = "\
module Reexporter (module Inner) where
import Inner
";
    let main_chirho = "\
module Main where
import Reexporter
main = add1 99
";
    let result_chirho = eval_modules_chirho(
        &[
            ("Inner.hs", inner_chirho),
            ("Reexporter.hs", reexporter_chirho),
            ("Main.hs", main_chirho),
        ],
        &mut source_map_chirho,
        None,
    );
    match &result_chirho {
        Ok(val_chirho) => {
            assert_eq!(
                *val_chirho,
                haskelujah_runtime_chirho::ValueChirho::IntChirho(100),
            );
        }
        Err(e_chirho) => panic!("module re-export should evaluate: {}", e_chirho),
    }
}
