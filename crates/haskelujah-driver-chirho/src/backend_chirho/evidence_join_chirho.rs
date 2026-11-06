// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Joining the checker's per-reference evidence to the desugarer's occurrence
//! ids. A reference is identified first by its PROVENANCE, the origin its
//! producer minted, which both phases read from the same AST node. What has no
//! provenance yet is identified by its source span; a placeholder span is not an
//! identity, and a genuine span claimed by more than one occurrence is refused
//! rather than guessed at. Which references get an occurrence id at all is
//! chosen here too, so the join and its population are read together.
//! workflow: language-features-chirho/dictionary-evidence-chirho
//! workflow: language-features-chirho/own-class-method-occurrences-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::provenance_chirho::{
    OccurrenceRoleChirho, OriginIdChirho, ProvenanceChirho,
};
use haskelujah_core_chirho::CoreIdChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho;

use crate::InferResultChirho;

/// This module's value binders own their spelling even when a class declares a namesake.
/// Class declarations are deliberately absent: their selectors keep the method route.
/// workflow: language-features-chirho/own-class-method-occurrences-chirho
pub(crate) fn source_value_names_chirho(module_chirho: &ModuleChirho) -> HashSet<String> {
    fn collect_pattern_chirho(pat_chirho: &PatChirho, names_chirho: &mut HashSet<String>) {
        match pat_chirho {
            PatChirho::VarChirho(name_chirho) => {
                names_chirho.insert(name_chirho.text_chirho().to_string());
            }
            PatChirho::AsChirho {
                name_chirho,
                pattern_chirho,
                ..
            } => {
                names_chirho.insert(name_chirho.text_chirho().to_string());
                collect_pattern_chirho(pattern_chirho, names_chirho);
            }
            PatChirho::ConChirho {
                args_chirho: patterns_chirho,
                ..
            }
            | PatChirho::TupleChirho {
                elements_chirho: patterns_chirho,
                ..
            }
            | PatChirho::ListChirho {
                elements_chirho: patterns_chirho,
                ..
            } => {
                for pattern_chirho in patterns_chirho {
                    collect_pattern_chirho(pattern_chirho, names_chirho);
                }
            }
            PatChirho::ParenChirho { inner_chirho, .. }
            | PatChirho::LazyChirho { inner_chirho, .. }
            | PatChirho::BangChirho { inner_chirho, .. } => {
                collect_pattern_chirho(inner_chirho, names_chirho)
            }
            PatChirho::TypeAnnotChirho { pat_chirho, .. }
            | PatChirho::ViewChirho { pat_chirho, .. } => {
                collect_pattern_chirho(pat_chirho, names_chirho)
            }
            PatChirho::InfixConChirho {
                left_chirho,
                right_chirho,
                ..
            } => {
                collect_pattern_chirho(left_chirho, names_chirho);
                collect_pattern_chirho(right_chirho, names_chirho);
            }
            PatChirho::RecordChirho { fields_chirho, .. } => {
                for field_chirho in fields_chirho {
                    collect_pattern_chirho(&field_chirho.pattern_chirho, names_chirho);
                }
            }
            PatChirho::LitChirho(_)
            | PatChirho::WildcardChirho(_)
            | PatChirho::NegChirho { .. } => {}
        }
    }
    let mut names_chirho = HashSet::new();
    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                names_chirho.insert(name_chirho.text_chirho().to_string());
            }
            DeclChirho::PatBindChirho { pat_chirho, .. } => {
                collect_pattern_chirho(pat_chirho, &mut names_chirho)
            }
            _ => {}
        }
    }
    names_chirho
}

