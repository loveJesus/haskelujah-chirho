// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The promoted-constructor scope on hand-built modules and interfaces: each provenance
//! rule of row 686's oracle set (C15b, C17, C18b, E01, E03) and the shapes it refuses.

use super::*;
use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;
use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};
use haskelujah_ast_chirho::promotion_chirho::{
    is_promoted_key_chirho, is_unknown_promoted_key_chirho,
};
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::{
    IfaceExportsChirho, IfaceTypeChirho, IfaceValueChirho, builtin_module_ifaces_chirho,
};

fn name_chirho(text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

fn qualified_chirho(qualifier_chirho: &str, text_chirho: &str) -> NameChirho {
    NameChirho::RawChirho(RawNameChirho::qualified_chirho(
        qualifier_chirho,
        text_chirho,
        SpanChirho::DUMMY_CHIRHO,
    ))
}

/// `data <type> = <constructor>` with no fields.
fn data_decl_chirho(type_chirho: &str, constructor_chirho: &str) -> DeclChirho {
    DeclChirho::DataDeclChirho {
        name_chirho: name_chirho(type_chirho),
        type_vars_chirho: Vec::new(),
        constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
            name_chirho: name_chirho(constructor_chirho),
            fields_chirho: Vec::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        deriving_chirho: Vec::new(),
        kind_sig_chirho: None,
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
    module_chirho: &str,
    alias_chirho: Option<&str>,
    qualified_chirho: bool,
) -> ImportDeclChirho {
    ImportDeclChirho {
        module_chirho: name_chirho(module_chirho),
        source_chirho: false,
        qualified_chirho,
        alias_chirho: alias_chirho.map(name_chirho),
        spec_chirho: None,
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    }
}

/// An interface exporting `type_chirho` with `constructors_chirho`, each recorded as
/// defined in `defining_chirho`.
fn iface_chirho(
    module_chirho: &str,
    type_chirho: &str,
    constructors_chirho: &[&str],
    defining_chirho: Option<&str>,
    origin_chirho: IfaceOriginChirho,
) -> ModuleIfaceChirho {
    let mut exports_chirho = IfaceExportsChirho {
        origin_chirho,
        ..IfaceExportsChirho::default()
    };
    exports_chirho.types_chirho.insert(
        type_chirho.to_string(),
        IfaceTypeChirho {
            declaration_chirho: None,
            name_chirho: type_chirho.to_string(),
            constructors_chirho: constructors_chirho
                .iter()
                .map(|c_chirho| c_chirho.to_string())
                .collect(),
            methods_chirho: Vec::new(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        },
    );
    for constructor_chirho in constructors_chirho {
        exports_chirho.values_chirho.insert(
            constructor_chirho.to_string(),
            IfaceValueChirho {
                availability_chirho:
                    crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
                name_chirho: constructor_chirho.to_string(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
                defining_module_chirho: defining_chirho.map(str::to_string),
            },
        );
    }
    ModuleIfaceChirho {
        name_chirho: module_chirho.to_string(),
        exports_chirho,
    }
}

fn source_key_chirho(module_chirho: &str, constructor_chirho: &str) -> PromotionResolutionChirho {
    PromotionResolutionChirho::ResolvedChirho(PromotedKeyChirho::source_chirho(
        module_chirho,
        constructor_chirho,
    ))
}

fn is_unknown_chirho(resolution_chirho: &PromotionResolutionChirho) -> bool {
    matches!(
        resolution_chirho,
        PromotionResolutionChirho::UnresolvedChirho(_)
    ) && is_unknown_promoted_key_chirho(resolution_chirho.key_chirho().as_str_chirho())
}

#[test]
fn keys_are_opaque_and_keep_origins_apart_chirho() {
    let builtin_chirho =
        PromotedKeyChirho::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "True");
    let source_chirho = PromotedKeyChirho::source_chirho("GHC.Types", "True");
    let unknown_chirho = PromotedKeyChirho::unknown_chirho("", "True");
    for key_chirho in [&builtin_chirho, &source_chirho, &unknown_chirho] {
        assert!(is_promoted_key_chirho(key_chirho.as_str_chirho()));
    }
    assert_ne!(
        builtin_chirho, source_chirho,
        "a source module spelling GHC.Types is not the compiler"
    );
    assert!(is_unknown_promoted_key_chirho(
        unknown_chirho.as_str_chirho()
    ));
    assert!(!is_unknown_promoted_key_chirho(
        builtin_chirho.as_str_chirho()
    ));
    assert_ne!(
        PromotedKeyChirho::source_chirho("A.B", "C"),
        PromotedKeyChirho::source_chirho("A", "B.C"),
        "the module length delimits module from constructor"
    );
}

#[test]
fn a_local_constructor_is_this_modules_and_the_type_namespace_comes_first_chirho() {
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(vec![data_decl_chirho("TChirho", "TChirho")], vec![]),
        &[],
    );
    let local_chirho = source_key_chirho("Main", "TChirho");
    assert_eq!(
        scope_chirho.quoted_chirho(&name_chirho("TChirho")),
        local_chirho,
        "C15b's 'TChirho"
    );
    assert_eq!(
        scope_chirho.quoted_chirho(&qualified_chirho("Main", "TChirho")),
        local_chirho
    );
    assert_eq!(
        scope_chirho.unquoted_chirho(&name_chirho("TChirho")),
        None,
        "an unquoted homonym is the type"
    );
}

#[test]
fn a_class_associated_family_is_in_the_type_namespace_chirho() {
    // `class CChirho a where type FamChirho a` beside `data DChirho = FamChirho`: GHC
    // reads an unquoted `FamChirho` as the family, and only `'FamChirho` as the constructor.
    let class_chirho = DeclChirho::ClassDeclChirho {
        context_chirho: Vec::new(),
        name_chirho: name_chirho("CChirho"),
        type_vars_chirho: Vec::new(),
        methods_chirho: Vec::new(),
        associated_tfs_chirho: vec![haskelujah_ast_chirho::decl_chirho::AssocTypeFamilyChirho {
            default_arguments_chirho: None,
            name_chirho: name_chirho("FamChirho"),
            result_kind_chirho: None,
            type_vars_chirho: vec![name_chirho("a").into()],
            data_family_chirho: false,
            default_rhs_chirho: None,
            default_params_chirho: Vec::new(),
            default_binders_chirho: None,
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }],
        fundeps_chirho: Vec::new(),
        span_chirho: SpanChirho::DUMMY_CHIRHO,
    };
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(
            vec![class_chirho, data_decl_chirho("DChirho", "FamChirho")],
            vec![],
        ),
        &[],
    );
    assert_eq!(
        scope_chirho.unquoted_chirho(&name_chirho("FamChirho")),
        None
    );
    assert_eq!(
        scope_chirho.quoted_chirho(&name_chirho("FamChirho")),
        source_key_chirho("Main", "FamChirho")
    );
}

#[test]
fn an_unquoted_constructor_with_no_type_is_promoted_chirho() {
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(vec![data_decl_chirho("SideChirho", "UChirho")], vec![]),
        &[],
    );
    assert_eq!(
        scope_chirho.unquoted_chirho(&name_chirho("UChirho")),
        Some(source_key_chirho("Main", "UChirho"))
    );
    assert_eq!(
        scope_chirho.unquoted_chirho(&name_chirho("SideChirho")),
        None
    );
}

