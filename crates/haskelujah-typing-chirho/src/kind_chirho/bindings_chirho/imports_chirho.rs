// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Portable defining-module kind contracts. Workflow: declaration-kinds-chirho.
use super::*;

#[derive(Clone, Copy, Debug)]
pub(super) enum KindHeadShapeChirho {
    NominalChirho,
    SynonymChirho,
    FamilyChirho,
    ClassChirho,
}

/// An opaque checked scheme. Its numeric identities are private to this
/// template and are renamed before entering another module's inference state.
#[derive(Clone, Debug)]
pub struct KindContractChirho {
    pub(super) binding_chirho: KindBindingChirho,
    pub(super) shape_chirho: KindHeadShapeChirho,
}

impl KindContractChirho {
    pub(super) fn binding_is_closed_chirho(binding_chirho: &KindBindingChirho) -> bool {
        let identities_chirho = |kind_chirho: &KindChirho| {
            kind_chirho
                .map_leaves_chirho(&mut |leaf_chirho| match leaf_chirho {
                    KindChirho::RigidChirho(identity_chirho) => {
                        KindChirho::VarChirho(*identity_chirho)
                    }
                    _ => leaf_chirho.clone(),
                })
                .free_vars_chirho()
        };
        match binding_chirho {
            KindBindingChirho::MonoChirho(body_chirho) => identities_chirho(body_chirho).is_empty(),
            KindBindingChirho::PolyChirho(scheme_chirho) => {
                let quantified_chirho: std::collections::HashSet<_> =
                    scheme_chirho.quantified_chirho.iter().copied().collect();
                if quantified_chirho.len() != scheme_chirho.quantified_chirho.len()
                    || scheme_chirho.classifiers_chirho.len() != quantified_chirho.len()
                    || scheme_chirho.source_names_chirho.len() != quantified_chirho.len()
                    || !scheme_chirho.specified_chirho.is_subset(&quantified_chirho)
                    || identities_chirho(&scheme_chirho.body_chirho)
                        .iter()
                        .any(|identity_chirho| !quantified_chirho.contains(identity_chirho))
                {
                    return false;
                }
                let mut prefix_chirho = std::collections::HashSet::new();
                for (identity_chirho, classifier_chirho) in scheme_chirho
                    .quantified_chirho
                    .iter()
                    .zip(&scheme_chirho.classifiers_chirho)
                {
                    if identities_chirho(classifier_chirho)
                        .iter()
                        .any(|dependency_chirho| !prefix_chirho.contains(dependency_chirho))
                    {
                        return false;
                    }
                    prefix_chirho.insert(*identity_chirho);
                }
                true
            }
        }
    }

    /// Authored contracts for interface-only modules must state a closed kind.
    /// This constructor never turns a free classifier into an implicit default.
    pub fn monomorphic_nominal_chirho(kind_chirho: KindChirho) -> Self {
        assert!(
            kind_chirho.free_vars_chirho().is_empty(),
            "a builtin monomorphic kind must be closed"
        );
        Self {
            binding_chirho: KindBindingChirho::PolyChirho(KindSchemeChirho::generalize_chirho(
                kind_chirho,
            )),
            shape_chirho: KindHeadShapeChirho::NominalChirho,
        }
    }

    pub fn map_constructor_names_chirho(
        &self,
        rename_chirho: &mut impl FnMut(&str) -> String,
    ) -> Self {
        let mut mapped_chirho = self.clone();
        let mut map_chirho = |kind_chirho: &KindChirho| {
            kind_chirho.map_leaves_chirho(&mut |leaf_chirho| match leaf_chirho {
                KindChirho::ConChirho(name_chirho) => {
                    KindChirho::ConChirho(rename_chirho(name_chirho))
                }
                _ => leaf_chirho.clone(),
            })
        };
        match &mut mapped_chirho.binding_chirho {
            KindBindingChirho::MonoChirho(kind_chirho) => *kind_chirho = map_chirho(kind_chirho),
            KindBindingChirho::PolyChirho(scheme_chirho) => {
                scheme_chirho.body_chirho = map_chirho(&scheme_chirho.body_chirho);
                scheme_chirho.classifiers_chirho = scheme_chirho
                    .classifiers_chirho
                    .iter()
                    .map(map_chirho)
                    .collect();
            }
        }
        mapped_chirho
    }
}

