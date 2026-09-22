// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The origin supply's two guarantees: no origin is minted twice, and producers
//! that borrow the one supply at different times continue it rather than
//! restarting it (lowering first, then early deriving, then the late GND pass).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use crate::provenance_chirho::{OccurrenceRoleChirho, OriginSupplyChirho, ProvenanceChirho};
use std::collections::HashSet;

/// A stand-in for a producer: it borrows the module's supply and mints what it
/// needs, the way lowering, deriving and GND each will.
fn produce_chirho(supply_chirho: &mut OriginSupplyChirho, count_chirho: usize) -> Vec<u32> {
    (0..count_chirho)
        .map(|_| supply_chirho.fresh_chirho().as_raw_chirho())
        .collect()
}

#[test]
fn no_origin_is_minted_twice_chirho() {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let minted_chirho = produce_chirho(&mut supply_chirho, 1000);
    let distinct_chirho: HashSet<u32> = minted_chirho.iter().copied().collect();
    assert_eq!(distinct_chirho.len(), 1000);
    assert_eq!(supply_chirho.minted_chirho(), 1000);
}

#[test]
fn producers_at_different_times_continue_one_supply_chirho() {
    // Three producers run at different points of one module compilation and
    // must never collide: a producer that started its own counter would reuse
    // the first origins, which is exactly the re-enumeration this rules out.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let lowering_chirho = produce_chirho(&mut supply_chirho, 4);
    let early_deriving_chirho = produce_chirho(&mut supply_chirho, 3);
    let late_gnd_chirho = produce_chirho(&mut supply_chirho, 2);
    let all_chirho: Vec<u32> = lowering_chirho
        .iter()
        .chain(&early_deriving_chirho)
        .chain(&late_gnd_chirho)
        .copied()
        .collect();
    let distinct_chirho: HashSet<u32> = all_chirho.iter().copied().collect();
    assert_eq!(distinct_chirho.len(), all_chirho.len(), "{all_chirho:?}");
    assert!(
        early_deriving_chirho
            .iter()
            .all(|origin_chirho| *origin_chirho >= 4)
    );
    assert!(
        late_gnd_chirho
            .iter()
            .all(|origin_chirho| *origin_chirho >= 7)
    );
}

#[test]
fn one_construct_distinguishes_its_uses_by_role_chirho() {
    // A do statement with a failable pattern makes two uses from ONE origin: its
    // bind and its fail. They are told apart by role, not by a second origin
    // minted downstream.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let statement_chirho = supply_chirho.fresh_chirho();
    let bind_chirho = ProvenanceChirho::new_chirho(statement_chirho, OccurrenceRoleChirho::Bind);
    let fail_chirho = ProvenanceChirho::new_chirho(statement_chirho, OccurrenceRoleChirho::Fail);
    assert_ne!(bind_chirho, fail_chirho);
    assert_eq!(bind_chirho.origin_chirho, fail_chirho.origin_chirho);
    assert_eq!(
        supply_chirho.minted_chirho(),
        1,
        "roles must not consume origins"
    );
}
