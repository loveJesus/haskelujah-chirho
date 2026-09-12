// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Quantification is a binding contract, not the presence of a free metavariable.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{KindChirho, KindInferCtxChirho, KindSubstChirho, KindVarChirho};
use std::collections::HashSet;

pub(super) struct OpenKindBinderChirho {
    pub(super) argument_chirho: KindChirho,
    pub(super) classifier_chirho: KindChirho,
    pub(super) specified_chirho: bool,
}

#[derive(Debug, Clone)]
pub(super) struct KindSchemeChirho {
    pub(super) quantified_chirho: Vec<KindVarChirho>,
    pub(super) specified_chirho: HashSet<KindVarChirho>,
    pub(super) classifiers_chirho: Vec<KindChirho>,
    pub(super) source_names_chirho: Vec<Option<String>>,
    pub(super) body_chirho: KindChirho,
}

#[derive(Debug, Clone)]
pub(super) enum KindBindingChirho {
    MonoChirho(KindChirho),
    PolyChirho(KindSchemeChirho),
}

impl KindSchemeChirho {
    pub(super) fn generalize_chirho(kind_chirho: KindChirho) -> Self {
        let body_chirho = abstract_rigid_kind_chirho(&kind_chirho);
        Self {
            quantified_chirho: body_chirho.free_vars_chirho(),
            specified_chirho: body_chirho.free_vars_chirho().into_iter().collect(),
            classifiers_chirho: body_chirho
                .free_vars_chirho()
                .iter()
                .map(|_| KindChirho::StarChirho)
                .collect(),
            source_names_chirho: body_chirho
                .free_vars_chirho()
                .iter()
                .map(|_| None)
                .collect(),
            body_chirho,
        }
    }

    fn apply_subst_chirho(&mut self, subst_chirho: &KindSubstChirho) {
        let bound_chirho = self.quantified_chirho.iter().copied().collect();
        self.body_chirho =
            apply_scoped_subst_chirho(&self.body_chirho, subst_chirho, &bound_chirho);
        for classifier_chirho in &mut self.classifiers_chirho {
            *classifier_chirho =
                apply_scoped_subst_chirho(classifier_chirho, subst_chirho, &bound_chirho);
        }
    }
}

impl KindBindingChirho {
    pub(super) fn body_chirho(&self) -> &KindChirho {
        match self {
            Self::MonoChirho(kind_chirho) => kind_chirho,
            Self::PolyChirho(scheme_chirho) => &scheme_chirho.body_chirho,
        }
    }

    pub(super) fn apply_subst_chirho(&mut self, subst_chirho: &KindSubstChirho) {
        match self {
            Self::MonoChirho(kind_chirho) => *kind_chirho = subst_chirho.apply_chirho(kind_chirho),
            Self::PolyChirho(scheme_chirho) => scheme_chirho.apply_subst_chirho(subst_chirho),
        }
    }

    pub(super) fn default_unquantified_chirho(&mut self) {
        let bound_chirho = match self {
            Self::MonoChirho(_) => HashSet::new(),
            Self::PolyChirho(scheme_chirho) => {
                scheme_chirho.quantified_chirho.iter().copied().collect()
            }
        };
        let body_chirho = default_unbound_chirho(self.body_chirho(), &bound_chirho);
        match self {
            Self::MonoChirho(kind_chirho) => *kind_chirho = body_chirho,
            Self::PolyChirho(scheme_chirho) => scheme_chirho.body_chirho = body_chirho,
        }
    }
}

fn abstract_rigid_kind_chirho(kind_chirho: &KindChirho) -> KindChirho {
    kind_chirho.map_leaves_chirho(&mut |term_chirho| match term_chirho {
        KindChirho::RigidChirho(variable_chirho) => KindChirho::VarChirho(*variable_chirho),
        _ => term_chirho.clone(),
    })
}

fn default_unbound_chirho(
    kind_chirho: &KindChirho,
    bound_chirho: &HashSet<KindVarChirho>,
) -> KindChirho {
    kind_chirho.map_leaves_chirho(&mut |term_chirho| match term_chirho {
        KindChirho::VarChirho(variable_chirho) if !bound_chirho.contains(variable_chirho) => {
            KindChirho::StarChirho
        }
        _ => term_chirho.clone(),
    })
}

