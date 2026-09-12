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

struct CheckedKindFamilyRowChirho {
    patterns_chirho: Vec<KindChirho>,
    result_chirho: Option<KindChirho>,
    hidden_chirho: Vec<KindChirho>,
    represented_chirho: bool,
}

impl KindInferCtxChirho {
    /// Open rows consume already-published LOCAL family contracts. Imported
    /// family kinds and associated rows need their own authority/scope; neither
    /// is manufactured by this pass. Workflow: declaration-kinds-chirho.
    pub(super) fn check_open_family_equations_chirho(&mut self, module_chirho: &ModuleChirho) {
        for declaration_chirho in &module_chirho.decls_chirho {
            let DeclChirho::TypeFamilyInstanceDeclChirho {
                family_name_chirho,
                lhs_types_chirho,
                rhs_chirho,
                ..
            } = declaration_chirho
            else {
                continue;
            };
            let name_chirho = self.canonical_kind_name_chirho(family_name_chirho);
            if !self.kind_family_names_chirho.contains(&name_chirho) {
                continue;
            }
            let Some(KindBindingChirho::PolyChirho(scheme_chirho)) =
                self.env_chirho.lookup_binding_chirho(&name_chirho).cloned()
            else {
                continue;
            };
            self.check_family_equation_kinds_chirho(
                &name_chirho,
                &scheme_chirho,
                &[],
                lhs_types_chirho,
                rhs_chirho,
            );
        }
    }

    /// One equation owns its pattern variables; a same-spelled declaration-head
    /// binder does not bind them. Both open and closed rows record nominal
    /// occurrence arguments here, before the type-level equation is converted.
    fn check_family_equation_kinds_chirho(
        &mut self,
        name_chirho: &str,
        scheme_chirho: &KindSchemeChirho,
        binders_chirho: &[TyVarChirho],
        arguments_chirho: &[TypeChirho],
        result_chirho: &TypeChirho,
    ) -> Option<CheckedKindFamilyRowChirho> {
        let valid_patterns_chirho =
            arguments_chirho
                .iter()
                .fold(true, |valid_chirho, argument_chirho| {
                    self.family_equation_term_is_valid_chirho(argument_chirho, true) && valid_chirho
                });
        let valid_result_chirho = self.family_equation_term_is_valid_chirho(result_chirho, false);
        if !valid_patterns_chirho || !valid_result_chirho {
            return None;
        }

        self.env_chirho.begin_scope_chirho();
        let outer_names_chirho = std::mem::take(&mut self.kind_var_cache_chirho);
        for name_chirho in binders_chirho
            .iter()
            .map(|binder_chirho| binder_chirho.text_chirho())
            .chain(outer_names_chirho.keys().map(String::as_str))
        {
            self.env_chirho.hide_chirho(name_chirho);
        }
        let (mut classifier_chirho, hidden_chirho) =
            self.open_kind_scheme_parts_chirho(scheme_chirho, false);
        let mut patterns_chirho = Vec::new();
        let mut represented_chirho = true;
        for argument_chirho in arguments_chirho {
            let argument_kind_chirho = self.infer_type_kind_chirho(argument_chirho);
            let pattern_chirho = self.family_term_chirho(argument_chirho);
            classifier_chirho = self.consume_kind_argument_chirho(
                classifier_chirho,
                &argument_kind_chirho,
                pattern_chirho.as_ref(),
                argument_chirho.span_chirho(),
                "family equation argument",
            );
            if let Some(pattern_chirho) = pattern_chirho {
                patterns_chirho.push(pattern_chirho);
            } else {
                represented_chirho = false;
            }
        }
        let result_kind_chirho = self.infer_type_kind_chirho(result_chirho);
        self.unify_chirho(
            &classifier_chirho,
            &result_kind_chirho,
            "family equation result",
            result_chirho.span_chirho(),
        );
        self.record_equation_kind_inputs_chirho(
            name_chirho,
            scheme_chirho,
            hidden_chirho.clone(),
            result_chirho.span_chirho(),
        );
        let result_chirho = self.family_term_chirho(result_chirho);
        self.env_chirho.end_scope_chirho();
        self.kind_var_cache_chirho = outer_names_chirho;
        Some(CheckedKindFamilyRowChirho {
            patterns_chirho,
            result_chirho,
            hidden_chirho,
            represented_chirho,
        })
    }