/// Evidence-threading P2b: standard single-param class methods and constrained
/// Prelude functions whose free references get per-occurrence ids so typing
/// evidence can drive dispatch.
/// Workflow: `spec-chirho/workflows-chirho/print-show-evidence-chirho.md`.
/// Monad-chain operators (>>=, >>, return, pure, fail) are deliberately
/// EXCLUDED — they have dedicated dispatch machinery and INV-001 protection.
const EVIDENCE_METHOD_NAMES_CHIRHO: &[&str] = &[
    "==",
    "/=",
    "<",
    "<=",
    ">",
    ">=",
    "compare",
    "max",
    "min",
    "+",
    "-",
    "*",
    // Fractional's `/` (row 609): its occurrence carries the checker's proof, so `(/ 2)` and a
    // polymorphic `x / 2` reach the key's own row rather than an operand guess.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    "/",
    "negate",
    "abs",
    "signum",
    "fromInteger",
    "div",
    "mod",
    "quot",
    "rem",
    "show",
    "print",
    "sum",
    "product",
    "maximum",
    "minimum",
    "mfix",
    "read",
    // `toEnum :: Enum a => Int -> a` carries its class variable only in the
    // result, so guessing from the argument always chooses Int. `fromEnum`
    // likewise needs its proven instance for bare uses and opaque arguments.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    "toEnum",
    "fromEnum",
    // A locally generalized alias such as `where nextChirho = succ` gets no
    // dictionary parameter in Core. Its one concrete instantiation is already
    // proved by typing, but the proof can reach the pass only through a minted
    // occurrence (row 607). `pred` is the symmetric Enum method.
    "succ",
    "pred",
    // The four enumerations are Enum methods too (row 671). Each reference takes an
    // occurrence of its own, so its proof reaches the dictionary pass by identity, never
    // by a per-name count beside a program-owned namesake.
    "enumFrom",
    "enumFromThen",
    "enumFromTo",
    "enumFromThenTo",
    // Like `toEnum`, the Bounded methods carry their class variable only in
    // the result. Their occurrence proof keeps a result-only polymorphic
    // binding on its own dictionary instead of defaulting it to Bounded Int.
    "minBound",
    "maxBound",
];

/// The monad-chain operators the standard set leaves out (INV-001). No rule
/// here opts them in, not even for a class this module declares itself.
const INV_001_NAMES_CHIRHO: [&str; 5] = [">>=", ">>", "return", "pure", "fail"];

/// The two INV-001 operators minted all the same (row 697). Their only accepted proof is the
/// exact predicate the checker found for the use, an indexed own key joined by the use's own
/// provenance (`admits_operator_proof_chirho`). Any other use goes back to its canonical id and
/// INV-001's path. `return`, `pure` and `fail` stay unminted.
/// workflow: monadic-dispatch-chirho
const OWN_PROOF_OPERATORS_CHIRHO: [&str; 2] = [">>=", ">>"];

/// Whether `name_chirho` is one of the two operators that take only an indexed own proof.
fn is_own_proof_operator_chirho(name_chirho: &str) -> bool {
    OWN_PROOF_OPERATORS_CHIRHO.contains(&name_chirho)
}

/// Whether provenance in `role_chirho` identifies a use of `name_chirho` to this join: a
/// written reference, or the `>>=` (Bind) or `>>` (Then) a do statement selected (row 697).
/// workflow: monadic-dispatch-chirho
pub(crate) fn joins_provenance_role_chirho(
    name_chirho: &str,
    role_chirho: OccurrenceRoleChirho,
) -> bool {
    match role_chirho {
        OccurrenceRoleChirho::Reference => true,
        OccurrenceRoleChirho::Bind => name_chirho == ">>=",
        OccurrenceRoleChirho::Then => name_chirho == ">>",
        _ => false,
    }
}

/// Whether a record for `name_chirho` proved at `class_chirho` and `key_chirho` may prove its
/// occurrence. Every record may, except a `>>=` or `>>` record, which proves only the exact
/// predicate that types its use: `Monad` at an indexed own key (`$own:i`), the binding's own
/// dictionary parameter for that predicate.
///
/// Three kinds of key prove nothing here, and the use keeps its canonical id and INV-001's
/// path: a ground key (`IO`, `Maybe`), a user monad, and a bare `$own` that a superclass
/// entailed. The bare superclass case is an unserved runtime boundary, not a compatible pass
/// (row 697).
/// workflow: monadic-dispatch-chirho
pub(crate) fn admits_operator_proof_chirho(
    name_chirho: &str,
    class_chirho: &str,
    key_chirho: &str,
) -> bool {
    if !is_own_proof_operator_chirho(name_chirho) {
        return true;
    }
    let bare_own_chirho = haskelujah_typing_chirho::infer_chirho::own_dictionary_key_chirho(None);
    class_chirho == "Monad"
        && key_chirho
            .strip_prefix(bare_own_chirho.as_str())
            .and_then(|rest_chirho| rest_chirho.strip_prefix(':'))
            .is_some_and(|index_chirho| {
                !index_chirho.is_empty()
                    && index_chirho
                        .bytes()
                        .all(|byte_chirho| byte_chirho.is_ascii_digit())
            })
}