/// Traverse only the scheme and substitution paths it reaches, never copy the
/// growing substitution just to remove this scheme's small set of bound names.
fn apply_scoped_subst_chirho(
    kind_chirho: &KindChirho,
    subst_chirho: &KindSubstChirho,
    bound_chirho: &HashSet<KindVarChirho>,
) -> KindChirho {
    kind_chirho.map_leaves_chirho(&mut |term_chirho| match term_chirho {
        KindChirho::VarChirho(variable_chirho) if !bound_chirho.contains(variable_chirho) => {
            subst_chirho
                .map_chirho
                .get(variable_chirho)
                .map(|value_chirho| {
                    apply_scoped_subst_chirho(value_chirho, subst_chirho, bound_chirho)
                })
                .unwrap_or_else(|| term_chirho.clone())
        }
        _ => term_chirho.clone(),
    })
}

impl KindInferCtxChirho {
    /// Generalize source identities in dependency order. Explicit binders are
    /// retained even when phantom; inferred classifiers precede their users.
    /// Workflow: language-features-chirho/declaration-kinds-chirho.
    pub(super) fn source_kind_scheme_chirho(
        &self,
        kind_chirho: KindChirho,
        explicit_chirho: Vec<KindVarChirho>,
        written_chirho: &HashSet<KindVarChirho>,
    ) -> KindSchemeChirho {
        let mut variables_chirho = explicit_chirho;
        variables_chirho.extend(
            abstract_rigid_kind_chirho(&self.subst_chirho.apply_chirho(&kind_chirho))
                .free_vars_chirho(),
        );
        self.bind_source_kind_scheme_chirho(kind_chirho, variables_chirho, written_chirho)
    }

    /// Bind only the selected source variables and their dependencies. During
    /// family inference, unselected classifier holes remain shared metavariables:
    /// equations determine them, rather than each equation freshening them away.
    pub(super) fn bind_source_kind_scheme_chirho(
        &self,
        kind_chirho: KindChirho,
        variables_chirho: Vec<KindVarChirho>,
        written_chirho: &HashSet<KindVarChirho>,
    ) -> KindSchemeChirho {
        let body_chirho = abstract_rigid_kind_chirho(&self.subst_chirho.apply_chirho(&kind_chirho));
        let mut ordered_chirho = Vec::new();
        let mut seen_chirho = HashSet::new();
        let mut classifiers_chirho = Vec::new();
        let mut source_names_chirho = Vec::new();
        let mut specified_chirho = HashSet::new();
        let mut pending_chirho = variables_chirho;
        pending_chirho.reverse();
        // A two-stage DFS adds dependencies before their binder without sorting
        // away source order. The visited set bounds work even on malformed cycles.
        let mut stack_chirho: Vec<_> = pending_chirho
            .into_iter()
            .map(|identity_chirho| (identity_chirho, false))
            .collect();
        while let Some((identity_chirho, ready_chirho)) = stack_chirho.pop() {
            let resolved_chirho = abstract_rigid_kind_chirho(
                &self
                    .subst_chirho
                    .apply_chirho(&KindChirho::VarChirho(identity_chirho)),
            );
            let KindChirho::VarChirho(variable_chirho) = resolved_chirho else {
                continue;
            };
            let classifier_chirho = self
                .kind_binder_classifiers_chirho
                .get(&identity_chirho)
                .or_else(|| self.kind_binder_classifiers_chirho.get(&variable_chirho))
                .map(|kind_chirho| {
                    abstract_rigid_kind_chirho(&self.subst_chirho.apply_chirho(kind_chirho))
                })
                .unwrap_or(KindChirho::StarChirho);
            if ready_chirho {
                ordered_chirho.push(variable_chirho);
                classifiers_chirho.push(classifier_chirho);
                source_names_chirho.push(
                    self.kind_binder_names_chirho
                        .get(&identity_chirho)
                        .or_else(|| self.kind_binder_names_chirho.get(&variable_chirho))
                        .cloned(),
                );
                let specificity_chirho = self
                    .kind_binder_specificity_chirho
                    .get(&identity_chirho)
                    .or_else(|| self.kind_binder_specificity_chirho.get(&variable_chirho));
                if matches!(
                    specificity_chirho,
                    Some(
                        haskelujah_ast_chirho::decl_chirho::TyVarSpecificityChirho::SpecifiedChirho
                    )
                ) || (specificity_chirho.is_none() && written_chirho.contains(&variable_chirho))
                {
                    specified_chirho.insert(variable_chirho);
                }
            } else if seen_chirho.insert(variable_chirho) {
                stack_chirho.push((identity_chirho, true));
                for dependency_chirho in classifier_chirho.free_vars_chirho().into_iter().rev() {
                    stack_chirho.push((dependency_chirho, false));
                }
            }
        }
        KindSchemeChirho {
            quantified_chirho: ordered_chirho,
            specified_chirho,
            classifiers_chirho,
            source_names_chirho,
            body_chirho,
        }
    }

