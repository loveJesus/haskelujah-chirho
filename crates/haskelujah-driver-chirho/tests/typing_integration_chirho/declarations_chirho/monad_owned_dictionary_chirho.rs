// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `>>=` and `>>` take the owning Monad dictionary (row 697).
//!
//! In a binding that owns a `Monad` dictionary (`sevenChirho :: Monad m => m Int`), `return`
//! and `pure` already took the owning Applicative, while `>>=` and `>>` kept their names when no
//! argument proved the monad, and STG then ran them as IO's bind and then. The body was right
//! only when the caller's monad was IO: the unary Maybe probe failed where the IO probe printed
//! 7 (#31932, #31935). Now the owning dictionary decides them as it decides `return`.
//!
//! The controls cover the whole boundary:
//! - generic bind and then at Maybe and the list monad, by operator and by do-notation;
//! - constrained helpers at IO, whose effects keep their exact order through IO's dictionary;
//! - evidence-free chains in `main`, a `Show`-constrained IO helper, and a program's own `>>`
//!   beside an owned `>>=`, all of which keep their existing bindings.
//!
//! Each program is embedded byte for byte from the focused child directory
//! `monad_owned_dictionary_chirho/`, which root's GHC plan reads too. Every expected output
//! below is a prediction until that plan runs on GHC 9.14.1.
//!
//! The Reader probe stays out. A user instance of a class with a superclass registers no
//! dictionary under the key its use site carries, which is a separate defect; this unit only
//! moves Reader's failure, it does not fix it.
//! workflow: monadic-dispatch-chirho

use super::assert_prints_everywhere_chirho;

/// O1 (sha256 54cf47f9): generic bind and then at Maybe and the list monad.
/// Predicted GHC 9.14.1 output, UNEXECUTED until root's plan runs.
const GENERIC_MONAD_MAYBE_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/generic_monad_maybe_chirho.hs");

/// O2 (sha256 19d5ef12): constrained helpers at IO with exact effect order, beside
/// evidence-free chains and a `Show`-constrained IO helper.
/// Predicted GHC 9.14.1 output, UNEXECUTED until root's plan runs.
const CONSTRAINED_IO_HELPERS_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/constrained_io_helpers_chirho.hs");

/// O3 (sha256 958bec3b): a program's own `>>` beside an owned `>>=`, with an evidence-free main.
/// Predicted GHC 9.14.1 output, UNEXECUTED until root's plan runs.
const OWNED_THEN_OPERATOR_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/owned_then_operator_chirho.hs");

#[test]
fn owned_bind_and_then_take_the_callers_maybe_and_list_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "GenericMonadMaybeChirho.hs",
        GENERIC_MONAD_MAYBE_CHIRHO,
        "Just 7\nJust 7\n[7]\nJust 5\nNothing\nJust 2\nNothing\n[5,6,5,6]\n",
    );
}

#[test]
fn constrained_io_helpers_keep_their_effect_order_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "ConstrainedIoHelpersChirho.hs",
        CONSTRAINED_IO_HELPERS_CHIRHO,
        "tick\ntick\n7\nrelay\nhello\nagain\n2\n14\n'a'\n'b'\ndone\nend\n",
    );
}

#[test]
fn a_programs_own_then_and_evidence_free_io_keep_their_bindings_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "OwnedThenOperatorChirho.hs",
        OWNED_THEN_OPERATOR_CHIRHO,
        "Just 43\n53\n",
    );
}
