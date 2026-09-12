// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, ClassMethodChirho, DeclChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
    ModuleChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::{
    build_iface_chirho, build_iface_with_imports_chirho, builtin_module_ifaces_chirho,
};
use crate::resolve_chirho::resolve_module_with_imports_chirho;

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn module_chirho(
    name_text_chirho: &str,
    exports_chirho: Option<Vec<ExportSpecChirho>>,
    imports_chirho: Vec<ImportDeclChirho>,
    decls_chirho: Vec<DeclChirho>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho(name_text_chirho),
        exports_chirho,
        imports_chirho,
        decls_chirho,
        extensions_chirho: vec!["TypeFamilies".to_string()],
        inline_pragmas_chirho: HashMap::new(),
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: vec![],
        deriving_via_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn associated_class_chirho() -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho("ContainerChirho"),
        type_vars_chirho: vec![TyVarChirho::plain_chirho(name_chirho("itemChirho"))],
        methods_chirho: vec![ClassMethodChirho {
            name_chirho: name_chirho("extractChirho"),
            ty_chirho: TypeChirho::VarChirho(name_chirho("itemChirho")),
            default_chirho: None,
            default_sig_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        associated_tfs_chirho: vec![
            AssocTypeFamilyChirho {
                name_chirho: name_chirho("ElementChirho"),
                type_vars_chirho: vec![name_chirho("itemChirho")],
                default_rhs_chirho: None,
                default_params_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            AssocTypeFamilyChirho {
                name_chirho: name_chirho("IndexChirho"),
                type_vars_chirho: vec![name_chirho("itemChirho")],
                default_rhs_chirho: None,
                default_params_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

#[test]
fn builtin_interfaces_export_source_visible_type_names_chirho() {
    let modules_chirho = builtin_module_ifaces_chirho();
    let expected_types_chirho = [
        ("Language.Haskell.TH.Syntax", "Q"),
        ("GHC.Exts", "ByteArray#"),
        ("GHC.Exts", "Word8#"),
        ("GHC.Exts", "WithDict"),
        ("GHC.Prim", "ThreadId#"),
        ("GHC.Types", "TYPE"),
        ("GHC.Base", "ByteArray#"),
        ("Control.Monad.Trans.Reader", "Reader"),
        ("GHC.Generics", "Generically"),
        ("GHC.Generics", "NoSelector"),
        ("GHC.IO.Handle.Types", "Handle__"),
        ("GHC.IO.Handle.Internals", "Handle__"),
        ("Data.Typeable", ":~:"),
        ("Data.Data", ":~:"),
        ("Language.Haskell.TH.Syntax", "Lift"),
        ("System.Posix.Types", "CClockId"),
        ("Text.ParserCombinators.Parsec", "GenParser"),
    ];

    for (module_name_chirho, type_name_chirho) in expected_types_chirho {
        let module_chirho = modules_chirho
            .iter()
            .find(|module_chirho| module_chirho.name_chirho == module_name_chirho)
            .unwrap_or_else(|| panic!("missing built-in module {module_name_chirho}"));
        assert!(
            module_chirho
                .exports_chirho
                .types_chirho
                .contains_key(type_name_chirho),
            "{module_name_chirho} does not export type {type_name_chirho}"
        );
    }

    for (module_name_chirho, constructor_name_chirho) in [
        ("GHC.Generics", "Generically"),
        ("Data.Typeable", "Refl"),
        ("Data.Data", "Refl"),
    ] {
        let module_chirho = modules_chirho
            .iter()
            .find(|module_chirho| module_chirho.name_chirho == module_name_chirho)
            .unwrap_or_else(|| panic!("missing built-in module {module_name_chirho}"));
        assert!(
            module_chirho
                .exports_chirho
                .values_chirho
                .contains_key(constructor_name_chirho),
            "{module_name_chirho} does not export constructor {constructor_name_chirho}"
        );
    }

    let ghc_exts_chirho = modules_chirho
        .iter()
        .find(|module_chirho| module_chirho.name_chirho == "GHC.Exts")
        .expect("GHC.Exts built-in interface should exist");
    assert_eq!(
        ghc_exts_chirho.exports_chirho.associated_types_chirho["IsList"],
        ["Item"]
    );
}

#[test]
fn class_all_exports_associated_types_and_relation_chirho() {
    let module_chirho = module_chirho(
        "SourceChirho",
        Some(vec![ExportSpecChirho::TyConChirho {
            name_chirho: name_chirho("ContainerChirho"),
            members_chirho: ExportMembersChirho::AllChirho,
        }]),
        vec![],
        vec![associated_class_chirho()],
    );
    let exports_chirho = build_iface_chirho(&module_chirho).exports_chirho;

    assert!(exports_chirho.types_chirho.contains_key("ElementChirho"));
    assert!(exports_chirho.types_chirho.contains_key("IndexChirho"));
    assert_eq!(
        exports_chirho.associated_types_chirho["ContainerChirho"],
        ["ElementChirho", "IndexChirho"]
    );
}

#[test]
fn class_selected_export_keeps_only_selected_associated_type_chirho() {
    let module_chirho = module_chirho(
        "SourceChirho",
        Some(vec![ExportSpecChirho::TyConChirho {
            name_chirho: name_chirho("ContainerChirho"),
            members_chirho: ExportMembersChirho::SomeChirho(vec![name_chirho("ElementChirho")]),
        }]),
        vec![],
        vec![associated_class_chirho()],
    );
    let exports_chirho = build_iface_chirho(&module_chirho).exports_chirho;

    assert!(exports_chirho.types_chirho.contains_key("ElementChirho"));
    assert!(!exports_chirho.types_chirho.contains_key("IndexChirho"));
    assert!(!exports_chirho.values_chirho.contains_key("extractChirho"));
    assert_eq!(
        exports_chirho.associated_types_chirho["ContainerChirho"],
        ["ElementChirho"]
    );
}

#[test]
fn module_reexport_respects_selected_associated_import_chirho() {
    let source_module_chirho = module_chirho(
        "SourceChirho",
        None,
        vec![],
        vec![associated_class_chirho()],
    );
    let source_iface_chirho = build_iface_chirho(&source_module_chirho);
    let facade_module_chirho = module_chirho(
        "FacadeChirho",
        Some(vec![ExportSpecChirho::ModuleChirho(name_chirho(
            "SourceChirho",
        ))]),
        vec![ImportDeclChirho {
            module_chirho: name_chirho("SourceChirho"),
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho: false,
                items_chirho: vec![ImportItemChirho::TyConChirho {
                    name_chirho: name_chirho("ContainerChirho"),
                    members_chirho: ExportMembersChirho::SomeChirho(vec![name_chirho(
                        "ElementChirho",
                    )]),
                }],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![],
    );
    let exports_chirho =
        build_iface_with_imports_chirho(&facade_module_chirho, &[source_iface_chirho])
            .exports_chirho;

    assert!(exports_chirho.types_chirho.contains_key("ContainerChirho"));
    assert!(exports_chirho.types_chirho.contains_key("ElementChirho"));
    assert!(!exports_chirho.types_chirho.contains_key("IndexChirho"));
    assert_eq!(
        exports_chirho.associated_types_chirho["ContainerChirho"],
        ["ElementChirho"]
    );
}

#[test]
fn module_reexport_hiding_all_removes_associated_types_chirho() {
    let source_module_chirho = module_chirho(
        "SourceChirho",
        None,
        vec![],
        vec![associated_class_chirho()],
    );
    let source_iface_chirho = build_iface_chirho(&source_module_chirho);
    let facade_module_chirho = module_chirho(
        "FacadeChirho",
        Some(vec![ExportSpecChirho::ModuleChirho(name_chirho(
            "SourceChirho",
        ))]),
        vec![ImportDeclChirho {
            module_chirho: name_chirho("SourceChirho"),
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho: true,
                items_chirho: vec![ImportItemChirho::TyConChirho {
                    name_chirho: name_chirho("ContainerChirho"),
                    members_chirho: ExportMembersChirho::AllChirho,
                }],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![],
    );
    let exports_chirho =
        build_iface_with_imports_chirho(&facade_module_chirho, &[source_iface_chirho])
            .exports_chirho;

    assert!(!exports_chirho.types_chirho.contains_key("ContainerChirho"));
    assert!(!exports_chirho.types_chirho.contains_key("ElementChirho"));
    assert!(!exports_chirho.types_chirho.contains_key("IndexChirho"));
    assert!(exports_chirho.associated_types_chirho.is_empty());
}

#[test]
fn selected_associated_import_reaches_unqualified_type_scope_chirho() {
    let source_module_chirho = module_chirho(
        "SourceChirho",
        None,
        vec![],
        vec![associated_class_chirho()],
    );
    let source_iface_chirho = build_iface_chirho(&source_module_chirho);
    let consumer_module_chirho = module_chirho(
        "ConsumerChirho",
        None,
        vec![ImportDeclChirho {
            module_chirho: name_chirho("SourceChirho"),
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho: false,
                items_chirho: vec![ImportItemChirho::TyConChirho {
                    name_chirho: name_chirho("ContainerChirho"),
                    members_chirho: ExportMembersChirho::SomeChirho(vec![name_chirho(
                        "ElementChirho",
                    )]),
                }],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![DeclChirho::TypeSigChirho {
            name_chirho: name_chirho("consumerChirho"),
            ty_chirho: TypeChirho::ConChirho(name_chirho("ElementChirho")),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho =
        resolve_module_with_imports_chirho(&consumer_module_chirho, &[source_iface_chirho]);
    assert!(result_chirho.diagnostics_chirho.is_empty_chirho());
    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("ElementChirho")
            .is_some()
    );
    assert!(
        result_chirho
            .env_chirho
            .lookup_type_chirho("IndexChirho")
            .is_none()
    );
}

#[test]
fn selected_associated_import_reaches_qualified_alias_scope_chirho() {
    let source_module_chirho = module_chirho(
        "SourceChirho",
        None,
        vec![],
        vec![associated_class_chirho()],
    );
    let source_iface_chirho = build_iface_chirho(&source_module_chirho);
    let consumer_module_chirho = module_chirho(
        "ConsumerChirho",
        None,
        vec![ImportDeclChirho {
            module_chirho: name_chirho("SourceChirho"),
            qualified_chirho: true,
            alias_chirho: Some(name_chirho("AliasChirho")),
            spec_chirho: Some(ImportSpecChirho {
                hiding_chirho: false,
                items_chirho: vec![ImportItemChirho::TyConChirho {
                    name_chirho: name_chirho("ContainerChirho"),
                    members_chirho: ExportMembersChirho::SomeChirho(vec![name_chirho(
                        "ElementChirho",
                    )]),
                }],
            }),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![DeclChirho::TypeSigChirho {
            name_chirho: name_chirho("consumerChirho"),
            ty_chirho: TypeChirho::ConChirho(NameChirho::RawChirho(
                RawNameChirho::qualified_chirho(
                    "AliasChirho",
                    "ElementChirho",
                    SpanChirho::DUMMY_CHIRHO,
                ),
            )),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
    );

    let result_chirho =
        resolve_module_with_imports_chirho(&consumer_module_chirho, &[source_iface_chirho]);
    assert!(result_chirho.diagnostics_chirho.is_empty_chirho());
    assert!(
        result_chirho
            .env_chirho
            .lookup_qualified_chirho(
                "AliasChirho",
                "ElementChirho",
                crate::env_chirho::NamespaceChirho::TypeChirho,
            )
            .is_some()
    );
}
