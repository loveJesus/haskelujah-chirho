// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Direct, identity-preserving HIR elaboration. Never run the legacy desugarer or dictionary pass.
//! workflow: next-pipeline-chirho/check-and-elaborate-chirho

use haskelujah_core_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};
use haskelujah_hir_chirho::{
    BindingChirho, ExprChirho, ExprKindChirho, NameIdChirho, WiredNameChirho,
};
use haskelujah_tc_chirho::CheckedModuleChirho;
use std::collections::HashMap;

use super::{NextErrorChirho, NextErrorKindChirho, types_chirho::CoreTypesChirho};

#[path = "patterns_chirho.rs"]
mod patterns_chirho;

struct ElaboratorChirho<'checked_chirho> {
    types_chirho: CoreTypesChirho<'checked_chirho>,
    next_id_chirho: u32,
    depth_left_chirho: usize,
}

impl ElaboratorChirho<'_> {
    fn binder_chirho(
        &mut self,
        name_chirho: NameIdChirho,
    ) -> Result<BinderChirho, NextErrorChirho> {
        let checked_chirho = self.types_chirho.checked_chirho;
        let info_chirho = checked_chirho
            .module_chirho
            .names_chirho
            .get(name_chirho.0 as usize)
            .ok_or_else(|| {
                NextErrorChirho::new_chirho(
                    NextErrorKindChirho::InvalidChirho,
                    "invalid HIR binder",
                )
            })?;
        let ty_chirho = *checked_chirho
            .names_chirho
            .get(&name_chirho)
            .ok_or_else(|| {
                NextErrorChirho::new_chirho(
                    NextErrorKindChirho::InvalidChirho,
                    "untyped HIR binder",
                )
            })?;
        Ok(BinderChirho {
            id_chirho: CoreIdChirho(name_chirho.0),
            name_chirho: info_chirho.occurrence_chirho.clone(),
            ty_chirho: self
                .types_chirho
                .binding_type_chirho(name_chirho, ty_chirho)?,
            span_chirho: info_chirho.span_chirho,
        })
    }

    fn binding_body_chirho(
        &mut self,
        binding_chirho: &BindingChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        let variables_chirho = self
            .types_chirho
            .quantified_chirho(binding_chirho.name_chirho)?;
        if variables_chirho.len() > self.depth_left_chirho {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next Core abstraction depth ceiling exhausted",
            ));
        }
        self.depth_left_chirho -= variables_chirho.len();
        let result_chirho = (|| {
            let mut body_chirho = self.expression_chirho(&binding_chirho.body_chirho)?;
            for variable_chirho in variables_chirho.iter().rev() {
                body_chirho = CoreExprChirho::TyLamChirho {
                    ty_var_chirho: format!("next_type_{}_chirho", variable_chirho.0),
                    body_chirho: Box::new(body_chirho),
                };
            }
            Ok(body_chirho)
        })();
        self.depth_left_chirho += variables_chirho.len();
        result_chirho
    }

    fn expression_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        let cost_chirho = self.expansion_depth_chirho(expression_chirho)?;
        if cost_chirho > self.depth_left_chirho {
            return Err(NextErrorChirho::new_chirho(
                NextErrorKindChirho::ResourceChirho,
                "next Core expression depth ceiling exhausted",
            ));
        }
        self.depth_left_chirho -= cost_chirho;
        let result_chirho = self.expression_body_chirho(expression_chirho);
        self.depth_left_chirho += cost_chirho;
        result_chirho
    }

    fn expression_body_chirho(
        &mut self,
        expression_chirho: &ExprChirho,
    ) -> Result<CoreExprChirho, NextErrorChirho> {
        Ok(match &expression_chirho.kind_chirho {
            ExprKindChirho::VarChirho(name_chirho) => {
                let info_chirho = &self.types_chirho.checked_chirho.module_chirho.names_chirho
                    [name_chirho.0 as usize];
                let mut reference_chirho = match info_chirho.wired_chirho {
                    Some(WiredNameChirho::TrueChirho | WiredNameChirho::FalseChirho) => {
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho: info_chirho.occurrence_chirho.clone(),
                            args_chirho: vec![],
                        }
                    }
                    _ => CoreExprChirho::VarChirho(CoreIdChirho(name_chirho.0)),
                };
                let checked_chirho = self.types_chirho.checked_chirho;
                if let Some(arguments_chirho) = checked_chirho
                    .instantiations_chirho
                    .get(&expression_chirho.id_chirho)
                {
                    for argument_chirho in arguments_chirho {
                        reference_chirho = CoreExprChirho::TyAppChirho {
                            expr_chirho: Box::new(reference_chirho),
                            ty_chirho: self.types_chirho.convert_chirho(*argument_chirho)?,
                        };
                    }
                }
                reference_chirho
            }
            ExprKindChirho::CharChirho(value_chirho) => {
                CoreExprChirho::LitChirho(CoreLitChirho::CharChirho(*value_chirho))
            }
            ExprKindChirho::StringChirho(value_chirho) => {
                CoreExprChirho::LitChirho(CoreLitChirho::StringChirho(value_chirho.clone()))
            }
            ExprKindChirho::AppChirho(function_chirho, argument_chirho) => {
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(self.expression_chirho(function_chirho)?),
                    arg_chirho: Box::new(self.expression_chirho(argument_chirho)?),
                }
            }
            ExprKindChirho::LamChirho(parameters_chirho, body_chirho) => {
                let mut body_chirho = self.expression_chirho(body_chirho)?;
                for parameter_chirho in parameters_chirho.iter().rev() {
                    body_chirho = CoreExprChirho::LamChirho {
                        binder_chirho: self.binder_chirho(*parameter_chirho)?,
                        body_chirho: Box::new(body_chirho),
                    };
                }
                body_chirho
            }
            ExprKindChirho::LetChirho(bindings_chirho, body_chirho) => CoreExprChirho::LetChirho {
                rec_chirho: true,
                binds_chirho: bindings_chirho
                    .iter()
                    .map(|binding_chirho| {
                        Ok((
                            self.binder_chirho(binding_chirho.name_chirho)?,
                            self.binding_body_chirho(binding_chirho)?,
                        ))
                    })
                    .collect::<Result<_, NextErrorChirho>>()?,
                body_chirho: Box::new(self.expression_chirho(body_chirho)?),
            },
            ExprKindChirho::IfChirho(condition_chirho, then_chirho, else_chirho) => {
                let id_chirho = CoreIdChirho(self.next_id_chirho);
                self.next_id_chirho = self.next_id_chirho.checked_add(1).ok_or_else(|| {
                    NextErrorChirho::new_chirho(
                        NextErrorKindChirho::ResourceChirho,
                        "Core ID overflow",
                    )
                })?;
                let checked_chirho = self.types_chirho.checked_chirho;
                let condition_ty_chirho =
                    checked_chirho.expressions_chirho[&condition_chirho.id_chirho];
                let result_ty_chirho =
                    checked_chirho.expressions_chirho[&expression_chirho.id_chirho];
                CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(self.expression_chirho(condition_chirho)?),
                    bind_chirho: BinderChirho {
                        id_chirho,
                        name_chirho: format!("next_case_{}_chirho", id_chirho.0),
                        ty_chirho: self.types_chirho.convert_chirho(condition_ty_chirho)?,
                        span_chirho: condition_chirho.span_chirho,
                    },
                    result_ty_chirho: self.types_chirho.convert_chirho(result_ty_chirho)?,
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".into()),
                            binders_chirho: vec![],
                            rhs_chirho: self.expression_chirho(then_chirho)?,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".into()),
                            binders_chirho: vec![],
                            rhs_chirho: self.expression_chirho(else_chirho)?,
                        },
                    ],
                }
            }
            ExprKindChirho::AnnChirho(body_chirho, _) => self.expression_chirho(body_chirho)?,
            ExprKindChirho::MatchChirho(inputs_chirho, arms_chirho) => {
                self.match_chirho(expression_chirho, inputs_chirho, arms_chirho)?
            }
            ExprKindChirho::ListChirho(items_chirho) => {
                let mut tail_chirho = CoreExprChirho::ConAppChirho {
                    con_name_chirho: "[]".into(),
                    args_chirho: vec![],
                };
                for item_chirho in items_chirho.iter().rev() {
                    tail_chirho = CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".into(),
                        args_chirho: vec![self.expression_chirho(item_chirho)?, tail_chirho],
                    };
                }
                tail_chirho
            }
            ExprKindChirho::TupleChirho(items_chirho) => CoreExprChirho::ConAppChirho {
                con_name_chirho: format!("({})", ",".repeat(items_chirho.len().saturating_sub(1))),
                args_chirho: items_chirho
                    .iter()
                    .map(|item_chirho| self.expression_chirho(item_chirho))
                    .collect::<Result<_, _>>()?,
            },
        })
    }
}

