// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Family scope and publication controls using real import/export syntax and
//! naming's admission function. No native execution is needed for this seam.

use super::*;
use haskelujah_ast_chirho::name_chirho::RawNameChirho;
use haskelujah_naming_chirho::iface_chirho::IfaceTypeChirho;
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::{FileIdChirho, SpanChirho};
use haskelujah_typing_chirho::infer_chirho::{
    TypeFamilyAliasResolutionChirho, TypeFamilyDefinitionChirho, TypeFamilyReductionChirho,
};

fn module_chirho(source_chirho: &str) -> ModuleChirho {
    let file_chirho = FileIdChirho::SYNTHETIC_CHIRHO;
    let tree_chirho = ParserChirho::new_chirho(source_chirho, file_chirho).parse_chirho();
    lower_module_chirho(&tree_chirho, file_chirho)
}

type AssociatedSpecChirho<'names_chirho> =
    (&'names_chirho str, &'names_chirho [&'names_chirho str]);

fn iface_chirho(
    module_chirho: &str,
    types_chirho: &[&str],
    associated_chirho: &[AssociatedSpecChirho<'_>],
) -> ModuleIfaceChirho {
    let mut iface_chirho = ModuleIfaceChirho {
        name_chirho: module_chirho.to_owned(),
        exports_chirho: Default::default(),
    };
    for name_chirho in types_chirho {
        iface_chirho.exports_chirho.types_chirho.insert(
            (*name_chirho).to_owned(),
            IfaceTypeChirho {
                name_chirho: (*name_chirho).to_owned(),
                constructors_chirho: Vec::new(),
                methods_chirho: Vec::new(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
    }
    for (parent_chirho, members_chirho) in associated_chirho {
        iface_chirho.exports_chirho.associated_types_chirho.insert(
            (*parent_chirho).to_owned(),
            members_chirho
                .iter()
                .map(|member_chirho| (*member_chirho).to_owned())
                .collect(),
        );
    }
    iface_chirho
}

fn family_chirho(
    registry_chirho: &mut TypeFamilyRegistryChirho,
    origin_chirho: &str,
    name_chirho: &str,
    exporters_chirho: &[&str],
) -> TypeFamilyIdChirho {
    let identity_chirho = TypeFamilyIdChirho::SourceChirho {
        module_chirho: origin_chirho.to_owned(),
        name_chirho: name_chirho.to_owned(),
    };
    registry_chirho
        .insert_chirho(
            TypeFamilyDefinitionChirho::new_chirho(
                identity_chirho.clone(),
                1,
                TypeFamilyReductionChirho::OpenChirho,
                Vec::new(),
            )
            .unwrap(),
        )
        .unwrap();
    for exporter_chirho in exporters_chirho {
        registry_chirho
            .export_alias_chirho(
                qualified_name_chirho(exporter_chirho, name_chirho),
                &identity_chirho,
            )
            .unwrap();
    }
    identity_chirho
}

fn scope_keys_chirho(
    scope_chirho: &TypeFamilyScopeChirho,
    registry_chirho: &TypeFamilyRegistryChirho,
    name_chirho: &str,
) -> BTreeSet<String> {
    scoped_family_keys_chirho(scope_chirho, name_chirho, registry_chirho)
}

fn export_identities_chirho(
    registry_chirho: &TypeFamilyRegistryChirho,
    name_chirho: &str,
) -> BTreeSet<TypeFamilyIdChirho> {
    match registry_chirho.resolve_export_chirho(name_chirho) {
        TypeFamilyAliasResolutionChirho::MissingChirho => BTreeSet::new(),
        TypeFamilyAliasResolutionChirho::UniqueChirho(definition_chirho) => {
            BTreeSet::from([definition_chirho.id_chirho().clone()])
        }
        TypeFamilyAliasResolutionChirho::AmbiguousChirho(identities_chirho) => {
            identities_chirho.into_iter().collect()
        }
    }
}

fn publish_chirho(
    module_chirho: &ModuleChirho,
    exported_chirho: &ModuleIfaceChirho,
    imported_chirho: &[ModuleIfaceChirho],
    registry_chirho: &mut TypeFamilyRegistryChirho,
) -> Result<(), FamilyExportsErrorChirho> {
    let scope_chirho =
        build_type_family_scope_chirho(module_chirho, imported_chirho, registry_chirho)?;
    publish_type_family_exports_chirho(
        module_chirho,
        exported_chirho,
        imported_chirho,
        &scope_chirho,
        registry_chirho,
    )
}

#[test]
fn imports_reuse_explicit_hiding_qualification_and_alias_admission_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let first_chirho = family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let qualified_chirho = family_chirho(&mut registry_chirho, "AChirho", "GChirho", &["AChirho"]);
    family_chirho(&mut registry_chirho, "BChirho", "FChirho", &["BChirho"]);
    let visible_chirho = family_chirho(&mut registry_chirho, "BChirho", "HChirho", &["BChirho"]);
    let module_chirho = module_chirho(
        "module UserChirho where\nimport AChirho (FChirho)\nimport qualified AChirho as QChirho (GChirho)\nimport BChirho hiding (FChirho)\n",
    );
    let imported_chirho = [
        iface_chirho("AChirho", &["FChirho", "GChirho"], &[]),
        iface_chirho("BChirho", &["FChirho", "HChirho"], &[]),
    ];
    let scope_chirho =
        build_type_family_scope_chirho(&module_chirho, &imported_chirho, &registry_chirho).unwrap();
    assert_eq!(
        scope_keys_chirho(&scope_chirho, &registry_chirho, "FChirho"),
        BTreeSet::from([first_chirho.key_chirho()])
    );
    assert_eq!(
        scope_keys_chirho(&scope_chirho, &registry_chirho, "AChirho.FChirho"),
        BTreeSet::from([first_chirho.key_chirho()])
    );
    assert_eq!(
        scope_keys_chirho(&scope_chirho, &registry_chirho, "QChirho.GChirho"),
        BTreeSet::from([qualified_chirho.key_chirho()])
    );
    for absent_chirho in ["GChirho", "AChirho.GChirho", "BChirho.FChirho"] {
        assert!(
            scope_keys_chirho(&scope_chirho, &registry_chirho, absent_chirho).is_empty(),
            "{absent_chirho}"
        );
    }
    assert_eq!(
        scope_keys_chirho(&scope_chirho, &registry_chirho, "HChirho"),
        BTreeSet::from([visible_chirho.key_chirho()])
    );
}

#[test]
fn a_value_export_cannot_authorize_a_type_family_import_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(
        &mut registry_chirho,
        "AChirho",
        "ValueOnlyChirho",
        &["AChirho"],
    );
    let mut imported_chirho = iface_chirho("AChirho", &["OrdinaryChirho"], &[]);
    imported_chirho
        .exports_chirho
        .types_chirho
        .get_mut("OrdinaryChirho")
        .unwrap()
        .constructors_chirho
        .push("ValueOnlyChirho".to_owned());
    let module_chirho = module_chirho("module UserChirho where\nimport AChirho\n");
    let scope_chirho =
        build_type_family_scope_chirho(&module_chirho, &[imported_chirho], &registry_chirho)
            .unwrap();
    assert!(scope_keys_chirho(&scope_chirho, &registry_chirho, "ValueOnlyChirho").is_empty());
    assert!(
        scope_keys_chirho(&scope_chirho, &registry_chirho, "AChirho.ValueOnlyChirho").is_empty()
    );
}

#[test]
fn local_type_shadowing_and_forward_family_ids_precede_registration_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let imported_f_chirho = family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    family_chirho(&mut registry_chirho, "AChirho", "GChirho", &["AChirho"]);
    let module_chirho = module_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule LocalChirho where\nimport AChirho\ndata FChirho = ValueChirho\ntype family GChirho aChirho\nclass CChirho aChirho where\n  type HChirho aChirho\n",
    );
    let imported_chirho = [iface_chirho("AChirho", &["FChirho", "GChirho"], &[])];
    let scope_chirho =
        build_type_family_scope_chirho(&module_chirho, &imported_chirho, &registry_chirho).unwrap();
    assert!(scope_keys_chirho(&scope_chirho, &registry_chirho, "FChirho").is_empty());
    assert_eq!(
        scope_keys_chirho(&scope_chirho, &registry_chirho, "AChirho.FChirho"),
        BTreeSet::from([imported_f_chirho.key_chirho()])
    );
    for name_chirho in ["GChirho", "HChirho"] {
        let expected_chirho = TypeFamilyIdChirho::SourceChirho {
            module_chirho: "LocalChirho".to_owned(),
            name_chirho: name_chirho.to_owned(),
        }
        .key_chirho();
        assert!(
            registry_chirho
                .definition_chirho(&expected_chirho)
                .is_none()
        );
        assert_eq!(
            scope_keys_chirho(&scope_chirho, &registry_chirho, name_chirho),
            BTreeSet::from([expected_chirho.clone()])
        );
        assert_eq!(
            scope_keys_chirho(
                &scope_chirho,
                &registry_chirho,
                &qualified_name_chirho("LocalChirho", name_chirho)
            ),
            BTreeSet::from([expected_chirho])
        );
    }
    assert!(scope_keys_chirho(&scope_chirho, &registry_chirho, "CChirho").is_empty());
}

#[test]
fn qualified_explicit_export_selects_its_origin_beside_a_bare_namesake_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let selected_chirho = family_chirho(&mut registry_chirho, "BChirho", "FChirho", &["BChirho"]);
    let module_chirho = module_chirho(
        "module ForwardChirho (QChirho.FChirho) where\nimport AChirho (FChirho)\nimport qualified BChirho as QChirho (FChirho)\n",
    );
    let imported_chirho = [
        iface_chirho("AChirho", &["FChirho"], &[]),
        iface_chirho("BChirho", &["FChirho"], &[]),
    ];
    publish_chirho(
        &module_chirho,
        &iface_chirho("ForwardChirho", &["FChirho"], &[]),
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
        BTreeSet::from([selected_chirho])
    );
}

#[test]
fn module_export_intersects_entities_instead_of_matching_only_the_name_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let second_chirho = family_chirho(&mut registry_chirho, "BChirho", "FChirho", &["BChirho"]);
    let imported_chirho = [
        iface_chirho("AChirho", &["FChirho"], &[]),
        iface_chirho("BChirho", &["FChirho"], &[]),
    ];
    let exported_chirho = iface_chirho("ForwardChirho", &["FChirho"], &[]);
    let disjoint_chirho = module_chirho(
        "module ForwardChirho (module QChirho) where\nimport AChirho (FChirho)\nimport qualified BChirho as QChirho (FChirho)\n",
    );
    publish_chirho(
        &disjoint_chirho,
        &exported_chirho,
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
    let overlapping_chirho = module_chirho(
        "module ForwardChirho (module QChirho) where\nimport AChirho (FChirho)\nimport BChirho as QChirho (FChirho)\n",
    );
    publish_chirho(
        &overlapping_chirho,
        &exported_chirho,
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
        BTreeSet::from([second_chirho])
    );
}

#[test]
fn forwarding_one_entity_deduplicates_but_distinct_origins_remain_ambiguous_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let shared_chirho = family_chirho(
        &mut registry_chirho,
        "OriginChirho",
        "FChirho",
        &["AChirho", "BChirho"],
    );
    let distinct_chirho =
        family_chirho(&mut registry_chirho, "OtherChirho", "FChirho", &["CChirho"]);
    let imported_chirho = [
        iface_chirho("AChirho", &["FChirho"], &[]),
        iface_chirho("BChirho", &["FChirho"], &[]),
        iface_chirho("CChirho", &["FChirho"], &[]),
    ];
    let first_chirho = module_chirho(
        "module ForwardChirho (module QChirho) where\nimport AChirho (FChirho)\nimport qualified BChirho as QChirho (FChirho)\n",
    );
    publish_chirho(
        &first_chirho,
        &iface_chirho("ForwardChirho", &["FChirho"], &[]),
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
        BTreeSet::from([shared_chirho.clone()])
    );
    let ambiguous_chirho = module_chirho(
        "module AmbiguousChirho (FChirho) where\nimport AChirho\nimport BChirho\nimport CChirho\n",
    );
    publish_chirho(
        &ambiguous_chirho,
        &iface_chirho("AmbiguousChirho", &["FChirho"], &[]),
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "AmbiguousChirho.FChirho"),
        BTreeSet::from([shared_chirho, distinct_chirho])
    );
}

