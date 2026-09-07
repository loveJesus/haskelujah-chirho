// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Canaries: is the checker still awake after this construct?
//!
//! Every silent-fabrication defect found on 2026-09-06/07 had one signature — a
//! construct the front end could not represent quietly stopped the declarations
//! AFTER it from being checked, and no gate could see it. The corpus only ever asks
//! "did this file error", so a file whose second half vanished still counts as a
//! pass. Seven such sites were found in one evening, none of them by a gate; each
//! surfaced because an unrelated defect happened to shout first.
//!
//! A canary asks the opposite question. Put a declaration that MUST fail after the
//! construct, and assert that it still fails. If it stops failing, the construct
//! above it is swallowing what follows.
//!
//! The method is due to `gpt_chirho`, who used it to prove the old data-instance
//! skip was eating the next declaration (a `MissingTypeChirho` decl placed after a
//! data instance checked clean on the pre-boundary commit and errors now).
//!
//! They live here, as a one-subject integration file beside `dictionary_evidence`
//! and `rigid_tyvars`, rather than as another sibling in `src/tests_chirho/` —
//! that directory is already 24 entries against a cap of 15, and this would have
//! been the 25th.
//!
//! **Why these are tests and not corpus files.** The first
//! run of these canaries as CLI probes produced a false alarm: the shared tree's
//! prebuilt `haskelujah` was four hours older than HEAD, so the canaries faithfully
//! reported a bug that had already been fixed. A `cargo test` cannot make that
//! mistake — the test binary is always built from the source under test. A canary
//! is only as trustworthy as the provenance of what it measures, so it belongs
//! where provenance is structural.

use haskelujah_driver::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

/// The declaration that must always be diagnosed. Its type does not exist, so any
/// checker that reaches it must complain, whatever precedes it.
const CANARY_DECL_CHIRHO: &str = "canaryChirho :: NoSuchTypeCanaryChirho\ncanaryChirho = ()\n";

/// Compile `preamble` followed by the canary and report whether the canary was
/// still diagnosed.
///
/// It is not enough to ask whether the compile FAILED. Several of the constructs
/// below are themselves unrepresented and may error on their own, and a canary that
/// accepted any error would then pass while the declaration after it was quietly
/// swallowed — green for precisely the reason this file exists to catch. So the
/// canary's own type name must appear in a diagnostic: proof the checker reached it.
fn canary_survives_chirho(preamble_chirho: &str) -> bool {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_chirho = format!("{preamble_chirho}{CANARY_DECL_CHIRHO}");
    match compile_source_chirho(&source_chirho, &mut source_map_chirho, "CanaryChirho.hs") {
        Ok(_) => false,
        Err(bundle_chirho) => bundle_chirho
            .diagnostics_chirho()
            .iter()
            .any(|d_chirho| d_chirho.message_chirho.contains("NoSuchTypeCanaryChirho")),
    }
}

#[test]
fn canary_alone_is_diagnosed_chirho() {
    // The control. If this ever passes, every other test in this file is vacuous
    // and is measuring nothing — exactly the failure mode the file exists to catch.
    assert!(
        canary_survives_chirho("module CanaryChirho where\n"),
        "the canary itself must be diagnosed, or no canary in this file means anything"
    );
}

#[test]
fn an_unrelated_error_does_not_name_the_canary_chirho() {
    // The second control, and the one that matters most. `canary_survives_chirho`
    // asks whether the canary's own type name reached a diagnostic, not merely
    // whether the compile failed. This proves that check discriminates: a preamble
    // that fails entirely on its own produces diagnostics that do NOT name the
    // canary. Without it, every canary below would start passing the moment its
    // construct began erroring for any unrelated reason — green for exactly the
    // reason this suite exists to detect.
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let bundle_chirho = compile_source_chirho(
        "module CanaryChirho where\nbroken_chirho = notDefinedAnywhereChirho\n",
        &mut source_map_chirho,
        "CanaryChirho.hs",
    )
    .expect_err("this preamble must fail on its own, or the control proves nothing");
    assert!(
        !bundle_chirho
            .diagnostics_chirho()
            .iter()
            .any(|d_chirho| d_chirho.message_chirho.contains("NoSuchTypeCanaryChirho")),
        "an unrelated failure must not name the canary, or the name check discriminates nothing"
    );
}

#[test]
fn declarations_survive_a_gadt_record_constructor_chirho() {
    // b2027c35. Before it, the record braces were handed to the type parser, which
    // could not read them, and the module lost everything after the declaration —
    // `main` did not even reach STG while `check` called the file clean.
    for preamble_chirho in [
        "{-# LANGUAGE GADTs #-}\nmodule CanaryChirho where\ndata T where\n  MkT :: { fld :: Int } -> T\n",
        // ...and with a refined result type, which is still unrepresented: the
        // constructor's scheme is wrong, but what FOLLOWS must still be checked.
        "{-# LANGUAGE GADTs #-}\nmodule CanaryChirho where\ndata T a where\n  MkT :: { fld :: Int } -> T Bool\n",
    ] {
        assert!(
            canary_survives_chirho(preamble_chirho),
            "a GADT record constructor swallowed the declaration after it: {preamble_chirho}"
        );
    }
}

#[test]
fn declarations_survive_a_data_instance_chirho() {
    // The skip used to run to the end of the module instead of stopping at the
    // declaration's own layout boundary. Data instances remain UNREPRESENTED — this
    // asserts only that being unrepresented stays local.
    assert!(
        canary_survives_chirho(
            "{-# LANGUAGE TypeFamilies #-}\nmodule CanaryChirho where\ndata family G a\ndata instance G Int = MkGInt\n"
        ),
        "a data instance swallowed the declaration after it"
    );
}

#[test]
fn declarations_survive_a_newtype_instance_chirho() {
    assert!(
        canary_survives_chirho(
            "{-# LANGUAGE TypeFamilies #-}\nmodule CanaryChirho where\ndata family G a\nnewtype instance G Int = MkGInt Int\n"
        ),
        "a newtype instance swallowed the declaration after it"
    );
}

#[test]
fn declarations_survive_type_data_chirho() {
    assert!(
        canary_survives_chirho(
            "{-# LANGUAGE TypeData #-}\nmodule CanaryChirho where\ntype data Letter = A | B\n"
        ),
        "a type data declaration swallowed the declaration after it"
    );
}

#[test]
fn declarations_survive_an_explicit_brace_block_chirho() {
    // gpt_chirho's layout root cause: an explicit `}` left a nested implicit
    // layout context above its matching `{`, burying the module context and
    // suppressing every later virtual semicolon. That is the general form of the
    // defect the other canaries catch one construct at a time, so it is worth
    // asserting on explicit braces directly rather than only where they happen to
    // appear in a declaration head.
    for preamble_chirho in [
        "module CanaryChirho where\nf x = case x of { 0 -> 1 ; _ -> 2 }\n",
        "module CanaryChirho where\ng y = let { a = 1 ; b = 2 } in a + b + y\n",
        "module CanaryChirho where\ndata R = R { rf :: Int }\nh r = r { rf = 1 }\n",
    ] {
        assert!(
            canary_survives_chirho(preamble_chirho),
            "an explicit brace block swallowed the declaration after it: {preamble_chirho}"
        );
    }
}
