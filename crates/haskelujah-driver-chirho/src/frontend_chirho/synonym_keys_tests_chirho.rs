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
    align_preferred_synonym_keys_chirho(
        &mut merged_chirho,
        &preferred_chirho,
        &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    )
    .expect("bounded synonym alignment");
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
    align_preferred_synonym_keys_chirho(
        &mut merged_chirho,
        &preferred_chirho,
        &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    )
    .expect("bounded synonym alignment");
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
    align_preferred_synonym_keys_chirho(
        &mut merged_chirho,
        &preferred_chirho,
        &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    )
    .expect("bounded synonym alignment");
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
    align_preferred_synonym_keys_chirho(
        &mut merged_chirho,
        &preferred_chirho,
        &mut haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::new_chirho(),
        SpanChirho::DUMMY_CHIRHO,
    )
    .expect("bounded synonym alignment");
    assert_eq!(merged_chirho.get("Parent"), Some(&synonym_chirho("Bool")));
}

fn checked_synonym_producer_chirho() -> crate::FrontendResultChirho {
    crate::run_frontend_chirho("{-# LANGUAGE RankNTypes, LiberalTypeSynonyms #-}\nmodule OriginChirho (SChirho) where\ntype SChirho aChirho = forall bChirho. bChirho -> aChirho\n", FileIdChirho::SYNTHETIC_CHIRHO, &haskelujah_naming_chirho::builtin_module_ifaces_chirho(), &HashMap::new()).expect("checked synonym producer")
}

#[test]
fn checked_synonym_export_reexport_and_use_preserve_canonical_owners_chirho() {
    let producer_chirho = checked_synonym_producer_chirho();
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let iface_chirho = crate::family_exports_chirho::build_family_aware_iface_chirho(
        &producer_chirho.module_chirho,
        &ifaces_chirho,
        &producer_chirho.infer_result_chirho.type_families_chirho,
    )
    .unwrap();
    let synonyms_chirho = crate::exported_type_synonyms_from_module_chirho(
        &producer_chirho.module_chirho,
        &iface_chirho,
        &HashMap::new(),
        &producer_chirho.infer_result_chirho,
    )
    .unwrap();
    let published_chirho = format!("{:?}", synonyms_chirho["SChirho"].1);
    assert!(
        published_chirho.contains("QuantifiedChirho"),
        "{published_chirho}"
    );
    assert!(
        !published_chirho.contains("InferenceChirho"),
        "{published_chirho}"
    );
    assert!(format!("{:?}", producer_chirho.module_chirho).contains("InferenceChirho"));
    ifaces_chirho.push(iface_chirho);
    let middle_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
        "module MiddleChirho (SChirho) where\nimport OriginChirho (SChirho)\n",
        FileIdChirho::SYNTHETIC_CHIRHO,
        &ifaces_chirho,
        &HashMap::new(),
        &synonyms_chirho,
        &producer_chirho.infer_result_chirho.type_families_chirho,
    )
    .expect("checked re-export");
    let iface_chirho = crate::family_exports_chirho::build_family_aware_iface_chirho(
        &middle_chirho.module_chirho,
        &ifaces_chirho,
        &middle_chirho.infer_result_chirho.type_families_chirho,
    )
    .unwrap();
    let forwarded_chirho = crate::exported_type_synonyms_from_module_chirho(
        &middle_chirho.module_chirho,
        &iface_chirho,
        &middle_chirho.resolved_imported_type_synonyms_chirho,
        &middle_chirho.infer_result_chirho,
    )
    .unwrap();
    assert_eq!(
        format!("{:?}", forwarded_chirho["SChirho"].1),
        published_chirho
    );
    ifaces_chirho.push(iface_chirho);
    crate::run_frontend_with_type_synonyms_and_type_families_chirho("{-# LANGUAGE RankNTypes, LiberalTypeSynonyms #-}\nmodule CallerChirho where\nimport MiddleChirho (SChirho)\nfChirho :: SChirho Int\nfChirho _ = undefined\n", FileIdChirho::SYNTHETIC_CHIRHO, &ifaces_chirho, &HashMap::new(), &forwarded_chirho, &middle_chirho.infer_result_chirho.type_families_chirho).expect("ordinary caller consumes the checked lexical forall");
}

#[test]
fn legacy_synonym_ast_cannot_replace_the_checked_catalogue_body_chirho() {
    let producer_chirho = checked_synonym_producer_chirho();
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let iface_chirho = crate::family_exports_chirho::build_family_aware_iface_chirho(
        &producer_chirho.module_chirho,
        &ifaces_chirho,
        &producer_chirho.infer_result_chirho.type_families_chirho,
    )
    .unwrap();
    let mut synonyms_chirho = crate::exported_type_synonyms_from_module_chirho(
        &producer_chirho.module_chirho,
        &iface_chirho,
        &HashMap::new(),
        &producer_chirho.infer_result_chirho,
    )
    .unwrap();
    for (_, rhs_chirho) in synonyms_chirho.values_mut() {
        let TypeChirho::ForallChirho { body_chirho, .. } = rhs_chirho else {
            panic!("actual producer owns a forall");
        };
        *rhs_chirho = *std::mem::replace(
            body_chirho,
            Box::new(TypeChirho::WildcardChirho {
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }),
        );
    }
    ifaces_chirho.push(iface_chirho);
    let result_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho("{-# LANGUAGE RankNTypes #-}\n{-# OPTIONS_GHC -fdefer-type-errors #-}\nmodule CallerChirho where\nimport OriginChirho (SChirho)\nfChirho :: SChirho Int\nfChirho _ = undefined\n", FileIdChirho::SYNTHETIC_CHIRHO, &ifaces_chirho, &HashMap::new(), &synonyms_chirho, &producer_chirho.infer_result_chirho.type_families_chirho ).expect("legacy AST mutation cannot override the checked producer body");
    let scheme_chirho = result_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho("fChirho")
        .expect("actual caller signature");
    assert_eq!(
        scheme_chirho.vars_chirho.len(),
        1,
        "the producer forall remains quantified"
    );
    assert_eq!(
        scheme_chirho.ty_chirho.free_vars_chirho(),
        scheme_chirho.vars_chirho.to_vec()
    );
}

#[test]
fn checked_synonym_export_budget_exhaustion_is_latched_and_coded_chirho() {
    let producer_chirho = checked_synonym_producer_chirho();
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let iface_chirho = crate::family_exports_chirho::build_family_aware_iface_chirho(
        &producer_chirho.module_chirho,
        &ifaces_chirho,
        &producer_chirho.infer_result_chirho.type_families_chirho,
    )
    .unwrap();
    let mut budget_chirho =
        haskelujah_ast_chirho::decl_chirho::GadtProjectionBudgetChirho::with_limits_chirho(
            haskelujah_ast_chirho::decl_chirho::GadtProjectionLimitsChirho {
                work_chirho: 1,
                output_chirho: 1,
                depth_chirho: 128,
            },
        );
    for _attempt_chirho in 0..2 {
        let error_chirho = crate::type_names_chirho::exported_type_synonyms_with_budget_chirho(
            &producer_chirho.module_chirho,
            &iface_chirho,
            &HashMap::new(),
            &producer_chirho.infer_result_chirho,
            &mut budget_chirho,
        )
        .err()
        .expect("no partial export on exhaustion");
        assert!(format!("{error_chirho}").contains("error[E0300]"));
    }
}
