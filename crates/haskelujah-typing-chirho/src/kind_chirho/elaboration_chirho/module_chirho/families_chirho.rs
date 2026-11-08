// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checked family equation traversal and declaration publication.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;

impl TypeElaboratorChirho<'_> {
    pub(super) fn publish_closed_families_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
        group_chirho: &[usize],
    ) -> Result<(), ElaborationErrorChirho> {
        use crate::infer_chirho::{
            TypeFamilyBranchChirho, TypeFamilyBranchIdChirho, TypeFamilyDefinitionChirho,
            TypeFamilyIdChirho, TypeFamilyReductionChirho, family_head_with_predeclared_chirho,
            held_equation_inputs_chirho,
        };
        use haskelujah_ast_chirho::name_chirho::{
            TypeConstructorIdentityChirho, TypeConstructorRoleChirho,
        };
        let mut predeclared_chirho = HashMap::new();
        for index_chirho in group_chirho {
            let DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                data_family_chirho: false,
                ..
            } = &module_chirho.decls_chirho[*index_chirho]
            else {
                continue;
            };
            let key_chirho = self.local_key_chirho(name_chirho)?;
            if let Some(scheme_chirho) = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(&key_chirho)
                .filter(|scheme_chirho| {
                    scheme_chirho.owner_chirho.is_none() && !scheme_chirho.held_chirho
                })
            {
                self.state_chirho
                    .charge_chirho(1, key_chirho.len(), name_chirho.span_chirho())?;
                predeclared_chirho
                    .insert(key_chirho.to_string(), scheme_chirho.reduction_arity_chirho);
            }
        }
        for index_chirho in group_chirho {
            let DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                closed_chirho: true,
                data_family_chirho: false,
                equations_chirho,
                span_chirho,
                ..
            } = &module_chirho.decls_chirho[*index_chirho]
            else {
                continue;
            };
            let span_chirho = *span_chirho;
            let key_chirho = self.local_key_chirho(name_chirho)?;
            let header_chirho = self
                .registry_chirho
                .declaration_schemes_chirho()
                .get_chirho(&key_chirho)
                .cloned()
                .ok_or_else(|| {
                    Self::error_chirho("checked family lost its declaration telescope", span_chirho)
                })?;
            if header_chirho.owner_chirho.is_some()
                || header_chirho.held_chirho
                || header_chirho.role_chirho != TypeConstructorRoleChirho::TypeFamilyChirho
            {
                return Err(Self::error_chirho(
                    "closed family declaration was not fully checked",
                    span_chirho,
                ));
            }
            let Some(TypeConstructorIdentityChirho::SourceChirho {
                defining_module_chirho,
                name_chirho: defining_name_chirho,
            }) = header_chirho
                .declaration_chirho
                .as_ref()
                .map(|proof_chirho| &proof_chirho.identity_chirho)
            else {
                return Err(Self::error_chirho(
                    "closed family has no checked source producer",
                    span_chirho,
                ));
            };
            if self.normalization_module_chirho.as_deref() != Some(defining_module_chirho.as_str())
                || defining_name_chirho != name_chirho.text_chirho()
            {
                return Err(Self::error_chirho(
                    "closed family producer disagrees with its defining declaration",
                    span_chirho,
                ));
            }
            let identity_chirho = TypeFamilyIdChirho::SourceChirho {
                module_chirho: self
                    .state_chirho
                    .name_chirho(defining_module_chirho, span_chirho)?,
                name_chirho: self
                    .state_chirho
                    .name_chirho(defining_name_chirho, span_chirho)?,
            };
            if identity_chirho.key_chirho() != key_chirho.as_ref() {
                return Err(Self::error_chirho(
                    "checked family identity lost its declaration boundary",
                    span_chirho,
                ));
            }
            let rows_chirho = self
                .declaration_variables_chirho
                .get_mut(&key_chirho)
                .and_then(|variables_chirho| variables_chirho.closed_family_equations_chirho.take())
                .ok_or_else(|| {
                    Self::error_chirho(
                        "closed family lost its checked equation producers",
                        span_chirho,
                    )
                })?;
            if rows_chirho.len() != equations_chirho.len() {
                return Err(Self::error_chirho(
                    "closed family equation producers were truncated",
                    span_chirho,
                ));
            }
            self.state_chirho
                .charge_chirho(rows_chirho.len(), rows_chirho.len(), span_chirho)?;
            let mut branches_chirho: Vec<TypeFamilyBranchChirho> =
                Vec::with_capacity(rows_chirho.len());
            let mut held_boundary_chirho = false;
            for (ordinal_chirho, row_chirho) in rows_chirho.into_iter().enumerate() {
                let ordinal_chirho = u32::try_from(ordinal_chirho).map_err(|_| {
                    self.state_chirho.resource_failure_chirho(
                        "closed family equation index bound exhausted",
                        span_chirho,
                    )
                })?;
                let mut held_chirho = held_boundary_chirho || row_chirho.terms_chirho.is_none();
                let (lhs_chirho, rhs_chirho, parameters_chirho) = if held_chirho {
                    self.state_chirho.charge_chirho(
                        header_chirho.reduction_arity_chirho,
                        header_chirho.reduction_arity_chirho,
                        span_chirho,
                    )?;
                    held_equation_inputs_chirho(
                        header_chirho.reduction_arity_chirho,
                        ordinal_chirho,
                    )
                } else {
                    let (lhs_chirho, rhs_chirho) = row_chirho.terms_chirho.ok_or_else(|| {
                        Self::error_chirho(
                            "held family equation cannot supply checked terms",
                            span_chirho,
                        )
                    })?;
                    let lhs_chirho = lhs_chirho
                        .iter()
                        .map(|ty_chirho| self.state_chirho.resolve_chirho(ty_chirho, span_chirho))
                        .collect::<Result<Vec<_>, _>>()?;
                    let rhs_chirho = self.state_chirho.resolve_chirho(&rhs_chirho, span_chirho)?;
                    match self.state_chirho.abstract_family_equation_chirho(
                        &lhs_chirho,
                        &rhs_chirho,
                        span_chirho,
                    )? {
                        Some(terms_chirho) => terms_chirho,
                        None => {
                            held_chirho = true;
                            self.state_chirho.charge_chirho(
                                header_chirho.reduction_arity_chirho,
                                header_chirho.reduction_arity_chirho,
                                span_chirho,
                            )?;
                            held_equation_inputs_chirho(
                                header_chirho.reduction_arity_chirho,
                                ordinal_chirho,
                            )
                        }
                    }
                };
                if lhs_chirho.len() != header_chirho.reduction_arity_chirho {
                    return Err(Self::error_chirho(
                        "checked family equation omits part of its complete telescope",
                        span_chirho,
                    ));
                }
                let mut blockers_chirho = Vec::new();
                for (prior_chirho, branch_chirho) in branches_chirho.iter().enumerate() {
                    self.state_chirho.charge_chirho(1, 1, span_chirho)?;
                    if branch_chirho.held_chirho {
                        blockers_chirho.push(prior_chirho);
                        continue;
                    }
                    let compatible_chirho = self.state_chirho.family_branches_compatible_chirho(
                        &branch_chirho.lhs_chirho,
                        &branch_chirho.rhs_chirho,
                        &lhs_chirho,
                        &rhs_chirho,
                        &|name_chirho| {
                            family_head_with_predeclared_chirho(
                                &self.registry_chirho,
                                &predeclared_chirho,
                                name_chirho,
                            )
                        },
                        span_chirho,
                    )?;
                    if !compatible_chirho {
                        blockers_chirho.push(prior_chirho);
                    }
                }
                held_boundary_chirho |= held_chirho;
                self.state_chirho
                    .charge_chirho(key_chirho.len(), key_chirho.len(), span_chirho)?;
                branches_chirho.push(TypeFamilyBranchChirho {
                    id_chirho: TypeFamilyBranchIdChirho {
                        family_chirho: identity_chirho.clone(),
                        producer_module_chirho: self
                            .state_chirho
                            .name_chirho(defining_module_chirho, span_chirho)?,
                        declaration_start_chirho: span_chirho.start_chirho().as_u32_chirho(),
                        equation_index_chirho: ordinal_chirho,
                    },
                    binders_chirho: parameters_chirho,
                    lhs_chirho,
                    rhs_chirho,
                    blockers_chirho,
                    held_chirho,
                });
            }
            let definition_chirho = TypeFamilyDefinitionChirho::new_chirho(
                identity_chirho,
                header_chirho.reduction_arity_chirho,
                TypeFamilyReductionChirho::ClosedChirho,
                branches_chirho,
            )
            .map_err(|error_chirho| Self::error_chirho(error_chirho.to_string(), span_chirho))?;
            self.registry_chirho
                .insert_chirho(definition_chirho)
                .map_err(|error_chirho| {
                    Self::error_chirho(error_chirho.to_string(), span_chirho)
                })?;
        }
        Ok(())
    }

    pub(super) fn elaborate_family_equations_chirho(
        &mut self,
        equations_chirho: &mut [TypeFamilyEquationChirho],
        key_chirho: &str,
        closed_chirho: bool,
        span_chirho: SpanChirho,
    ) -> Result<(), ElaborationErrorChirho> {
        let mut checked_chirho = if closed_chirho {
            self.state_chirho.charge_chirho(
                equations_chirho.len(),
                equations_chirho.len(),
                span_chirho,
            )?;
            Some(Vec::with_capacity(equations_chirho.len()))
        } else {
            None
        };
        for equation_chirho in equations_chirho {
            let terms_chirho = self.elaborate_equation_chirho(equation_chirho, key_chirho)?;
            if let Some(checked_chirho) = &mut checked_chirho {
                checked_chirho.push(CheckedFamilyEquationChirho { terms_chirho });
            }
        }
        if let Some(checked_chirho) = checked_chirho {
            self.declaration_variables_chirho
                .get_mut(key_chirho)
                .ok_or_else(|| {
                    Self::error_chirho(
                        "checked closed family lost its declaration owner",
                        span_chirho,
                    )
                })?
                .closed_family_equations_chirho = Some(checked_chirho);
        }
        Ok(())
    }

    pub(super) fn elaborate_equation_chirho(
        &mut self,
        equation_chirho: &mut TypeFamilyEquationChirho,
        key_chirho: &str,
    ) -> Result<Option<(Vec<TyChirho>, TyChirho)>, ElaborationErrorChirho> {
        if equation_chirho.is_held_chirho() {
            return Ok(None);
        }
        self.scoped_chirho(|this_chirho| {
            // Equation variables have their own lexical owner, independent of the header.
            let header_names_chirho = this_chirho
                .declaration_variables_chirho
                .get(key_chirho)
                .map(|variables_chirho| {
                    variables_chirho
                        .written_chirho
                        .iter()
                        .map(|(name_chirho, _, _)| name_chirho.clone())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            for name_chirho in header_names_chirho {
                this_chirho.hide_variable_chirho(&name_chirho, equation_chirho.span_chirho)?;
            }
            if let Some(binders_chirho) = &mut equation_chirho.explicit_binders_chirho {
                this_chirho.bind_forall_chirho(binders_chirho)?;
            }
            let head_chirho = equation_chirho.head_chirho.as_mut().ok_or_else(|| {
                Self::error_chirho("family equation has no head", equation_chirho.span_chirho)
            })?;
            this_chirho
                .elaborate_family_application_chirho(
                    head_chirho,
                    &mut equation_chirho.kind_args_chirho,
                    &mut equation_chirho.lhs_types_chirho,
                    &mut equation_chirho.rhs_chirho,
                    equation_chirho.span_chirho,
                )
                .map(Some)
        })
    }

    pub(in crate::kind_chirho::elaboration_chirho) fn elaborate_family_application_chirho(
        &mut self,
        head_chirho: &mut NameChirho,
        hidden_chirho: &mut Vec<TypeChirho>,
        visible_chirho: &mut Vec<TypeChirho>,
        rhs_chirho: &mut TypeChirho,
        span_chirho: SpanChirho,
    ) -> Result<(Vec<TyChirho>, TyChirho), ElaborationErrorChirho> {
        let mut constraint_chirho = ConstraintChirho::ClassChirho {
            class_chirho: Box::new(head_chirho.clone()),
            kind_args_chirho: std::mem::take(hidden_chirho),
            args_chirho: std::mem::take(visible_chirho),
            span_chirho,
        };
        // Families return arbitrary kinds, so use the spine without imposing Constraint.
        let ConstraintChirho::ClassChirho {
            class_chirho,
            kind_args_chirho,
            args_chirho,
            ..
        } = &mut constraint_chirho
        else {
            unreachable!()
        };
        let mut syntax_chirho = TypeChirho::ConChirho(class_chirho.as_ref().clone());
        for argument_chirho in std::mem::take(kind_args_chirho) {
            syntax_chirho = TypeChirho::KindApplicationChirho {
                fun_chirho: Box::new(syntax_chirho),
                arg_chirho: Box::new(argument_chirho),
                written_chirho: !self.rechecking_chirho,
                span_chirho,
            };
        }
        for argument_chirho in std::mem::take(args_chirho) {
            syntax_chirho = TypeChirho::AppChirho {
                fun_chirho: Box::new(syntax_chirho),
                arg_chirho: Box::new(argument_chirho),
                span_chirho,
            };
        }
        let left_chirho = self.infer_type_chirho(&mut syntax_chirho)?;
        let right_chirho = self.infer_type_at_kind_chirho(rhs_chirho, &left_chirho.kind_chirho)?;
        loop {
            match syntax_chirho {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    visible_chirho.push(*arg_chirho);
                    syntax_chirho = *fun_chirho;
                }
                TypeChirho::KindApplicationChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => {
                    hidden_chirho.push(*arg_chirho);
                    syntax_chirho = *fun_chirho;
                }
                TypeChirho::ConChirho(name_chirho) => {
                    *head_chirho = name_chirho;
                    break;
                }
                _ => {
                    return Err(Self::error_chirho(
                        "family application lost its checked head",
                        span_chirho,
                    ));
                }
            }
        }
        visible_chirho.reverse();
        hidden_chirho.reverse();
        let mut term_chirho = left_chirho.term_chirho;
        let mut arguments_chirho = Vec::new();
        while let TyChirho::AppChirho(fun_chirho, argument_chirho) = term_chirho {
            self.state_chirho.charge_chirho(1, 1, span_chirho)?;
            arguments_chirho.push(*argument_chirho);
            term_chirho = *fun_chirho;
        }
        arguments_chirho.reverse();
        Ok((arguments_chirho, right_chirho.term_chirho))
    }
}
