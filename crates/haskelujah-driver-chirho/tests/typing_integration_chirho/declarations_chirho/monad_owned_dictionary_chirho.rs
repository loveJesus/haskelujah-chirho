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
//! are measured in `.../owned-monad-unary-r1-chirho/summary-chirho.json` (33735a02), R1 and R2
//! in `.../owned-monad-display-r1-chirho/summary-chirho.json` (c053a423).
//!
//! The first gate (5d564764) passed every one of these on the interpreter. Natively, two
//! display consumers failed that the bind route never reaches. R1 and R2 reproduce each one
//! directly, with no Monad dispatch at all:
//! - R1: a list typed `m Int` at `m := []` reached `print` keyed `[] Int`, which names no Show
//!   row. It now keys `[Int]`, like the list it is (O1's last line, A2).
//! - R2: `print` at a binding's own `Show` key reached the generic Int-only binding and
//!   printed a Char as its code point. It now shows through that parameter's `show` (O2).
//!
//! Row 699 adds the Reader probes, RD1-RD4. On the baseline, a user instance of a class with a
//! superclass, such as `Monad (ReaderChirho r)`, registered no dictionary under the constructor
//! its use site carries: generic-head rule G6 refused a class with superclasses, so the use site
//! got an explicit missing-dictionary error. RD1 and RD2 must print GHC's output on every engine,
//! RD2 through the instance's own callbacks and its superclass slots. GHC refuses RD3 and RD4,
//! whose superclass dictionary cannot be built, and so must every engine, by name. Each Reader
//! probe reports every engine's outcome, so one engine's failure hides no other.
//!
//! Row 699's repair serves a superclass slot by the instance its template matches, and gives the
//! compiler's own Monad GHC's `>>` default. W1 runs that default at its own type and through a
//! caller's dictionary, in effect order. W2 keeps the Reader's Functor and Applicative in another
//! module. The dictionary pass cannot see them, so the head stays on the prior path, and the
//! module is never refused for it. RD3 and RD4's refusal belongs to the checker's superclass
//! obligations (GHC-39999), whose completeness gate knows when an instance universe is closed.
//!
//! Row 699's B1 and B2 (#32333), and `pure` (#32342): a `>>=`, `>>`, `fmap` or `pure` used at a
//! concrete head this module declares takes the checker's identity-stamped proof for that head to
//! the dictionary the pass registered for it. RD2's top-level `fmap`, and W1's direct `>>` and
//! `pure 'z'`, reach their written instances that way. W3 runs such a `>>` beside `main`'s own
//! `fmap`, `pure` and `return`, which keep their own bindings.
//! workflow: monadic-dispatch-chirho

use haskelujah_driver::{compile_source_chirho, compile_source_with_search_path_chirho};
use haskelujah_span_chirho::SourceMapChirho;

use super::{as_ghc_everywhere_chirho, assert_prints_everywhere_chirho, every_engine_chirho};

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
/// "no matching alternative" on b5 and frozen 47d. GHC 9.14.1 prints `Just 7`.
const POLYMORPHIC_MAYBE_MONAD_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/polymorphic_maybe_monad_chirho.hs");

/// U2 (sha256 8043777a, root's unary PolymorphicIOMonadChirho): printed 7 on b5 and frozen
/// 47d. GHC 9.14.1 prints 7.
const POLYMORPHIC_IO_MONAD_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/polymorphic_io_monad_chirho.hs");

/// R1 (sha256 35acb0e1): `sameChirho :: mChirho Int -> mChirho Int` at a list. GHC 9.14.1
/// prints `[5,6]`.
const APPLIED_LIST_DISPLAY_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/applied_list_display_chirho.hs");

/// R2 (sha256 49d09506): `print` inside `showOneChirho :: Show aChirho => aChirho -> IO ()` at
/// Char. GHC 9.14.1 prints `'a'`.
const OWN_SHOW_PRINT_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/own_show_print_chirho.hs");

/// RD1 (sha256 edfe0611, root's retained UnaryReaderMonadChirho): `sevenChirho :: Monad m =>
/// m Int` at a written `Monad (ReaderChirho r)`, whose class has superclasses. Predicted 7.
const UNARY_READER_MONAD_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/unary_reader_monad_chirho.hs");

/// RD2 (sha256 084f64ab): one `Monad m` helper at that Reader and at IO. Its `>>=` callback
/// reads the environment, and it takes `fmap` and `pure` through Monad's superclass slots.
/// Predicted 41, 11, then 7.
const READER_SUPERCLASS_CALLBACKS_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/reader_superclass_callbacks_chirho.hs");

/// RD3 (sha256 b2e0aea9): `Monad (ReaderChirho r)` with no Applicative instance at all.
/// Predicted GHC rejection: no Applicative instance for the superclass.
const READER_MISSING_APPLICATIVE_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/reader_missing_applicative_chirho.hs");

/// RD4 (sha256 8cd77b7c): a context-free `Monad (ReaderChirho r)` over `instance Show r =>
/// Applicative (ReaderChirho r)`. Predicted GHC rejection: `Show r` for the superclass.
const READER_CONTEXTUAL_APPLICATIVE_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/reader_contextual_applicative_chirho.hs");

/// W1: `StateChirho` writes `>>=` alone. `>>` runs at its own type, through a `Monad m` caller,
/// and in a `do` block, with the effects in order, by GHC's default `m >> k = m >>= \_ -> k`.
/// GHC 9.14.1 printed the output below (#32280,
/// `tmp-chirho/ghc-contract-refresh-chirho/reader-generic-repair-r1-chirho`, summary 2b839776).
const STATE_OMITTED_THEN_CHIRHO: &str = include_str!(
    "monad_owned_dictionary_chirho/written_monads_chirho/state_omitted_then_chirho.hs"
);