/// The names whose every free reference the desugarer gives an occurrence id of
/// its own, so the checker's proof for that one use reaches the dictionary pass:
/// the standard set, and each method of a single-parameter class the compiling
/// module declares. Sharing one id for every use, a user method had its
/// dictionary guessed from sibling arguments: `map neededChirho [[1 :: Int]]`
/// ran `NeedsChirho Int`'s method on each list.
///
/// Boundaries, each keeping its shared id: a seeded standard class's methods
/// outside the standard set (`fmap`, `<>`, `toList`), a class imported from
/// another module (its rows live in that module's Core), a multi-parameter
/// class, and the INV-001 names. The exceptions are `>>=` and `>>`, minted for
/// their indexed own proof alone (`OWN_PROOF_OPERATORS_CHIRHO`, row 697).
/// Shadows stay the desugarer's: a lexical, top-level or field binding is found
/// in scope before any occurrence is minted, and an occurrence the join proves
/// nothing for is restored to its canonical id.
/// workflow: language-features-chirho/own-class-method-occurrences-chirho
pub(crate) fn method_occurrence_names_chirho(module_chirho: &ModuleChirho) -> HashSet<String> {
    let own_methods_chirho = module_chirho
        .decls_chirho
        .iter()
        .flat_map(|decl_chirho| match decl_chirho {
            DeclChirho::ClassDeclChirho {
                type_vars_chirho,
                methods_chirho,
                ..
            } if type_vars_chirho.len() == 1 => methods_chirho.as_slice(),
            _ => &[],
        })
        .map(|method_chirho| method_chirho.name_chirho.text_chirho().to_string());
    EVIDENCE_METHOD_NAMES_CHIRHO
        .iter()
        .map(|name_chirho| name_chirho.to_string())
        .chain(own_methods_chirho)
        .filter(|name_chirho| !INV_001_NAMES_CHIRHO.contains(&name_chirho.as_str()))
        .chain(
            OWN_PROOF_OPERATORS_CHIRHO
                .iter()
                .map(|name_chirho| name_chirho.to_string()),
        )
        .collect()
}

/// One record's proof for an occurrence, its class and type key, with the ground application
/// its predicate resolved to, if any.
type AppliedProofChirho = ((String, String), Option<TyChirho>);

/// What the occurrence join proves: each occurrence's class and type key, and, where the
/// checker recorded one, the ground application its predicate resolved to (row 694, M1).
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(crate) struct OccurrenceJoinChirho {
    pub(crate) evidence_chirho: HashMap<CoreIdChirho, (String, String)>,
    pub(crate) applied_tys_chirho: HashMap<CoreIdChirho, TyChirho>,
}