#[test]
fn associated_exports_follow_the_parent_qualifier_and_admission_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let first_chirho = family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let second_chirho = family_chirho(&mut registry_chirho, "BChirho", "FChirho", &["BChirho"]);
    let mut imported_chirho = [
        iface_chirho(
            "AChirho",
            &["CChirho", "FChirho"],
            &[("CChirho", &["FChirho"])],
        ),
        iface_chirho(
            "BChirho",
            &["CChirho", "FChirho"],
            &[("CChirho", &["FChirho"])],
        ),
    ];
    let qualified_chirho = module_chirho(
        "module ForwardChirho (QChirho.CChirho(..)) where\nimport AChirho (CChirho(..))\nimport qualified BChirho as QChirho (CChirho(..))\n",
    );
    let exported_chirho = iface_chirho(
        "ForwardChirho",
        &["CChirho", "FChirho"],
        &[("CChirho", &["FChirho"])],
    );
    publish_chirho(
        &qualified_chirho,
        &exported_chirho,
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
        BTreeSet::from([second_chirho])
    );
    imported_chirho[1] = iface_chirho(
        "BChirho",
        &["DChirho", "FChirho"],
        &[("DChirho", &["FChirho"])],
    );
    let unqualified_chirho = module_chirho(
        "module ForwardChirho (CChirho(..)) where\nimport AChirho (CChirho(..))\nimport BChirho (DChirho(..))\n",
    );
    publish_chirho(
        &unqualified_chirho,
        &exported_chirho,
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
        BTreeSet::from([first_chirho])
    );
    for import_list_chirho in ["FChirho", "CChirho"] {
        let source_chirho = format!(
            "module ForwardChirho (CChirho(..)) where\nimport AChirho ({import_list_chirho})\n"
        );
        publish_chirho(
            &module_chirho(&source_chirho),
            &exported_chirho,
            &imported_chirho,
            &mut registry_chirho,
        )
        .unwrap();
        assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
    }
}