    /// Process a type family declaration to determine the kind of the family.
    pub(super) fn infer_type_family_decl_kind_chirho(
        &mut self,
        name_chirho: &str,
        type_vars_chirho: &[TyVarChirho],
        kind_sig_chirho: Option<&DeclKindSigChirho>,
        span_chirho: SpanChirho,
        poly_kinds_enabled_chirho: bool,
    ) {
        if let Some(standalone_chirho) =
            kind_sig_chirho.and_then(DeclKindSigChirho::standalone_chirho)
        {
            let head_chirho = self.prepare_kind_head_chirho(
                type_vars_chirho,
                Some(standalone_chirho),
                span_chirho,
                "type family declaration signature",
            );
            if let Some(result_chirho) = kind_sig_chirho.and_then(DeclKindSigChirho::result_chirho)
            {
                let (result_kind_chirho, written_chirho, _) =
                    self.elaborate_inline_kind_chirho(result_chirho);
                self.unify_chirho(
                    &head_chirho
                        .complete_tail_chirho
                        .expect("complete head has a tail"),
                    &result_kind_chirho,
                    "type family declaration signature",
                    span_chirho,
                );
                self.pending_written_kinds_chirho.extend(
                    written_chirho
                        .into_iter()
                        .map(|variable_chirho| (variable_chirho, span_chirho)),
                );
            }
            self.pending_written_kinds_chirho.extend(
                head_chirho
                    .written_variables_chirho
                    .into_iter()
                    .map(|variable_chirho| (variable_chirho, span_chirho)),
            );
            let binding_chirho = KindBindingChirho::PolyChirho(
                head_chirho
                    .complete_scheme_chirho
                    .expect("complete signature has a scheme"),
            );
            if let Some(existing_chirho) =
                self.env_chirho.lookup_binding_chirho(name_chirho).cloned()
            {
                let existing_chirho = self.instantiate_binding_chirho(&existing_chirho);
                let kind_chirho = self.instantiate_binding_chirho(&binding_chirho);
                self.unify_chirho(
                    &existing_chirho,
                    &kind_chirho,
                    "type family declaration",
                    span_chirho,
                );
            }
            self.env_chirho
                .bind_entry_chirho(name_chirho.to_owned(), binding_chirho);
            return;
        }
        let result_kind_chirho = kind_sig_chirho.and_then(DeclKindSigChirho::result_chirho);
        let head_chirho = self.prepare_kind_head_chirho(
            type_vars_chirho,
            None,
            span_chirho,
            "type family declaration",
        );

        let result_kind_chirho = result_kind_chirho
            .map(|kind_ty_chirho| self.type_to_kind_chirho(kind_ty_chirho))
            .unwrap_or_else(|| {
                if poly_kinds_enabled_chirho {
                    self.fresh_kind_chirho()
                } else {
                    KindChirho::StarChirho
                }
            });
        let kind_chirho =
            Self::compose_kind_head_chirho(head_chirho.parameters_chirho, result_kind_chirho);

        if let Some(existing_chirho) = self.env_chirho.lookup_chirho(name_chirho) {
            let existing_chirho = existing_chirho.clone();
            self.unify_chirho(
                &existing_chirho,
                &kind_chirho,
                "type family declaration",
                span_chirho,
            );
        }
        self.env_chirho
            .bind_chirho(name_chirho.to_string(), kind_chirho);
    }

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
        let scheme_chirho = match self
            .env_chirho
            .lookup_binding_chirho(&canonical_chirho)
            .expect("family head prepared")
        {
            // A complete signature owns its quantifiers. Header names have
            // independently scoped identities and cannot reconstruct them.
            KindBindingChirho::PolyChirho(scheme_chirho) => scheme_chirho.clone(),
            KindBindingChirho::MonoChirho(kind_chirho) => {
                let body_chirho = self.subst_chirho.apply_chirho(kind_chirho);
                // Written kind names may be matched by equations, unlike
                // classifier holes which the equation bodies must determine.
                let source_variables_chirho: std::collections::HashSet<_> =
                    self.kind_var_cache_chirho.values().copied().collect();
                let quantified_chirho = body_chirho
                    .free_vars_chirho()
                    .into_iter()
                    .filter(|variable_chirho| source_variables_chirho.contains(variable_chirho))
                    .collect();
                self.bind_source_kind_scheme_chirho(
                    body_chirho,
                    quantified_chirho,
                    &source_variables_chirho,
                )
            }
        };
        let mut represented_inputs_chirho =
            binders_chirho.iter().all(TyVarChirho::is_visible_chirho);
        let mut rows_chirho = Vec::new();
        for equation_chirho in equations_chirho {
            let Some(row_chirho) = self.check_family_equation_kinds_chirho(
                &canonical_chirho,
                &scheme_chirho,
                binders_chirho,
                &equation_chirho.lhs_types_chirho,
                &equation_chirho.rhs_chirho,
            ) else {
                continue;
            };
            // A visible-only row is sound only when checking the WHOLE row
            // imposes no condition on any hidden input.
            let mut hidden_variables_chirho = std::collections::HashSet::new();
            if !row_chirho.hidden_chirho.is_empty() {
                represented_inputs_chirho &= row_chirho
                    .patterns_chirho
                    .iter()
                    .all(|pattern_chirho| matches!(pattern_chirho, KindChirho::VarChirho(_)));
            }
            for argument_chirho in &row_chirho.hidden_chirho {
                if let KindChirho::VarChirho(variable_chirho) =
                    self.subst_chirho.apply_chirho(argument_chirho)
                {
                    represented_inputs_chirho &= hidden_variables_chirho.insert(variable_chirho);
                } else {
                    represented_inputs_chirho = false;
                }
            }
            if let Some(rhs_chirho) = row_chirho.result_chirho
                && row_chirho.represented_chirho
            {
                let close_chirho =
                    |term_chirho: &KindChirho| self.subst_chirho.apply_chirho(term_chirho);
                let patterns_chirho: Vec<_> = row_chirho
                    .patterns_chirho
                    .iter()
                    .map(close_chirho)
                    .collect();
                let rhs_chirho = close_chirho(&rhs_chirho);
                let bound_chirho: std::collections::HashSet<_> = patterns_chirho
                    .iter()
                    .flat_map(KindChirho::free_vars_chirho)
                    .collect();
                represented_inputs_chirho &= rhs_chirho
                    .free_vars_chirho()
                    .iter()
                    .all(|variable_chirho| bound_chirho.contains(variable_chirho));
                rows_chirho.push((patterns_chirho, rhs_chirho));
            }
        }
        if !represented_inputs_chirho
            || rows_chirho.len() != equations_chirho.len()
            || self.diagnostics_chirho.error_count_chirho() != error_count_chirho
        {
            // Hidden matching inputs and unsupported terms are NOT erased into
            // a visible-only row that might pick the wrong equation.
            return;
        }
        if result_chirho.injectivity_chirho.is_some() {
            match validate_injectivity_chirho(
                &rows_chirho,
                &injective_chirho,
                &|head_chirho| self.kind_family_names_chirho.contains(head_chirho),
                &|head_chirho, arity_chirho| {
                    let family_chirho = self.kind_families_chirho.get(head_chirho)?;
                    (family_chirho.equations_chirho.first()?.0.len() == arity_chirho)
                        .then_some(family_chirho.injective_chirho.as_slice())
                },
            ) {
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
                }
                | TypeChirho::KindAppChirho {
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
                // The classifier is checked as a kind, not an equation
                // pattern: families in a written classifier are not nested
                // family applications in the matching term itself.
                TypeChirho::KindAnnotChirho { type_chirho, .. } => pending_chirho.push(type_chirho),
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

    pub(super) fn family_term_chirho(&mut self, ty_chirho: &TypeChirho) -> Option<KindChirho> {
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
            // Every anonymous pattern binds independently; it is not Type and
            // must not share a textual key with another underscore.
            TypeChirho::WildcardChirho { span_chirho } => {
                Some(self.source_wildcard_term_chirho(*span_chirho))
            }
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
            TypeChirho::TupleChirho {
                elements_chirho, ..
            } => Some(KindChirho::tuple_chirho(
                elements_chirho
                    .iter()
                    .map(|element_chirho| self.family_term_chirho(element_chirho))
                    .collect::<Option<Vec<_>>>()?,
            )),
            TypeChirho::ParenChirho { inner_chirho, .. } => self.family_term_chirho(inner_chirho),
            TypeChirho::KindAnnotChirho { type_chirho, .. } => self.family_term_chirho(type_chirho),
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
            Self::AppChirho(fun_chirho, _) | Self::KindAppChirho(fun_chirho, _) => {
                fun_chirho.head_name_chirho()
            }
            _ => None,
        }
    }
    fn parts_chirho(&self) -> Option<(&'static str, Vec<&Self>)> {
        match self {
            Self::KindAppChirho(fun_chirho, argument_chirho) => {
                Some(("kind_application_chirho", vec![fun_chirho, argument_chirho]))
            }
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
    fn application_parts_chirho(&self) -> Option<(&Self, &Self)> {
        if let Self::AppChirho(fun_chirho, argument_chirho) = self {
            Some((fun_chirho, argument_chirho))
        } else {
            None
        }
    }
    fn map_children_chirho(&self, map_chirho: &mut impl FnMut(&Self) -> Self) -> Self {
        match self {
            Self::KindAppChirho(fun_chirho, argument_chirho) => Self::KindAppChirho(
                Box::new(map_chirho(fun_chirho)),
                Box::new(map_chirho(argument_chirho)),
            ),
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
