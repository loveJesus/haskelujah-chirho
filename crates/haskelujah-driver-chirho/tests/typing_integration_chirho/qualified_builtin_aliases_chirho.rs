// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A qualified builtin alias takes its bare builtin scheme where GHC 9.14.1 agrees (row 608).
//!
//! `import qualified Data.List as L` typed `L.reverse` as the placeholder `forall a. a`, so
//! `print (L.reverse xs)` defaulted Show to Int: natives failed "no native Show instance for a
//! boxed value", and beside a local binder every engine failed "no matching alternative". The
//! generated table now gives such an alias its bare scheme exactly where GHC types the export so.
//! Every program and expectation is GHC 9.14.1's, embedded byte for byte from the measured file.
//! workflow: compiler-pipeline-chirho/builtin-type-environment-chirho

use haskelujah_driver::{compile_source_with_search_path_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_test_harness_chirho::native_chirho::{
    NativeBackendChirho, native_round_trip_chirho,
};
use haskelujah_typing_chirho::infer_chirho::{
    builtin_value_schemes_chirho, qualified_builtin_alias_scheme_chirho,
};

use super::assert_prints_everywhere_chirho;

const BACKENDS_CHIRHO: [NativeBackendChirho; 2] = [
    NativeBackendChirho::LlvmChirho,
    NativeBackendChirho::CraneliftChirho,
];

/// Agreeing aliases from Prelude, Data.List and Data.Char. Each typed as the
/// placeholder before, so every engine failed at the first `print`.
const AGREEING_ALIASES_CHIRHO: &str = r#"module Main where
import qualified Data.Char as C
import qualified Data.List as L
import qualified Prelude as P
import Prelude
main :: IO ()
main = do
  print (L.reverse [1, 2, 3 :: Int])
  print (L.sort [3, 1, 2 :: Int], L.nub [1, 1, 2 :: Int])
  print (C.toUpper 'a', map C.toUpper "ab", C.ord 'A')
  print (P.length [1, 2, 3 :: Int], 1 P.+ (2 :: Int), P.map (P.* 2) [1, 2 :: Int])
  print (P.Just 'x', P.show (P.Left 1 :: Either Int Bool))
  print (L.lookup 'b' (zip "ab" [1, 2 :: Int]), L.isPrefixOf "ab" "abc")
"#;

/// The Row 597/598 seam, a module's own top-level `reverse` beside `L.reverse`, and
/// Row 607's cell, `C.toUpper` beside a local `toUpper = succ`.
const SEAMS_CHIRHO: &str = r#"module Main where
import Prelude hiding (reverse)
import qualified Data.Char as C
import qualified Data.List as L
reverse :: [a] -> [a]
reverse = id
main :: IO ()
main = do
  print (L.reverse [1, 2, 3 :: Int], reverse [1, 2, 3 :: Int])
  print (map C.toUpper "ab", toUpper 'c')
  where
    toUpper = succ
"#;

/// Routes the table leaves alone, already exact: `M.fromMaybe` and `F.foldr` (their
/// GHC types disagree with the bare schemes), the keyed `Map.size`, and an unqualified `sort`.
const UNCHANGED_ROUTES_CHIRHO: &str = r#"module Main where
import qualified Data.Foldable as F
import qualified Data.Map as Map
import qualified Data.Maybe as M
import Data.List (sort)
main :: IO ()
main = do
  print (M.fromMaybe 0 (Just (1 :: Int)))
  print (F.foldr (+) 0 [1, 2, 3 :: Int])
  print (Map.size (Map.fromList [(1 :: Int, 'a')]))
  print (sort [3, 1, 2 :: Int])
"#;

/// Three agreeing Data.List aliases, then `L.intercalate`, whose String-only bare
/// scheme disagrees with GHC's and keeps the placeholder.
const LIST_PRINTS_CHIRHO: &str = r#"module Main where
import qualified Data.List as L
main :: IO ()
main = do
  print (L.reverse [1, 2, 3 :: Int])
  print (L.sort [3, 1, 2 :: Int])
  print (L.nub [1, 1, 2 :: Int])
  print (L.intercalate ", " ["a", "b"])
"#;

/// `O.comparing` now has its scheme, which reaches the missing runtime body.
const QUALIFIED_COMPARING_CHIRHO: &str = r#"module Main where
import qualified Data.Function as F
import qualified Data.Ord as O
import qualified Data.List as L
main :: IO ()
main = print (L.sortBy (O.comparing negate) [1, 3, 2 :: Int], (F.on (+) abs) (-1) (2 :: Int))
"#;

/// The same body through a plain import, failing the same way on the base.
const PLAIN_COMPARING_CHIRHO: &str = r#"module Main where
import Data.List (sortBy)
import Data.Ord (comparing)
main :: IO ()
main = print (sortBy (comparing negate) [1, 3, 2 :: Int])
"#;

/// A sibling user module, and an alias of it that GHC 9.14.1 accepts and runs (`10`).
const USER_LIB_CHIRHO: &str = r#"module LibChirho (twiceChirho) where
twiceChirho :: Int -> Int
twiceChirho = (* 2)
"#;
const USER_ALIAS_CHIRHO: &str = r#"module Main where
import qualified LibChirho as L
main :: IO ()
main = print (L.twiceChirho 5)
"#;

#[test]
fn agreeing_qualified_builtin_aliases_print_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "AgreeingAliasesChirho.hs",
        AGREEING_ALIASES_CHIRHO,
        "[3,2,1]\n([1,2,3],[1,2])\n('A',\"AB\",65)\n(3,3,[2,4])\n(Just 'x',\"Left 1\")\n(Just 2,True)\n",
    );
}

