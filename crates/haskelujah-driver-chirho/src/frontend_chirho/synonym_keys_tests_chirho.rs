// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 656: the bare synonym keys `align_preferred_synonym_keys_chirho` adds for a module, and the
//! ones it never adds. Each case reads the module's real import list through
//! `collect_preferred_qualified_type_names_chirho`, as the frontend does.
//! workflow: compiler-pipeline-chirho/type-scope-resolution-chirho

use std::collections::HashMap;

use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_naming_chirho::iface_chirho::{
    IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho,
};
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::{FileIdChirho, SpanChirho};

use super::{
    ImportedTypeSynonymsChirho, align_preferred_synonym_keys_chirho,
    collect_preferred_qualified_type_names_chirho,
};

/// A nullary synonym standing for the type `target_chirho`.
fn synonym_chirho(target_chirho: &str) -> (Vec<String>, TypeChirho) {
    (
        vec![],
        TypeChirho::ConChirho(NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            target_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))),
    )
}

/// A source module's interface exporting the type names `types_chirho`.
fn iface_exporting_chirho(module_chirho: &str, types_chirho: &[&str]) -> ModuleIfaceChirho {
    let mut exports_chirho = IfaceExportsChirho::default();
    for type_chirho in types_chirho {
        exports_chirho.types_chirho.insert(
            (*type_chirho).to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: (*type_chirho).to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
    }
    ModuleIfaceChirho {
        name_chirho: module_chirho.to_string(),
        exports_chirho,
    }
}

/// The preferred exporters the frontend computes for `source_chirho` over `ifaces_chirho`.
fn preferred_for_chirho(
    source_chirho: &str,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> HashMap<String, String> {
    let green_chirho =
        ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho();
    let module_chirho = lower_module_chirho(&green_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
    collect_preferred_qualified_type_names_chirho(&module_chirho, ifaces_chirho)
}

#[test]
fn a_unique_exporters_synonym_gets_its_bare_key_chirho() {
    // Main imports A for another name only, and A is the one exporter of `Parent`.
    let ifaces_chirho = [iface_exporting_chirho("A", &["Parent", "Other"])];
    let preferred_chirho =
        preferred_for_chirho("module Main where\nimport A (Other)\n", &ifaces_chirho);
    assert_eq!(
        preferred_chirho.get("Parent").map(String::as_str),
        Some("A")
    );
    let mut merged_chirho: ImportedTypeSynonymsChirho =
        HashMap::from([("A.Parent".to_string(), synonym_chirho("Int"))]);
    align_preferred_synonym_keys_chirho(&mut merged_chirho, &preferred_chirho);
    assert_eq!(merged_chirho.get("Parent"), Some(&synonym_chirho("Int")));
}

#[test]
fn a_name_two_imports_export_gets_no_bare_key_chirho() {
    let ifaces_chirho = [
        iface_exporting_chirho("A", &["Parent"]),
        iface_exporting_chirho("B", &["Parent"]),
    ];
    // Either import alone makes its module the exporter; together they nominate none.
    let alone_chirho = preferred_for_chirho("module Main where\nimport A ()\n", &ifaces_chirho);
    assert_eq!(alone_chirho.get("Parent").map(String::as_str), Some("A"));
    let preferred_chirho = preferred_for_chirho(
        "module Main where\nimport A ()\nimport B ()\n",
        &ifaces_chirho,
    );
    assert!(
        !preferred_chirho.contains_key("Parent"),
        "{preferred_chirho:?}"
    );
    let mut merged_chirho: ImportedTypeSynonymsChirho = HashMap::from([
        ("A.Parent".to_string(), synonym_chirho("Int")),
        ("B.Parent".to_string(), synonym_chirho("Bool")),
    ]);
    align_preferred_synonym_keys_chirho(&mut merged_chirho, &preferred_chirho);
    assert!(!merged_chirho.contains_key("Parent"), "{merged_chirho:?}");
}

#[test]
fn no_bare_key_without_the_exporters_qualified_synonym_chirho() {
    // A exports `Parent`, but no synonym of A's reached the merged input.
    let ifaces_chirho = [iface_exporting_chirho("A", &["Parent"])];
    let preferred_chirho = preferred_for_chirho("module Main where\nimport A ()\n", &ifaces_chirho);
    assert_eq!(
        preferred_chirho.get("Parent").map(String::as_str),
        Some("A")
    );
    let mut merged_chirho: ImportedTypeSynonymsChirho =
        HashMap::from([("C.Parent".to_string(), synonym_chirho("Int"))]);
    align_preferred_synonym_keys_chirho(&mut merged_chirho, &preferred_chirho);
    assert!(!merged_chirho.contains_key("Parent"), "{merged_chirho:?}");
}

#[test]
fn an_existing_bare_key_is_never_overwritten_chirho() {
    let ifaces_chirho = [iface_exporting_chirho("A", &["Parent"])];
    let preferred_chirho = preferred_for_chirho("module Main where\nimport A ()\n", &ifaces_chirho);
    assert_eq!(
        preferred_chirho.get("Parent").map(String::as_str),
        Some("A")
    );
    let mut merged_chirho: ImportedTypeSynonymsChirho = HashMap::from([
        ("A.Parent".to_string(), synonym_chirho("Int")),
        ("Parent".to_string(), synonym_chirho("Bool")),
    ]);
    align_preferred_synonym_keys_chirho(&mut merged_chirho, &preferred_chirho);
    assert_eq!(merged_chirho.get("Parent"), Some(&synonym_chirho("Bool")));
}
