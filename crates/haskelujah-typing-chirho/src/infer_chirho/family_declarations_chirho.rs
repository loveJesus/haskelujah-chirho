// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equation-local binding and registration for open and closed type families.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;

/// Matching inputs retain their visibility when exported to another module.
/// A kind input is not an extra ordinary argument or mere source annotation.
#[derive(Clone, Debug)]
pub struct TypeFamilyClauseChirho {
    pub kind_inputs_chirho: Vec<TyChirho>,
    pub type_inputs_chirho: Vec<TyChirho>,
    pub result_chirho: TyChirho,
}

impl TypeFamilyClauseChirho {
    pub fn ordinary_chirho(type_inputs_chirho: Vec<TyChirho>, result_chirho: TyChirho) -> Self {
        Self {
            kind_inputs_chirho: Vec::new(),
            type_inputs_chirho,
            result_chirho,
        }
    }
}

impl InferCtxChirho {
    /// A validated closed-family dependency can determine arguments from a
    /// result. Commit the tentative substitutions only if forward reduction
    /// then proves the ORIGINAL equality; a row-local variable cannot escape.
    pub(super) fn improve_closed_family_equality_chirho(
        &self,
        left_chirho: &TyChirho,
        right_chirho: &TyChirho,
        span_chirho: SpanChirho,
    ) -> Option<SubstChirho> {
        let elaboration_chirho = self.kind_elaboration_chirho.as_ref()?;
        for (application_chirho, result_chirho) in
            [(left_chirho, right_chirho), (right_chirho, left_chirho)]
        {
            let (head_chirho, arguments_chirho) =
                family_application_spine_chirho(application_chirho);
            let TyChirho::ConChirho(name_chirho) = head_chirho else {
                continue;
            };
            if self.is_known_nominal_name_chirho(name_chirho) {
                continue;
            }
            let Some(positions_chirho) = elaboration_chirho
                .closed_family_injectivity_chirho
                .get(name_chirho)
            else {
                continue;
            };
            let Some(equalities_chirho) = self
                .declaration_contracts_chirho
                .inverse_closed_family_chirho(
                    name_chirho,
                    &arguments_chirho,
                    result_chirho,
                    positions_chirho,
                    &|name_chirho| self.type_families_chirho.contains_key(name_chirho),
                )
            else {
                continue;
            };
            let mut substitution_chirho = SubstChirho::empty_chirho();
            let mut failed_chirho = false;
            for (argument_chirho, determined_chirho) in equalities_chirho {
                match unify_chirho(
                    &substitution_chirho.apply_ty_chirho(&argument_chirho),
                    &substitution_chirho.apply_ty_chirho(&determined_chirho),
                    span_chirho,
                ) {
                    Ok(next_chirho) => {
                        substitution_chirho = next_chirho.compose_chirho(&substitution_chirho)
                    }
                    Err(_) => {
                        failed_chirho = true;
                        break;
                    }
                }
            }
            if !failed_chirho
                && self.normalize_ty_chirho(&substitution_chirho.apply_ty_chirho(left_chirho))
                    == self.normalize_ty_chirho(&substitution_chirho.apply_ty_chirho(right_chirho))
            {
                return Some(substitution_chirho);
            }
        }
        None
    }

    pub(super) fn reduce_type_family_application_chirho(
        &self,
        name_chirho: &str,
        arguments_chirho: &[TyChirho],
    ) -> Option<TyChirho> {
        self.reduce_family_spine_chirho(
            name_chirho,
            &arguments_chirho
                .iter()
                .map(|argument_chirho| (argument_chirho.clone(), false))
                .collect::<Vec<_>>(),
        )
    }

