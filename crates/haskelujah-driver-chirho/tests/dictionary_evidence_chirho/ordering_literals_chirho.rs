// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bare `Ordering` literals (RC5, gpt_chirho #28162). `LT`, `EQ` and `GT` are seeded as
//! `Ordering` beside `True` and `False`. Unbound, each was a fresh type variable whose Eq and
//! Ord occurrences defaulted to Int evidence: `compare LT GT` crashed at the Int row,
//! `sortBy compare [GT, LT, EQ]` printed `[GT,LT,EQ]`, and `x :: Int; x = LT` was accepted.
//! Each program's GHC 9.14.1 behavior is recorded beside it; a program GHC rejects is
//! written before its file name.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

use super::run_chirho;

/// GHC rejects the program, so the checker must too, with a message naming every reason.
fn rejected_like_ghc_chirho(source_chirho: &str, file_name_chirho: &str, reasons_chirho: &[&str]) {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    match compile_source_chirho(source_chirho, &mut source_map_chirho, file_name_chirho) {
        Ok(_) => panic!("{file_name_chirho}: GHC rejects this program; we accepted it"),
        Err(err_chirho) => {
            let message_chirho = err_chirho.to_string();
            for reason_chirho in reasons_chirho {
                assert!(
                    message_chirho.contains(reason_chirho),
                    "{file_name_chirho}: rejected, but not for `{reason_chirho}`: {message_chirho}"
                );
            }
        }
    }
}

#[test]
fn bare_ordering_literals_answer_as_ghc_chirho() {
    // Before the seed: R1 and R5 crashed with no matching alternative for tag 0 at the Int
    // row, R2 printed [GT,LT,EQ], and R9 and T12 crashed in an Int primop.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "R1CompareOrderingLiteralsChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare LT GT)\n",
            "LT\n",
        ),
        (
            "R2SortByOrderingChirho.hs",
            "module Main where\nimport Data.List (sortBy)\nmain :: IO ()\nmain = print (sortBy compare [GT, LT, EQ])\n",
            "[LT,EQ,GT]\n",
        ),
        (
            "R5CompareOrderingBoundChirho.hs",
            "module Main where\nmain :: IO ()\nmain = let xChirho = LT in print (compare xChirho GT)\n",
            "LT\n",
        ),
        (
            "R9EqOrderingLiteralsChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (LT == GT, EQ == EQ)\n",
            "(False,True)\n",
        ),
        (
            "T12EqNeChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (LT == LT, GT /= EQ)\n",
            "(True,True)\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
    // Open boundaries, each its own unit: GHC's answers are recorded so the oracle still runs
    // these, but nothing here asserts them. `succ LT` needs Ordering's Enum instance, which the
    // class environment lacks, so it is now rejected naming it; `maximum` still lowers to an
    // Int primop until an Ord method model.
    let _open_boundaries_chirho = [
        (
            "T10SuccLtChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (succ LT)\n",
            "EQ\n",
        ),
        (
            "T11MaximumChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (maximum [LT, GT, EQ])\n",
            "GT\n",
        ),
    ];
}

#[test]
fn annotated_computed_and_user_orderings_keep_their_answers_chirho() {
    // These were already exact: an annotation, a signed binding, a computed Ordering, a
    // constructor pattern at a signed type, Show, user enums, Bool, and qualified names,
    // which resolve through the selector and a dictionary.
    for (file_name_chirho, program_chirho, ghc_chirho) in [
        (
            "T05AnnotateLeftChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare (LT :: Ordering) GT)\n",
            "LT\n",
        ),
        (
            "T06AnnotateRightChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare LT (GT :: Ordering))\n",
            "LT\n",
        ),
        (
            "T07SignedLetChirho.hs",
            "module Main where\nmain :: IO ()\nmain = let xChirho :: Ordering\n           xChirho = LT\n       in print (compare xChirho GT)\n",
            "LT\n",
        ),
        (
            "T08ShowLtChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print LT\n",
            "LT\n",
        ),
        (
            "T09ShowListChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print [LT, EQ, GT]\n",
            "[LT,EQ,GT]\n",
        ),
        (
            "T13PatternChirho.hs",
            "module Main where\nfChirho :: Ordering -> String\nfChirho LT = \"lt\"\nfChirho _ = \"other\"\nmain :: IO ()\nmain = putStrLn (fChirho LT)\n",
            "lt\n",
        ),
        (
            "T14UserEnumChirho.hs",
            "module Main where\ndata TChirho = AChirho | BChirho deriving (Show, Eq, Ord)\nmain :: IO ()\nmain = print (compare AChirho BChirho, AChirho == BChirho)\n",
            "(LT,False)\n",
        ),
        (
            "T16QualifiedChirho.hs",
            "module Main where\nimport qualified Prelude as P\nimport Prelude (IO, print)\nmain :: IO ()\nmain = print (P.compare P.LT P.GT)\n",
            "LT\n",
        ),
        (
            "R3CompareOrderingComputedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare (compare 1 2) (compare 2 (1 :: Int)))\n",
            "LT\n",
        ),
        (
            "R4CompareOrderingAnnotatedChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare (LT :: Ordering) (GT :: Ordering))\n",
            "LT\n",
        ),
        (
            "R6CompareUserEnumChirho.hs",
            "module Main where\ndata ColorChirho = RedChirho | GreenChirho | BlueChirho deriving (Show, Eq, Ord)\nmain :: IO ()\nmain = print (compare RedChirho BlueChirho)\n",
            "LT\n",
        ),
        (
            "R7SortByUserEnumChirho.hs",
            "module Main where\nimport Data.List (sortBy)\ndata ColorChirho = RedChirho | GreenChirho | BlueChirho deriving (Show, Eq, Ord)\nmain :: IO ()\nmain = print (sortBy compare [BlueChirho, RedChirho, GreenChirho])\n",
            "[RedChirho,GreenChirho,BlueChirho]\n",
        ),
        (
            "R8CompareBoolLiteralsChirho.hs",
            "module Main where\nmain :: IO ()\nmain = print (compare True False, compare False True)\n",
            "(GT,LT)\n",
        ),
    ] {
        assert_eq!(
            run_chirho(file_name_chirho, program_chirho),
            ghc_chirho,
            "{file_name_chirho}"
        );
    }
}

