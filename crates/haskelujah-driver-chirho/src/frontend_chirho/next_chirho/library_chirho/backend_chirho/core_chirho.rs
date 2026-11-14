// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source-owned backend bodies reuse normal closed types and strict Core lambdas.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho
use super::*;
use crate::next_chirho::library_chirho::backend_chirho::{self, ProviderChirho};
use haskelujah_hir_chirho::{
    InstanceDeclChirho, InstanceMethodChirho, PrimitiveBindingChirho, SurfaceTypeChirho,
};
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
            .ok_or_else(|| error_chirho("primitive lost its defining name"))?;
        let provider_chirho = backend_chirho::operation_chirho(&info_chirho.original_chirho())
            .ok_or_else(|| error_chirho("no backend body for original name"))?;
        if provider_chirho.contract_chirho() != declaration_chirho.contract_chirho {
            return Err(error_chirho("primitive contract disagrees with backend"));
        }
        let binder_chirho = self.binder_chirho(declaration_chirho.name_chirho)?;
        let ty_chirho = self.types_chirho.copy_chirho(&binder_chirho.ty_chirho)?;
        let rhs_chirho =
            self.primitive_expression_chirho(ty_chirho, provider_chirho, info_chirho.span_chirho)?;
        Ok(CoreBindingChirho {
            binder_chirho,
            rhs_chirho,
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        })
    }

    pub(super) fn declared_method_binding_chirho(
        &mut self,
        instance_chirho: &InstanceDeclChirho,
        method_chirho: &InstanceMethodChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        let checked_chirho = self.types_chirho.checked_chirho;
        let ExprKindChirho::DeclaredBodyChirho(owner_chirho) =
            method_chirho.binding_chirho.body_chirho.kind_chirho
        else {
            return Err(error_chirho("missing declared method root"));
        };
        let declared_chirho = checked_chirho
            .module_chirho
            .declared_bodies_chirho
            .get(&owner_chirho)
            .ok_or_else(|| error_chirho("method root has no manifest body"))?;
        let key_chirho = &declared_chirho.key_chirho;
        let actual_head_chirho = match instance_chirho.head_chirho {
            SurfaceTypeChirho::IntChirho => WiredNameChirho::IntChirho,
            SurfaceTypeChirho::IntegerChirho => WiredNameChirho::IntegerChirho,
            _ => return Err(error_chirho("declared method has no wired instance head")),
        };
        if owner_chirho != method_chirho.binding_chirho.name_chirho
            || checked_chirho.module_chirho.unit_chirho
                != haskelujah_hir_chirho::UnitIdentityChirho::CompilerChirho
            || key_chirho.unit_chirho != checked_chirho.module_chirho.unit_chirho
            || key_chirho.module_chirho != checked_chirho.module_chirho.module_chirho
            || key_chirho.head_chirho != actual_head_chirho
            || key_chirho.class_chirho
                != checked_chirho.module_chirho.names_chirho
                    [instance_chirho.class_chirho.0 as usize]
                    .original_chirho()
            || key_chirho.method_chirho
                != checked_chirho.module_chirho.names_chirho[method_chirho.method_chirho.0 as usize]
                    .original_chirho()
            || !instance_chirho.binders_chirho.is_empty()
            || !instance_chirho.context_chirho.is_empty()
        {
            return Err(error_chirho(
                "declared body is outside its manifest method owner",
            ));
        }
        let provider_chirho = backend_chirho::declared_operation_chirho(key_chirho)
            .ok_or_else(|| error_chirho("method has no exact backend provider"))?;
        if declared_chirho.contract_chirho != provider_chirho.contract_chirho() {
            return Err(error_chirho("method backend contract drift"));
        }
        if !checked_chirho
            .expressions_chirho
            .contains_key(&method_chirho.binding_chirho.body_chirho.id_chirho)
        {
            return Err(error_chirho("method body lacks its checked type"));
        }
        let binder_chirho = self.binder_chirho(owner_chirho)?;
        let ty_chirho = self.types_chirho.copy_chirho(&binder_chirho.ty_chirho)?;
        self.primitive_expression_chirho(
            ty_chirho,
            provider_chirho,
            method_chirho.binding_chirho.span_chirho,
        )
    }

    fn primitive_expression_chirho(
        &mut self,
        mut ty_chirho: CoreTypeChirho,
        provider_chirho: ProviderChirho,
        span_chirho: haskelujah_span_chirho::SpanChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        let mut parameters_chirho = Vec::new();
        for _position_chirho in 0..provider_chirho.contract_chirho().arity_chirho {
            let CoreTypeChirho::FunChirho(argument_chirho, result_chirho, _) = ty_chirho else {
                return Err(error_chirho("backend body lacks a checked arrow"));
            };
            parameters_chirho.push(self.fresh_binder_chirho(*argument_chirho, span_chirho)?);
            ty_chirho = *result_chirho;
        }
        let mut evaluated_chirho = Vec::new();
        for parameter_chirho in &parameters_chirho {
            let ty_chirho = self.types_chirho.copy_chirho(&parameter_chirho.ty_chirho)?;
            evaluated_chirho.push(self.fresh_binder_chirho(ty_chirho, span_chirho)?);
        }
        self.types_chirho.reserve_term_chirho()?;
        let mut body_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: provider_chirho.primitive_chirho().into(),
            args_chirho: evaluated_chirho
                .iter()
                .map(|parameter_chirho| CoreExprChirho::VarChirho(parameter_chirho.id_chirho))
                .collect(),
        };
        // Force on the continuation stack, left-to-right, only after saturation.
        for (parameter_chirho, evaluated_chirho) in
            parameters_chirho.iter().zip(evaluated_chirho).rev()
        {
            self.types_chirho.reserve_term_chirho()?;
            body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(parameter_chirho.id_chirho)),
                bind_chirho: evaluated_chirho,
                result_ty_chirho: self.types_chirho.copy_chirho(&ty_chirho)?,
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DefaultChirho,
                    binders_chirho: vec![],
                    rhs_chirho: body_chirho,
                }],
            };
        }
        for parameter_chirho in parameters_chirho.into_iter().rev() {
            self.types_chirho.reserve_term_chirho()?;
            body_chirho = CoreExprChirho::LamChirho {
                binder_chirho: parameter_chirho,
                body_chirho: Box::new(body_chirho),
            };
        }
        Ok(body_chirho)
    }
}
fn error_chirho(message_chirho: &str) -> NextErrorChirho {
    NextErrorChirho::new_chirho(NextErrorKindChirho::UnsupportedChirho, message_chirho)
}
