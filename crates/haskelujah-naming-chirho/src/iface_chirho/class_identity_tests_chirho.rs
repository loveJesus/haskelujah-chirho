// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin class identity (#31176): the scope's trust and exclusion rules on hand-built
//! modules and interfaces, and the supplier guard over the real builtin interfaces.

use super::*;
use crate::iface_chirho::merge_module_ifaces_chirho;
use haskelujah_ast_chirho::decl_chirho::DeclChirho;
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho, ImportSpecChirho,
};
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, builtin_module_ifaces_chirho};

/// Builtin interfaces that export a table class but are not trusted to provide it: module
/// names GHC 9.14.1 does not ship.
const UNTRUSTED_CLASS_EXPORTERS_CHIRHO: &[(&str, &str)] = &[
    ("Show", "Show"),
    ("Monad", "Monad"),
    ("MonadFail", "MonadFail"),
];

const FUNCTOR_METHODS_CHIRHO: &[&str] = &["fmap", "<$"];

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// `class <name> a` with no methods.
fn class_decl_chirho(class_text_chirho: &str) -> DeclChirho {
    DeclChirho::ClassDeclChirho {
        context_chirho: vec![],
        name_chirho: name_chirho(class_text_chirho),
        type_vars_chirho: vec![name_chirho("a").into()],
        methods_chirho: vec![],
        associated_tfs_chirho: vec![],
        fundeps_chirho: vec![],
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

fn module_chirho(
    decls_chirho: Vec<DeclChirho>,
    imports_chirho: Vec<ImportDeclChirho>,
) -> ModuleChirho {
    ModuleChirho {
        name_chirho: name_chirho("Main"),
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
    module_text_chirho: &str,
    alias_chirho: Option<&str>,
    qualified_chirho: bool,
) -> ImportDeclChirho {
    ImportDeclChirho {
        package_chirho: None,
        source_chirho: false,
        module_chirho: name_chirho(module_text_chirho),
        qualified_chirho,
        alias_chirho: alias_chirho.map(name_chirho),
        spec_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// An interface of `origin_chirho` exporting `class_text_chirho` as a type entry that
/// lists `methods_chirho`.
fn class_iface_chirho(
    module_text_chirho: &str,
    class_text_chirho: &str,
    methods_chirho: &[&str],
    origin_chirho: IfaceOriginChirho,
) -> ModuleIfaceChirho {
    let mut exports_chirho = IfaceExportsChirho {
        origin_chirho,
        ..IfaceExportsChirho::default()
    };
    exports_chirho.types_chirho.insert(
        class_text_chirho.to_string(),
        IfaceTypeChirho {
            declaration_chirho: None,
            name_chirho: class_text_chirho.to_string(),
            constructors_chirho: vec![],
            methods_chirho: methods_chirho
                .iter()
                .map(|method_chirho| method_chirho.to_string())
                .collect(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    );
    ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: module_text_chirho.to_string(),
        exports_chirho,
    }
}

fn builtin_functor_chirho(module_text_chirho: &str) -> ModuleIfaceChirho {
    class_iface_chirho(
        module_text_chirho,
        "Functor",
        FUNCTOR_METHODS_CHIRHO,
        IfaceOriginChirho::BuiltinChirho,
    )
}

fn source_functor_chirho() -> ModuleIfaceChirho {
    class_iface_chirho(
        "FunctorsChirho",
        "Functor",
        FUNCTOR_METHODS_CHIRHO,
        IfaceOriginChirho::SourceChirho,
    )
}

fn proven_chirho(scope_chirho: &BuiltinClassScopeChirho, spelling_chirho: &str) -> bool {
    scope_chirho.class_chirho(spelling_chirho).is_some()
}

#[test]
fn an_unqualified_trusted_provider_binds_its_bare_and_qualified_spellings_chirho() {
    let scope_chirho = build_builtin_class_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("Prelude", None, false)]),
        &[builtin_functor_chirho("Prelude")],
    );
    let class_chirho = scope_chirho
        .class_chirho("Functor")
        .expect("Prelude's Functor is the builtin class");
    assert_eq!(class_chirho.parameter_arities_chirho, &[1]);
    assert!(proven_chirho(&scope_chirho, "Prelude.Functor"));
    assert!(!proven_chirho(&scope_chirho, "Main.Functor"));
}

#[test]
fn a_qualified_alias_binds_only_its_own_spelling_chirho() {
    let scope_chirho = build_builtin_class_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("Prelude", Some("P"), true)]),
        &[builtin_functor_chirho("Prelude")],
    );
    assert!(proven_chirho(&scope_chirho, "P.Functor"));
    assert!(!proven_chirho(&scope_chirho, "Functor"));
    assert!(!proven_chirho(&scope_chirho, "Prelude.Functor"));
}

