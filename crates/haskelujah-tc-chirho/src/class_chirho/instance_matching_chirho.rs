// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! One-way matching for a constructor applied to distinct variables. No Wanted is unified here.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum InstanceHeadChirho {
    ConstructorChirho(crate::TyConChirho),
    StructuralChirho(StructuralHeadChirho),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum StructuralHeadChirho {
    TupleChirho(usize),
    FunctionChirho,
}

impl SolverChirho {
    pub(crate) fn instance_shape_chirho(
        &mut self,
        argument_chirho: TyChirho,
    ) -> Result<Option<(InstanceHeadChirho, Vec<TyChirho>)>, TcErrorChirho> {
        let mut head_chirho = self.arena_chirho.resolve_chirho(argument_chirho)?;
        let mut arguments_chirho = Vec::new();
        loop {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            match self.arena_chirho.node_chirho(head_chirho)? {
                TyNodeChirho::AppChirho(function_chirho, argument_chirho) => {
                    arguments_chirho.push(*argument_chirho);
                    head_chirho = self.arena_chirho.resolve_chirho(*function_chirho)?;
                }
                TyNodeChirho::ConChirho(constructor_chirho) => {
                    arguments_chirho.reverse();
                    return Ok(Some((
                        InstanceHeadChirho::ConstructorChirho(*constructor_chirho),
                        arguments_chirho,
                    )));
                }
                TyNodeChirho::TupleChirho(items_chirho) if arguments_chirho.is_empty() => {
                    let items_chirho = items_chirho.clone();
                    for _item_chirho in &items_chirho {
                        self.arena_chirho.reserve_auxiliary_chirho()?;
                    }
                    return Ok(Some((
                        InstanceHeadChirho::StructuralChirho(StructuralHeadChirho::TupleChirho(
                            items_chirho.len(),
                        )),
                        items_chirho,
                    )));
                }
                TyNodeChirho::FunChirho(argument_chirho, result_chirho)
                    if arguments_chirho.is_empty() =>
                {
                    return Ok(Some((
                        InstanceHeadChirho::StructuralChirho(StructuralHeadChirho::FunctionChirho),
                        vec![*argument_chirho, *result_chirho],
                    )));
                }
                _ => return Ok(None),
            }
        }
    }

    pub(crate) fn register_instance_head_chirho(
        &mut self,
        class_chirho: NameIdChirho,
        head_chirho: TyChirho,
        binders_chirho: &[TyChirho],
        instance_chirho: NameIdChirho,
    ) -> Result<(), TcErrorChirho> {
        let Some((constructor_chirho, arguments_chirho)) =
            self.instance_shape_chirho(head_chirho)?
        else {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::UnsupportedChirho,
                "next instances require a constructor head applied to distinct variables",
            ));
        };
        let binders_set_chirho: HashSet<_> = binders_chirho.iter().copied().collect();
        let mut seen_chirho = HashSet::new();
        for argument_chirho in &arguments_chirho {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            if !binders_set_chirho.contains(argument_chirho)
                || !seen_chirho.insert(*argument_chirho)
            {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UnsupportedChirho,
                    "next instance head arguments must be distinct variables",
                ));
            }
        }
        if seen_chirho.len() != binders_chirho.len() {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::UnsupportedChirho,
                "next instance binders must be exactly its constructor arguments",
            ));
        }
        self.arena_chirho.reserve_auxiliary_chirho()?;
        if self
            .constructor_instances_chirho
            .insert((class_chirho, constructor_chirho), instance_chirho)
            .is_some()
        {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::TypeMismatchChirho,
                "duplicate source class instance",
            ));
        }
        Ok(())
    }

    pub(crate) fn instance_application_chirho(
        &mut self,
        instance_chirho: NameIdChirho,
        arguments_chirho: &[TyChirho],
    ) -> Result<(TyChirho, Vec<TyChirho>), TcErrorChirho> {
        let info_chirho = self.instance_info_chirho(instance_chirho)?.clone();
        if info_chirho.binders_chirho.len() != arguments_chirho.len() {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "instance evidence type argument arity disagrees with its declaration",
            ));
        }
        let mut substitution_chirho = HashMap::new();
        for (binder_chirho, argument_chirho) in
            info_chirho.binders_chirho.iter().zip(arguments_chirho)
        {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            if self.arena_chirho.kind_chirho(*binder_chirho)?
                != self.arena_chirho.kind_chirho(*argument_chirho)?
            {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "instance evidence type argument kind disagrees with its binder",
                ));
            }
            substitution_chirho.insert(*binder_chirho, *argument_chirho);
        }
        let head_chirho = self
            .arena_chirho
            .replace_chirho(info_chirho.head_chirho, substitution_chirho.clone())?;
        let predicate_chirho = self.predicate_chirho(info_chirho.class_chirho, head_chirho)?;
        let mut context_chirho = Vec::new();
        for predicate_chirho in info_chirho.context_chirho {
            context_chirho.push(
                self.arena_chirho
                    .replace_chirho(predicate_chirho, substitution_chirho.clone())?,
            );
        }
        Ok((predicate_chirho, context_chirho))
    }

    pub(super) fn match_instance_chirho(
        &mut self,
        class_chirho: NameIdChirho,
        argument_chirho: TyChirho,
    ) -> Result<Option<(NameIdChirho, Vec<TyChirho>)>, TcErrorChirho> {
        let Some((constructor_chirho, arguments_chirho)) =
            self.instance_shape_chirho(argument_chirho)?
        else {
            return Ok(None);
        };
        let Some(instance_chirho) = self
            .constructor_instances_chirho
            .get(&(class_chirho, constructor_chirho))
            .copied()
        else {
            return Ok(None);
        };
        let info_chirho = self.instance_info_chirho(instance_chirho)?.clone();
        let Some((_, patterns_chirho)) = self.instance_shape_chirho(info_chirho.head_chirho)?
        else {
            return Err(TcErrorChirho::new_chirho(
                TcErrorKindChirho::InvalidReferenceChirho,
                "checked instance lost its constructor head",
            ));
        };
        if patterns_chirho.len() != arguments_chirho.len() {
            return Ok(None);
        }
        let substitution_chirho: HashMap<_, _> =
            patterns_chirho.into_iter().zip(arguments_chirho).collect();
        let mut matched_chirho = Vec::new();
        for binder_chirho in info_chirho.binders_chirho {
            self.arena_chirho.reserve_auxiliary_chirho()?;
            matched_chirho.push(*substitution_chirho.get(&binder_chirho).ok_or_else(|| {
                TcErrorChirho::new_chirho(
                    TcErrorKindChirho::InvalidReferenceChirho,
                    "checked instance binder is absent from its head",
                )
            })?);
        }
        Ok(Some((instance_chirho, matched_chirho)))
    }
}
