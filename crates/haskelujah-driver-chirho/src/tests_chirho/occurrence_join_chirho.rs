// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Boundaries of the method-occurrence evidence join, exercised directly because
//! no source program reaches them today: a proof serves one reference, and an
//! occurrence whose span is ambiguous is refused rather than guessed at.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use crate::join_occurrence_evidence_chirho;
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_span_chirho::{ByteOffsetChirho, SpanChirho};
use haskelujah_typing_chirho::infer_chirho::{InferResultChirho, MethodOccurrenceRecordChirho};
use std::collections::HashMap;

fn span_chirho(start_chirho: u32, end_chirho: u32) -> SpanChirho {
    // A REAL file id: the join treats a synthetic or dummy span as no identity at
    // all, so a control built on one would prove nothing about identified spans.
    let mut source_map_chirho = haskelujah_span_chirho::SourceMapChirho::new_chirho();
    let file_id_chirho =
        source_map_chirho.add_file_chirho("OccurrenceJoinChirho.hs", "module M where\n");
    SpanChirho::new_chirho(
        file_id_chirho,
        ByteOffsetChirho::new_chirho(start_chirho),
        ByteOffsetChirho::new_chirho(end_chirho),
    )
}

fn record_chirho(
    name_chirho: &str,
    ordinal_chirho: u32,
    ty_key_chirho: &str,
    span_chirho: SpanChirho,
) -> MethodOccurrenceRecordChirho {
    MethodOccurrenceRecordChirho {
        name_chirho: name_chirho.to_string(),
        ordinal_chirho,
        class_name_chirho: "Show".to_string(),
        ty_key_chirho: ty_key_chirho.to_string(),
        span_chirho,
    }
}

/// A result carrying only what the join reads, with a class that declares `show`
/// so every record below is authoritative.
fn infer_result_chirho(records_chirho: Vec<MethodOccurrenceRecordChirho>) -> InferResultChirho {
    use haskelujah_typing_chirho::class_chirho::{ClassDeclChirho, ClassEnvChirho};
    let mut class_env_chirho = ClassEnvChirho::new_chirho();
    let mut methods_chirho = HashMap::new();
    methods_chirho.insert(
        "show".to_string(),
        haskelujah_typing_chirho::ty_chirho::SchemeChirho::mono_chirho(
            haskelujah_typing_chirho::ty_chirho::TyChirho::string_chirho(),
        ),
    );
    class_env_chirho.classes_chirho.insert(
        "Show".to_string(),
        ClassDeclChirho {
            name_chirho: "Show".to_string(),
            supers_chirho: vec![],
            var_chirho: haskelujah_typing_chirho::ty_chirho::TyVarChirho(0),
            methods_chirho,
            extra_vars_chirho: vec![],
            fundeps_chirho: vec![],
            defaults_chirho: HashMap::new(),
        },
    );
    let mut totals_chirho = HashMap::new();
    totals_chirho.insert("show".to_string(), records_chirho.len() as u32);
    InferResultChirho {
        subst_chirho: haskelujah_typing_chirho::subst_chirho::SubstChirho::empty_chirho(),
        env_chirho: haskelujah_typing_chirho::env_chirho::TyEnvChirho::new_chirho(),
        class_env_chirho,
        type_families_chirho: Default::default(),
        diagnostics_chirho: haskelujah_diagnostics_chirho::DiagnosticBundleChirho::empty_chirho(),
        method_occurrences_chirho: records_chirho,
        method_occurrence_totals_chirho: totals_chirho,
        literal_evidence_chirho: HashMap::new(),
        reference_evidence_chirho: HashMap::new(),
    }
}

#[test]
fn a_record_consumed_by_span_is_never_reused_by_position_chirho() {
    // Two occurrences: one identified by its span, one with no span at all. The
    // identified one's record must not also serve the unspanned one.
    let identified_chirho = CoreIdChirho(10);
    let unspanned_chirho = CoreIdChirho(11);
    let mut occurrences_chirho = HashMap::new();
    occurrences_chirho.insert(identified_chirho, ("show".to_string(), CoreIdChirho(1)));
    occurrences_chirho.insert(unspanned_chirho, ("show".to_string(), CoreIdChirho(1)));
    let mut spans_chirho = HashMap::new();
    spans_chirho.insert(identified_chirho, span_chirho(100, 104));

    let evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho(vec![record_chirho(
            "show",
            0,
            "WChirho",
            span_chirho(100, 104),
        )]),
        &occurrences_chirho,
        &HashMap::new(),
        &spans_chirho,
    );

    assert_eq!(
        evidence_chirho
            .get(&identified_chirho)
            .map(|(_, key_chirho)| key_chirho.as_str()),
        Some("WChirho"),
        "the spanned occurrence keeps its own proof"
    );
    assert!(
        !evidence_chirho.contains_key(&unspanned_chirho),
        "a consumed record must not be handed to a second reference: {evidence_chirho:?}"
    );
}

