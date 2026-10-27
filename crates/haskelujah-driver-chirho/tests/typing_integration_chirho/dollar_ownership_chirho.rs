// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! An infix `$` stays an operator through lowering (row 619). Before this, fixity resolution turned
//! every infix `$` into an application by its spelling, so a program's own or local `$` was
//! silently the Prelude's on every engine. Now the desugarer gives an owned `$` its own binding
//! (row 614's ownership check) and makes the Prelude's `$` an application, and the type checker
//! types the Prelude's `$` as before: `runST $ do ...` still type-checks. Every program here is
//! GHC 9.14.1's, byte for byte as measured.
//! workflow: compiler-pipeline-chirho/operator-fixity-chirho

use super::{assert_compile_success_chirho, assert_prints_everywhere_chirho};

/// A program's own `$`, signed and unsigned, a `where`-bound one, a declared `infixr 0` chain,
/// and one applied to a `do` block.
const OWNED_DOLLARS_CHIRHO: [(&str, &str, &str); 5] = [
    (
        "DHiddenDollarSigChirho.hs",
        "module Main where\nimport Prelude hiding (($))\n($) :: (Int -> Int) -> Int -> Int\nf $ x = f (f x)\nmain :: IO ()\nmain = print ((+ 1) $ 5)\n",
        "7\n",
    ),
    (
        "DHiddenDollarUnsignedChirho.hs",
        "module Main where\nimport Prelude hiding (($))\nf $ x = f (f x)\nmain :: IO ()\nmain = print ((+ 1) $ (5 :: Int))\n",
        "7\n",
    ),
    (
        "DWhereDollarChirho.hs",
        "module Main where\nmain :: IO ()\nmain = print ((+ 1) $ (5 :: Int))\n  where\n    f $ x = f (f x)\n",
        "7\n",
    ),
    (
        "DDollarChainChirho.hs",
        "module Main where\nimport Prelude hiding (($))\ninfixr 0 $\n($) :: (Int -> Int) -> Int -> Int\nf $ x = f x * 10\nmain :: IO ()\nmain = print ((+ 1) $ (* 2) $ 3)\n",
        "610\n",
    ),
    (
        "DDollarDoChirho.hs",
        "module Main where\nimport Prelude hiding (($))\n($) :: (IO () -> IO ()) -> IO () -> IO ()\nf $ x = f x >> f x\nmain :: IO ()\nmain = id $ do\n  putStrLn \"once\"\n",
        "once\nonce\n",
    ),
];

/// What held: the Prelude's `$` in chains and before `map`, and sections of an owned `$`.
const DOLLAR_HOLDS_CHIRHO: [(&str, &str, &str); 2] = [
    (
        "HPreludeDollarChirho.hs",
        "module Main where\nmain :: IO ()\nmain = do\n  print $ (+ 1) $ (* 2) $ 3\n  mapM_ print $ map (subtract 1) [1, 2 :: Int]\n",
        "7\n0\n1\n",
    ),
    (
        "DDollarSectionsChirho.hs",
        "module Main where\nimport Prelude hiding (($))\n($) :: (Int -> Int) -> Int -> Int\nf $ x = f (f x)\nmain :: IO ()\nmain = print (map ((+ 1) $) [5], map ($ 5) [(* 2)])\n",
        "([7],[20])\n",
    ),
];

#[test]
fn an_owned_or_local_dollar_keeps_its_own_binding_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in OWNED_DOLLARS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

#[test]
fn the_prelude_dollar_and_owned_sections_hold_everywhere_chirho() {
    for (file_name_chirho, source_chirho, ghc_chirho) in DOLLAR_HOLDS_CHIRHO {
        assert_prints_everywhere_chirho(file_name_chirho, source_chirho, ghc_chirho);
    }
}

/// `runST $ do ...` keeps type-checking with `$` kept as an operator: the Prelude's `$` is typed as
/// before. (Running `runST` is a separate gap on main; GHC prints 55.)
#[test]
fn run_st_dollar_do_still_type_checks_chirho() {
    assert_compile_success_chirho(
        "HRunSTDoChirho.hs",
        "module Main where\nimport Control.Monad.ST\nimport Data.STRef\nsumST :: [Int] -> Int\nsumST xs = runST $ do\n  ref <- newSTRef 0\n  mapM_ (\\x -> modifySTRef ref (+ x)) xs\n  readSTRef ref\nmain :: IO ()\nmain = print (sumST [1 .. 10])\n",
    );
}