impl KindInferCtxChirho {
    pub(super) fn seed_imported_kind_contracts_chirho(
        &mut self,
        imported_chirho: &HashMap<String, KindContractChirho>,
    ) {
        let mut names_chirho: Vec<_> = imported_chirho.keys().collect();
        names_chirho.sort();
        for name_chirho in names_chirho {
            let contract_chirho = &imported_chirho[name_chirho];
            let mut binding_chirho = contract_chirho.binding_chirho.clone();
            if let KindBindingChirho::PolyChirho(scheme_chirho) = &mut binding_chirho {
                let mut rename_chirho = KindSubstChirho::empty_chirho();
                let quantified_chirho: Vec<_> = scheme_chirho
                    .quantified_chirho
                    .iter()
                    .map(|old_chirho| {
                        let fresh_chirho = self.fresh_var_chirho();
                        rename_chirho
                            .map_chirho
                            .insert(*old_chirho, KindChirho::VarChirho(fresh_chirho));
                        fresh_chirho
                    })
                    .collect();
                let specified_chirho = scheme_chirho
                    .quantified_chirho
                    .iter()
                    .zip(&quantified_chirho)
                    .filter_map(|(old_chirho, fresh_chirho)| {
                        scheme_chirho
                            .specified_chirho
                            .contains(old_chirho)
                            .then_some(*fresh_chirho)
                    })
                    .collect();
                // A simultaneous renaming must not chase newly allocated ids as
                // keys in the defining module's unrelated numeric namespace.
                let rebase_chirho = |kind_chirho: &KindChirho| {
                    kind_chirho.map_leaves_chirho(&mut |leaf_chirho| match leaf_chirho {
                        KindChirho::VarChirho(identity_chirho)
                        | KindChirho::RigidChirho(identity_chirho) => rename_chirho
                            .map_chirho
                            .get(identity_chirho)
                            .cloned()
                            .unwrap_or_else(|| leaf_chirho.clone()),
                        _ => leaf_chirho.clone(),
                    })
                };
                scheme_chirho.body_chirho = rebase_chirho(&scheme_chirho.body_chirho);
                scheme_chirho.classifiers_chirho = scheme_chirho
                    .classifiers_chirho
                    .iter()
                    .map(rebase_chirho)
                    .collect();
                scheme_chirho.quantified_chirho = quantified_chirho;
                scheme_chirho.specified_chirho = specified_chirho;
            }
            self.env_chirho
                .bind_entry_chirho(name_chirho.clone(), binding_chirho);
            if matches!(
                contract_chirho.shape_chirho,
                KindHeadShapeChirho::FamilyChirho
            ) {
                // The checked provider owns this classification. Its equations
                // are not transported here, but neither row checking nor kind
                // equality may treat the imported head as nominally injective.
                self.kind_family_names_chirho.insert(name_chirho.clone());
            }
            self.imported_kind_shapes_chirho
                .insert(name_chirho.clone(), contract_chirho.shape_chirho);
        }
    }

    pub(super) fn export_kind_contracts_chirho(
        &mut self,
        module_chirho: &ModuleChirho,
    ) -> HashMap<String, KindContractChirho> {
        let mut contracts_chirho = HashMap::new();
        for declaration_chirho in &module_chirho.decls_chirho {
            let (name_chirho, shape_chirho) = match declaration_chirho {
                DeclChirho::DataDeclChirho { name_chirho, .. }
                | DeclChirho::NewtypeDeclChirho { name_chirho, .. } => {
                    (name_chirho, KindHeadShapeChirho::NominalChirho)
                }
                DeclChirho::TypeAliasDeclChirho { name_chirho, .. } => {
                    (name_chirho, KindHeadShapeChirho::SynonymChirho)
                }
                DeclChirho::TypeFamilyDeclChirho { name_chirho, .. } => {
                    (name_chirho, KindHeadShapeChirho::FamilyChirho)
                }
                DeclChirho::ClassDeclChirho { name_chirho, .. } => {
                    (name_chirho, KindHeadShapeChirho::ClassChirho)
                }
                _ => continue,
            };
            if let Some(binding_chirho) = self
                .env_chirho
                .lookup_binding_chirho(name_chirho.text_chirho())
            {
                if !KindContractChirho::binding_is_closed_chirho(binding_chirho) {
                    self.diagnostics_chirho
                        .push_chirho(DiagnosticChirho::error_with_code_chirho(
                            ErrorCodeChirho::error_chirho(KIND_MISMATCH_CODE_CHIRHO),
                            "kind contract is not closed over its declared classifier dependencies",
                            name_chirho.span_chirho(),
                        ));
                    continue;
                }
                contracts_chirho.insert(
                    name_chirho.text_chirho().to_owned(),
                    KindContractChirho {
                        binding_chirho: binding_chirho.clone(),
                        shape_chirho,
                    },
                );
            }
        }
        contracts_chirho
    }
}
