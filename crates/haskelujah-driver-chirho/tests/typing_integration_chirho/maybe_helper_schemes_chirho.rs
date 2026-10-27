// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Data.Maybe helpers at GHC's types (row 611).
//!
//! `fromMaybe`, `isJust`, `isNothing` and `maybe` took any constructor `f a` (row 429's shortcut
//! for programs declaring their own `Maybe`). So a list, an `Either`, a user `Box` or a rigid
//! variable checked, and then printed a wrong value natively or failed at run time. Each now names
//! `Maybe`, as GHC 9.14.1 does: the misuses are frontend rejections on every engine, and the
//! valid uses keep GHC's answers. Every program is embedded byte for byte from the measured file.
//! workflow: compiler-pipeline-chirho/builtin-type-environment-chirho

use super::{assert_prints_everywhere_chirho, rejected_like_ghc_chirho};

/// `fromMaybe`, `isJust`, `isNothing` and `maybe` at their GHC types: Just and Nothing, a lookup
/// result, partial and first-class uses, distinct input and result types, and Fractional
/// defaulting through `fromMaybe` and `maybe`.
const VALID_USES_CHIRHO: &str = r#"module Main where
import Data.Maybe (fromMaybe, isJust, isNothing)
main :: IO ()
main = do
  print (fromMaybe 0 (Just 3 :: Maybe Int))
  print (fromMaybe 0 (Nothing :: Maybe Int))
  print (map (fromMaybe 'x') [Just 'a', Nothing])
  print (fromMaybe "none" (lookup 2 [(1, "one"), (2, "two")]))
  print (isJust (lookup 1 [(1 :: Int, 'a')]), isNothing (lookup 3 [(1 :: Int, 'a')]), filter isJust [Just 1, Nothing, Just (3 :: Int)])
  print (maybe "none" show (Just (3 :: Int)))
  print (map (maybe 0 (+ 1)) [Just 1, Nothing :: Maybe Int])
  print (fromMaybe 0 (Just 1.5))
  print (maybe 'z' succ (Just 'a'))
  let f = fromMaybe (0 :: Int)
      g = maybe 0 (* 2)
  print (map f [Just 1, Nothing], g (Just (3 :: Int)), any isJust [Nothing, Just 'q'])
"#;

/// A program-owned top-level `fromMaybe` over lists, and a let-bound `isJust`, both answering for
/// themselves.
const PROGRAM_SHADOWS_CHIRHO: &str = r#"module Main where
import Data.Maybe hiding (fromMaybe)
fromMaybe :: a -> [a] -> a
fromMaybe d [] = d
fromMaybe _ (x : _) = x
main :: IO ()
main = print (fromMaybe 0 [5 :: Int], let isJust _ = False in isJust (Just 'x'))
"#;

/// `import qualified Data.Maybe as M`: the four aliases now agree with GHC, so row 608's table
/// types them instead of the placeholder.
const QUALIFIED_ALIASES_CHIRHO: &str = r#"module Main where
import qualified Data.Maybe as M
main :: IO ()
main = print (M.fromMaybe 0 (Just 1 :: Maybe Int), M.isJust (Just 'c'), M.isNothing (Nothing :: Maybe Int), M.maybe 0 (+ 1) (Just 2 :: Maybe Int))
"#;

/// GHC 9.14.1 rejects each of these with GHC-83865 or GHC-25897: the argument is not a `Maybe`.
const MISUSES_CHIRHO: [(&str, &str, &str); 7] = [
    (
        r#"module Main where
import Data.Maybe (fromMaybe)
main :: IO ()
main = print (fromMaybe 0 [1 :: Int])
"#,
        "FromMaybeListChirho.hs",
        "found `[]`",
    ),
    (
        r#"module Main where
import Data.Maybe (fromMaybe)
main :: IO ()
main = print (fromMaybe 0 (Right 1 :: Either String Int))
"#,
        "FromMaybeEitherChirho.hs",
        "found `(Either [Char])`",
    ),
    (
        r#"module Main where
import Data.Maybe (isJust)
main :: IO ()
main = print (isJust [1 :: Int])
"#,
        "IsJustListChirho.hs",
        "found `[]`",
    ),
    (
        r#"module Main where
import Data.Maybe (isNothing)
data Box a = Box a
main :: IO ()
main = print (isNothing (Box 'x'))
"#,
        "IsNothingBoxChirho.hs",
        "found `Box`",
    ),
    (
        r#"module Main where
main :: IO ()
main = print (maybe 0 (+ 1) (Right 2 :: Either () Int))
"#,
        "MaybeEitherChirho.hs",
        "found `(Either ())`",
    ),
    (
        r#"module Main where
main :: IO ()
main = print (maybe 0 (+ 1) [2 :: Int])
"#,
        "MaybeListChirho.hs",
        "found `[]`",
    ),
    (
        r#"module Main where
import Data.Maybe (isJust)
probeChirho :: g Int -> Bool
probeChirho = isJust
main :: IO ()
main = print (probeChirho (Just 1))
"#,
        "RigidConstructorChirho.hs",
        "found `g`",
    ),
];

/// The shape rows 429's tests once pinned: a module's own `Maybe` handed to the builtin helper.
/// GHC 9.14.1 rejects it (GHC-87543, an ambiguous `Just`).
const LOCAL_MAYBE_CHIRHO: &str = r#"module Test where
data Maybe a = Nothing | Just a
main = if isJust (Just 42) then 1 else 0
"#;

#[test]
fn maybe_helpers_answer_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "ValidUsesChirho.hs",
        VALID_USES_CHIRHO,
        "3\n0\n\"ax\"\n\"two\"\n(True,True,[Just 1,Just 3])\n\"3\"\n[2,0]\n1.5\n'b'\n([1,0],6,True)\n",
    );
    assert_prints_everywhere_chirho(
        "ProgramShadowsChirho.hs",
        PROGRAM_SHADOWS_CHIRHO,
        "(5,False)\n",
    );
}

#[test]
fn qualified_data_maybe_aliases_answer_as_ghc_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "QualifiedAliasesChirho.hs",
        QUALIFIED_ALIASES_CHIRHO,
        "(1,True,True,3)\n",
    );
}

#[test]
fn maybe_helpers_reject_every_other_constructor_chirho() {
    for (source_chirho, file_name_chirho, found_chirho) in MISUSES_CHIRHO {
        rejected_like_ghc_chirho(
            source_chirho,
            file_name_chirho,
            &["type mismatch: expected `Maybe`", found_chirho],
        );
    }
}

#[test]
fn a_local_maybe_is_not_the_builtin_maybe_chirho() {
    rejected_like_ghc_chirho(
        LOCAL_MAYBE_CHIRHO,
        "LocalMaybeChirho.hs",
        &["type mismatch: expected `Prelude.Maybe`, found `Maybe`"],
    );
}
