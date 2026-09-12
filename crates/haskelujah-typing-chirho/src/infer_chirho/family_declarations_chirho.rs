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
                    equations_chirho,
                    ..
                } => {
                    let equations_chirho = equations_chirho
                        .iter()
                        .filter_map(|equation_chirho| {
                            self.lower_local_family_equation_chirho(
                                &equation_chirho.lhs_types_chirho,
                                &equation_chirho.rhs_chirho,
                            )
                        })
                        .collect();
                    self.register_elaborated_type_family_chirho(
                        name_chirho.text_chirho().to_string(),
                        equations_chirho,
                    );
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

// Associated instances still use their enclosing class/instance parameter
// contract below. Their nominal indices need that scope's elaboration, not a
// top-level row checker applied without the enclosing instance binders.
impl InferCtxChirho {
    fn lower_associated_family_equation_chirho(
        &self,
        patterns_chirho: &[TypeChirho],
        result_chirho: &TypeChirho,
    ) -> (Vec<TyChirho>, TyChirho) {
        // Each equation binds its own pattern variables. Declaration-head names
        // neither bind differently named equation locals nor scope over the RHS.
        let parameters_chirho = collect_free_type_vars_from_ast_chirho(patterns_chirho);
        let mut converter_chirho = self.owned_synonym_converter_chirho();
        let patterns_chirho = patterns_chirho
            .iter()
            .map(|pattern_chirho| {
                converter_chirho.convert_chirho(pattern_chirho, &parameters_chirho)
            })
            .collect();
        let result_chirho = converter_chirho.convert_chirho(result_chirho, &parameters_chirho);
        (patterns_chirho, result_chirho)
    }

    /// Explicit equations override defaults; both retain this module's proven
    /// promoted identity while keeping their enclosing instance binding contract.
    /// Workflow: language-features-chirho/flat-type-syntax-chirho.
    pub(super) fn register_associated_family_equations_chirho(
        &mut self,
        class_name_chirho: &str,
        types_chirho: &[TypeChirho],
        instances_chirho: &[haskelujah_ast_chirho::decl_chirho::AssocTfInstanceChirho],
    ) {
        for instance_chirho in instances_chirho {
            let (lhs_chirho, rhs_chirho) = self.lower_associated_family_equation_chirho(
                &instance_chirho.lhs_types_chirho,
                &instance_chirho.rhs_chirho,
            );
            self.register_type_family_instance_chirho(
                instance_chirho.family_name_chirho.text_chirho().to_owned(),
                lhs_chirho,
                rhs_chirho,
            );
        }

        let Some(defaults_chirho) = self
            .assoc_type_defaults_chirho
            .get(class_name_chirho)
            .cloned()
        else {
            return;
        };
        let instance_variables_chirho = collect_free_type_vars_from_ast_chirho(types_chirho);
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
                .unwrap_or(&default_chirho.family_params_chirho);
            let mut converter_chirho = self.owned_synonym_converter_chirho();
            let mut bindings_chirho = HashMap::new();
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
                        let head_chirho = converter_chirho.convert_chirho(
                            &types_chirho[index_chirho],
                            &instance_variables_chirho,
                        );
                        bindings_chirho.insert(parameter_chirho.clone(), head_chirho.clone());
                        lhs_chirho.push(head_chirho);
                    }
                    _ => lhs_chirho.push(TyChirho::ForallVarChirho(parameter_chirho.clone())),
                }
            }
            let rhs_pattern_chirho = converter_chirho.convert_chirho(
                &default_chirho.rhs_chirho,
                &default_chirho.family_params_chirho,
            );
            let rhs_chirho = substitute_type_vars_chirho(&rhs_pattern_chirho, &bindings_chirho);
            self.register_type_family_instance_chirho(
                default_chirho.family_chirho,
                lhs_chirho,
                rhs_chirho,
            );
        }
    }
}

pub(super) fn collect_free_type_vars_from_ast_chirho(
    patterns_chirho: &[TypeChirho],
) -> Vec<String> {
    let mut variables_chirho = Vec::new();
    let mut seen_chirho = HashSet::new();
    let mut pending_chirho: Vec<_> = patterns_chirho.iter().rev().collect();
    while let Some(pattern_chirho) = pending_chirho.pop() {
        match pattern_chirho {
            TypeChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if seen_chirho.insert(text_chirho) {
                    variables_chirho.push(text_chirho.to_string());
                }
            }
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            }
            | TypeChirho::KindAppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => {
                pending_chirho.push(arg_chirho);
                pending_chirho.push(fun_chirho);
            }
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                ..
            } => {
                pending_chirho.push(result_chirho);
                pending_chirho.push(arg_chirho);
            }
            TypeChirho::ListChirho { element_chirho, .. } => {
                pending_chirho.push(element_chirho);
            }
            TypeChirho::TupleChirho {
                elements_chirho, ..
            }
            | TypeChirho::PromotedListChirho {
                elements_chirho, ..
            } => {
                pending_chirho.extend(elements_chirho.iter().rev());
            }
            TypeChirho::ParenChirho { inner_chirho, .. } => {
                pending_chirho.push(inner_chirho);
            }
            TypeChirho::KindAnnotChirho {
                type_chirho,
                kind_chirho,
                ..
            } => {
                pending_chirho.push(kind_chirho);
                pending_chirho.push(type_chirho);
            }
            TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::LitChirho { .. }
            | TypeChirho::WildcardChirho { .. }
            | TypeChirho::ForallChirho { .. }
            | TypeChirho::RequiredForallChirho { .. }
            | TypeChirho::QualChirho { .. } => {}
        }
    }
    variables_chirho
}
