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

    pub(super) fn install_primitive_declarations_chirho(
        &mut self,
        policy_chirho: &LoweringPolicyChirho,
    ) -> ResultChirho<Vec<NameIdChirho>> {
        let mut wired_exports_chirho = Vec::new();
        for (original_chirho, wired_chirho) in &policy_chirho.wired_declarations_chirho {
            self.own_manifest_name_chirho(original_chirho)?;
            if original_chirho.namespace_chirho != NamespaceChirho::TypeChirho
                || *wired_chirho != WiredNameChirho::IntegerChirho
                || original_chirho.module_chirho != wired_chirho.defining_module_chirho()
                || original_chirho.occurrence_chirho != "Integer"
            {
                return Err(self.error_chirho(
                    "unsupported own wired declaration",
                    SpanChirho::DUMMY_CHIRHO,
                ));
            }
            wired_exports_chirho.push(self.bind_chirho(
                &original_chirho.occurrence_chirho,
                NamespaceChirho::TypeChirho,
                SpanChirho::DUMMY_CHIRHO,
                false,
                Some(*wired_chirho),
            )?);
        }
        for (original_chirho, arity_chirho) in &policy_chirho.primitive_bindings_chirho {
            self.own_manifest_name_chirho(original_chirho)?;
            if original_chirho.namespace_chirho != NamespaceChirho::ValueChirho
                || !(1..=2).contains(arity_chirho)
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
        Ok(wired_exports_chirho)
    }

    pub(super) fn primitive_bindings_chirho(
        &mut self,
        policy_chirho: &LoweringPolicyChirho,
        signatures_chirho: &mut HashMap<String, &TypeChirho>,
    ) -> ResultChirho<Vec<PrimitiveBindingChirho>> {
        let mut result_chirho = Vec::new();
        for (original_chirho, arity_chirho) in &policy_chirho.primitive_bindings_chirho {
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
                arity_chirho: *arity_chirho,
            });
        }
        Ok(result_chirho)
    }
}
