// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordering operators through a body-backed `compare` (row 551, gpt_chirho #28919). The
//! desugarer turned infix and section `<`, `<=`, `>` and `>=` into Int primops, which met
//! Strings, lists, derived values and Orderings as heap values or C strings: the
//! interpreter refused them (`LtIntChirho: expected Int#`). Where the operand's key has a
//! body-backed `$prim_Ord_compare_<key>` row, the pass now reads one `compare a b`. Keys
//! without such a row (`Maybe`, tuples) and prefix `(<)` stay as they were; `max` and
//! `min` read the same rows since row 565.
//! Each program's GHC 9.14.1 output is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_core_chirho::pretty_module_chirho;
use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::{assert_native_chirho, run_chirho, run_result_chirho};

/// String, `[Int]` (a prefix included), a derived enum and record, and Ordering through
/// all four operators and both sections; Int, Char, Bool and Double keep their primops.
const COMPARE_ROUTE_CHIRHO: &str = "module Main where\ndata ColorChirho = RedChirho | GreenChirho deriving (Eq, Ord, Show)\ndata PairChirho = PairChirho Int Int deriving (Eq, Ord, Show)\nmain :: IO ()\nmain = do\n  print (\"b\" < \"a\")\n  print (\"b\" <= \"a\")\n  print (\"b\" > \"a\")\n  print (\"b\" >= \"a\")\n  print (\"a\" <= \"a\")\n  print (\"a\" < \"a\")\n  print ([1, 3 :: Int] < [1, 2])\n  print ([1, 2 :: Int] <= [1, 2])\n  print ([1, 2 :: Int] > [1, 2, 0])\n  print ([2 :: Int] > [1, 9])\n  print (GreenChirho > RedChirho)\n  print (RedChirho >= GreenChirho)\n  print (PairChirho 1 3 < PairChirho 1 2)\n  print (PairChirho 1 2 <= PairChirho 1 2)\n  print (GT > LT)\n  print (EQ <= LT)\n  print (LT < EQ)\n  print ((< \"m\") \"b\")\n  print ((\"m\" >=) \"b\")\n  print ((3 :: Int) < 4)\n  print ('a' > 'b')\n  print (True > False)\n  print ((-1.5 :: Double) < (-2.5))\n";

/// GHC 9.14.1's output for [`COMPARE_ROUTE_CHIRHO`].
const COMPARE_ROUTE_GHC_CHIRHO: &str = "False\nFalse\nTrue\nTrue\nTrue\nFalse\nFalse\nTrue\nFalse\nTrue\nTrue\nFalse\nFalse\nTrue\nTrue\nFalse\nTrue\nTrue\nTrue\nTrue\nFalse\nTrue\nFalse\n";

#[test]
fn ordering_operators_answer_as_ghc_through_compare_everywhere_chirho() {
    assert_eq!(
        run_chirho("CompareRouteChirho.hs", COMPARE_ROUTE_CHIRHO),
        COMPARE_ROUTE_GHC_CHIRHO,
        "interpreter"
    );
    assert_native_chirho(
        "CompareRouteChirho.hs",
        COMPARE_ROUTE_CHIRHO,
        COMPARE_ROUTE_GHC_CHIRHO,
    );
}

/// The interpreter still fails on keys without a body-backed compare row and on prefix
/// `(<)`; GHC answers False, False and False. Each is a later Ord brick. (`max` moved with
/// row 565; see `extrema_chirho.rs`.)
#[test]
fn orderings_outside_the_compare_route_still_fail_in_the_interpreter_chirho() {
    for (file_name_chirho, program_chirho) in [
        (
            "MaybeLessChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (Just (2 :: Int) < Nothing)\n",
        ),
        (
            "TupleLessChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ((1 :: Int, 3 :: Int) < (1, 2))\n",
        ),
        (
            "PrefixLessChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print ((<) \"b\" \"a\")\n",
        ),
    ] {
        assert!(
            run_result_chirho(file_name_chirho, program_chirho).is_err(),
            "{file_name_chirho}: moved outside row 551's route"
        );
    }
}

/// The routed `>=` reads one `compare "b" "a"`: main mentions the `[Char]` compare row once
/// and no Int ordering primop.
#[test]
fn a_routed_ordering_evaluates_compare_once_chirho() {
    let result_chirho = compile_source_chirho(
        "module Main where\nmain :: IO ()\nmain = print (\"b\" >= \"a\")\n",
        &mut SourceMapChirho::new_chirho(),
        "CompareOnceChirho.hs",
    )
    .unwrap_or_else(|_| panic!("GHC compiles this program"));
    let compare_id_chirho = result_chirho
        .core_chirho
        .bindings_chirho
        .iter()
        .find(|binding_chirho| {
            binding_chirho.binder_chirho.name_chirho == "$prim_Ord_compare_[Char]"
        })
        .map(|binding_chirho| binding_chirho.binder_chirho.id_chirho)
        .expect("the [Char] compare row");
    let printed_chirho = pretty_module_chirho(&result_chirho.core_chirho);
    let main_chirho = printed_chirho
        .lines()
        .find(|line_chirho| line_chirho.starts_with("main = "))
        .expect("main's binding");
    let token_chirho = format!("v{}", compare_id_chirho.0);
    let uses_chirho = main_chirho
        .match_indices(&token_chirho)
        .filter(|(at_chirho, _)| {
            !main_chirho[at_chirho + token_chirho.len()..]
                .starts_with(|c_chirho: char| c_chirho.is_ascii_digit())
        })
        .count();
    assert_eq!(uses_chirho, 1, "{main_chirho}");
    assert!(!main_chirho.contains(">=#"), "{main_chirho}");
}