#[test]
fn the_standard_true_needs_the_builtin_interface_chirho() {
    let prelude_chirho = builtin_module_ifaces_chirho()
        .into_iter()
        .find(|iface_chirho| iface_chirho.name_chirho == "Prelude")
        .expect("builtin Prelude");
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("Prelude", None, false)]),
        &[prelude_chirho],
    );
    let builtin_true_chirho = PromotionResolutionChirho::ResolvedChirho(
        PromotedKeyChirho::builtin_chirho(BUILTIN_CONSTRUCTOR_MODULE_CHIRHO, "True"),
    );
    assert_eq!(
        scope_chirho.quoted_chirho(&name_chirho("True")),
        builtin_true_chirho,
        "C17's 'True"
    );
    assert_eq!(
        scope_chirho.unquoted_chirho(&name_chirho("True")),
        Some(builtin_true_chirho),
        "C17's unquoted True is the same constructor"
    );

    let source_bool_chirho = iface_chirho(
        "MyBoolChirho",
        "Bool",
        &["False", "True"],
        Some("MyBoolChirho"),
        IfaceOriginChirho::SourceChirho,
    );
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("MyBoolChirho", None, false)]),
        &[source_bool_chirho],
    );
    assert_eq!(
        scope_chirho.quoted_chirho(&name_chirho("True")),
        source_key_chirho("MyBoolChirho", "True"),
        "a source module's True is its own (E03)"
    );
}

