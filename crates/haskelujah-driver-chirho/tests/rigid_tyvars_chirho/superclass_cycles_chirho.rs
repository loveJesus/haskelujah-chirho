// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Unit 519, T16414: superclass cycles. UndecidableSuperClasses allows a class
//! to reach itself through its superclasses. The superclass walk that context
//! reduction, given entailment and the fundep refutation's subclass guard
//! share visits each class once, so such a cycle ends the walk at every caller.
//! Every program was checked with GHC 9.14.1 (`-fno-code`) and with frozen
//! CLIs exactly as built here:
//! - held ba1e5c29 overflowed its stack on C1 and C2;
//! - the fundep candidates 3ff084c1 and f9974cc4 also overflowed on C5 and
//!   T16414.
//!
//! The receipts are retained under `tmp-chirho/superclass-cycle-1018-chirho/`
//! of claude2_chirho's worktree.
//! workflow: language-features-chirho/fundep-generalization-chirho

use super::check_chirho;
use super::fundep_generalization_chirho::miss_chirho;

const HEADER_CHIRHO: &str = "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE UndecidableSuperClasses, MultiParamTypeClasses, FunctionalDependencies #-}";

/// `AllChirho` and `AllTwoChirho` name each other as superclasses, as T16414's
/// `All` and `All2` do. `FooChirho` stands apart from both.
const CYCLE_CHIRHO: &str = "class AllTwoChirho aChirho => AllChirho aChirho\nclass AllChirho aChirho => AllTwoChirho aChirho\nclass FooChirho aChirho\n";

const USE_ALL_CHIRHO: &str =
    "useAllChirho :: AllChirho aChirho => aChirho -> Int\nuseAllChirho _ = 0\n";

const USE_ALL_TWO_CHIRHO: &str =
    "useAllTwoChirho :: AllTwoChirho aChirho => aChirho -> Int\nuseAllTwoChirho _ = 0\n";

const USE_FOO_CHIRHO: &str =
    "useFooChirho :: FooChirho aChirho => aChirho -> Int\nuseFooChirho _ = 0\n";

/// GHC's should_fail T16414, byte for byte. GHC rejects it for overlapping
/// `AllZip2` instances, a rule outside unit 519. Its unrelated cycle
/// `All -> All2 -> All` sent the subclass guard into a walk that never ended.
const T16414_CHIRHO: &str =
    include_str!("../../../../ghc-tests-chirho/typecheck-chirho/should_fail/T16414.hs");

fn program_chirho(module_chirho: &str, declarations_chirho: &[&str]) -> String {
    format!(
        "{HEADER_CHIRHO}\nmodule {module_chirho} where\n\n{CYCLE_CHIRHO}\n{}\n",
        declarations_chirho.join("\n")
    )
}

/// GHC accepts each.
const ACCEPTED_CHIRHO: &[(&str, &[&str])] = &[
    // Context reduction compares the two scheme predicates over one type.
    (
        "CycleC1Chirho",
        &[
            USE_ALL_CHIRHO,
            USE_FOO_CHIRHO,
            "bothChirho xChirho = useAllChirho xChirho + useFooChirho xChirho",
        ],
    ),
    // Given entailment meets the cyclic given first, then the given that entails.
    (
        "CycleC2Chirho",
        &[
            USE_FOO_CHIRHO,
            "givenChirho :: (AllChirho aChirho, FooChirho aChirho) => aChirho -> Int\ngivenChirho xChirho = useFooChirho xChirho",
        ],
    ),
    // A superclass reached through the cycle is still entailed.
    (
        "CycleC4Chirho",
        &[
            USE_ALL_TWO_CHIRHO,
            "viaCycleChirho :: AllChirho aChirho => aChirho -> Int\nviaCycleChirho xChirho = useAllTwoChirho xChirho",
        ],
    ),
];

/// GHC rejects it with "No instance for". Beside the cycle, the subclass guard
/// still answers that no class extends `CChirho`, so the refutation fires
/// (F12's shape).
const REJECTED_CHIRHO: &[(&str, &[&str])] = &[(
    "CycleC5Chirho",
    &[
        "class CChirho aChirho bChirho | aChirho -> bChirho where\n  methChirho :: aChirho -> bChirho -> ()\n\ndata PlainChirho = PlainChirho\n",
        "fChirho xChirho = \\yChirho -> methChirho (xChirho :: PlainChirho) yChirho",
    ],
)];

/// Every program in the table that misses its GHC verdict, with the reason.
fn misses_chirho(expect_rejected_chirho: bool, programs_chirho: &[(&str, &[&str])]) -> Vec<String> {
    programs_chirho
        .iter()
        .filter_map(|(module_chirho, declarations_chirho)| {
            miss_chirho(
                expect_rejected_chirho,
                module_chirho,
                &program_chirho(module_chirho, declarations_chirho),
            )
        })
        .collect()
}

#[test]
fn a_superclass_cycle_keeps_valid_programs_accepted_chirho() {
    let missed_chirho = misses_chirho(false, ACCEPTED_CHIRHO);
    assert!(
        missed_chirho.is_empty(),
        "GHC accepts each of these: {missed_chirho:#?}"
    );
}

#[test]
fn the_fundep_refutation_still_fires_beside_a_superclass_cycle_chirho() {
    let missed_chirho = misses_chirho(true, REJECTED_CHIRHO);
    assert!(
        missed_chirho.is_empty(),
        "GHC rejects each of these with `No instance for`: {missed_chirho:#?}"
    );
}

#[test]
fn t16414_keeps_the_held_route_chirho() {
    // Held ba1e5c29 accepts T16414. GHC's overlap rejection is not claimed here.
    if let Err(err_chirho) = check_chirho("T16414.hs", T16414_CHIRHO) {
        panic!("T16414: held accepts this program, we reported: {err_chirho}");
    }
}