#[test]
fn a_local_parent_shadows_imported_associated_families_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let local_chirho = family_chirho(&mut registry_chirho, "ForwardChirho", "GChirho", &[]);
    let module_chirho = module_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule ForwardChirho (CChirho(..)) where\nimport AChirho (CChirho(..))\nclass CChirho aChirho where\n  type GChirho aChirho\n",
    );
    let imported_chirho = [iface_chirho(
        "AChirho",
        &["CChirho", "FChirho"],
        &[("CChirho", &["FChirho"])],
    )];
    let exported_chirho = iface_chirho(
        "ForwardChirho",
        &["CChirho", "FChirho", "GChirho"],
        &[("CChirho", &["GChirho"])],
    );
    publish_chirho(
        &module_chirho,
        &exported_chirho,
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.GChirho"),
        BTreeSet::from([local_chirho])
    );
}

#[test]
fn default_exports_and_the_actual_interface_both_bound_publication_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let imported_chirho = [iface_chirho("AChirho", &["FChirho"], &[])];
    let local_chirho = family_chirho(&mut registry_chirho, "ForwardChirho", "FChirho", &[]);
    let own_chirho = module_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule ForwardChirho where\ntype family FChirho aChirho\n",
    );
    publish_chirho(
        &own_chirho,
        &iface_chirho("ForwardChirho", &["FChirho"], &[]),
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
        BTreeSet::from([local_chirho])
    );
    let implicit_chirho = module_chirho("module ForwardChirho where\nimport AChirho\n");
    publish_chirho(
        &implicit_chirho,
        &iface_chirho("ForwardChirho", &["FChirho"], &[]),
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
    let explicit_chirho = module_chirho("module ForwardChirho (FChirho) where\nimport AChirho\n");
    publish_chirho(
        &explicit_chirho,
        &iface_chirho("ForwardChirho", &[], &[]),
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
}

#[test]
fn publication_replaces_changed_origins_and_clears_removed_or_ordinary_types_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let second_chirho = family_chirho(&mut registry_chirho, "BChirho", "FChirho", &["BChirho"]);
    let imported_chirho = vec![
        iface_chirho("AChirho", &["FChirho"], &[]),
        iface_chirho("BChirho", &["FChirho"], &[]),
    ];
    let exported_chirho = iface_chirho("ForwardChirho", &["FChirho"], &[]);
    for origin_chirho in ["AChirho", "BChirho"] {
        let source_chirho =
            format!("module ForwardChirho (FChirho) where\nimport {origin_chirho}\n");
        publish_chirho(
            &module_chirho(&source_chirho),
            &exported_chirho,
            &imported_chirho,
            &mut registry_chirho,
        )
        .unwrap();
    }
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
        BTreeSet::from([second_chirho])
    );
    let ordinary_chirho =
        module_chirho("module ForwardChirho (FChirho) where\ndata FChirho = ValueChirho\n");
    publish_chirho(
        &ordinary_chirho,
        &exported_chirho,
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
    let forwarding_chirho = module_chirho("module ForwardChirho (FChirho) where\nimport AChirho\n");
    publish_chirho(
        &forwarding_chirho,
        &exported_chirho,
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    let removed_chirho = module_chirho("module ForwardChirho () where\n");
    publish_chirho(
        &removed_chirho,
        &iface_chirho("ForwardChirho", &[], &[]),
        &imported_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
}

#[test]
fn missing_local_definition_fails_before_any_export_alias_changes_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let previous_chirho = family_chirho(
        &mut registry_chirho,
        "PreviousChirho",
        "GChirho",
        &["ForwardChirho"],
    );
    family_chirho(&mut registry_chirho, "AChirho", "GChirho", &["AChirho"]);
    let module_chirho = module_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule ForwardChirho (FChirho, GChirho) where\nimport AChirho (GChirho)\ntype family FChirho aChirho\n",
    );
    let result_chirho = publish_chirho(
        &module_chirho,
        &iface_chirho("ForwardChirho", &["FChirho", "GChirho"], &[]),
        &[iface_chirho("AChirho", &["GChirho"], &[])],
        &mut registry_chirho,
    );
    assert!(matches!(
        result_chirho,
        Err(FamilyExportsErrorChirho::MissingDefinitionChirho(_))
    ));
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.GChirho"),
        BTreeSet::from([previous_chirho])
    );
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
}

