// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A record's authority is judged against the scheme its use took (row 698). A qualified
//! use keeps the qualified entry the checker resolved it under, and the join consults that
//! entry beside the bare name, so `P.print` keeps its Show proof beside a program's own
//! `print :: String -> IO ()`. A use without an entry is judged by its bare name as before,
//! and an entry is consulted, never trusted.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::provenance_chirho::{
    OccurrenceRoleChirho, OriginSupplyChirho, ProvenanceChirho,
};
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::infer_chirho::{
    MethodOccurrenceRecordChirho, own_dictionary_key_chirho,
};
use haskelujah_typing_chirho::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

use super::occurrence_join_chirho::{infer_result_chirho, record_chirho};
use crate::join_occurrence_evidence_chirho;

/// `print :: String -> IO ()`, a program's own, constrained by no class.
fn own_print_scheme_chirho() -> SchemeChirho {
    SchemeChirho::mono_chirho(TyChirho::fun_chirho(
        TyChirho::string_chirho(),
        TyChirho::io_chirho(TyChirho::unit_chirho()),
    ))
}

/// `Show a => a -> IO ()`, the Prelude's `print`.
fn show_print_scheme_chirho() -> SchemeChirho {
    let var_chirho = TyVarChirho(0);
    SchemeChirho {
        vars_chirho: vec![var_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Show".to_string(),
            ty_chirho: TyChirho::VarChirho(var_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::VarChirho(var_chirho),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        ),
    }
}

/// The proof a `print` occurrence takes from one Show record at the own key, both under one
/// origin, beside a program's own `print` and the given further environment entries. Show
/// declares only `show` here, so only the function-scheme arm can admit the record.
fn joined_print_chirho(
    resolved_entry_chirho: Option<&str>,
    entries_chirho: &[(&str, SchemeChirho)],
) -> Option<(String, String)> {
    let mut supply_chirho = OriginSupplyChirho::new_chirho();
    let origin_chirho = supply_chirho.fresh_chirho();
    let mut infer_chirho = infer_result_chirho(vec![MethodOccurrenceRecordChirho {
        origin_chirho: Some(origin_chirho),
        resolved_entry_chirho: resolved_entry_chirho.map(str::to_string),
        ..record_chirho(
            "print",
            0,
            &own_dictionary_key_chirho(Some(0)),
            SpanChirho::DUMMY_CHIRHO,
        )
    }]);
    infer_chirho
        .env_chirho
        .bind_chirho("print".to_string(), own_print_scheme_chirho());
    for (entry_chirho, scheme_chirho) in entries_chirho {
        infer_chirho
            .env_chirho
            .bind_chirho((*entry_chirho).to_string(), scheme_chirho.clone());
    }
    let occurrence_chirho = CoreIdChirho(30);
    join_occurrence_evidence_chirho(
        &infer_chirho,
        &HashMap::from([(occurrence_chirho, ("print".to_string(), CoreIdChirho(1)))]),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::from([(
            occurrence_chirho,
            ProvenanceChirho::new_chirho(origin_chirho, OccurrenceRoleChirho::Reference),
        )]),
        &Default::default(),
    )
    .evidence_chirho
    .remove(&occurrence_chirho)
}

#[test]
fn a_qualified_use_keeps_the_proof_its_resolved_entry_requires_chirho() {
    assert_eq!(
        joined_print_chirho(Some("P.print"), &[("P.print", show_print_scheme_chirho())]),
        Some(("Show".to_string(), own_dictionary_key_chirho(Some(0))))
    );
}

#[test]
fn a_use_without_an_entry_is_still_judged_by_its_bare_name_chirho() {
    // The bare `print` is the program's own, with no Show: the proof stays refused, even
    // with a Show-constrained `P.print` in scope.
    assert_eq!(
        joined_print_chirho(None, &[("P.print", show_print_scheme_chirho())]),
        None
    );
}

#[test]
fn an_entry_is_consulted_never_trusted_chirho() {
    // An entry whose scheme lacks the class proves nothing, and neither does an entry the
    // environment does not hold.
    assert_eq!(
        joined_print_chirho(Some("Q.print"), &[("Q.print", own_print_scheme_chirho())]),
        None
    );
    assert_eq!(joined_print_chirho(Some("R.print"), &[]), None);
}