    pub(super) fn rigidify_kind_variables_chirho(
        &mut self,
        variables_chirho: impl IntoIterator<Item = KindVarChirho>,
    ) {
        let mut variables_chirho: Vec<_> = variables_chirho.into_iter().collect();
        variables_chirho.sort_unstable();
        variables_chirho.dedup();
        for variable_chirho in variables_chirho {
            if let KindChirho::VarChirho(unresolved_chirho) = self
                .subst_chirho
                .apply_chirho(&KindChirho::VarChirho(variable_chirho))
            {
                let rigid_chirho = KindChirho::RigidChirho(self.fresh_var_chirho());
                self.subst_chirho
                    .map_chirho
                    .insert(unresolved_chirho, rigid_chirho);
            }
        }
    }

    pub(super) fn instantiate_binding_chirho(
        &mut self,
        binding_chirho: &KindBindingChirho,
    ) -> KindChirho {
        match binding_chirho {
            KindBindingChirho::MonoChirho(kind_chirho) => {
                self.subst_chirho.apply_chirho(kind_chirho)
            }
            KindBindingChirho::PolyChirho(scheme_chirho) => {
                self.open_kind_scheme_chirho(scheme_chirho, false)
            }
        }
    }

    pub(super) fn open_kind_scheme_chirho(
        &mut self,
        scheme_chirho: &KindSchemeChirho,
        rigid_chirho: bool,
    ) -> KindChirho {
        self.open_kind_scheme_parts_chirho(scheme_chirho, rigid_chirho)
            .0
    }

    pub(super) fn instantiate_source_binding_chirho(
        &mut self,
        name_chirho: &str,
        binding_chirho: &KindBindingChirho,
        span_chirho: super::SpanChirho,
    ) -> KindChirho {
        let (kind_chirho, arguments_chirho) = match binding_chirho {
            KindBindingChirho::MonoChirho(kind_chirho) => {
                (self.subst_chirho.apply_chirho(kind_chirho), Vec::new())
            }
            KindBindingChirho::PolyChirho(scheme_chirho) => {
                self.open_kind_scheme_parts_chirho(scheme_chirho, false)
            }
        };
        self.record_kind_application_chirho(
            name_chirho,
            binding_chirho,
            arguments_chirho,
            span_chirho,
        );
        kind_chirho
    }

    pub(super) fn open_kind_scheme_parts_chirho(
        &mut self,
        scheme_chirho: &KindSchemeChirho,
        rigid_chirho: bool,
    ) -> (KindChirho, Vec<KindChirho>) {
        let (body_chirho, binders_chirho) =
            self.open_kind_scheme_contract_chirho(scheme_chirho, rigid_chirho);
        (
            body_chirho,
            binders_chirho
                .into_iter()
                .map(|binder_chirho| binder_chirho.argument_chirho)
                .collect(),
        )
    }

    fn open_kind_scheme_contract_chirho(
        &mut self,
        scheme_chirho: &KindSchemeChirho,
        rigid_chirho: bool,
    ) -> (KindChirho, Vec<OpenKindBinderChirho>) {
        let bound_chirho = scheme_chirho.quantified_chirho.iter().copied().collect();
        let body_chirho = apply_scoped_subst_chirho(
            &scheme_chirho.body_chirho,
            &self.subst_chirho,
            &bound_chirho,
        );
        let mut replacement_chirho = KindSubstChirho::empty_chirho();
        let mut arguments_chirho = Vec::with_capacity(scheme_chirho.quantified_chirho.len());
        for variable_chirho in &scheme_chirho.quantified_chirho {
            let fresh_chirho = self.fresh_var_chirho();
            let kind_chirho = if rigid_chirho {
                KindChirho::RigidChirho(fresh_chirho)
            } else {
                self.instantiated_kind_variables_chirho.insert(fresh_chirho);
                KindChirho::VarChirho(fresh_chirho)
            };
            replacement_chirho
                .map_chirho
                .insert(*variable_chirho, kind_chirho.clone());
            arguments_chirho.push(kind_chirho);
        }
        (
            replacement_chirho.apply_chirho(&body_chirho),
            arguments_chirho
                .into_iter()
                .enumerate()
                .map(|(index_chirho, argument_chirho)| OpenKindBinderChirho {
                    argument_chirho,
                    classifier_chirho: replacement_chirho.apply_chirho(&apply_scoped_subst_chirho(
                        &scheme_chirho.classifiers_chirho[index_chirho],
                        &self.subst_chirho,
                        &bound_chirho,
                    )),
                    specified_chirho: scheme_chirho
                        .specified_chirho
                        .contains(&scheme_chirho.quantified_chirho[index_chirho]),
                })
                .collect(),
        )
    }