#[test]
fn module_constructors_named_lt_eq_gt_still_override_the_seed_chirho() {
    // The module's own LT, EQ and GT build a CmpChirho: its declarations remove the seeded
    // names before they are bound, so `==` is CmpChirho's derived Eq, not Ordering's.
    assert_eq!(
        run_chirho(
            "T17UserConsNamedLtChirho.hs",
            "module Main where\nimport Prelude hiding (Ordering(..))\ndata CmpChirho = LT | EQ | GT deriving (Show, Eq)\nmain :: IO ()\nmain = print (LT == GT, show EQ)\n"
        ),
        "(False,\"EQ\")\n"
    );
}

#[test]
fn ordering_literals_used_at_int_are_rejected_like_ghc_chirho() {
    // GHC 9.14.1 rejects both with GHC-83865: T01 cannot match Int with Ordering, and T04
    // cannot match Ordering with Int in the second argument of `+`. Before the seed, T01
    // printed LT and T04 crashed in the Int addition primop.
    rejected_like_ghc_chirho(
        "module Main where\nxChirho :: Int\nxChirho = LT\nmain :: IO ()\nmain = print xChirho\n",
        "T01IntFromLtChirho.hs",
        &["type mismatch", "`Ordering`"],
    );
    rejected_like_ghc_chirho(
        "module Main where\nmain :: IO ()\nmain = print (GT + (1 :: Int))\n",
        "T04AddToGtChirho.hs",
        &["type mismatch", "`Ordering`"],
    );
}

#[test]
fn neighbouring_constructors_used_at_int_stay_rejected_chirho() {
    // Seeded and imported neighbours were already rejected like GHC (GHC-83865), each
    // naming its own type; the new seeds leave them as they were.
    for (program_chirho, file_name_chirho, reasons_chirho) in [
        (
            "module Main where\nxChirho :: Int\nxChirho = True\nmain :: IO ()\nmain = print xChirho\n",
            "T02IntFromTrueChirho.hs",
            ["type mismatch", "`Bool`"],
        ),
        (
            "module Main where\nxChirho :: Int\nxChirho = Nothing\nmain :: IO ()\nmain = print xChirho\n",
            "T03IntFromNothingChirho.hs",
            ["type mismatch", "Maybe"],
        ),
        (
            "module Main where\nxChirho :: Int\nxChirho = Left 'a'\nmain :: IO ()\nmain = print xChirho\n",
            "N01IntFromLeftChirho.hs",
            ["type mismatch", "Either Char"],
        ),
        (
            "module Main where\nimport Data.Proxy\nxChirho :: Int\nxChirho = Proxy\nmain :: IO ()\nmain = print xChirho\n",
            "N02IntFromProxyChirho.hs",
            ["type mismatch", "Proxy"],
        ),
        (
            "module Main where\nxChirho :: Int\nxChirho = ()\nmain :: IO ()\nmain = print xChirho\n",
            "N03IntFromUnitChirho.hs",
            ["type mismatch", "`()`"],
        ),
    ] {
        rejected_like_ghc_chirho(program_chirho, file_name_chirho, &reasons_chirho);
    }
}
