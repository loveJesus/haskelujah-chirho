// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked boot promises are compared after both frontend runs have succeeded.
//! Workflow: compiler-pipeline-chirho/module-search-authority-chirho.
use crate::FrontendResultChirho;
use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use std::collections::HashMap;

pub(super) fn validate_boot_declarations_chirho(
    boot_chirho: &FrontendResultChirho,
) -> Result<(), String> {
    for declaration_chirho in &boot_chirho.module_chirho.decls_chirho {
        match declaration_chirho {
            DeclChirho::TypeSigChirho { .. }
            | DeclChirho::DataDeclChirho { .. }
            | DeclChirho::NewtypeDeclChirho { .. }
            | DeclChirho::TypeAliasDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. }
            | DeclChirho::TypeFamilyDeclChirho {
                closed_chirho: false,
                ..
            } => {}
            DeclChirho::FunBindChirho { name_chirho, .. } => {
                return Err(format!(
                    "boot contract cannot contain a value definition: {}",
                    name_chirho.text_chirho()
                ));
            }
            _ => {
                return Err(format!(
                    "boot contract for {} contains a declaration whose agreement is not represented yet",
                    boot_chirho.module_chirho.name_chirho.full_name_chirho()
                ));
            }
        }
    }
    Ok(())
}

fn declaration_name_chirho(declaration_chirho: &DeclChirho) -> Option<&str> {
    match declaration_chirho {
        DeclChirho::DataDeclChirho { name_chirho, .. }
        | DeclChirho::NewtypeDeclChirho { name_chirho, .. }
        | DeclChirho::TypeAliasDeclChirho { name_chirho, .. }
        | DeclChirho::TypeFamilyDeclChirho { name_chirho, .. }
        | DeclChirho::ClassDeclChirho { name_chirho, .. }
        | DeclChirho::FunBindChirho { name_chirho, .. } => Some(name_chirho.text_chirho()),
        _ => None,
    }
}

fn same_value_chirho(
    name_chirho: &str,
    boot_chirho: &FrontendResultChirho,
    implementation_chirho: &FrontendResultChirho,
) -> bool {
    let expected_chirho = boot_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho(name_chirho);
    let actual_chirho = implementation_chirho
        .infer_result_chirho
        .env_chirho
        .lookup_chirho(name_chirho);
    matches!((expected_chirho, actual_chirho), (Some(expected_chirho), Some(actual_chirho)) if expected_chirho.alpha_equivalent_chirho(actual_chirho))
}

fn constructor_name_chirho(constructor_chirho: &ConDeclChirho) -> &str {
    match constructor_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.text_chirho(),
    }
}

fn same_constructors_chirho(
    promised_chirho: &[ConDeclChirho],
    actual_chirho: &[ConDeclChirho],
    boot_chirho: &FrontendResultChirho,
    implementation_chirho: &FrontendResultChirho,
) -> bool {
    // No constructors in a boot data head denotes an abstract nominal promise.
    promised_chirho.is_empty() || (promised_chirho.len() == actual_chirho.len()
        && promised_chirho.iter().zip(actual_chirho).all(|(promised_chirho, actual_chirho)| {
            let name_chirho = constructor_name_chirho(promised_chirho);
            if name_chirho != constructor_name_chirho(actual_chirho) || !same_value_chirho(name_chirho, boot_chirho, implementation_chirho) { return false; }
            match (promised_chirho, actual_chirho) {
                (ConDeclChirho::OrdinaryChirho { fields_chirho: promised_chirho, .. }, ConDeclChirho::OrdinaryChirho { fields_chirho: actual_chirho, .. }) => {
                    promised_chirho.len() == actual_chirho.len() && promised_chirho.iter().zip(actual_chirho).all(|(promised_chirho, actual_chirho)| promised_chirho.0 == actual_chirho.0)
                },
                (ConDeclChirho::RecordChirho { fields_chirho: promised_chirho, .. }, ConDeclChirho::RecordChirho { fields_chirho: actual_chirho, .. }) => {
                    let layout_chirho = |fields_chirho: &[haskelujah_ast_chirho::decl_chirho::FieldDeclChirho]| fields_chirho.iter().flat_map(|field_chirho| field_chirho.names_chirho.iter().map(|name_chirho| (name_chirho.text_chirho().to_owned(), field_chirho.strictness_chirho))).collect::<Vec<_>>();
                    layout_chirho(promised_chirho) == layout_chirho(actual_chirho)
                },
                (ConDeclChirho::GadtChirho { .. }, ConDeclChirho::GadtChirho { .. }) => true,
                _ => false,
            }
        }))
}

