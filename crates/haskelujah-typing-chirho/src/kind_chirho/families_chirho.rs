// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Kind-family elaboration consumes represented equations, not source-name guards.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::*;
use crate::families_chirho::FamilyTermChirho;
use crate::families_chirho::family_injectivity_chirho::validate_injectivity_chirho;
use haskelujah_ast_chirho::decl_chirho::{TypeFamilyEquationChirho, TypeFamilyResultChirho};
use haskelujah_ast_chirho::name_chirho::NameChirho;

#[derive(Debug, Clone)]
pub(super) struct KindFamilyChirho {
    pub(super) equations_chirho: Vec<(Vec<KindChirho>, KindChirho)>,
    pub(super) injective_chirho: Vec<usize>,
}

impl KindInferCtxChirho {
    pub(super) fn check_kind_family_chirho(
        &mut self,
        name_chirho: &NameChirho,
        binders_chirho: &[TyVarChirho],
        result_chirho: &TypeFamilyResultChirho,
        closed_chirho: bool,
        equations_chirho: &[TypeFamilyEquationChirho],
        span_chirho: SpanChirho,
    ) {
        let error_count_chirho = self.diagnostics_chirho.error_count_chirho();
        let mut injective_chirho = Vec::new();
        if let Some(annotation_chirho) = &result_chirho.injectivity_chirho {
            if result_chirho
                .binder_chirho
                .as_ref()
                .is_none_or(|binder_chirho| {
                    binder_chirho.text_chirho() != annotation_chirho.result_chirho.text_chirho()
                })
            {
                self.family_error_chirho(
                    "injectivity must name the declared result binder",
                    annotation_chirho.span_chirho,
                );
            }
            for parameter_chirho in &annotation_chirho.parameters_chirho {
                if let Some(index_chirho) = binders_chirho.iter().position(|binder_chirho| {
                    binder_chirho.text_chirho() == parameter_chirho.text_chirho()
                }) {
                    injective_chirho.push(index_chirho);
                } else if !self
                    .kind_var_cache_chirho
                    .contains_key(parameter_chirho.text_chirho())
                {
                    self.family_error_chirho(
                        "injectivity names an unbound family parameter",
                        parameter_chirho.span_chirho(),
                    );
                }
            }
        }
        if result_chirho
            .binder_chirho
            .as_ref()
            .is_some_and(|result_chirho| {
                binders_chirho
                    .iter()
                    .any(|binder_chirho| binder_chirho.text_chirho() == result_chirho.text_chirho())
                    || self
                        .kind_var_cache_chirho
                        .contains_key(result_chirho.text_chirho())
            })
        {
            self.family_error_chirho("family result binder is not fresh", span_chirho);
        }
        if !closed_chirho {
            return;
        }

        let canonical_chirho = self.canonical_kind_name_chirho(name_chirho);
        let family_kind_chirho = self.subst_chirho.apply_chirho(
            self.env_chirho
                .lookup_chirho(&canonical_chirho)
                .expect("family head prepared"),
        );
        // Written kind names can be matched implicitly by equations, unlike
        // unsolved classifier holes which equations still need to determine.
        let source_variables_chirho: std::collections::HashSet<_> =
            self.kind_var_cache_chirho.values().copied().collect();
        let quantified_chirho: Vec<_> = family_kind_chirho
            .free_vars_chirho()
            .into_iter()
            .filter(|variable_chirho| source_variables_chirho.contains(variable_chirho))
            .collect();
        let represented_inputs_chirho = quantified_chirho.is_empty()
            && binders_chirho.iter().all(TyVarChirho::is_visible_chirho);
        let family_binding_chirho = KindBindingChirho::PolyChirho(KindSchemeChirho {
            quantified_chirho,
            body_chirho: family_kind_chirho,
        });
        let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
        let mut rows_chirho = Vec::new();
        for equation_chirho in equations_chirho {
            let valid_patterns_chirho = equation_chirho.lhs_types_chirho.iter().fold(
                true,
                |valid_chirho, pattern_chirho| {
                    self.family_equation_term_is_valid_chirho(pattern_chirho, true) && valid_chirho
                },
            );
            let valid_result_chirho =
                self.family_equation_term_is_valid_chirho(&equation_chirho.rhs_chirho, false);
            if !valid_patterns_chirho || !valid_result_chirho {
                continue;
            }
            self.env_chirho.begin_scope_chirho();
            self.kind_var_cache_chirho.clear();
            let mut classifier_chirho = self.instantiate_binding_chirho(&family_binding_chirho);
            let mut patterns_chirho = Vec::new();
            let mut represented_chirho = true;
            for argument_chirho in &equation_chirho.lhs_types_chirho {
                let argument_kind_chirho = self.infer_type_kind_chirho(argument_chirho);
                let result_kind_chirho = self.fresh_kind_chirho();
                self.unify_chirho(
                    &classifier_chirho,
                    &KindChirho::arrow_chirho(argument_kind_chirho, result_kind_chirho.clone()),
                    "family equation argument",
                    argument_chirho.span_chirho(),
                );
                classifier_chirho = result_kind_chirho;
                if let Some(pattern_chirho) = self.family_term_chirho(argument_chirho) {
                    patterns_chirho.push(pattern_chirho);
                } else {
                    represented_chirho = false;
                }
            }
            let rhs_kind_chirho = self.infer_type_kind_chirho(&equation_chirho.rhs_chirho);
            self.unify_chirho(
                &classifier_chirho,
                &rhs_kind_chirho,
                "family equation result",
                equation_chirho.rhs_chirho.span_chirho(),
            );
            if let Some(rhs_chirho) = self.family_term_chirho(&equation_chirho.rhs_chirho)
                && represented_chirho
            {
                rows_chirho.push((patterns_chirho, rhs_chirho));
            }
            self.env_chirho.end_scope_chirho();
        }
        self.kind_var_cache_chirho = outer_names_chirho;
        if !represented_inputs_chirho
            || rows_chirho.len() != equations_chirho.len()
            || self.diagnostics_chirho.error_count_chirho() != error_count_chirho
        {
            // Hidden matching inputs and unsupported terms are NOT erased into
            // a visible-only row that might pick the wrong equation.
            return;
        }
        if result_chirho.injectivity_chirho.is_some() {
            match validate_injectivity_chirho(&rows_chirho, &injective_chirho, &|head_chirho| {
                self.kind_family_names_chirho.contains(head_chirho)
            }) {
                Ok(true) => {}
                Ok(false) => injective_chirho.clear(),
                Err(message_chirho) => {
                    self.family_error_chirho(message_chirho, span_chirho);
                    injective_chirho.clear();
                }
            }
        }
        self.kind_families_chirho.insert(
            canonical_chirho,
            KindFamilyChirho {
                equations_chirho: rows_chirho,
                injective_chirho,
            },
        );
    }

