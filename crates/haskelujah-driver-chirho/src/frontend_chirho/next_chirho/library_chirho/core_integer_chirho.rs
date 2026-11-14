// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A manifest-declared backend body is a normal original-name definition at link time.
//! Workflow: next-pipeline-chirho/ghc-implementation-map-chirho.

use super::*;
use haskelujah_hir_chirho::PrimitiveBindingChirho;
use haskelujah_typing_chirho::ty_chirho::TyChirho as CoreTypeChirho;

impl ElaboratorChirho<'_> {
    pub(super) fn primitive_binding_chirho(
        &mut self,
        declaration_chirho: &PrimitiveBindingChirho,
    ) -> Result<CoreBindingChirho, NextErrorChirho> {
        let info_chirho = self
            .types_chirho
            .checked_chirho
            .module_chirho
            .names_chirho
            .get(declaration_chirho.name_chirho.0 as usize)
            .ok_or_else(|| error_chirho("primitive body lost its defining name"))?;
        let operation_chirho =
            super::super::library_chirho::integer_operation_chirho(&info_chirho.original_chirho())
                .ok_or_else(|| error_chirho("no declared primitive body for this original name"))?;
        if operation_chirho.arity_chirho() != declaration_chirho.arity_chirho {
            return Err(error_chirho(
                "primitive arity differs from its checked source signature",
            ));
        }
        let binder_chirho = self.binder_chirho(declaration_chirho.name_chirho)?;
        let mut ty_chirho = self.types_chirho.copy_chirho(&binder_chirho.ty_chirho)?;
        let mut parameters_chirho = Vec::new();
        for _position_chirho in 0..operation_chirho.arity_chirho() {
            let CoreTypeChirho::FunChirho(argument_chirho, result_chirho, _) = ty_chirho else {
                return Err(error_chirho("primitive body has no checked arrow type"));
            };
            parameters_chirho
                .push(self.fresh_binder_chirho(*argument_chirho, info_chirho.span_chirho)?);
            ty_chirho = *result_chirho;
        }
        let mut evaluated_chirho = Vec::new();
        for parameter_chirho in &parameters_chirho {
            let ty_chirho = self.types_chirho.copy_chirho(&parameter_chirho.ty_chirho)?;
            evaluated_chirho.push(self.fresh_binder_chirho(ty_chirho, info_chirho.span_chirho)?);
        }
        self.types_chirho.reserve_term_chirho()?;
        let mut rhs_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: operation_chirho.primitive_chirho().into(),
            args_chirho: evaluated_chirho
                .iter()
                .map(|parameter_chirho| CoreExprChirho::VarChirho(parameter_chirho.id_chirho))
                .collect(),
        };
        // Demand on the STG continuation stack, left-to-right and only after saturation.
        // A primitive must never re-enter the host evaluator with outer roots in Rust locals.
        for (parameter_chirho, evaluated_chirho) in
            parameters_chirho.iter().zip(evaluated_chirho).rev()
        {
            self.types_chirho.reserve_term_chirho()?;
            rhs_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(parameter_chirho.id_chirho)),
                bind_chirho: evaluated_chirho,
                result_ty_chirho: self.types_chirho.copy_chirho(&ty_chirho)?,
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho,
                }],
            };
        }
        for parameter_chirho in parameters_chirho.into_iter().rev() {
            self.types_chirho.reserve_term_chirho()?;
            rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(rhs_chirho),
            };
        }
        Ok(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        })
    }
}

fn error_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}
