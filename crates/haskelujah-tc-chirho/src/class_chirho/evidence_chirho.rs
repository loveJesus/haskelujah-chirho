// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Scope-owned A-normal dictionary evidence. No textual authority or legacy dictionary lookup.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;
use haskelujah_hir_chirho::NameIdChirho;
use std::collections::HashMap;

#[cfg(test)]
#[path = "evidence_tests_chirho.rs"]
mod evidence_tests_chirho;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvVarChirho {
    owner_chirho: u64,
    index_chirho: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvTermChirho {
    VariableChirho(EvVarChirho),
    InstanceChirho {
        instance_chirho: NameIdChirho,
        type_arguments_chirho: Vec<TyChirho>,
        dictionary_arguments_chirho: Vec<EvVarChirho>,
    },
    SuperclassChirho {
        class_chirho: NameIdChirho,
        index_chirho: usize,
        argument_chirho: TyChirho,
        dictionary_chirho: EvVarChirho,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKindChirho {
    RootChirho,
    GroupChirho,
    InstanceChirho,
    MethodChirho,
    SignatureChirho,
    SuperclassChirho,
    DefaultMethodChirho,
}

#[derive(Debug, Clone, Copy)]
pub enum WantedOriginChirho {
    OccurrenceChirho,
    InstanceContextChirho,
    SuperclassChirho { naked_chirho: bool },
}

#[derive(Debug, Clone)]
pub struct GivenDictChirho {
    pub predicate_chirho: TyChirho,
    pub evidence_chirho: EvVarChirho,
    pub superclass_depth_chirho: u32,
    pub blocked_chirho: bool,
}

#[derive(Debug, Clone)]
pub struct WantedDictChirho {
    pub predicate_chirho: TyChirho,
    pub evidence_chirho: EvVarChirho,
    pub scope_chirho: ScopeIdChirho,
    pub span_chirho: SpanChirho,
    pub origin_chirho: WantedOriginChirho,
}

#[derive(Debug, Clone)]
pub(super) struct EvidenceRecordChirho {
    predicate_chirho: TyChirho,
    scope_chirho: ScopeIdChirho,
    given_chirho: bool,
    term_chirho: Option<EvTermChirho>,
}

#[derive(Debug, Clone)]
pub struct MethodInfoChirho {
    pub name_chirho: NameIdChirho,
    pub scheme_chirho: crate::SchemeChirho,
}

#[derive(Debug, Clone)]
pub struct ClassInfoChirho {
    pub name_chirho: NameIdChirho,
    pub parameter_chirho: TyChirho,
    pub superclasses_chirho: Vec<TyChirho>,
    pub methods_chirho: Vec<MethodInfoChirho>,
}

#[derive(Debug, Clone)]
pub struct InstanceMethodInfoChirho {
    pub method_chirho: NameIdChirho,
    pub binding_chirho: NameIdChirho,
    pub scope_chirho: ScopeIdChirho,
}

#[derive(Debug, Clone)]
pub struct InstanceInfoChirho {
    pub name_chirho: NameIdChirho,
    pub class_chirho: NameIdChirho,
    pub binders_chirho: Vec<TyChirho>,
    pub context_chirho: Vec<TyChirho>,
    pub head_chirho: TyChirho,
    pub scope_chirho: ScopeIdChirho,
    pub methods_chirho: Vec<InstanceMethodInfoChirho>,
}

impl SolverChirho {
    pub fn class_info_chirho(
        &self,
        name_chirho: NameIdChirho,
    ) -> Result<&ClassInfoChirho, TcErrorChirho> {
        self.classes_chirho.get(&name_chirho).ok_or_else(|| {
            TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "class identity has no checked declaration",
            )
        })
    }

    pub fn instance_info_chirho(
        &self,
        name_chirho: NameIdChirho,
    ) -> Result<&InstanceInfoChirho, TcErrorChirho> {
        self.instances_chirho.get(&name_chirho).ok_or_else(|| {
            TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "instance identity has no checked declaration",
            )
        })
    }

    pub fn scope_kind_chirho(
        &self,
        scope_chirho: ScopeIdChirho,
    ) -> Result<ScopeKindChirho, TcErrorChirho> {
        Ok(self.frame_chirho(scope_chirho)?.kind_chirho)
    }

    fn frame_chirho(&self, scope_chirho: ScopeIdChirho) -> Result<&ScopeChirho, TcErrorChirho> {
        self.level_chirho(scope_chirho)?;
        Ok(&self.scopes_chirho[scope_chirho.index_chirho])
    }

    pub fn scope_parent_chirho(
        &self,
        scope_chirho: ScopeIdChirho,
    ) -> Result<Option<ScopeIdChirho>, TcErrorChirho> {
        Ok(self.frame_chirho(scope_chirho)?.parent_chirho)
    }

    pub fn scope_givens_chirho(
        &self,
        scope_chirho: ScopeIdChirho,
    ) -> Result<&[GivenDictChirho], TcErrorChirho> {
        Ok(&self.frame_chirho(scope_chirho)?.givens_chirho)
    }

    pub fn scope_bindings_chirho(
        &self,
        scope_chirho: ScopeIdChirho,
    ) -> Result<&[EvVarChirho], TcErrorChirho> {
        Ok(&self.frame_chirho(scope_chirho)?.evidence_bindings_chirho)
    }

    pub(crate) fn within_chirho(
        &self,
        mut child_chirho: ScopeIdChirho,
        ancestor_chirho: ScopeIdChirho,
    ) -> Result<bool, TcErrorChirho> {
        self.level_chirho(ancestor_chirho)?;
        loop {
            if child_chirho == ancestor_chirho {
                return Ok(true);
            }
            let Some(parent_chirho) = self.scope_parent_chirho(child_chirho)? else {
                return Ok(false);
            };
            child_chirho = parent_chirho;
        }
    }

    fn evidence_record_chirho(
        &self,
        variable_chirho: EvVarChirho,
    ) -> Result<&EvidenceRecordChirho, TcErrorChirho> {
        if variable_chirho.owner_chirho != self.arena_chirho.identity_chirho() {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "dictionary evidence belongs to another module",
            ));
        }
        self.evidence_chirho
            .get(variable_chirho.index_chirho)
            .ok_or_else(|| {
                TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "invalid dictionary evidence variable",
                )
            })
    }

    pub fn evidence_predicate_chirho(
        &self,
        variable_chirho: EvVarChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        Ok(self
            .evidence_record_chirho(variable_chirho)?
            .predicate_chirho)
    }

    pub fn evidence_term_chirho(
        &self,
        variable_chirho: EvVarChirho,
    ) -> Result<Option<&EvTermChirho>, TcErrorChirho> {
        Ok(self
            .evidence_record_chirho(variable_chirho)?
            .term_chirho
            .as_ref())
    }

    pub(crate) fn predicate_chirho(
        &mut self,
        class_chirho: NameIdChirho,
        argument_chirho: TyChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        let constructor_chirho = self
            .arena_chirho
            .con_chirho(crate::TyConChirho::ClassChirho(class_chirho))?;
        let predicate_chirho = self
            .arena_chirho
            .intern_chirho(TyNodeChirho::AppChirho(constructor_chirho, argument_chirho))?;
        let constraint_chirho = self
            .arena_chirho
            .con_chirho(crate::TyConChirho::ConstraintChirho)?;
        if self.arena_chirho.kind_chirho(predicate_chirho)? != constraint_chirho {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::KindMismatchChirho,
                "dictionary predicate requires kind Constraint",
            ));
        }
        Ok(predicate_chirho)
    }

    pub fn predicate_view_chirho(
        &self,
        predicate_chirho: TyChirho,
    ) -> Result<(NameIdChirho, TyChirho), TcErrorChirho> {
        let TyNodeChirho::AppChirho(head_chirho, argument_chirho) = self
            .arena_chirho
            .node_chirho(self.arena_chirho.resolve_chirho(predicate_chirho)?)?
        else {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "dictionary predicate is not a class application",
            ));
        };
        let TyNodeChirho::ConChirho(crate::TyConChirho::ClassChirho(class_chirho)) =
            self.arena_chirho.node_chirho(*head_chirho)?
        else {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "dictionary predicate has no class identity",
            ));
        };
        if !self.classes_chirho.contains_key(class_chirho) {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "dictionary class has no declaration",
            ));
        }
        Ok((*class_chirho, *argument_chirho))
    }

    fn allocate_evidence_chirho(
        &mut self,
        scope_chirho: ScopeIdChirho,
        predicate_chirho: TyChirho,
        given_chirho: bool,
    ) -> Result<EvVarChirho, TcErrorChirho> {
        self.level_chirho(scope_chirho)?;
        self.predicate_view_chirho(predicate_chirho)?;
        self.arena_chirho.reserve_auxiliary_chirho()?;
        let variable_chirho = EvVarChirho {
            owner_chirho: self.arena_chirho.identity_chirho(),
            index_chirho: self.evidence_chirho.len(),
        };
        self.evidence_chirho.push(EvidenceRecordChirho {
            predicate_chirho,
            scope_chirho,
            given_chirho,
            term_chirho: None,
        });
        Ok(variable_chirho)
    }

    pub(crate) fn want_dictionary_chirho(
        &mut self,
        scope_chirho: ScopeIdChirho,
        predicate_chirho: TyChirho,
        span_chirho: SpanChirho,
    ) -> Result<EvVarChirho, TcErrorChirho> {
        self.want_dictionary_origin_chirho(
            scope_chirho,
            predicate_chirho,
            span_chirho,
            WantedOriginChirho::OccurrenceChirho,
        )
    }

    fn want_dictionary_origin_chirho(
        &mut self,
        scope_chirho: ScopeIdChirho,
        predicate_chirho: TyChirho,
        span_chirho: SpanChirho,
        origin_chirho: WantedOriginChirho,
    ) -> Result<EvVarChirho, TcErrorChirho> {
        let evidence_chirho =
            self.allocate_evidence_chirho(scope_chirho, predicate_chirho, false)?;
        self.arena_chirho.reserve_auxiliary_chirho()?;
        self.dictionary_wanteds_chirho.push(WantedDictChirho {
            predicate_chirho,
            evidence_chirho,
            scope_chirho,
            span_chirho,
            origin_chirho,
        });
        Ok(evidence_chirho)
    }

    pub(crate) fn given_dictionary_chirho(
        &mut self,
        scope_chirho: ScopeIdChirho,
        predicate_chirho: TyChirho,
    ) -> Result<EvVarChirho, TcErrorChirho> {
        let evidence_chirho =
            self.allocate_evidence_chirho(scope_chirho, predicate_chirho, true)?;
        self.arena_chirho.reserve_auxiliary_chirho()?;
        self.scopes_chirho[scope_chirho.index_chirho]
            .givens_chirho
            .push(GivenDictChirho {
                predicate_chirho,
                evidence_chirho,
                superclass_depth_chirho: 0,
                blocked_chirho: false,
            });
        Ok(evidence_chirho)
    }

    fn bind_evidence_chirho(
        &mut self,
        variable_chirho: EvVarChirho,
        term_chirho: EvTermChirho,
    ) -> Result<(), TcErrorChirho> {
        let record_chirho = self.evidence_record_chirho(variable_chirho)?;
        if record_chirho.given_chirho {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "a dictionary parameter cannot be assigned",
            ));
        }
        if let Some(previous_chirho) = &record_chirho.term_chirho {
            return if previous_chirho == &term_chirho {
                Ok(())
            } else {
                Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "dictionary evidence was assigned twice",
                ))
            };
        }
        let scope_chirho = record_chirho.scope_chirho;
        let predicate_chirho = record_chirho.predicate_chirho;
        let proved_chirho = match &term_chirho {
            EvTermChirho::VariableChirho(other_chirho) => {
                let other_record_chirho = self.evidence_record_chirho(*other_chirho)?;
                if !self.within_chirho(scope_chirho, other_record_chirho.scope_chirho)?
                    || (!other_record_chirho.given_chirho
                        && other_chirho.index_chirho >= variable_chirho.index_chirho)
                {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::InvalidReferenceChirho,
                        "dictionary evidence escapes its lexical owner or cycles",
                    ));
                }
                other_record_chirho.predicate_chirho
            }
            EvTermChirho::InstanceChirho {
                instance_chirho,
                type_arguments_chirho,
                dictionary_arguments_chirho,
            } => {
                let (predicate_chirho, context_chirho) =
                    self.instance_application_chirho(*instance_chirho, type_arguments_chirho)?;
                if context_chirho.len() != dictionary_arguments_chirho.len() {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::InvalidReferenceChirho,
                        "instance evidence dictionary arity disagrees with its declaration",
                    ));
                }
                for (expected_chirho, argument_chirho) in
                    context_chirho.into_iter().zip(dictionary_arguments_chirho)
                {
                    let argument_record_chirho = self.evidence_record_chirho(*argument_chirho)?;
                    if *argument_chirho == variable_chirho
                        || !self.within_chirho(scope_chirho, argument_record_chirho.scope_chirho)?
                    {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "instance dictionary argument escapes its lexical owner or cycles",
                        ));
                    }
                    let actual_chirho = argument_record_chirho.predicate_chirho;
                    let expected_chirho = self
                        .arena_chirho
                        .replace_chirho(expected_chirho, HashMap::new())?;
                    let actual_chirho = self
                        .arena_chirho
                        .replace_chirho(actual_chirho, HashMap::new())?;
                    if expected_chirho != actual_chirho {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "instance dictionary arguments prove different predicates or are out of order",
                        ));
                    }
                }
                predicate_chirho
            }
            EvTermChirho::SuperclassChirho { .. } => {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "superclass evidence has no producer in this source subset",
                ));
            }
        };
        let expected_chirho = self
            .arena_chirho
            .replace_chirho(predicate_chirho, HashMap::new())?;
        let proved_chirho = self
            .arena_chirho
            .replace_chirho(proved_chirho, HashMap::new())?;
        if expected_chirho != proved_chirho {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "dictionary evidence proves a different predicate",
            ));
        }
        self.arena_chirho.reserve_auxiliary_chirho()?;
        self.evidence_chirho[variable_chirho.index_chirho].term_chirho = Some(term_chirho);
        self.scopes_chirho[scope_chirho.index_chirho]
            .evidence_bindings_chirho
            .push(variable_chirho);
        Ok(())
    }

    pub(crate) fn dictionary_candidates_chirho(
        &mut self,
        scope_chirho: ScopeIdChirho,
    ) -> Result<Vec<TyChirho>, TcErrorChirho> {
        let mut result_chirho = Vec::new();
        let mut seen_chirho = HashSet::new();
        for index_chirho in 0..self.dictionary_wanteds_chirho.len() {
            let wanted_chirho = self.dictionary_wanteds_chirho[index_chirho].clone();
            if self
                .evidence_term_chirho(wanted_chirho.evidence_chirho)?
                .is_some()
                || !self.within_chirho(wanted_chirho.scope_chirho, scope_chirho)?
            {
                continue;
            }
            let variables_chirho = self
                .arena_chirho
                .free_variables_chirho(&[wanted_chirho.predicate_chirho])?;
            let mut current_chirho = wanted_chirho.scope_chirho;
            let mut blocked_chirho = false;
            while current_chirho != scope_chirho {
                let frame_chirho = self.frame_chirho(current_chirho)?;
                for variable_chirho in &variables_chirho {
                    if let TyNodeChirho::RigidChirho(rigid_chirho) =
                        self.arena_chirho.node_chirho(*variable_chirho)?
                        && frame_chirho.skolems_chirho.contains(rigid_chirho)
                    {
                        blocked_chirho = true;
                    }
                }
                current_chirho = frame_chirho.parent_chirho.ok_or_else(|| {
                    TcErrorChirho::new_chirho(
                        TcErrorKindChirho::InvalidReferenceChirho,
                        "wanted scope is not a descendant of its group",
                    )
                })?;
            }
            if blocked_chirho {
                continue;
            }
            let predicate_chirho = self
                .arena_chirho
                .replace_chirho(wanted_chirho.predicate_chirho, HashMap::new())?;
            if seen_chirho.insert(predicate_chirho) {
                result_chirho.push(predicate_chirho);
            }
        }
        Ok(result_chirho)
    }

    pub(crate) fn solve_dictionaries_chirho(&mut self) -> Result<(), TcErrorChirho> {
        if let Some(error_chirho) = &self.failure_chirho {
            return Err(error_chirho.clone());
        }
        let result_chirho = self.solve_dictionary_work_chirho();
        if let Err(error_chirho) = &result_chirho {
            self.failure_chirho = Some(error_chirho.clone());
        }
        result_chirho
    }

    fn solve_dictionary_work_chirho(&mut self) -> Result<(), TcErrorChirho> {
        let mut earlier_chirho = HashMap::new();
        let mut given_cache_chirho: HashMap<ScopeIdChirho, HashMap<TyChirho, EvVarChirho>> =
            HashMap::new();
        let mut next_chirho = 0;
        while next_chirho < self.dictionary_wanteds_chirho.len() {
            let index_chirho = next_chirho;
            next_chirho += 1;
            let wanted_chirho = self.dictionary_wanteds_chirho[index_chirho].clone();
            if self
                .evidence_term_chirho(wanted_chirho.evidence_chirho)?
                .is_some()
            {
                continue;
            }
            let predicate_chirho = self
                .arena_chirho
                .replace_chirho(wanted_chirho.predicate_chirho, HashMap::new())?;
            let (class_chirho, argument_chirho) = self.predicate_view_chirho(predicate_chirho)?;
            let mut scope_chirho = Some(wanted_chirho.scope_chirho);
            let mut found_chirho = None;
            while let Some(current_chirho) = scope_chirho {
                if let std::collections::hash_map::Entry::Vacant(entry_chirho) =
                    given_cache_chirho.entry(current_chirho)
                {
                    let mut entries_chirho = HashMap::new();
                    let mut givens_chirho = self.scope_givens_chirho(current_chirho)?.to_vec();
                    givens_chirho.sort_by_key(|given_chirho| given_chirho.superclass_depth_chirho);
                    for given_chirho in givens_chirho {
                        if given_chirho.blocked_chirho {
                            continue;
                        }
                        let predicate_chirho = self
                            .arena_chirho
                            .replace_chirho(given_chirho.predicate_chirho, HashMap::new())?;
                        entries_chirho
                            .entry(predicate_chirho)
                            .or_insert(given_chirho.evidence_chirho);
                    }
                    entry_chirho.insert(entries_chirho);
                }
                if let Some(evidence_chirho) =
                    given_cache_chirho[&current_chirho].get(&predicate_chirho)
                {
                    found_chirho = Some(*evidence_chirho);
                    break;
                }
                scope_chirho = self.scope_parent_chirho(current_chirho)?;
            }
            if let Some(evidence_chirho) = found_chirho {
                self.bind_evidence_chirho(
                    wanted_chirho.evidence_chirho,
                    EvTermChirho::VariableChirho(evidence_chirho),
                )?;
            } else if let Some(evidence_chirho) =
                earlier_chirho.get(&(wanted_chirho.scope_chirho, predicate_chirho))
            {
                self.bind_evidence_chirho(
                    wanted_chirho.evidence_chirho,
                    EvTermChirho::VariableChirho(*evidence_chirho),
                )?;
            } else if let Some((instance_chirho, type_arguments_chirho)) =
                self.match_instance_chirho(class_chirho, argument_chirho)?
            {
                let (_, context_chirho) =
                    self.instance_application_chirho(instance_chirho, &type_arguments_chirho)?;
                let mut dictionary_arguments_chirho = Vec::new();
                // Admitted contexts mention only bare, distinct head variables: each child
                // is a strict subterm of the selected head, so expansion cannot cycle.
                for predicate_chirho in context_chirho {
                    dictionary_arguments_chirho.push(self.want_dictionary_origin_chirho(
                        wanted_chirho.scope_chirho,
                        predicate_chirho,
                        wanted_chirho.span_chirho,
                        WantedOriginChirho::InstanceContextChirho,
                    )?);
                }
                self.bind_evidence_chirho(
                    wanted_chirho.evidence_chirho,
                    EvTermChirho::InstanceChirho {
                        instance_chirho,
                        type_arguments_chirho,
                        dictionary_arguments_chirho,
                    },
                )?;
            }
            earlier_chirho
                .entry((wanted_chirho.scope_chirho, predicate_chirho))
                .or_insert(wanted_chirho.evidence_chirho);
        }
        Ok(())
    }

    pub(crate) fn unresolved_dictionaries_chirho(
        &self,
    ) -> Result<Vec<WantedDictChirho>, TcErrorChirho> {
        let mut result_chirho = Vec::new();
        for wanted_chirho in &self.dictionary_wanteds_chirho {
            if self
                .evidence_term_chirho(wanted_chirho.evidence_chirho)?
                .is_none()
            {
                result_chirho.push(wanted_chirho.clone());
            }
        }
        Ok(result_chirho)
    }
}