    fn family_error_chirho(&mut self, message_chirho: &str, span_chirho: SpanChirho) {
        self.diagnostics_chirho
            .push_chirho(DiagnosticChirho::error_with_code_chirho(
                ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                message_chirho,
                span_chirho,
            ));
    }

    /// Family equations are monotypes on both sides. An opaque forall is not
    /// a pattern with zero variables or a substitutable RHS.
    fn family_equation_term_is_valid_chirho(
        &mut self,
        term_chirho: &TypeChirho,
        pattern_chirho: bool,
    ) -> bool {
        let mut pending_chirho = vec![term_chirho];
        while let Some(term_chirho) = pending_chirho.pop() {
            if let Some((name_chirho, expanded_chirho)) =
                self.expand_type_kind_synonym_once_chirho(term_chirho)
                && !self
                    .expanding_type_kind_synonyms_chirho
                    .contains(&name_chirho)
            {
                self.expanding_type_kind_synonyms_chirho.push(name_chirho);
                let valid_chirho =
                    self.family_equation_term_is_valid_chirho(&expanded_chirho, pattern_chirho);
                self.expanding_type_kind_synonyms_chirho.pop();
                if !valid_chirho {
                    return false;
                }
                continue;
            }
            match term_chirho {
                TypeChirho::ForallChirho { .. }
                | TypeChirho::RequiredForallChirho { .. }
                | TypeChirho::QualChirho { .. } => {
                    self.family_error_chirho(
                        "illegal polymorphic type in family equation",
                        term_chirho.span_chirho(),
                    );
                    return false;
                }
                TypeChirho::ConChirho(name_chirho)
                    if pattern_chirho
                        && self
                            .kind_family_names_chirho
                            .contains(&self.canonical_kind_name_chirho(name_chirho)) =>
                {
                    self.family_error_chirho(
                        "illegal type family application in an equation pattern",
                        term_chirho.span_chirho(),
                    );
                    return false;
                }
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    ..
                } => pending_chirho.extend([fun_chirho.as_ref(), arg_chirho.as_ref()]),
                TypeChirho::FunChirho {
                    arg_chirho,
                    result_chirho,
                    ..
                } => pending_chirho.extend([arg_chirho.as_ref(), result_chirho.as_ref()]),
                TypeChirho::ParenChirho { inner_chirho, .. } => pending_chirho.push(inner_chirho),
                TypeChirho::ListChirho { element_chirho, .. } => {
                    pending_chirho.push(element_chirho)
                }
                TypeChirho::TupleChirho {
                    elements_chirho, ..
                }
                | TypeChirho::PromotedListChirho {
                    elements_chirho, ..
                } => pending_chirho.extend(elements_chirho),
                _ => {}
            }
        }
        true
    }

    fn family_term_chirho(&mut self, ty_chirho: &TypeChirho) -> Option<KindChirho> {
        // A synonym is transparent to injectivity. Keeping `Erase a` as a
        // nominal application would falsely prove that its result determines a.
        if let Some((name_chirho, expanded_chirho)) =
            self.expand_type_kind_synonym_once_chirho(ty_chirho)
        {
            if self
                .expanding_type_kind_synonyms_chirho
                .contains(&name_chirho)
            {
                return None;
            }
            self.expanding_type_kind_synonyms_chirho.push(name_chirho);
            let term_chirho = self.family_term_chirho(&expanded_chirho);
            self.expanding_type_kind_synonyms_chirho.pop();
            return term_chirho;
        }
        match ty_chirho {
            TypeChirho::VarChirho(_)
            | TypeChirho::ConChirho(_)
            | TypeChirho::PromotedConChirho { .. }
            | TypeChirho::LitChirho { .. } => Some(self.interpret_kind_term_chirho(ty_chirho)),
            TypeChirho::AppChirho {
                fun_chirho,
                arg_chirho,
                ..
            } => Some(KindChirho::app_chirho(
                self.family_term_chirho(fun_chirho)?,
                self.family_term_chirho(arg_chirho)?,
            )),
            TypeChirho::FunChirho {
                arg_chirho,
                result_chirho,
                mult_chirho,
                ..
            } if mult_chirho.is_none() => Some(KindChirho::arrow_chirho(
                self.family_term_chirho(arg_chirho)?,
                self.family_term_chirho(result_chirho)?,
            )),
            TypeChirho::ListChirho { element_chirho, .. } => Some(KindChirho::app_chirho(
                KindChirho::ConChirho("[]".into()),
                self.family_term_chirho(element_chirho)?,
            )),
            TypeChirho::ParenChirho { inner_chirho, .. } => self.family_term_chirho(inner_chirho),
            _ => None,
        }
    }
}