#[test]
fn an_erased_qualified_operator_is_reported_without_guessing_a_suffix_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let mut module_chirho = module_chirho("module ForwardChirho where\n");
    module_chirho.exports_chirho = Some(vec![ExportSpecChirho::TyConChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "QChirho.:.:",
            SpanChirho::DUMMY_CHIRHO,
        )),
        members_chirho: ExportMembersChirho::NoneChirho,
    }]);
    let result_chirho = publish_chirho(
        &module_chirho,
        &iface_chirho("ForwardChirho", &[":.:"], &[]),
        &[],
        &mut registry_chirho,
    );
    assert!(matches!(
        result_chirho,
        Err(FamilyExportsErrorChirho::UnstructuredQualifiedNameChirho(_))
    ));
}

#[test]
fn another_modules_interface_cannot_authorize_publication_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let module_chirho = module_chirho("module ForwardChirho (FChirho) where\nimport AChirho\n");
    let result_chirho = publish_chirho(
        &module_chirho,
        &iface_chirho("OtherChirho", &["FChirho"], &[]),
        &[iface_chirho("AChirho", &["FChirho"], &[])],
        &mut registry_chirho,
    );
    assert!(matches!(
        result_chirho,
        Err(FamilyExportsErrorChirho::MismatchedInterfaceChirho { .. })
    ));
    assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
    assert!(export_identities_chirho(&registry_chirho, "OtherChirho.FChirho").is_empty());
}

