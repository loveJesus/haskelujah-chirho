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
    fn append_text_chirho(
        &mut self,
        text_chirho: CoreExprChirho,
        tail_chirho: CoreIdChirho,
        result_ty_chirho: &CoreTypeChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        use haskelujah_hir_chirho::{NameIdChirho, NamespaceChirho, UnitIdentityChirho};
        use haskelujah_tc_chirho::{TyConChirho, TyNodeChirho};
        let text_type_chirho = CoreTypeChirho::AppChirho(
            Box::new(CoreTypeChirho::ConChirho("[]".into())),
            Box::new(CoreTypeChirho::ConChirho("Char".into())),
        );
        if *result_ty_chirho != text_type_chirho {
            return Err(error_chirho("formatting result is not checked String"));
        }
        let CoreTypeChirho::AppChirho(_, text_element_chirho) = result_ty_chirho else {
            return Err(error_chirho("formatting result lost its list element"));
        };
        let text_element_chirho = self.types_chirho.copy_chirho(text_element_chirho)?;
        let checked_chirho = self.types_chirho.checked_chirho;
        let mut name_chirho = None;
        for (index_chirho, info_chirho) in
            checked_chirho.module_chirho.names_chirho.iter().enumerate()
        {
            self.types_chirho.reserve_term_chirho()?;
            if info_chirho.unit_chirho == UnitIdentityChirho::CompilerChirho
                && info_chirho.module_chirho
                    == crate::next_chirho::library_chirho::BASE_MODULE_CHIRHO
                && info_chirho.namespace_chirho == NamespaceChirho::ValueChirho
                && info_chirho.occurrence_chirho == "++"
                && name_chirho
                    .replace(NameIdChirho(index_chirho as u32))
                    .is_some()
            {
                return Err(error_chirho("text join has multiple original-name ids"));
            }
        }
        let name_chirho = name_chirho
            .ok_or_else(|| error_chirho("formatting needs the checked source append original"))?;
        let scheme_chirho = checked_chirho
            .schemes_chirho
            .get(&name_chirho)
            .ok_or_else(|| error_chirho("text join has no checked scheme"))?;
        if !scheme_chirho.context_chirho.is_empty() || scheme_chirho.quantified_chirho.len() != 1 {
            return Err(error_chirho("text join context or quantifiers changed"));
        }
        let arena_chirho = &checked_chirho.solver_chirho.arena_chirho;
        let TyNodeChirho::FunChirho(left_chirho, rest_chirho) =
            arena_chirho.node_chirho(scheme_chirho.body_chirho)?
        else {
            return Err(error_chirho("text join lacks its first arrow"));
        };
        let TyNodeChirho::FunChirho(right_chirho, output_chirho) =
            arena_chirho.node_chirho(*rest_chirho)?
        else {
            return Err(error_chirho("text join lacks its second arrow"));
        };
        let TyNodeChirho::AppChirho(list_chirho, element_chirho) =
            arena_chirho.node_chirho(*left_chirho)?
        else {
            return Err(error_chirho("text join does not take a polymorphic list"));
        };
        if left_chirho != right_chirho
            || left_chirho != output_chirho
            || *element_chirho != scheme_chirho.quantified_chirho[0]
            || !matches!(
                arena_chirho.node_chirho(*list_chirho)?,
                TyNodeChirho::ConChirho(TyConChirho::ListChirho)
            )
        {
            return Err(error_chirho("text join is not forall a. [a] -> [a] -> [a]"));
        }
        for _node_chirho in 0..5 {
            self.types_chirho.reserve_term_chirho()?;
        }
        Ok(CoreExprChirho::AppChirho {
            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::TyAppChirho {
                    expr_chirho: Box::new(CoreExprChirho::VarChirho(CoreIdChirho(name_chirho.0))),
                    ty_chirho: text_element_chirho,
                }),
                arg_chirho: Box::new(text_chirho),
            }),
            arg_chirho: Box::new(CoreExprChirho::VarChirho(tail_chirho)),
        })
    }
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
        if let Some(provider_chirho) =
            backend_chirho::io_chirho::operation_chirho(&info_chirho.original_chirho())
        {
            if declaration_chirho.contract_chirho != provider_chirho.contract_chirho() {
                return Err(error_chirho("IO primitive contract drift"));
            }
            let binder_chirho = self.binder_chirho(declaration_chirho.name_chirho)?;
            let ty_chirho = self.types_chirho.copy_chirho(&binder_chirho.ty_chirho)?;
            let rhs_chirho =
                self.io_expression_chirho(ty_chirho, provider_chirho, info_chirho.span_chirho)?;
            return Ok(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
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
        let demand_chirho = provider_chirho.demand_chirho();
        let forced_chirho: std::collections::BTreeSet<_> =
            demand_chirho.force_order_chirho.iter().copied().collect();
        let operands_chirho: std::collections::BTreeSet<_> =
            demand_chirho.arguments_chirho.iter().copied().collect();
        if parameters_chirho.len() > 3
            || forced_chirho != operands_chirho
            || forced_chirho.len() != demand_chirho.force_order_chirho.len()
            || operands_chirho.len() != demand_chirho.arguments_chirho.len()
            || forced_chirho
                .iter()
                .any(|&position_chirho| position_chirho >= parameters_chirho.len())
            || forced_chirho.len() + usize::from(demand_chirho.tail_chirho.is_some())
                != parameters_chirho.len()
            || demand_chirho.tail_chirho.is_some_and(|position_chirho| {
                position_chirho >= parameters_chirho.len()
                    || forced_chirho.contains(&position_chirho)
            })
        {
            return Err(error_chirho(
                "backend demand does not cover its checked parameters exactly once",
            ));
        }
        let mut evaluated_chirho: Vec<Option<BinderChirho>> = vec![None; parameters_chirho.len()];
        for &position_chirho in demand_chirho.arguments_chirho {
            let ty_chirho = self
                .types_chirho
                .copy_chirho(&parameters_chirho[position_chirho].ty_chirho)?;
            evaluated_chirho[position_chirho] =
                Some(self.fresh_binder_chirho(ty_chirho, span_chirho)?);
        }
        self.types_chirho.reserve_term_chirho()?;
        let mut body_chirho = CoreExprChirho::PrimOpChirho {
            name_chirho: provider_chirho.primitive_chirho().into(),
            args_chirho: demand_chirho
                .arguments_chirho
                .iter()
                .map(|&position_chirho| {
                    evaluated_chirho[position_chirho]
                        .as_ref()
                        .map(|binder_chirho| CoreExprChirho::VarChirho(binder_chirho.id_chirho))
                        .ok_or_else(|| error_chirho("primitive operand has no demanded binder"))
                })
                .collect::<Result<_, _>>()?,
        };
        if let Some(tail_chirho) = demand_chirho.tail_chirho {
            if parameters_chirho[tail_chirho].ty_chirho != ty_chirho {
                return Err(error_chirho(
                    "backend tail type differs from its text result",
                ));
            }
            body_chirho = self.append_text_chirho(
                body_chirho,
                parameters_chirho[tail_chirho].id_chirho,
                &ty_chirho,
            )?;
        }
        // The force order belongs to the source provider. Operands remain positional;
        // a lazy tail is never passed to PrimOp, whose machine frame forces every operand.
        for &position_chirho in demand_chirho.force_order_chirho.iter().rev() {
            self.types_chirho.reserve_term_chirho()?;
            body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(
                    parameters_chirho[position_chirho].id_chirho,
                )),
                bind_chirho: evaluated_chirho[position_chirho]
                    .take()
                    .ok_or_else(|| error_chirho("demanded operand has no case binder"))?,
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