pub(super) fn elaborate_chirho(
    checked_chirho: &CheckedModuleChirho,
) -> Result<CoreModuleChirho, NextErrorChirho> {
    let mut elaborator_chirho = ElaboratorChirho {
        types_chirho: CoreTypesChirho::new_chirho(checked_chirho),
        depth_left_chirho: 256,
        next_id_chirho: u32::try_from(checked_chirho.module_chirho.names_chirho.len()).map_err(
            |_| {
                NextErrorChirho::new_chirho(NextErrorKindChirho::ResourceChirho, "Core ID overflow")
            },
        )?,
    };
    let mut bindings_chirho = Vec::new();
    for declaration_chirho in &checked_chirho.module_chirho.data_chirho {
        for constructor_chirho in &declaration_chirho.constructors_chirho {
            bindings_chirho
                .push(elaborator_chirho.constructor_chirho(constructor_chirho.name_chirho)?);
        }
    }
    for binding_chirho in &checked_chirho.module_chirho.bindings_chirho {
        bindings_chirho.push(CoreBindingChirho {
            binder_chirho: elaborator_chirho.binder_chirho(binding_chirho.name_chirho)?,
            rhs_chirho: elaborator_chirho.binding_body_chirho(binding_chirho)?,
            is_rec_chirho: true,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
    let names_chirho = checked_chirho
        .module_chirho
        .names_chirho
        .iter()
        .enumerate()
        .map(|(index_chirho, info_chirho)| {
            (
                CoreIdChirho(index_chirho as u32),
                info_chirho.occurrence_chirho.clone(),
            )
        })
        .collect();
    Ok(CoreModuleChirho {
        name_chirho: checked_chirho.module_chirho.module_chirho.clone(),
        bindings_chirho,
        names_chirho,
        specialize_pragmas_chirho: HashMap::new(),
        foreign_exports_chirho: Vec::new(),
        provenance_chirho: Default::default(),
    })
}