#[test]
fn value_only_export_names_do_not_become_family_identity_errors_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let mut module_chirho = module_chirho("module ForwardChirho where\n");
    module_chirho.exports_chirho = Some(vec![ExportSpecChirho::VarChirho(NameChirho::RawChirho(
        RawNameChirho::unqualified_chirho("PChirho..", SpanChirho::DUMMY_CHIRHO),
    ))]);
    publish_chirho(
        &module_chirho,
        &iface_chirho("ForwardChirho", &[], &[]),
        &[],
        &mut registry_chirho,
    )
    .unwrap();
}

#[test]
fn a_qualified_operator_uses_its_exact_admitted_spelling_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let selected_chirho = family_chirho(&mut registry_chirho, "AChirho", ":.:", &["AChirho"]);
    let mut module_chirho =
        module_chirho("module ForwardChirho where\nimport qualified AChirho as QChirho\n");
    module_chirho.exports_chirho = Some(vec![ExportSpecChirho::TyConChirho {
        name_chirho: NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            "QChirho.:.:",
            SpanChirho::DUMMY_CHIRHO,
        )),
        members_chirho: ExportMembersChirho::NoneChirho,
    }]);
    publish_chirho(
        &module_chirho,
        &iface_chirho("ForwardChirho", &[":.:"], &[]),
        &[iface_chirho("AChirho", &[":.:"], &[])],
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.:.:"),
        BTreeSet::from([selected_chirho.clone()])
    );
    module_chirho.exports_chirho = Some(vec![ExportSpecChirho::VarChirho(NameChirho::RawChirho(
        RawNameChirho::unqualified_chirho("QChirho.:.:", SpanChirho::DUMMY_CHIRHO),
    ))]);
    publish_chirho(
        &module_chirho,
        &iface_chirho("ForwardChirho", &[":.:"], &[]),
        &[iface_chirho("AChirho", &[":.:"], &[])],
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "ForwardChirho.:.:"),
        BTreeSet::from([selected_chirho])
    );
}

#[test]
fn separate_imports_can_admit_a_parent_and_its_same_entity_member_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let selected_chirho = family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let imported_chirho = [iface_chirho(
        "AChirho",
        &["CChirho", "FChirho"],
        &[("CChirho", &["FChirho"])],
    )];
    let exported_chirho = iface_chirho(
        "ForwardChirho",
        &["CChirho", "FChirho"],
        &[("CChirho", &["FChirho"])],
    );
    for source_chirho in [
        "module ForwardChirho (CChirho(FChirho)) where\nimport AChirho (CChirho)\nimport AChirho (FChirho)\n",
        "module ForwardChirho (QChirho.CChirho(FChirho)) where\nimport qualified AChirho as QChirho (CChirho)\nimport qualified AChirho as QChirho (FChirho)\n",
        "module ForwardChirho (QChirho.CChirho(FChirho)) where\nimport qualified AChirho as QChirho (CChirho)\nimport qualified AChirho as RChirho (FChirho)\n",
        "module ForwardChirho (QChirho.CChirho(FChirho)) where\nimport qualified AChirho as QChirho (CChirho)\nimport AChirho (FChirho)\n",
    ] {
        publish_chirho(
            &module_chirho(source_chirho),
            &exported_chirho,
            &imported_chirho,
            &mut registry_chirho,
        )
        .unwrap();
        assert_eq!(
            export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho"),
            BTreeSet::from([selected_chirho.clone()])
        );
    }
}

#[test]
fn explicit_associated_family_needs_its_entity_not_a_namesake_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    family_chirho(&mut registry_chirho, "BChirho", "FChirho", &["BChirho"]);
    let imported_chirho = [
        iface_chirho(
            "AChirho",
            &["CChirho", "FChirho"],
            &[("CChirho", &["FChirho"])],
        ),
        iface_chirho("BChirho", &["FChirho"], &[]),
    ];
    for source_chirho in [
        "module ForwardChirho (QChirho.CChirho(FChirho)) where\nimport qualified AChirho as QChirho (CChirho)\n",
        "module ForwardChirho (QChirho.CChirho(FChirho)) where\nimport qualified AChirho as QChirho (CChirho)\nimport qualified BChirho as RChirho (FChirho)\n",
    ] {
        let result_chirho = publish_chirho(
            &module_chirho(source_chirho),
            &iface_chirho(
                "ForwardChirho",
                &["CChirho", "FChirho"],
                &[("CChirho", &["FChirho"])],
            ),
            &imported_chirho,
            &mut registry_chirho,
        );
        assert!(matches!(
            result_chirho,
            Err(FamilyExportsErrorChirho::UnadmittedAssociatedFamilyChirho { .. })
        ));
        assert!(export_identities_chirho(&registry_chirho, "ForwardChirho.FChirho").is_empty());
    }
}