    fn reduce_family_spine_chirho(
        &self,
        name_chirho: &str,
        arguments_chirho: &[(TyChirho, bool)],
    ) -> Option<TyChirho> {
        use crate::families_chirho::{FamilyReductionChirho, reduce_one_equation_chirho};
        if self.is_known_nominal_name_chirho(name_chirho) {
            return None;
        }
        let mut budget_chirho = 16_384;
        let terms_chirho: Vec<_> = arguments_chirho
            .iter()
            .map(|(term_chirho, _)| term_chirho.clone())
            .collect();
        for equations_chirho in self.lookup_type_family_equation_sets_chirho(name_chirho) {
            for equation_chirho in equations_chirho {
                let hidden_chirho = equation_chirho.kind_inputs_chirho.len();
                let arity_chirho = hidden_chirho + equation_chirho.type_inputs_chirho.len();
                if arguments_chirho.len() < arity_chirho
                    || arguments_chirho.iter().take(arity_chirho).enumerate().any(
                        |(index_chirho, (_, invisible_chirho))| {
                            *invisible_chirho != (index_chirho < hidden_chirho)
                        },
                    )
                {
                    return None;
                }
                match reduce_one_equation_chirho(
                    equation_chirho
                        .kind_inputs_chirho
                        .iter()
                        .chain(&equation_chirho.type_inputs_chirho),
                    &equation_chirho.result_chirho,
                    &terms_chirho[..arity_chirho],
                    &|head_chirho| {
                        // Matching visits every node: use bounded hash lookups,
                        // not the legacy all-family suffix search per node.
                        !self.is_known_nominal_name_chirho(head_chirho)
                            && (self.type_families_chirho.contains_key(head_chirho)
                                || self.type_families_chirho.contains_key(
                                    head_chirho.rsplit('.').next().unwrap_or(head_chirho),
                                ))
                    },
                    &mut budget_chirho,
                ) {
                    FamilyReductionChirho::ReducedChirho(mut result_chirho) => {
                        for (argument_chirho, invisible_chirho) in &arguments_chirho[arity_chirho..]
                        {
                            result_chirho = if *invisible_chirho {
                                TyChirho::KindAppChirho(
                                    Box::new(result_chirho),
                                    Box::new(argument_chirho.clone()),
                                )
                            } else {
                                TyChirho::AppChirho(
                                    Box::new(result_chirho),
                                    Box::new(argument_chirho.clone()),
                                )
                            };
                        }
                        return Some(result_chirho);
                    }
                    FamilyReductionChirho::ApartChirho => {}
                    FamilyReductionChirho::StuckChirho | FamilyReductionChirho::LimitedChirho => {
                        return None;
                    }
                }
            }
        }
        if arguments_chirho
            .iter()
            .any(|(_, invisible_chirho)| *invisible_chirho)
        {
            return None;
        }
        reduce_builtin_type_family_application_chirho(name_chirho, &terms_chirho)
    }