pub(super) fn check_boot_agreement_chirho(
    boot_chirho: &FrontendResultChirho,
    implementation_chirho: &FrontendResultChirho,
    boot_iface_chirho: &haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho,
    implementation_iface_chirho: &haskelujah_naming_chirho::iface_chirho::ModuleIfaceChirho,
) -> Result<(), String> {
    for name_chirho in boot_iface_chirho.exports_chirho.values_chirho.keys() {
        if !implementation_iface_chirho
            .exports_chirho
            .values_chirho
            .contains_key(name_chirho)
        {
            return Err(format!(
                "boot contract exports {name_chirho}, but implementation {} does not export it",
                implementation_iface_chirho.name_chirho
            ));
        }
    }
    for (name_chirho, expected_chirho) in &boot_iface_chirho.exports_chirho.types_chirho {
        let compatible_chirho = implementation_iface_chirho
            .exports_chirho
            .types_chirho
            .get(name_chirho)
            .is_some_and(|actual_chirho| {
                let constructors_chirho: std::collections::HashSet<_> =
                    actual_chirho.constructors_chirho.iter().collect();
                let methods_chirho: std::collections::HashSet<_> =
                    actual_chirho.methods_chirho.iter().collect();
                expected_chirho
                    .constructors_chirho
                    .iter()
                    .all(|name_chirho| constructors_chirho.contains(name_chirho))
                    && expected_chirho
                        .methods_chirho
                        .iter()
                        .all(|name_chirho| methods_chirho.contains(name_chirho))
            });
        if !compatible_chirho {
            return Err(format!(
                "boot contract exports type or members of {name_chirho} not exported by implementation {}",
                implementation_iface_chirho.name_chirho
            ));
        }
    }
    let declarations_chirho: HashMap<_, _> = implementation_chirho
        .module_chirho
        .decls_chirho
        .iter()
        .filter_map(|declaration_chirho| {
            declaration_name_chirho(declaration_chirho)
                .map(|name_chirho| (name_chirho, declaration_chirho))
        })
        .collect();
    let mismatch_chirho = |name_chirho: &str| {
        format!(
            "boot contract mismatch for {}.{name_chirho}",
            boot_chirho.module_chirho.name_chirho.full_name_chirho()
        )
    };
    for (name_chirho, expected_chirho) in &boot_chirho.type_contracts_chirho.kinds_chirho {
        if !declarations_chirho.contains_key(name_chirho.as_str())
            || !implementation_chirho
                .type_contracts_chirho
                .kinds_chirho
                .get(name_chirho)
                .is_some_and(|actual_chirho| expected_chirho.alpha_equivalent_chirho(actual_chirho))
        {
            return Err(mismatch_chirho(name_chirho));
        }
    }
    for declaration_chirho in &boot_chirho.module_chirho.decls_chirho {
        let valid_chirho = match declaration_chirho {
            DeclChirho::TypeSigChirho { name_chirho, .. } => {
                declarations_chirho
                    .get(name_chirho.text_chirho())
                    .is_some_and(|actual_chirho| {
                        matches!(actual_chirho, DeclChirho::FunBindChirho { .. })
                    })
                    && same_value_chirho(
                        name_chirho.text_chirho(),
                        boot_chirho,
                        implementation_chirho,
                    )
            }
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                ..
            } => match declarations_chirho.get(name_chirho.text_chirho()) {
                Some(DeclChirho::DataDeclChirho {
                    constructors_chirho: actual_chirho,
                    ..
                }) => same_constructors_chirho(
                    constructors_chirho,
                    actual_chirho,
                    boot_chirho,
                    implementation_chirho,
                ),
                _ => false,
            },
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                ..
            } => match declarations_chirho.get(name_chirho.text_chirho()) {
                Some(DeclChirho::NewtypeDeclChirho {
                    constructor_chirho: actual_chirho,
                    ..
                }) => same_constructors_chirho(
                    std::slice::from_ref(constructor_chirho),
                    std::slice::from_ref(actual_chirho),
                    boot_chirho,
                    implementation_chirho,
                ),
                _ => false,
            },
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                type_vars_chirho,
                result_chirho,
                closed_chirho: false,
                ..
            } => match declarations_chirho.get(name_chirho.text_chirho()) {
                Some(DeclChirho::TypeFamilyDeclChirho {
                    type_vars_chirho: actual_vars_chirho,
                    result_chirho: actual_result_chirho,
                    closed_chirho: false,
                    ..
                }) => {
                    let positions_chirho = |parameters_chirho: &[haskelujah_ast_chirho::decl_chirho::TyVarChirho], result_chirho: &haskelujah_ast_chirho::decl_chirho::TypeFamilyResultChirho| {
                        result_chirho.injectivity_chirho.as_ref().map(|dependency_chirho| dependency_chirho.parameters_chirho.iter().map(|name_chirho| parameters_chirho.iter().position(|parameter_chirho| parameter_chirho.name_chirho.text_chirho() == name_chirho.text_chirho())).collect::<Vec<_>>())
                    };
                    positions_chirho(type_vars_chirho, result_chirho)
                        == positions_chirho(actual_vars_chirho, actual_result_chirho)
                }
                _ => false,
            },
            DeclChirho::TypeAliasDeclChirho { name_chirho, .. } => match (
                boot_chirho
                    .type_contracts_chirho
                    .synonyms_chirho
                    .get(name_chirho.text_chirho()),
                implementation_chirho
                    .type_contracts_chirho
                    .synonyms_chirho
                    .get(name_chirho.text_chirho()),
            ) {
                (Some(expected_chirho), Some(actual_chirho)) => {
                    expected_chirho.alpha_equivalent_chirho(actual_chirho)
                }
                _ => false,
            },
            DeclChirho::FixityDeclChirho { .. } => {
                return Err("boot contract fixity agreement is not represented yet".to_owned());
            }
            _ => false,
        };
        if !valid_chirho {
            let name_chirho =
                if let DeclChirho::TypeSigChirho { name_chirho, .. } = declaration_chirho {
                    name_chirho.text_chirho()
                } else {
                    declaration_name_chirho(declaration_chirho).unwrap_or("declaration")
                };
            return Err(mismatch_chirho(name_chirho));
        }
    }
    Ok(())
}