#[test]
fn central_frontend_honors_the_qualified_parent_oracle_chirho() {
    let mut registry_chirho = crate::seed_builtin_type_families_chirho();
    let expected_chirho = family_chirho(&mut registry_chirho, "AChirho", "FChirho", &["AChirho"]);
    let mut imported_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    imported_chirho.push(iface_chirho(
        "AChirho",
        &["CChirho", "FChirho"],
        &[("CChirho", &["FChirho"])],
    ));
    let run_chirho = |source_chirho: &str| {
        crate::run_frontend_with_type_synonyms_and_type_families_chirho(
            source_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
            &imported_chirho,
            &std::collections::HashMap::new(),
            &std::collections::HashMap::new(),
            &registry_chirho,
        )
    };
    for member_import_chirho in [
        "import qualified AChirho as QChirho (FChirho)",
        "import qualified AChirho as RChirho (FChirho)",
        "import AChirho (FChirho)",
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeFamilies #-}}\nmodule ForwardChirho (QChirho.CChirho(FChirho)) where\nimport qualified AChirho as QChirho (CChirho)\n{member_import_chirho}\n"
        );
        let checked_chirho = run_chirho(&source_chirho)
            .expect("GHC accepts the same member through each import path");
        assert_eq!(
            export_identities_chirho(
                &checked_chirho.infer_result_chirho.type_families_chirho,
                "ForwardChirho.FChirho"
            ),
            BTreeSet::from([expected_chirho.clone()])
        );
    }
    let rejected_chirho = run_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule ForwardChirho (QChirho.CChirho(FChirho)) where\nimport qualified AChirho as QChirho (CChirho)\n",
    );
    let diagnostics_chirho = rejected_chirho
        .err()
        .expect("GHC-76037: the member was not imported");
    assert!(diagnostics_chirho.to_string().contains("FChirho"));
    assert!(diagnostics_chirho.to_string().contains("not in scope"));
}

#[test]
fn exported_synonym_family_identity_wins_over_the_importers_namesake_chirho() {
    let initial_chirho = crate::seed_builtin_type_families_chirho();
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let exporter_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule AChirho (FChirho, SChirho) where\ntype family FChirho aChirho where\n  FChirho Int = Bool\ntype SChirho = FChirho Int\n",
        FileIdChirho::SYNTHETIC_CHIRHO, &ifaces_chirho,
        &std::collections::HashMap::new(), &std::collections::HashMap::new(), &initial_chirho,
    ).expect("the defining module is valid");
    let exported_iface_chirho = build_family_aware_iface_chirho(
        &exporter_chirho.module_chirho,
        &ifaces_chirho,
        &exporter_chirho.infer_result_chirho.type_families_chirho,
    );
    let synonyms_chirho = crate::exported_type_synonyms_from_module_chirho(
        &exporter_chirho.module_chirho,
        &exported_iface_chirho,
        &exporter_chirho.resolved_imported_type_synonyms_chirho,
        &exporter_chirho.infer_result_chirho,
    );
    let exported_key_chirho = match &synonyms_chirho["SChirho"].1 {
        haskelujah_ast_chirho::ty_chirho::TypeChirho::AppChirho { fun_chirho, .. } => {
            match fun_chirho.as_ref() {
                haskelujah_ast_chirho::ty_chirho::TypeChirho::ConChirho(name_chirho) => name_chirho
                    .family_key_chirho()
                    .expect("exporter stamps its own family")
                    .to_owned(),
                _ => panic!("synonym head must remain a family reference"),
            }
        }
        _ => panic!("synonym keeps its application"),
    };
    assert_eq!(
        exported_key_chirho,
        TypeFamilyIdChirho::SourceChirho {
            module_chirho: "AChirho".to_owned(),
            name_chirho: "FChirho".to_owned()
        }
        .key_chirho()
    );
    ifaces_chirho.push(exported_iface_chirho);
    let importer_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule BChirho where\nimport AChirho (SChirho)\ntype family FChirho aChirho where\n  FChirho Int = Char\nvalueChirho :: SChirho\nvalueChirho = True\n",
        FileIdChirho::SYNTHETIC_CHIRHO,
        &ifaces_chirho,
        &std::collections::HashMap::new(),
        &synonyms_chirho,
        &exporter_chirho.infer_result_chirho.type_families_chirho,
    );
    assert!(
        importer_chirho.is_ok(),
        "importer must use A's Bool result, not its own Char result: {:?}",
        importer_chirho.err()
    );
}

