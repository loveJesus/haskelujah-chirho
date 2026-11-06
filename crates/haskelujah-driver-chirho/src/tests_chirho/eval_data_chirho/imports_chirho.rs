// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Module import evaluation tests: synthetic modules, ImportQualifiedPost and PackageImports (row 700).

use crate::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

// ── Synthetic module imports ─────────────────────────────────────────

#[test]
fn eval_import_data_map_chirho() {
    // import Data.Map functions via synthetic module interface
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Map (mapInsert, mapLookup, mapEmpty)
main = case mapLookup 1 (mapInsert 1 99 mapEmpty) of
         Just x  -> x
         Nothing -> 0
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
        ),
        Err(e_chirho) => panic!("import Data.Map: {}", e_chirho),
    }
}

#[test]
fn eval_import_data_map_size_chirho() {
    // import Data.Map, use mapSize
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
import Data.Map
main = Map.size (Map.insert 2 20 (Map.insert 1 10 Map.empty))
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(2)
        ),
        Err(e_chirho) => panic!("import Data.Map size: {}", e_chirho),
    }
}

#[test]
fn eval_import_qualified_data_map_chirho() {
    // import qualified Data.Map as Map
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as Map
main = case Map.mapLookup 42 (Map.mapInsert 42 100 Map.mapEmpty) of
         Just v  -> v
         Nothing -> 0
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(100)
        ),
        Err(e_chirho) => panic!("import qualified Data.Map: {}", e_chirho),
    }
}

#[test]
fn eval_import_qualified_data_map_no_alias_chirho() {
    // import qualified Data.Map (no alias) → use Data.Map.mapInsert
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map
main = case Data.Map.mapLookup 42 (Data.Map.mapInsert 42 100 Data.Map.mapEmpty) of
         Just v  -> v
         Nothing -> 0
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(100)
        ),
        Err(e_chirho) => panic!("import qualified Data.Map (no alias): {}", e_chirho),
    }
}

#[test]
fn eval_import_qualified_data_map_real_exports_chirho() {
    // Real Data.Map export names should route to backed map* Prelude wrappers.
    use crate::{eval_source_chirho, eval_source_with_machine_chirho};
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import qualified Data.Map as M
main = case M.lookup 2 (M.insert 2 20 (M.fromList [(1, 10)])) of
         Just v  -> v
         Nothing -> 0
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
        ),
        Err(e_chirho) => panic!("qualified Data.Map real exports: {}", e_chirho),
    }

    let mut print_sm_chirho = SourceMapChirho::new_chirho();
    let print_src_chirho = "\
module Test where
import qualified Data.Map as M
main = print (M.size (M.insert 2 \"b\" (M.fromList [(1, \"a\")])))
";
    let print_result_chirho = eval_source_with_machine_chirho(
        print_src_chirho,
        &mut print_sm_chirho,
        "TestChirho.hs",
        None,
    );
    match print_result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho, "2\n");
        }
        Err(e_chirho) => panic!("qualified Data.Map real export print: {}", e_chirho),
    }
}

#[test]
fn eval_import_data_set_chirho() {
    // import Data.Set functions
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "\
module Test where
import Data.Set (setInsert, setMember, setEmpty)
main = case setMember 5 (setInsert 5 (setInsert 3 setEmpty)) of
         True  -> 1
         False -> 0
";
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None);
    match result_chirho {
        Ok(val_chirho) => assert_eq!(
            val_chirho,
            haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
        ),
        Err(e_chirho) => panic!("import Data.Set: {}", e_chirho),
    }
}

// ── ImportQualifiedPost test ────────────────────────────────────────

#[test]
fn import_qualified_post_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE ImportQualifiedPost #-}
module Test where
import Data.Map qualified as Map
main = print (Map.mapSize (Map.mapInsert 1 "a" Map.mapEmpty))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("ImportQualifiedPost failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "1\n");
}

// ── PackageImports test ─────────────────────────────────────────────

#[test]
fn package_imports_chirho() {
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"
{-# LANGUAGE PackageImports #-}
module Test where
import "base" Data.List
main = putStrLn "works"
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None)
            .unwrap_or_else(|e_chirho| panic!("PackageImports failed: {}", e_chirho));
    assert_eq!(m_chirho.io_output_chirho, "works\n");
}
