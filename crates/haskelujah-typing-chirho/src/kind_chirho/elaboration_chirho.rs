// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Solved kind arguments crossing into type inference. Workflow: declaration-kinds-chirho.
use super::*;

pub(super) struct PendingKindApplicationChirho {
    head_chirho: String,
    // Already-polymorphic binders have fresh occurrence arguments. A binder
    // generalized only after this occurrence retains the group's own identity.
    arguments_chirho: HashMap<KindVarChirho, KindChirho>,
}

#[derive(Clone, Debug)]
pub(crate) struct ElaboratedKindBinderChirho {
    pub(crate) identity_chirho: KindVarChirho,
    pub(crate) name_chirho: Option<String>,
    pub(crate) specified_chirho: bool,
}

impl ElaboratedKindBinderChirho {
    pub(crate) fn parameter_name_chirho(&self) -> String {
        self.name_chirho
            .clone()
            .unwrap_or_else(|| format!("$kind_chirho_{}", self.identity_chirho.0))
    }
}

/// Local nominal and transparent-synonym contracts share the solved indices.
/// Family equation
/// indices and imported constructor contracts require their own complete
/// elaboration; a missing entry does not authorize inventing their arguments.
#[derive(Clone, Debug, Default)]
pub struct KindElaborationChirho {
    pub(crate) applications_chirho: HashMap<SpanChirho, Vec<KindChirho>>,
    pub(crate) nominal_heads_chirho: HashMap<String, Vec<ElaboratedKindBinderChirho>>,
    pub(crate) synonym_heads_chirho: HashMap<String, Vec<ElaboratedKindBinderChirho>>,
    pub(crate) source_names_chirho: HashMap<KindVarChirho, String>,
}

impl KindInferCtxChirho {
    pub(super) fn record_kind_application_chirho(
        &mut self,
        name_chirho: &str,
        binding_chirho: &KindBindingChirho,
        arguments_chirho: Vec<KindChirho>,
        span_chirho: SpanChirho,
    ) {
        if span_chirho == SpanChirho::DUMMY_CHIRHO
            || !self.local_kind_decl_names_chirho.contains(name_chirho)
        {
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
                arguments_chirho,
            },
        );
    }

    pub(super) fn finish_kind_elaboration_chirho(
        &self,
        module_chirho: &ModuleChirho,
    ) -> KindElaborationChirho {
        let mut nominal_heads_chirho = HashMap::new();
        let mut synonym_heads_chirho = HashMap::new();
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
        let applications_chirho = self
            .kind_applications_chirho
            .iter()
            .filter_map(|(span_chirho, occurrence_chirho)| {
                let Some(KindBindingChirho::PolyChirho(scheme_chirho)) = self
                    .env_chirho
                    .lookup_binding_chirho(&occurrence_chirho.head_chirho)
                else {
                    return None;
                };
                Some((
                    *span_chirho,
                    scheme_chirho
                        .quantified_chirho
                        .iter()
                        .map(|identity_chirho| {
                            let argument_chirho = occurrence_chirho
                                .arguments_chirho
                                .get(identity_chirho)
                                .cloned()
                                .unwrap_or(KindChirho::VarChirho(*identity_chirho));
                            self.subst_chirho.apply_chirho(&argument_chirho)
                        })
                        .collect(),
                ))
            })
            .collect();
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
            nominal_heads_chirho,
            synonym_heads_chirho,
            source_names_chirho,
        }
    }
}
