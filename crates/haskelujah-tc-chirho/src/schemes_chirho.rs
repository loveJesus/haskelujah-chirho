// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Equality-only let schemes: solve, quantify deeper free metas, instantiate at each use.
//! GHC reference: GHC.Tc.Gen.Bind.tcPolyInfer and GHC.Tc.Solver.simplifyInfer.
//! workflow: next-pipeline-chirho/ghc-implementation-map-chirho

use std::collections::{HashMap, HashSet};

use crate::{
    TcErrorChirho, TcErrorKindChirho, TyChirho, TyConChirho, TyNodeChirho, TypeArenaChirho,
};

#[derive(Debug, Clone)]
pub struct SchemeChirho {
    pub body_chirho: TyChirho,
    pub quantified_chirho: Vec<TyChirho>,
    pub context_chirho: Vec<TyChirho>,
}

impl TypeArenaChirho {
    /// Final zonking only, after every module equality has solved and schemes are frozen.
    /// As in GHC.Tc.Zonk.Type, internal variables absent from all exported schemes
    /// must not leave free type variables in Core. This is not inference/defaulting.
    pub(crate) fn finish_metas_chirho(&mut self) -> Result<(), TcErrorChirho> {
        let pending_chirho: Vec<_> = self
            .metas_chirho
            .iter()
            .enumerate()
            .filter(|(_, cell_chirho)| {
                cell_chirho.fill_chirho.is_none() && !cell_chirho.generalized_chirho
            })
            .map(|(index_chirho, _)| index_chirho)
            .collect();
        if pending_chirho.is_empty() {
            return Ok(());
        }
        let type_chirho = self.con_chirho(TyConChirho::TypeChirho)?;
        let any_chirho = self.con_chirho(TyConChirho::AnyChirho)?;
        for index_chirho in pending_chirho {
            if self.resolve_chirho(self.metas_chirho[index_chirho].kind_chirho)? != type_chirho {
                return Err(TcErrorChirho::new_chirho(
                    TcErrorKindChirho::UnsupportedChirho,
                    "next final zonking requires checked Type-kinded variables",
                ));
            }
            self.metas_chirho[index_chirho].fill_chirho = Some(any_chirho);
        }
        Ok(())
    }

