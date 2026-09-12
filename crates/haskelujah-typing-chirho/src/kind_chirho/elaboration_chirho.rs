// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Solved kind arguments crossing into type inference. Workflow: declaration-kinds-chirho.
use super::*;
use std::collections::HashSet;

pub(super) struct PendingKindApplicationChirho {
    head_chirho: String,
    namespace_chirho: KindHeadNamespaceChirho,
    // Already-polymorphic binders have fresh occurrence arguments. A binder
    // generalized only after this occurrence retains the group's own identity.
    arguments_chirho: HashMap<KindVarChirho, KindChirho>,
}

#[derive(Clone, Copy)]
pub(super) enum KindHeadNamespaceChirho {
    TypeChirho,
    PromotedChirho,
}

#[derive(Clone, Debug)]
pub(crate) struct ElaboratedKindBinderChirho {
    pub(crate) identity_chirho: KindVarChirho,
    pub(crate) name_chirho: Option<String>,
    pub(crate) specified_chirho: bool,
}

impl ElaboratedKindBinderChirho {
    pub(crate) fn identity_key_chirho(&self) -> String {
        format!("$kind_chirho_{}", self.identity_chirho.0)
    }

    pub(crate) fn parameter_name_chirho(&self) -> String {
        self.name_chirho
            .clone()
            .unwrap_or_else(|| self.identity_key_chirho())
    }
}

/// Local nominal, synonym and family contracts share solved indices. Equation
/// matching inputs use a separate occurrence map: an RHS's own head can have a
/// different kind contract. Missing imported contracts authorize no invention.
#[derive(Clone, Debug, Default)]
pub struct KindElaborationChirho {
    pub(crate) applications_chirho: HashMap<SpanChirho, Vec<KindChirho>>,
    pub(crate) wildcard_terms_chirho: HashMap<SpanChirho, KindChirho>,
    pub(crate) nominal_heads_chirho: HashMap<String, Vec<ElaboratedKindBinderChirho>>,
    pub(crate) synonym_heads_chirho: HashMap<String, Vec<ElaboratedKindBinderChirho>>,
    pub(crate) family_heads_chirho: HashMap<String, Vec<ElaboratedKindBinderChirho>>,
    pub(crate) promoted_heads_chirho: HashMap<String, Vec<ElaboratedKindBinderChirho>>,
    pub(crate) equation_inputs_chirho: HashMap<SpanChirho, Vec<KindChirho>>,
    pub(crate) source_names_chirho: HashMap<KindVarChirho, String>,
}

impl KindInferCtxChirho {
    /// An anonymous source pattern still owns an identity. Its inferred index
    /// may occur inside the RHS constructor's classifier; freshening the LHS a
    /// second time during conversion would make that valid RHS look unbound.
    /// This map is bounded by source occurrences and retired with the module.
    pub(super) fn source_wildcard_term_chirho(&mut self, span_chirho: SpanChirho) -> KindChirho {
        if span_chirho == SpanChirho::DUMMY_CHIRHO {
            return self.fresh_kind_chirho();
        }
        if let Some(term_chirho) = self.kind_wildcard_terms_chirho.get(&span_chirho) {
            return self.subst_chirho.apply_chirho(term_chirho);
        }
        let term_chirho = self.fresh_kind_chirho();
        self.kind_wildcard_terms_chirho
            .insert(span_chirho, term_chirho.clone());
        term_chirho
    }

    pub(super) fn record_equation_kind_inputs_chirho(
        &mut self,
        name_chirho: &str,
        scheme_chirho: &KindSchemeChirho,
        arguments_chirho: Vec<KindChirho>,
        result_span_chirho: SpanChirho,
    ) {
        if result_span_chirho == SpanChirho::DUMMY_CHIRHO {
            return;
        }
        self.kind_equation_inputs_chirho.insert(
            result_span_chirho,
            PendingKindApplicationChirho {
                head_chirho: name_chirho.to_owned(),
                namespace_chirho: KindHeadNamespaceChirho::TypeChirho,
                arguments_chirho: scheme_chirho
                    .quantified_chirho
                    .iter()
                    .copied()
                    .zip(arguments_chirho)
                    .collect(),
            },
        );
    }

    pub(super) fn record_kind_application_chirho(
        &mut self,
        name_chirho: &str,
        namespace_chirho: KindHeadNamespaceChirho,
        binding_chirho: &KindBindingChirho,
        arguments_chirho: Vec<KindChirho>,
        span_chirho: SpanChirho,
    ) {
        let known_chirho = match namespace_chirho {
            KindHeadNamespaceChirho::TypeChirho => {
                self.local_kind_decl_names_chirho.contains(name_chirho)
                    || self.imported_kind_shapes_chirho.contains_key(name_chirho)
            }
            KindHeadNamespaceChirho::PromotedChirho => self
                .env_chirho
                .lookup_promoted_binding_chirho(name_chirho)
                .is_some(),
        };
        if span_chirho == SpanChirho::DUMMY_CHIRHO || !known_chirho {
            return;
        }
        let arguments_chirho = match binding_chirho {
            KindBindingChirho::MonoChirho(_) => HashMap::new(),
            KindBindingChirho::PolyChirho(scheme_chirho) => scheme_chirho
                .quantified_chirho
                .iter()
                .copied()
                .zip(arguments_chirho)
                .collect(),
        };
        self.kind_applications_chirho.insert(
            span_chirho,
            PendingKindApplicationChirho {
                head_chirho: name_chirho.to_owned(),
                namespace_chirho,
                arguments_chirho,
            },
        );
    }

