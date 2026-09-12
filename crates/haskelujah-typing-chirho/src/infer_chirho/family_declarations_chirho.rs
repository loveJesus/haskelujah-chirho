// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equation-local binding and registration for open and closed type families.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;

impl InferCtxChirho {
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
                    self.register_type_family_chirho(
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
                    let Some((patterns_chirho, result_chirho)) =
                        self.lower_local_family_equation_chirho(lhs_types_chirho, rhs_chirho)
                    else {
                        continue;
                    };
                    self.register_type_family_instance_chirho(
                        family_name_chirho.text_chirho().to_string(),
                        patterns_chirho,
                        result_chirho,
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
    ) -> Option<(Vec<TyChirho>, TyChirho)> {
        let mut variables_chirho = HashMap::new();
        let policy_chirho =
            super::ast_conversion_chirho::TypeConversionChirho::FamilyEquationChirho;
        let patterns_chirho: Vec<_> = patterns_chirho
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
        let bound_chirho: HashSet<_> = patterns_chirho
            .iter()
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
        Some((patterns_chirho, body_chirho))
    }
}

// Associated instances still use their enclosing class/instance parameter
// contract below. Their nominal indices need that scope's elaboration, not a
// top-level row checker applied without the enclosing instance binders.
pub(super) fn lower_family_equation_chirho(
    patterns_chirho: &[TypeChirho],
    result_chirho: &TypeChirho,
) -> (Vec<TyChirho>, TyChirho) {
    // Each equation binds its own pattern variables. Declaration-head names
    // neither bind differently named equation locals nor scope over the RHS.
    let parameters_chirho = collect_free_type_vars_from_ast_chirho(patterns_chirho);
    let mut converter_chirho = super::ast_conversion_chirho::SynonymTypeConverterChirho::default();
    let patterns_chirho = patterns_chirho
        .iter()
        .map(|pattern_chirho| converter_chirho.convert_chirho(pattern_chirho, &parameters_chirho))
        .collect();
    let result_chirho = converter_chirho.convert_chirho(result_chirho, &parameters_chirho);
    (patterns_chirho, result_chirho)
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