#[test]
fn a_local_declaration_excludes_its_spellings_chirho() {
    let scope_chirho = build_builtin_class_scope_chirho(
        &module_chirho(
            vec![class_decl_chirho("Functor")],
            vec![
                import_chirho("Prelude", None, false),
                import_chirho("Prelude", Some("P"), true),
            ],
        ),
        &[builtin_functor_chirho("Prelude")],
    );
    assert!(!proven_chirho(&scope_chirho, "Functor"));
    assert!(!proven_chirho(&scope_chirho, "Main.Functor"));
    assert!(proven_chirho(&scope_chirho, "P.Functor"));
}

#[test]
fn a_source_interface_never_proves_a_builtin_class_chirho() {
    let scope_chirho = build_builtin_class_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("FunctorsChirho", None, false)]),
        &[source_functor_chirho()],
    );
    assert!(!proven_chirho(&scope_chirho, "Functor"));
    assert!(!proven_chirho(&scope_chirho, "FunctorsChirho.Functor"));
}

#[test]
fn exclusion_is_monotonic_whatever_is_imported_after_it_chirho() {
    let ifaces_chirho = [
        builtin_functor_chirho("Prelude"),
        builtin_functor_chirho("Data.Functor"),
        source_functor_chirho(),
    ];
    for order_chirho in [
        ["Data.Functor", "FunctorsChirho", "Prelude"],
        ["FunctorsChirho", "Prelude", "Data.Functor"],
    ] {
        let imports_chirho = order_chirho
            .iter()
            .map(|module_text_chirho| import_chirho(module_text_chirho, Some("F"), true))
            .collect();
        let scope_chirho = build_builtin_class_scope_chirho(
            &module_chirho(vec![], imports_chirho),
            &ifaces_chirho,
        );
        assert!(
            !proven_chirho(&scope_chirho, "F.Functor"),
            "order {order_chirho:?} restored a conflicted spelling"
        );
    }
}

#[test]
fn the_same_class_from_two_trusted_providers_stays_proven_chirho() {
    let scope_chirho = build_builtin_class_scope_chirho(
        &module_chirho(
            vec![],
            vec![
                import_chirho("Prelude", None, false),
                import_chirho("Data.Functor", None, false),
            ],
        ),
        &[
            builtin_functor_chirho("Prelude"),
            builtin_functor_chirho("Data.Functor"),
        ],
    );
    assert!(proven_chirho(&scope_chirho, "Functor"));
}

#[test]
fn an_untrusted_provider_or_a_methodless_entry_proves_nothing_chirho() {
    let untrusted_chirho =
        class_iface_chirho("Monad", "Monad", &[">>="], IfaceOriginChirho::BuiltinChirho);
    let methodless_chirho =
        class_iface_chirho("Prelude", "Functor", &[], IfaceOriginChirho::BuiltinChirho);
    let scope_chirho = build_builtin_class_scope_chirho(
        &module_chirho(
            vec![],
            vec![
                import_chirho("Monad", None, false),
                import_chirho("Prelude", None, false),
            ],
        ),
        &[untrusted_chirho, methodless_chirho],
    );
    assert!(!proven_chirho(&scope_chirho, "Monad"));
    assert!(!proven_chirho(&scope_chirho, "Functor"));
}

#[test]
fn hiding_the_class_leaves_it_unbound_chirho() {
    let mut hiding_chirho = import_chirho("Prelude", None, false);
    hiding_chirho.spec_chirho = Some(ImportSpecChirho {
        hiding_chirho: true,
        items_chirho: vec![ImportItemChirho::TyConChirho {
            name_chirho: name_chirho("Functor"),
            members_chirho: ExportMembersChirho::NoneChirho,
        }],
    });
    let scope_chirho = build_builtin_class_scope_chirho(
        &module_chirho(vec![], vec![hiding_chirho]),
        &[builtin_functor_chirho("Prelude")],
    );
    assert!(!proven_chirho(&scope_chirho, "Functor"));
    assert!(!proven_chirho(&scope_chirho, "Prelude.Functor"));
}

#[test]
fn every_listed_provider_exports_the_class_entry_with_its_witness_chirho() {
    let ifaces_chirho = builtin_module_ifaces_chirho();
    for entry_chirho in BUILTIN_CLASSES_CHIRHO {
        let class_text_chirho = entry_chirho.class_chirho.name_chirho;
        for provider_chirho in entry_chirho.providers_chirho {
            let iface_chirho = ifaces_chirho
                .iter()
                .find(|iface_chirho| iface_chirho.name_chirho == *provider_chirho)
                .unwrap_or_else(|| panic!("no builtin interface {provider_chirho}"));
            assert_eq!(
                trusted_builtin_class_chirho(iface_chirho, class_text_chirho)
                    .map(|trusted_chirho| trusted_chirho.class_chirho.name_chirho),
                Some(class_text_chirho),
                "{provider_chirho} must export class {class_text_chirho} listing `{}`",
                entry_chirho.witness_method_chirho
            );
        }
    }
}

