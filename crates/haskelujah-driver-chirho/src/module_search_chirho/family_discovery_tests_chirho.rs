// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Behavioral controls for declaration-only discovery. Compiler execution is
//! confined to the ordinary test runner; these tests never invoke external tools.

use super::super::DiscoveredSourceModuleChirho;
use super::*;
use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_parser_chirho::lower_chirho::lower_module_chirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_typing_chirho::infer_chirho::seed_type_family_registry_chirho;

fn module_chirho(source_chirho: &str) -> ModuleChirho {
    let tree_chirho =
        ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho();
    let mut module_chirho = lower_module_chirho(&tree_chirho, FileIdChirho::SYNTHETIC_CHIRHO);
    crate::inject_prelude_import_chirho(&mut module_chirho);
    module_chirho
}

fn discovery_chirho(
    checked_chirho: &str,
    sources_chirho: &[&str],
) -> (TypeFamilyRegistryChirho, Vec<ModuleIfaceChirho>) {
    discovery_with_boots_chirho(checked_chirho, sources_chirho, &[])
}

struct SourceScratchChirho(std::path::PathBuf);

impl SourceScratchChirho {
    fn new_chirho() -> Self {
        static NEXT_CHIRHO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let path_chirho = std::env::temp_dir().join(format!(
            "source_family_boot_chirho_{}_{}",
            std::process::id(),
            NEXT_CHIRHO.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path_chirho).expect("create source family boot fixture");
        Self(path_chirho)
    }
}

impl Drop for SourceScratchChirho {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn discovery_with_boots_chirho(
    checked_chirho: &str,
    sources_chirho: &[&str],
    boots_chirho: &[(&str, &str)],
) -> (TypeFamilyRegistryChirho, Vec<ModuleIfaceChirho>) {
    let checked_module_chirho = module_chirho(checked_chirho);
    let scratch_chirho = (!boots_chirho.is_empty()).then(SourceScratchChirho::new_chirho);
    let checked_path_chirho = scratch_chirho.as_ref().map(|scratch_chirho| {
        scratch_chirho.0.join(format!(
            "{}.hs",
            checked_module_chirho.name_chirho.full_name_chirho()
        ))
    });
    if let Some(path_chirho) = &checked_path_chirho {
        std::fs::write(path_chirho, checked_chirho).expect("write checked source fixture");
    }
    for (name_chirho, source_chirho) in boots_chirho {
        std::fs::write(
            scratch_chirho
                .as_ref()
                .expect("boot fixture directory")
                .0
                .join(name_chirho),
            source_chirho,
        )
        .expect("write boot fixture");
    }
    let mut modules_chirho = DiscoveredSourceModulesChirho::new();
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    for source_chirho in sources_chirho {
        let module_chirho = module_chirho(source_chirho);
        let iface_chirho = haskelujah_naming_chirho::build_iface_with_imports_chirho(
            &module_chirho,
            &ifaces_chirho,
        );
        admit_authoritative_iface_chirho(&mut ifaces_chirho, iface_chirho);
        let name_chirho = module_chirho.name_chirho.full_name_chirho();
        let path_chirho = scratch_chirho
            .as_ref()
            .map_or_else(std::path::PathBuf::new, |scratch_chirho| {
                scratch_chirho.0.join(format!("{name_chirho}.hs"))
            });
        if scratch_chirho.is_some() {
            std::fs::write(&path_chirho, source_chirho).expect("write companion fixture");
        }
        modules_chirho.insert(
            name_chirho,
            DiscoveredSourceModuleChirho {
                module_chirho,
                path_chirho,
            },
        );
    }
    let mut registry_chirho = seed_type_family_registry_chirho();
    register_discovered_type_families_chirho(
        &checked_module_chirho,
        DiscoveredFamilyContextChirho {
            modules_chirho: &modules_chirho,
            ifaces_chirho: &mut ifaces_chirho,
            registry_chirho: &mut registry_chirho,
            imported_synonyms_chirho: &ImportedTypeSynonymsChirho::new(),
            checked_path_chirho: checked_path_chirho.as_deref(),
        },
    )
    .expect("authoritative source family declarations register");
    (registry_chirho, ifaces_chirho)
}

fn checks_chirho(
    source_chirho: &str,
    registry_chirho: &TypeFamilyRegistryChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) {
    let result_chirho = crate::run_frontend_with_type_synonyms_and_type_families_chirho(
        source_chirho,
        FileIdChirho::SYNTHETIC_CHIRHO,
        ifaces_chirho,
        &std::collections::HashMap::new(),
        &ImportedTypeSynonymsChirho::new(),
        registry_chirho,
    );
    assert!(
        result_chirho.is_ok(),
        "registered imported family should check: {:?}",
        result_chirho.err()
    );
}

#[test]
fn imported_closed_family_is_registered_from_its_source_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies #-}\nmodule CheckedChirho where\nimport OriginChirho\nvalueChirho :: FChirho Int\nvalueChirho = 3\n";
    let (registry_chirho, ifaces_chirho) = discovery_chirho(
        source_chirho,
        &[
            "{-# LANGUAGE TypeFamilies #-}\nmodule OriginChirho where\ntype family FChirho aChirho where\n  FChirho Int = Int\n",
        ],
    );
    assert!(matches!(
        registry_chirho.resolve_export_chirho("OriginChirho.FChirho"),
        TypeFamilyAliasResolutionChirho::UniqueChirho(_)
    ));
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
}

#[test]
fn source_companion_can_instance_the_checked_modules_open_family_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies #-}\nmodule OwnerChirho where\nimport InstancesChirho\ntype family FChirho aChirho\ntype family UnrelatedChirho aChirho where\n  UnrelatedChirho aChirho = Int\nvalueChirho :: FChirho Int\nvalueChirho = 3\n";
    let (registry_chirho, ifaces_chirho) = discovery_with_boots_chirho(
        source_chirho,
        &[
            "{-# LANGUAGE TypeFamilies #-}\nmodule InstancesChirho where\nimport {-# SOURCE #-} OwnerChirho\ntype instance FChirho Int = Int\n",
        ],
        &[(
            "OwnerChirho.hs-boot",
            "{-# LANGUAGE TypeFamilies #-}\nmodule OwnerChirho where\ntype family FChirho aChirho\n",
        )],
    );
    let TypeFamilyAliasResolutionChirho::UniqueChirho(definition_chirho) =
        registry_chirho.resolve_export_chirho("OwnerChirho.FChirho")
    else {
        panic!("source owner publishes its real open family");
    };
    assert_eq!(definition_chirho.branches_chirho().len(), 1);
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
}

