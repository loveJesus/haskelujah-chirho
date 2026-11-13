// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A fragment cannot authorize open-import scope. Select explicit, checked exports only.
use super::*;
use haskelujah_ast_chirho::module_chirho::{
    ExportMembersChirho, ImportDeclChirho, ImportItemChirho,
};

impl LowerChirho<'_> {
    fn push_scope_name_chirho(
        &mut self,
        selected_chirho: &mut Vec<OriginalNameChirho>,
        original_chirho: &OriginalNameChirho,
        span_chirho: SpanChirho,
    ) -> ResultChirho<()> {
        self.charge_names_chirho(
            original_chirho.module_chirho.len() + original_chirho.occurrence_chirho.len() + 1,
            span_chirho,
        )?;
        selected_chirho.push(original_chirho.clone());
        Ok(())
    }

    pub(in crate::lower_chirho) fn selected_exports_chirho(
        &mut self,
        interface_chirho: &InterfaceChirho,
        import_chirho: &ImportDeclChirho,
    ) -> ResultChirho<Vec<OriginalNameChirho>> {
        let Some(spec_chirho) = &import_chirho.spec_chirho else {
            if interface_chirho.completeness_chirho == CompletenessChirho::SliceChirho {
                return Err(self.error_chirho("open import needs the sliced module's complete export set; use an explicit checked import list", import_chirho.span_chirho));
            }
            let mut selected_chirho = Vec::new();
            for original_chirho in &interface_chirho.exports_chirho {
                self.push_scope_name_chirho(
                    &mut selected_chirho,
                    original_chirho,
                    import_chirho.span_chirho,
                )?;
            }
            return Ok(selected_chirho);
        };
        if spec_chirho.hiding_chirho {
            return Err(self.error_chirho(
                "next does not yet check hiding import lists",
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
                ImportItemChirho::UnrepresentedChirho(_) => {
                    return Err(self.error_chirho(
                        "unrepresented explicit import item",
                        import_chirho.span_chirho,
                    ));
                }
            };
            let original_chirho = interface_chirho
                .exports_chirho
                .iter()
                .find(|original_chirho| {
                    original_chirho.namespace_chirho == namespace_chirho
                        && original_chirho.occurrence_chirho == name_chirho.text_chirho()
                })
                .ok_or_else(|| {
                    if interface_chirho.completeness_chirho == CompletenessChirho::SliceChirho {
                        self.error_chirho(
                            "explicit import item is absent from the checked slice",
                            import_chirho.span_chirho,
                        )
                    } else {
                        self.invalid_class_chirho(
                            "explicit import item is not exported",
                            import_chirho.span_chirho,
                        )
                    }
                })?;
            self.push_scope_name_chirho(
                &mut selected_chirho,
                original_chirho,
                import_chirho.span_chirho,
            )?;
            if let Some(members_chirho) = members_chirho {
                let requested_chirho = match members_chirho {
                    ExportMembersChirho::NoneChirho => continue,
                    ExportMembersChirho::AllChirho => None,
                    ExportMembersChirho::SomeChirho(names_chirho) => Some(names_chirho),
                };
                let class_chirho = interface_chirho
                    .classes_chirho
                    .get(original_chirho)
                    .ok_or_else(|| {
                        self.error_chirho(
                            "next explicit member imports require a checked class owner",
                            import_chirho.span_chirho,
                        )
                    })?;
                if let Some(names_chirho) = requested_chirho {
                    for name_chirho in names_chirho {
                        let method_chirho = class_chirho
                            .methods_chirho
                            .iter()
                            .find(|method_chirho| {
                                method_chirho.name_chirho.occurrence_chirho
                                    == name_chirho.text_chirho()
                            })
                            .ok_or_else(|| {
                                self.invalid_class_chirho(
                                    "imported member does not belong to its class",
                                    import_chirho.span_chirho,
                                )
                            })?;
                        if !interface_chirho
                            .exports_chirho
                            .contains(&method_chirho.name_chirho)
                        {
                            return Err(self.error_chirho(
                                "class member is not in the checked export set",
                                import_chirho.span_chirho,
                            ));
                        }
                        self.push_scope_name_chirho(
                            &mut selected_chirho,
                            &method_chirho.name_chirho,
                            import_chirho.span_chirho,
                        )?;
                    }
                } else {
                    for method_chirho in &class_chirho.methods_chirho {
                        if !interface_chirho
                            .exports_chirho
                            .contains(&method_chirho.name_chirho)
                        {
                            return Err(self.error_chirho(
                                "class member is not in the checked export set",
                                import_chirho.span_chirho,
                            ));
                        }
                        self.push_scope_name_chirho(
                            &mut selected_chirho,
                            &method_chirho.name_chirho,
                            import_chirho.span_chirho,
                        )?;
                    }
                }
            }
        }
        selected_chirho.sort();
        selected_chirho.dedup();
        Ok(selected_chirho)
    }
}