#[test]
fn central_frontend_preserves_an_associated_member_through_a_real_forwarder_chirho() {
    let mut registry_chirho = crate::seed_builtin_type_families_chirho();
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    for source_chirho in [
        "{-# LANGUAGE TypeFamilies #-}\nmodule OriginChirho (CChirho(..)) where\nclass CChirho aChirho where\n  type FChirho aChirho\n",
        "{-# LANGUAGE TypeFamilies #-}\nmodule MemberChirho (FChirho) where\nimport OriginChirho (FChirho)\n",
        "{-# LANGUAGE TypeFamilies #-}\nmodule UserChirho (QChirho.CChirho(FChirho)) where\nimport qualified OriginChirho as QChirho (CChirho)\nimport qualified MemberChirho as RChirho (FChirho)\n",
    ] {
        let frontend_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
            source_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
            &ifaces_chirho,
            &std::collections::HashMap::new(),
            &std::collections::HashMap::new(),
            &registry_chirho,
        )
        .expect("the associated family keeps the defining entity through the forwarder");
        let iface_chirho = build_family_aware_iface_chirho(
            &frontend_chirho.module_chirho,
            &ifaces_chirho,
            &frontend_chirho.infer_result_chirho.type_families_chirho,
        );
        assert!(
            iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key("FChirho")
        );
        registry_chirho = frontend_chirho.infer_result_chirho.type_families_chirho;
        ifaces_chirho.push(iface_chirho);
    }
    assert_eq!(
        export_identities_chirho(&registry_chirho, "UserChirho.FChirho"),
        BTreeSet::from([TypeFamilyIdChirho::SourceChirho {
            module_chirho: "OriginChirho".to_owned(),
            name_chirho: "FChirho".to_owned(),
        }])
    );
}

#[test]
fn central_frontend_keeps_explicit_operator_type_exports_out_of_the_value_namespace_chirho() {
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let origin_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
        "{-# LANGUAGE TypeFamilies, TypeOperators, ExplicitNamespaces #-}\nmodule OriginChirho (type (.), (.)) where\nimport Prelude hiding ((.))\ntype family aChirho . bChirho where\n  Int . Bool = Char\n(.) :: Int -> Int -> Int\naChirho . bChirho = aChirho + bChirho\n",
        FileIdChirho::SYNTHETIC_CHIRHO, &ifaces_chirho,
        &std::collections::HashMap::new(), &std::collections::HashMap::new(),
        &crate::seed_builtin_type_families_chirho(),
    ).expect("the oracle's origin defines both namespaces");
    let origin_iface_chirho = build_family_aware_iface_chirho(
        &origin_chirho.module_chirho,
        &ifaces_chirho,
        &origin_chirho.infer_result_chirho.type_families_chirho,
    );
    assert!(
        origin_iface_chirho
            .exports_chirho
            .types_chirho
            .contains_key(".")
    );
    assert!(
        origin_iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key(".")
    );
    ifaces_chirho.push(origin_iface_chirho);
    let expected_chirho = BTreeSet::from([TypeFamilyIdChirho::SourceChirho {
        module_chirho: "OriginChirho".to_owned(),
        name_chirho: ".".to_owned(),
    }]);
    for (exports_chirho, exports_type_chirho, type_only_chirho) in [
        ("(QChirho..)", false, false),
        ("type (QChirho..)", true, true),
        ("type (QChirho..), (QChirho..)", true, false),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeFamilies, TypeOperators, ExplicitNamespaces #-}}\nmodule ForwardChirho ({exports_chirho}) where\nimport qualified OriginChirho as QChirho\n"
        );
        let forwarded_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
            &source_chirho,
            FileIdChirho::SYNTHETIC_CHIRHO,
            &ifaces_chirho,
            &std::collections::HashMap::new(),
            &std::collections::HashMap::new(),
            &origin_chirho.infer_result_chirho.type_families_chirho,
        )
        .expect("export filtering does not turn the value-only route into a family error");
        let forwarded_iface_chirho = build_family_aware_iface_chirho(
            &forwarded_chirho.module_chirho,
            &ifaces_chirho,
            &forwarded_chirho.infer_result_chirho.type_families_chirho,
        );
        assert_eq!(
            forwarded_iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key("."),
            exports_type_chirho,
            "{exports_chirho}"
        );
        assert_eq!(
            export_identities_chirho(
                &forwarded_chirho.infer_result_chirho.type_families_chirho,
                "ForwardChirho.."
            ),
            if exports_type_chirho {
                expected_chirho.clone()
            } else {
                BTreeSet::new()
            }
        );
        if type_only_chirho {
            assert!(
                !forwarded_iface_chirho
                    .exports_chirho
                    .values_chirho
                    .contains_key(".")
            );
        }
        let mut imported_chirho = ifaces_chirho.clone();
        imported_chirho.push(forwarded_iface_chirho);
        let consumed_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
            "{-# LANGUAGE TypeFamilies, TypeOperators, ExplicitNamespaces #-}\nmodule UserChirho where\nimport ForwardChirho (type (.))\nidentityChirho :: (Int . Bool) -> Char\nidentityChirho valueChirho = valueChirho\n",
            FileIdChirho::SYNTHETIC_CHIRHO,
            &imported_chirho,
            &std::collections::HashMap::new(),
            &std::collections::HashMap::new(),
            &forwarded_chirho.infer_result_chirho.type_families_chirho,
        );
        assert_eq!(
            consumed_chirho.is_ok(),
            exports_type_chirho,
            "{exports_chirho}: {:?}",
            consumed_chirho.err()
        );
    }
    // The oracle also accepts the two qualified-value forwarding consumers.
    // Their existing value-lookup gap is outside this family-namespace change.
}

