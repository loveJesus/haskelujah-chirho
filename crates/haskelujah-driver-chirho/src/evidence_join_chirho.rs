// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Joining the checker's per-reference evidence to the desugarer's occurrence
//! ids. A reference is identified by its SOURCE SPAN; a placeholder span is not
//! an identity, and a genuine span claimed by more than one occurrence is refused
//! rather than guessed at.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use crate::InferResultChirho;

/// Join the checker's occurrence records to the desugarer's occurrence ids.
///
/// A record is matched to the reference it describes BY SPAN. The two phases
/// number references differently (declaration order in the desugarer, inference
/// order in the checker, with instance bodies last), so position is not an
/// identity and matching by it exchanged evidence between references.
///
/// What remains positional serves only references nothing can identify yet: the
/// desugarer mints occurrences at several sites and only the variable-reference
/// site records a span, and the deriving pass gives every reference it generates
/// the same placeholder. Those take unconsumed records in order, count-guarded.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(crate) fn join_occurrence_evidence_chirho(
    infer_result_chirho: &InferResultChirho,
    method_occurrences_chirho: &std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        (String, haskelujah_core_chirho::CoreIdChirho),
    >,
    literal_occurrence_spans_chirho: &std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        haskelujah_span_chirho::SpanChirho,
    >,
    reference_occurrence_spans_chirho: &std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        haskelujah_span_chirho::SpanChirho,
    >,
) -> std::collections::HashMap<haskelujah_core_chirho::CoreIdChirho, (String, String)> {
    // Literal evidence joins by span, exactly: the checker solved this very
    // literal. It is inserted first so the per-name ordinal join below never
    // overrides it. The engine's body-backed rows are keyed "Int"; the checker
    // defaults an unconstrained literal to "Integer" per the Report.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    let mut evidence_chirho: std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        (String, String),
    > = std::collections::HashMap::new();
    for (occ_id_chirho, span_chirho) in literal_occurrence_spans_chirho {
        if let Some(record_chirho) = infer_result_chirho.literal_evidence_chirho.get(span_chirho) {
            let ty_key_chirho = if record_chirho.ty_key_chirho == "Integer" {
                "Int".to_string()
            } else {
                record_chirho.ty_key_chirho.clone()
            };
            evidence_chirho.insert(
                *occ_id_chirho,
                (record_chirho.class_name_chirho.clone(), ty_key_chirho),
            );
        }
    }
    let mut occ_ids_by_name_chirho: std::collections::HashMap<
        String,
        Vec<haskelujah_core_chirho::CoreIdChirho>,
    > = std::collections::HashMap::new();
    for (occ_id_chirho, (name_chirho, _canon_chirho)) in method_occurrences_chirho {
        occ_ids_by_name_chirho
            .entry(name_chirho.clone())
            .or_default()
            .push(*occ_id_chirho);
    }
    for ids_chirho in occ_ids_by_name_chirho.values_mut() {
        ids_chirho.sort_by_key(|id_chirho| id_chirho.0);
    }
    // A method occurrence is identified by its SOURCE SPAN, not by its position.
    // The checker's ordinal is a per-name counter in inference-visit order, and
    // instance method bodies are inferred last (phase 3e), while the desugarer
    // numbers occurrences in declaration order. Joining those by position made
    // `main`'s `show` and an instance body's `show` exchange evidence, so the
    // same program printed `W3` or `WChirho 3` depending only on which
    // declaration came first (measured 2026-09-19). A genuine span that belongs
    // to more than one occurrence, or to another name, yields no verdict here,
    // and it does NOT fall through to the positional path below either: a genuine
    // span belongs to this join whether or not this join could use it.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    // A DUMMY or synthetic span is not an identity: the deriving pass gives every
    // reference it generates the same placeholder, so indexing by it would make
    // unrelated references look like one another. Such an occurrence is treated
    // as having no span at all, here and in the pool below.
    let identifying_span_chirho = |span_chirho: &haskelujah_span_chirho::SpanChirho| {
        *span_chirho != haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO
            && span_chirho.file_id_chirho()
                != haskelujah_span_chirho::FileIdChirho::SYNTHETIC_CHIRHO
    };
    let occurrence_span_chirho = |occ_id_chirho: &haskelujah_core_chirho::CoreIdChirho| {
        reference_occurrence_spans_chirho
            .get(occ_id_chirho)
            .filter(|span_chirho| identifying_span_chirho(span_chirho))
            .copied()
    };
    let mut occ_ids_by_span_chirho: std::collections::HashMap<
        haskelujah_span_chirho::SpanChirho,
        Vec<(haskelujah_core_chirho::CoreIdChirho, String)>,
    > = std::collections::HashMap::new();
    for (occ_id_chirho, (name_chirho, _canon_chirho)) in method_occurrences_chirho {
        if let Some(span_chirho) = occurrence_span_chirho(occ_id_chirho) {
            occ_ids_by_span_chirho
                .entry(span_chirho)
                .or_default()
                .push((*occ_id_chirho, name_chirho.clone()));
        }
    }
    let record_is_authoritative_chirho =
        |record_chirho: &haskelujah_typing_chirho::infer_chirho::MethodOccurrenceRecordChirho| {
            // A record is authoritative only when its class either declares
            // this method (`==` -> Eq) or constrains this function's own
            // scheme (`print :: Show a => ...`). Incidental predicates never
            // drive occurrence dispatch.
            let name_chirho = &record_chirho.name_chirho;
            let class_declares_reference_chirho = infer_result_chirho
                .class_env_chirho
                .classes_chirho
                .get(&record_chirho.class_name_chirho)
                .is_some_and(|class_chirho| class_chirho.methods_chirho.contains_key(name_chirho));
            let function_requires_class_chirho = infer_result_chirho
                .env_chirho
                .lookup_chirho(name_chirho)
                .is_some_and(|scheme_chirho| {
                    scheme_chirho.preds_chirho.iter().any(|pred_chirho| {
                        pred_chirho.class_name_chirho == record_chirho.class_name_chirho
                    })
                });
            class_declares_reference_chirho || function_requires_class_chirho
        };
    let backed_key_chirho =
        |record_chirho: &haskelujah_typing_chirho::infer_chirho::MethodOccurrenceRecordChirho| {
            // The engine's body-backed instance rows are keyed "Int"; typing
            // defaults ambiguous numerics to "Integer" per the Report. Map
            // the evidence key onto the backed row at this boundary only.
            if record_chirho.ty_key_chirho == "Integer" {
                "Int".to_string()
            } else {
                record_chirho.ty_key_chirho.clone()
            }
        };
    let mut consumed_records_chirho: std::collections::HashSet<usize> =
        std::collections::HashSet::new();
    let mut identified_occurrences_chirho: std::collections::HashSet<
        haskelujah_core_chirho::CoreIdChirho,
    > = std::collections::HashSet::new();
    for (record_index_chirho, record_chirho) in infer_result_chirho
        .method_occurrences_chirho
        .iter()
        .enumerate()
    {
        if !record_is_authoritative_chirho(record_chirho) {
            continue;
        }
        if !identifying_span_chirho(&record_chirho.span_chirho) {
            continue;
        }
        let Some(candidates_chirho) = occ_ids_by_span_chirho.get(&record_chirho.span_chirho) else {
            continue;
        };
        let [(occ_id_chirho, occ_name_chirho)] = candidates_chirho.as_slice() else {
            continue;
        };
        if occ_name_chirho != &record_chirho.name_chirho {
            continue;
        }
        consumed_records_chirho.insert(record_index_chirho);
        identified_occurrences_chirho.insert(*occ_id_chirho);
        evidence_chirho.entry(*occ_id_chirho).or_insert_with(|| {
            (
                record_chirho.class_name_chirho.clone(),
                backed_key_chirho(record_chirho),
            )
        });
    }
    // What remains of the old positional join, and the three boundaries it
    // respects. The desugarer mints method occurrences at several sites and only
    // the variable-reference site records a span, so a GENERATED occurrence (a
    // literal's `fromInteger`, an operator's `+`) carries none, and the deriving
    // pass gives every reference it generates one placeholder span. Those are the
    // only occurrences this path may fill.
    //   1. It may only use a record the span join did NOT consume: a proof belongs
    //      to one reference, and vacancy in the consumer map is not ownership of
    //      it (gpt_chirho, room #24056).
    //   2. An occurrence carrying a GENUINE span is never eligible, whether or not
    //      the span join could use that span (gpt_chirho's counterexample, room
    //      #24227).
    //   3. A placeholder span is not a span at all, on either side.
    // The count guard compares the unidentified occurrences against the
    // unconsumed records, not the totals.
    // MEASURED four ways: deleting this path outright turns 19 driver tests red
    // (do-notation, mdo, deriving, MPTC, fundeps, six native round trips);
    // restricting it to names whose occurrences ALL lack spans, and separately
    // refusing every occurrence the span join had not matched, each regress a
    // derived Show of a Bool field in a native round trip to `MixChirho 2 1`,
    // where main prints `MixChirho 2 True`; and the shape below keeps both
    // families correct. It goes away when every mint site carries its own
    // occurrence provenance, which is its own brick.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    for (name_chirho, ids_chirho) in &occ_ids_by_name_chirho {
        // Eligible here: an occurrence with NO identifying span at all, after the
        // identified ones are excluded. Anything carrying a genuine span belongs
        // to the span join, whether or not that join could use it: admitting a
        // contested span merely because no record happened to claim it would hand
        // two references their proofs by position, which is the boundary this
        // join exists to hold (gpt_chirho's counterexample, room #24227 — two
        // occurrences sharing a genuine span X, two unconsumed records carrying a
        // different genuine span Y).
        // A placeholder span is not a span for this purpose, so the deriving
        // pass's generated references still qualify; that is what keeps a derived
        // `Show`'s Bool field from rendering as `1` in a native round trip.
        let unidentified_chirho: Vec<haskelujah_core_chirho::CoreIdChirho> = ids_chirho
            .iter()
            .filter(|id_chirho| {
                !identified_occurrences_chirho.contains(id_chirho)
                    && occurrence_span_chirho(id_chirho).is_none()
            })
            .copied()
            .collect();
        let unconsumed_chirho: Vec<
            &haskelujah_typing_chirho::infer_chirho::MethodOccurrenceRecordChirho,
        > = infer_result_chirho
            .method_occurrences_chirho
            .iter()
            .enumerate()
            .filter(|(index_chirho, record_chirho)| {
                &record_chirho.name_chirho == name_chirho
                    && !consumed_records_chirho.contains(index_chirho)
                    && record_is_authoritative_chirho(record_chirho)
            })
            .map(|(_index_chirho, record_chirho)| record_chirho)
            .collect();
        if unidentified_chirho.is_empty() || unconsumed_chirho.len() != unidentified_chirho.len() {
            continue;
        }
        for (occ_id_chirho, record_chirho) in unidentified_chirho.iter().zip(unconsumed_chirho) {
            // The engine's body-backed instance rows are keyed "Int"; typing
            // defaults ambiguous numerics to "Integer" per the Report. Map
            // the evidence key onto the backed row at this boundary only.
            evidence_chirho.entry(*occ_id_chirho).or_insert_with(|| {
                (
                    record_chirho.class_name_chirho.clone(),
                    backed_key_chirho(record_chirho),
                )
            });
        }
    }
    evidence_chirho
}