    pub(super) fn finish_kind_elaboration_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) -> KindElaborationChirho {
        let mut nominal_heads_chirho = HashMap::new();
        let mut synonym_heads_chirho = HashMap::new();
        let mut family_heads_chirho = HashMap::new();
        let mut promoted_heads_chirho = HashMap::new();
        for occurrence_chirho in self.kind_applications_chirho.values() {
            if !matches!(
                occurrence_chirho.namespace_chirho,
                KindHeadNamespaceChirho::PromotedChirho
            ) || promoted_heads_chirho.contains_key(&occurrence_chirho.head_chirho)
            {
                continue;
            }
            if let Some(KindBindingChirho::PolyChirho(scheme_chirho)) = self
                .env_chirho
                .lookup_promoted_binding_chirho(&occurrence_chirho.head_chirho)
            {
                promoted_heads_chirho.insert(
                    occurrence_chirho.head_chirho.clone(),
                    scheme_chirho
                        .quantified_chirho
                        .iter()
                        .enumerate()
                        .map(
                            |(index_chirho, identity_chirho)| ElaboratedKindBinderChirho {
                                identity_chirho: *identity_chirho,
                                name_chirho: scheme_chirho.source_names_chirho[index_chirho]
                                    .clone(),
                                specified_chirho: scheme_chirho
                                    .specified_chirho
                                    .contains(identity_chirho),
                            },
                        )
                        .collect(),
                );
            }
        }
        for (name_chirho, shape_chirho) in &self.imported_kind_shapes_chirho {
            if self.local_kind_decl_names_chirho.contains(name_chirho) {
                continue;
            }
            let Some(KindBindingChirho::PolyChirho(scheme_chirho)) =
                self.env_chirho.lookup_binding_chirho(name_chirho)
            else {
                continue;
            };
            let heads_chirho = match shape_chirho {
                imports_chirho::KindHeadShapeChirho::NominalChirho => &mut nominal_heads_chirho,
                imports_chirho::KindHeadShapeChirho::SynonymChirho => &mut synonym_heads_chirho,
                imports_chirho::KindHeadShapeChirho::FamilyChirho => &mut family_heads_chirho,
                imports_chirho::KindHeadShapeChirho::ClassChirho => continue,
            };
            if !scheme_chirho.quantified_chirho.is_empty() {
                heads_chirho.insert(
                    name_chirho.clone(),
                    scheme_chirho
                        .quantified_chirho
                        .iter()
                        .enumerate()
                        .map(
                            |(index_chirho, identity_chirho)| ElaboratedKindBinderChirho {
                                identity_chirho: *identity_chirho,
                                name_chirho: scheme_chirho.source_names_chirho[index_chirho]
                                    .clone(),
                                specified_chirho: scheme_chirho
                                    .specified_chirho
                                    .contains(identity_chirho),
                            },
                        )
                        .collect(),
                );
            }
        }
        for declaration_chirho in &module_chirho.decls_chirho {
            let (name_chirho, parameters_chirho, heads_chirho) = match declaration_chirho {
                DeclChirho::DataDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    ..
                }
                | DeclChirho::NewtypeDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    ..
                } => (
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    &mut nominal_heads_chirho,
                ),
                DeclChirho::TypeAliasDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    ..
                } => (
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    &mut synonym_heads_chirho,
                ),
                DeclChirho::TypeFamilyDeclChirho {
                    name_chirho,
                    type_vars_chirho,
                    ..
                } => (
                    name_chirho.text_chirho(),
                    type_vars_chirho,
                    &mut family_heads_chirho,
                ),
                _ => continue,
            };
            let Some(KindBindingChirho::PolyChirho(scheme_chirho)) =
                self.env_chirho.lookup_binding_chirho(name_chirho)
            else {
                continue;
            };
            let mut binders_chirho: Vec<_> = scheme_chirho
                .quantified_chirho
                .iter()
                .enumerate()
                .map(
                    |(index_chirho, identity_chirho)| ElaboratedKindBinderChirho {
                        identity_chirho: *identity_chirho,
                        name_chirho: scheme_chirho.source_names_chirho[index_chirho].clone(),
                        specified_chirho: scheme_chirho.specified_chirho.contains(identity_chirho),
                    },
                )
                .collect();
            // Explicit declaration-head binders rename the signature's specified
            // quantifiers; the signature spelling does not scope over the body.
            let mut next_chirho = 0;
            for parameter_chirho in parameters_chirho
                .iter()
                .filter(|parameter_chirho| !parameter_chirho.is_visible_chirho())
            {
                while next_chirho < binders_chirho.len()
                    && !binders_chirho[next_chirho].specified_chirho
                {
                    next_chirho += 1;
                }
                if let Some(binder_chirho) = binders_chirho.get_mut(next_chirho) {
                    binder_chirho.name_chirho = Some(parameter_chirho.text_chirho().to_owned());
                    next_chirho += 1;
                }
            }
            if !binders_chirho.is_empty() {
                if let Some(module_chirho) = &self.local_kind_module_chirho {
                    heads_chirho.insert(
                        format!("{module_chirho}.{name_chirho}"),
                        binders_chirho.clone(),
                    );
                }
                heads_chirho.insert(name_chirho.to_owned(), binders_chirho);
            }
        }
        let finish_occurrences_chirho = |occurrences_chirho: &HashMap<
            SpanChirho,
            PendingKindApplicationChirho,
        >| {
            let mut invalid_chirho = Vec::new();
            let finished_chirho = occurrences_chirho
                .iter()
                .filter_map(|(span_chirho, occurrence_chirho)| {
                    let binding_chirho = match occurrence_chirho.namespace_chirho {
                        KindHeadNamespaceChirho::TypeChirho => self
                            .env_chirho
                            .lookup_binding_chirho(&occurrence_chirho.head_chirho),
                        KindHeadNamespaceChirho::PromotedChirho => self
                            .env_chirho
                            .lookup_promoted_binding_chirho(&occurrence_chirho.head_chirho),
                    };
                    let Some(KindBindingChirho::PolyChirho(scheme_chirho)) = binding_chirho else {
                        return None;
                    };
                    // Publishing an inferred head can replace a written
                    // metavariable with its rigid representative. Re-key
                    // captures through that identity substitution, never
                    // through source spelling or equation order.
                    let quantified_chirho: HashSet<_> =
                        scheme_chirho.quantified_chirho.iter().copied().collect();
                    let mut arguments_chirho = HashMap::new();
                    for (identity_chirho, argument_chirho) in &occurrence_chirho.arguments_chirho {
                        let resolved_chirho = if quantified_chirho.contains(identity_chirho) {
                            *identity_chirho
                        } else {
                            match self
                                .subst_chirho
                                .apply_chirho(&KindChirho::VarChirho(*identity_chirho))
                            {
                                KindChirho::VarChirho(resolved_chirho)
                                | KindChirho::RigidChirho(resolved_chirho) => resolved_chirho,
                                _ => continue,
                            }
                        };
                        if !quantified_chirho.contains(&resolved_chirho) {
                            continue;
                        }
                        let argument_chirho = self.subst_chirho.apply_chirho(argument_chirho);
                        if let Some(previous_chirho) =
                            arguments_chirho.insert(resolved_chirho, argument_chirho.clone())
                            && previous_chirho != argument_chirho
                        {
                            invalid_chirho.push(*span_chirho);
                            return None;
                        }
                    }
                    Some((
                        *span_chirho,
                        scheme_chirho
                            .quantified_chirho
                            .iter()
                            .map(|identity_chirho| {
                                let argument_chirho = arguments_chirho
                                    .get(identity_chirho)
                                    .cloned()
                                    .unwrap_or(KindChirho::VarChirho(*identity_chirho));
                                self.subst_chirho.apply_chirho(&argument_chirho)
                            })
                            .collect(),
                    ))
                })
                .collect();
            (finished_chirho, invalid_chirho)
        };
        let (applications_chirho, mut invalid_chirho) =
            finish_occurrences_chirho(&self.kind_applications_chirho);
        let (equation_inputs_chirho, invalid_equations_chirho) =
            finish_occurrences_chirho(&self.kind_equation_inputs_chirho);
        invalid_chirho.extend(invalid_equations_chirho);
        for span_chirho in invalid_chirho {
            self.diagnostics_chirho
                .push_chirho(DiagnosticChirho::error_with_code_chirho(
                    ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                    "inconsistent kind arguments after declaration publication",
                    span_chirho,
                ));
        }
        let mut source_names_chirho = HashMap::new();
        let mut names_chirho: Vec<_> = self.kind_binder_names_chirho.iter().collect();
        names_chirho.sort_by_key(|(identity_chirho, _)| **identity_chirho);
        for (identity_chirho, name_chirho) in names_chirho {
            if let KindChirho::VarChirho(resolved_chirho)
            | KindChirho::RigidChirho(resolved_chirho) = self
                .subst_chirho
                .apply_chirho(&KindChirho::VarChirho(*identity_chirho))
            {
                source_names_chirho
                    .entry(resolved_chirho)
                    .or_insert_with(|| name_chirho.clone());
            }
        }
        KindElaborationChirho {
            applications_chirho,
            wildcard_terms_chirho: self
                .kind_wildcard_terms_chirho
                .iter()
                .map(|(span_chirho, term_chirho)| {
                    (*span_chirho, self.subst_chirho.apply_chirho(term_chirho))
                })
                .collect(),
            nominal_heads_chirho,
            synonym_heads_chirho,
            family_heads_chirho,
            promoted_heads_chirho,
            equation_inputs_chirho,
            source_names_chirho,
        }
    }
}
