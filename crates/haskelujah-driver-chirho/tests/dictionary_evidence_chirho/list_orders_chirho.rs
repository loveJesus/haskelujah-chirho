// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! How rows from the conditional list generator answer (gpt_chirho disposition
//! on #28041). A row named for a list key keeps its method's own calling
//! convention, so a caller that finds it by name gets either the method's
//! answer or a failure naming the method, never a partial application.
//! Each program's GHC 9.14.1 behavior is recorded beside it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::run_result_chirho;

#[test]
fn a_missing_list_method_fails_by_name_where_its_row_is_found_by_name_chirho() {
    // GHC 9.14.1 compiles this with -Wmissing-methods, prints `[` and stops with
    // "No instance nor default method for class operation neededChirho". The
    // stub used to take the element dictionary first: `map` passed each list
    // as that dictionary, and the program printed [[1]] and succeeded.
    let err_chirho = run_result_chirho(
        "MissingListMethodUnderMapChirho.hs",
        "module Main where\nclass NeedsChirho a where\n  neededChirho :: a -> Int\ninstance NeedsChirho Int where\n  neededChirho x = x\ninstance NeedsChirho a => NeedsChirho [a]\nmain :: IO ()\nmain = print (map neededChirho [[1 :: Int]])\n",
    )
    .expect_err("GHC fails at run time; a missing method must never answer");
    assert!(
        err_chirho.contains("missing method NeedsChirho.neededChirho for [Int]"),
        "the failure must name the missing method: {err_chirho}"
    );
}
