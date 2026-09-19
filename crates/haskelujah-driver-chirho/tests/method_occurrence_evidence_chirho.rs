// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A class method's evidence belongs to the SOURCE REFERENCE that asked for it.
//! The checker and the desugarer used to agree only on how MANY references a name
//! had, and the join matched them by position: the checker counts in
//! inference-visit order (instance bodies last), the desugarer in declaration
//! order. So `main`'s `show` and an instance body's `show` exchanged evidence and
//! the program's meaning depended on which declaration came first. Every output
//! below is GHC 9.14.1's, measured 2026-09-19.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn output_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
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

const INSTANCE_FIRST_CHIRHO: &str = "module Main where
data WChirho = WChirho Int
instance Show WChirho where
  show (WChirho n) = \"W\" ++ show n
main :: IO ()
main = putStrLn (show (WChirho 3))
";

const MAIN_FIRST_CHIRHO: &str = "module Main where
data WChirho = WChirho Int
main :: IO ()
main = putStrLn (show (WChirho 3))
instance Show WChirho where
  show (WChirho n) = \"W\" ++ show n
";

#[test]
fn declaration_order_does_not_change_the_answer_chirho() {
    assert_eq!(
        output_chirho("InstanceFirstChirho.hs", INSTANCE_FIRST_CHIRHO),
        "W3\n"
    );
    assert_eq!(
        output_chirho("MainFirstChirho.hs", MAIN_FIRST_CHIRHO),
        "W3\n"
    );
}

#[test]
fn a_newtype_field_does_not_escape_its_own_instance_chirho() {
    // Printed `3` before: the occurrence took the inner `show n`'s Int evidence,
    // and newtype erasure handed the raw field to showInt#.
    let source_chirho = "module Main where
newtype VChirho = VChirho Int
instance Show VChirho where
  show (VChirho n) = \"V\" ++ show n
main :: IO ()
main = putStrLn (show (VChirho 3))
";
    assert_eq!(
        output_chirho("NewtypeFieldChirho.hs", source_chirho),
        "V3\n"
    );
}

#[test]
fn a_nullary_and_a_one_field_instance_coexist_chirho() {
    // This shape did not print a wrong answer before: it died with
    // "no matching alternative for tag 66".
    let source_chirho = "module Main where
data PChirho = PChirho
instance Show PChirho where
  show _ = \"pp\"
data WChirho = WChirho Int
instance Show WChirho where
  show (WChirho n) = \"W\" ++ show n
main :: IO ()
main = do
  putStrLn (show PChirho)
  putStrLn (show (WChirho 3))
";
    assert_eq!(
        output_chirho("NullaryAndFieldChirho.hs", source_chirho),
        "pp\nW3\n"
    );
}

#[test]
fn an_inner_reference_bound_in_a_where_keeps_its_own_evidence_chirho() {
    // The `where` form used to reach the fallback path and die with
    // "no matching alternative for tag 0".
    let source_chirho = "module Main where
data WChirho = WChirho Int
instance Show WChirho where
  show (WChirho n) = tagChirho
    where
      tagChirho = \"W\" ++ show n
main :: IO ()
main = putStrLn (show (WChirho 3))
";
    assert_eq!(output_chirho("WhereBoundChirho.hs", source_chirho), "W3\n");
}

#[test]
fn two_instances_and_two_uses_each_take_their_own_chirho() {
    let source_chirho = "module Main where
data WChirho = WChirho Int
data ZChirho = ZChirho Int
instance Show WChirho where
  show (WChirho n) = \"W\" ++ show n
instance Show ZChirho where
  show (ZChirho n) = \"Z\" ++ show n
main :: IO ()
main = putStrLn (show (WChirho 3) ++ show (ZChirho 4))
";
    assert_eq!(
        output_chirho("TwoInstancesChirho.hs", source_chirho),
        "W3Z4\n"
    );
}

const ALPHA_CHIRHO: &str = "module AlphaChirho where
class TagChirho a where
  tagChirho :: a -> Int
data AChirho = AChirho
instance TagChirho AChirho where
  tagChirho _ = 1
";

const BETA_CHIRHO: &str = "module BetaChirho where
import AlphaChirho
data BChirho = BChirho
instance TagChirho BChirho where
  tagChirho _ = 2
";

fn cross_module_compiles_chirho(main_chirho: &str) {
    // What this control can and cannot assert today. `eval_modules_chirho` returns
    // the entry value UNFORCED (a heap pointer) and hands back no machine, so a
    // cross-module dispatch cannot be asserted by value or by output through it,
    // and `haskelujah run` cannot resolve sibling modules at all (measured
    // 2026-09-19). So this pins the frontend half: the instance in another module
    // is visible to the reference here, in either import order. GHC 9.14.1 runs
    // the `print` form of both and prints 12.
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    haskelujah_driver::compile_modules_chirho(
        &[
            ("AlphaChirho.hs", ALPHA_CHIRHO),
            ("BetaChirho.hs", BETA_CHIRHO),
            ("Main.hs", main_chirho),
        ],
        &mut source_map_chirho,
    )
    .unwrap_or_else(|diag_chirho| panic!("the cross-module program must compile: {diag_chirho:?}"));
}

#[test]
fn an_instance_from_another_module_is_selected_by_its_own_reference_chirho() {
    cross_module_compiles_chirho(
        "module Main where
import AlphaChirho
import BetaChirho
main = print (tagChirho AChirho * 10 + tagChirho BChirho)
",
    );
}

#[test]
fn the_import_order_does_not_change_the_selection_chirho() {
    cross_module_compiles_chirho(
        "module Main where
import BetaChirho
import AlphaChirho
main = print (tagChirho AChirho * 10 + tagChirho BChirho)
",
    );
}
