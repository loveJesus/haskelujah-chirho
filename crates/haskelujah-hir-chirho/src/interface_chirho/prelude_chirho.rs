// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Join source facade presence with an independently checked terminal declaration.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_ast_chirho::module_chirho::{ExportMembersChirho, ImportItemChirho};

impl LowerChirho<'_> {
    pub(in crate::lower_chirho) fn install_prelude_chirho(
        &mut self,
        module_chirho: &AstModuleChirho,
        policy_chirho: &LoweringPolicyChirho,
        imports_chirho: &[ImportedInterfaceChirho],
    ) -> ResultChirho<()> {
        let mut proven_chirho = BTreeMap::new();
        if let Some(routes_chirho) = &policy_chirho.prelude_routes_chirho {
            for (key_chirho, alternatives_chirho) in routes_chirho.iter() {
                self.charge_names_chirho(key_chirho.0.len() + 1, module_chirho.span_chirho)?;
                for route_chirho in alternatives_chirho {
                    self.charge_names_chirho(1, module_chirho.span_chirho)?;
                    let original_chirho = &route_chirho.original_chirho;
                    if original_chirho.occurrence_chirho != key_chirho.0
                        || original_chirho.namespace_chirho != key_chirho.1
                        || original_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
                        || !policy_chirho
                            .prelude_exports_chirho
                            .as_ref()
                            .is_some_and(|exports_chirho| exports_chirho.contains(key_chirho))
                    {
                        return Err(self.error_chirho(
                            "Prelude route disagrees with its source name account",
                            module_chirho.span_chirho,
                        ));
                    }
                    let Some(import_chirho) = imports_chirho.iter().find(|import_chirho| {
                        let interface_chirho = &import_chirho.interface_chirho;
                        interface_chirho.unit_chirho == original_chirho.unit_chirho
                            && interface_chirho.module_chirho == original_chirho.module_chirho
                    }) else {
                        continue;
                    };
                    let interface_chirho = &import_chirho.interface_chirho;
                    if !interface_chirho.exports_chirho.contains(original_chirho) {
                        continue;
                    }
                    if let Some(parent_chirho) = &route_chirho.parent_chirho
                        && !interface_chirho
                            .classes_chirho
                            .get(parent_chirho)
                            .is_some_and(|class_chirho| {
                                class_chirho.methods_chirho.iter().any(|method_chirho| {
                                    method_chirho.name_chirho == *original_chirho
                                })
                            })
                    {
                        continue;
                    }
                    let id_chirho =
                        *import_chirho
                            .names_chirho
                            .get(original_chirho)
                            .ok_or_else(|| {
                                self.error_chirho(
                                    "Prelude terminal export has no imported declaration",
                                    module_chirho.span_chirho,
                                )
                            })?;
                    if proven_chirho
                        .insert(key_chirho.clone(), id_chirho)
                        .is_some_and(|old_chirho| old_chirho != id_chirho)
                    {
                        return Err(self.error_chirho(
                            "conflicting checked Prelude facade identities",
                            module_chirho.span_chirho,
                        ));
                    }
                }
            }
        }
        if self.implicit_prelude_chirho {
            self.prelude_scope_chirho = proven_chirho.clone();
        }
        for import_chirho in &module_chirho.imports_chirho {
            if import_chirho.module_chirho.full_name_chirho() != "Prelude" {
                continue;
            }
            if import_chirho.qualified_chirho || import_chirho.alias_chirho.is_some() {
                return Err(self.error_chirho(
                    "qualified Prelude imports require a supported scope model",
                    import_chirho.span_chirho,
                ));
            }
            let Some(spec_chirho) = &import_chirho.spec_chirho else {
                return Err(self.error_chirho(
                    "open Prelude imports need every export's checked defining interface",
                    import_chirho.span_chirho,
                ));
            };
            if spec_chirho.hiding_chirho {
                return Err(self.error_chirho(
                    "Prelude hiding lists are not yet supported",
                    import_chirho.span_chirho,
                ));
            }
            let mut selected_chirho = Vec::new();
            for item_chirho in &spec_chirho.items_chirho {
                let (name_chirho, namespace_chirho, members_chirho) = match item_chirho {
                    ImportItemChirho::VarChirho(name_chirho) => {
                        (name_chirho, NamespaceChirho::ValueChirho, None)
                    }
                    ImportItemChirho::TyConChirho {
                        name_chirho,
                        members_chirho,
                    } => (
                        name_chirho,
                        NamespaceChirho::TypeChirho,
                        Some(members_chirho),
                    ),
                    _ => {
                        return Err(self.error_chirho(
                            "unrepresented Prelude import item",
                            import_chirho.span_chirho,
                        ));
                    }
                };
                let key_chirho = (name_chirho.text_chirho().to_string(), namespace_chirho);
                if !policy_chirho
                    .prelude_exports_chirho
                    .as_ref()
                    .is_some_and(|exports_chirho| exports_chirho.contains(&key_chirho))
                {
                    return Err(self.invalid_class_chirho(
                        "explicit import item is not exported by Prelude",
                        import_chirho.span_chirho,
                    ));
                }
                let id_chirho = *proven_chirho.get(&key_chirho).ok_or_else(|| {
                    self.error_chirho(
                        "Prelude import has no checked defining declaration",
                        import_chirho.span_chirho,
                    )
                })?;
                selected_chirho.push((key_chirho, id_chirho));
                if let Some(members_chirho) = members_chirho {
                    let class_chirho = self
                        .imported_classes_chirho
                        .iter()
                        .find(|class_chirho| class_chirho.name_chirho == id_chirho);
                    let names_chirho = match members_chirho {
                        ExportMembersChirho::NoneChirho => Vec::new(),
                        ExportMembersChirho::AllChirho => class_chirho
                            .ok_or_else(|| {
                                self.error_chirho(
                                    "Prelude constructor members are not checked",
                                    import_chirho.span_chirho,
                                )
                            })?
                            .methods_chirho
                            .iter()
                            .map(|method_chirho| {
                                self.names_chirho[method_chirho.name_chirho.0 as usize]
                                    .occurrence_chirho
                                    .clone()
                            })
                            .collect(),
                        ExportMembersChirho::SomeChirho(names_chirho) => names_chirho
                            .iter()
                            .map(|name_chirho| name_chirho.text_chirho().to_string())
                            .collect(),
                    };
                    for name_chirho in names_chirho {
                        let key_chirho = (name_chirho, NamespaceChirho::ValueChirho);
                        let member_chirho = *proven_chirho.get(&key_chirho).ok_or_else(|| {
                            self.error_chirho(
                                "Prelude class member has no checked defining declaration",
                                import_chirho.span_chirho,
                            )
                        })?;
                        if !class_chirho.is_some_and(|class_chirho| {
                            class_chirho
                                .methods_chirho
                                .iter()
                                .any(|method_chirho| method_chirho.name_chirho == member_chirho)
                        }) {
                            return Err(self.invalid_class_chirho(
                                "Prelude import member does not belong to its class",
                                import_chirho.span_chirho,
                            ));
                        }
                        selected_chirho.push((key_chirho, member_chirho));
                    }
                }
            }
            for ((name_chirho, namespace_chirho), id_chirho) in selected_chirho {
                for text_chirho in [name_chirho.clone(), format!("Prelude.{name_chirho}")] {
                    self.charge_names_chirho(text_chirho.len() + 1, import_chirho.span_chirho)?;
                    let ids_chirho = self
                        .import_scope_chirho
                        .entry((text_chirho, namespace_chirho))
                        .or_default();
                    if !ids_chirho.contains(&id_chirho) {
                        ids_chirho.push(id_chirho);
                    }
                }
            }
        }
        Ok(())
    }
}