#[test]
fn a_builtin_interface_outside_the_compilers_table_proves_nothing_chirho() {
    let builtin_first_chirho = iface_chirho(
        "Data.Monoid",
        "First",
        &["First"],
        None,
        IfaceOriginChirho::BuiltinChirho,
    );
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("Data.Monoid", None, false)]),
        &[builtin_first_chirho],
    );
    assert!(is_unknown_chirho(
        &scope_chirho.quoted_chirho(&name_chirho("First"))
    ));
}

#[test]
fn a_stub_interface_proves_no_origin_chirho() {
    let stub_chirho = iface_chirho(
        "StubChirho",
        "SideChirho",
        &["TChirho"],
        None,
        IfaceOriginChirho::SourceChirho,
    );
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("StubChirho", None, false)]),
        &[stub_chirho],
    );
    assert!(is_unknown_chirho(
        &scope_chirho.quoted_chirho(&name_chirho("TChirho"))
    ));
}

#[test]
fn an_alias_and_a_reexport_name_one_constructor_chirho() {
    // E01: DefChirho defines the constructor; ReexportChirho re-exports it, so its
    // interface records DefChirho as the definer.
    let defining_chirho = iface_chirho(
        "DefChirho",
        "SideChirho",
        &["TChirho"],
        Some("DefChirho"),
        IfaceOriginChirho::SourceChirho,
    );
    let reexport_chirho = iface_chirho(
        "ReexportChirho",
        "SideChirho",
        &["TChirho"],
        Some("DefChirho"),
        IfaceOriginChirho::SourceChirho,
    );
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(
            vec![],
            vec![
                import_chirho("DefChirho", Some("AlphaChirho"), true),
                import_chirho("ReexportChirho", Some("BetaChirho"), true),
            ],
        ),
        &[defining_chirho, reexport_chirho],
    );
    let constructor_chirho = source_key_chirho("DefChirho", "TChirho");
    assert_eq!(
        scope_chirho.quoted_chirho(&qualified_chirho("AlphaChirho", "TChirho")),
        constructor_chirho
    );
    assert_eq!(
        scope_chirho.quoted_chirho(&qualified_chirho("BetaChirho", "TChirho")),
        constructor_chirho
    );
    assert_eq!(
        scope_chirho.unquoted_chirho(&qualified_chirho("BetaChirho", "TChirho")),
        Some(constructor_chirho)
    );
}

#[test]
fn distinct_modules_sharing_a_spelling_stay_distinct_chirho() {
    // C18b: the same type and constructor spelling defined in two modules.
    let left_chirho = iface_chirho(
        "LeftConChirho",
        "SideChirho",
        &["TChirho"],
        Some("LeftConChirho"),
        IfaceOriginChirho::SourceChirho,
    );
    let right_chirho = iface_chirho(
        "RightConChirho",
        "SideChirho",
        &["TChirho"],
        Some("RightConChirho"),
        IfaceOriginChirho::SourceChirho,
    );
    let qualified_scope_chirho = build_promotion_scope_chirho(
        &module_chirho(
            vec![],
            vec![
                import_chirho("LeftConChirho", Some("LeftChirho"), true),
                import_chirho("RightConChirho", Some("RightChirho"), true),
            ],
        ),
        &[left_chirho.clone(), right_chirho.clone()],
    );
    assert_ne!(
        qualified_scope_chirho.quoted_chirho(&qualified_chirho("LeftChirho", "TChirho")),
        qualified_scope_chirho.quoted_chirho(&qualified_chirho("RightChirho", "TChirho"))
    );
    let unqualified_scope_chirho = build_promotion_scope_chirho(
        &module_chirho(
            vec![],
            vec![
                import_chirho("LeftConChirho", None, false),
                import_chirho("RightConChirho", None, false),
            ],
        ),
        &[left_chirho, right_chirho],
    );
    assert!(
        is_unknown_chirho(&unqualified_scope_chirho.quoted_chirho(&name_chirho("TChirho"))),
        "an ambiguous bare spelling proves no constructor"
    );
}

#[test]
fn a_value_no_exported_type_lists_is_no_promotion_chirho() {
    let mut functions_chirho = iface_chirho(
        "FunctionsChirho",
        "SideChirho",
        &[],
        Some("FunctionsChirho"),
        IfaceOriginChirho::SourceChirho,
    );
    functions_chirho.exports_chirho.values_chirho.insert(
        "MkPatternChirho".to_string(),
        IfaceValueChirho {
            availability_chirho: crate::iface_chirho::IfaceValueAvailabilityChirho::AvailableChirho,
            name_chirho: "MkPatternChirho".to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
            defining_module_chirho: Some("FunctionsChirho".to_string()),
        },
    );
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(vec![], vec![import_chirho("FunctionsChirho", None, false)]),
        &[functions_chirho],
    );
    assert_eq!(
        scope_chirho.unquoted_chirho(&name_chirho("MkPatternChirho")),
        None,
        "a pattern synonym or function is not a constructor"
    );
    assert!(is_unknown_chirho(
        &scope_chirho.quoted_chirho(&name_chirho("MkPatternChirho"))
    ));
}