    /// Open one head, then consume its entire mixed argument spine once. An @
    /// argument selects the next specified quantifier; it is never an arrow.
    pub(super) fn infer_type_application_kind_chirho(
        &mut self,
        ty_chirho: &super::TypeChirho,
    ) -> KindChirho {
        use super::TypeChirho;
        let mut head_chirho = ty_chirho;
        let mut arguments_chirho = Vec::new();
        loop {
            match head_chirho {
                TypeChirho::AppChirho {
                    fun_chirho,
                    arg_chirho,
                    span_chirho,
                }
                | TypeChirho::KindAppChirho {
                    fun_chirho,
                    arg_chirho,
                    span_chirho,
                } => {
                    arguments_chirho.push((
                        arg_chirho.as_ref(),
                        matches!(head_chirho, TypeChirho::KindAppChirho { .. }),
                        *span_chirho,
                    ));
                    head_chirho = fun_chirho;
                }
                TypeChirho::ParenChirho { inner_chirho, .. } => head_chirho = inner_chirho,
                _ => break,
            }
        }
        let binding_chirho = match head_chirho {
            TypeChirho::ConChirho(name_chirho) => {
                let name_chirho = self.canonical_kind_name_chirho(name_chirho);
                self.env_chirho
                    .lookup_binding_chirho(&name_chirho)
                    .or_else(|| self.env_chirho.lookup_promoted_binding_chirho(&name_chirho))
                    .cloned()
            }
            TypeChirho::VarChirho(name_chirho) => self
                .env_chirho
                .lookup_binding_chirho(name_chirho.text_chirho())
                .cloned(),
            TypeChirho::PromotedConChirho { name_chirho, .. } => self
                .env_chirho
                .lookup_promoted_binding_chirho(&self.canonical_kind_name_chirho(name_chirho))
                .cloned(),
            _ => None,
        };
        let authoritative_chirho = binding_chirho.is_some();
        let (mut tail_chirho, binders_chirho) = match &binding_chirho {
            Some(KindBindingChirho::PolyChirho(scheme_chirho)) => {
                self.open_kind_scheme_contract_chirho(scheme_chirho, false)
            }
            Some(KindBindingChirho::MonoChirho(kind_chirho)) => {
                (self.subst_chirho.apply_chirho(kind_chirho), Vec::new())
            }
            None => (self.infer_type_kind_chirho(head_chirho), Vec::new()),
        };
        let mut binder_index_chirho = 0;
        for (argument_chirho, invisible_chirho, span_chirho) in arguments_chirho.into_iter().rev() {
            let classifier_chirho = self.infer_type_kind_chirho(argument_chirho);
            if invisible_chirho {
                while binder_index_chirho < binders_chirho.len()
                    && !binders_chirho[binder_index_chirho].specified_chirho
                {
                    binder_index_chirho += 1;
                }
                if let Some(binder_chirho) = binders_chirho.get(binder_index_chirho) {
                    self.unify_chirho(
                        &binder_chirho.classifier_chirho,
                        &classifier_chirho,
                        "visible kind argument",
                        argument_chirho.span_chirho(),
                    );
                    let term_chirho = self.interpret_kind_term_chirho(argument_chirho);
                    self.unify_chirho(
                        &binder_chirho.argument_chirho,
                        &term_chirho,
                        "visible kind argument",
                        span_chirho,
                    );
                    binder_index_chirho += 1;
                } else if authoritative_chirho {
                    self.diagnostics_chirho.push_chirho(
                        super::DiagnosticChirho::error_with_code_chirho(
                            super::ErrorCodeChirho::error_chirho(super::KIND_MISMATCH_CODE_CHIRHO),
                            "no specified kind argument is available for this @ application",
                            argument_chirho.span_chirho(),
                        ),
                    );
                }
            } else {
                binder_index_chirho = binders_chirho.len();
                let term_chirho = self.interpret_kind_term_chirho(argument_chirho);
                tail_chirho = self.consume_kind_argument_chirho(
                    tail_chirho,
                    &classifier_chirho,
                    Some(&term_chirho),
                    span_chirho,
                    "type application",
                );
            }
        }
        if let (TypeChirho::ConChirho(name_chirho), Some(binding_chirho)) =
            (head_chirho, &binding_chirho)
        {
            self.record_kind_application_chirho(
                &self.canonical_kind_name_chirho(name_chirho),
                binding_chirho,
                binders_chirho
                    .into_iter()
                    .map(|binder_chirho| binder_chirho.argument_chirho)
                    .collect(),
                ty_chirho.span_chirho(),
            );
        }
        self.subst_chirho.apply_chirho(&tail_chirho)
    }

