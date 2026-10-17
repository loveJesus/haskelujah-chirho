// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A pattern guard's GADT refinement reaches its own arm's result, and only
//! that arm's. Each program was run on GHC 9.14.1 (probe set R01-R13, with
//! gpt_chirho's and claude2_chirho's controls of 2026-10-17). The guard stack
//! decided an arm's result after the arm's scope had closed, so the refinement
//! was gone and valid programs were rejected against their signatures; every
//! arm is now judged inside its own scope.
//! workflow: language-features-chirho/pattern-matching-chirho

use super::match_rows_chirho::{fails_under_chirho, prints_under_chirho};

const PRAGMAS_CHIRHO: &str = "GADTs, LambdaCase, MultiWayIf";

const TYPES_CHIRHO: &str = "data EqualChirho a b where\n  ReflChirho :: EqualChirho a a\ndata ExprChirho a where\n  IntEChirho :: Int -> ExprChirho Int\n  BoolEChirho :: Bool -> ExprChirho Bool\n";

fn prints_chirho(body_chirho: &str) -> String {
    prints_under_chirho(PRAGMAS_CHIRHO, &format!("{TYPES_CHIRHO}{body_chirho}"))
}

fn fails_chirho(body_chirho: &str) -> String {
    fails_under_chirho(PRAGMAS_CHIRHO, &format!("{TYPES_CHIRHO}{body_chirho}"))
}

#[test]
fn a_refined_arm_meets_its_equations_signature_chirho() {
    // R01 (gpt_chirho's control), R02 (the refinement meets a rigid result),
    // R03 (two arms, each under its own refinement).
    assert_eq!(
        prints_chirho(
            "goodChirho :: EqualChirho a Bool -> a -> Bool\ngoodChirho eq value | ReflChirho <- eq = value\nmain :: IO ()\nmain = print (goodChirho ReflChirho True)"
        ),
        "True\n"
    );
    assert_eq!(
        prints_chirho(
            "castChirho :: EqualChirho a Bool -> a -> a\ncastChirho eq value | ReflChirho <- eq = not value\ncastChirho _ value = value\nmain :: IO ()\nmain = print (castChirho ReflChirho True)"
        ),
        "False\n"
    );
    assert_eq!(
        prints_chirho(
            "evalChirho :: ExprChirho a -> a\nevalChirho e\n  | IntEChirho n <- e = n + 1\n  | BoolEChirho b <- e = not b\nmain :: IO ()\nmain = print (evalChirho (IntEChirho 41), evalChirho (BoolEChirho True))"
        ),
        "(42,False)\n"
    );
}

#[test]
fn a_refined_arm_in_an_alternative_or_a_where_function_chirho() {
    // R04 a case alternative's guard, R10 a `where` function with its own
    // signature, R06 a \case alternative, R05 a multi-way if.
    assert_eq!(
        prints_chirho(
            "goodChirho :: EqualChirho a Bool -> a -> Bool\ngoodChirho eq value = case () of\n  _ | ReflChirho <- eq -> value\nmain :: IO ()\nmain = print (goodChirho ReflChirho False)"
        ),
        "False\n"
    );
    assert_eq!(
        prints_chirho(
            "outerChirho :: EqualChirho a Bool -> a -> Bool\nouterChirho eq value = innerChirho eq value\n  where\n    innerChirho :: EqualChirho b Bool -> b -> Bool\n    innerChirho e v | ReflChirho <- e = v\nmain :: IO ()\nmain = print (outerChirho ReflChirho True)"
        ),
        "True\n"
    );
    assert_eq!(
        prints_chirho(
            "evalChirho :: ExprChirho a -> a\nevalChirho = \\case\n  e | IntEChirho n <- e -> n * 2\n    | BoolEChirho b <- e -> b\nmain :: IO ()\nmain = print (evalChirho (IntEChirho 21), evalChirho (BoolEChirho False))"
        ),
        "(42,False)\n"
    );
    assert_eq!(
        prints_chirho(
            "goodChirho :: EqualChirho a Bool -> a -> Bool\ngoodChirho eq value = if | ReflChirho <- eq -> value | otherwise -> False\nmain :: IO ()\nmain = print (goodChirho ReflChirho True)"
        ),
        "True\n"
    );
}

#[test]
fn a_refinement_reaches_no_other_arm_or_equation_chirho() {
    // R07: the next equation does not see the refinement.
    let failure_chirho = fails_chirho(
        "badChirho :: EqualChirho a Bool -> a -> a\nbadChirho eq value | ReflChirho <- eq = True\nbadChirho _ value = True\nmain :: IO ()\nmain = print (0 :: Int)",
    );
    assert!(failure_chirho.contains("type mismatch"), "{failure_chirho}");
    // R11: nor does the next arm. Main accepted this.
    let failure_chirho = fails_chirho(
        "badChirho :: EqualChirho a Bool -> a -> Bool\nbadChirho eq value\n  | ReflChirho <- eq, False = value\n  | otherwise = value\nmain :: IO ()\nmain = print (0 :: Int)",
    );
    assert!(failure_chirho.contains("type mismatch"), "{failure_chirho}");
    // R09: an arm without the refinement needs `Num a` for its literal.
    let failure_chirho = fails_chirho(
        "evalChirho :: ExprChirho a -> a\nevalChirho e\n  | IntEChirho n <- e, n > 0 = n\n  | otherwise = 0\nmain :: IO ()\nmain = print (evalChirho (IntEChirho 5))",
    );
    assert!(failure_chirho.contains("Num a"), "{failure_chirho}");
}

#[test]
fn an_arm_that_cannot_meet_its_signature_is_reported_chirho() {
    // R08: in the refined type, as GHC states it (`Bool` against `Int`).
    let failure_chirho = fails_chirho(
        "badChirho :: EqualChirho a Bool -> a -> Int\nbadChirho eq value | ReflChirho <- eq = value\nmain :: IO ()\nmain = print (0 :: Int)",
    );
    assert!(failure_chirho.contains("found `Bool`"), "{failure_chirho}");
    // R13 a first arm and R12 a later one, with no GADT at all. Main
    // accepted R12: a later arm was compared only with the first, leniently.
    let failure_chirho = fails_chirho(
        "fChirho :: Int -> Bool\nfChirho x\n  | x > 0 = (1 :: Int)\n  | otherwise = True\nmain :: IO ()\nmain = print (fChirho 1)",
    );
    assert!(failure_chirho.contains("type mismatch"), "{failure_chirho}");
    let failure_chirho = fails_chirho(
        "fChirho :: Int -> Bool\nfChirho x\n  | x > 0 = True\n  | otherwise = (1 :: Int)\nmain :: IO ()\nmain = print (fChirho 1)",
    );
    assert!(failure_chirho.contains("type mismatch"), "{failure_chirho}");
}
