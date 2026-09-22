// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The checker's records name the occurrences the producers minted: through the
//! real front end, every record that carries an origin carries the origin of an
//! occurrence of that very name, uses of one method at two types keep two
//! origins, an occurrence with two predicates keeps both under one origin, and
//! the references deriving generates are identified as well.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use std::collections::{HashMap, HashSet};

use haskelujah_ast_chirho::occurrences_chirho::{OccurrenceMutChirho, visit_decl_chirho};
use haskelujah_ast_chirho::provenance_chirho::OriginIdChirho;
use haskelujah_span_chirho::SourceMapChirho;
use haskelujah_syntax_chirho::SourceFileChirho;

fn frontend_chirho(source_chirho: &str) -> crate::FrontendResultChirho {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let source_file_chirho = SourceFileChirho::from_source_map_chirho(
        &mut source_map_chirho,
        "OccurrenceRecordsChirho.hs",
        source_chirho,
    );
    crate::run_frontend_chirho(
        source_chirho,
        source_file_chirho.file_id_chirho(),
        &haskelujah_naming_chirho::builtin_module_ifaces_chirho(),
        &HashMap::new(),
    )
    .unwrap_or_else(|diag_chirho| panic!("front end rejected the control: {diag_chirho:?}"))
}

/// Each occurrence in the checked module, by origin, with the text it names.
fn occurrence_texts_chirho(
    result_chirho: &mut crate::FrontendResultChirho,
) -> HashMap<OriginIdChirho, String> {
    let mut texts_chirho = HashMap::new();
    for decl_chirho in &mut result_chirho.module_chirho.decls_chirho {
        visit_decl_chirho(decl_chirho, &mut |occurrence_chirho| {
            let OccurrenceMutChirho::ReferenceChirho(name_chirho) = &occurrence_chirho;
            if let Some(origin_chirho) = occurrence_chirho.origin_chirho() {
                texts_chirho.insert(origin_chirho, name_chirho.text_chirho().to_string());
            }
        });
    }
    texts_chirho
}

#[test]
fn every_record_names_an_occurrence_of_its_own_name_chirho() {
    let mut result_chirho = frontend_chirho(
        "module Main where\n\
         data ColorChirho = RedChirho | BlueChirho deriving (Show, Eq)\n\
         describeChirho :: (Show a, Num a) => a -> String\n\
         describeChirho n = show (n + 1)\n\
         main :: IO ()\n\
         main = do\n\
         \x20 putStrLn (show (3 :: Int))\n\
         \x20 putStrLn (show True)\n\
         \x20 putStrLn (describeChirho (2 :: Int))\n\
         \x20 print (RedChirho == BlueChirho)\n",
    );
    let texts_chirho = occurrence_texts_chirho(&mut result_chirho);
    let records_chirho = &result_chirho.infer_result_chirho.method_occurrences_chirho;
    let identified_chirho: Vec<_> = records_chirho
        .iter()
        .filter(|record_chirho| record_chirho.origin_chirho.is_some())
        .collect();
    assert!(!identified_chirho.is_empty(), "{records_chirho:?}");
    for record_chirho in &identified_chirho {
        let origin_chirho = record_chirho.origin_chirho.expect("filtered");
        assert_eq!(
            texts_chirho.get(&origin_chirho),
            Some(&record_chirho.name_chirho),
            "a record points at an occurrence of another name: {record_chirho:?}"
        );
    }

    // `show` at Int and at Bool: two occurrences, two origins, two keys.
    let show_keys_chirho: HashMap<OriginIdChirho, &str> = identified_chirho
        .iter()
        .filter(|record_chirho| {
            record_chirho.name_chirho == "show" && record_chirho.class_name_chirho == "Show"
        })
        .map(|record_chirho| {
            (
                record_chirho.origin_chirho.expect("filtered"),
                record_chirho.ty_key_chirho.as_str(),
            )
        })
        .collect();
    let distinct_keys_chirho: HashSet<&str> = show_keys_chirho.values().copied().collect();
    assert!(
        distinct_keys_chirho.contains("Int") && distinct_keys_chirho.contains("Bool"),
        "{show_keys_chirho:?}"
    );

    // `describeChirho` carries two predicates: both records, one origin.
    let describe_chirho: Vec<_> = identified_chirho
        .iter()
        .filter(|record_chirho| record_chirho.name_chirho == "describeChirho")
        .collect();
    let classes_chirho: HashSet<&str> = describe_chirho
        .iter()
        .map(|record_chirho| record_chirho.class_name_chirho.as_str())
        .collect();
    let origins_chirho: HashSet<OriginIdChirho> = describe_chirho
        .iter()
        .map(|record_chirho| record_chirho.origin_chirho.expect("filtered"))
        .collect();
    assert_eq!(
        classes_chirho,
        HashSet::from(["Show", "Num"]),
        "{describe_chirho:?}"
    );
    assert_eq!(origins_chirho.len(), 1, "{describe_chirho:?}");
    let reference_evidence_chirho = &result_chirho
        .infer_result_chirho
        .reference_evidence_by_origin_chirho;
    let describe_origin_chirho = *origins_chirho.iter().next().expect("one origin");
    assert_eq!(
        reference_evidence_chirho
            .get(&describe_origin_chirho)
            .map(Vec::len),
        Some(2),
        "the reference evidence keeps both predicates under the one origin"
    );
}

#[test]
fn records_from_derived_code_are_identified_chirho() {
    // The derived `Show` instance references `showsPrec` and friends under one
    // placeholder span; with origins, each of its records names its own
    // occurrence instead of waiting for a positional guess.
    let mut result_chirho = frontend_chirho(
        "module Main where\n\
         data PairChirho = PairChirho Bool Int deriving Show\n\
         main :: IO ()\n\
         main = print (PairChirho True 2)\n",
    );
    let texts_chirho = occurrence_texts_chirho(&mut result_chirho);
    let generated_chirho: Vec<_> = result_chirho
        .infer_result_chirho
        .method_occurrences_chirho
        .iter()
        .filter(|record_chirho| {
            record_chirho.span_chirho == haskelujah_span_chirho::SpanChirho::DUMMY_CHIRHO
        })
        .collect();
    assert!(
        !generated_chirho.is_empty(),
        "the derived instance recorded nothing"
    );
    for record_chirho in &generated_chirho {
        let origin_chirho = record_chirho
            .origin_chirho
            .unwrap_or_else(|| panic!("a generated record has no origin: {record_chirho:?}"));
        assert_eq!(
            texts_chirho.get(&origin_chirho),
            Some(&record_chirho.name_chirho)
        );
    }
}