#[test]
fn unimported_same_directory_family_source_has_no_registration_effect_chirho() {
    let source_chirho = "module CheckedChirho where\nvalueChirho :: Int\nvalueChirho = 3\n";
    let (registry_chirho, ifaces_chirho) = discovery_chirho(
        source_chirho,
        &[
            "{-# LANGUAGE TypeFamilies #-}\nmodule UnusedChirho where\ntype family HiddenChirho aChirho\ntype instance MissingChirho Int = Bool\n",
        ],
    );
    assert!(matches!(
        registry_chirho.resolve_export_chirho("UnusedChirho.HiddenChirho"),
        TypeFamilyAliasResolutionChirho::MissingChirho
    ));
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
}

#[test]
fn qualified_value_operator_exports_do_not_publish_type_family_aliases_chirho() {
    let source_chirho = "module CheckedChirho where\nimport ValuesChirho ()\nimport TypesChirho ()\nvalueChirho :: Int\nvalueChirho = 3\n";
    let (registry_chirho, ifaces_chirho) = discovery_chirho(
        source_chirho,
        &[
            "{-# LANGUAGE TypeFamilies, TypeOperators, ExplicitNamespaces #-}\nmodule OriginChirho (type (++), (++)) where\ntype family aChirho ++ bChirho where\n  aChirho ++ bChirho = aChirho\n(++) :: Int -> Int -> Int\naChirho ++ bChirho = aChirho\n",
            "{-# LANGUAGE TypeOperators #-}\nmodule ValuesChirho ((QChirho.++)) where\nimport qualified OriginChirho as QChirho\n",
            "{-# LANGUAGE TypeOperators, ExplicitNamespaces #-}\nmodule TypesChirho (type (QChirho.++)) where\nimport qualified OriginChirho as QChirho\n",
        ],
    );
    assert!(matches!(
        registry_chirho.resolve_export_chirho("ValuesChirho.++"),
        TypeFamilyAliasResolutionChirho::MissingChirho
    ));
    assert!(matches!(
        registry_chirho.resolve_export_chirho("TypesChirho.++"),
        TypeFamilyAliasResolutionChirho::UniqueChirho(_)
    ));
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
}

#[test]
fn closed_family_forwarding_is_ready_before_an_importers_equation_chirho() {
    let source_chirho = "{-# LANGUAGE TypeFamilies #-}\nmodule CheckedChirho where\nimport ForwardChirho\ntype family OwnChirho aChirho where\n  OwnChirho aChirho = FChirho aChirho\nvalueChirho :: OwnChirho Int\nvalueChirho = 3\n";
    let (registry_chirho, ifaces_chirho) = discovery_chirho(
        source_chirho,
        &[
            "{-# LANGUAGE TypeFamilies #-}\nmodule OriginChirho where\ntype family FChirho aChirho where\n  FChirho Int = Int\n",
            "module ForwardChirho (module OriginChirho) where\nimport OriginChirho\n",
        ],
    );
    checks_chirho(source_chirho, &registry_chirho, &ifaces_chirho);
}

#[test]
fn failed_registration_leaves_registry_and_interfaces_unchanged_chirho() {
    let checked_chirho = module_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule CheckedChirho where\nimport BadChirho\n",
    );
    let bad_chirho = module_chirho(
        "{-# LANGUAGE TypeFamilies #-}\nmodule BadChirho where\ntype instance MissingChirho Int = Bool\n",
    );
    let mut modules_chirho = DiscoveredSourceModulesChirho::new();
    modules_chirho.insert(
        "BadChirho".to_owned(),
        DiscoveredSourceModuleChirho {
            module_chirho: bad_chirho,
            path_chirho: std::path::PathBuf::new(),
        },
    );
    let mut ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let before_ifaces_chirho = format!("{ifaces_chirho:?}");
    let mut registry_chirho = seed_type_family_registry_chirho();
    let before_registry_chirho = format!("{registry_chirho:?}");
    let result_chirho = register_discovered_type_families_chirho(
        &checked_chirho,
        DiscoveredFamilyContextChirho {
            modules_chirho: &modules_chirho,
            ifaces_chirho: &mut ifaces_chirho,
            registry_chirho: &mut registry_chirho,
            imported_synonyms_chirho: &ImportedTypeSynonymsChirho::new(),
            checked_path_chirho: None,
        },
    );
    assert!(result_chirho.is_err());
    assert_eq!(format!("{ifaces_chirho:?}"), before_ifaces_chirho);
    assert_eq!(format!("{registry_chirho:?}"), before_registry_chirho);
}
