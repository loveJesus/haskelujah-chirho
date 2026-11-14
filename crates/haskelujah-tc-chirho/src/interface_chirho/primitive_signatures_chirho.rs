// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source/class checked types constrain every backend body. No source spelling grants one.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_hir_chirho::{
    ExprKindChirho, InstanceDeclChirho, InstanceMethodChirho, PrimitiveBindingChirho,
    UnitIdentityChirho, WiredNameChirho,
};

impl CheckedModuleChirho {
    pub(super) fn primitive_signatures_chirho(
        &mut self,
        declarations_chirho: &[PrimitiveBindingChirho],
    ) -> Result<(), TcErrorChirho> {
        for declaration_chirho in declarations_chirho {
            if !declaration_chirho
                .signature_chirho
                .binders_chirho
                .is_empty()
                || !declaration_chirho
                    .signature_chirho
                    .context_chirho
                    .is_empty()
            {
                return Err(primitive_error_chirho());
            }
            self.signature_chirho(
                declaration_chirho.name_chirho,
                &declaration_chirho.signature_chirho,
            )?;
            let expected_chirho = crate::interface_chirho::closed_contract_chirho(
                self,
                &declaration_chirho.contract_chirho,
            )?;
            if self.names_chirho[&declaration_chirho.name_chirho] != expected_chirho {
                return Err(primitive_error_chirho());
            }
        }
        Ok(())
    }

    pub(super) fn declared_method_type_chirho(
        &mut self,
        instance_chirho: &InstanceDeclChirho,
        method_chirho: &InstanceMethodChirho,
        head_chirho: TyChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        let ExprKindChirho::DeclaredBodyChirho(owner_chirho) =
            method_chirho.binding_chirho.body_chirho.kind_chirho
        else {
            return Err(primitive_error_chirho());
        };
        if owner_chirho != method_chirho.binding_chirho.name_chirho
            || self.module_chirho.unit_chirho != UnitIdentityChirho::CompilerChirho
            || !instance_chirho.binders_chirho.is_empty()
            || !instance_chirho.context_chirho.is_empty()
        {
            return Err(primitive_error_chirho());
        }
        let body_chirho = self
            .module_chirho
            .declared_bodies_chirho
            .get(&owner_chirho)
            .cloned()
            .ok_or_else(primitive_error_chirho)?;
        let key_chirho = &body_chirho.key_chirho;
        if key_chirho.unit_chirho != self.module_chirho.unit_chirho
            || key_chirho.module_chirho != self.module_chirho.module_chirho
            || key_chirho.class_chirho
                != self.module_chirho.names_chirho[instance_chirho.class_chirho.0 as usize]
                    .original_chirho()
            || key_chirho.method_chirho
                != self.module_chirho.names_chirho[method_chirho.method_chirho.0 as usize]
                    .original_chirho()
        {
            return Err(primitive_error_chirho());
        }
        let head_con_chirho = match key_chirho.head_chirho {
            WiredNameChirho::IntChirho => TyConChirho::IntChirho,
            WiredNameChirho::IntegerChirho => TyConChirho::IntegerChirho,
            _ => return Err(primitive_error_chirho()),
        };
        if head_chirho
            != self
                .solver_chirho
                .arena_chirho
                .con_chirho(head_con_chirho)?
        {
            return Err(primitive_error_chirho());
        }
        let ty_chirho =
            crate::interface_chirho::closed_contract_chirho(self, &body_chirho.contract_chirho)?;
        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
        if !self.declared_body_uses_chirho.insert(owner_chirho) {
            return Err(primitive_error_chirho());
        }
        self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
        self.expressions_chirho.insert(
            method_chirho.binding_chirho.body_chirho.id_chirho,
            ty_chirho,
        );
        Ok(ty_chirho)
    }
}

fn primitive_error_chirho() -> TcErrorChirho {
    TcErrorChirho::new_chirho(
        TcErrorKindChirho::UnsupportedChirho,
        "source or class method type disagrees with its declared backend body",
    )
}
