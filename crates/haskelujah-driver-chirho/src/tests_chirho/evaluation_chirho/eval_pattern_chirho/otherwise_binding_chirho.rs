// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `otherwise` in a guard is whatever binding the name denotes there, never its
//! spelling. Each program was run on GHC 9.14.1 and its output is the
//! expectation (probe set O01-O14, reported by gpt_chirho and claude2_chirho on
//! 2026-10-17). The desugarer treated every variable spelled `otherwise` as
//! `True`: a parameter, `where`, `let`, pattern guard, alternative or top-level
//! definition that rebinds it took the wrong arm, and a shadow bound to bottom
//! was never evaluated.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::match_rows_chirho::{PRAGMAS_CHIRHO, fails_under_chirho, prints_under_chirho};

fn prints_chirho(body_chirho: &str) -> String {
    prints_under_chirho(PRAGMAS_CHIRHO, body_chirho)
}

fn fails_chirho(body_chirho: &str) -> String {
    fails_under_chirho(PRAGMAS_CHIRHO, body_chirho)
}

#[test]
fn a_parameter_named_otherwise_is_that_parameter_chirho() {
    // O01, and O10: a False parameter falls through to the next equation.
    assert_eq!(
        prints_chirho(
            "fChirho :: Bool -> Int -> Int\nfChirho otherwise x\n  | otherwise = 1\n  | x > 0 = 2\n  | True = 3\nmain :: IO ()\nmain = print (fChirho False 5, fChirho True 5, fChirho False 0)"
        ),
        "(2,1,3)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Bool -> Int -> Int\nfChirho otherwise x\n  | otherwise = 1\nfChirho _ x = x\nmain :: IO ()\nmain = print (fChirho False 7, fChirho True 7)"
        ),
        "(7,1)\n"
    );
}

#[test]
fn a_local_otherwise_is_the_local_binding_chirho() {
    // O03 `where`, O12 a nested `where`, O04 a `let` qualifier, O14 a do-let
    // function's parameter, O02 a `let` around a multi-way if.
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | otherwise = 1\n  | True = x\n  where otherwise = x > 10\nmain :: IO ()\nmain = print (fChirho 5, fChirho 20)"
        ),
        "(5,1)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x = go x\n  where\n    go n\n      | otherwise = n\n      | True = 0\n      where otherwise = n > 3\nmain :: IO ()\nmain = print (fChirho 2, fChirho 9)"
        ),
        "(0,9)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> Int\nfChirho x\n  | let otherwise = x > 10, otherwise = 1\n  | True = x\nmain :: IO ()\nmain = print (fChirho 5, fChirho 20)"
        ),
        "(5,1)\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = do\n  let fChirho otherwise\n        | otherwise = 1\n        | True = (2 :: Int)\n  print (fChirho False, fChirho True)"
        ),
        "(2,1)\n"
    );
    assert_eq!(
        prints_chirho(
            "main :: IO ()\nmain = print (let otherwise = False in if | otherwise -> (1 :: Int) | True -> 2)"
        ),
        "2\n"
    );
}

#[test]
fn a_pattern_guard_or_an_alternative_may_bind_otherwise_chirho() {
    // O05 a pattern guard's binder, O06 a case alternative, O07 a \case one.
    assert_eq!(
        prints_chirho(
            "fChirho :: Maybe Bool -> Int\nfChirho m\n  | Just otherwise <- m, otherwise = 1\n  | True = 0\nmain :: IO ()\nmain = print (map fChirho [Just True, Just False, Nothing])"
        ),
        "[1,0,0]\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Bool -> Int\nfChirho otherwise = case () of\n  _ | otherwise -> 1\n    | True -> 0\nmain :: IO ()\nmain = print (fChirho True, fChirho False)"
        ),
        "(1,0)\n"
    );
    assert_eq!(
        prints_chirho(
            "fChirho :: Bool -> Int\nfChirho = \\case\n  otherwise | otherwise -> 1\n            | True -> 0\nmain :: IO ()\nmain = print (fChirho True, fChirho False)"
        ),
        "(1,0)\n"
    );
}

#[test]
fn a_top_level_otherwise_replaces_the_preludes_chirho() {
    // O11.
    assert_eq!(
        prints_chirho(
            "import Prelude hiding (otherwise)\notherwise :: Bool\notherwise = False\nfChirho :: Int -> Int\nfChirho x\n  | otherwise = 1\n  | True = x\nmain :: IO ()\nmain = print (fChirho 5)"
        ),
        "5\n"
    );
}

#[test]
fn a_shadowing_otherwise_is_evaluated_chirho() {
    // O08: GHC fails with the shadow's own error.
    let failure_chirho = fails_chirho(
        "main :: IO ()\nmain = print (let otherwise = error \"shadow-bottom-chirho\" in if | otherwise -> (1 :: Int) | True -> 2)",
    );
    assert!(
        failure_chirho.contains("shadow-bottom-chirho"),
        "{failure_chirho}"
    );
    // O13: a False shadow leaves no arm, and GHC fails as non-exhaustive.
    let failure_chirho = fails_chirho(
        "fChirho :: Int -> Int\nfChirho x\n  | otherwise = 1\n  where otherwise = False\nmain :: IO ()\nmain = print (fChirho 1)",
    );
    assert!(
        failure_chirho.contains("Non-exhaustive"),
        "{failure_chirho}"
    );
}

#[test]
fn the_preludes_otherwise_still_takes_its_arm_chirho() {
    // O09: unqualified in an equation, in a multi-way if, and qualified.
    assert_eq!(
        prints_chirho(
            "fChirho :: Int -> String\nfChirho x\n  | x > 0 = \"pos\"\n  | otherwise = \"non-pos\"\ngChirho :: Int -> Int\ngChirho x = if | x > 0 -> 1 | otherwise -> 2\nhChirho :: Int -> Int\nhChirho x\n  | x > 0 = 1\n  | Prelude.otherwise = 2\nmain :: IO ()\nmain = print (map fChirho [1, 0], map gChirho [1, 0], map hChirho [1, 0])"
        ),
        "([\"pos\",\"non-pos\"],[1,2],[1,2])\n"
    );
}