#[test]
fn every_builtin_class_export_is_a_listed_or_named_untrusted_provider_chirho() {
    for iface_chirho in builtin_module_ifaces_chirho() {
        for entry_chirho in BUILTIN_CLASSES_CHIRHO {
            let class_text_chirho = entry_chirho.class_chirho.name_chirho;
            let Some(export_chirho) = iface_chirho
                .exports_chirho
                .types_chirho
                .get(class_text_chirho)
            else {
                continue;
            };
            if !export_chirho
                .methods_chirho
                .iter()
                .any(|method_chirho| method_chirho == entry_chirho.witness_method_chirho)
            {
                continue;
            }
            let module_text_chirho = iface_chirho.name_chirho.as_str();
            let listed_chirho = entry_chirho.providers_chirho.contains(&module_text_chirho);
            let untrusted_chirho =
                UNTRUSTED_CLASS_EXPORTERS_CHIRHO.contains(&(module_text_chirho, class_text_chirho));
            assert!(
                listed_chirho != untrusted_chirho,
                "builtin {module_text_chirho} exports class {class_text_chirho}: \
                 list it as exactly one of provider or untrusted"
            );
        }
    }
}

#[test]
fn the_table_names_each_class_once_at_a_known_parameter_kind_chirho() {
    for (index_chirho, entry_chirho) in BUILTIN_CLASSES_CHIRHO.iter().enumerate() {
        assert!(
            BUILTIN_CLASSES_CHIRHO[index_chirho + 1..]
                .iter()
                .all(|later_chirho| {
                    later_chirho.class_chirho.name_chirho != entry_chirho.class_chirho.name_chirho
                }),
            "{} is listed twice",
            entry_chirho.class_chirho.name_chirho
        );
        assert!(
            entry_chirho.class_chirho.parameter_arities_chirho == TYPE_PARAMETER_CHIRHO
                || entry_chirho.class_chirho.parameter_arities_chirho
                    == CONSTRUCTOR_PARAMETER_CHIRHO
        );
    }
}

#[test]
fn instance_head_coverage_is_a_producer_fact_not_a_builtin_origin_chirho() {
    let interfaces_chirho = builtin_module_ifaces_chirho();
    for name_chirho in ["Prelude", "Data.Kind", "Data.Proxy"] {
        let mut producer_chirho = interfaces_chirho
            .iter()
            .find(|interface_chirho| interface_chirho.name_chirho == name_chirho)
            .unwrap()
            .clone();
        // Retain this control's original metadata-free legacy merge lane.
        producer_chirho.exposure_chirho = None;
        assert!(
            producer_chirho
                .exports_chirho
                .unary_variable_instance_heads_covered_chirho
        );
        let unknown_chirho = ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: name_chirho.into(),
            exports_chirho: IfaceExportsChirho {
                origin_chirho: IfaceOriginChirho::BuiltinChirho,
                ..IfaceExportsChirho::default()
            },
        };
        for copies_chirho in [
            vec![producer_chirho.clone(), unknown_chirho.clone()],
            vec![unknown_chirho.clone(), producer_chirho.clone()],
        ] {
            let merged_chirho = merge_module_ifaces_chirho(copies_chirho);
            assert!(
                !merged_chirho[0]
                    .exports_chirho
                    .unary_variable_instance_heads_covered_chirho
            );
        }
    }
}

#[test]
fn source_interface_copies_cannot_acquire_instance_coverage_by_merge_order_chirho() {
    let mut producer_chirho = builtin_module_ifaces_chirho()
        .into_iter()
        .find(|interface_chirho| interface_chirho.name_chirho == "Data.Proxy")
        .unwrap();
    // Selection evidence is absent in this original legacy-copy control.
    producer_chirho.exposure_chirho = None;
    let source_chirho = ModuleIfaceChirho {
        exposure_chirho: None,
        name_chirho: producer_chirho.name_chirho.clone(),
        exports_chirho: IfaceExportsChirho::default(),
    };
    for copies_chirho in [
        vec![producer_chirho.clone(), source_chirho.clone()],
        vec![source_chirho.clone(), producer_chirho.clone()],
    ] {
        let merged_chirho = merge_module_ifaces_chirho(copies_chirho);
        assert!(
            !merged_chirho[0]
                .exports_chirho
                .unary_variable_instance_heads_covered_chirho
        );
    }
}