/// Join the checker's occurrence records to the desugarer's occurrence ids.
///
/// First by provenance: a record carrying an origin belongs to the occurrence
/// the desugarer minted for that same use, and to nothing else. An occurrence
/// with provenance whose record is missing or conflicting receives no evidence,
/// and it is never rescued by span or by position.
///
/// Then by span, for occurrences without provenance. The two phases number
/// references differently (declaration order in the desugarer, inference order
/// in the checker, with instance bodies last), so position is not an identity
/// and matching by it exchanged evidence between references.
///
/// What remains positional serves only the population with no identity at all:
/// occurrences with neither provenance nor a genuine span, and records with no
/// origin, taken in order under a count guard.
///
/// Each occurrence also takes the ground application its proof's predicate resolved to,
/// where the checker recorded one (row 694, M1): only from the very record that proves it,
/// and from an origin's records only when every one proving that occurrence agrees.
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
    occurrence_provenance_chirho: &HashMap<CoreIdChirho, ProvenanceChirho>,
) -> OccurrenceJoinChirho {
    let mut applied_chirho: HashMap<CoreIdChirho, TyChirho> = HashMap::new();
    // Literal evidence joins by span, exactly: the checker solved this very
    // literal. It is inserted first so the per-name ordinal join below never
    // overrides it. The engine's body-backed rows are keyed "Int"; the checker
    // defaults an unconstrained literal to "Integer" per the Report.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    let mut evidence_chirho: std::collections::HashMap<
        haskelujah_core_chirho::CoreIdChirho,
        (String, String),
    > = std::collections::HashMap::new();
    // A literal with provenance takes the evidence of its own origin or none; a
    // literal without it (not yet stamped) is still found by its span.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    for (occ_id_chirho, span_chirho) in literal_occurrence_spans_chirho {
        let record_chirho = match occurrence_provenance_chirho.get(occ_id_chirho) {
            Some(provenance_chirho) => infer_result_chirho
                .literal_evidence_by_origin_chirho
                .get(&provenance_chirho.origin_chirho),
            None => infer_result_chirho.literal_evidence_chirho.get(span_chirho),
        };
        if let Some(record_chirho) = record_chirho {
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
    let has_provenance_chirho =
        |occ_id_chirho: &CoreIdChirho| occurrence_provenance_chirho.contains_key(occ_id_chirho);
    for (occ_id_chirho, (name_chirho, _canon_chirho)) in method_occurrences_chirho {
        if has_provenance_chirho(occ_id_chirho) {
            continue;
        }
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
            let entry_requires_class_chirho = |entry_chirho: &str| {
                infer_result_chirho
                    .env_chirho
                    .lookup_chirho(entry_chirho)
                    .is_some_and(|scheme_chirho| {
                        scheme_chirho.preds_chirho.iter().any(|pred_chirho| {
                            pred_chirho.class_name_chirho == record_chirho.class_name_chirho
                        })
                    })
            };
            // The function's own scheme is the one its bare name binds, as before, or the
            // qualified entry the checker resolved the use under: `P.print` keeps its Show
            // proof beside a program's own `print :: String -> IO ()` (row 698). The entry
            // is consulted, never trusted; a use without one is judged by its bare name.
            let function_requires_class_chirho = entry_requires_class_chirho(name_chirho.as_str())
                || record_chirho
                    .resolved_entry_chirho
                    .as_deref()
                    .is_some_and(entry_requires_class_chirho);
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
    // By provenance. The desugarer may desugar one source use more than once
    // (each copy is that same use, at that same type), so every occurrence of an
    // origin takes the proof; a record for another name (the `/=` rewrite's
    // `==`) is not this occurrence's proof.
    // workflow: language-features-chirho/dictionary-evidence-chirho
    let mut occurrences_by_origin_chirho: HashMap<OriginIdChirho, Vec<(CoreIdChirho, &String)>> =
        HashMap::new();
    for (occ_id_chirho, provenance_chirho) in occurrence_provenance_chirho {
        if let Some((name_chirho, _canon_chirho)) = method_occurrences_chirho.get(occ_id_chirho)
            && joins_provenance_role_chirho(name_chirho, provenance_chirho.role_chirho)
        {
            occurrences_by_origin_chirho
                .entry(provenance_chirho.origin_chirho)
                .or_default()
                .push((*occ_id_chirho, name_chirho));
        }
    }
    let mut proofs_by_occurrence_chirho: HashMap<CoreIdChirho, Vec<(String, String)>> =
        HashMap::new();
    let mut applied_by_occurrence_chirho: HashMap<CoreIdChirho, Vec<AppliedProofChirho>> =
        HashMap::new();
    for (record_index_chirho, record_chirho) in infer_result_chirho
        .method_occurrences_chirho
        .iter()
        .enumerate()
    {
        let Some(origin_chirho) = record_chirho.origin_chirho else {
            continue;
        };
        if !record_is_authoritative_chirho(record_chirho) {
            continue;
        }
        // `>>=` and `>>` keep only an indexed own proof, before the record contributes
        // anything to its occurrence (row 697).
        if !admits_operator_proof_chirho(
            &record_chirho.name_chirho,
            &record_chirho.class_name_chirho,
            &record_chirho.ty_key_chirho,
        ) {
            continue;
        }
        let Some(candidates_chirho) = occurrences_by_origin_chirho.get(&origin_chirho) else {
            continue;
        };
        if candidates_chirho
            .iter()
            .any(|(_occ_id_chirho, name_chirho)| **name_chirho != record_chirho.name_chirho)
        {
            continue;
        }
        consumed_records_chirho.insert(record_index_chirho);
        for (occ_id_chirho, _name_chirho) in candidates_chirho {
            identified_occurrences_chirho.insert(*occ_id_chirho);
            proofs_by_occurrence_chirho
                .entry(*occ_id_chirho)
                .or_default()
                .push((
                    record_chirho.class_name_chirho.clone(),
                    backed_key_chirho(record_chirho),
                ));
            applied_by_occurrence_chirho
                .entry(*occ_id_chirho)
                .or_default()
                .push((
                    (
                        record_chirho.class_name_chirho.clone(),
                        backed_key_chirho(record_chirho),
                    ),
                    record_chirho.applied_ty_chirho.clone(),
                ));
        }
    }
    for (occ_id_chirho, proofs_chirho) in &proofs_by_occurrence_chirho {
        if let Some(proof_chirho) = single_proof_chirho(proofs_chirho) {
            let chosen_chirho = evidence_chirho
                .entry(*occ_id_chirho)
                .or_insert(proof_chirho)
                .clone();
            if let Some(applied_ty_chirho) = agreed_applied_ty_chirho(
                applied_by_occurrence_chirho
                    .get(occ_id_chirho)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
                &chosen_chirho,
            ) {
                applied_chirho.insert(*occ_id_chirho, applied_ty_chirho);
            }
        }
    }
    // By span, for the occurrences without provenance, and only from records
    // without an origin: both sides of a legacy join are legacy. A record with an
    // origin belongs to that origin's occurrence whether or not one exists, so
    // an origin-owned proof never reaches another occurrence through its span
    // (gpt_chirho review F1, #24598).
    for (record_index_chirho, record_chirho) in infer_result_chirho
        .method_occurrences_chirho
        .iter()
        .enumerate()
    {
        if record_chirho.origin_chirho.is_some() {
            continue;
        }
        // `>>=` and `>>` are proved by provenance alone, never by a span (row 697).
        if is_own_proof_operator_chirho(&record_chirho.name_chirho) {
            continue;
        }
        if consumed_records_chirho.contains(&record_index_chirho) {
            continue;
        }
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
        if let std::collections::hash_map::Entry::Vacant(vacant_chirho) =
            evidence_chirho.entry(*occ_id_chirho)
        {
            vacant_chirho.insert((
                record_chirho.class_name_chirho.clone(),
                backed_key_chirho(record_chirho),
            ));
            if let Some(applied_ty_chirho) = &record_chirho.applied_ty_chirho {
                applied_chirho.insert(*occ_id_chirho, applied_ty_chirho.clone());
            }
        }
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
        // `>>=` and `>>` are proved by provenance alone, never by position (row 697).
        if is_own_proof_operator_chirho(name_chirho) {
            continue;
        }
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
        // An occurrence with provenance belongs to the provenance join alone, and
        // a record with an origin belongs to its own occurrence alone: the
        // fallback serves only what has no identity at all (gpt_chirho #24575).
        let unidentified_chirho: Vec<haskelujah_core_chirho::CoreIdChirho> = ids_chirho
            .iter()
            .filter(|id_chirho| {
                !identified_occurrences_chirho.contains(id_chirho)
                    && !has_provenance_chirho(id_chirho)
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
                    && record_chirho.origin_chirho.is_none()
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
            if let std::collections::hash_map::Entry::Vacant(vacant_chirho) =
                evidence_chirho.entry(*occ_id_chirho)
            {
                vacant_chirho.insert((
                    record_chirho.class_name_chirho.clone(),
                    backed_key_chirho(record_chirho),
                ));
                if let Some(applied_ty_chirho) = &record_chirho.applied_ty_chirho {
                    applied_chirho.insert(*occ_id_chirho, applied_ty_chirho.clone());
                }
            }
        }
    }
    OccurrenceJoinChirho {
        evidence_chirho,
        applied_tys_chirho: applied_chirho,
    }
}

/// The applied type every record proving `chosen_chirho` for one occurrence agrees on; `None`
/// when one carries none or two differ.
fn agreed_applied_ty_chirho(
    candidates_chirho: &[AppliedProofChirho],
    chosen_chirho: &(String, String),
) -> Option<TyChirho> {
    let mut agreed_chirho: Option<&TyChirho> = None;
    for (proof_chirho, applied_ty_chirho) in candidates_chirho {
        if proof_chirho != chosen_chirho {
            continue;
        }
        let applied_ty_chirho = applied_ty_chirho.as_ref()?;
        match agreed_chirho {
            Some(seen_chirho) if seen_chirho != applied_ty_chirho => return None,
            _ => agreed_chirho = Some(applied_ty_chirho),
        }
    }
    agreed_chirho.cloned()
}

/// Join the checker's per-predicate evidence for constrained references to the
/// desugarer's reference occurrences: by provenance, or by span for an
/// occurrence that has no provenance. An occurrence with provenance whose origin
/// has no agreed evidence receives none; its span is not consulted.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(crate) fn join_reference_evidence_chirho(
    infer_result_chirho: &InferResultChirho,
    reference_occurrence_spans_chirho: &HashMap<CoreIdChirho, haskelujah_span_chirho::SpanChirho>,
    occurrence_provenance_chirho: &HashMap<CoreIdChirho, ProvenanceChirho>,
) -> HashMap<CoreIdChirho, Vec<haskelujah_typing_chirho::infer_chirho::ReferenceEvidenceChirho>> {
    reference_occurrence_spans_chirho
        .iter()
        .filter_map(|(occ_id_chirho, span_chirho)| {
            let records_chirho = match occurrence_provenance_chirho.get(occ_id_chirho) {
                Some(provenance_chirho)
                    if provenance_chirho.role_chirho == OccurrenceRoleChirho::Reference =>
                {
                    infer_result_chirho
                        .reference_evidence_by_origin_chirho
                        .get(&provenance_chirho.origin_chirho)?
                }
                Some(_other_role_chirho) => return None,
                None => infer_result_chirho
                    .reference_evidence_chirho
                    .get(span_chirho)?,
            };
            Some((
                *occ_id_chirho,
                records_chirho
                    .iter()
                    .map(|record_chirho| {
                        // The engine's body-backed rows are keyed "Int"; the
                        // checker defaults an ambiguous numeric to "Integer".
                        let key_chirho = record_chirho.ty_key_chirho.as_deref().map(|key_chirho| {
                            if key_chirho == "Integer" {
                                "Int".to_string()
                            } else {
                                key_chirho.to_string()
                            }
                        });
                        // The applied type and a multi-parameter head ride along unchanged:
                        // they are the checker's structure, which the dictionary pass
                        // instantiates (row 694).
                        haskelujah_typing_chirho::infer_chirho::ReferenceEvidenceChirho {
                            applied_head_chirho: record_chirho.applied_head_chirho.clone(),
                            class_name_chirho: record_chirho.class_name_chirho.clone(),
                            ty_key_chirho: key_chirho,
                            applied_ty_chirho: record_chirho.applied_ty_chirho.clone(),
                        }
                    })
                    .collect(),
            ))
        })
        .collect()
}

/// Join the compiler-selected Enum method of each range to that range's proof.
/// A source-written name never enters this map; it keeps the ordinary ownership
/// checks even when its spelling is an enumeration method. Ranges have no AST
/// origin yet, so only a genuine span and one solved Enum predicate identify one.
/// workflow: language-features-chirho/dictionary-evidence-chirho
pub(crate) fn join_range_evidence_chirho(
    infer_result_chirho: &InferResultChirho,
    occurrences_chirho: &HashMap<CoreIdChirho, (String, CoreIdChirho)>,
    ranges_chirho: &HashMap<CoreIdChirho, haskelujah_span_chirho::SpanChirho>,
) -> HashMap<CoreIdChirho, (String, String)> {
    ranges_chirho
        .iter()
        .filter_map(|(id_chirho, span_chirho)| {
            use haskelujah_span_chirho::{FileIdChirho, SpanChirho};
            if *span_chirho == SpanChirho::DUMMY_CHIRHO
                || span_chirho.file_id_chirho() == FileIdChirho::SYNTHETIC_CHIRHO
            {
                return None;
            }
            let (method_chirho, _canonical_chirho) = occurrences_chirho.get(id_chirho)?;
            if !matches!(
                method_chirho.as_str(),
                "enumFrom" | "enumFromThen" | "enumFromTo" | "enumFromThenTo"
            ) {
                return None;
            }
            let [proof_chirho] = infer_result_chirho
                .reference_evidence_chirho
                .get(span_chirho)?
                .as_slice()
            else {
                return None;
            };
            if proof_chirho.class_name_chirho != "Enum" {
                return None;
            }
            let key_chirho = proof_chirho.ty_key_chirho.as_deref()?;
            let key_chirho = if key_chirho == "Integer" {
                "Int"
            } else {
                key_chirho
            };
            Some((*id_chirho, ("Enum".to_string(), key_chirho.to_string())))
        })
        .collect()
}

/// The proof an occurrence takes from the records of its own origin: the first
/// authoritative one in checker order, unless some class was proved at two
/// different types, which is a conflict and proves nothing.
fn single_proof_chirho(proofs_chirho: &[(String, String)]) -> Option<(String, String)> {
    let conflicting_chirho = proofs_chirho.iter().any(|(class_chirho, key_chirho)| {
        proofs_chirho
            .iter()
            .any(|(other_class_chirho, other_key_chirho)| {
                class_chirho == other_class_chirho && key_chirho != other_key_chirho
            })
    });
    if conflicting_chirho {
        return None;
    }
    proofs_chirho.first().cloned()
}
