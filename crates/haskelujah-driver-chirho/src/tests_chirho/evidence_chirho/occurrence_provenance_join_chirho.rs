// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The join by provenance, and the boundary it keeps with the fallbacks
//! (gpt_chirho #24541, #24575): records reach their own occurrences whatever the
//! order of either side; an occurrence with provenance but no proof is never
//! rescued; a record with an origin never serves another occurrence; a class
//! proved at two types proves nothing; every copy of one use takes its proof;
//! a literal takes its own origin's evidence, never its placeholder span's.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::provenance_chirho::{
    OccurrenceRoleChirho, OriginIdChirho, OriginSupplyChirho, ProvenanceChirho,
};
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::infer_chirho::MethodOccurrenceRecordChirho;

use super::occurrence_join_chirho::{infer_result_chirho, record_chirho};
use crate::join_occurrence_evidence_chirho;

/// A record for `show` at `ty_key_chirho`, identified by `origin_chirho`, under
/// the placeholder span generated code carries.
fn identified_record_chirho(
    ty_key_chirho: &str,
    origin_chirho: Option<OriginIdChirho>,
) -> MethodOccurrenceRecordChirho {
    MethodOccurrenceRecordChirho {
        origin_chirho,
        ..record_chirho("show", 0, ty_key_chirho, SpanChirho::DUMMY_CHIRHO)
    }
}

fn reference_chirho(origin_chirho: OriginIdChirho) -> ProvenanceChirho {
    ProvenanceChirho::new_chirho(origin_chirho, OccurrenceRoleChirho::Reference)
}

/// Occurrences of `show`, each a fresh id sharing one canonical id.
fn show_occurrences_chirho(ids_chirho: &[u32]) -> HashMap<CoreIdChirho, (String, CoreIdChirho)> {
    ids_chirho
        .iter()
        .map(|id_chirho| {
            (
                CoreIdChirho(*id_chirho),
                ("show".to_string(), CoreIdChirho(1)),
            )
        })
        .collect()
}

fn join_chirho(
    records_chirho: Vec<MethodOccurrenceRecordChirho>,
    occurrence_ids_chirho: &[u32],
    provenance_chirho: HashMap<CoreIdChirho, ProvenanceChirho>,
) -> HashMap<CoreIdChirho, (String, String)> {
    join_occurrence_evidence_chirho(
        &infer_result_chirho(records_chirho),
        &show_occurrences_chirho(occurrence_ids_chirho),
        &HashMap::new(),
        &HashMap::new(),
        &provenance_chirho,
    )
}

fn proof_chirho(key_chirho: &str) -> (String, String) {
    ("Show".to_string(), key_chirho.to_string())
}

#[test]
fn records_reach_their_own_occurrences_in_any_order_chirho() {
    // The checker met the Bool use first and the desugarer the Int use first:
    // provenance, not order, decides.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let int_use_chirho = supply_chirho.fresh_chirho();
    let bool_use_chirho = supply_chirho.fresh_chirho();
    let evidence_chirho = join_chirho(
        vec![
            identified_record_chirho("Bool", Some(bool_use_chirho)),
            identified_record_chirho("Int", Some(int_use_chirho)),
        ],
        &[10, 11],
        HashMap::from([
            (CoreIdChirho(10), reference_chirho(int_use_chirho)),
            (CoreIdChirho(11), reference_chirho(bool_use_chirho)),
        ]),
    );
    assert_eq!(
        evidence_chirho.get(&CoreIdChirho(10)),
        Some(&proof_chirho("Int"))
    );
    assert_eq!(
        evidence_chirho.get(&CoreIdChirho(11)),
        Some(&proof_chirho("Bool"))
    );
}

#[test]
fn an_occurrence_with_provenance_and_no_proof_is_never_rescued_chirho() {
    // A spare record with no origin exists for `show`, and positionally it would
    // fit. The occurrence has an identity, so it takes nothing but its own proof.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let unproved_use_chirho = supply_chirho.fresh_chirho();
    let evidence_chirho = join_chirho(
        vec![identified_record_chirho("Int", None)],
        &[10],
        HashMap::from([(CoreIdChirho(10), reference_chirho(unproved_use_chirho))]),
    );
    assert_eq!(evidence_chirho.get(&CoreIdChirho(10)), None);
}

#[test]
fn a_record_with_an_origin_never_serves_another_occurrence_chirho() {
    // The record's own occurrence is absent; an occurrence with no identity is
    // there to take it by position, and must not.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let elsewhere_chirho = supply_chirho.fresh_chirho();
    let evidence_chirho = join_chirho(
        vec![identified_record_chirho("Int", Some(elsewhere_chirho))],
        &[10],
        HashMap::new(),
    );
    assert_eq!(evidence_chirho.get(&CoreIdChirho(10)), None);
}

