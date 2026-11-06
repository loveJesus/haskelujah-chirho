// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! `>>=` and `>>` take the dictionary of the predicate the checker proved for each use
//! (row 697).
//!
//! Inside `sevenChirho :: Monad m => m Int`, `return` already took a dictionary, but `>>=` and
//! `>>` kept their names whenever no argument proved the monad. STG then ran them as IO's bind
//! and then, so the body was right only at IO (#31932, #31935).
//!
//! The checker already proves every use. It keys a use at a rigid signature variable as that
//! exact predicate, `$own:i`, and a use at a concrete type by its head. That proof now reaches
//! the dictionary pass through the existing occurrence join, so the predicate that types each
//! use selects its dictionary. Binding membership alone is not authority, which is what f40
//! wrongly took.
//!
//! What the controls pin:
//! - root's two discriminators: an IO chain inside a helper that owns a Maybe dictionary stays
//!   IO's, and two Monad predicates in one binding each keep their own dictionary;
//! - generic bind and then at Maybe and the list monad;
//! - IO helpers whose effects keep their exact order;
//! - evidence-free chains in `main`, a `Show`-only IO helper, and a program's own `>>` beside a
//!   proven `>>=`;
//! - the unary Maybe and IO probes.
//!
//! Each program is embedded byte for byte from the focused child directory
//! `monad_owned_dictionary_chirho/`. Root's fixtures keep their canonical hashes. Root measured
//! A1, A2 and O1-O3 on GHC 9.14.1, and each printed exactly the output below (#31960):
//! `tmp-chirho/ghc-contract-refresh-chirho/owned-monad-authority-r1-chirho/summary-chirho.json`
//! (81fc2d5c) and `.../owned-monad-f40-r1-chirho/summary-chirho.json` (803257ec). U1 and U2
//! have no GHC run yet.
//!
//! The Reader probe stays out. A user instance of a class with a superclass registers no
//! dictionary under the key its use site carries, which is a separate defect.
//! workflow: monadic-dispatch-chirho

use super::assert_prints_everywhere_chirho;

/// A1 (sha256 c2a75189, root's InnerIoUnderMaybeOwnerChirho, #31949): `keepChirho` owns a
/// Monad dictionary, called at Maybe, but its `>>` is proved at IO.
/// GHC 9.14.1 prints `inner`, then 7.
const INNER_IO_UNDER_MAYBE_OWNER_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/inner_io_under_maybe_owner_chirho.hs");

/// A2 (sha256 62898014, root's TwoMonadOwnersChirho, #31949): two Monad predicates in one
/// binding, each `>>` proved at its own. GHC 9.14.1 prints `(Just 2,[4,5])`.
const TWO_MONAD_OWNERS_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/two_monad_owners_chirho.hs");

/// O1 (sha256 54cf47f9): generic bind and then at Maybe and the list monad.
/// GHC 9.14.1 prints the output below.
const GENERIC_MONAD_MAYBE_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/generic_monad_maybe_chirho.hs");

/// O2 (sha256 19d5ef12): IO helpers with exact effect order, beside evidence-free chains and a
/// `Show`-constrained IO helper. GHC 9.14.1 prints the output below.
const CONSTRAINED_IO_HELPERS_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/constrained_io_helpers_chirho.hs");

/// O3 (sha256 958bec3b): a program's own `>>` beside a proven `>>=`, with an evidence-free main.
/// GHC 9.14.1 prints the output below.
const OWNED_THEN_OPERATOR_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/owned_then_operator_chirho.hs");

/// U1 (sha256 e9fc38c2, root's unary PolymorphicMaybeMonadChirho): the parent failed with
/// "no matching alternative" on b5 and frozen 47d. Predicted `Just 7`; no GHC run yet.
const POLYMORPHIC_MAYBE_MONAD_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/polymorphic_maybe_monad_chirho.hs");

/// U2 (sha256 8043777a, root's unary PolymorphicIOMonadChirho): printed 7 on b5 and frozen
/// 47d. Predicted 7; no GHC run yet.
const POLYMORPHIC_IO_MONAD_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/polymorphic_io_monad_chirho.hs");

#[test]
fn an_io_chain_under_a_maybe_owner_stays_io_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "InnerIoUnderMaybeOwnerChirho.hs",
        INNER_IO_UNDER_MAYBE_OWNER_CHIRHO,
        "inner\n7\n",
    );
}

#[test]
fn two_monad_owners_each_keep_their_own_dictionary_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "TwoMonadOwnersChirho.hs",
        TWO_MONAD_OWNERS_CHIRHO,
        "(Just 2,[4,5])\n",
    );
}

#[test]
fn proven_bind_and_then_take_the_callers_maybe_and_list_everywhere_chirho() {
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

#[test]
fn unary_maybe_and_io_monad_helpers_run_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "PolymorphicMaybeMonadChirho.hs",
        POLYMORPHIC_MAYBE_MONAD_CHIRHO,
        "Just 7\n",
    );
    assert_prints_everywhere_chirho(
        "PolymorphicIOMonadChirho.hs",
        POLYMORPHIC_IO_MONAD_CHIRHO,
        "7\n",
    );
}
