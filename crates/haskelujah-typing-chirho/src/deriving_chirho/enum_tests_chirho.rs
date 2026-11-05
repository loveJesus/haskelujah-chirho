// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 669: which local types the derived-Enum metadata describes.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    ConDeclChirho, DeclChirho, DerivedClassChirho, StrictnessChirho,
};
use haskelujah_ast_chirho::module_chirho::ModuleChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;

use super::super::{gen_span_chirho, var_name_chirho};
use super::derived_enum_metadata_chirho;

/// A constructor with `fields_chirho` lazy `Int` fields.
fn constructor_chirho(name_chirho: &str, fields_chirho: usize) -> ConDeclChirho {
    ConDeclChirho::OrdinaryChirho {
        name_chirho: var_name_chirho(name_chirho),
        fields_chirho: (0..fields_chirho)
            .map(|_| {
                (
                    StrictnessChirho::LazyChirho,
                    TypeChirho::ConChirho(var_name_chirho("Int")),
                )
            })
            .collect(),
        span_chirho: gen_span_chirho(),
    }
}

fn data_chirho(
    name_chirho: &str,
    parameters_chirho: &[&str],
    constructors_chirho: Vec<ConDeclChirho>,
    deriving_chirho: &[&str],
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: var_name_chirho(name_chirho),
        type_vars_chirho: parameters_chirho
            .iter()
            .map(|parameter_chirho| var_name_chirho(parameter_chirho).into())
            .collect(),
        constructors_chirho,
        deriving_chirho: deriving_chirho
            .iter()
            .map(|class_chirho| {
                DerivedClassChirho::unspecified_chirho(var_name_chirho(class_chirho))
            })
            .collect(),
        kind_sig_chirho: None,
        span_chirho: gen_span_chirho(),
    }
}

fn metadata_for_chirho(decls_chirho: Vec<DeclChirho>) -> HashMap<String, Vec<String>> {
    derived_enum_metadata_chirho(&ModuleChirho {
        name_chirho: var_name_chirho("Main"),
        exports_chirho: None,
        imports_chirho: vec![],
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: gen_span_chirho(),
        origin_supply_chirho: Default::default(),
    })
}

#[test]
fn an_enumeration_type_records_its_constructors_in_order_chirho() {
    let metadata_chirho = metadata_for_chirho(vec![data_chirho(
        "ColorChirho",
        &[],
        vec![
            constructor_chirho("RedChirho", 0),
            constructor_chirho("GreenChirho", 0),
            constructor_chirho("BlueChirho", 0),
        ],
        &["Show", "Enum"],
    )]);
    assert_eq!(
        metadata_chirho.get("ColorChirho"),
        Some(&vec![
            "RedChirho".to_string(),
            "GreenChirho".to_string(),
            "BlueChirho".to_string(),
        ])
    );
}

#[test]
fn a_singleton_type_records_its_one_constructor_chirho() {
    let metadata_chirho = metadata_for_chirho(vec![data_chirho(
        "OnlyChirho",
        &[],
        vec![constructor_chirho("OnlyChirho", 0)],
        &["Enum"],
    )]);
    assert_eq!(
        metadata_chirho.get("OnlyChirho"),
        Some(&vec!["OnlyChirho".to_string()])
    );
}

#[test]
fn shapes_the_derived_instance_refuses_are_not_recorded_chirho() {
    let metadata_chirho = metadata_for_chirho(vec![
        data_chirho(
            "FieldChirho",
            &[],
            vec![constructor_chirho("FieldChirho", 1)],
            &["Enum"],
        ),
        data_chirho(
            "ParamChirho",
            &["a"],
            vec![constructor_chirho("ParamChirho", 0)],
            &["Enum"],
        ),
        data_chirho("VoidChirho", &[], vec![], &["Enum"]),
        data_chirho(
            "PlainChirho",
            &[],
            vec![constructor_chirho("PlainChirho", 0)],
            &["Show"],
        ),
    ]);
    assert!(metadata_chirho.is_empty(), "{metadata_chirho:?}");
}