#[test]
fn trusted_providers_give_standard_constructors_the_compilers_identity_chirho() {
    // H01's 'Nothing/'Just and 'Left/'Right, H02's 'Lifted/'Unlifted and H03's RuntimeRep:
    // the ordered reducer needs them as distinct, proven constructors.
    let builtins_chirho = builtin_module_ifaces_chirho();
    let scope_chirho = build_promotion_scope_chirho(
        &module_chirho(
            vec![],
            vec![
                import_chirho("Prelude", None, false),
                import_chirho("GHC.Exts", None, false),
            ],
        ),
        &builtins_chirho,
    );
    for (constructor_chirho, defining_chirho) in [
        ("Nothing", "GHC.Maybe"),
        ("Just", "GHC.Maybe"),
        ("Left", "Data.Either"),
        ("Right", "Data.Either"),
        ("Lifted", "GHC.Types"),
        ("Unlifted", "GHC.Types"),
        ("IntRep", "GHC.Types"),
        ("BoxedRep", "GHC.Types"),
    ] {
        assert_eq!(
            scope_chirho.quoted_chirho(&name_chirho(constructor_chirho)),
            PromotionResolutionChirho::ResolvedChirho(PromotedKeyChirho::builtin_chirho(
                defining_chirho,
                constructor_chirho,
            )),
            "'{constructor_chirho}"
        );
    }
}

#[test]
fn builtin_homonyms_and_source_forwarders_prove_nothing_chirho() {
    // A builtin interface outside Maybe's named providers (a strict Maybe) and a source
    // module forwarding the standard constructors are no proof of the standard identity.
    for provider_chirho in [
        iface_chirho(
            "Data.Strict.Maybe",
            "Maybe",
            &["Nothing", "Just"],
            None,
            IfaceOriginChirho::BuiltinChirho,
        ),
        iface_chirho(
            "ForwarderChirho",
            "Maybe",
            &["Nothing", "Just"],
            None,
            IfaceOriginChirho::SourceChirho,
        ),
    ] {
        let imported_chirho = provider_chirho.name_chirho.clone();
        let scope_chirho = build_promotion_scope_chirho(
            &module_chirho(vec![], vec![import_chirho(&imported_chirho, None, false)]),
            &[provider_chirho],
        );
        assert!(
            is_unknown_chirho(&scope_chirho.quoted_chirho(&name_chirho("Just"))),
            "{imported_chirho}"
        );
    }
}

#[test]
fn every_trusted_parent_is_listed_by_exactly_its_named_providers_chirho() {
    // A parent that names its providers is listed, with constructors, by those builtin
    // interfaces and no others, and every listing of a table parent has the table's
    // constructors. A future builtin homonym fails here instead of sharing base's identity.
    let builtins_chirho = builtin_module_ifaces_chirho();
    for entry_chirho in BUILTIN_PROMOTED_PARENTS_CHIRHO {
        let expected_chirho: std::collections::BTreeSet<&str> =
            entry_chirho.constructors_chirho.iter().copied().collect();
        let mut listing_chirho: Vec<&str> = Vec::new();
        for iface_chirho in &builtins_chirho {
            let Some(ty_chirho) = iface_chirho
                .exports_chirho
                .types_chirho
                .get(entry_chirho.parent_chirho)
            else {
                continue;
            };
            if ty_chirho.constructors_chirho.is_empty() {
                continue;
            }
            let listed_chirho: std::collections::BTreeSet<&str> = ty_chirho
                .constructors_chirho
                .iter()
                .map(String::as_str)
                .collect();
            assert_eq!(
                listed_chirho, expected_chirho,
                "{} lists {}",
                iface_chirho.name_chirho, entry_chirho.parent_chirho
            );
            listing_chirho.push(iface_chirho.name_chirho.as_str());
        }
        if let Some(providers_chirho) = entry_chirho.providers_chirho {
            listing_chirho.sort_unstable();
            let mut named_chirho = providers_chirho.to_vec();
            named_chirho.sort_unstable();
            assert_eq!(
                listing_chirho, named_chirho,
                "{}",
                entry_chirho.parent_chirho
            );
        }
    }
}
