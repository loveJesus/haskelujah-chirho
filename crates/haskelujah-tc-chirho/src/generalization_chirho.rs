// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Solve, promote restricted candidates, quantify, then abstract evidence at its lexical owner.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;

impl CheckedModuleChirho {
    pub(super) fn close_group_chirho(
        &mut self,
        bindings_chirho: &[BindingChirho],
        group_chirho: &[usize],
        outer_level_chirho: u32,
    ) -> Result<(), TcErrorChirho> {
        self.solver_chirho.solve_chirho()?;
        self.solver_chirho.solve_dictionaries_chirho()?;
        let candidates_chirho = self
            .solver_chirho
            .dictionary_candidates_chirho(self.scope_chirho)?;
        let restricted_chirho = group_chirho
            .iter()
            .any(|index_chirho| bindings_chirho[*index_chirho].restricted_chirho);
        if restricted_chirho {
            self.solver_chirho
                .arena_chirho
                .promote_chirho(&candidates_chirho, outer_level_chirho)?;
        }
        let roots_chirho: Vec<_> = group_chirho
            .iter()
            .map(|index_chirho| self.names_chirho[&bindings_chirho[*index_chirho].name_chirho])
            .collect();
        let quantified_chirho = self
            .solver_chirho
            .arena_chirho
            .quantify_group_chirho(&roots_chirho, outer_level_chirho)?;
        let mut context_chirho = Vec::new();
        let mut dictionaries_chirho = Vec::new();
        if !restricted_chirho {
            for predicate_chirho in candidates_chirho {
                let variables_chirho = self
                    .solver_chirho
                    .arena_chirho
                    .free_variables_chirho(&[predicate_chirho])?;
                if !quantified_chirho
                    .iter()
                    .any(|variable_chirho| variables_chirho.contains(variable_chirho))
                {
                    continue;
                }
                let (_, argument_chirho) =
                    self.solver_chirho.predicate_view_chirho(predicate_chirho)?;
                if !matches!(
                    self.solver_chirho
                        .arena_chirho
                        .node_chirho(argument_chirho)?,
                    TyNodeChirho::MetaChirho(_) | TyNodeChirho::RigidChirho(_)
                ) {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "next does not yet generalize non-variable class contexts",
                    ));
                }
                if group_chirho.len() > 1 {
                    for root_chirho in &roots_chirho {
                        let free_chirho = self
                            .solver_chirho
                            .arena_chirho
                            .free_variables_chirho(&[*root_chirho])?;
                        if !variables_chirho.is_subset(&free_chirho) {
                            return Err(TcErrorChirho::new_chirho(
                                TcErrorKindChirho::UnsupportedChirho,
                                "next recursive class exports need impedance matching/defaulting",
                            ));
                        }
                    }
                }
                let evidence_chirho = self
                    .solver_chirho
                    .given_dictionary_chirho(self.scope_chirho, predicate_chirho)?;
                context_chirho.push(predicate_chirho);
                dictionaries_chirho.push(evidence_chirho);
            }
        }
        self.solver_chirho.solve_dictionaries_chirho()?;
        for &index_chirho in group_chirho {
            let binding_chirho = &bindings_chirho[index_chirho];
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            for _binder_chirho in &quantified_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            }
            for _predicate_chirho in &context_chirho {
                self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            }
            self.schemes_chirho.insert(
                binding_chirho.name_chirho,
                SchemeChirho {
                    body_chirho: self.names_chirho[&binding_chirho.name_chirho],
                    quantified_chirho: quantified_chirho.clone(),
                    context_chirho: context_chirho.clone(),
                },
            );
            self.solver_chirho.arena_chirho.reserve_auxiliary_chirho()?;
            self.abstractions_chirho.insert(
                binding_chirho.name_chirho,
                AbstractionChirho {
                    type_binders_chirho: quantified_chirho.clone(),
                    dictionary_binders_chirho: dictionaries_chirho.clone(),
                    scope_chirho: self.scope_chirho,
                },
            );
        }
        Ok(())
    }
}
