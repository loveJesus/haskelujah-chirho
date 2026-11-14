// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Check actual source signatures against the opaque backend's closed Integer contract.
//! Workflow: next-pipeline-chirho/ghc-implementation-map-chirho.

use super::*;
use haskelujah_hir_chirho::PrimitiveBindingChirho;

impl CheckedModuleChirho {
    pub(super) fn primitive_signatures_chirho(
        &mut self,
        declarations_chirho: &[PrimitiveBindingChirho],
    ) -> Result<(), TcErrorChirho> {
        let integer_chirho = self
            .solver_chirho
            .arena_chirho
            .con_chirho(TyConChirho::IntegerChirho)?;
        for declaration_chirho in declarations_chirho {
            if !(1..=2).contains(&declaration_chirho.arity_chirho)
                || !declaration_chirho
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
            let mut body_chirho = self.names_chirho[&declaration_chirho.name_chirho];
            for _argument_chirho in 0..declaration_chirho.arity_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
                match self.solver_chirho.arena_chirho.node_chirho(body_chirho)? {
                    TyNodeChirho::FunChirho(argument_chirho, result_chirho)
                        if *argument_chirho == integer_chirho =>
                    {
                        body_chirho = *result_chirho
                    }
                    _ => return Err(primitive_error_chirho()),
                }
            }
            if body_chirho != integer_chirho {
                return Err(primitive_error_chirho());
            }
        }
        Ok(())
    }
}

fn primitive_error_chirho() -> TcErrorChirho {
    TcErrorChirho::new_chirho(
        TcErrorKindChirho::UnsupportedChirho,
        "source signature disagrees with its declared opaque Integer backend body",
    )
}
