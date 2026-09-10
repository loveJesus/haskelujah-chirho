// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Quantification is a binding contract, not the presence of a free metavariable.
//! Workflow: language-features-chirho/declaration-kinds-chirho.

use super::{KindChirho, KindInferCtxChirho, KindSubstChirho, KindVarChirho};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub(super) struct KindSchemeChirho {
    pub(super) quantified_chirho: Vec<KindVarChirho>,
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
            body_chirho,
        }
    }

    fn apply_subst_chirho(&mut self, subst_chirho: &KindSubstChirho) {
        let bound_chirho = self.quantified_chirho.iter().copied().collect();
        self.body_chirho =
            apply_scoped_subst_chirho(&self.body_chirho, subst_chirho, &bound_chirho);
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
        let bound_chirho = scheme_chirho.quantified_chirho.iter().copied().collect();
        let body_chirho = apply_scoped_subst_chirho(
            &scheme_chirho.body_chirho,
            &self.subst_chirho,
            &bound_chirho,
        );
        let mut replacement_chirho = KindSubstChirho::empty_chirho();
        for variable_chirho in &scheme_chirho.quantified_chirho {
            let fresh_chirho = self.fresh_var_chirho();
            let kind_chirho = if rigid_chirho {
                KindChirho::RigidChirho(fresh_chirho)
            } else {
                KindChirho::VarChirho(fresh_chirho)
            };
            replacement_chirho
                .map_chirho
                .insert(*variable_chirho, kind_chirho);
        }
        replacement_chirho.apply_chirho(&body_chirho)
    }

    pub(super) fn publish_kind_chirho(
        &mut self,
        name_chirho: &str,
        named_variables_chirho: &HashSet<KindVarChirho>,
    ) {
        if let Some(KindBindingChirho::MonoChirho(kind_chirho)) =
            self.env_chirho.lookup_binding_chirho(name_chirho).cloned()
        {
            let kind_chirho = self.subst_chirho.apply_chirho(&kind_chirho);
            self.default_inferred_runtime_variables_chirho(&kind_chirho, named_variables_chirho);
            let kind_chirho = self.subst_chirho.apply_chirho(&kind_chirho);
            let kind_chirho = if self.poly_kinds_enabled_chirho {
                kind_chirho
            } else {
                super::default_kind_vars_chirho(&abstract_rigid_kind_chirho(&kind_chirho))
            };
            self.env_chirho
                .bind_generalized_chirho(name_chirho.to_owned(), kind_chirho);
        }
    }
}
