// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Acyclic superclass search and owner-local Given projections. Parameters keep their exact arity.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::evidence_chirho::{BindingKindChirho, EvidenceKindChirho};
use super::*;
use std::collections::VecDeque;

impl SolverChirho {
    pub fn dictionary_fields_chirho(
        &self,
        class_chirho: NameIdChirho,
    ) -> Result<Vec<(NameIdChirho, TyChirho)>, TcErrorChirho> {
        let info_chirho = self.class_info_chirho(class_chirho)?;
        if info_chirho.superclasses_chirho.len() != info_chirho.superclass_selectors_chirho.len() {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "superclass declaration and selector layout disagree",
            ));
        }
        Ok(info_chirho
            .superclass_selectors_chirho
            .iter()
            .copied()
            .zip(info_chirho.superclasses_chirho.iter().copied())
            .chain(info_chirho.methods_chirho.iter().map(|method_chirho| {
                (
                    method_chirho.name_chirho,
                    method_chirho.scheme_chirho.body_chirho,
                )
            }))
            .collect())
    }

    pub(crate) fn check_superclass_cycles_chirho(&mut self) -> Result<(), TcErrorChirho> {
        self.has_superclasses_chirho = self
            .classes_chirho
            .values()
            .any(|info_chirho| !info_chirho.superclasses_chirho.is_empty());
        let mut colors_chirho = HashMap::new();
        let mut classes_chirho: Vec<_> = self.classes_chirho.keys().copied().collect();
        classes_chirho.sort_by_key(|name_chirho| name_chirho.0);
        for class_chirho in classes_chirho {
            if colors_chirho.get(&class_chirho) == Some(&2) {
                continue;
            }
            let mut stack_chirho = vec![(class_chirho, 0)];
            colors_chirho.insert(class_chirho, 1);
            while let Some((current_chirho, index_chirho)) = stack_chirho.last_mut() {
                self.arena_chirho.reserve_auxiliary_chirho()?;
                let supers_chirho = &self.class_info_chirho(*current_chirho)?.superclasses_chirho;
                if *index_chirho == supers_chirho.len() {
                    colors_chirho.insert(*current_chirho, 2);
                    stack_chirho.pop();
                    continue;
                }
                let predicate_chirho = supers_chirho[*index_chirho];
                *index_chirho += 1;
                let (parent_chirho, _) = self.predicate_view_chirho(predicate_chirho)?;
                match colors_chirho.get(&parent_chirho) {
                    Some(1) => {
                        return Err(TcErrorChirho::new_chirho(
                            TcErrorKindChirho::TypeMismatchChirho,
                            "cycle in source class superclasses",
                        ));
                    }
                    Some(2) => {}
                    _ => {
                        colors_chirho.insert(parent_chirho, 1);
                        stack_chirho.push((parent_chirho, 0));
                    }
                }
            }
        }
        Ok(())
    }

    fn superclass_path_chirho(
        &mut self,
        source_chirho: NameIdChirho,
        target_chirho: NameIdChirho,
    ) -> Result<Option<Vec<(NameIdChirho, usize)>>, TcErrorChirho> {
        let mut queue_chirho = VecDeque::from([source_chirho]);
        let mut parents_chirho = HashMap::from([(source_chirho, None)]);
        while let Some(class_chirho) = queue_chirho.pop_front() {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            if class_chirho == target_chirho {
                let mut path_chirho = Vec::new();
                let mut current_chirho = target_chirho;
                while let Some((parent_chirho, index_chirho)) = parents_chirho[&current_chirho] {
                    self.arena_chirho.reserve_auxiliary_chirho()?;
                    path_chirho.push((parent_chirho, index_chirho));
                    current_chirho = parent_chirho;
                }
                path_chirho.reverse();
                return Ok(Some(path_chirho));
            }
            let supers_chirho = self
                .class_info_chirho(class_chirho)?
                .superclasses_chirho
                .clone();
            for (index_chirho, predicate_chirho) in supers_chirho.into_iter().enumerate() {
                self.arena_chirho.reserve_auxiliary_chirho()?;
                let (super_chirho, _) = self.predicate_view_chirho(predicate_chirho)?;
                if let std::collections::hash_map::Entry::Vacant(entry_chirho) =
                    parents_chirho.entry(super_chirho)
                {
                    entry_chirho.insert(Some((class_chirho, index_chirho)));
                    queue_chirho.push_back(super_chirho);
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn minimal_superclass_context_chirho(
        &mut self,
        predicates_chirho: Vec<TyChirho>,
    ) -> Result<Vec<TyChirho>, TcErrorChirho> {
        let mut by_argument_chirho: HashMap<TyChirho, HashMap<NameIdChirho, TyChirho>> =
            HashMap::new();
        for &predicate_chirho in &predicates_chirho {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            let (class_chirho, argument_chirho) = self.predicate_view_chirho(predicate_chirho)?;
            by_argument_chirho
                .entry(argument_chirho)
                .or_default()
                .insert(class_chirho, predicate_chirho);
        }
        let mut redundant_chirho = HashSet::new();
        for &predicate_chirho in &predicates_chirho {
            let (source_chirho, argument_chirho) = self.predicate_view_chirho(predicate_chirho)?;
            let mut queue_chirho = VecDeque::from([source_chirho]);
            let mut seen_chirho = HashSet::from([source_chirho]);
            while let Some(class_chirho) = queue_chirho.pop_front() {
                self.arena_chirho.reserve_auxiliary_chirho()?;
                let supers_chirho = self
                    .class_info_chirho(class_chirho)?
                    .superclasses_chirho
                    .clone();
                for superclass_chirho in supers_chirho {
                    self.arena_chirho.reserve_auxiliary_chirho()?;
                    let (superclass_chirho, _) = self.predicate_view_chirho(superclass_chirho)?;
                    if seen_chirho.insert(superclass_chirho) {
                        if let Some(predicate_chirho) =
                            by_argument_chirho[&argument_chirho].get(&superclass_chirho)
                        {
                            redundant_chirho.insert(*predicate_chirho);
                        }
                        queue_chirho.push_back(superclass_chirho);
                    }
                }
            }
        }
        Ok(predicates_chirho
            .into_iter()
            .filter(|predicate_chirho| !redundant_chirho.contains(predicate_chirho))
            .collect())
    }

    pub(super) fn superclass_given_chirho(
        &mut self,
        mut scope_chirho: ScopeIdChirho,
        predicate_chirho: TyChirho,
    ) -> Result<Option<EvVarChirho>, TcErrorChirho> {
        let (target_chirho, argument_chirho) = self.predicate_view_chirho(predicate_chirho)?;
        loop {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            if let Some(variable_chirho) = self.scopes_chirho[scope_chirho.index_chirho]
                .derived_givens_chirho
                .get(&predicate_chirho)
            {
                return Ok(Some(*variable_chirho));
            }
            let roots_chirho = self.scope_givens_chirho(scope_chirho)?.to_vec();
            for root_chirho in roots_chirho {
                self.arena_chirho.reserve_auxiliary_chirho()?;
                if root_chirho.blocked_chirho {
                    continue;
                }
                let root_predicate_chirho = self
                    .arena_chirho
                    .replace_chirho(root_chirho.predicate_chirho, HashMap::new())?;
                let (source_chirho, root_argument_chirho) =
                    self.predicate_view_chirho(root_predicate_chirho)?;
                if root_argument_chirho != argument_chirho {
                    continue;
                }
                let Some(path_chirho) =
                    self.superclass_path_chirho(source_chirho, target_chirho)?
                else {
                    continue;
                };
                let mut dictionary_chirho = root_chirho.evidence_chirho;
                for (class_chirho, index_chirho) in path_chirho {
                    let predicate_chirho = self.superclass_predicate_chirho(
                        class_chirho,
                        index_chirho,
                        argument_chirho,
                    )?;
                    if let Some(existing_chirho) = self.scopes_chirho[scope_chirho.index_chirho]
                        .derived_givens_chirho
                        .get(&predicate_chirho)
                    {
                        dictionary_chirho = *existing_chirho;
                        continue;
                    }
                    let variable_chirho = self.allocate_evidence_chirho(
                        scope_chirho,
                        predicate_chirho,
                        EvidenceKindChirho::BindingChirho(BindingKindChirho::SuperclassChirho),
                    )?;
                    self.bind_evidence_chirho(
                        variable_chirho,
                        EvTermChirho::SuperclassChirho {
                            class_chirho,
                            index_chirho,
                            argument_chirho,
                            dictionary_chirho,
                        },
                    )?;
                    self.arena_chirho.reserve_auxiliary_chirho()?;
                    self.scopes_chirho[scope_chirho.index_chirho]
                        .derived_givens_chirho
                        .insert(predicate_chirho, variable_chirho);
                    dictionary_chirho = variable_chirho;
                }
                return Ok(Some(dictionary_chirho));
            }
            let Some(parent_chirho) = self.scope_parent_chirho(scope_chirho)? else {
                return Ok(None);
            };
            scope_chirho = parent_chirho;
        }
    }

    fn superclass_predicate_chirho(
        &mut self,
        class_chirho: NameIdChirho,
        index_chirho: usize,
        argument_chirho: TyChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        let info_chirho = self.class_info_chirho(class_chirho)?;
        let predicate_chirho = *info_chirho
            .superclasses_chirho
            .get(index_chirho)
            .ok_or_else(|| {
                TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "invalid superclass field index",
                )
            })?;
        let parameter_chirho = info_chirho.parameter_chirho;
        self.arena_chirho.replace_chirho(
            predicate_chirho,
            HashMap::from([(parameter_chirho, argument_chirho)]),
        )
    }

    pub(super) fn validate_superclass_evidence_chirho(
        &mut self,
        variable_chirho: EvVarChirho,
        class_chirho: NameIdChirho,
        index_chirho: usize,
        argument_chirho: TyChirho,
        dictionary_chirho: EvVarChirho,
    ) -> Result<TyChirho, TcErrorChirho> {
        let destination_chirho = self.evidence_record_chirho(variable_chirho)?;
        let source_chirho = self.evidence_record_chirho(dictionary_chirho)?;
        if destination_chirho.kind_chirho
            != EvidenceKindChirho::BindingChirho(BindingKindChirho::SuperclassChirho)
            || !source_chirho.kind_chirho.is_given_chirho()
            || source_chirho.scope_chirho != destination_chirho.scope_chirho
            || dictionary_chirho.index_chirho >= variable_chirho.index_chirho
        {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "superclass projection requires an earlier Given in its owning scope",
            ));
        }
        if source_chirho.kind_chirho
            == EvidenceKindChirho::BindingChirho(BindingKindChirho::SuperclassChirho)
            && self.evidence_term_chirho(dictionary_chirho)?.is_none()
        {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "superclass projection source has no validated definition",
            ));
        }
        let source_predicate_chirho = source_chirho.predicate_chirho;
        let expected_chirho = self.predicate_chirho(class_chirho, argument_chirho)?;
        if self
            .arena_chirho
            .replace_chirho(source_predicate_chirho, HashMap::new())?
            != self
                .arena_chirho
                .replace_chirho(expected_chirho, HashMap::new())?
        {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "superclass projection uses a different class or argument",
            ));
        }
        self.superclass_predicate_chirho(class_chirho, index_chirho, argument_chirho)
    }
}
