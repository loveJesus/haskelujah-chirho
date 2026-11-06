// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Positive role production and transport, including the unknown boundaries.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::*;
use crate::iface_chirho::{
    IfaceExportsChirho, IfaceTypeChirho, build_iface_chirho, build_iface_with_imports_chirho,
    builtin_module_ifaces_chirho, merge_module_ifaces_chirho,
};
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
};
use haskelujah_ast_chirho::name_chirho::{
    CompilerTypeConstructorChirho, NameChirho, RawNameChirho, TypeConstructorDeclarationChirho,
    TypeConstructorIdentityChirho, TypeConstructorRoleChirho,
};
use haskelujah_span_chirho::SpanChirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}
fn module_chirho(
    module_name_chirho: &str,
    decls_chirho: Vec<DeclChirho>,
    imports_chirho: Vec<ImportDeclChirho>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho(module_name_chirho),
        exports_chirho: None,
        imports_chirho,
        decls_chirho,
        extensions_chirho: vec![],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
        origin_supply_chirho: Default::default(),
    }
}
fn import_chirho(
    module_name_chirho: &str,
    alias_chirho: Option<&str>,
    qualified_chirho: bool,
) -> ImportDeclChirho {
    ImportDeclChirho {
        package_chirho: None,
        module_chirho: name_chirho(module_name_chirho),
        source_chirho: false,
        qualified_chirho,
        alias_chirho: alias_chirho.map(name_chirho),
        spec_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}
fn data_chirho(
    type_name_chirho: &str,
    role_chirho: Option<TypeConstructorRoleChirho>,
) -> DeclChirho {
    let mut defining_name_chirho = name_chirho(type_name_chirho);
    if let Some(role_chirho) = role_chirho {
        defining_name_chirho.set_written_type_declaration_role_chirho(role_chirho);
    }
    DeclChirho::DataDeclChirho {
        name_chirho: defining_name_chirho,
        type_vars_chirho: vec![],
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            prefix_chirho: Default::default(),
            name_chirho: name_chirho("MkChirho"),
            fields_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}
fn iface_chirho(
    module_name_chirho: &str,
    type_name_chirho: &str,
    declaration_chirho: Option<TypeConstructorDeclarationChirho>,
) -> ModuleIfaceChirho {
    let mut exports_chirho = IfaceExportsChirho::default();
    exports_chirho.types_chirho.insert(
        type_name_chirho.to_owned(),
        IfaceTypeChirho {
            name_chirho: type_name_chirho.to_owned(),
            declaration_chirho,
            constructors_chirho: vec!["MkChirho".to_owned()],
            methods_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    );
    ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: module_name_chirho.to_owned(),
        exports_chirho,
    }
}
fn source_chirho(
    module_name_chirho: &str,
    type_name_chirho: &str,
    role_chirho: TypeConstructorRoleChirho,
) -> TypeConstructorDeclarationChirho {
    TypeConstructorDeclarationChirho::source_chirho(
        module_name_chirho,
        type_name_chirho,
        role_chirho,
    )
}

#[test]
fn an_actual_written_data_header_supplies_its_defining_identity_chirho() {
    let module_chirho = module_chirho(
        "DefChirho",
        vec![data_chirho(
            "TChirho",
            Some(TypeConstructorRoleChirho::DataChirho),
        )],
        vec![],
    );
    let scope_chirho = build_type_constructor_scope_chirho(&module_chirho, &[]);
    let expected_chirho = source_chirho(
        "DefChirho",
        "TChirho",
        TypeConstructorRoleChirho::DataChirho,
    );
    assert_eq!(
        scope_chirho.declaration_chirho("TChirho"),
        Some(&expected_chirho)
    );
    assert_eq!(
        scope_chirho.declaration_chirho("DefChirho.TChirho"),
        Some(&expected_chirho)
    );
    assert_eq!(
        build_iface_chirho(&module_chirho)
            .expect("small source interface fixture fits constructor projection budget")
            .exports_chirho
            .types_chirho["TChirho"]
            .declaration_chirho
            .as_ref(),
        Some(&expected_chirho)
    );
}

#[test]
fn unannotated_ast_or_stub_members_never_supply_an_algebraic_role_chirho() {
    let module_chirho = module_chirho(
        "Main",
        vec![data_chirho("LocalChirho", None)],
        vec![import_chirho("StubChirho", None, false)],
    );
    let scope_chirho = build_type_constructor_scope_chirho(
        &module_chirho,
        &[iface_chirho("StubChirho", "StubChirho", None)],
    );
    assert!(scope_chirho.declaration_chirho("LocalChirho").is_none());
    assert!(scope_chirho.declaration_chirho("StubChirho").is_none());
    assert!(
        scope_chirho
            .declaration_chirho("StubChirho.StubChirho")
            .is_none()
    );
}

#[test]
fn non_algebraic_roles_survive_without_becoming_ordinary_data_chirho() {
    for role_chirho in [
        TypeConstructorRoleChirho::ClassChirho,
        TypeConstructorRoleChirho::TypeSynonymChirho,
        TypeConstructorRoleChirho::TypeFamilyChirho,
        TypeConstructorRoleChirho::DataFamilyChirho,
    ] {
        let expected_chirho = source_chirho("DefChirho", "TChirho", role_chirho);
        let module_chirho = module_chirho(
            "Main",
            vec![],
            vec![import_chirho("ForwardChirho", Some("QChirho"), true)],
        );
        let scope_chirho = build_type_constructor_scope_chirho(
            &module_chirho,
            &[iface_chirho(
                "ForwardChirho",
                "TChirho",
                Some(expected_chirho.clone()),
            )],
        );
        assert_eq!(
            scope_chirho.declaration_chirho("QChirho.TChirho"),
            Some(&expected_chirho)
        );
        assert!(!expected_chirho.is_ordinary_data_chirho());
        // This is category classification only. Data-family nominal
        // decomposition has its own policy, like ordinary data (GHC Note).
    }
}

#[test]
fn alias_import_lists_and_hiding_keep_only_admitted_spellings_chirho() {
    let expected_chirho = source_chirho(
        "DefChirho",
        "TChirho",
        TypeConstructorRoleChirho::DataChirho,
    );
    let exported_chirho = iface_chirho("DefChirho", "TChirho", Some(expected_chirho.clone()));
    let mut imported_chirho = import_chirho("DefChirho", Some("QChirho"), true);
    imported_chirho.spec_chirho = Some(ImportSpecChirho {
        hiding_chirho: false,
        items_chirho: vec![ImportItemChirho::TyConChirho {
            name_chirho: name_chirho("TChirho"),
            members_chirho: ExportMembersChirho::NoneChirho,
        }],
    });
    let mut module_chirho = module_chirho("Main", vec![], vec![imported_chirho]);
    let scope_chirho =
        build_type_constructor_scope_chirho(&module_chirho, std::slice::from_ref(&exported_chirho));
    assert_eq!(
        scope_chirho.declaration_chirho("QChirho.TChirho"),
        Some(&expected_chirho)
    );
    assert!(scope_chirho.declaration_chirho("TChirho").is_none());
    assert!(
        scope_chirho
            .declaration_chirho("DefChirho.TChirho")
            .is_none()
    );
    module_chirho.imports_chirho[0]
        .spec_chirho
        .as_mut()
        .unwrap()
        .hiding_chirho = true;
    assert!(
        build_type_constructor_scope_chirho(&module_chirho, &[exported_chirho])
            .declaration_chirho("QChirho.TChirho")
            .is_none()
    );
}

#[test]
fn a_local_homonym_has_source_identity_and_shadows_the_bare_import_chirho() {
    let module_chirho = module_chirho(
        "Main",
        vec![data_chirho(
            "Maybe",
            Some(TypeConstructorRoleChirho::DataChirho),
        )],
        vec![import_chirho("Prelude", Some("PChirho"), false)],
    );
    let scope_chirho =
        build_type_constructor_scope_chirho(&module_chirho, &builtin_module_ifaces_chirho());
    assert_eq!(
        scope_chirho.declaration_chirho("Maybe"),
        Some(&source_chirho(
            "Main",
            "Maybe",
            TypeConstructorRoleChirho::DataChirho
        ))
    );
    assert_eq!(
        scope_chirho.declaration_chirho("PChirho.Maybe"),
        Some(&TypeConstructorDeclarationChirho::compiler_data_chirho(
            CompilerTypeConstructorChirho::MaybeChirho
        ))
    );
}

#[test]
fn conflicts_and_unknown_imports_cannot_restore_a_proof_by_iteration_order_chirho() {
    let left_chirho = source_chirho(
        "LeftChirho",
        "TChirho",
        TypeConstructorRoleChirho::DataChirho,
    );
    let right_chirho = source_chirho(
        "RightChirho",
        "TChirho",
        TypeConstructorRoleChirho::DataChirho,
    );
    let ifaces_chirho = vec![
        iface_chirho("LeftChirho", "TChirho", Some(left_chirho.clone())),
        iface_chirho("RightChirho", "TChirho", Some(right_chirho)),
        iface_chirho("StubChirho", "TChirho", None),
    ];
    for imports_chirho in [
        vec![
            import_chirho("LeftChirho", None, false),
            import_chirho("RightChirho", None, false),
            import_chirho("LeftChirho", None, false),
        ],
        vec![
            import_chirho("StubChirho", None, false),
            import_chirho("LeftChirho", None, false),
        ],
        vec![
            import_chirho("LeftChirho", None, false),
            import_chirho("StubChirho", None, false),
        ],
    ] {
        let scope_chirho = build_type_constructor_scope_chirho(
            &module_chirho("Main", vec![], imports_chirho),
            &ifaces_chirho,
        );
        assert!(scope_chirho.declaration_chirho("TChirho").is_none());
        assert_eq!(
            scope_chirho.declaration_chirho("LeftChirho.TChirho"),
            Some(&left_chirho)
        );
    }
}

#[test]
fn reexports_and_constructor_hiding_keep_the_original_definition_chirho() {
    let expected_chirho = source_chirho(
        "DefChirho",
        "TChirho",
        TypeConstructorRoleChirho::DataChirho,
    );
    let exported_chirho = iface_chirho("DefChirho", "TChirho", Some(expected_chirho.clone()));
    let mut forward_chirho = module_chirho(
        "ForwardChirho",
        vec![],
        vec![import_chirho("DefChirho", None, false)],
    );
    forward_chirho.exports_chirho = Some(vec![ExportSpecChirho::TyConChirho {
        name_chirho: name_chirho("TChirho"),
        members_chirho: ExportMembersChirho::NoneChirho,
    }]);
    let forwarded_chirho = build_iface_with_imports_chirho(&forward_chirho, &[exported_chirho])
        .expect("small source interface fixture fits constructor projection budget");
    assert!(
        forwarded_chirho.exports_chirho.types_chirho["TChirho"]
            .constructors_chirho
            .is_empty()
    );
    assert_eq!(
        forwarded_chirho.exports_chirho.types_chirho["TChirho"]
            .declaration_chirho
            .as_ref(),
        Some(&expected_chirho)
    );
    let main_chirho = module_chirho(
        "Main",
        vec![],
        vec![import_chirho("ForwardChirho", Some("QChirho"), true)],
    );
    assert_eq!(
        build_type_constructor_scope_chirho(&main_chirho, &[forwarded_chirho])
            .declaration_chirho("QChirho.TChirho"),
        Some(&expected_chirho)
    );
}

#[test]
fn snapshot_merge_keeps_identical_proof_and_drops_weak_or_conflicting_proof_chirho() {
    let expected_chirho = source_chirho(
        "DefChirho",
        "TChirho",
        TypeConstructorRoleChirho::DataChirho,
    );
    let known_chirho = iface_chirho("DefChirho", "TChirho", Some(expected_chirho.clone()));
    let merged_chirho =
        merge_module_ifaces_chirho(vec![known_chirho.clone(), known_chirho.clone()]);
    assert_eq!(
        merged_chirho[0].exports_chirho.types_chirho["TChirho"]
            .declaration_chirho
            .as_ref(),
        Some(&expected_chirho)
    );
    for weak_chirho in [
        iface_chirho("DefChirho", "TChirho", None),
        iface_chirho(
            "DefChirho",
            "TChirho",
            Some(source_chirho(
                "OtherChirho",
                "TChirho",
                TypeConstructorRoleChirho::DataChirho,
            )),
        ),
    ] {
        let merged_chirho = merge_module_ifaces_chirho(vec![
            known_chirho.clone(),
            weak_chirho,
            known_chirho.clone(),
        ]);
        assert!(
            merged_chirho[0].exports_chirho.types_chirho["TChirho"]
                .declaration_chirho
                .is_none()
        );
    }
}

#[test]
fn only_explicit_compiler_data_seed_producers_supply_builtin_roles_chirho() {
    let program_chirho = module_chirho(
        "Main",
        vec![],
        vec![
            import_chirho("Prelude", Some("PChirho"), true),
            import_chirho("Data.Proxy", Some("QChirho"), true),
        ],
    );
    let scope_chirho =
        build_type_constructor_scope_chirho(&program_chirho, &builtin_module_ifaces_chirho());
    for (spelling_chirho, seed_chirho) in [
        ("PChirho.Bool", CompilerTypeConstructorChirho::BoolChirho),
        ("PChirho.Maybe", CompilerTypeConstructorChirho::MaybeChirho),
        (
            "PChirho.Either",
            CompilerTypeConstructorChirho::EitherChirho,
        ),
        (
            "PChirho.Ordering",
            CompilerTypeConstructorChirho::OrderingChirho,
        ),
        ("QChirho.Proxy", CompilerTypeConstructorChirho::ProxyChirho),
    ] {
        assert_eq!(
            scope_chirho.declaration_chirho(spelling_chirho),
            Some(&TypeConstructorDeclarationChirho::compiler_data_chirho(
                seed_chirho
            ))
        );
    }
    let mut untrusted_chirho = iface_chirho("Data.Strict.Maybe", "Maybe", None);
    untrusted_chirho.exports_chirho.origin_chirho =
        crate::iface_chirho::IfaceOriginChirho::BuiltinChirho;
    let untrusted_program_chirho = module_chirho(
        "Main",
        vec![],
        vec![import_chirho("Data.Strict.Maybe", None, false)],
    );
    assert!(
        build_type_constructor_scope_chirho(&untrusted_program_chirho, &[untrusted_chirho])
            .declaration_chirho("Maybe")
            .is_none()
    );
}

#[test]
fn written_role_is_cloned_transport_metadata_outside_name_equality_and_hash_chirho() {
    let original_chirho = name_chirho("TChirho");
    let mut marked_chirho = original_chirho.clone();
    marked_chirho.set_written_type_declaration_role_chirho(TypeConstructorRoleChirho::DataChirho);
    assert_eq!(original_chirho, marked_chirho);
    let mut original_hash_chirho = std::collections::hash_map::DefaultHasher::new();
    let mut marked_hash_chirho = std::collections::hash_map::DefaultHasher::new();
    original_chirho.hash(&mut original_hash_chirho);
    marked_chirho.hash(&mut marked_hash_chirho);
    assert_eq!(original_hash_chirho.finish(), marked_hash_chirho.finish());
    assert_eq!(
        marked_chirho.clone().written_type_declaration_role_chirho(),
        Some(TypeConstructorRoleChirho::DataChirho)
    );
}

#[test]
fn source_identity_keys_are_delimited_and_distinct_from_compiler_seeds_chirho() {
    let left_chirho = TypeConstructorIdentityChirho::SourceChirho {
        defining_module_chirho: "A.B".to_owned(),
        name_chirho: "C".to_owned(),
    };
    let right_chirho = TypeConstructorIdentityChirho::SourceChirho {
        defining_module_chirho: "A".to_owned(),
        name_chirho: "B.C".to_owned(),
    };
    assert_ne!(left_chirho.key_chirho(), right_chirho.key_chirho());
    assert_ne!(
        TypeConstructorIdentityChirho::SourceChirho {
            defining_module_chirho: "Prelude".to_owned(),
            name_chirho: "Maybe".to_owned()
        }
        .key_chirho(),
        TypeConstructorIdentityChirho::CompilerChirho(CompilerTypeConstructorChirho::MaybeChirho)
            .key_chirho()
    );
}