#[test]
fn two_occurrences_sharing_one_span_are_refused_chirho() {
    // An ambiguous span identifies nothing, and neither occurrence may then be
    // assigned by position even though the counts line up.
    let first_chirho = CoreIdChirho(20);
    let second_chirho = CoreIdChirho(21);
    let shared_chirho = span_chirho(200, 204);
    let mut occurrences_chirho = HashMap::new();
    occurrences_chirho.insert(first_chirho, ("show".to_string(), CoreIdChirho(2)));
    occurrences_chirho.insert(second_chirho, ("show".to_string(), CoreIdChirho(2)));
    let mut spans_chirho = HashMap::new();
    spans_chirho.insert(first_chirho, shared_chirho);
    spans_chirho.insert(second_chirho, shared_chirho);

    let evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho(vec![
            record_chirho("show", 0, "WChirho", shared_chirho),
            record_chirho("show", 1, "ZChirho", shared_chirho),
        ]),
        &occurrences_chirho,
        &HashMap::new(),
        &spans_chirho,
    );

    assert!(
        evidence_chirho.is_empty(),
        "an ambiguous span must identify nothing and must not fall through to a positional assignment: {evidence_chirho:?}"
    );
}

#[test]
fn unspanned_occurrences_still_take_their_records_in_order_chirho() {
    // The generated-occurrence path the desugarer still depends on: no spans at
    // all, counts agree, so the records apply in order.
    let first_chirho = CoreIdChirho(30);
    let second_chirho = CoreIdChirho(31);
    let mut occurrences_chirho = HashMap::new();
    occurrences_chirho.insert(first_chirho, ("show".to_string(), CoreIdChirho(3)));
    occurrences_chirho.insert(second_chirho, ("show".to_string(), CoreIdChirho(3)));

    let evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho(vec![
            record_chirho("show", 0, "WChirho", SpanChirho::DUMMY_CHIRHO),
            record_chirho("show", 1, "ZChirho", SpanChirho::DUMMY_CHIRHO),
        ]),
        &occurrences_chirho,
        &HashMap::new(),
        &HashMap::new(),
    );

    assert_eq!(evidence_chirho.len(), 2, "{evidence_chirho:?}");
}

#[test]
fn dummy_spans_are_not_an_identity_chirho() {
    // The deriving pass gives every reference it generates the SAME placeholder
    // span. Treating that as a shared span would make unrelated references look
    // like one another and refuse them all; measured, that renders a derived
    // `Show`'s Bool field as `1` in a native round trip. A dummy span counts as
    // no span, so these two take their records in order.
    let first_chirho = CoreIdChirho(40);
    let second_chirho = CoreIdChirho(41);
    let mut occurrences_chirho = HashMap::new();
    occurrences_chirho.insert(first_chirho, ("show".to_string(), CoreIdChirho(4)));
    occurrences_chirho.insert(second_chirho, ("show".to_string(), CoreIdChirho(4)));
    let mut spans_chirho = HashMap::new();
    spans_chirho.insert(first_chirho, SpanChirho::DUMMY_CHIRHO);
    spans_chirho.insert(second_chirho, SpanChirho::DUMMY_CHIRHO);

    let evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho(vec![
            record_chirho("show", 0, "Int", SpanChirho::DUMMY_CHIRHO),
            record_chirho("show", 1, "Bool", SpanChirho::DUMMY_CHIRHO),
        ]),
        &occurrences_chirho,
        &HashMap::new(),
        &spans_chirho,
    );

    assert_eq!(
        evidence_chirho.len(),
        2,
        "generated references sharing the placeholder span must still be served: {evidence_chirho:?}"
    );
}

#[test]
fn a_contested_genuine_span_is_refused_even_when_no_record_claims_it_chirho() {
    // gpt_chirho's counterexample (room #24227): two occurrences share a genuine
    // span X while the two unconsumed records carry a different genuine span Y.
    // The span join consumes nothing, and an earlier version of the fallback
    // admitted both occurrences because no record claimed X, then assigned their
    // proofs by position on a 2/2 count. A genuine span belongs to the span join
    // whether or not that join could use it.
    let first_chirho = CoreIdChirho(50);
    let second_chirho = CoreIdChirho(51);
    let span_x_chirho = span_chirho(300, 304);
    let span_y_chirho = span_chirho(400, 404);
    let mut occurrences_chirho = HashMap::new();
    occurrences_chirho.insert(first_chirho, ("show".to_string(), CoreIdChirho(5)));
    occurrences_chirho.insert(second_chirho, ("show".to_string(), CoreIdChirho(5)));
    let mut spans_chirho = HashMap::new();
    spans_chirho.insert(first_chirho, span_x_chirho);
    spans_chirho.insert(second_chirho, span_x_chirho);

    let evidence_chirho = join_occurrence_evidence_chirho(
        &infer_result_chirho(vec![
            record_chirho("show", 0, "WChirho", span_y_chirho),
            record_chirho("show", 1, "ZChirho", span_y_chirho),
        ]),
        &occurrences_chirho,
        &HashMap::new(),
        &spans_chirho,
    );

    assert!(
        evidence_chirho.is_empty(),
        "occurrences sharing a genuine span must not be served by position: {evidence_chirho:?}"
    );
}