    pub(super) fn reduce_families_chirho(
        &self,
        ty_chirho: &TyChirho,
        depth_chirho: usize,
    ) -> TyChirho {
        if depth_chirho > 100 {
            return ty_chirho.clone();
        }
        match ty_chirho {
            TyChirho::ConChirho(_) | TyChirho::AppChirho(_, _) | TyChirho::KindAppChirho(_, _) => {
                let (head_chirho, arguments_chirho) = family_application_spine_chirho(ty_chirho);
                let arguments_chirho: Vec<_> = arguments_chirho
                    .into_iter()
                    .map(|(argument_chirho, invisible_chirho)| {
                        (
                            self.reduce_families_chirho(argument_chirho, depth_chirho + 1),
                            invisible_chirho,
                        )
                    })
                    .collect();
                if let TyChirho::ConChirho(name_chirho) = head_chirho {
                    if let Some(result_chirho) =
                        self.reduce_family_spine_chirho(name_chirho, &arguments_chirho)
                    {
                        return self.reduce_families_chirho(&result_chirho, depth_chirho + 1);
                    }
                }
                let mut result_chirho = if matches!(head_chirho, TyChirho::ConChirho(_)) {
                    head_chirho.clone()
                } else {
                    self.reduce_families_chirho(head_chirho, depth_chirho + 1)
                };
                for (argument_chirho, invisible_chirho) in arguments_chirho {
                    result_chirho = if invisible_chirho {
                        TyChirho::KindAppChirho(Box::new(result_chirho), Box::new(argument_chirho))
                    } else {
                        TyChirho::AppChirho(Box::new(result_chirho), Box::new(argument_chirho))
                    };
                }
                result_chirho
            }
            TyChirho::FunChirho(argument_chirho, result_chirho, multiplicity_chirho) => {
                TyChirho::FunChirho(
                    Box::new(self.reduce_families_chirho(argument_chirho, depth_chirho)),
                    Box::new(self.reduce_families_chirho(result_chirho, depth_chirho)),
                    *multiplicity_chirho,
                )
            }
            TyChirho::ListChirho(element_chirho) => TyChirho::ListChirho(Box::new(
                self.reduce_families_chirho(element_chirho, depth_chirho),
            )),
            TyChirho::TupleChirho(elements_chirho) => TyChirho::TupleChirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| self.reduce_families_chirho(element_chirho, depth_chirho))
                    .collect(),
            ),
            TyChirho::ForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::ForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.reduce_families_chirho(body_chirho, depth_chirho)),
            },
            TyChirho::RequiredForallChirho {
                vars_chirho,
                body_chirho,
            } => TyChirho::RequiredForallChirho {
                vars_chirho: vars_chirho.clone(),
                body_chirho: Box::new(self.reduce_families_chirho(body_chirho, depth_chirho)),
            },
            _ => ty_chirho.clone(),
        }
    }

    pub(super) fn register_module_type_families_chirho(&mut self, module_chirho: &ModuleChirho) {
        for declaration_chirho in &module_chirho.decls_chirho {
            match declaration_chirho {
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    data_chirho,
                    body_chirho,
                    span_chirho,
                    ..
                } => {
                    let equations_chirho: Vec<_> = body_chirho
                        .equations_chirho()
                        .iter()
                        .filter_map(|equation_chirho| {
                            self.lower_local_family_equation_chirho(
                                &equation_chirho.lhs_types_chirho,
                                &equation_chirho.rhs_chirho,
                            )
                        })
                        .collect();
                    if let Err(reason_chirho) =
                        self.declaration_contracts_chirho.record_family_chirho(
                            name_chirho.text_chirho(),
                            body_chirho,
                            &equations_chirho,
                        )
                    {
                        self.diagnostics_chirho.push_chirho(
                            DiagnosticChirho::error_with_code_chirho(
                                ErrorCodeChirho::error_chirho(300),
                                reason_chirho,
                                *span_chirho,
                            ),
                        );
                    }
                    // Data families have nominal applications, not reduction rows.
                    // Workflow: language-features-chirho/declaration-kinds-chirho.
                    if !data_chirho {
                        self.register_elaborated_type_family_chirho(
                            name_chirho.text_chirho().to_string(),
                            equations_chirho,
                        );
                    }
                }
                DeclChirho::TypeFamilyInstanceDeclChirho {
                    family_name_chirho,
                    lhs_types_chirho,
                    rhs_chirho,
                    ..
                } => {
                    let Some(equation_chirho) =
                        self.lower_local_family_equation_chirho(lhs_types_chirho, rhs_chirho)
                    else {
                        continue;
                    };
                    self.register_elaborated_type_family_chirho(
                        family_name_chirho.text_chirho().to_string(),
                        vec![equation_chirho],
                    );
                }
                _ => {}
            }
        }
    }

    /// The source converter consumes exactly the kind arguments established at
    /// these occurrences. Family definitions cannot eagerly reduce themselves,
    /// and a variable seen only on the RHS is not an invented row parameter.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    fn lower_local_family_equation_chirho(
        &mut self,
        patterns_chirho: &[TypeChirho],
        result_chirho: &TypeChirho,
    ) -> Option<TypeFamilyClauseChirho> {
        let mut variables_chirho = HashMap::new();
        let policy_chirho =
            super::ast_conversion_chirho::TypeConversionChirho::FamilyEquationChirho;
        let hidden_chirho = self
            .kind_elaboration_chirho
            .as_ref()
            .and_then(|elaboration_chirho| {
                elaboration_chirho
                    .equation_inputs_chirho
                    .get(&result_chirho.span_chirho())
            })
            .cloned()
            .unwrap_or_default();
        let kind_inputs_chirho: Vec<_> = hidden_chirho
            .iter()
            .map(|argument_chirho| {
                self.kind_term_type_chirho(argument_chirho, &mut variables_chirho, &mut Vec::new())
            })
            .collect();
        let type_inputs_chirho: Vec<_> = patterns_chirho
            .iter()
            .map(|pattern_chirho| {
                self.ast_type_with_policy_chirho(
                    pattern_chirho,
                    &mut variables_chirho,
                    policy_chirho,
                )
            })
            .collect();
        let body_chirho =
            self.ast_type_with_policy_chirho(result_chirho, &mut variables_chirho, policy_chirho);
        let bound_chirho: HashSet<_> = kind_inputs_chirho
            .iter()
            .chain(&type_inputs_chirho)
            .flat_map(TyChirho::free_vars_chirho)
            .collect();
        if !body_chirho
            .free_vars_chirho()
            .iter()
            .all(|variable_chirho| bound_chirho.contains(variable_chirho))
        {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(300),
                    "family equation result contains variables outside its matching inputs",
                    result_chirho.span_chirho(),
                ));
            return None;
        }
        Some(TypeFamilyClauseChirho {
            kind_inputs_chirho,
            type_inputs_chirho,
            result_chirho: body_chirho,
        })
    }
}