    pub(super) fn publish_kind_chirho(
        &mut self,
        name_chirho: &str,
        named_variables_chirho: &HashSet<KindVarChirho>,
    ) {
        // A leading explicit forall may have been published before the body,
        // while ordinary head classifiers remained inference holes. Close those
        // remaining holes at the same SCC boundary as a wholly inferred head.
        if let Some(KindBindingChirho::PolyChirho(mut scheme_chirho)) =
            self.env_chirho.lookup_binding_chirho(name_chirho).cloned()
        {
            scheme_chirho.apply_subst_chirho(&self.subst_chirho);
            let mut protected_chirho = named_variables_chirho.clone();
            protected_chirho.extend(scheme_chirho.quantified_chirho.iter().copied());
            self.default_inferred_runtime_variables_chirho(
                &scheme_chirho.body_chirho,
                &protected_chirho,
            );
            scheme_chirho.apply_subst_chirho(&self.subst_chirho);
            if self.poly_kinds_enabled_chirho {
                scheme_chirho.body_chirho = abstract_rigid_kind_chirho(&scheme_chirho.body_chirho);
                for variable_chirho in scheme_chirho.body_chirho.free_vars_chirho() {
                    if !scheme_chirho.quantified_chirho.contains(&variable_chirho) {
                        scheme_chirho.quantified_chirho.push(variable_chirho);
                        scheme_chirho
                            .source_names_chirho
                            .push(self.kind_binder_names_chirho.get(&variable_chirho).cloned());
                        scheme_chirho.classifiers_chirho.push(
                            self.kind_binder_classifiers_chirho
                                .get(&variable_chirho)
                                .map(|classifier_chirho| {
                                    self.subst_chirho.apply_chirho(classifier_chirho)
                                })
                                .unwrap_or(KindChirho::StarChirho),
                        );
                        if named_variables_chirho.contains(&variable_chirho) {
                            scheme_chirho.specified_chirho.insert(variable_chirho);
                        }
                    }
                }
            } else {
                scheme_chirho.body_chirho = default_unbound_chirho(
                    &scheme_chirho.body_chirho,
                    &scheme_chirho.quantified_chirho.iter().copied().collect(),
                );
            }
            self.env_chirho.bind_entry_chirho(
                name_chirho.to_owned(),
                KindBindingChirho::PolyChirho(scheme_chirho),
            );
            return;
        }
        if let Some(KindBindingChirho::MonoChirho(kind_chirho)) =
            self.env_chirho.lookup_binding_chirho(name_chirho).cloned()
        {
            // Retain identity provenance before substitution turns a written
            // variable into its rigid representative. The representative alone
            // does not tell us whether the source binder was specified.
            let source_variables_chirho = kind_chirho.free_vars_chirho();
            let kind_chirho = self.subst_chirho.apply_chirho(&kind_chirho);
            self.default_inferred_runtime_variables_chirho(&kind_chirho, named_variables_chirho);
            let kind_chirho = self.subst_chirho.apply_chirho(&kind_chirho);
            let kind_chirho = if self.poly_kinds_enabled_chirho {
                kind_chirho
            } else {
                // Default the identities, not merely a copy of the body. They
                // also occur in pending applications and the pre-substitution
                // source variable list below. Otherwise a monokinded head
                // publishes unused hidden binders with unsolved occurrences.
                for variable_chirho in abstract_rigid_kind_chirho(&kind_chirho).free_vars_chirho() {
                    self.subst_chirho
                        .map_chirho
                        .insert(variable_chirho, KindChirho::StarChirho);
                }
                super::default_kind_vars_chirho(&abstract_rigid_kind_chirho(&kind_chirho))
            };
            self.env_chirho.bind_entry_chirho(
                name_chirho.to_owned(),
                KindBindingChirho::PolyChirho(self.source_kind_scheme_chirho(
                    kind_chirho,
                    source_variables_chirho,
                    named_variables_chirho,
                )),
            );
        }
    }
}
