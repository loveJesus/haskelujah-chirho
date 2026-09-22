// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The origin supply's two guarantees: no origin is minted twice, and producers
//! that borrow the one supply at different times continue it rather than
//! restarting it (lowering first, then early deriving, then the late GND pass).
//! workflow: language-features-chirho/dictionary-evidence-chirho

use crate::name_chirho::RawNameChirho;
use crate::provenance_chirho::{OccurrenceRoleChirho, OriginSupplyChirho, ProvenanceChirho};
use haskelujah_span_chirho::SpanChirho;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

fn hash_of_chirho(name_chirho: &RawNameChirho) -> u64 {
    let mut hasher_chirho = DefaultHasher::new();
    name_chirho.hash(&mut hasher_chirho);
    hasher_chirho.finish()
}

#[test]
fn an_origin_does_not_change_what_a_name_is_chirho() {
    // Name equality serves AST comparison and name lookup. Two occurrences of
    // `show` under one placeholder span, as derived instances produce them, must
    // stay equal names whatever their origins, so no lookup changes meaning.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let bare_chirho = RawNameChirho::unqualified_chirho("show", SpanChirho::DUMMY_CHIRHO);
    let first_chirho = bare_chirho
        .clone()
        .with_origin_chirho(supply_chirho.fresh_chirho());
    let second_chirho = bare_chirho
        .clone()
        .with_origin_chirho(supply_chirho.fresh_chirho());
    assert_ne!(first_chirho.origin_chirho, second_chirho.origin_chirho);
    for name_chirho in [&first_chirho, &second_chirho] {
        assert_eq!(*name_chirho, bare_chirho);
        assert_eq!(hash_of_chirho(name_chirho), hash_of_chirho(&bare_chirho));
    }
    let names_chirho: HashSet<RawNameChirho> = [first_chirho, second_chirho].into();
    assert_eq!(names_chirho.len(), 1);
}

#[test]
fn moving_an_occurrence_keeps_its_origin_chirho() {
    // A pass that clones or moves the same occurrence must not lose or renumber
    // its identity; only the producer mints.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let origin_chirho = supply_chirho.fresh_chirho();
    let name_chirho =
        RawNameChirho::qualified_chirho("Data.List", "sort", SpanChirho::DUMMY_CHIRHO)
            .with_origin_chirho(origin_chirho);
    let moved_chirho = Box::new(name_chirho.clone());
    assert_eq!(name_chirho.origin_chirho, Some(origin_chirho));
    assert_eq!(moved_chirho.origin_chirho, Some(origin_chirho));
    assert_eq!(supply_chirho.minted_chirho(), 1);
}

#[test]
fn a_name_without_an_origin_prints_as_before_chirho() {
    let bare_chirho = RawNameChirho::unqualified_chirho("x", SpanChirho::DUMMY_CHIRHO);
    let rendered_chirho = format!("{bare_chirho:?}");
    assert!(
        rendered_chirho.contains("text_chirho: \"x\""),
        "{rendered_chirho}"
    );
    assert!(
        !rendered_chirho.contains("origin_chirho"),
        "{rendered_chirho}"
    );
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let stamped_chirho = bare_chirho.with_origin_chirho(supply_chirho.fresh_chirho());
    assert!(format!("{stamped_chirho:?}").contains("origin_chirho"));
}
