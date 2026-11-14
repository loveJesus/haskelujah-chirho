// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Presence facts may reject; a sliced defining unit cannot certify instance absence.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use haskelujah_hir_chirho::UnitIdentityChirho;

impl CheckedModuleChirho {
    fn sliced_name_chirho(&self, name_chirho: NameIdChirho) -> bool {
        self.module_chirho
            .names_chirho
            .get(name_chirho.0 as usize)
            .is_some_and(|info_chirho| {
                self.module_chirho
                    .sliced_units_chirho
                    .contains(&info_chirho.unit_chirho)
            })
    }
    fn sliced_head_chirho(&mut self, mut ty_chirho: TyChirho) -> Result<bool, TcErrorChirho> {
        loop {
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            ty_chirho = self.solver_chirho.arena_chirho.resolve_chirho(ty_chirho)?;
            match self.solver_chirho.arena_chirho.node_chirho(ty_chirho)? {
                TyNodeChirho::AppChirho(function_chirho, _) => ty_chirho = *function_chirho,
                TyNodeChirho::ConChirho(
                    TyConChirho::DeclaredChirho(name_chirho)
                    | TyConChirho::ClassChirho(name_chirho),
                ) => return Ok(self.sliced_name_chirho(*name_chirho)),
                TyNodeChirho::ConChirho(_)
                | TyNodeChirho::FunChirho(..)
                | TyNodeChirho::TupleChirho(_) => {
                    return Ok(self
                        .module_chirho
                        .sliced_units_chirho
                        .contains(&UnitIdentityChirho::CompilerChirho));
                }
                _ => return Ok(false),
            }
        }
    }
    pub(super) fn instance_absence_is_partial_chirho(
        &mut self,
        class_chirho: NameIdChirho,
        head_chirho: TyChirho,
    ) -> Result<bool, TcErrorChirho> {
        // A library cannot declare instances of a complete home-unit class it cannot name.
        // Keep a deliberately sliced home unit conservative as well.
        if !self.sliced_name_chirho(class_chirho)
            && self
                .module_chirho
                .names_chirho
                .get(class_chirho.0 as usize)
                .is_some_and(|info_chirho| {
                    info_chirho.unit_chirho == UnitIdentityChirho::StandaloneHomeChirho
                })
        {
            return Ok(false);
        }
        Ok(self.sliced_name_chirho(class_chirho) || self.sliced_head_chirho(head_chirho)?)
    }
    pub(super) fn check_partial_instance_chirho(
        &mut self,
        class_chirho: NameIdChirho,
        head_chirho: TyChirho,
    ) -> Result<(), TcErrorChirho> {
        // Compiler-unit source is admitted only by the driver's pinned manifests.
        // Its actual instances can belong to a class defined in another library module.
        // Visible duplicate registration still precedes this exemption.
        if self.module_chirho.unit_chirho == UnitIdentityChirho::CompilerChirho {
            return Ok(());
        }
        if self.sliced_name_chirho(class_chirho) && self.sliced_head_chirho(head_chirho)? {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::UnsupportedChirho,
                "local instance could collide with unseen instances in a sliced defining unit",
            ));
        }
        Ok(())
    }
}