#[test]
fn a_class_proved_at_two_types_proves_nothing_chirho() {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let use_chirho = supply_chirho.fresh_chirho();
    let evidence_chirho = join_chirho(
        vec![
            identified_record_chirho("Int", Some(use_chirho)),
            identified_record_chirho("Bool", Some(use_chirho)),
        ],
        &[10],
        HashMap::from([(CoreIdChirho(10), reference_chirho(use_chirho))]),
    );
    assert_eq!(evidence_chirho.get(&CoreIdChirho(10)), None);
}

#[test]
fn every_copy_of_one_use_takes_its_proof_chirho() {
    // The desugarer may desugar one source use twice; both copies are that use,
    // at that type.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let use_chirho = supply_chirho.fresh_chirho();
    let evidence_chirho = join_chirho(
        vec![identified_record_chirho("Int", Some(use_chirho))],
        &[10, 11],
        HashMap::from([
            (CoreIdChirho(10), reference_chirho(use_chirho)),
            (CoreIdChirho(11), reference_chirho(use_chirho)),
        ]),
    );
    assert_eq!(
        evidence_chirho.get(&CoreIdChirho(10)),
        Some(&proof_chirho("Int"))
    );
    assert_eq!(
        evidence_chirho.get(&CoreIdChirho(11)),
        Some(&proof_chirho("Int"))
    );
}

/// A literal occurrence `10` of `fromInteger` with the given provenance, joined
/// against origin-keyed and span-keyed literal evidence.
fn join_literal_chirho(
    provenance_chirho: Option<ProvenanceChirho>,
    by_origin_chirho: Vec<(OriginIdChirho, &str)>,
    by_span_chirho: Vec<(SpanChirho, &str)>,
) -> Option<(String, String)> {
    use haskelujah_typing_chirho::infer_chirho::LiteralEvidenceChirho;
    let evidence_chirho = |key_chirho: &str| LiteralEvidenceChirho {
        class_name_chirho: "Num".to_string(),
        ty_key_chirho: key_chirho.to_string(),
    };
    let mut result_chirho = infer_result_chirho(vec![]);
    result_chirho.literal_evidence_by_origin_chirho = by_origin_chirho
        .into_iter()
        .map(|(origin_chirho, key_chirho)| (origin_chirho, evidence_chirho(key_chirho)))
        .collect();
    result_chirho.literal_evidence_chirho = by_span_chirho
        .into_iter()
        .map(|(span_chirho, key_chirho)| (span_chirho, evidence_chirho(key_chirho)))
        .collect();
    let occurrences_chirho = HashMap::from([(
        CoreIdChirho(10),
        ("fromInteger".to_string(), CoreIdChirho(1)),
    )]);
    let literal_spans_chirho = HashMap::from([(CoreIdChirho(10), SpanChirho::DUMMY_CHIRHO)]);
    let provenance_map_chirho: HashMap<CoreIdChirho, ProvenanceChirho> = provenance_chirho
        .map(|provenance_chirho| (CoreIdChirho(10), provenance_chirho))
        .into_iter()
        .collect();
    join_occurrence_evidence_chirho(
        &result_chirho,
        &occurrences_chirho,
        &literal_spans_chirho,
        &HashMap::new(),
        &provenance_map_chirho,
    )
    .remove(&CoreIdChirho(10))
}

#[test]
fn a_generated_literal_takes_its_own_origins_evidence_chirho() {
    // Under the placeholder span the span map could only say what some other
    // literal was; the literal's own origin says what this one is.
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let literal_chirho = supply_chirho.fresh_chirho();
    let joined_chirho = join_literal_chirho(
        Some(ProvenanceChirho::new_chirho(
            literal_chirho,
            OccurrenceRoleChirho::IntegerLiteral,
        )),
        vec![(literal_chirho, "Double")],
        vec![(SpanChirho::DUMMY_CHIRHO, "Int")],
    );
    assert_eq!(
        joined_chirho,
        Some(("Num".to_string(), "Double".to_string()))
    );
}

#[test]
fn a_literal_with_provenance_and_no_evidence_is_not_rescued_by_span_chirho() {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let literal_chirho = supply_chirho.fresh_chirho();
    let joined_chirho = join_literal_chirho(
        Some(ProvenanceChirho::new_chirho(
            literal_chirho,
            OccurrenceRoleChirho::IntegerLiteral,
        )),
        vec![],
        vec![(SpanChirho::DUMMY_CHIRHO, "Int")],
    );
    assert_eq!(joined_chirho, None);
}