impl FamilyTermChirho for KindChirho {
    type VariableChirho = KindVarChirho;
    fn variable_chirho(&self) -> Option<KindVarChirho> {
        if let Self::VarChirho(variable_chirho) = self {
            Some(*variable_chirho)
        } else {
            None
        }
    }
    fn unknown_chirho(&self) -> bool {
        matches!(
            self,
            Self::VarChirho(_) | Self::RigidChirho(_) | Self::BoundChirho(_)
        )
    }
    fn head_name_chirho(&self) -> Option<&str> {
        match self {
            Self::ConChirho(name_chirho) => Some(name_chirho),
            Self::AppChirho(fun_chirho, _) => fun_chirho.head_name_chirho(),
            _ => None,
        }
    }
    fn parts_chirho(&self) -> Option<(&'static str, Vec<&Self>)> {
        match self {
            Self::AppChirho(fun_chirho, argument_chirho) => {
                Some(("application_chirho", vec![fun_chirho, argument_chirho]))
            }
            Self::ArrowChirho(argument_chirho, result_chirho) => {
                Some(("function_chirho", vec![argument_chirho, result_chirho]))
            }
            Self::DependentChirho {
                argument_chirho,
                result_chirho,
            } => Some(("dependent_chirho", vec![argument_chirho, result_chirho])),
            _ => None,
        }
    }
    fn map_children_chirho(&self, map_chirho: &mut impl FnMut(&Self) -> Self) -> Self {
        match self {
            Self::AppChirho(fun_chirho, argument_chirho) => {
                Self::app_chirho(map_chirho(fun_chirho), map_chirho(argument_chirho))
            }
            Self::ArrowChirho(argument_chirho, result_chirho) => {
                Self::arrow_chirho(map_chirho(argument_chirho), map_chirho(result_chirho))
            }
            Self::DependentChirho {
                argument_chirho,
                result_chirho,
            } => Self::DependentChirho {
                argument_chirho: Box::new(map_chirho(argument_chirho)),
                result_chirho: Box::new(map_chirho(result_chirho)),
            },
            _ => self.clone(),
        }
    }
    fn application_chirho(fun_chirho: Self, argument_chirho: Self) -> Self {
        Self::app_chirho(fun_chirho, argument_chirho)
    }
}