#[test]
fn the_namesake_seam_and_the_local_enum_cell_print_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "SeamsChirho.hs",
        SEAMS_CHIRHO,
        "([3,2,1],[1,2,3])\n(\"AB\",'d')\n",
    );
}

#[test]
fn routes_the_table_leaves_alone_hold_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "UnchangedRoutesChirho.hs",
        UNCHANGED_ROUTES_CHIRHO,
        "1\n6\n1\n[1,2,3]\n",
    );
}

#[test]
fn list_prints_stop_only_at_the_disagreeing_intercalate_chirho() {
    // The interpreter prints GHC's four lines. Natively, the three agreeing aliases now print, and
    // the named Show failure stays at `L.intercalate`, whose alias keeps the placeholder.
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        LIST_PRINTS_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "ListPrintsChirho.hs",
        None,
    )
    .expect("the interpreter runs it");
    assert_eq!(
        machine_chirho.io_output_chirho,
        "[3,2,1]\n[1,2,3]\n[1,2]\n\"a, b\"\n"
    );
    for backend_chirho in BACKENDS_CHIRHO {
        assert_eq!(
            native_round_trip_chirho(LIST_PRINTS_CHIRHO, backend_chirho, "")
                .expect_err("the intercalate line stays a named failure"),
            "native program emitted stderr (exit 1): haskelujah: error: show: no native Show instance for a boxed value\n\nstdout:\n[3,2,1]\n[1,2,3]\n[1,2]\n",
            "{backend_chirho:?}"
        );
    }
}

#[test]
fn a_qualified_comparing_fails_as_the_plain_import_does_chirho() {
    // `comparing` has a builtin scheme but no runtime body. Typed, the alias reaches that named
    // failure with nothing on stdout, exactly as the plain import does.
    let interpreted_chirho = |source_chirho: &str| {
        let error_chirho = eval_source_with_machine_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "ComparingChirho.hs",
            None,
        )
        .expect_err("no runtime body for comparing")
        .to_string();
        error_chirho
            .split(" for CoreId")
            .next()
            .unwrap_or_default()
            .to_string()
    };
    assert_eq!(
        interpreted_chirho(QUALIFIED_COMPARING_CHIRHO),
        interpreted_chirho(PLAIN_COMPARING_CHIRHO)
    );
    assert!(
        interpreted_chirho(QUALIFIED_COMPARING_CHIRHO).contains("missing STG binding `comparing`")
    );
    for backend_chirho in BACKENDS_CHIRHO {
        for source_chirho in [QUALIFIED_COMPARING_CHIRHO, PLAIN_COMPARING_CHIRHO] {
            assert_eq!(
                native_round_trip_chirho(source_chirho, backend_chirho, "")
                    .expect_err("no runtime body"),
                "native program emitted stderr (exit 1): haskelujah: error: missing native binding `comparing`\n\nstdout:\n",
                "{backend_chirho:?}"
            );
        }
    }
}

#[test]
fn a_user_module_alias_stays_off_the_builtin_table_chirho() {
    // The table holds builtin modules only, so a sibling module's alias compiles as before.
    let dir_chirho = tempfile::tempdir().expect("a scratch directory");
    std::fs::write(dir_chirho.path().join("LibChirho.hs"), USER_LIB_CHIRHO)
        .expect("the sibling module");
    compile_source_with_search_path_chirho(
        USER_ALIAS_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "Main.hs",
        dir_chirho.path(),
    )
    .expect("GHC accepts the user module's alias");
    assert!(
        qualified_builtin_alias_scheme_chirho(
            "LibChirho",
            "twiceChirho",
            &builtin_value_schemes_chirho()
        )
        .is_none()
    );
}