    /// A binding group has already solved every equality before this admission.
    pub(crate) fn quantify_group_chirho(
        &mut self,
        roots_chirho: &[TyChirho],
        outer_level_chirho: u32,
    ) -> Result<Vec<TyChirho>, TcErrorChirho> {
        let mut work_chirho = roots_chirho.to_vec();
        let mut seen_chirho = HashSet::new();
        let mut quantified_chirho = Vec::new();
        let type_chirho = self.con_chirho(TyConChirho::TypeChirho)?;
        while let Some(ty_chirho) = work_chirho.pop() {
            let ty_chirho = self.resolve_chirho(ty_chirho)?;
            if !seen_chirho.insert(ty_chirho) {
                continue;
            }
            match self.node_chirho(ty_chirho)?.clone() {
                TyNodeChirho::MetaChirho(meta_chirho) => {
                    let cell_chirho = self.meta_cell_chirho(meta_chirho)?;
                    if !cell_chirho.generalized_chirho
                        && cell_chirho.level_chirho > outer_level_chirho
                    {
                        if self.resolve_chirho(cell_chirho.kind_chirho)? != type_chirho {
                            return Err(TcErrorChirho::new_chirho(
                                TcErrorKindChirho::UnsupportedChirho,
                                "next generalization currently requires checked Type-kinded variables",
                            ));
                        }
                        quantified_chirho.push(ty_chirho);
                    }
                }
                TyNodeChirho::AppChirho(function_chirho, argument_chirho)
                | TyNodeChirho::FunChirho(function_chirho, argument_chirho) => {
                    work_chirho.push(argument_chirho);
                    work_chirho.push(function_chirho);
                }
                TyNodeChirho::TupleChirho(items_chirho) => {
                    work_chirho.extend(items_chirho.into_iter().rev())
                }
                TyNodeChirho::ConChirho(_) => {}
                TyNodeChirho::ForallChirho { .. } | TyNodeChirho::RigidChirho(_) => {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "next does not yet generalize explicit quantified or rigid signatures",
                    ));
                }
            }
        }
        quantified_chirho.sort_unstable_by_key(|ty_chirho| {
            match self.node_chirho(*ty_chirho).unwrap() {
                TyNodeChirho::MetaChirho(meta_chirho) => meta_chirho.index_chirho,
                _ => unreachable!(),
            }
        });
        for ty_chirho in &quantified_chirho {
            let TyNodeChirho::MetaChirho(meta_chirho) = self.node_chirho(*ty_chirho)? else {
                unreachable!()
            };
            let index_chirho = meta_chirho.index_chirho as usize;
            self.metas_chirho[index_chirho].generalized_chirho = true;
        }
        Ok(quantified_chirho)
    }

    pub(crate) fn instantiate_chirho(
        &mut self,
        scheme_chirho: &SchemeChirho,
        level_chirho: u32,
    ) -> Result<(TyChirho, Vec<TyChirho>, Vec<TyChirho>), TcErrorChirho> {
        if scheme_chirho.quantified_chirho.is_empty() {
            for _predicate_chirho in &scheme_chirho.context_chirho {
                self.reserve_auxiliary_chirho()?;
            }
            return Ok((
                self.resolve_chirho(scheme_chirho.body_chirho)?,
                Vec::new(),
                scheme_chirho.context_chirho.clone(),
            ));
        }
        let mut replacements_chirho = HashMap::new();
        let mut arguments_chirho = Vec::new();
        for quantified_chirho in &scheme_chirho.quantified_chirho {
            let kind_chirho = match self.node_chirho(*quantified_chirho)? {
                TyNodeChirho::MetaChirho(meta_chirho)
                    if self.meta_cell_chirho(*meta_chirho)?.generalized_chirho =>
                {
                    self.meta_cell_chirho(*meta_chirho)?.kind_chirho
                }
                TyNodeChirho::RigidChirho(rigid_chirho) => {
                    self.rigid_cell_chirho(*rigid_chirho)?.kind_chirho
                }
                _ => {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::InvalidReferenceChirho,
                        "scheme binder was not generalized or rigid",
                    ));
                }
            };
            let argument_chirho = self.fresh_meta_chirho(level_chirho, kind_chirho)?;
            replacements_chirho.insert(*quantified_chirho, argument_chirho);
            arguments_chirho.push(argument_chirho);
        }
        let mut roots_chirho = scheme_chirho.context_chirho.clone();
        roots_chirho.push(scheme_chirho.body_chirho);
        let mut roots_chirho = self.replace_roots_chirho(&roots_chirho, replacements_chirho)?;
        let body_chirho = roots_chirho.pop().unwrap();
        Ok((body_chirho, arguments_chirho, roots_chirho))
    }

    pub(crate) fn replace_chirho(
        &mut self,
        root_chirho: TyChirho,
        images_chirho: HashMap<TyChirho, TyChirho>,
    ) -> Result<TyChirho, TcErrorChirho> {
        Ok(self
            .replace_roots_chirho(&[root_chirho], images_chirho)?
            .remove(0))
    }

    fn replace_roots_chirho(
        &mut self,
        roots_chirho: &[TyChirho],
        mut images_chirho: HashMap<TyChirho, TyChirho>,
    ) -> Result<Vec<TyChirho>, TcErrorChirho> {
        let mut work_chirho = roots_chirho
            .iter()
            .rev()
            .map(|ty_chirho| (*ty_chirho, false))
            .collect::<Vec<_>>();
        while let Some((ty_chirho, ready_chirho)) = work_chirho.pop() {
            let ty_chirho = self.resolve_chirho(ty_chirho)?;
            if images_chirho.contains_key(&ty_chirho) {
                continue;
            }
            let node_chirho = self.node_chirho(ty_chirho)?.clone();
            let children_chirho = match &node_chirho {
                TyNodeChirho::AppChirho(left_chirho, right_chirho)
                | TyNodeChirho::FunChirho(left_chirho, right_chirho) => {
                    vec![*left_chirho, *right_chirho]
                }
                TyNodeChirho::TupleChirho(items_chirho) => items_chirho.clone(),
                TyNodeChirho::ForallChirho { .. } => {
                    return Err(TcErrorChirho::new_chirho(
                        TcErrorKindChirho::UnsupportedChirho,
                        "next instantiation does not yet open explicit foralls",
                    ));
                }
                _ => vec![],
            };
            if !ready_chirho {
                self.reserve_auxiliary_chirho()?;
                work_chirho.push((ty_chirho, true));
                for child_chirho in children_chirho.into_iter().rev() {
                    let child_chirho = self.resolve_chirho(child_chirho)?;
                    if !images_chirho.contains_key(&child_chirho) {
                        work_chirho.push((child_chirho, false));
                    }
                }
                continue;
            }
            let lookup_chirho = |child_chirho: TyChirho| -> Result<TyChirho, TcErrorChirho> {
                images_chirho
                    .get(&self.resolve_chirho(child_chirho)?)
                    .copied()
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "scheme substitution child was not prepared",
                        )
                    })
            };
            let replaced_chirho = match node_chirho {
                TyNodeChirho::AppChirho(left_chirho, right_chirho) => TyNodeChirho::AppChirho(
                    lookup_chirho(left_chirho)?,
                    lookup_chirho(right_chirho)?,
                ),
                TyNodeChirho::FunChirho(left_chirho, right_chirho) => TyNodeChirho::FunChirho(
                    lookup_chirho(left_chirho)?,
                    lookup_chirho(right_chirho)?,
                ),
                TyNodeChirho::TupleChirho(items_chirho) => TyNodeChirho::TupleChirho(
                    items_chirho
                        .into_iter()
                        .map(lookup_chirho)
                        .collect::<Result<_, _>>()?,
                ),
                other_chirho => other_chirho,
            };
            let image_chirho = self.intern_chirho(replaced_chirho)?;
            images_chirho.insert(ty_chirho, image_chirho);
        }
        roots_chirho
            .iter()
            .map(|root_chirho| {
                images_chirho
                    .get(&self.resolve_chirho(*root_chirho)?)
                    .copied()
                    .ok_or_else(|| {
                        TcErrorChirho::new_chirho(
                            TcErrorKindChirho::InvalidReferenceChirho,
                            "scheme substitution produced no root",
                        )
                    })
            })
            .collect()
    }
}
