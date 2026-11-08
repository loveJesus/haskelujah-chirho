// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Selected constructor layout and actual selected label owners share one map.
//! Workflow: compiler-pipeline-chirho/value-scope-resolution-chirho.

use haskelujah_ast_chirho::module_chirho::{ImportDeclChirho, ModuleChirho};
use haskelujah_ast_chirho::name_chirho::TypeConstructorDeclarationChirho;
use haskelujah_naming_chirho::{ModuleIfaceChirho, env_chirho::NamespaceChirho};
use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::infer_chirho::ConstructorFieldScopeChirho;
use std::collections::HashMap;

/// A cold, module-local view of the existing selected import/name producer.
/// This cache dies before inference; it is not another constructor registry.
struct SelectedImportChirho<'input_chirho> {
    import_chirho: &'input_chirho ImportDeclChirho,
    iface_chirho: &'input_chirho ModuleIfaceChirho,
    names_chirho: Vec<(String, NamespaceChirho, SpanChirho)>,
}

fn label_is_selected_chirho(
    owner_chirho: &TypeConstructorDeclarationChirho,
    label_chirho: &str,
    imports_chirho: &[SelectedImportChirho<'_>],
) -> bool {
    imports_chirho.iter().any(|selected_chirho| {
        selected_chirho
            .names_chirho
            .iter()
            .any(|(name_chirho, namespace_chirho, _)| {
                name_chirho == label_chirho && *namespace_chirho == NamespaceChirho::ValueChirho
            })
            && selected_chirho
                .iface_chirho
                .exports_chirho
                .values_chirho
                .get(label_chirho)
                .and_then(|value_chirho| value_chirho.constructor_fields_chirho.as_ref())
                .and_then(|account_chirho| account_chirho.label_owners_chirho())
                .is_some_and(|owners_chirho| owners_chirho.contains(owner_chirho))
    })
}

pub(super) fn selected_import_field_scopes_chirho(
    module_chirho: &ModuleChirho,
    ifaces_chirho: &[ModuleIfaceChirho],
) -> HashMap<String, ConstructorFieldScopeChirho> {
    let imports_chirho: Vec<_> = module_chirho
        .imports_chirho
        .iter()
        .filter_map(|import_chirho| {
            let iface_chirho = haskelujah_naming_chirho::iface_chirho::select_import_iface_chirho(
                ifaces_chirho,
                import_chirho,
            )?;
            let names_chirho =
                haskelujah_naming_chirho::resolve_chirho::compute_imported_names_chirho(
                    &iface_chirho.exports_chirho,
                    &import_chirho.spec_chirho,
                );
            Some(SelectedImportChirho {
                import_chirho,
                iface_chirho,
                names_chirho,
            })
        })
        .collect();
    let mut scopes_chirho = HashMap::new();
    for selected_chirho in &imports_chirho {
        let qualifier_chirho = selected_chirho.import_chirho.effective_qualifier_chirho();
        for (name_chirho, namespace_chirho, _) in &selected_chirho.names_chirho {
            if *namespace_chirho != NamespaceChirho::ValueChirho {
                continue;
            }
            let account_chirho = selected_chirho
                .iface_chirho
                .exports_chirho
                .values_chirho
                .get(name_chirho)
                .and_then(|value_chirho| value_chirho.constructor_fields_chirho.as_ref());
            let scope_chirho = match account_chirho {
                Some(account_chirho) if account_chirho.is_blocked_chirho() => {
                    Some(ConstructorFieldScopeChirho::blocked_chirho())
                }
                Some(account_chirho) => {
                    account_chirho
                        .known_chirho()
                        .map(|(owner_chirho, physical_chirho)| {
                            // GHC references 32870/32887: selected labels may be supplied
                            // separately or qualified-only; hidden labels do not bind.
                            let eligible_chirho = physical_chirho
                                .iter()
                                .filter(|label_chirho| {
                                    label_is_selected_chirho(
                                        owner_chirho,
                                        label_chirho,
                                        &imports_chirho,
                                    )
                                })
                                .cloned()
                                .collect();
                            ConstructorFieldScopeChirho::selected_chirho(
                                owner_chirho.clone(),
                                physical_chirho.to_vec(),
                                eligible_chirho,
                            )
                        })
                }
                // Ownerless builtin/raw declarations retain only the previous
                // explicitly unproven typing compatibility path.
                None => selected_chirho
                    .iface_chirho
                    .exports_chirho
                    .types_chirho
                    .values()
                    .find(|type_chirho| {
                        type_chirho.declaration_chirho.is_none()
                            && type_chirho.constructors_chirho.contains(name_chirho)
                            && !type_chirho.methods_chirho.is_empty()
                    })
                    .map(|type_chirho| {
                        ConstructorFieldScopeChirho::legacy_unproven_chirho(
                            type_chirho.methods_chirho.clone(),
                        )
                    }),
            };
            let full_name_chirho = format!("{qualifier_chirho}.{name_chirho}");
            scopes_chirho.remove(&full_name_chirho);
            if !selected_chirho.import_chirho.qualified_chirho {
                scopes_chirho.remove(name_chirho);
            }
            if let Some(scope_chirho) = scope_chirho {
                scopes_chirho.insert(full_name_chirho, scope_chirho.clone());
                if !selected_chirho.import_chirho.qualified_chirho {
                    scopes_chirho.insert(name_chirho.clone(), scope_chirho);
                }
            }
        }
    }
    scopes_chirho
}
