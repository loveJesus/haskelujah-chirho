// For God so loved the world, that he gave his only begotten Son, that whosoever
// believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{
    AssocTypeFamilyChirho, ClassMethodChirho, ConDeclChirho, DeclChirho, FieldDeclChirho,
    PatSynDirChirho, StrictnessChirho, TyVarChirho,
};
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ExportSpecChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
    ModuleChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::pat_chirho::PatChirho;
use haskelujah_ast_chirho::ty_chirho::TypeChirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::{
    ModuleIfaceChirho, build_iface_chirho, build_iface_with_imports_chirho,
    builtin_module_ifaces_chirho, merge_module_ifaces_chirho,
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
        origin_supply_chirho: Default::default(),
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

/// `FacadeChirho`, importing `SourceChirho` with `spec_chirho` and
/// re-exporting it whole through `module SourceChirho`.
fn facade_reexporting_chirho(spec_chirho: ImportSpecChirho) -> ModuleChirho {
    module_chirho(
        "FacadeChirho",
        Some(vec![ExportSpecChirho::ModuleChirho(name_chirho(
            "SourceChirho",
        ))]),
        vec![ImportDeclChirho {
            module_chirho: name_chirho("SourceChirho"),
            qualified_chirho: false,
            alias_chirho: None,
            spec_chirho: Some(spec_chirho),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        vec![],
    )
}

fn plain_data_chirho(
    type_text_chirho: &str,
    constructors_chirho: Vec<ConDeclChirho>,
) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho(type_text_chirho),
        type_vars_chirho: vec![],
        constructors_chirho,
        deriving_chirho: vec![],
        kind_sig_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn ordinary_constructor_chirho(constructor_text_chirho: &str) -> ConDeclChirho {
    ConDeclChirho::OrdinaryChirho {
        name_chirho: name_chirho(constructor_text_chirho),
        fields_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

// Row 536: an item lowering could not represent names nothing. A list holding
// only that item re-exports nothing, and a `hiding` list holding it hides
// nothing, exactly as when lowering dropped it.
#[test]
fn an_unrepresented_item_reexports_and_hides_nothing_chirho() {
    let source_iface_chirho = build_iface_chirho(&module_chirho(
        "SourceChirho",
        None,
        vec![],
        vec![associated_class_chirho()],
    ));
    let unrepresented_chirho = vec![ImportItemChirho::UnrepresentedChirho(
        SpanChirho::DUMMY_CHIRHO,
    )];

    let listed_chirho = build_iface_with_imports_chirho(
        &facade_reexporting_chirho(ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: unrepresented_chirho.clone(),
        }),
        std::slice::from_ref(&source_iface_chirho),
    )
    .exports_chirho;
    assert!(listed_chirho.types_chirho.is_empty());
    assert!(listed_chirho.values_chirho.is_empty());

    let hidden_chirho = build_iface_with_imports_chirho(
        &facade_reexporting_chirho(ImportSpecChirho {
            hiding_chirho: true,
            items_chirho: unrepresented_chirho,
        }),
        std::slice::from_ref(&source_iface_chirho),
    )
    .exports_chirho;
    for type_name_chirho in ["ContainerChirho", "ElementChirho", "IndexChirho"] {
        assert!(
            hidden_chirho.types_chirho.contains_key(type_name_chirho),
            "{type_name_chirho}"
        );
    }
    assert!(hidden_chirho.values_chirho.contains_key("extractChirho"));
}

// Row 536: a source module's own types are complete as exported, except a
// data type with a GADT constructor, whose record fields are not lowered.
#[test]
fn a_source_module_marks_the_types_it_declares_complete_chirho() {
    let iface_chirho = build_iface_chirho(&module_chirho(
        "SourceChirho",
        None,
        vec![],
        vec![
            plain_data_chirho(
                "RecordChirho",
                vec![ConDeclChirho::RecordChirho {
                    name_chirho: name_chirho("RecordChirho"),
                    fields_chirho: vec![FieldDeclChirho {
                        names_chirho: vec![name_chirho("nameChirho")],
                        ty_chirho: TypeChirho::ConChirho(name_chirho("Int")),
                        strictness_chirho: StrictnessChirho::LazyChirho,
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
            ),
            DeclChirho::NewtypeDeclChirho {
                name_chirho: name_chirho("WrapChirho"),
                type_vars_chirho: vec![],
                constructor_chirho: ordinary_constructor_chirho("WrapChirho"),
                deriving_chirho: vec![],
                kind_sig_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            associated_class_chirho(),
            plain_data_chirho(
                "GadtChirho",
                vec![ConDeclChirho::GadtChirho {
                    name_chirho: name_chirho("MkGadtChirho"),
                    ty_chirho: TypeChirho::ConChirho(name_chirho("GadtChirho")),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
            ),
        ],
    ));
    let exports_chirho = &iface_chirho.exports_chirho;
    for type_name_chirho in ["RecordChirho", "WrapChirho", "ContainerChirho"] {
        assert!(
            exports_chirho
                .complete_member_types_chirho
                .contains(type_name_chirho),
            "{type_name_chirho}"
        );
    }
    assert!(
        !exports_chirho
            .complete_member_types_chirho
            .contains("GadtChirho")
    );
    // The record's field is one of its members, so `RecordChirho(..)` brings it.
    assert!(
        exports_chirho.types_chirho["RecordChirho"]
            .methods_chirho
            .iter()
            .any(|method_chirho| method_chirho == "nameChirho")
    );
}

// Row 536: a bundled `T(.., P)` loses `P` when `..` absorbs it, so a module
// that declares or enables pattern synonyms marks none of its types.
#[test]
fn a_module_that_can_bundle_pattern_synonyms_marks_nothing_chirho() {
    let declaring_chirho = module_chirho(
        "DeclaringChirho",
        None,
        vec![],
        vec![
            plain_data_chirho(
                "PlainChirho",
                vec![ordinary_constructor_chirho("PlainChirho")],
            ),
            DeclChirho::PatSynDeclChirho {
                name_chirho: name_chirho("SynonymChirho"),
                args_chirho: vec![],
                dir_chirho: PatSynDirChirho::ImplBidirChirho,
                pat_chirho: PatChirho::WildcardChirho(SpanChirho::DUMMY_CHIRHO),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        ],
    );
    assert!(
        build_iface_chirho(&declaring_chirho)
            .exports_chirho
            .complete_member_types_chirho
            .is_empty()
    );

    let mut enabling_chirho = module_chirho(
        "EnablingChirho",
        None,
        vec![],
        vec![plain_data_chirho(
            "PlainChirho",
            vec![ordinary_constructor_chirho("PlainChirho")],
        )],
    );
    enabling_chirho
        .extensions_chirho
        .push("PatternSynonyms".to_string());
    assert!(
        build_iface_chirho(&enabling_chirho)
            .exports_chirho
            .complete_member_types_chirho
            .is_empty()
    );
}

// Row 536: a re-exported type keeps only its own module's mark; the module
// that re-exports it marks nothing for it.
#[test]
fn a_reexported_type_is_not_marked_by_the_reexporting_module_chirho() {
    let source_iface_chirho = build_iface_chirho(&module_chirho(
        "SourceChirho",
        None,
        vec![],
        vec![associated_class_chirho()],
    ));
    assert!(
        source_iface_chirho
            .exports_chirho
            .complete_member_types_chirho
            .contains("ContainerChirho")
    );
    let facade_chirho = build_iface_with_imports_chirho(
        &facade_reexporting_chirho(ImportSpecChirho {
            hiding_chirho: false,
            items_chirho: vec![ImportItemChirho::TyConChirho {
                name_chirho: name_chirho("ContainerChirho"),
                members_chirho: ExportMembersChirho::AllChirho,
            }],
        }),
        std::slice::from_ref(&source_iface_chirho),
    );
    assert!(
        facade_chirho
            .exports_chirho
            .types_chirho
            .contains_key("ContainerChirho")
    );
    assert!(
        facade_chirho
            .exports_chirho
            .complete_member_types_chirho
            .is_empty()
    );
}

// Row 536: the builtin NonEmpty entry is base's declaration exactly, so it is
// marked; a partial builtin entry is not; and merging in a copy of the module
// that marks another type keeps both marks.
#[test]
fn the_builtin_non_empty_entry_is_complete_and_a_disjoint_merge_keeps_it_chirho() {
    let builtin_ifaces_chirho = builtin_module_ifaces_chirho();
    let non_empty_chirho = builtin_ifaces_chirho
        .iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Data.List.NonEmpty")
        .expect("a builtin Data.List.NonEmpty interface");
    let exports_chirho = &non_empty_chirho.exports_chirho;
    assert!(
        exports_chirho
            .complete_member_types_chirho
            .contains("NonEmpty")
    );
    assert_eq!(
        exports_chirho.types_chirho["NonEmpty"].constructors_chirho,
        ["(:|)"]
    );
    assert!(
        exports_chirho.types_chirho["NonEmpty"]
            .methods_chirho
            .is_empty()
    );

    // `Semigroup(..)` lists only `<>` here, so no builtin entry for it is marked.
    let semigroup_exporters_chirho: Vec<_> = builtin_ifaces_chirho
        .iter()
        .filter(|iface_chirho| {
            iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key("Semigroup")
        })
        .collect();
    assert!(!semigroup_exporters_chirho.is_empty());
    assert!(semigroup_exporters_chirho.iter().all(|iface_chirho| {
        !iface_chirho
            .exports_chirho
            .complete_member_types_chirho
            .contains("Semigroup")
    }));

    // A copy of the module that marks only a type the builtin entry lacks:
    // marks merge per type, so both survive.
    let source_chirho = build_iface_chirho(&module_chirho(
        "Data.List.NonEmpty",
        None,
        vec![],
        vec![plain_data_chirho(
            "ExtraChirho",
            vec![ordinary_constructor_chirho("ExtraChirho")],
        )],
    ));
    let merged_chirho = merge_module_ifaces_chirho(vec![non_empty_chirho.clone(), source_chirho]);
    assert_eq!(merged_chirho.len(), 1);
    let marks_chirho = &merged_chirho[0].exports_chirho.complete_member_types_chirho;
    assert!(marks_chirho.contains("NonEmpty"));
    assert!(marks_chirho.contains("ExtraChirho"));
}

/// A copy of module `MergedChirho` declaring `data T = T` for each name. With
/// pattern synonyms enabled it exports every one of them unmarked.
fn merged_copy_chirho(
    type_texts_chirho: &[&str],
    enables_pattern_synonyms_chirho: bool,
) -> ModuleIfaceChirho {
    let mut copy_chirho = module_chirho(
        "MergedChirho",
        None,
        vec![],
        type_texts_chirho
            .iter()
            .map(|type_text_chirho| {
                plain_data_chirho(
                    type_text_chirho,
                    vec![ordinary_constructor_chirho(type_text_chirho)],
                )
            })
            .collect(),
    );
    if enables_pattern_synonyms_chirho {
        copy_chirho
            .extensions_chirho
            .push("PatternSynonyms".to_string());
    }
    build_iface_chirho(&copy_chirho)
}

/// The sorted marks of the one interface these copies merge into.
fn merged_marks_chirho(copies_chirho: Vec<ModuleIfaceChirho>) -> Vec<String> {
    let merged_chirho = merge_module_ifaces_chirho(copies_chirho);
    assert_eq!(merged_chirho.len(), 1);
    let mut marks_chirho: Vec<String> = merged_chirho[0]
        .exports_chirho
        .complete_member_types_chirho
        .iter()
        .cloned()
        .collect();
    marks_chirho.sort();
    marks_chirho
}

// Row 536: copies of one module merge their marks per type. Disjoint marked
// types all survive, and a type every copy marks stays marked. A copy that
// exports the type unmarked may export a member no entry lists (a bundled
// pattern synonym), so it clears the mark in any order, while the marked
// copy's other types keep theirs.
#[test]
fn copies_of_one_module_merge_their_marks_per_type_chirho() {
    let unmarked_chirho = merged_copy_chirho(&["SharedChirho"], true);
    assert!(
        unmarked_chirho
            .exports_chirho
            .types_chirho
            .contains_key("SharedChirho")
    );
    assert!(
        unmarked_chirho
            .exports_chirho
            .complete_member_types_chirho
            .is_empty()
    );
    let marked_chirho = merged_copy_chirho(&["SharedChirho", "OwnChirho"], false);

    assert_eq!(
        merged_marks_chirho(vec![
            merged_copy_chirho(&["LeftChirho"], false),
            merged_copy_chirho(&["RightChirho"], false),
        ]),
        ["LeftChirho", "RightChirho"]
    );
    assert_eq!(
        merged_marks_chirho(vec![
            merged_copy_chirho(&["SharedChirho"], false),
            marked_chirho.clone(),
        ]),
        ["OwnChirho", "SharedChirho"]
    );
    for copies_chirho in [
        vec![marked_chirho.clone(), unmarked_chirho.clone()],
        vec![unmarked_chirho.clone(), marked_chirho.clone()],
        vec![
            marked_chirho.clone(),
            unmarked_chirho.clone(),
            marked_chirho.clone(),
        ],
    ] {
        assert_eq!(merged_marks_chirho(copies_chirho), ["OwnChirho"]);
    }
}
