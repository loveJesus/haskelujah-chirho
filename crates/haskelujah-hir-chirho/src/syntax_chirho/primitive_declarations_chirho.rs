// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Backend bodies are admitted by a defining-unit manifest, never by source spelling.
//! Workflow: next-pipeline-chirho/ghc-implementation-map-chirho.

use super::*;

impl LowerChirho<'_> {
    fn own_manifest_name_chirho(&self, original_chirho: &OriginalNameChirho) -> ResultChirho<()> {
        if self.unit_chirho != UnitIdentityChirho::CompilerChirho
            || original_chirho.unit_chirho != self.unit_chirho
            || original_chirho.module_chirho != self.module_chirho
        {
            return Err(self.error_chirho(
                "backend declaration has no defining-unit manifest",
                SpanChirho::DUMMY_CHIRHO,
            ));
        }
        Ok(())
    }

    pub(super) fn bind_wired_chirho(
        &mut self,
        wired_chirho: WiredNameChirho,
    ) -> ResultChirho<NameIdChirho> {
        let original_chirho = wired_chirho.original_chirho();
        if let Some(id_chirho) = self.imported_names_chirho.get(&original_chirho).copied() {
            let info_chirho = &self.names_chirho[id_chirho.0 as usize];
            let bound_chirho = self.env_chirho.lookup_chirho(
                wired_chirho.occurrence_chirho(),
                wired_chirho.namespace_chirho(),
            );
            if info_chirho.original_chirho() != original_chirho
                || info_chirho.wired_chirho != Some(wired_chirho)
                || bound_chirho
                    .is_none_or(|bound_chirho| bound_chirho.def_id_chirho.0 != id_chirho.0)
            {
                return Err(self.error_chirho(
                    "wired seed disagrees with its canonical language binding",
                    SpanChirho::DUMMY_CHIRHO,
                ));
            }
            return Ok(id_chirho);
        }
        let id_chirho = self.bind_chirho(
            wired_chirho.occurrence_chirho(),
            wired_chirho.namespace_chirho(),
            SpanChirho::DUMMY_CHIRHO,
            false,
            Some(wired_chirho),
        )?;
        let actual_chirho = self.names_chirho[id_chirho.0 as usize].original_chirho();
        if actual_chirho != original_chirho {
            return Err(self.error_chirho(
                "wired seed has no exact original-name identity",
                SpanChirho::DUMMY_CHIRHO,
            ));
        }
        self.charge_names_chirho(
            actual_chirho.module_chirho.len()
                + actual_chirho.occurrence_chirho.len()
                + std::mem::size_of::<(OriginalNameChirho, NameIdChirho)>(),
            SpanChirho::DUMMY_CHIRHO,
        )?;
        self.imported_names_chirho.insert(actual_chirho, id_chirho);
        Ok(id_chirho)
    }

    pub(super) fn install_wired_declarations_chirho(
        &mut self,
        policy_chirho: &LoweringPolicyChirho,
    ) -> ResultChirho<Vec<NameIdChirho>> {
        let mut wired_exports_chirho = Vec::new();
        for (original_chirho, wired_chirho) in &policy_chirho.wired_declarations_chirho {
            self.own_manifest_name_chirho(original_chirho)?;
            if original_chirho.namespace_chirho != NamespaceChirho::TypeChirho
                || !matches!(
                    wired_chirho,
                    WiredNameChirho::IntegerChirho | WiredNameChirho::IOChirho
                )
                || original_chirho.module_chirho != wired_chirho.defining_module_chirho()
                || original_chirho.occurrence_chirho != wired_chirho.occurrence_chirho()
            {
                return Err(self.error_chirho(
                    "unsupported own wired declaration",
                    SpanChirho::DUMMY_CHIRHO,
                ));
            }
            wired_exports_chirho.push(self.bind_wired_chirho(*wired_chirho)?);
        }
        Ok(wired_exports_chirho)
    }

    pub(super) fn install_primitive_declarations_chirho(
        &mut self,
        policy_chirho: &LoweringPolicyChirho,
    ) -> ResultChirho<()> {
        for (original_chirho, contract_chirho) in &policy_chirho.primitive_bindings_chirho {
            self.own_manifest_name_chirho(original_chirho)?;
            if original_chirho.namespace_chirho != NamespaceChirho::ValueChirho
                || !(0..=3).contains(&contract_chirho.arity_chirho)
            {
                return Err(self.error_chirho(
                    "unsupported backend primitive declaration",
                    SpanChirho::DUMMY_CHIRHO,
                ));
            }
            self.bind_chirho(
                &original_chirho.occurrence_chirho,
                NamespaceChirho::ValueChirho,
                SpanChirho::DUMMY_CHIRHO,
                false,
                None,
            )?;
        }
        Ok(())
    }

    pub(super) fn primitive_bindings_chirho(
        &mut self,
        policy_chirho: &LoweringPolicyChirho,
        signatures_chirho: &mut HashMap<String, &TypeChirho>,
    ) -> ResultChirho<Vec<PrimitiveBindingChirho>> {
        let mut result_chirho = Vec::new();
        for (original_chirho, contract_chirho) in &policy_chirho.primitive_bindings_chirho {
            let signature_chirho = signatures_chirho
                .remove(&original_chirho.occurrence_chirho)
                .ok_or_else(|| {
                    self.error_chirho(
                        "backend primitive has no source signature",
                        SpanChirho::DUMMY_CHIRHO,
                    )
                })?;
            self.env_chirho.push_scope_chirho();
            let signature_chirho = self.signature_body_chirho(signature_chirho);
            self.env_chirho.pop_scope_chirho();
            let (signature_chirho, _) = signature_chirho?;
            let name_chirho = self
                .env_chirho
                .lookup_chirho(
                    &original_chirho.occurrence_chirho,
                    NamespaceChirho::ValueChirho,
                )
                .map(|info_chirho| NameIdChirho(info_chirho.def_id_chirho.0))
                .ok_or_else(|| {
                    self.error_chirho(
                        "backend primitive lost its own declaration",
                        SpanChirho::DUMMY_CHIRHO,
                    )
                })?;
            result_chirho.push(PrimitiveBindingChirho {
                name_chirho,
                signature_chirho,
                contract_chirho: contract_chirho.clone(),
            });
        }
        Ok(result_chirho)
    }
}
