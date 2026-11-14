// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A driver-owned method substitution consumes exactly one explicit original group.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;

impl LowerChirho<'_> {
    pub(super) fn validate_body_policy_chirho(&mut self) -> ResultChirho<()> {
        for key_chirho in self.instance_body_policy_chirho.keys() {
            self.room_chirho(0, SpanChirho::DUMMY_CHIRHO)?;
            self.next_expr_chirho += 1;
            if self.unit_chirho != UnitIdentityChirho::CompilerChirho
                || key_chirho.unit_chirho != self.unit_chirho
                || key_chirho.module_chirho != self.module_chirho
                || key_chirho.class_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
                || key_chirho.method_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
                || key_chirho.class_chirho.namespace_chirho != NamespaceChirho::TypeChirho
                || key_chirho.method_chirho.namespace_chirho != NamespaceChirho::ValueChirho
                || key_chirho.method_chirho.module_chirho != key_chirho.class_chirho.module_chirho
            {
                return Err(self.error_chirho(
                    "instance body has no defining-unit manifest",
                    SpanChirho::DUMMY_CHIRHO,
                ));
            }
        }
        Ok(())
    }

    pub(super) fn declared_method_body_chirho(
        &mut self,
        class_chirho: NameIdChirho,
        head_chirho: &SurfaceTypeChirho,
        method_chirho: NameIdChirho,
        binding_chirho: NameIdChirho,
        span_chirho: SpanChirho,
    ) -> ResultChirho<Option<ExprChirho>> {
        let head_chirho = match head_chirho {
            SurfaceTypeChirho::IntChirho => WiredNameChirho::IntChirho,
            SurfaceTypeChirho::IntegerChirho => WiredNameChirho::IntegerChirho,
            _ => return Ok(None),
        };
        let key_chirho = DeclaredBodyKeyChirho {
            unit_chirho: self.unit_chirho.clone(),
            module_chirho: self.module_chirho.clone(),
            class_chirho: self.names_chirho[class_chirho.0 as usize].original_chirho(),
            head_chirho,
            method_chirho: self.names_chirho[method_chirho.0 as usize].original_chirho(),
        };
        let Some(contract_chirho) = self.instance_body_policy_chirho.get(&key_chirho) else {
            return Ok(None);
        };
        if !self.consumed_bodies_chirho.insert(key_chirho.clone()) {
            return Err(self.error_chirho("manifest instance body was consumed twice", span_chirho));
        }
        let contract_chirho = contract_chirho.clone();
        self.room_chirho(0, span_chirho)?;
        self.next_expr_chirho += 1;
        self.declared_bodies_chirho.insert(
            binding_chirho,
            DeclaredBodyChirho {
                key_chirho,
                contract_chirho,
            },
        );
        self.node_chirho(
            ExprKindChirho::DeclaredBodyChirho(binding_chirho),
            span_chirho,
        )
        .map(Some)
    }
}