/// Inspect one application spine without treating invisible inputs as ordinary
/// arguments. Used by reduction and by the no-injectivity deferral check.
pub(super) fn family_application_spine_chirho(
    ty_chirho: &TyChirho,
) -> (&TyChirho, Vec<(&TyChirho, bool)>) {
    let mut head_chirho = ty_chirho;
    let mut arguments_chirho = Vec::new();
    loop {
        match head_chirho {
            TyChirho::AppChirho(fun_chirho, argument_chirho) => {
                arguments_chirho.push((argument_chirho.as_ref(), false));
                head_chirho = fun_chirho;
            }
            TyChirho::KindAppChirho(fun_chirho, argument_chirho) => {
                arguments_chirho.push((argument_chirho.as_ref(), true));
                head_chirho = fun_chirho;
            }
            _ => break,
        }
    }
    arguments_chirho.reverse();
    (head_chirho, arguments_chirho)
}

// Explicit associated instances consume the kind pass's enclosing-instance
// elaboration through the same converter as top-level rows.
impl InferCtxChirho {
    /// Explicit equations override defaults; both retain this module's proven
    /// promoted identity while keeping their enclosing instance binding contract.
    /// Workflow: language-features-chirho/flat-type-syntax-chirho.
    pub(super) fn register_associated_family_equations_chirho(
        &mut self,
        class_name_chirho: &str,
        types_chirho: &[TypeChirho],
        instances_chirho: &[haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho],
        span_chirho: SpanChirho,
    ) {
        for instance_chirho in instances_chirho {
            let Some(equation_chirho) = self.lower_local_family_equation_chirho(
                &instance_chirho.lhs_types_chirho,
                &instance_chirho.rhs_chirho,
            ) else {
                continue;
            };
            self.register_elaborated_type_family_chirho(
                instance_chirho.family_name_chirho.text_chirho().to_owned(),
                vec![equation_chirho],
            );
        }

        let Some(defaults_chirho) = self
            .assoc_type_defaults_chirho
            .get(class_name_chirho)
            .cloned()
        else {
            return;
        };
        let policy_chirho =
            super::ast_conversion_chirho::TypeConversionChirho::FamilyEquationChirho;
        for default_chirho in defaults_chirho {
            if instances_chirho.iter().any(|instance_chirho| {
                instance_chirho.family_name_chirho.text_chirho() == default_chirho.family_chirho
            }) {
                continue;
            }
            // Family parameter i names class parameter j through the family
            // declaration's positions, not the default equation's spellings.
            let declared_names_chirho = self
                .assoc_type_declared_params_chirho
                .get(&default_chirho.family_chirho)
                .unwrap_or(&default_chirho.family_params_chirho)
                .clone();
            let Some((generic_kinds_chirho, instance_kinds_chirho)) = self
                .kind_elaboration_chirho
                .as_ref()
                .and_then(|elaboration_chirho| {
                    Some((
                        elaboration_chirho
                            .applications_chirho
                            .get(&default_chirho.span_chirho)?
                            .clone(),
                        elaboration_chirho
                            .associated_defaults_chirho
                            .get(&(span_chirho, default_chirho.family_chirho.clone()))?
                            .clone(),
                    ))
                })
            else {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(300),
                        "associated family default has no checked instance kind arguments",
                        span_chirho,
                    ));
                continue;
            };
            let mut variables_chirho = HashMap::new();
            for parameter_chirho in &default_chirho.family_params_chirho {
                let identity_chirho = TyVarChirho(self.next_var_chirho);
                self.next_var_chirho += 1;
                variables_chirho.insert(parameter_chirho.clone(), identity_chirho);
            }
            let mut instance_variables_chirho = HashMap::new();
            let kind_inputs_chirho: Vec<_> = instance_kinds_chirho
                .iter()
                .map(|kind_chirho| {
                    self.kind_term_type_chirho(
                        kind_chirho,
                        &mut instance_variables_chirho,
                        &mut Vec::new(),
                    )
                })
                .collect();
            let generic_inputs_chirho: Vec<_> = generic_kinds_chirho
                .iter()
                .map(|kind_chirho| {
                    self.kind_term_type_chirho(kind_chirho, &mut variables_chirho, &mut Vec::new())
                })
                .collect();
            if generic_inputs_chirho.len() != kind_inputs_chirho.len()
                || generic_inputs_chirho
                    .iter()
                    .any(|input_chirho| !matches!(input_chirho, TyChirho::VarChirho(_)))
            {
                self.diagnostics_chirho
                    .push_chirho(DiagnosticChirho::error_with_code_chirho(
                        ErrorCodeChirho::error_chirho(300),
                        "associated family default kind binders do not match its instance contract",
                        span_chirho,
                    ));
                continue;
            }
            let mut bindings_chirho = SubstChirho::empty_chirho();
            for (generic_chirho, actual_chirho) in
                generic_inputs_chirho.iter().zip(&kind_inputs_chirho)
            {
                if let TyChirho::VarChirho(identity_chirho) = generic_chirho {
                    bindings_chirho.insert_chirho(*identity_chirho, actual_chirho.clone());
                }
            }
            let mut lhs_chirho = Vec::new();
            for (position_chirho, parameter_chirho) in
                default_chirho.family_params_chirho.iter().enumerate()
            {
                let declared_chirho = declared_names_chirho
                    .get(position_chirho)
                    .unwrap_or(parameter_chirho);
                match default_chirho
                    .class_params_chirho
                    .iter()
                    .position(|class_parameter_chirho| class_parameter_chirho == declared_chirho)
                {
                    Some(index_chirho) if index_chirho < types_chirho.len() => {
                        let head_chirho = self.ast_type_with_policy_chirho(
                            &types_chirho[index_chirho],
                            &mut instance_variables_chirho,
                            policy_chirho,
                        );
                        bindings_chirho
                            .insert_chirho(variables_chirho[parameter_chirho], head_chirho.clone());
                        lhs_chirho.push(head_chirho);
                    }
                    _ => lhs_chirho.push(TyChirho::VarChirho(variables_chirho[parameter_chirho])),
                }
            }
            let rhs_pattern_chirho = self.ast_type_with_policy_chirho(
                &default_chirho.rhs_chirho,
                &mut variables_chirho,
                policy_chirho,
            );
            let rhs_chirho = bindings_chirho.apply_ty_chirho(&rhs_pattern_chirho);
            self.register_elaborated_type_family_chirho(
                default_chirho.family_chirho,
                vec![TypeFamilyClauseChirho {
                    kind_inputs_chirho,
                    type_inputs_chirho: lhs_chirho,
                    result_chirho: rhs_chirho,
                }],
            );
        }
    }
}