#[test]
fn symbolic_associated_members_keep_their_written_export_namespace_chirho() {
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let body_chirho = "class CChirho aChirho where\n  type aChirho # bChirho\n  (#) :: aChirho -> aChirho -> Int\n";
    let origin_source_chirho = format!(
        "{{-# LANGUAGE TypeFamilies, TypeOperators, ExplicitNamespaces #-}}\nmodule OriginChirho (CChirho(..)) where\n{body_chirho}"
    );
    let origin_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
        &origin_source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        &ifaces_chirho,
        &std::collections::HashMap::new(),
        &std::collections::HashMap::new(),
        &crate::seed_builtin_type_families_chirho(),
    )
    .expect("the class declares the symbolic name in each namespace");
    ifaces_chirho.push(build_family_aware_iface_chirho(
        &origin_chirho.module_chirho,
        &ifaces_chirho,
        &origin_chirho.infer_result_chirho.type_families_chirho,
    ));
    for (member_chirho, exports_type_chirho) in [("(#)", false), ("type (#)", true)] {
        for local_chirho in [false, true] {
            let source_chirho = if local_chirho {
                format!(
                    "{{-# LANGUAGE TypeFamilies, TypeOperators, ExplicitNamespaces #-}}\nmodule UserChirho (UserChirho.CChirho({member_chirho})) where\n{body_chirho}"
                )
            } else {
                format!(
                    "{{-# LANGUAGE TypeFamilies, TypeOperators, ExplicitNamespaces #-}}\nmodule UserChirho (QChirho.CChirho({member_chirho})) where\nimport qualified OriginChirho as QChirho\n"
                )
            };
            let frontend_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
                &source_chirho,
                FileIdChirho::SYNTHETIC_CHIRHO,
                &ifaces_chirho,
                &std::collections::HashMap::new(),
                &std::collections::HashMap::new(),
                &origin_chirho.infer_result_chirho.type_families_chirho,
            )
            .expect("member namespaces do not turn into a scope error");
            let actual_chirho = build_family_aware_iface_chirho(
                &frontend_chirho.module_chirho,
                &ifaces_chirho,
                &frontend_chirho.infer_result_chirho.type_families_chirho,
            );
            assert!(
                actual_chirho
                    .exports_chirho
                    .types_chirho
                    .contains_key("CChirho")
            );
            assert_eq!(
                actual_chirho.exports_chirho.types_chirho.contains_key("#"),
                exports_type_chirho
            );
            assert_eq!(
                actual_chirho.exports_chirho.values_chirho.contains_key("#"),
                !exports_type_chirho
            );
            assert_eq!(
                !export_identities_chirho(
                    &frontend_chirho.infer_result_chirho.type_families_chirho,
                    "UserChirho.#"
                )
                .is_empty(),
                exports_type_chirho
            );
        }
    }
}

#[test]
fn self_module_export_keeps_local_and_admitted_self_alias_entities_chirho() {
    let mut registry_chirho = TypeFamilyRegistryChirho::new_chirho();
    let local_chirho = family_chirho(&mut registry_chirho, "SelfChirho", "FChirho", &[]);
    let imported_chirho = family_chirho(&mut registry_chirho, "AChirho", "GChirho", &["AChirho"]);
    family_chirho(&mut registry_chirho, "BChirho", "HChirho", &["BChirho"]);
    let module_chirho = module_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule SelfChirho (module SelfChirho) where\nimport AChirho as SelfChirho (GChirho)\nimport qualified BChirho as SelfChirho (HChirho)\ntype family FChirho aChirho\n",
    );
    let ifaces_chirho = [
        iface_chirho("AChirho", &["GChirho"], &[]),
        iface_chirho("BChirho", &["HChirho"], &[]),
    ];
    publish_chirho(
        &module_chirho,
        &iface_chirho("SelfChirho", &["FChirho", "GChirho", "HChirho"], &[]),
        &ifaces_chirho,
        &mut registry_chirho,
    )
    .unwrap();
    assert_eq!(
        export_identities_chirho(&registry_chirho, "SelfChirho.FChirho"),
        BTreeSet::from([local_chirho])
    );
    assert_eq!(
        export_identities_chirho(&registry_chirho, "SelfChirho.GChirho"),
        BTreeSet::from([imported_chirho])
    );
    assert!(export_identities_chirho(&registry_chirho, "SelfChirho.HChirho").is_empty());
}