/// W2: another module, `ReaderBaseChirho`, holds the Reader's Functor and Applicative. Main
/// writes the Monad and runs it. GHC 9.14.1 printed 7 (#32280, the same summary).
const READER_BASE_CHIRHO: &str =
    include_str!("monad_owned_dictionary_chirho/written_monads_chirho/ReaderBaseChirho.hs");
const READER_EXTERNAL_MAIN_CHIRHO: &str = include_str!(
    "monad_owned_dictionary_chirho/written_monads_chirho/reader_external_main_chirho.hs"
);

/// W3 (sha256 78df2c43): `>>` at `TallyChirho Bool`, a concrete head this module declares, takes
/// that head's registered dictionary and so GHC's `>>` default (row 699, B1). `main`'s own
/// where-bound `fmap`, `pure` and `return` keep their own bindings. GHC 9.14.1 printed `3`, then
/// `(200,1005,9)` (#32345, `tmp-chirho/ghc-contract-refresh-chirho/reader-concrete-heads-r1-chirho`,
/// summary df032931).
const OWNED_METHOD_VALUES_CHIRHO: &str = include_str!(
    "monad_owned_dictionary_chirho/written_monads_chirho/owned_method_values_chirho.hs"
);

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

#[test]
fn a_list_typed_through_a_constructor_variable_prints_as_its_list_everywhere_chirho() {
    assert_prints_everywhere_chirho(
        "AppliedListDisplayChirho.hs",
        APPLIED_LIST_DISPLAY_CHIRHO,
        "[5,6]\n",
    );
}

#[test]
fn print_at_a_bindings_own_show_dictionary_shows_a_char_everywhere_chirho() {
    assert_prints_everywhere_chirho("OwnShowPrintChirho.hs", OWN_SHOW_PRINT_CHIRHO, "'a'\n");
}

#[test]
fn a_monad_helper_at_a_written_reader_instance_matches_ghc_everywhere_chirho() {
    assert_eq!(
        every_engine_chirho("UnaryReaderMonadChirho.hs", UNARY_READER_MONAD_CHIRHO),
        as_ghc_everywhere_chirho("7\n"),
    );
}

#[test]
fn reader_callbacks_and_superclass_methods_take_the_written_instance_everywhere_chirho() {
    assert_eq!(
        every_engine_chirho(
            "ReaderSuperclassCallbacksChirho.hs",
            READER_SUPERCLASS_CALLBACKS_CHIRHO,
        ),
        as_ghc_everywhere_chirho("41\n11\n7\n"),
    );
}

#[test]
fn a_reader_instance_whose_superclass_dictionary_cannot_be_built_is_refused_chirho() {
    // Both files are measured before the one assertion, so an accepted first file never hides
    // the second file's outcome: its refusal, or every engine's run when it compiles.
    let outcomes_chirho: Vec<_> = [
        (
            "ReaderMissingApplicativeChirho.hs",
            READER_MISSING_APPLICATIVE_CHIRHO,
        ),
        (
            "ReaderContextualApplicativeChirho.hs",
            READER_CONTEXTUAL_APPLICATIVE_CHIRHO,
        ),
    ]
    .into_iter()
    .map(|(file_name_chirho, source_chirho)| {
        let compiled_chirho = compile_source_chirho(
            source_chirho,
            &mut SourceMapChirho::new_chirho(),
            file_name_chirho,
        );
        let engines_chirho = compiled_chirho
            .is_ok()
            .then(|| every_engine_chirho(file_name_chirho, source_chirho));
        let refusal_chirho = compiled_chirho
            .err()
            .map(|refusal_chirho| refusal_chirho.to_string());
        (file_name_chirho, refusal_chirho, engines_chirho)
    })
    .collect();
    assert!(
        outcomes_chirho
            .iter()
            .all(|(_, refusal_chirho, _)| refusal_chirho.is_some()),
        "GHC refuses both; any file that compiled ran on every engine instead: {outcomes_chirho:?}"
    );
}

#[test]
fn a_monad_that_writes_only_bind_runs_ghcs_then_everywhere_chirho() {
    assert_eq!(
        every_engine_chirho("StateOmittedThenChirho.hs", STATE_OMITTED_THEN_CHIRHO),
        as_ghc_everywhere_chirho("[1,2]\n[3,4,3]\n'z'\n[6,7]\n"),
    );
}

#[test]
fn a_concrete_head_takes_its_dictionary_beside_the_programs_own_method_values_chirho() {
    assert_eq!(
        every_engine_chirho("OwnedMethodValuesChirho.hs", OWNED_METHOD_VALUES_CHIRHO),
        as_ghc_everywhere_chirho("3\n(200,1005,9)\n"),
    );
}

#[test]
fn a_monad_over_another_modules_applicative_is_never_refused_chirho() {
    // NotBuiltHere (#32245): this Core input holds none of the rows behind the superclass
    // dictionaries, so the Monad head is a named gap on the prior path, never a module refusal.
    let dir_chirho = tempfile::tempdir().expect("a scratch directory");
    std::fs::write(
        dir_chirho.path().join("ReaderBaseChirho.hs"),
        READER_BASE_CHIRHO,
    )
    .expect("the sibling module");
    let compiled_chirho = compile_source_with_search_path_chirho(
        READER_EXTERNAL_MAIN_CHIRHO,
        &mut SourceMapChirho::new_chirho(),
        "Main.hs",
        dir_chirho.path(),
    );
    assert!(
        compiled_chirho.is_ok(),
        "GHC accepts the program: {:?}",
        compiled_chirho
            .err()
            .map(|refusal_chirho| refusal_chirho.to_string())
    );
}
