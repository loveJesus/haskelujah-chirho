// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A qualified reference names an import or the module itself, never a local binder (row 598).
//!
//! `L.reverse` resolved by its bare text, so a local `reverse` captured it on every engine,
//! and the checker could type it as the local. A qualifier no import binds checked and ran
//! where GHC 9.14.1 reports GHC-76037. Every program and expectation here is GHC 9.14.1's,
//! embedded byte for byte from the measured file.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

/// Every local binder form beside a qualified namesake: a `where` under a qualified-only
/// import, a `let`, a function and a lambda parameter, and a `case` binder. Each local
/// captured the qualified reference on every engine.
const LOCAL_BINDERS_CHIRHO: &str = r#"module Main where
import Data.List as L
import qualified Data.List as Q
sortedChirho :: [Int] -> [Int]
sortedChirho xs = Q.sort xs
  where
    sort = reverse
parameterChirho :: ([Int] -> [Int]) -> ([Int], [Int])
parameterChirho reverse = (L.reverse [1, 2, 3], reverse [1, 2, 3])
main :: IO ()
main = do
  print (sortedChirho [2, 1, 3])
  print (let reverse = id in (L.reverse [1, 2, 3 :: Int], reverse [1, 2, 3 :: Int]))
  print (parameterChirho id)
  print ((\reverse -> (L.reverse [1, 2, 3 :: Int], reverse [1, 2, 3 :: Int])) id)
  print (case id of reverse -> (L.reverse [1, 2, 3 :: Int], reverse [1, 2, 3 :: Int]))
"#;

/// The module's own name beside a local `fooChirho`, the implicit Prelude's beside a local
/// `length`, and a recursive own-qualified call. The first two took the local.
const OWN_AND_PRELUDE_QUALIFIERS_CHIRHO: &str = r#"module Main where
fooChirho :: Int
fooChirho = 1
countChirho :: Int -> Int
countChirho 0 = 0
countChirho n = Main.countChirho (n - 1) + 1
main :: IO ()
main = do
  print (let fooChirho = 2 :: Int in (Main.fooChirho, fooChirho))
  print (let length = const (0 :: Int) in (Prelude.length [1, 2, 3 :: Int], length "abc"))
  print (countChirho 3)
"#;

/// A headerless module is `Main`, and an import's `L.reverse` sits beside the module's own
/// top-level `reverse`, which row 597 spells apart for the natives.
const HEADERLESS_NAMESAKES_CHIRHO: &str = r#"import Prelude hiding (reverse)
import qualified Data.List as L
fooChirho :: Int
fooChirho = 1
reverse :: [a] -> [a]
reverse = id
main :: IO ()
main = do
  print (let fooChirho = 2 :: Int in (Main.fooChirho, fooChirho))
  print (L.reverse [1, 2, 3 :: Int] == [3, 2, 1], reverse [1, 2, 3 :: Int] == [1, 2, 3])
"#;

/// Qualified uses that already answered: `C.toUpper`, `L.sort` under a signature, `L.reverse`
/// compared, and a seeded Data.Map alias beside a local `size`.
const QUALIFIED_HOLDS_CHIRHO: &str = r#"module Main where
import qualified Data.Char as C
import qualified Data.List as L
import qualified Data.Map as M
sortedChirho :: [Int] -> [Int]
sortedChirho = L.sort
main :: IO ()
main = do
  putStrLn (map C.toUpper "ab")
  print (sortedChirho [3, 1, 2])
  print (L.reverse [1, 2, 3 :: Int] == [3, 2, 1])
  print (let size = const (0 :: Int) in (M.size (M.fromList [(1 :: Int, 'a')]), size "x"))
"#;

#[test]
fn a_qualified_reference_skips_every_local_binder_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "LocalBindersChirho.hs",
        LOCAL_BINDERS_CHIRHO,
        "[1,2,3]\n([3,2,1],[1,2,3])\n([3,2,1],[1,2,3])\n([3,2,1],[1,2,3])\n([3,2,1],[1,2,3])\n",
    );
}

#[test]
fn a_module_qualifies_its_own_top_level_and_its_prelude_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "OwnAndPreludeQualifiersChirho.hs",
        OWN_AND_PRELUDE_QUALIFIERS_CHIRHO,
        "(1,2)\n(3,0)\n3\n",
    );
    assert_prints_everywhere_chirho(
        "HeaderlessNamesakesChirho.hs",
        HEADERLESS_NAMESAKES_CHIRHO,
        "(1,2)\n(True,True)\n",
    );
}

#[test]
fn qualified_uses_that_already_answered_still_do_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "QualifiedHoldsChirho.hs",
        QUALIFIED_HOLDS_CHIRHO,
        "AB\n[1,2,3]\nTrue\n(1,0)\n",
    );
}

#[test]
fn an_unimported_qualifier_is_out_of_scope_chirho() {
    // GHC 9.14.1: GHC-76037 for each. No import binds `X`; an alias replaces its module's
    // name; and an explicit `import qualified Prelude as P` leaves no implicit `Prelude`.
    for (source_chirho, file_name_chirho, reason_chirho) in [
        (
            "module Main where\nmain :: IO ()\nmain = print (X.reverse [1, 2, 3 :: Int])\n",
            "U01UnknownQualifierChirho.hs",
            "variable not in scope: `X.reverse`",
        ),
        (
            "module Main where\nmain :: IO ()\nmain = print (Data.List.reverse [1, 2, 3 :: Int])\n",
            "U02ModuleNotImportedChirho.hs",
            "variable not in scope: `Data.List.reverse`",
        ),
        (
            "module Main where\nimport Data.List as L\nmain :: IO ()\nmain = print (Data.List.reverse [1, 2, 3 :: Int])\n",
            "U03ModuleNameAfterAliasChirho.hs",
            "variable not in scope: `Data.List.reverse`",
        ),
        (
            "module Main where\nmain :: IO ()\nmain = let reverse = id in print (X.reverse [1, 2, 3 :: Int])\n",
            "U04UnknownQualifierLocalChirho.hs",
            "variable not in scope: `X.reverse`",
        ),
        (
            "module Main where\nmain :: IO ()\nmain = print (1 X.+ (2 :: Int))\n",
            "U05UnknownQualifiedOperatorChirho.hs",
            "variable not in scope: `X.+`",
        ),
        (
            "module Main where\nmain :: IO ()\nmain = print (X.Just (1 :: Int))\n",
            "U06UnknownQualifiedConstructorChirho.hs",
            "data constructor not in scope: `X.Just`",
        ),
        (
            "{-# LANGUAGE NoImplicitPrelude #-}\nmodule Main where\nimport qualified Prelude as P\nmain :: P.IO ()\nmain = Prelude.print (1 :: P.Int)\n",
            "U07AliasReplacesPreludeChirho.hs",
            "variable not in scope: `Prelude.print`",
        ),
    ] {
        rejected_like_ghc_chirho(source_chirho, file_name_chirho, &[reason_chirho]);
    }
}

#[test]
fn a_qualified_name_its_import_lacks_never_falls_back_to_a_local_chirho() {
    // GHC 9.14.1: GHC-76037. `Data.List` exports no `nonexistentChirho`, and the checker's bare
    // fallback read the `let`-bound one before it kept to the module level.
    rejected_like_ghc_chirho(
        "module Main where\nimport qualified Data.List as L\nmain :: IO ()\nmain = let nonexistentChirho = id in print (L.nonexistentChirho [1, 2, 3 :: Int])\n",
        "U08AliasLacksNameLocalChirho.hs",
        &["unbound variable: `nonexistentChirho`"],
    );
}
