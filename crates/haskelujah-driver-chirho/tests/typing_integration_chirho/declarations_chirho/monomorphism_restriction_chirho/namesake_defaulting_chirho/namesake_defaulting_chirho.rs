// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 692: the restriction's defaulting under a program's own `Num`, kept apart from the C10
//! dispatch controls (gpt #32627). Both programs are in the GHC reference plan `c10-controls`.
//! A restricted binding holding only the compiler's literal `Num` defaults as GHC defaults it;
//! one holding the program's own `Num` has no default. The restriction's active-record check,
//! which this brick leaves in place, is predicted to refuse the first.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use super::{checks_like_ghc_chirho, rejected_like_ghc_chirho};

/// `countChirho = 3` beside the program's own `class Num`.
const RESTRICTED_LITERAL_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("restricted_literal_under_namesake_chirho.hs");
/// `countChirho = fromInteger 3` through the program's own `class Num`.
const RESTRICTED_SOURCE_WANTED_UNDER_NAMESAKE_CHIRHO: &str =
    include_str!("restricted_source_wanted_under_namesake_chirho.hs");

/// GHC accepts it: the literal's Num is the compiler's, and it defaults to Integer.
#[test]
fn a_restricted_compiler_literal_defaults_under_a_namesake_chirho() {
    checks_like_ghc_chirho(
        "restricted_literal_under_namesake_chirho.hs",
        RESTRICTED_LITERAL_UNDER_NAMESAKE_CHIRHO,
    );
}

/// GHC refuses it: the program's own Num is no standard class, so nothing defaults it.
#[test]
fn a_restricted_source_wanted_has_no_default_under_a_namesake_chirho() {
    rejected_like_ghc_chirho(
        RESTRICTED_SOURCE_WANTED_UNDER_NAMESAKE_CHIRHO,
        "restricted_source_wanted_under_namesake_chirho.hs",
        &["Num"],
    );
}
