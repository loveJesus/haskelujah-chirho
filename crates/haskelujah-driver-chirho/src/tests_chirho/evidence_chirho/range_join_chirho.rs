// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The range channel carries a syntax proof only to its producer-marked occurrence.

use std::collections::HashMap;

use crate::evidence_join_chirho::join_range_evidence_chirho;
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_span_chirho::{ByteOffsetChirho, SourceMapChirho, SpanChirho};
use haskelujah_typing_chirho::infer_chirho::{InferCtxChirho, ReferenceEvidenceChirho};
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, TyChirho};

fn range_span_chirho() -> SpanChirho {
    let mut map_chirho = SourceMapChirho::new_chirho();
    let file_chirho = map_chirho.add_file_chirho("RangeChirho.hs", "[1..3]");
    SpanChirho::new_chirho(
        file_chirho,
        ByteOffsetChirho::new_chirho(0),
        ByteOffsetChirho::new_chirho(6),
    )
}

#[test]
fn only_the_marked_range_receives_its_enum_proof_chirho() {
    let span_chirho = range_span_chirho();
    let range_chirho = CoreIdChirho(0);
    let written_chirho = CoreIdChirho(1);
    let wrong_method_chirho = CoreIdChirho(2);
    let mut result_chirho = InferCtxChirho::new_chirho().finish_chirho();
    result_chirho.env_chirho.bind_chirho(
        "enumFromTo".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );
    result_chirho.reference_evidence_chirho.insert(
        span_chirho,
        vec![ReferenceEvidenceChirho {
            ground_pred_chirho: None,
            applied_head_chirho: None,
            class_name_chirho: "Enum".to_string(),
            ty_key_chirho: Some("Integer".to_string()),
            applied_ty_chirho: None,
        }],
    );
    let occurrences_chirho = HashMap::from([
        (range_chirho, ("enumFromTo".to_string(), CoreIdChirho(3))),
        (written_chirho, ("enumFromTo".to_string(), CoreIdChirho(4))),
        (wrong_method_chirho, ("show".to_string(), CoreIdChirho(5))),
    ]);
    let ranges_chirho = HashMap::from([
        (range_chirho, span_chirho),
        (wrong_method_chirho, span_chirho),
    ]);
    assert_eq!(
        join_range_evidence_chirho(&result_chirho, &occurrences_chirho, &ranges_chirho),
        HashMap::from([(range_chirho, ("Enum".to_string(), "Int".to_string()))]),
    );
}

#[test]
fn range_proofs_require_a_real_span_and_one_solved_enum_obligation_chirho() {
    let span_chirho = range_span_chirho();
    let range_chirho = CoreIdChirho(0);
    let occurrences_chirho =
        HashMap::from([(range_chirho, ("enumFromTo".to_string(), CoreIdChirho(1)))]);
    let ranges_chirho = HashMap::from([(range_chirho, span_chirho)]);
    for records_chirho in [
        Vec::new(),
        vec![ReferenceEvidenceChirho {
            ground_pred_chirho: None,
            applied_head_chirho: None,
            class_name_chirho: "Enum".to_string(),
            ty_key_chirho: None,
            applied_ty_chirho: None,
        }],
        vec![ReferenceEvidenceChirho {
            ground_pred_chirho: None,
            applied_head_chirho: None,
            class_name_chirho: "Num".to_string(),
            ty_key_chirho: Some("Int".to_string()),
            applied_ty_chirho: None,
        }],
        vec![
            ReferenceEvidenceChirho {
                ground_pred_chirho: None,
                applied_head_chirho: None,
                class_name_chirho: "Enum".to_string(),
                ty_key_chirho: Some("Int".to_string()),
                applied_ty_chirho: None,
            },
            ReferenceEvidenceChirho {
                ground_pred_chirho: None,
                applied_head_chirho: None,
                class_name_chirho: "Enum".to_string(),
                ty_key_chirho: Some("Char".to_string()),
                applied_ty_chirho: None,
            },
        ],
    ] {
        let mut result_chirho = InferCtxChirho::new_chirho().finish_chirho();
        result_chirho
            .reference_evidence_chirho
            .insert(span_chirho, records_chirho);
        assert!(
            join_range_evidence_chirho(&result_chirho, &occurrences_chirho, &ranges_chirho)
                .is_empty(),
        );
    }
    let mut result_chirho = InferCtxChirho::new_chirho().finish_chirho();
    result_chirho.reference_evidence_chirho.insert(
        SpanChirho::DUMMY_CHIRHO,
        vec![ReferenceEvidenceChirho {
            ground_pred_chirho: None,
            applied_head_chirho: None,
            class_name_chirho: "Enum".to_string(),
            ty_key_chirho: Some("Char".to_string()),
            applied_ty_chirho: None,
        }],
    );
    assert!(
        join_range_evidence_chirho(
            &result_chirho,
            &occurrences_chirho,
            &HashMap::from([(range_chirho, SpanChirho::DUMMY_CHIRHO)]),
        )
        .is_empty(),
    );
}
